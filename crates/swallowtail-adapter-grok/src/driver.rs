use crate::connection::AcpConnection;
use crate::failure::{failure, malformed, unsupported};
use crate::turn::ActiveTurn;
use serde_json::{Value, json};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, CredentialMechanism,
    HarnessConfigurationPosture, HarnessIsolation, PreflightPlan, ResourceAccess,
    ResourceRepresentation, SessionAccessPolicy, SessionProviderStatePolicy, SessionRef,
};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CancellationAcknowledgement, CancellationControl, CleanupOutcome,
    CredentialLease, ExecutableRef, HostServices, InteractiveSessionDriver,
    InteractiveSessionHandle, JoinedTask, NegotiatedSessionModelOption,
    NegotiatedSessionModelOptions, OpenSessionRequest, ProcessHandle, ProcessRequest,
    RegisteredToolCleanupCause, RequestId, ResourceLease, ResumeSessionRequest, RuntimeFailure,
    RuntimeSessionId, ScopeId, SessionCleanupRequest, SessionResumeBinding, TerminalOutcome,
    TurnHandle, TurnRequest, validate_session_plan_agreement, validate_session_resource_lease,
};

use crate::GrokAcpDriver;
use crate::registered_tool::{PendingRegisteredOpen, prepare_registered};

const DRIVER_ID: &str = "swallowtail.grok-build.acp";
const AUTH_METHOD: &str = "cached_token";

impl GrokAcpDriver {
    fn validate_plan(
        &self,
        plan: &PreflightPlan,
    ) -> Result<crate::selection::GrokPlanSelection, RuntimeFailure> {
        if plan.driver_identity().id().as_str() != DRIVER_ID {
            return Err(failure(
                "swallowtail.grok.acp.plan_driver_mismatch",
                "Preflight plan is bound to a different driver",
            ));
        }
        if plan.credential_mechanism() != &CredentialMechanism::InteractiveOauth
            || plan.credential_reference() != Some(self.credential())
            || plan.endpoint_audience().as_str() != crate::GROK_BUILD_SUBSCRIPTION_AUDIENCE
        {
            return Err(failure(
                "swallowtail.grok.acp.access_profile_rejected",
                "Grok Build requires its delegated subscription OAuth profile",
            ));
        }
        if plan.harness_configuration_posture() != Some(HarnessConfigurationPosture::Ambient) {
            return Err(failure(
                "swallowtail.grok.acp.configuration_posture_rejected",
                "Grok Build requires explicit ambient harness configuration",
            ));
        }
        if plan.requirements().harness_isolation() != Some(HarnessIsolation::AmbientHost) {
            return Err(failure(
                "swallowtail.grok.acp.isolation_rejected",
                "Grok Build requires explicit ambient-host isolation",
            ));
        }
        if plan.requirements().operation_shape()
            == swallowtail_core::OperationShape::InteractiveSession
        {
            if plan.requirements().session_provider_state_policy()
                != Some(SessionProviderStatePolicy::DurableProviderSessionPreserved)
            {
                return Err(failure(
                    "swallowtail.grok.acp.provider_state_rejected",
                    "Grok Build requires preserved durable provider-session state",
                ));
            }
        } else if plan.requirements().operation_shape()
            != swallowtail_core::OperationShape::StructuredRun
            || !plan.requirements().capabilities().any(|requirement| {
                requirement.capability() == swallowtail_core::Capability::ProviderDurableRetention
            })
        {
            return Err(failure(
                "swallowtail.grok.acp.provider_state_rejected",
                "Grok Build requires explicit durable provider retention",
            ));
        }
        let selected = crate::selection::select_grok_acp_plan(plan)?;
        if plan
            .model_id()
            .is_none_or(|model| model.as_str() != selected.expected_model())
        {
            return Err(failure(
                "swallowtail.grok.acp.model_rejected",
                "Grok Build requires the preflight-bound qualified model",
            ));
        }
        Ok(selected)
    }
}

