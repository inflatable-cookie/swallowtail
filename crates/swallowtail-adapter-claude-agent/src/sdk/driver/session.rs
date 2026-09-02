//! One open Claude Agent SDK sidecar session and its descendant-tree close.

use super::handle::{ClaudeAgentSdkTurnHandle, SessionCancellation, TurnBinding};
use super::startup::SessionReadiness;
use super::validation::validate_turn;
use crate::sdk::close::SidecarNativeJoin;
use crate::sdk::connection::SdkConnection;
use crate::sdk::failure::failure;
use crate::sdk::turn::SdkActiveTurn;
use crate::sdk::wire::ClaudeAgentSdkCommand;
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use swallowtail_runtime::{
    BoxFuture, CancellationControl, CleanupOutcome, CredentialLease, HostServices,
    InteractiveSessionHandle, JoinedTask, RequestId, ResourceLease, RuntimeFailure,
    RuntimeSessionId, TurnHandle, TurnRequest,
};

mod close;
mod deadline;

pub(super) use close::merge_cleanup;
use deadline::{reap_finished, spawn_turn_deadline};

/// Bound the sidecar states, and honours, when joining its own retained
/// native child handle before the host escalates.
pub(super) const CLOSE_JOIN_BOUND_MS: u64 = 2_000;
/// Runtime capability that must be advertised before an interrupt receipt is
/// admissible.
pub(super) const INTERRUPT_RECEIPT_CAPABILITY: &str = "interrupt_receipt_v1";

pub(super) type ActiveSlot = Arc<Mutex<Option<ActiveTurn>>>;

/// One live turn and the host-deadline task that bounds it.
pub(super) struct ActiveTurn {
    pub(super) turn: Arc<SdkActiveTurn>,
    pub(super) deadline_task: Option<Box<dyn JoinedTask>>,
}

pub(super) struct ClaudeAgentSdkSessionHandle {
    pub(super) request_id: RequestId,
    pub(super) runtime_id: RuntimeSessionId,
    pub(super) execution_host_id: swallowtail_core::ExecutionHostId,
    pub(super) connection: Arc<SdkConnection>,
    pub(super) cancellation: SessionCancellation,
    pub(super) pump_task: Option<Box<dyn JoinedTask>>,
    pub(super) services: HostServices,
    pub(super) resource: Option<ResourceLease>,
    pub(super) credential: Option<CredentialLease>,
    pub(super) readiness: SessionReadiness,
    pub(super) active: ActiveSlot,
}

impl InteractiveSessionHandle for ClaudeAgentSdkSessionHandle {
    fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    fn session_id(&self) -> &RuntimeSessionId {
        &self.runtime_id
    }

    fn provider_session_ref(&self) -> Option<&swallowtail_core::SessionRef> {
        None
    }

    fn resume_binding(&self) -> Option<&swallowtail_runtime::SessionResumeBinding> {
        None
    }

