use crate::connection::AcpConnection;
use crate::failure::{failure, malformed, unsupported};
use crate::turn::ActiveTurn;
use serde_json::{Value, json};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::{
    CancellationScope, CredentialMechanism, HarnessConfigurationPosture, HarnessIsolation,
    PreflightPlan, ResourceAccess, ResourceRepresentation, SessionAccessPolicy,
    SessionProviderStatePolicy, SessionRef, SupportAuthority,
};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CancellationAcknowledgement, CancellationControl, CleanupOutcome,
    ExecutableRef, HostServices, InteractiveSessionDriver, InteractiveSessionHandle, JoinedTask,
    OpenSessionRequest, ProcessHandle, ProcessRequest, RequestId, ResourceLease,
    ResumeSessionRequest, RuntimeFailure, RuntimeSessionId, ScopeId, TerminalOutcome, TurnHandle,
    TurnRequest, validate_session_resource_lease,
};

use crate::CursorAcpDriver;

const DRIVER_ID: &str = "swallowtail.cursor-agent.acp";
const AUTH_METHOD: &str = "cursor_login";

impl CursorAcpDriver {
    fn validate_plan(
        &self,
        plan: &PreflightPlan,
    ) -> Result<crate::selection::CursorPlanSelection, RuntimeFailure> {
        if plan.driver_identity().id().as_str() != DRIVER_ID {
            return Err(failure(
                "swallowtail.cursor.acp.plan_driver_mismatch",
                "Preflight plan is bound to a different driver",
            ));
        }
        if plan.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
            || plan.credential_reference().is_some()
            || plan.endpoint_audience().as_str() != crate::CURSOR_SUBSCRIPTION_AUDIENCE
            || plan.access_status().support_authority() != SupportAuthority::ProviderSupported
        {
            return Err(failure(
                "swallowtail.cursor.acp.access_profile_rejected",
                "Cursor ACP requires its delegated local-login subscription profile",
            ));
        }
        if plan.harness_configuration_posture() != Some(HarnessConfigurationPosture::Ambient) {
            return Err(failure(
                "swallowtail.cursor.acp.configuration_posture_rejected",
                "Cursor Agent requires explicit ambient harness configuration",
            ));
        }
        if plan.requirements().harness_isolation() != Some(HarnessIsolation::AmbientHost) {
            return Err(failure(
                "swallowtail.cursor.acp.isolation_rejected",
                "Cursor Agent requires explicit ambient-host isolation",
            ));
        }
        if plan.requirements().operation_shape()
            != swallowtail_core::OperationShape::InteractiveSession
            || plan.requirements().session_provider_state_policy()
                != Some(SessionProviderStatePolicy::DurableProviderSessionPreserved)
        {
            return Err(failure(
                "swallowtail.cursor.acp.provider_state_rejected",
                "Cursor ACP requires preserved durable provider-session state",
            ));
        }
        if plan.provider_id().is_some()
            || plan.model_id().is_some()
            || plan.model_route_id().is_some()
        {
            return Err(failure(
                "swallowtail.cursor.acp.model_rejected",
                "Cursor ACP model selection is not qualified on this route",
            ));
        }
        crate::selection::select_cursor_acp_plan(plan)
    }
}

impl InteractiveSessionDriver for CursorAcpDriver {
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

impl CursorAcpDriver {
    async fn start_session(
        &self,
        plan: &PreflightPlan,
        request: &OpenSessionRequest,
        services: &HostServices,
    ) -> Result<CursorSessionHandle, RuntimeFailure> {
        let _selected = self.validate_plan(plan)?;
        services.require_execution_host(plan.execution_host_id())?;
        validate_open(plan, request, services)?;
        let mut attachment = self.start_attachment(plan, request, services).await?;
        let opened = async {
            let initialize = attachment.connection.initialize().await?;
            validate_initialize(&initialize)?;
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
            Ok::<_, RuntimeFailure>(provider_id)
        }
        .await;
        let provider_id = match opened {
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
                RuntimeSessionId::new(format!("cursor-acp:{}", request.request_id().as_str()))
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
            services,
        ))
    }
}

include!("driver/attachment.rs");
include!("driver/session.rs");
include!("driver/validation.rs");
