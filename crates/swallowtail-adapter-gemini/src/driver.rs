use crate::connection::AcpConnection;
use crate::failure::{failure, malformed, unsupported};
use crate::turn::ActiveTurn;
use serde_json::{Value, json};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, CancellationScope, Capability,
    CapabilityConstraint, CredentialMechanism, CredentialRef, DriverDescriptor, DriverRole,
    ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation, HostServiceKind,
    IntegrationFamilyId, OperationShape, PreflightPlan, ResourceAccess, ResourceRepresentation,
    SessionAccessPolicy, SessionRef, TransportFamilyId,
};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CancellationAcknowledgement, CancellationControl, CleanupOutcome,
    EnvironmentRef, ExecutableRef, HostServices, InteractiveSessionDriver,
    InteractiveSessionHandle, JoinedTask, NegotiatedSessionModelOption,
    NegotiatedSessionModelOptions, OpenSessionRequest, ProcessHandle, ProcessRequest, RequestId,
    ResourceLease, ResumeSessionRequest, RuntimeFailure, RuntimeSessionId, ScopeId,
    TerminalOutcome, TurnHandle, TurnRequest, validate_session_resource_lease,
};

const DRIVER_ID: &str = "swallowtail.gemini.acp";

/// Low-level interactive driver for the installed Gemini CLI ACP agent.
pub struct GeminiAcpDriver {
    isolated_environment: EnvironmentRef,
    credential: CredentialRef,
}

impl GeminiAcpDriver {
    /// Binds the isolated launch environment and Developer API credential.
    #[must_use]
    pub const fn new(isolated_environment: EnvironmentRef, credential: CredentialRef) -> Self {
        Self {
            isolated_environment,
            credential,
        }
    }

    fn validate_plan(
        &self,
        plan: &PreflightPlan,
    ) -> Result<crate::selection::GeminiPlanSelection, RuntimeFailure> {
        if plan.driver_identity().id().as_str() != DRIVER_ID {
            return Err(failure(
                "swallowtail.gemini.acp.plan_driver_mismatch",
                "Preflight plan is bound to a different driver",
            ));
        }
        if plan.credential_mechanism() != &CredentialMechanism::ApiKey
            || plan.credential_reference() != Some(&self.credential)
            || plan.endpoint_audience().as_str() != "gemini-developer-api"
        {
            return Err(failure(
                "swallowtail.gemini.acp.access_profile_rejected",
                "Gemini ACP requires its configured Developer API key profile",
            ));
        }
        if plan.harness_configuration_posture() != Some(HarnessConfigurationPosture::Ambient) {
            return Err(failure(
                "swallowtail.gemini.acp.configuration_posture_rejected",
                "Gemini ACP requires explicit ambient configuration inside its selected environment",
            ));
        }
        if plan.requirements().harness_isolation() != Some(HarnessIsolation::AmbientHost) {
            return Err(failure(
                "swallowtail.gemini.acp.isolation_rejected",
                "Gemini ACP requires explicit ambient-host isolation posture",
            ));
        }
        crate::selection::select_gemini_acp_plan(plan)
    }
}

/// Describes the installed Gemini CLI ACP discovery and session roles.
#[must_use]
pub fn gemini_acp_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("gemini-cli").expect("static family id is valid"),
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
            HostServiceKind::WorkingResourceIo,
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
    .with_interface_compatibility(crate::gemini_cli_acp_claim())
}

