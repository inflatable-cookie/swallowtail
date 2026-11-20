use super::{
    command::arguments,
    failure::{failure, malformed, protocol_failure, unsupported},
    mcp::{
        OpenCodeAcpRemoteMcpPlacement, OpenCodeAcpStdioMcpServer, entry_conflict,
        production_mcp_servers,
    },
};
use serde_json::{Value, json};
use std::sync::Arc;
use swallowtail_acp_client::{
    AcpActivityConfig, AcpConnection, AcpConnectionHooks, AcpDiagnostics, AcpSessionConfig,
    AcpSessionHandle, AcpSessionOpenContext, AcpTransport, ActiveTurnConfig, MessageClassification,
    PlanProjection, default_permission_namespace, open_acp_session_with_hooks,
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

const DRIVER_ID: &str = "swallowtail.opencode.acp";
const RECEIVE_FRAMING_LIMITS: FramingLimits =
    FramingLimits::new(DEFAULT_MAX_FRAME_BYTES, DEFAULT_MAX_BUFFER_BYTES);

struct OpenCodeAcpHooks;

impl AcpConnectionHooks for OpenCodeAcpHooks {
    fn on_begin_close<'a>(&'a self, connection: &'a AcpConnection) -> BoxFuture<'a, ()> {
        Box::pin(async move {
            if let Some(session_id) = connection.session_id()
                && let Ok(close) = connection
                    .begin_request("session/close", json!({"sessionId": session_id}))
                    .await
            {
                drop(close);
            }
        })
    }
}

fn opencode_acp_diagnostics() -> AcpDiagnostics {
    AcpDiagnostics {
        route_name: "opencode.acp",
        activity: AcpActivityConfig {
            id_prefix: "opencode-acp",
            namespace_prefix: "opencode.acp",
            plan_projection: PlanProjection::Ignore,
            message_classification: MessageClassification::Standard,
            malformed,
        },
        malformed,
        protocol_failure,
        version_mismatch: || {
            failure(
                "swallowtail.opencode.acp.version_mismatch",
                "OpenCode answered a provider-fixed ACP protocolVersion other than 1",
            )
        },
        connection_closed: || {
            failure(
                "swallowtail.opencode.acp.connection_closed",
                "OpenCode ACP connection is closed",
            )
        },
        correlation_limit: || {
            failure(
                "swallowtail.opencode.acp.correlation_limit",
                "OpenCode ACP request correlation limit was exceeded",
            )
        },
        session_duplicate: || {
            failure(
                "swallowtail.opencode.acp.session_duplicate",
                "OpenCode returned more than one ACP session",
            )
        },
        turn_active: || {
            failure(
                "swallowtail.opencode.acp.turn_active",
                "OpenCode session already has an active turn",
            )
        },
        unknown_response: || {
            failure(
                "swallowtail.opencode.acp.unknown_response",
                "OpenCode returned an unknown ACP response",
            )
        },
        update_without_turn: || {
            failure(
                "swallowtail.opencode.acp.update_without_turn",
                "OpenCode updated a session without an active turn",
            )
        },
        agent_cancel_unsupported: || {
            failure(
                "swallowtail.opencode.acp.agent_cancel_unsupported",
                "OpenCode sent an unsupported client cancellation notification",
            )
        },
        notification_unsupported: || {
            failure(
                "swallowtail.opencode.acp.notification_unsupported",
                "OpenCode sent an unsupported ACP notification",
            )
        },
        callback_unsupported: || {
            failure(
                "swallowtail.opencode.acp.callback_unsupported",
                "OpenCode requested an unsupported ACP client callback",
            )
        },
        permission_without_turn: || {
            failure(
                "swallowtail.opencode.acp.permission_without_turn",
                "OpenCode requested permission without an active turn",
            )
        },
        session_mismatch_callback: || {
            failure(
                "swallowtail.opencode.acp.session_mismatch",
                "OpenCode callback does not match the active session",
            )
        },
        session_mismatch_update: || {
            failure(
                "swallowtail.opencode.acp.session_mismatch",
                "OpenCode update does not match the active session",
            )
        },
        process_cleanup_failed: || {
            SafeDiagnostic::new(
                "swallowtail.opencode.acp.process_cleanup_failed",
                "OpenCode process cleanup failed",
            )
        },
        cleanup_missing: || {
            SafeDiagnostic::new(
                "swallowtail.opencode.acp.cleanup_missing",
                "OpenCode process cleanup did not complete",
            )
        },
        connection_ended: || {
            failure(
                "swallowtail.opencode.acp.connection_ended",
                "OpenCode ACP connection ended",
            )
        },
        output_limit_exceeded: || {
            failure(
                "swallowtail.opencode.acp.output_limit_exceeded",
                "OpenCode output exceeded the adapter limit",
            )
        },
        prompt_stopped: || {
            SafeDiagnostic::new(
                "swallowtail.opencode.acp.prompt_stopped",
                "OpenCode stopped before completing the turn",
            )
        },
        stop_reason_unsupported: || {
            failure(
                "swallowtail.opencode.acp.stop_reason_unsupported",
                "OpenCode returned an unsupported ACP stop reason",
            )
        },
        task_service_missing: || {
            failure(
                "swallowtail.opencode.acp.task_service_missing",
                "OpenCode ACP requires a scoped task service",
            )
        },
        task_join_failed: || {
            SafeDiagnostic::new(
                "swallowtail.opencode.acp.task_join_failed",
                "OpenCode ACP protocol task did not join cleanly",
            )
        },
        turn_join_failed: || {
            SafeDiagnostic::new(
                "swallowtail.opencode.acp.turn_join_failed",
                "OpenCode ACP prompt task did not join cleanly",
            )
        },
        resource_release_failed: || {
            SafeDiagnostic::new(
                "swallowtail.opencode.acp.resource_release_failed",
                "OpenCode ACP working-resource service disappeared during cleanup",
            )
        },
        classify_rpc_error: |error| {
            let message = error.message().to_ascii_lowercase();
            if message.contains("unauthorized")
                || message.contains("unauthenticated")
                || message.contains("not logged in")
            {
                failure(
                    "swallowtail.opencode.acp.host_auth_required",
                    "OpenCode ACP requires host-owned OpenCode login before session work",
                )
            } else {
                failure(
                    "swallowtail.opencode.acp.request_rejected",
                    "OpenCode rejected an ACP request",
                )
            }
        },
        permission_namespace: default_permission_namespace,
        unsupported,
    }
}

/// Low-level interactive driver for the installed OpenCode ACP agent.
pub struct OpenCodeAcpDriver {
    isolated_environment: EnvironmentRef,
    stdio_mcp: Option<OpenCodeAcpStdioMcpServer>,
    http_mcp: Option<OpenCodeAcpRemoteMcpPlacement>,
}

impl OpenCodeAcpDriver {
    /// Binds the isolated launch environment. Credentials stay host-owned.
    #[must_use]
    pub const fn new(isolated_environment: EnvironmentRef) -> Self {
        Self {
            isolated_environment,
            stdio_mcp: None,
            http_mcp: None,
        }
    }

    /// Admits one route-owned stdio MCP declaration onto production `session/new`.
    pub fn with_stdio_mcp_server(
        self,
        server: OpenCodeAcpStdioMcpServer,
    ) -> Result<Self, RuntimeFailure> {
        if self.http_mcp.is_some() {
            return Err(entry_conflict());
        }
        let _ = server.to_acp_value()?;
        Ok(self.with_prepared_stdio_mcp(Some(server)))
    }

    /// Admits one route-owned streamable-HTTP MCP declaration onto production `session/new`.
    pub fn with_http_mcp_placement(
        self,
        server: OpenCodeAcpRemoteMcpPlacement,
    ) -> Result<Self, RuntimeFailure> {
        if self.stdio_mcp.is_some() {
            return Err(entry_conflict());
        }
        let _ = server.to_acp_http_value()?;
        Ok(self.with_prepared_http_mcp(Some(server)))
    }

    pub(crate) fn with_prepared_stdio_mcp(
        mut self,
        server: Option<OpenCodeAcpStdioMcpServer>,
    ) -> Self {
        self.stdio_mcp = server;
        self
    }

    pub(crate) fn with_prepared_http_mcp(
        mut self,
        server: Option<OpenCodeAcpRemoteMcpPlacement>,
    ) -> Self {
        self.http_mcp = server;
        self
    }

    fn validate_plan(
        &self,
        plan: &PreflightPlan,
    ) -> Result<super::selection::OpenCodeAcpPlanSelection, RuntimeFailure> {
        if plan.driver_identity().id().as_str() != DRIVER_ID {
            return Err(failure(
                "swallowtail.opencode.acp.plan_driver_mismatch",
                "Preflight plan is bound to a different driver",
            ));
        }
        if plan.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
            || plan.credential_reference().is_some()
            || plan.endpoint_audience().as_str() != super::OPENCODE_ACP_HOST_ACCOUNT_AUDIENCE
        {
            return Err(failure(
                "swallowtail.opencode.acp.access_profile_rejected",
                "OpenCode ACP requires its host-owned OpenCode login profile",
            ));
        }
        if plan.harness_configuration_posture() != Some(HarnessConfigurationPosture::Ambient) {
            return Err(failure(
                "swallowtail.opencode.acp.configuration_posture_rejected",
                "OpenCode ACP requires explicit ambient configuration inside its selected environment",
            ));
        }
        if plan.requirements().harness_isolation() != Some(HarnessIsolation::AmbientHost) {
            return Err(failure(
                "swallowtail.opencode.acp.isolation_rejected",
                "OpenCode ACP requires explicit ambient-host isolation posture",
            ));
        }
        super::selection::select_opencode_acp_plan(plan)
    }
}

/// Describes the installed OpenCode ACP discovery and session roles.
#[must_use]
pub fn opencode_acp_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("opencode-acp").expect("static family id is valid"),
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
    .with_interface_compatibility(super::opencode_acp_claim())
}

