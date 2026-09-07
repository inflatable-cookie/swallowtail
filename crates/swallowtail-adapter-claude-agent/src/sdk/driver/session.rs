//! One open Claude Agent SDK sidecar session and its descendant-tree close.

use super::handle::{ClaudeAgentSdkTurnHandle, SessionCancellation, TurnBinding};
use super::startup::SessionReadiness;
use super::validation::validate_turn;
use crate::sdk::bounded::HostBound;
use crate::sdk::connection::SdkConnection;
use crate::sdk::failure::{command_rejected, failure, session_rejected_terminal};
use crate::sdk::profile::{ClaudeAgentSdkPermissionMode, ClaudeAgentSdkSessionProfile};
use crate::sdk::turn::SdkActiveTurn;
use crate::sdk::wire::{ClaudeAgentSdkCommand, ClaudeAgentSdkFailureCode};
use serde_json::json;
use std::sync::{Arc, Mutex};
use swallowtail_runtime::{
    BoxFuture, CancellationControl, CleanupOutcome, CredentialLease, HostServices,
    InteractiveSessionHandle, JoinedTask, RequestId, ResourceLease, RuntimeFailure,
    RuntimeSessionId, ScopeId, SessionResumeBinding, TurnHandle, TurnRequest, WorkingResourceRef,
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
    pub(super) plan: swallowtail_core::PreflightPlan,
    pub(super) working_resource: WorkingResourceRef,
    pub(super) access_policy: swallowtail_core::SessionAccessPolicy,
    pub(super) provider_session_ref: Option<swallowtail_core::SessionRef>,
    pub(super) resume_binding: Option<SessionResumeBinding>,
    pub(super) readiness: SessionReadiness,
    pub(super) active: ActiveSlot,
    /// The last mode the sidecar confirmed, starting at the one it echoed at
    /// open.
    pub(super) permission_mode: ClaudeAgentSdkPermissionMode,
    /// Correlation counter, so each change carries its own single-use id.
    pub(super) permission_mode_changes: u32,
    /// Correlation counter for model changes.
    pub(super) model_changes: u32,
    pub(super) first_turn_rejection: Option<ClaudeAgentSdkFailureCode>,
}

impl InteractiveSessionHandle for ClaudeAgentSdkSessionHandle {
    fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    fn session_id(&self) -> &RuntimeSessionId {
        &self.runtime_id
    }

    fn provider_session_ref(&self) -> Option<&swallowtail_core::SessionRef> {
        self.provider_session_ref.as_ref()
    }

    fn resume_binding(&self) -> Option<&swallowtail_runtime::SessionResumeBinding> {
        self.resume_binding.as_ref()
    }

