use crate::connection::AcpConnection;
use crate::failure::{failure, malformed, unsupported};
use crate::turn::ActiveTurn;
use serde_json::{Value, json};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::{
    CancellationScope, CredentialMechanism, HarnessConfigurationPosture, HarnessIsolation,
    PreflightPlan, ResourceAccess, ResourceRepresentation, SessionAccessPolicy,
    SessionProviderStatePolicy, SessionRef,
};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CancellationAcknowledgement, CancellationControl, CleanupOutcome,
    CredentialLease, ExecutableRef, HostServices, InteractiveSessionDriver,
    InteractiveSessionHandle, JoinedTask, NegotiatedSessionModelOption,
    NegotiatedSessionModelOptions, OpenSessionRequest, ProcessHandle, ProcessRequest, RequestId,
    ResourceLease, ResumeSessionRequest, RuntimeFailure, RuntimeSessionId, ScopeId,
    TerminalOutcome, TurnHandle, TurnRequest, validate_session_resource_lease,
};

use crate::GrokAcpDriver;

const DRIVER_ID: &str = "swallowtail.grok-build.acp";
const EXPECTED_MODEL: &str = "grok-4.5";
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
        if plan
            .model_id()
            .is_none_or(|model| model.as_str() != EXPECTED_MODEL)
        {
            return Err(failure(
                "swallowtail.grok.acp.model_rejected",
                "Grok Build requires the preflight-bound qualified model",
            ));
        }
        crate::selection::select_grok_acp_plan(plan)
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
        let mut attachment = self.start_attachment(plan, request, services).await?;
        let opened = async {
            let initialize = attachment.connection.initialize().await?;
            let model_options =
                validate_initialize(&initialize, selected.version(), EXPECTED_MODEL)?;
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
        Ok(attachment.into_session(
            request.request_id().clone(),
            runtime_id,
            provider_ref,
            provider_id,
            model_options,
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
