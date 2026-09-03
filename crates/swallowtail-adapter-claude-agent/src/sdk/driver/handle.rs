use super::session::ActiveSlot;
use crate::sdk::bounded::HostBound;
use crate::sdk::connection::SdkConnection;
use crate::sdk::failure::failure;
use crate::sdk::turn::SdkActiveTurn;
use crate::sdk::wire::ClaudeAgentSdkCommand;
use serde_json::{Value, json};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use swallowtail_core::{CancellationScope, TurnRef};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CallbackAbandonment, CallbackExchange, CancellationAcknowledgement,
    CancellationControl, CleanupOutcome, HarnessCommandResponse, HarnessScheduledMessage,
    RuntimeFailure, RuntimeTurnId, TerminalOutcome, TurnHandle,
};

/// Interrupts the live turn through the SDK's own control surface. A receipt
/// exists only where the runtime advertised `interrupt_receipt_v1`; absence is
/// reported honestly rather than assumed away.
pub(super) struct TurnCancellation {
    connection: Arc<SdkConnection>,
    turn: Arc<SdkActiveTurn>,
    receipts_advertised: bool,
    bounded: HostBound,
    requested: AtomicBool,
}

impl TurnCancellation {
    pub(super) fn new(
        connection: Arc<SdkConnection>,
        turn: Arc<SdkActiveTurn>,
        receipts_advertised: bool,
        bounded: HostBound,
    ) -> Self {
        Self {
            connection,
            turn,
            receipts_advertised,
            bounded,
            requested: AtomicBool::new(false),
        }
    }
}

impl CancellationControl for TurnCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::ActiveTurn
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let already = self.requested.swap(true, Ordering::SeqCst);
        Box::pin(async move {
            if already {
                return Ok(CancellationAcknowledgement::AlreadyRequested);
            }
            self.turn.mark_cancelled();
            let id = format!("interrupt:{}", self.turn.runtime_id().as_str());
            // Cancellation is a request, never a claim of provider truth. Both
            // halves are bounded by the caller's turn deadline: the wire write
            // itself, because a stalled process write would otherwise hold this
            // public control forever, and then the receipt.
            let Some(sent) = self
                .bounded
                .run(
                    self.connection
                        .send(id, ClaudeAgentSdkCommand::Interrupt, json!({})),
                )
                .await
            else {
                return Ok(CancellationAcknowledgement::Requested);
            };
            let pending = sent?;
            let Some(response) = self.bounded.run(pending).await else {
                return Ok(CancellationAcknowledgement::Requested);
            };
            let response = response?;
            if !response.success {
                return Err(failure(
                    "swallowtail.claude-agent.sdk.interrupt_rejected",
                    "Claude Agent SDK sidecar rejected the interrupt",
                ));
            }
            let receipt = response
                .data
                .as_ref()
                .and_then(|data| data.get("receipt"))
                .and_then(Value::as_bool)
                .unwrap_or(false);
            if receipt && !self.receipts_advertised {
                return Err(failure(
                    "swallowtail.claude-agent.sdk.interrupt_receipt_unadvertised",
                    "Claude Agent SDK sidecar reported an interrupt receipt the runtime never advertised",
                ));
            }
            Ok(CancellationAcknowledgement::Requested)
        })
    }
}

/// Session-scoped cancellation is a local request, and deliberately performs
/// no host call.
///
/// `CancellationControl::request` carries no caller deadline, so any host await
/// here would be an unbounded public control. Instead the request is recorded
/// and the live turn is marked cancelled; the descendant termination itself is
/// owned by `close`, which runs inside the caller's cleanup deadline. This
/// never claims the provider stopped, which cancellation may not claim anyway.
pub(super) struct SessionCancellation {
    active: ActiveSlot,
    requested: AtomicBool,
}

impl SessionCancellation {
    pub(super) fn new(active: ActiveSlot) -> Self {
        Self {
            active,
            requested: AtomicBool::new(false),
        }
    }

    /// Reports whether the consumer asked the session to cancel, so close can
    /// treat the session as already cancelled.
    pub(super) fn was_requested(&self) -> bool {
        self.requested.load(Ordering::SeqCst)
    }
}

impl CancellationControl for SessionCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::InteractiveSession
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let already = self.requested.swap(true, Ordering::SeqCst);
        // No awaits at all: this returns inside any caller bound by construction.
        if let Some(active) = self
            .active
            .lock()
            .expect("SDK sidecar active lock poisoned")
            .as_ref()
        {
            active.turn.mark_cancelled();
        }
        Box::pin(async move {
            if already {
                Ok(CancellationAcknowledgement::AlreadyRequested)
            } else {
                Ok(CancellationAcknowledgement::Requested)
            }
        })
    }
}

pub(super) struct ClaudeAgentSdkTurnHandle {
    runtime_id: RuntimeTurnId,
    events: Option<BoxEventStream>,
    callbacks: Option<CallbackExchange>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: TurnCancellation,
    turn: Arc<SdkActiveTurn>,
    connection: Arc<SdkConnection>,
    active: ActiveSlot,
}

pub(super) struct TurnBinding {
    pub(super) connection: Arc<SdkConnection>,
    pub(super) turn: Arc<SdkActiveTurn>,
    pub(super) active: ActiveSlot,
    pub(super) receipts_advertised: bool,
    pub(super) bounded: HostBound,
}

impl ClaudeAgentSdkTurnHandle {
    pub(super) fn new(
        runtime_id: RuntimeTurnId,
        events: BoxEventStream,
        callbacks: CallbackExchange,
        terminal: BoxFuture<'static, TerminalOutcome>,
        binding: TurnBinding,
    ) -> Self {
        Self {
            runtime_id,
            events: Some(events),
            callbacks: Some(callbacks),
            terminal: Some(terminal),
            cancellation: TurnCancellation::new(
                Arc::clone(&binding.connection),
                Arc::clone(&binding.turn),
                binding.receipts_advertised,
                binding.bounded.clone(),
            ),
            turn: binding.turn,
            connection: binding.connection,
            active: binding.active,
        }
    }
}

impl TurnHandle for ClaudeAgentSdkTurnHandle {
    fn turn_id(&self) -> &RuntimeTurnId {
        &self.runtime_id
    }

    fn provider_turn_ref(&self) -> Option<&TurnRef> {
        None
    }

    fn take_events(&mut self) -> Option<BoxEventStream> {
        self.events.take()
    }

    fn take_callbacks(&mut self) -> Option<CallbackExchange> {
        self.callbacks.take()
    }

    fn schedule_harness_message(
        &mut self,
        message: HarnessScheduledMessage,
    ) -> BoxFuture<'_, Result<HarnessCommandResponse, RuntimeFailure>> {
        Box::pin(async move {
            Ok(HarnessCommandResponse::new(
                message.command_id().clone(),
                swallowtail_runtime::HarnessCommandAcknowledgement::Rejected,
            ))
        })
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        &self.cancellation
    }

    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>> {
        self.terminal.take()
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            if !self.turn.is_finished() {
                let _ = self.cancellation.request().await;
            }
            self.turn
                .abandon_admissions(CallbackAbandonment::TurnTerminated);
            self.connection.clear_active_turn(&self.turn);
            let mut slot = self
                .active
                .lock()
                .expect("SDK sidecar active lock poisoned");
            if slot
                .as_ref()
                .is_some_and(|active| Arc::ptr_eq(&active.turn, &self.turn))
            {
                slot.take();
            }
            CleanupOutcome::NotApplicable
        })
    }
}