    fn start_turn<'a>(
        &'a mut self,
        request: TurnRequest,
        services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn TurnHandle>, RuntimeFailure>> {
        Box::pin(async move {
            services.require_execution_host(&self.execution_host_id)?;
            if let Some(original_code) = self.first_turn_rejection {
                return Err(session_rejected_terminal(original_code));
            }
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
                self.readiness.admitted_mcp_tools().to_vec(),
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
                Ok(response) if response.success => {
                    if let Err(error) = self.readiness.confirm_first_turn(response.data.as_ref()) {
                        let original_code = first_turn_rejection_code(&error);
                        return Err(self.reject_first_turn(&turn, original_code, error));
                    }
                    if let Some(provider_session_ref) =
                        self.readiness.provider_session_ref().cloned()
                    {
                        let binding = self.make_resume_binding(provider_session_ref.clone())?;
                        self.provider_session_ref = Some(provider_session_ref);
                        self.resume_binding = Some(binding);
                    }
                    Ok(Box::new(ClaudeAgentSdkTurnHandle::new(
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
                    )) as Box<dyn TurnHandle>)
                }
                Ok(response) => {
                    let code = response
                        .failure_code
                        .expect("a rejected response carries its fixed sidecar code");
                    let (original_code, error) = match code {
                        ClaudeAgentSdkFailureCode::SessionRejectedTerminal => {
                            let original_code = response
                                .original_failure_code
                                .expect("a terminal rejection carries its original code");
                            (original_code, session_rejected_terminal(original_code))
                        }
                        _ => (code, self.query_rejected(code)),
                    };
                    Err(self.reject_first_turn(&turn, original_code, error))
                }
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
    /// Returns the model requested by the plan at open.
    #[must_use]
    pub fn requested_model(&self) -> &str {
        self.readiness.requested_model()
    }

    /// Returns the effective model reported by the SDK `system/init` evidence.
    #[must_use]
    pub fn effective_model(&self) -> &str {
        self.readiness.effective_model()
    }

    /// Returns the readiness evidence stage: initialize-served requested
    /// model support at open, or confirmed after first-turn `system/init`.
    #[must_use]
    pub fn readiness_state(&self) -> &'static str {
        self.readiness.readiness_state()
    }

    /// Returns the bounded model list reported by `Query.supportedModels` at
    /// open. An empty list means the SDK supplied no usable catalogue.
    #[must_use]
    pub fn supported_models(&self) -> &[String] {
        self.readiness.supported_models()
    }

    /// Returns the effort evidence currently available for this session.
    #[must_use]
    pub const fn effort(&self) -> crate::sdk::profile::ClaudeAgentSdkEffortOutcome {
        self.readiness.effort()
    }

    /// Calls `Query.setModel` only for a model in the open-time supported
    /// list. Success requires the sidecar to return that exact model; an
    /// unconfirmed change leaves the previously confirmed model effective.
    pub fn set_model<'a>(
        &'a mut self,
        model: &'a str,
        services: HostServices,
        deadline: swallowtail_runtime::Deadline,
    ) -> BoxFuture<'a, Result<String, RuntimeFailure>> {
        Box::pin(async move {
            services.require_execution_host(&self.execution_host_id)?;
            if self.readiness.effective_model().is_empty()
                || !self
                    .readiness
                    .supported_models()
                    .iter()
                    .any(|supported| supported == model)
            {
                return Err(model_unsupported());
            }
            let bounded = HostBound::new(
                services
                    .time()
                    .cloned()
                    .expect("validated sidecar time service"),
                deadline,
            );
            self.model_changes += 1;
            let id = format!(
                "set-model:{}:{}",
                self.request_id.as_str(),
                self.model_changes
            );
            let Some(response) = bounded
                .run(self.connection.command(
                    id,
                    ClaudeAgentSdkCommand::SetModel,
                    json!({"model": model}),
                ))
                .await
            else {
                return Err(model_change_unconfirmed());
            };
            let response = response?;
            if !response.success {
                // A supported request that the SDK or sidecar rejects has no
                // effective-model confirmation. Preserve the prior effective
                // model and expose the same typed outcome as a void response.
                let _code = response
                    .failure_code
                    .expect("a rejected response carries its fixed sidecar code");
                return Err(model_change_unconfirmed());
            }
            let confirmed = response
                .data
                .as_ref()
                .and_then(|data| data.get("model"))
                .and_then(serde_json::Value::as_str);
            if confirmed != Some(model) {
                return Err(model_change_unconfirmed());
            }
            self.readiness.confirm_model_change(model);
            Ok(model.to_owned())
        })
    }

    /// Returns the observed Node runtime version from the sidecar open.
    #[must_use]
    pub fn node_version(&self) -> &str {
        self.readiness.node_version()
    }