impl InteractiveSessionDriver for GrokAcpDriver {
    fn open_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            Ok(
                Box::new(self.start_session(&plan, &request, &services).await?)
                    as Box<dyn InteractiveSessionHandle>,
            )
        })
    }

    fn resume_session(
        &self,
        _plan: PreflightPlan,
        _request: ResumeSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async { Err(unsupported("session resume")) })
    }

    fn recover_session_attachment(
        &self,
        plan: PreflightPlan,
        request: ResumeSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            validate_session_plan_agreement(&plan, request.plan_agreement())?;
            let selected = self.validate_plan(&plan)?;
            let permission_handling = permission_handling(&plan)?;
            services.require_execution_host(plan.execution_host_id())?;
            validate_recovery(&plan, &request, &services)?;
            let mut attachment = self
                .start_attachment(
                    &plan,
                    request.request_id(),
                    request.working_resource(),
                    request.access_policy(),
                    &services,
                    &mut OpenBound::unbounded(),
                )
                .await?;
            let recovered = async {
                let initialize = attachment.connection.initialize().await?;
                let model_options = validate_initialize(
                    &initialize,
                    selected.version(),
                    selected.expected_model(),
                )?;
                attachment.connection.activate_cached_token().await?;
                let provider_ref = request.provider_session_ref().clone();
                let response = attachment
                    .connection
                    .recover_session_attachment(provider_ref.clone(), attachment.cwd.clone())
                    .await?;
                if response.get("sessionId").and_then(Value::as_str)
                    != Some(provider_ref.as_provider_value())
                {
                    return Err(failure(
                        "swallowtail.grok.acp.attachment_recovery_response_mismatch",
                        "Grok Build attached a different provider session",
                    ));
                }
                let runtime_id =
                    RuntimeSessionId::new(format!("grok-acp:{}", request.request_id().as_str()))
                        .map_err(|_| malformed())?;
                Ok((runtime_id, provider_ref, model_options))
            }
            .await;
            match recovered {
                Ok((runtime_id, provider_ref, model_options)) => {
                    let provider_id = provider_ref.as_provider_value().to_owned();
                    Ok(Box::new(attachment.into_session(
                        GrokSessionInput {
                            request_id: request.request_id().clone(),
                            runtime_id,
                            provider_ref,
                            provider_id,
                            binding: request.resume_binding().clone(),
                            model_options,
                            permission_handling,
                            registered: None,
                        },
                        &services,
                    )) as Box<dyn InteractiveSessionHandle>)
                }
                Err(error) => {
                    let _ = attachment.abort(&services).await;
                    Err(error)
                }
            }
        })
    }
}

