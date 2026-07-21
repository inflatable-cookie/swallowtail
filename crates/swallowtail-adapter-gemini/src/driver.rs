use crate::connection::AcpConnection;
use crate::failure::{failure, malformed, unsupported};
use crate::turn::ActiveTurn;
use serde_json::{Value, json};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, CancellationScope, CredentialMechanism,
    CredentialRef, DriverDescriptor, DriverRole, ExecutionLayer, HostServiceKind,
    IntegrationFamilyId, OperationShape, PreflightPlan, ResourceAccess, ResourceRepresentation,
    SessionAccessPolicy, SessionRef, TransportFamilyId,
};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CancellationAcknowledgement, CancellationControl, CleanupOutcome,
    EnvironmentRef, ExecutableRef, HostServices, InteractiveSessionDriver,
    InteractiveSessionHandle, JoinedTask, OpenSessionRequest, ProcessHandle, ProcessRequest,
    RequestId, ResourceLease, ResumeSessionRequest, RuntimeFailure, RuntimeSessionId, ScopeId,
    TerminalOutcome, TurnHandle, TurnRequest, validate_session_access_plan,
    validate_session_resource_lease,
};

const DRIVER_ID: &str = "swallowtail.gemini.acp";
const GEMINI_VERSION: &str = "0.51.0";

pub struct GeminiAcpDriver {
    isolated_environment: EnvironmentRef,
    credential: CredentialRef,
}

impl GeminiAcpDriver {
    #[must_use]
    pub const fn new(isolated_environment: EnvironmentRef, credential: CredentialRef) -> Self {
        Self {
            isolated_environment,
            credential,
        }
    }

    fn validate_plan(&self, plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
        if plan.driver_identity().id().as_str() != DRIVER_ID {
            return Err(failure(
                "swallowtail.gemini.acp.plan_driver_mismatch",
                "Preflight plan is bound to a different driver",
            ));
        }
        if plan.credential_mechanism() != &CredentialMechanism::ApiKey
            || plan.credential_reference() != Some(&self.credential)
        {
            return Err(failure(
                "swallowtail.gemini.acp.access_profile_rejected",
                "Gemini ACP requires its configured Developer API key profile",
            ));
        }
        Ok(())
    }
}

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
    .with_roles([DriverRole::InteractiveSession])
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
}

impl InteractiveSessionDriver for GeminiAcpDriver {
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
                "gemini-acp:session:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| malformed())?;
            let resource_service = services
                .working_resource()
                .cloned()
                .expect("validated working-resource service");
            let resource = resource_service
                .resolve(
                    scope.clone(),
                    request
                        .working_resource()
                        .expect("validated resource")
                        .clone(),
                    ResourceAccess::Read,
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
            let cwd = resource
                .filesystem()
                .expect("validated filesystem lease")
                .as_driver_value()
                .to_owned();
            let result = self
                .start_session(&plan, &request, &services, scope, resource, cwd)
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

impl GeminiAcpDriver {
    async fn start_session(
        &self,
        plan: &PreflightPlan,
        request: &OpenSessionRequest,
        services: &HostServices,
        scope: ScopeId,
        resource: ResourceLease,
        cwd: String,
    ) -> Result<GeminiSessionHandle, (RuntimeFailure, ResourceLease)> {
        let process_service = services
            .process()
            .cloned()
            .expect("validated process service");
        let process_request = gemini_process_request(
            ExecutableRef::from_instance_target(plan.instance_target_ref()),
            self.isolated_environment.clone(),
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
        let resource_io = services
            .working_resource_io()
            .cloned()
            .expect("validated working-resource I/O service");
        let connection = AcpConnection::new(Arc::clone(&process), resource.clone(), resource_io);
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
            validate_initialize(&initialize)?;
            let response = connection
                .request("session/new", json!({"cwd": cwd, "mcpServers": []}))
                .await?;
            parse_new_session(&response)
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
            RuntimeSessionId::new(format!("gemini-acp:{}", request.request_id().as_str()))
                .map_err(|_| (malformed(), resource.clone()))?;
        let active = Arc::new(Mutex::new(None));
        Ok(GeminiSessionHandle {
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

fn gemini_process_request(
    executable: ExecutableRef,
    environment: EnvironmentRef,
    resource: swallowtail_runtime::WorkingResourceRef,
) -> ProcessRequest {
    ProcessRequest::new(executable)
        .with_arguments([
            "--acp".to_owned(),
            "--approval-mode".to_owned(),
            "plan".to_owned(),
        ])
        .with_environment([environment])
        .with_working_resource(resource)
}

#[cfg(test)]
include!("driver/tests.rs");
