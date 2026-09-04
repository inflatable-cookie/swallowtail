//! One open Claude Agent SDK sidecar session and its descendant-tree close.

use super::handle::{ClaudeAgentSdkTurnHandle, SessionCancellation, TurnBinding};
use super::startup::SessionReadiness;
use super::validation::validate_turn;
use crate::sdk::bounded::HostBound;
use crate::sdk::connection::SdkConnection;
use crate::sdk::failure::failure;
use crate::sdk::profile::{ClaudeAgentSdkPermissionMode, ClaudeAgentSdkSessionProfile};
use crate::sdk::turn::SdkActiveTurn;
use crate::sdk::wire::ClaudeAgentSdkCommand;
use serde_json::json;
use std::sync::{Arc, Mutex};
use swallowtail_runtime::{
    BoxFuture, CancellationControl, CleanupOutcome, CredentialLease, HostServices,
    InteractiveSessionHandle, JoinedTask, RequestId, ResourceLease, RuntimeFailure,
    RuntimeSessionId, ScopeId, TurnHandle, TurnRequest,
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

/// One open Claude Agent SDK sidecar session.
///
/// This is the route-local handle. It carries the whole shared
/// `InteractiveSessionHandle` surface and adds mid-session permission-mode
/// control, which no provider-neutral trait declares.
pub struct ClaudeAgentSdkSessionHandle {
    pub(super) request_id: RequestId,
    pub(super) runtime_id: RuntimeSessionId,
    pub(super) execution_host_id: swallowtail_core::ExecutionHostId,
    pub(super) connection: Arc<SdkConnection>,
    pub(super) cancellation: SessionCancellation,
    pub(super) pump_task: Option<Box<dyn JoinedTask>>,
    /// The enclosing cleanup guardian, already started under a reservation
    /// taken before this session acquired a credential, a resource, a process,
    /// or a task. Activating it later is infallible, and dropping it is a host
    /// handoff rather than a synchronous join.
    pub(super) close_guardian: Option<crate::sdk::guardian::SessionGuardian>,
    pub(super) services: HostServices,
    pub(super) resource: Option<ResourceLease>,
    pub(super) credential: Option<CredentialLease>,
    pub(super) readiness: SessionReadiness,
    pub(super) active: ActiveSlot,
    /// The last mode the sidecar confirmed, starting at the one it echoed at
    /// open.
    pub(super) permission_mode: ClaudeAgentSdkPermissionMode,
    /// Correlation counter, so each change carries its own single-use id.
    pub(super) permission_mode_changes: u32,
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
                self.readiness.profile(),
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
        // Ownership moves first. The guardian takes the connection, process,
        // pump, remaining turn-deadline task, and both leases here, before the
        // public cleanup future exists at all, so the runtime refusing that
        // future or the caller dropping it cannot strand any of them.
        let guardian = close::activate(&mut self, deadline, true);
        let settle_services = services.clone();
        // One deadline, applied by the shared cleanup bound and again inside
        // every stage so no single stage can consume the whole budget.
        swallowtail_runtime::bound_session_cleanup(
            execution_host_id,
            request,
            services,
            Box::pin(async move { close::settle(guardian, &settle_services, deadline).await }),
        )
    }
}

/// A session dropped without close still hands its whole state to the enclosing
/// guardian rather than dropping a live process, pump, and two leases outside
/// any ordered continuation.
///
/// There is no caller deadline on this path, so the guardian skips the
/// cooperative stages and goes straight to the host termination request and the
/// ordered release. Its own drop then transfers the guardian to the owning host
/// instead of joining it on the dropping thread.
impl Drop for ClaudeAgentSdkSessionHandle {
    fn drop(&mut self) {
        if self.close_guardian.is_none() {
            return;
        }
        let now = self.services.time().map_or_else(
            || swallowtail_runtime::MonotonicInstant::from_ticks(0),
            |time| time.now(),
        );
        drop(close::activate(
            self,
            swallowtail_runtime::Deadline::at(now),
            false,
        ));
    }
}

impl ClaudeAgentSdkSessionHandle {
    /// Returns the admitted tool set and current effective permission mode.
    #[must_use]
    pub const fn session_profile(&self) -> ClaudeAgentSdkSessionProfile {
        self.readiness
            .profile()
            .with_permission_mode(self.permission_mode)
    }

    /// Returns the effective permission mode this session is running under.
    ///
    /// At open this is the confirmed value the sidecar echoed. After a
    /// successful change it is the value the sidecar confirmed for that
    /// change. It is never a value this side merely requested.
    #[must_use]
    pub const fn permission_mode(&self) -> ClaudeAgentSdkPermissionMode {
        self.permission_mode
    }

    /// Changes the permission mode of this live session and returns the mode
    /// the sidecar confirmed.
    ///
    /// The admitted tool set never widens here: only the three modes this
    /// route represents are reachable, and an auto-approving upstream mode is
    /// unrepresentable. The whole exchange is raced against the caller's
    /// deadline, and a rejected, unanswered, or differently-answered change is
    /// a typed failure — this never reports success on an unconfirmed change.
    pub fn set_permission_mode<'a>(
        &'a mut self,
        mode: ClaudeAgentSdkPermissionMode,
        services: HostServices,
        deadline: swallowtail_runtime::Deadline,
    ) -> BoxFuture<'a, Result<ClaudeAgentSdkPermissionMode, RuntimeFailure>> {
        Box::pin(async move {
            services.require_execution_host(&self.execution_host_id)?;
            let bounded = HostBound::new(
                services
                    .time()
                    .cloned()
                    .expect("validated sidecar time service"),
                deadline,
            );
            self.permission_mode_changes += 1;
            let id = format!(
                "set-permission-mode:{}:{}",
                self.request_id.as_str(),
                self.permission_mode_changes
            );
            let Some(response) = bounded
                .run(self.connection.command(
                    id,
                    ClaudeAgentSdkCommand::SetPermissionMode,
                    json!({"mode": mode.as_str()}),
                ))
                .await
            else {
                return Err(permission_mode_unconfirmed());
            };
            let response = response?;
            if !response.success {
                return Err(failure(
                    "swallowtail.claude-agent.sdk.permission_mode_rejected",
                    "Claude Agent SDK sidecar rejected the permission-mode change",
                ));
            }
            // The confirmation is the sidecar's own echo of the mode it
            // applied. A missing or different echo is an unconfirmed change,
            // never a silent success.
            let confirmed = response
                .data
                .as_ref()
                .and_then(|data| data.get("permissionMode"))
                .and_then(serde_json::Value::as_str);
            if confirmed != Some(mode.as_str()) {
                return Err(permission_mode_unconfirmed());
            }
            self.permission_mode = mode;
            Ok(mode)
        })
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

fn turn_deadline_elapsed() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.turn_deadline_elapsed",
        "Claude Agent SDK sidecar turn reached its host deadline",
    )
}

fn permission_mode_unconfirmed() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.permission_mode_unconfirmed",
        "Claude Agent SDK sidecar did not confirm the requested permission mode",
    )
}

fn query_rejected() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.query_rejected",
        "Claude Agent SDK sidecar rejected the query before acceptance",
    )
}