impl GrokAcpDriver {
    async fn start_session(
        &self,
        plan: &PreflightPlan,
        request: &OpenSessionRequest,
        services: &HostServices,
    ) -> Result<GrokSessionHandle, RuntimeFailure> {
        let selected = self.validate_plan(plan)?;
        services.require_execution_host(plan.execution_host_id())?;
        validate_open(plan, request, services)?;
        let permission_handling = permission_handling(plan)?;
        let working_resource = request
            .working_resource()
            .expect("validated working resource")
            .clone();
        let access_policy = request.access_policy().clone();
        // A registered open is bounded end to end, so a provider that holds
        // stdio open without answering cannot strand a minted lease, its
        // listener, or the route's own resources.
        let registered_deadline = match self.registered_tools() {
            Some(binding) => Some(validate_registered_open(binding, services)?),
            None => None,
        };
        // The lease, listener, and rendezvous are minted before the provider
        // process starts, so a registered open that cannot be admitted never
        // reaches a Grok spawn and has nothing to abort.
        let mut registered = match self.registered_tools() {
            Some(binding) => {
                Some(prepare_registered(binding, plan, request.request_id(), services).await?)
            }
            None => None,
        };
        // One bound covers the whole minted-open lifecycle, not just the ACP
        // exchanges: credential acquisition, resource resolution, and process
        // startup are inside it too, so nothing can stay pending forever while
        // a lease and its listener are already held.
        let mut bound = match registered_deadline {
            Some(deadline) => OpenBound::until(
                services
                    .time()
                    .expect("validated registered-tool time service")
                    .wait_until(deadline),
            ),
            None => OpenBound::unbounded(),
        };
        let mut attachment = match self
            .start_attachment(
                plan,
                request.request_id(),
                &working_resource,
                &access_policy,
                services,
                &mut bound,
            )
            .await
        {
            Ok(attachment) => attachment,
            Err(error) => {
                let cleanup =
                    abandon_registered(registered.take(), services, cause_for(&error)).await;
                return Err(surface_cleanup_failure(error, cleanup));
            }
        };
        let opened = async {
            let initialize = bound.run(attachment.connection.initialize()).await?;
            let model_options =
                validate_initialize(&initialize, selected.version(), selected.expected_model())?;
            bound
                .run(attachment.connection.activate_cached_token())
                .await?;
            // Omission stays byte-identical: without a registered binding this
            // is the same empty list the merged route sends.
            let mcp_servers = registered.as_ref().map_or_else(
                || json!([]),
                |pending| json!([pending.declaration().to_acp_value()]),
            );
            let response = bound
                .run(attachment.connection.request(
                    "session/new",
                    json!({"cwd": attachment.cwd, "mcpServers": mcp_servers}),
                ))
                .await?;
            let provider_id = response
                .get("sessionId")
                .and_then(Value::as_str)
                .ok_or_else(malformed)?
                .to_owned();
            // Grok spawns the declared courier from `session/new`, so readiness
            // is only observable after the provider answered it.
            if let Some(pending) = registered.as_mut() {
                pending.wait_until_ready()?;
            }
            Ok::<_, RuntimeFailure>((provider_id, model_options))
        }
        .await;
        let (provider_id, model_options) = match opened {
            Ok(opened) => opened,
            Err(error) => {
                // Registered cleanup runs before the route releases its own
                // leases, so a retained bridge lease is never masked by a clean
                // resource or credential release.
                let cleanup =
                    abandon_registered(registered.take(), services, cause_for(&error)).await;
                abort_after_registered(&mut attachment, services, &cleanup).await;
                return Err(surface_cleanup_failure(error, cleanup));
            }
        };
        let identities = (|| {
            attachment.connection.set_session_id(provider_id.clone())?;
            let provider_ref = SessionRef::new(&provider_id).map_err(|_| malformed())?;
            let runtime_id =
                RuntimeSessionId::new(format!("grok-acp:{}", request.request_id().as_str()))
                    .map_err(|_| malformed())?;
            Ok::<_, RuntimeFailure>((provider_ref, runtime_id))
        })();
        let (provider_ref, runtime_id) = match identities {
            Ok(identities) => identities,
            Err(error) => {
                let cleanup = abandon_registered(
                    registered.take(),
                    services,
                    RegisteredToolCleanupCause::ProviderFailure,
                )
                .await;
                abort_after_registered(&mut attachment, services, &cleanup).await;
                return Err(surface_cleanup_failure(error, cleanup));
            }
        };
        let binding = SessionResumeBinding::new(
            provider_ref.clone(),
            plan.instance_id().clone(),
            plan.execution_host_id().clone(),
            plan.model_route_id().expect("validated route").clone(),
            plan.model_id().expect("validated model").clone(),
            working_resource,
            access_policy,
        );
        Ok(attachment.into_session(
            GrokSessionInput {
                request_id: request.request_id().clone(),
                runtime_id,
                provider_ref,
                provider_id,
                binding,
                model_options,
                permission_handling,
                registered: registered.map(|pending| Arc::new(pending.claim())),
            },
            services,
        ))
    }
}

/// Closes a registered lease that never reached an open session.
async fn abandon_registered(
    pending: Option<PendingRegisteredOpen>,
    services: &HostServices,
    cause: RegisteredToolCleanupCause,
) -> CleanupOutcome {
    match pending {
        Some(pending) => pending.abandon(services, cause).await,
        None => CleanupOutcome::NotApplicable,
    }
}

/// Aborts the attachment, retaining its leases when registered cleanup failed.
///
/// A retained bridge lease means the host still holds work it could not join.
/// Reporting that diagnostic is not enough: the working resource and the
/// credential must stay held rather than be returned for reuse.
async fn abort_after_registered(
    attachment: &mut PendingAttachment,
    services: &HostServices,
    cleanup: &CleanupOutcome,
) {
    if matches!(
        cleanup,
        CleanupOutcome::Failed(_) | CleanupOutcome::Degraded(_)
    ) {
        let _ = attachment.abort_retaining(services).await;
    } else {
        let _ = attachment.abort(services).await;
    }
}