impl InteractiveSessionDriver for OpenCodeAcpDriver {
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
                "opencode-acp:session:{}",
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

impl OpenCodeAcpDriver {
    async fn start_session(
        &self,
        plan: &PreflightPlan,
        request: &OpenSessionRequest,
        services: &HostServices,
        scope: ScopeId,
        resource: ResourceLease,
        selected: super::selection::OpenCodeAcpPlanSelection,
    ) -> Result<AcpSessionHandle, Box<(RuntimeFailure, ResourceLease)>> {
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
        let stdio_mcp = self.stdio_mcp.clone();
        let http_mcp = self.http_mcp.clone();
        open_acp_session_with_hooks(
            transport,
            AcpSessionOpenContext {
                scope,
                request_id: request.request_id().clone(),
                execution_host_id: plan.execution_host_id().clone(),
                resource,
                services: services.clone(),
                config: opencode_acp_session_config(),
            },
            Arc::new(OpenCodeAcpHooks),
            move |connection| async move {
                let initialize = connection.initialize().await?;
                validate_initialize(&initialize, selected.version())?;
                let mcp_servers = production_mcp_servers(stdio_mcp.as_ref(), http_mcp.as_ref())?;
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

fn opencode_acp_session_config() -> AcpSessionConfig {
    let diagnostics = opencode_acp_diagnostics();
    AcpSessionConfig::new("opencode-acp", diagnostics).with_turn_config(
        ActiveTurnConfig::new(diagnostics).with_content_unsupported(|| {
            failure(
                "swallowtail.opencode.acp.content_unsupported",
                "OpenCode returned unsupported ACP content",
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
            RuntimeTurnId::new("opencode-non-text").expect("valid turn id"),
            "sess-1".to_owned(),
            opencode_acp_session_config().turn_config,
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
            "swallowtail.opencode.acp.content_unsupported"
        );
    }
}
