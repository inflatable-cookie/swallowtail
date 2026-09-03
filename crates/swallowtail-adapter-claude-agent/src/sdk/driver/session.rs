//! One open Claude Agent SDK sidecar session and its descendant-tree close.

use super::handle::{ClaudeAgentSdkTurnHandle, SessionCancellation, TurnBinding};
use super::startup::SessionReadiness;
use super::validation::validate_turn;
use crate::sdk::bounded::HostBound;
use crate::sdk::connection::SdkConnection;
use crate::sdk::failure::failure;
use crate::sdk::turn::SdkActiveTurn;
use crate::sdk::wire::ClaudeAgentSdkCommand;
use serde_json::json;
use std::sync::{Arc, Mutex};
use swallowtail_runtime::{
    BoxFuture, CancellationControl, CleanupOutcome, CredentialLease, HostServices,
    InteractiveSessionHandle, JoinedTask, RequestId, ResourceLease, RuntimeFailure,
    RuntimeSessionId, ScopeId, TaskReapReservation, TurnHandle, TurnRequest,
};

mod close;
mod deadline;

use deadline::{reap_finished, spawn_turn_deadline};

/// Runtime capability that must be advertised before an interrupt receipt is
/// admissible.
pub(super) const INTERRUPT_RECEIPT_CAPABILITY: &str = "interrupt_receipt_v1";

pub(super) type ActiveSlot = Arc<Mutex<Option<ActiveTurn>>>;

/// One live turn and the host-deadline task that bounds it.
pub(super) struct ActiveTurn {
    pub(super) turn: Arc<SdkActiveTurn>,
    pub(super) deadline_task: Option<Box<dyn JoinedTask>>,
    /// The exact scope that task was spawned under, so an unfinished one can
    /// be handed back to the host that owns it.
    pub(super) deadline_scope: ScopeId,
}

pub(super) struct ClaudeAgentSdkSessionHandle {
    pub(super) request_id: RequestId,
    pub(super) runtime_id: RuntimeSessionId,
    pub(super) execution_host_id: swallowtail_core::ExecutionHostId,
    pub(super) connection: Arc<SdkConnection>,
    pub(super) cancellation: SessionCancellation,
    pub(super) pump_task: Option<Box<dyn JoinedTask>>,
    /// Host authority for the enclosing close guardian, granted before this
    /// session acquired a credential, a resource, a process, or a task. Holding
    /// it is what makes the later guardian transfer non-fallible.
    pub(super) close_reservation: Option<Box<dyn TaskReapReservation>>,
    /// The exact scope that reservation was issued for.
    pub(super) close_scope: ScopeId,
    /// The exact scope the pump task was spawned under.
    pub(super) session_scope: ScopeId,
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
            let turn_deadline = request.deadline().expect("validated turn deadline");
            reap_finished(
                &self.active,
                &services,
                &self.execution_host_id,
                turn_deadline,
            )
            .await;
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
            let deadline = turn_deadline;
            let (turn, events, callbacks, terminal) = SdkActiveTurn::new(
                request.turn_id().clone(),
                Arc::downgrade(&self.connection),
                Some(deadline),
            )?;
            self.connection.set_active_turn(Arc::clone(&turn))?;
            // The host deadline races real completion. On expiry it interrupts
            // provider work and resolves the turn as timed out rather than
            // letting an unbounded provider turn hold the session.
            let (deadline_task, deadline_scope) = match spawn_turn_deadline(
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
                deadline_scope,
            });
            // The public start is raced against the caller's turn deadline, so
            // a sidecar that stops answering cannot hold this future open.
            let bounded = HostBound::new(
                services
                    .time()
                    .cloned()
                    .expect("validated sidecar time service"),
                deadline,
            );
            let id = format!("query:{}", request.turn_id().as_str());
            let response = bounded
                .run(self.connection.command(
                    id,
                    ClaudeAgentSdkCommand::Query,
                    json!({"text": request.content().as_str()}),
                ))
                .await;
            let response = match response {
                Some(response) => response,
                None => {
                    return Err(self.reject_turn(&turn, turn_deadline_elapsed()));
                }
            };
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
                        bounded,
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

    /// Closes inside the caller's single cleanup deadline.
    ///
    /// The shared bound covers the whole ordered continuation the enclosing
    /// guardian runs: turn resolution, interruption, the close command, host
    /// escalation, root observation, the pump join, and both lease releases. No
    /// stage restarts it, and expiry transfers that guardian and returns
    /// unconfirmed cleanup rather than extending the public future.
    fn close(
        mut self: Box<Self>,
        request: swallowtail_runtime::SessionCleanupRequest,
        services: HostServices,
    ) -> BoxFuture<'static, CleanupOutcome> {
        let execution_host_id = self.execution_host_id.clone();
        let deadline = request.deadline();
        // One deadline, applied by the shared cleanup bound and again inside
        // every stage so no single stage can consume the whole budget.
        swallowtail_runtime::bound_session_cleanup(
            execution_host_id,
            request,
            services,
            Box::pin(async move { close::close_session(&mut self, deadline).await }),
        )
    }
}

/// A session dropped without close still must not join a running pump on the
/// dropping thread. The pump was started under this session's own reservation,
/// so the owning host takes it back here; a pump that already finished is
/// refused and joined by ordinary drop, which cannot block.
impl Drop for ClaudeAgentSdkSessionHandle {
    fn drop(&mut self) {
        if self.pump_task.is_none() {
            return;
        }
        if let Some(service) = self.services.task() {
            let _ = swallowtail_runtime::ScopedTaskService::relinquish(
                service.as_ref(),
                &self.session_scope,
                &mut self.pump_task,
            );
        }
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

fn turn_deadline_elapsed() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.turn_deadline_elapsed",
        "Claude Agent SDK sidecar turn reached its host deadline",
    )
}

fn query_rejected() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.query_rejected",
        "Claude Agent SDK sidecar rejected the query before acceptance",
    )
}