    fn start_turn<'a>(
        &'a mut self,
        request: TurnRequest,
        services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn TurnHandle>, RuntimeFailure>> {
        Box::pin(async move {
            services.require_execution_host(&self.execution_host_id)?;
            validate_turn(&request)?;
            reap_finished(&self.active).await;
            if self
                .active
                .lock()
                .expect("SDK sidecar active lock poisoned")
                .is_some()
            {
                return Err(failure(
                    "swallowtail.claude-agent.sdk.turn_active",
                    "Claude Agent SDK sidecar session already has an active turn",
                ));
            }
            let deadline = request.deadline().expect("validated turn deadline");
            let (turn, events, callbacks, terminal) = SdkActiveTurn::new(
                request.turn_id().clone(),
                Arc::downgrade(&self.connection),
                Some(deadline),
            )?;
            self.connection.set_active_turn(Arc::clone(&turn))?;
            // The host deadline races real completion. On expiry it interrupts
            // provider work and resolves the turn as timed out rather than
            // letting an unbounded provider turn hold the session.
            let deadline_task = match spawn_turn_deadline(
                &services,
                Arc::clone(&self.connection),
                Arc::clone(&turn),
                deadline,
            ) {
                Ok(task) => task,
                Err(error) => {
                    self.connection.clear_active_turn(&turn);
                    return Err(error);
                }
            };
            *self
                .active
                .lock()
                .expect("SDK sidecar active lock poisoned") = Some(ActiveTurn {
                turn: Arc::clone(&turn),
                deadline_task: Some(deadline_task),
            });
            let id = format!("query:{}", request.turn_id().as_str());
            let response = self
                .connection
                .command(
                    id,
                    ClaudeAgentSdkCommand::Query,
                    json!({"text": request.content().as_str()}),
                )
                .await;
            match response {
                Ok(response) if response.success => Ok(Box::new(ClaudeAgentSdkTurnHandle::new(
                    request.turn_id().clone(),
                    events,
                    callbacks,
                    Box::pin(terminal),
                    TurnBinding {
                        connection: Arc::clone(&self.connection),
                        turn,
                        active: Arc::clone(&self.active),
                        receipts_advertised: self
                            .readiness
                            .advertises(INTERRUPT_RECEIPT_CAPABILITY),
                    },
                )) as Box<dyn TurnHandle>),
                Ok(_) => Err(self.reject_turn(&turn, query_rejected())),
                Err(error) => Err(self.reject_turn(&turn, error)),
            }
        })
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        &self.cancellation
    }

    fn close(mut self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            let active = self
                .active
                .lock()
                .expect("SDK sidecar active lock poisoned")
                .take();
            // Closing the session ends any live turn first. The turn must
            // reach its terminal outcome before its host-deadline task can be
            // joined: that task waits on completion or expiry, so joining it
            // while the turn is still open would wait for an event that close
            // itself just prevented.
            if let Some(active) = &active
                && !active.turn.is_finished()
            {
                active.turn.mark_cancelled();
                active
                    .turn
                    .fail_connection(swallowtail_core::SafeDiagnostic::new(
                        "swallowtail.claude-agent.sdk.session_closing",
                        "Claude Agent SDK sidecar session closed while a turn was active",
                    ));
            }
            let turn_active = active.is_some();
            if let Some(mut active) = active
                && let Some(task) = active.deadline_task.take()
            {
                let _ = task.join().await;
            }
            let state = close::close_tree(
                &self.connection,
                &self.request_id,
                turn_active,
                self.pump_task.take(),
            )
            .await;
            let resource = close::release_resource(self.resource.take(), &self.services).await;
            let credential =
                close::release_credential(self.credential.take(), &self.services).await;
            merge_cleanup(merge_cleanup(state.cleanup_outcome(), resource), credential)
        })
    }
}

impl ClaudeAgentSdkSessionHandle {
    fn reject_turn(&self, turn: &Arc<SdkActiveTurn>, error: RuntimeFailure) -> RuntimeFailure {
        turn.fail_connection(error.diagnostic().clone());
        self.connection.clear_active_turn(turn);
        self.active
            .lock()
            .expect("SDK sidecar active lock poisoned")
            .take();
        error
    }
}

pub(super) fn native_join(data: Option<&Value>) -> Option<SidecarNativeJoin> {
    let data = data?;
    if data.get("joinBoundMs").and_then(Value::as_u64) != Some(CLOSE_JOIN_BOUND_MS) {
        return None;
    }
    let observed = data.get("nativeExitObserved").and_then(Value::as_bool)?;
    let join = SidecarNativeJoin::from_sidecar(data.get("closeState")?.as_str()?)?;
    // A reported join must carry the observation that produced it.
    match (join, observed) {
        (SidecarNativeJoin::Observed, true) | (SidecarNativeJoin::Unconfirmed, false) => Some(join),
        _ => None,
    }
}

fn query_rejected() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.query_rejected",
        "Claude Agent SDK sidecar rejected the query before acceptance",
    )
}
