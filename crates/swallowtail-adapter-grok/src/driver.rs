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
    NegotiatedSessionModelOptions, OpenSessionRequest, ProcessHandle, ProcessRequest, RequestId,
    ResourceLease, ResumeSessionRequest, RuntimeFailure, RuntimeSessionId, ScopeId,
    SessionResumeBinding, TerminalOutcome, TurnHandle, TurnRequest,
    validate_session_plan_agreement, validate_session_resource_lease,
};

use crate::GrokAcpDriver;

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
            services.require_execution_host(plan.execution_host_id())?;
            validate_recovery(&plan, &request, &services)?;
            let mut attachment = self
                .start_attachment(
                    &plan,
                    request.request_id(),
                    request.working_resource(),
                    request.access_policy(),
                    &services,
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
        let working_resource = request
            .working_resource()
            .expect("validated working resource")
            .clone();
        let access_policy = request.access_policy().clone();
        let mut attachment = self
            .start_attachment(
                plan,
                request.request_id(),
                &working_resource,
                &access_policy,
                services,
            )
            .await?;
        let opened = async {
            let initialize = attachment.connection.initialize().await?;
            let model_options = validate_initialize(
                &initialize,
                selected.version(),
                selected.expected_model(),
            )?;
            attachment.connection.activate_cached_token().await?;
            let response = attachment
                .connection
                .request(
                    "session/new",
                    json!({"cwd": attachment.cwd, "mcpServers": []}),
                )
                .await?;
            let provider_id = response
                .get("sessionId")
                .and_then(Value::as_str)
                .ok_or_else(malformed)?
                .to_owned();
            Ok::<_, RuntimeFailure>((provider_id, model_options))
        }
        .await;
        let (provider_id, model_options) = match opened {
            Ok(opened) => opened,
            Err(error) => {
                let _ = attachment.abort(services).await;
                return Err(error);
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
                let _ = attachment.abort(services).await;
                return Err(error);
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
            },
            services,
        ))
    }
}

include!("driver/attachment.rs");
include!("driver/session.rs");
include!("driver/run.rs");
include!("driver/validation.rs");

#[cfg(test)]
mod tests;
