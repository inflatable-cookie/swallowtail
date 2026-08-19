use crate::{
    command::arguments,
    connection::AcpConnection,
    failure::{failure, malformed, unsupported},
    turn::ActiveTurn,
};
use serde_json::{Value, json};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, CancellationScope, CredentialMechanism,
    DriverDescriptor, DriverRole, ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation,
    HostServiceKind, IntegrationFamilyId, OperationShape, PreflightPlan, ResourceAccess,
    ResourceRepresentation, SessionAccessPolicy, SessionRef, TransportFamilyId,
};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CancellationAcknowledgement, CancellationControl, CleanupOutcome,
    EnvironmentRef, ExecutableRef, HostServices, InteractiveSessionDriver,
    InteractiveSessionHandle, JoinedTask, OpenSessionRequest, ProcessHandle, ProcessRequest,
    RequestId, ResourceLease, ResumeSessionRequest, RuntimeFailure, RuntimeSessionId, ScopeId,
    TerminalOutcome, TurnHandle, TurnRequest, validate_session_resource_lease,
};

const DRIVER_ID: &str = "swallowtail.copilot-cli.acp";

/// Low-level interactive driver for the installed Copilot CLI ACP agent.
pub struct CopilotCliAcpDriver {
    isolated_environment: EnvironmentRef,
}

impl CopilotCliAcpDriver {
    /// Binds the isolated launch environment. Credentials stay host-owned.
    #[must_use]
    pub const fn new(isolated_environment: EnvironmentRef) -> Self {
        Self {
            isolated_environment,
        }
    }

    fn validate_plan(
        &self,
        plan: &PreflightPlan,
    ) -> Result<crate::selection::CopilotCliPlanSelection, RuntimeFailure> {
        if plan.driver_identity().id().as_str() != DRIVER_ID {
            return Err(failure(
                "swallowtail.copilot-cli.acp.plan_driver_mismatch",
                "Preflight plan is bound to a different driver",
            ));
        }
        if plan.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
            || plan.credential_reference().is_some()
            || plan.endpoint_audience().as_str() != crate::COPILOT_CLI_HOST_ACCOUNT_AUDIENCE
        {
            return Err(failure(
                "swallowtail.copilot-cli.acp.access_profile_rejected",
                "Copilot CLI ACP requires its host-owned GitHub-login or BYOK profile",
            ));
        }
        if plan.harness_configuration_posture() != Some(HarnessConfigurationPosture::Ambient) {
            return Err(failure(
                "swallowtail.copilot-cli.acp.configuration_posture_rejected",
                "Copilot CLI ACP requires explicit ambient configuration inside its selected environment",
            ));
        }
        if plan.requirements().harness_isolation() != Some(HarnessIsolation::AmbientHost) {
            return Err(failure(
                "swallowtail.copilot-cli.acp.isolation_rejected",
                "Copilot CLI ACP requires explicit ambient-host isolation posture",
            ));
        }
        crate::selection::select_copilot_cli_acp_plan(plan)
    }
}

/// Describes the installed Copilot CLI ACP discovery and session roles.
#[must_use]
pub fn copilot_cli_acp_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("copilot-cli").expect("static family id is valid"),
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
    .with_interface_compatibility(crate::copilot_cli_acp_claim())
}

impl InteractiveSessionDriver for CopilotCliAcpDriver {
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
                "copilot-cli-acp:session:{}",
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
                Err((error, resource)) => {
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

impl CopilotCliAcpDriver {
    async fn start_session(
        &self,
        plan: &PreflightPlan,
        request: &OpenSessionRequest,
        services: &HostServices,
        scope: ScopeId,
        resource: ResourceLease,
        selected: crate::selection::CopilotCliPlanSelection,
    ) -> Result<CopilotCliSessionHandle, (RuntimeFailure, ResourceLease)> {
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
                Err(error) => return Err((error, resource)),
            };
        let connection = AcpConnection::new(Arc::clone(&process), services.clone());
        let pump_connection = Arc::clone(&connection);
        let task_service = services.task().cloned().expect("validated task service");
        let pump_task = match task_service
            .spawn(scope, Box::pin(async move { pump_connection.pump().await }))
        {
            Ok(task) => task,
            Err(error) => {
                let _ = process.force_stop().await;
                let _ = process.wait().await;
                return Err((error, resource));
            }
        };
        let opened = async {
            let initialize = connection.initialize().await?;
            validate_initialize(&initialize, selected.version())?;
            connection
                .request("session/new", json!({"cwd": cwd, "mcpServers": []}))
                .await
                .and_then(parse_new_session)
        }
        .await;
        let provider_id = match opened {
            Ok(provider_id) => provider_id,
            Err(error) => {
                connection.begin_close().await;
                let _ = pump_task.join().await;
                return Err((error, resource));
            }
        };
        if let Err(error) = connection.set_session_id(provider_id.clone()) {
            connection.begin_close().await;
            let _ = pump_task.join().await;
            return Err((error, resource));
        }
        let provider_ref = match SessionRef::new(&provider_id) {
            Ok(provider_ref) => provider_ref,
            Err(_) => {
                connection.begin_close().await;
                let _ = pump_task.join().await;
                return Err((malformed(), resource));
            }
        };
        let runtime_id =
            RuntimeSessionId::new(format!("copilot-cli-acp:{}", request.request_id().as_str()))
                .map_err(|_| (malformed(), resource.clone()))?;
        let active = Arc::new(Mutex::new(None));
        Ok(CopilotCliSessionHandle {
            request_id: request.request_id().clone(),
            runtime_id,
            provider_ref,
            provider_id,
            execution_host_id: plan.execution_host_id().clone(),
            connection: Arc::clone(&connection),
            cancellation: SessionCancellation::new(connection),
            pump_task: Some(pump_task),
            services: services.clone(),
            resource: Some(resource),
            active,
        })
    }
}

include!("driver/validation.rs");
include!("driver/cancellation.rs");
include!("driver/turn_handle.rs");
include!("driver/session.rs");
