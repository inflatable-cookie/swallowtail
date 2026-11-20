use crate::{
    command::arguments,
    failure::{failure, malformed, protocol_failure, unsupported},
    mcp::{KiroAcpRemoteMcpPlacement, production_mcp_servers},
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

const DRIVER_ID: &str = "swallowtail.kiro.acp";
const RECEIVE_FRAMING_LIMITS: FramingLimits =
    FramingLimits::new(DEFAULT_MAX_FRAME_BYTES, DEFAULT_MAX_BUFFER_BYTES);

fn kiro_diagnostics() -> AcpDiagnostics {
    AcpDiagnostics {
        route_name: "kiro.acp",
        activity: AcpActivityConfig {
            id_prefix: "kiro-acp",
            namespace_prefix: "kiro.acp",
            plan_projection: PlanProjection::Ignore,
            message_classification: MessageClassification::Standard,
            malformed,
        },
        malformed,
        protocol_failure,
        version_mismatch: || {
            failure(
                "swallowtail.kiro.acp.version_mismatch",
                "Kiro negotiated an incompatible ACP version",
            )
        },
        connection_closed: || {
            failure(
                "swallowtail.kiro.acp.connection_closed",
                "Kiro ACP connection is closed",
            )
        },
        correlation_limit: || {
            failure(
                "swallowtail.kiro.acp.correlation_limit",
                "Kiro ACP request correlation limit was exceeded",
            )
        },
        session_duplicate: || {
            failure(
                "swallowtail.kiro.acp.session_duplicate",
                "Kiro returned more than one ACP session",
            )
        },
        turn_active: || {
            failure(
                "swallowtail.kiro.acp.turn_active",
                "Kiro session already has an active turn",
            )
        },
        unknown_response: || {
            failure(
                "swallowtail.kiro.acp.unknown_response",
                "Kiro returned an unknown ACP response",
            )
        },
        update_without_turn: || {
            failure(
                "swallowtail.kiro.acp.update_without_turn",
                "Kiro updated a session without an active turn",
            )
        },
        agent_cancel_unsupported: || {
            failure(
                "swallowtail.kiro.acp.agent_cancel_unsupported",
                "Kiro sent an unsupported client cancellation notification",
            )
        },
        notification_unsupported: || {
            failure(
                "swallowtail.kiro.acp.notification_unsupported",
                "Kiro sent an unsupported ACP notification",
            )
        },
        callback_unsupported: || {
            failure(
                "swallowtail.kiro.acp.callback_unsupported",
                "Kiro requested an unsupported ACP client callback",
            )
        },
        permission_without_turn: || {
            failure(
                "swallowtail.kiro.acp.permission_without_turn",
                "Kiro requested permission without an active turn",
            )
        },
        session_mismatch_callback: || {
            failure(
                "swallowtail.kiro.acp.session_mismatch",
                "Kiro callback does not match the active session",
            )
        },
        session_mismatch_update: || {
            failure(
                "swallowtail.kiro.acp.session_mismatch",
                "Kiro update does not match the active session",
            )
        },
        process_cleanup_failed: || {
            SafeDiagnostic::new(
                "swallowtail.kiro.acp.process_cleanup_failed",
                "Kiro process cleanup failed",
            )
        },
        cleanup_missing: || {
            SafeDiagnostic::new(
                "swallowtail.kiro.acp.cleanup_missing",
                "Kiro process cleanup did not complete",
            )
        },
        connection_ended: || {
            failure(
                "swallowtail.kiro.acp.connection_ended",
                "Kiro ACP connection ended",
            )
        },
        output_limit_exceeded: || {
            failure(
                "swallowtail.kiro.acp.output_limit_exceeded",
                "Kiro output exceeded the adapter limit",
            )
        },
        prompt_stopped: || {
            SafeDiagnostic::new(
                "swallowtail.kiro.acp.prompt_stopped",
                "Kiro stopped before completing the turn",
            )
        },
        stop_reason_unsupported: || {
            failure(
                "swallowtail.kiro.acp.stop_reason_unsupported",
                "Kiro returned an unsupported ACP stop reason",
            )
        },
        task_service_missing: || {
            failure(
                "swallowtail.kiro.acp.task_service_missing",
                "Kiro ACP requires a scoped task service",
            )
        },
        task_join_failed: || {
            SafeDiagnostic::new(
                "swallowtail.kiro.acp.task_join_failed",
                "Kiro ACP protocol task did not join cleanly",
            )
        },
        turn_join_failed: || {
            SafeDiagnostic::new(
                "swallowtail.kiro.acp.turn_join_failed",
                "Kiro ACP prompt task did not join cleanly",
            )
        },
        resource_release_failed: || {
            SafeDiagnostic::new(
                "swallowtail.kiro.acp.resource_release_failed",
                "Kiro ACP working-resource service disappeared during cleanup",
            )
        },
        classify_rpc_error: |error| {
            if error.message().contains("not authenticated")
                || error.message().contains("sign in")
                || error.message().contains("API key")
            {
                failure(
                    "swallowtail.kiro.acp.host_auth_required",
                    "Kiro ACP requires host-owned login or API key before session work",
                )
            } else {
                failure(
                    "swallowtail.kiro.acp.request_rejected",
                    "Kiro rejected an ACP request",
                )
            }
        },
        permission_namespace: default_permission_namespace,
        unsupported,
    }
}

/// Low-level interactive driver for the installed Kiro ACP agent.
pub struct KiroAcpDriver {
    isolated_environment: EnvironmentRef,
    http_mcp: Option<KiroAcpRemoteMcpPlacement>,
}

impl KiroAcpDriver {
    /// Binds the isolated launch environment. Credentials stay host-owned.
    #[must_use]
    pub const fn new(isolated_environment: EnvironmentRef) -> Self {
        Self {
            isolated_environment,
            http_mcp: None,
        }
    }

    /// Admits one route-owned streamable-HTTP MCP declaration onto production `session/new`.
    pub fn with_http_mcp_placement(
        self,
        server: KiroAcpRemoteMcpPlacement,
    ) -> Result<Self, RuntimeFailure> {
        let _ = server.to_acp_http_value()?;
        Ok(self.with_prepared_http_mcp(Some(server)))
    }

    pub(crate) fn with_prepared_http_mcp(
        mut self,
        server: Option<KiroAcpRemoteMcpPlacement>,
    ) -> Self {
        self.http_mcp = server;
        self
    }

    fn validate_plan(
        &self,
        plan: &PreflightPlan,
    ) -> Result<crate::selection::KiroPlanSelection, RuntimeFailure> {
        if plan.driver_identity().id().as_str() != DRIVER_ID {
            return Err(failure(
                "swallowtail.kiro.acp.plan_driver_mismatch",
                "Preflight plan is bound to a different driver",
            ));
        }
        if plan.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
            || plan.credential_reference().is_some()
            || plan.endpoint_audience().as_str() != crate::KIRO_LOCAL_ACCOUNT_AUDIENCE
        {
            return Err(failure(
                "swallowtail.kiro.acp.access_profile_rejected",
                "Kiro ACP requires its host-owned local-account profile",
            ));
        }
        if plan.harness_configuration_posture() != Some(HarnessConfigurationPosture::Ambient) {
            return Err(failure(
                "swallowtail.kiro.acp.configuration_posture_rejected",
                "Kiro ACP requires explicit ambient configuration inside its selected environment",
            ));
        }
        if plan.requirements().harness_isolation() != Some(HarnessIsolation::AmbientHost) {
            return Err(failure(
                "swallowtail.kiro.acp.isolation_rejected",
                "Kiro ACP requires explicit ambient-host isolation posture",
            ));
        }
        crate::selection::select_kiro_acp_plan(plan)
    }
}

/// Describes the installed Kiro ACP discovery and session roles.
#[must_use]
pub fn kiro_acp_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("kiro-cli").expect("static family id is valid"),
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
    .with_interface_compatibility(crate::kiro_acp_claim())
}