    /// Returns `Qualified` for the exact point or `UnverifiedNewer` for a
    /// newer runtime that passed the sidecar floor.
    #[must_use]
    pub const fn node_version_posture(&self) -> &'static str {
        self.readiness.node_version_posture()
    }

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

    /// Returns per-server MCP connection evidence observed at open.
    ///
    /// Empty when the session was prepared without declared servers. Provider
    /// error text, URLs, and config objects never appear here.
    #[must_use]
    pub fn mcp_server_status(&self) -> &[crate::sdk::mcp::ClaudeAgentSdkMcpServerStatus] {
        self.readiness.mcp_server_status()
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
                return Err(command_rejected(
                    "swallowtail.claude-agent.sdk.permission_mode_rejected",
                    "Claude Agent SDK sidecar rejected the permission-mode change",
                    response
                        .failure_code
                        .expect("a rejected response carries its fixed sidecar code"),
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

    fn reject_first_turn(
        &mut self,
        turn: &Arc<SdkActiveTurn>,
        original_code: ClaudeAgentSdkFailureCode,
        error: RuntimeFailure,
    ) -> RuntimeFailure {
        self.first_turn_rejection = Some(original_code);
        self.reject_turn(turn, error)
    }

    fn query_rejected(&self, code: crate::sdk::wire::ClaudeAgentSdkFailureCode) -> RuntimeFailure {
        match code {
            crate::sdk::wire::ClaudeAgentSdkFailureCode::ResumeCwdMismatch => failure(
                "swallowtail.claude-agent.sdk.resume_cwd_mismatch",
                "Claude Agent SDK resume init did not report the leased working directory",
            ),
            crate::sdk::wire::ClaudeAgentSdkFailureCode::ResumeAccountMismatch => failure(
                "swallowtail.claude-agent.sdk.resume_account_mismatch",
                "Claude Agent SDK resume init did not report the verified first-party account",
            ),
            crate::sdk::wire::ClaudeAgentSdkFailureCode::ResumeSessionUnknown => failure(
                "swallowtail.claude-agent.sdk.resume_session_unknown",
                "Claude Agent SDK could not identify the bound provider session",
            ),
            crate::sdk::wire::ClaudeAgentSdkFailureCode::ResumeBoundaryInvalid => failure(
                "swallowtail.claude-agent.sdk.resume_boundary_invalid",
                "Claude Agent SDK rejected the resume message boundary",
            ),
            _ => command_rejected(
                "swallowtail.claude-agent.sdk.query_rejected",
                "Claude Agent SDK sidecar rejected the query before acceptance",
                code,
            ),
        }
    }

    fn make_resume_binding(
        &self,
        provider_session_ref: swallowtail_core::SessionRef,
    ) -> Result<SessionResumeBinding, RuntimeFailure> {
        let route_id = self.plan.model_route_id().cloned().ok_or_else(|| {
            failure(
                "swallowtail.claude-agent.sdk.resume_route_missing",
                "Claude Agent SDK resumed session has no bound model route",
            )
        })?;
        let model_id = self.plan.model_id().cloned().ok_or_else(|| {
            failure(
                "swallowtail.claude-agent.sdk.resume_model_missing",
                "Claude Agent SDK resumed session has no bound model",
            )
        })?;
        Ok(SessionResumeBinding::new(
            provider_session_ref,
            self.plan.instance_id().clone(),
            self.execution_host_id.clone(),
            route_id,
            model_id,
            self.working_resource.clone(),
            self.access_policy.clone(),
        ))
    }
}

fn first_turn_rejection_code(error: &RuntimeFailure) -> ClaudeAgentSdkFailureCode {
    match error.diagnostic().code() {
        "swallowtail.claude-agent.sdk.init_missing" => ClaudeAgentSdkFailureCode::InitMissing,
        "swallowtail.claude-agent.sdk.cwd_mismatch" => ClaudeAgentSdkFailureCode::CwdMismatch,
        "swallowtail.claude-agent.sdk.open_mismatch" => ClaudeAgentSdkFailureCode::ModelMismatch,
        "swallowtail.claude-agent.sdk.model_missing" => ClaudeAgentSdkFailureCode::ModelMissing,
        "swallowtail.claude-agent.sdk.supported_model_rejected" => {
            ClaudeAgentSdkFailureCode::SupportedModelRejected
        }
        "swallowtail.claude-agent.sdk.effort_unconfirmed" => {
            ClaudeAgentSdkFailureCode::EffortUnconfirmed
        }
        "swallowtail.claude-agent.sdk.capabilities_invalid" => {
            ClaudeAgentSdkFailureCode::CapabilitiesInvalid
        }
        "swallowtail.claude-agent.sdk.resume_cwd_mismatch" => {
            ClaudeAgentSdkFailureCode::ResumeCwdMismatch
        }
        "swallowtail.claude-agent.sdk.resume_account_mismatch" => {
            ClaudeAgentSdkFailureCode::ResumeAccountMismatch
        }
        "swallowtail.claude-agent.sdk.resume_session_unknown" => {
            ClaudeAgentSdkFailureCode::ResumeSessionUnknown
        }
        _ => ClaudeAgentSdkFailureCode::CommandFailed,
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

fn model_unsupported() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.model_unsupported",
        "Claude Agent SDK model was not present in the open-time supported model list",
    )
}

fn model_change_unconfirmed() -> RuntimeFailure {
    failure(
        "swallowtail.claude-agent.sdk.model_change_unconfirmed",
        "Claude Agent SDK sidecar did not confirm the requested model change",
    )
}
