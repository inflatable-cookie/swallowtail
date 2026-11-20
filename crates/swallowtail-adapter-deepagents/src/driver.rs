use crate::{
    command::arguments,
    failure::{failure, malformed, protocol_failure, unsupported},
};
use serde_json::{Value, json};
use std::sync::Arc;
use swallowtail_acp_client::{
    AcpActivityConfig, AcpDiagnostics, AcpSessionConfig, AcpSessionOpenContext, AcpTransport,
    ActiveTurnConfig, MessageClassification, PlanProjection, default_permission_namespace,
    open_acp_session,
};
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, CredentialMechanism, DriverDescriptor, DriverRole,
    ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation, HostServiceKind,
    IntegrationFamilyId, OperationShape, PreflightPlan, ResourceAccess, ResourceRepresentation,
    SafeDiagnostic, SessionAccessPolicy, TransportFamilyId,
};
use swallowtail_protocol_acp::{DEFAULT_MAX_BUFFER_BYTES, DEFAULT_MAX_FRAME_BYTES, FramingLimits};
use swallowtail_runtime::{
    BoxFuture, EnvironmentRef, ExecutableRef, HostServices, InteractiveSessionDriver,
    InteractiveSessionHandle, OpenSessionRequest, ProcessHandle, ProcessRequest, ResourceLease,
    ResumeSessionRequest, RuntimeFailure, ScopeId, validate_session_resource_lease,
};

const DRIVER_ID: &str = "swallowtail.deepagents.acp";
const RECEIVE_FRAMING_LIMITS: FramingLimits =
    FramingLimits::new(DEFAULT_MAX_FRAME_BYTES, DEFAULT_MAX_BUFFER_BYTES);

fn deepagents_diagnostics() -> AcpDiagnostics {
    AcpDiagnostics {
        route_name: "deepagents.acp",
        activity: AcpActivityConfig {
            id_prefix: "deepagents-acp",
            namespace_prefix: "deepagents.acp",
            plan_projection: PlanProjection::Ignore,
            message_classification: MessageClassification::Standard,
            malformed,
        },
        malformed,
        protocol_failure,
        version_mismatch: || {
            failure(
                "swallowtail.deepagents.acp.version_mismatch",
                "Deep Agents negotiated an incompatible ACP version",
            )
        },
        connection_closed: || {
            failure(
                "swallowtail.deepagents.acp.connection_closed",
                "Deep Agents ACP connection is closed",
            )
        },
        correlation_limit: || {
            failure(
                "swallowtail.deepagents.acp.correlation_limit",
                "Deep Agents ACP request correlation limit was exceeded",
            )
        },
        session_duplicate: || {
            failure(
                "swallowtail.deepagents.acp.session_duplicate",
                "Deep Agents returned more than one ACP session",
            )
        },
        turn_active: || {
            failure(
                "swallowtail.deepagents.acp.turn_active",
                "Deep Agents session already has an active turn",
            )
        },
        unknown_response: || {
            failure(
                "swallowtail.deepagents.acp.unknown_response",
                "Deep Agents returned an unknown ACP response",
            )
        },
        update_without_turn: || {
            failure(
                "swallowtail.deepagents.acp.update_without_turn",
                "Deep Agents updated a session without an active turn",
            )
        },
        agent_cancel_unsupported: || {
            failure(
                "swallowtail.deepagents.acp.agent_cancel_unsupported",
                "Deep Agents sent an unsupported client cancellation notification",
            )
        },
        notification_unsupported: || {
            failure(
                "swallowtail.deepagents.acp.notification_unsupported",
                "Deep Agents sent an unsupported ACP notification",
            )
        },
        callback_unsupported: || {
            failure(
                "swallowtail.deepagents.acp.callback_unsupported",
                "Deep Agents requested an unsupported ACP client callback",
            )
        },
        permission_without_turn: || {
            failure(
                "swallowtail.deepagents.acp.permission_without_turn",
                "Deep Agents requested permission without an active turn",
            )
        },
        session_mismatch_callback: || {
            failure(
                "swallowtail.deepagents.acp.session_mismatch",
                "Deep Agents callback does not match the active session",
            )
        },
        session_mismatch_update: || {
            failure(
                "swallowtail.deepagents.acp.session_mismatch",
                "Deep Agents update does not match the active session",
            )
        },
        process_cleanup_failed: || {
            SafeDiagnostic::new(
                "swallowtail.deepagents.acp.process_cleanup_failed",
                "Deep Agents process cleanup failed",
            )
        },
        cleanup_missing: || {
            SafeDiagnostic::new(
                "swallowtail.deepagents.acp.cleanup_missing",
                "Deep Agents process cleanup did not complete",
            )
        },
        connection_ended: || {
            failure(
                "swallowtail.deepagents.acp.connection_ended",
                "Deep Agents ACP connection ended",
            )
        },
        output_limit_exceeded: || {
            failure(
                "swallowtail.deepagents.acp.output_limit_exceeded",
                "Deep Agents output exceeded the adapter limit",
            )
        },
        prompt_stopped: || {
            SafeDiagnostic::new(
                "swallowtail.deepagents.acp.prompt_stopped",
                "Deep Agents stopped before completing the turn",
            )
        },
        stop_reason_unsupported: || {
            failure(
                "swallowtail.deepagents.acp.stop_reason_unsupported",
                "Deep Agents returned an unsupported ACP stop reason",
            )
        },
        task_service_missing: || {
            failure(
                "swallowtail.deepagents.acp.task_service_missing",
                "Deep Agents ACP requires a scoped task service",
            )
        },
        task_join_failed: || {
            SafeDiagnostic::new(
                "swallowtail.deepagents.acp.task_join_failed",
                "Deep Agents ACP protocol task did not join cleanly",
            )
        },
        turn_join_failed: || {
            SafeDiagnostic::new(
                "swallowtail.deepagents.acp.turn_join_failed",
                "Deep Agents ACP prompt task did not join cleanly",
            )
        },
        resource_release_failed: || {
            SafeDiagnostic::new(
                "swallowtail.deepagents.acp.resource_release_failed",
                "Deep Agents ACP working-resource service disappeared during cleanup",
            )
        },
        classify_rpc_error: |error| {
            if error.message().contains("not authenticated")
                || error.message().contains("sign in")
                || error.message().contains("API key")
            {
                failure(
                    "swallowtail.deepagents.acp.host_auth_required",
                    "Deep Agents ACP requires host-owned login or API key before session work",
                )
            } else {
                failure(
                    "swallowtail.deepagents.acp.request_rejected",
                    "Deep Agents rejected an ACP request",
                )
            }
        },
        permission_namespace: default_permission_namespace,
        unsupported,
    }
}

