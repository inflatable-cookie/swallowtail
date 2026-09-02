//! One open Claude Agent SDK sidecar session and its descendant-tree close.

use super::handle::{ClaudeAgentSdkTurnHandle, SessionCancellation, TurnBinding};
use super::startup::SessionReadiness;
use super::validation::validate_turn;
use crate::sdk::close::ClaudeAgentSdkCloseState;
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

pub(super) use close::merge_cleanup;

/// Bound the sidecar states, and honours, when joining its own retained
/// native child handle before the host escalates.
pub(super) const CLOSE_JOIN_BOUND_MS: u64 = 2_000;
/// Runtime capability that must be advertised before an interrupt receipt is
/// admissible.
pub(super) const INTERRUPT_RECEIPT_CAPABILITY: &str = "interrupt_receipt_v1";

pub(super) type ActiveSlot = Arc<Mutex<Option<Arc<SdkActiveTurn>>>>;

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
            self.reap_finished();
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
            let (turn, events, callbacks, terminal) = SdkActiveTurn::new(
                request.turn_id().clone(),
                Arc::downgrade(&self.connection),
                request.deadline(),
            )?;
            self.connection.set_active_turn(Arc::clone(&turn))?;
            *self
                .active
                .lock()
                .expect("SDK sidecar active lock poisoned") = Some(Arc::clone(&turn));
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
            if let Some(turn) = &active
                && !turn.is_finished()
            {
                turn.mark_cancelled();
            }
            let state = close::close_tree(
                &self.connection,
                &self.request_id,
                active.is_some(),
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
    fn reap_finished(&self) {
        let mut active = self
            .active
            .lock()
            .expect("SDK sidecar active lock poisoned");
        if active.as_ref().is_some_and(|turn| turn.is_finished()) {
            active.take();
        }
    }

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

pub(super) fn close_state(data: Option<&Value>) -> Option<ClaudeAgentSdkCloseState> {
    let data = data?;
    if data.get("joinBoundMs").and_then(Value::as_u64) != Some(CLOSE_JOIN_BOUND_MS) {
        return None;
    }
    let observed = data.get("nativeExitObserved").and_then(Value::as_bool)?;
    let state = ClaudeAgentSdkCloseState::from_sidecar(data.get("closeState")?.as_str()?)?;
    // A reported graceful join must carry the observation that produced it.
    match (state, observed) {
        (ClaudeAgentSdkCloseState::Graceful, true)
        | (ClaudeAgentSdkCloseState::Unconfirmed, false) => Some(state),
        _ => None,
    }
}

fn query_rejected() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.query_rejected",
        "Claude Agent SDK sidecar rejected the query before acceptance",
    )
}