impl InteractiveSessionDriver for KiroAcpDriver {
    fn open_session(
        &self,
        plan: PreflightPlan,
        request: OpenSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async move {
            let selected = self.validate_plan(&plan)?;
            services.require_execution_host(plan.execution_host_id())?;
            validate_open(&plan, &request, &services)?;
            let scope = ScopeId::new(format!(
                "kiro-acp:session:{}",
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
                .start_session(&plan, &request, &services, scope, resource, selected)
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

impl KiroAcpDriver {
    async fn start_session(
        &self,
        plan: &PreflightPlan,
        request: &OpenSessionRequest,
        services: &HostServices,
        scope: ScopeId,
        resource: ResourceLease,
        selected: crate::selection::KiroPlanSelection,
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
        let http_mcp = self.http_mcp.clone();
        open_acp_session(
            transport,
            AcpSessionOpenContext {
                scope,
                request_id: request.request_id().clone(),
                execution_host_id: plan.execution_host_id().clone(),
                resource,
                services: services.clone(),
                config: kiro_session_config(),
            },
            move |connection| async move {
                let initialize = connection.initialize().await?;
                validate_initialize(&initialize, selected.version())?;
                let mcp_servers = production_mcp_servers(http_mcp.as_ref())?;
                connection
                    .request(
                        "session/new",
                        json!({"cwd": cwd, "mcpServers": mcp_servers}),
                    )
                    .await
                    .and_then(parse_new_session)
            },
        )
        .await
    }
}

fn kiro_session_config() -> AcpSessionConfig {
    let diagnostics = kiro_diagnostics();
    AcpSessionConfig::new("kiro-acp", diagnostics).with_turn_config(
        ActiveTurnConfig::new(diagnostics).with_content_unsupported(|| {
            failure(
                "swallowtail.kiro.acp.content_unsupported",
                "Kiro returned unsupported ACP content",
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
            RuntimeTurnId::new("kiro-non-text").expect("valid turn id"),
            "sess-1".to_owned(),
            kiro_session_config().turn_config,
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
            "swallowtail.kiro.acp.content_unsupported"
        );
    }
}