/// Low-level interactive driver for the installed Deep Agents ACP agent.
pub struct DeepAgentsAcpDriver {
    isolated_environment: EnvironmentRef,
}

impl DeepAgentsAcpDriver {
    /// Binds the isolated launch environment. Credentials stay host-owned.
    #[must_use]
    pub const fn new(isolated_environment: EnvironmentRef) -> Self {
        Self {
            isolated_environment,
        }
    }

    fn validate_plan(&self, plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
        if plan.driver_identity().id().as_str() != DRIVER_ID {
            return Err(failure(
                "swallowtail.deepagents.acp.plan_driver_mismatch",
                "Preflight plan is bound to a different driver",
            ));
        }
        if plan.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
            || plan.credential_reference().is_some()
            || plan.endpoint_audience().as_str() != crate::DEEPAGENTS_PROVIDER_API_KEY_AUDIENCE
        {
            return Err(failure(
                "swallowtail.deepagents.acp.access_profile_rejected",
                "Deep Agents ACP requires its host-owned provider-api-key profile",
            ));
        }
        if plan.harness_configuration_posture() != Some(HarnessConfigurationPosture::Ambient) {
            return Err(failure(
                "swallowtail.deepagents.acp.configuration_posture_rejected",
                "Deep Agents ACP requires explicit ambient configuration inside its selected environment",
            ));
        }
        if plan.requirements().harness_isolation() != Some(HarnessIsolation::AmbientHost) {
            return Err(failure(
                "swallowtail.deepagents.acp.isolation_rejected",
                "Deep Agents ACP requires explicit ambient-host isolation posture",
            ));
        }
        crate::selection::select_deepagents_acp_plan(plan)
    }
}

/// Describes the installed Deep Agents ACP discovery and session roles.
#[must_use]
pub fn deepagents_acp_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("deepagents-acp").expect("static family id is valid"),
        TransportFamilyId::new("acp-v1-stdio").expect("static transport id is valid"),
    )
    .with_roles([DriverRole::Discovery, DriverRole::InteractiveSession])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([OperationShape::InteractiveSession])
    .with_required_host_services(
        DriverRole::InteractiveSession,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::WorkingResource,
        ],
    )
    .with_required_host_services(
        DriverRole::Discovery,
        [
            HostServiceKind::Task,
            HostServiceKind::Time,
            HostServiceKind::Process,
        ],
    )
    .with_discovery_actions([swallowtail_core::DiscoveryAction::Probe])
    .with_interface_compatibility(crate::deepagents_acp_claim())
}