/// Maps one open failure onto the exact Contract 063 cleanup cause.
fn cause_for(error: &RuntimeFailure) -> RegisteredToolCleanupCause {
    if error.diagnostic().code() == REGISTERED_OPEN_DEADLINE_CODE {
        RegisteredToolCleanupCause::Deadline
    } else {
        RegisteredToolCleanupCause::ProviderFailure
    }
}

/// Reports a failed registered cleanup rather than the failure that hid it.
///
/// A retained lease is the more serious truth: an open that also failed to
/// close its bridge lease must never surface as the ordinary provider error.
fn surface_cleanup_failure(error: RuntimeFailure, cleanup: CleanupOutcome) -> RuntimeFailure {
    match cleanup {
        CleanupOutcome::Failed(diagnostic) | CleanupOutcome::Degraded(diagnostic) => {
            RuntimeFailure::new(diagnostic)
        }
        CleanupOutcome::Clean | CleanupOutcome::NotApplicable => error,
    }
}

/// Safe code reported when the registered-open deadline expires.
const REGISTERED_OPEN_DEADLINE_CODE: &str = "swallowtail.grok.acp.registered_tool.open_deadline";

/// Requires the services and unelapsed deadline a bounded registered open needs.
fn validate_registered_open(
    binding: &crate::registered_tool::GrokRegisteredToolBinding,
    services: &HostServices,
) -> Result<swallowtail_runtime::Deadline, RuntimeFailure> {
    let deadline = binding.require_deadline()?;
    binding.require_turn()?;
    let time = services.time().ok_or_else(|| {
        failure(
            "swallowtail.grok.acp.registered_tool.time_service_missing",
            "Grok Build ACP registered-tool open requires a host time service to bound it",
        )
    })?;
    if time.now() >= deadline.instant() {
        return Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
            REGISTERED_OPEN_DEADLINE_CODE,
            "Grok Build ACP registered-tool deadline elapsed before provider work",
        )));
    }
    Ok(deadline)
}

/// One deadline shared by every step of a bounded open.
///
/// Each step is raced individually rather than the whole sequence being
/// dropped on expiry, so a step that owns partial resources still runs its own
/// cleanup instead of being cancelled mid-flight. Without a registered binding
/// there is no deadline and every step is awaited exactly as the merged route
/// awaits it.
struct OpenBound {
    deadline:
        Option<swallowtail_runtime::BoxFuture<'static, swallowtail_runtime::DeadlineObservation>>,
    expired: bool,
}

impl OpenBound {
    const fn unbounded() -> Self {
        Self {
            deadline: None,
            expired: false,
        }
    }

    const fn until(
        deadline: swallowtail_runtime::BoxFuture<'static, swallowtail_runtime::DeadlineObservation>,
    ) -> Self {
        Self {
            deadline: Some(deadline),
            expired: false,
        }
    }

    async fn run<T>(
        &mut self,
        step: impl std::future::Future<Output = Result<T, RuntimeFailure>>,
    ) -> Result<T, RuntimeFailure> {
        if self.expired {
            return Err(registered_open_expired());
        }
        let Some(deadline) = self.deadline.as_mut() else {
            return step.await;
        };
        let mut step = std::pin::pin!(step);
        let mut fired = false;
        let result = std::future::poll_fn(|context| {
            use std::task::Poll;
            if let Poll::Ready(result) = step.as_mut().poll(context) {
                Poll::Ready(Some(result))
            } else if deadline.as_mut().poll(context).is_ready() {
                fired = true;
                Poll::Ready(None)
            } else {
                Poll::Pending
            }
        })
        .await;
        if fired {
            self.expired = true;
        }
        result.unwrap_or_else(|| Err(registered_open_expired()))
    }
}

fn registered_open_expired() -> RuntimeFailure {
    RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
        REGISTERED_OPEN_DEADLINE_CODE,
        "Grok Build ACP registered-tool open exceeded its deadline",
    ))
}

include!("driver/attachment.rs");
include!("driver/session.rs");
include!("driver/run.rs");
include!("driver/validation.rs");

#[cfg(test)]
mod tests;