impl InteractiveSessionDriver for GeminiAcpDriver {
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
                "gemini-acp:session:{}",
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

impl GeminiAcpDriver {
    async fn start_session(
        &self,
        plan: &PreflightPlan,
        request: &OpenSessionRequest,
        services: &HostServices,
        scope: ScopeId,
        resource: ResourceLease,
        selected: crate::selection::GeminiPlanSelection,
    ) -> Result<GeminiSessionHandle, Box<(RuntimeFailure, ResourceLease)>> {
        let cwd = resource
            .filesystem()
            .expect("validated filesystem lease")
            .as_driver_value()
            .to_owned();
        let process_service = services
            .process()
            .cloned()
            .expect("validated process service");
        let resource_access =
            session_resource_access(plan).map_err(|error| (error, resource.clone()))?;
        let process_request = gemini_process_request(
            ExecutableRef::from_instance_target(plan.instance_target_ref()),
            self.isolated_environment.clone(),
            request
                .working_resource()
                .expect("validated resource")
                .clone(),
            resource_access,
        );
        let process: Arc<dyn ProcessHandle> =
            match process_service.start(scope.clone(), process_request).await {
                Ok(process) => Arc::from(process),
                Err(error) => return Err(Box::new((error, resource))),
            };
        let resource_io = services
            .working_resource_io()
            .cloned()
            .expect("validated working-resource I/O service");
        let connection = AcpConnection::new(
            Arc::clone(&process),
            resource.clone(),
            resource_io,
            resource_access == ResourceAccess::ReadWrite,
            services.clone(),
        );
        let pump_connection = Arc::clone(&connection);
        let task_service = services.task().cloned().expect("validated task service");
        let pump_task = match task_service
            .spawn(scope, Box::pin(async move { pump_connection.pump().await }))
        {
            Ok(task) => task,
            Err(error) => {
                let _ = process.force_stop().await;
                let _ = process.wait().await;
                return Err(Box::new((error, resource)));
            }
        };
        let opened = async {
            let initialize = connection.initialize().await?;
            validate_initialize(&initialize, selected.version())?;
            let response = connection
                .request("session/new", json!({"cwd": cwd, "mcpServers": []}))
                .await?;
            parse_new_session(&response, resource_access)
        }
        .await;
        let opened = match opened {
            Ok(opened) => opened,
            Err(error) => {
                connection.begin_close().await;
                let _ = pump_task.join().await;
                return Err(Box::new((error, resource)));
            }
        };
        let provider_id = opened.provider_id;
        if let Err(error) = connection.set_session_id(provider_id.clone()) {
            connection.begin_close().await;
            let _ = pump_task.join().await;
            return Err(Box::new((error, resource)));
        }
        let provider_ref = match SessionRef::new(&provider_id) {
            Ok(provider_ref) => provider_ref,
            Err(_) => {
                connection.begin_close().await;
                let _ = pump_task.join().await;
                return Err(Box::new((malformed(), resource)));
            }
        };
        let runtime_id =
            RuntimeSessionId::new(format!("gemini-acp:{}", request.request_id().as_str()))
                .map_err(|_| Box::new((malformed(), resource.clone())))?;
        let active = Arc::new(Mutex::new(None));
        Ok(GeminiSessionHandle {
            request_id: request.request_id().clone(),
            runtime_id,
            provider_ref,
            provider_id,
            model_options: opened.model_options,
            execution_host_id: plan.execution_host_id().clone(),
            connection: Arc::clone(&connection),
            cancellation: SessionCancellation::new(connection),
            pump_task: Some(pump_task),
            services: services.clone(),
            resource: Some(resource),
            expected_mode: provider_mode_id(resource_access),
            active,
        })
    }
}

include!("driver/validation.rs");
include!("driver/cancellation.rs");
include!("driver/turn_handle.rs");
include!("driver/session.rs");

fn gemini_process_request(
    executable: ExecutableRef,
    environment: EnvironmentRef,
    resource: swallowtail_runtime::WorkingResourceRef,
    resource_access: ResourceAccess,
) -> ProcessRequest {
    let approval_mode = match resource_access {
        ResourceAccess::Read => "plan",
        ResourceAccess::ReadWrite => "auto_edit",
    };
    ProcessRequest::new(executable)
        .with_arguments([
            "--acp".to_owned(),
            "--approval-mode".to_owned(),
            approval_mode.to_owned(),
        ])
        .with_environment([environment])
        .with_working_resource(resource)
}

fn provider_mode_id(resource_access: ResourceAccess) -> &'static str {
    match resource_access {
        ResourceAccess::Read => "plan",
        ResourceAccess::ReadWrite => "autoEdit",
    }
}

fn session_resource_access(plan: &PreflightPlan) -> Result<ResourceAccess, RuntimeFailure> {
    plan.requirements()
        .session_access_policy()
        .and_then(SessionAccessPolicy::resource_access)
        .ok_or_else(|| {
            failure(
                "swallowtail.gemini.acp.resource_access_missing",
                "Gemini ACP requires explicit working-resource access",
            )
        })
}

#[cfg(test)]
include!("driver/tests.rs");