impl InteractiveSessionDriver for DeepAgentsAcpDriver {
    fn open_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            self.validate_plan(&plan)?;
            services.require_execution_host(plan.execution_host_id())?;
            validate_open(&plan, &request, &services)?;
            let scope = ScopeId::new(format!(
                "deepagents-acp:session:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| malformed())?;
            let resource_service = services
                .working_resource()
                .cloned()
                .expect("validated working-resource service");
            let resource_access = session_resource_access(&plan)?;
            let resource = resource_service
                .resolve(
                    scope.clone(),
                    request
                        .working_resource()
                        .expect("validated resource")
                        .clone(),
                    resource_access,
                    ResourceRepresentation::Filesystem,
                )
                .await?;
            if let Err(error) = validate_session_resource_lease(
                request.access_policy(),
                request.working_resource().expect("validated resource"),
                &resource,
            ) {
                let _ = resource_service.release(resource).await;
                return Err(error);
            }
            let result = self
                .start_session(&plan, &request, &services, scope, resource)
                .await;
            match result {
                Ok(session) => Ok(Box::new(session) as Box<dyn InteractiveSessionHandle>),
                Err(pair) => {
                    let (error, resource) = *pair;
                    let _ = resource_service.release(resource).await;
                    Err(error)
                }
            }
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

impl DeepAgentsAcpDriver {
    async fn start_session(
        &self,
        plan: &PreflightPlan,
        request: &OpenSessionRequest,
        services: &HostServices,
        scope: ScopeId,
        resource: ResourceLease,
    ) -> Result<swallowtail_acp_client::AcpSessionHandle, Box<(RuntimeFailure, ResourceLease)>>
    {
        let cwd = resource
            .filesystem()
            .expect("validated filesystem lease")
            .as_driver_value()
            .to_owned();
        let process_service = services
            .process()
            .cloned()
            .expect("validated process service");
        let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
            plan.instance_target_ref(),
        ))
        .with_arguments(arguments())
        .with_environment([self.isolated_environment.clone()])
        .with_working_resource(
            request
                .working_resource()
                .expect("validated resource")
                .clone(),
        );
        let process: Arc<dyn ProcessHandle> =
            match process_service.start(scope.clone(), process_request).await {
                Ok(process) => Arc::from(process),
                Err(error) => return Err(Box::new((error, resource))),
            };
        let transport = AcpTransport::stdio(process, RECEIVE_FRAMING_LIMITS);
        open_acp_session(
            transport,
            AcpSessionOpenContext {
                scope,
                request_id: request.request_id().clone(),
                execution_host_id: plan.execution_host_id().clone(),
                resource,
                services: services.clone(),
                config: deepagents_session_config(),
            },
            move |connection| async move {
                let initialize = connection.initialize().await?;
                validate_initialize(&initialize)?;
                connection
                    .request("session/new", json!({"cwd": cwd, "mcpServers": []}))
                    .await
                    .and_then(parse_new_session)
            },
        )
        .await
    }
}

fn deepagents_session_config() -> AcpSessionConfig {
    let diagnostics = deepagents_diagnostics();
    AcpSessionConfig::new("deepagents-acp", diagnostics).with_turn_config(
        ActiveTurnConfig::new(diagnostics).with_content_unsupported(|| {
            failure(
                "swallowtail.deepagents.acp.content_unsupported",
                "Deep Agents returned unsupported ACP content",
            )
        }),
    )
}

include!("driver/validation.rs");

#[cfg(test)]
mod driver_tests {
    use super::*;
    use swallowtail_acp_client::{AcpTurnSink, ActiveTurn};
    use swallowtail_runtime::RuntimeTurnId;

    #[test]
    fn non_text_agent_content_reports_content_unsupported() {
        let (turn, _events, _terminal) = ActiveTurn::new(
            RuntimeTurnId::new("deepagents-non-text").expect("valid turn id"),
            "sess-1".to_owned(),
            deepagents_session_config().turn_config,
        )
        .expect("turn creates");
        let err = turn
            .handle_update(&json!({
                "sessionId": "sess-1",
                "update": {
                    "sessionUpdate": "agent_message_chunk",
                    "content": {"type": "image", "mimeType": "image/png", "data": "AAAA"}
                }
            }))
            .expect_err("non-text content must fail");
        assert_eq!(
            err.diagnostic().code(),
            "swallowtail.deepagents.acp.content_unsupported"
        );
    }
}
