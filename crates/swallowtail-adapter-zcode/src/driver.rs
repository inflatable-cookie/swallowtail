use crate::handle::{ZcodeCancellation, ZcodeRunHandle};
use crate::mode::ZcodeAppServerMode;
use std::sync::Arc;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, DriverDescriptor, DriverRole, ExecutionLayer,
    HostServiceKind, IntegrationFamilyId, OperationShape, PreflightPlan, TransportFamilyId,
};
use swallowtail_runtime::{
    ActivityOperationId, BoxFuture, EnvironmentRef, ExecutableRef, HostServices, ProcessHandle,
    ProcessRequest, RunHandle, RuntimeEvent, RuntimeEventKind, RuntimeFailure, RuntimeRunId,
    ScopeId, StructuredRunDriver, StructuredRunRequest, runtime_event_channel,
    terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 16_384;

#[must_use]
/// Returns the exact installed ZCode app-server descriptor.
pub fn zcode_app_server_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(crate::DRIVER_ID).expect("static ZCode driver id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("zcode").expect("static ZCode family id is valid"),
        TransportFamilyId::new("zcode-app-server-ndjson-stdio")
            .expect("static ZCode transport id is valid"),
    )
    .with_roles([DriverRole::Discovery, DriverRole::StructuredRun])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([OperationShape::StructuredRun])
    .with_required_host_services(
        DriverRole::Discovery,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::Time,
        ],
    )
    .with_required_host_services(
        DriverRole::StructuredRun,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::Time,
        ],
    )
    .with_discovery_actions([swallowtail_core::DiscoveryAction::Probe])
    .with_interface_compatibility(crate::zcode_app_server_claim())
}

/// Low-level exact ZCode app-server driver.
pub struct ZcodeAppServerDriver {
    environment: EnvironmentRef,
    mode: ZcodeAppServerMode,
}

impl ZcodeAppServerDriver {
    #[must_use]
    /// Creates a driver with host-approved settings and an explicit session mode.
    pub const fn new(environment: EnvironmentRef, mode: ZcodeAppServerMode) -> Self {
        Self { environment, mode }
    }

    #[must_use]
    /// Returns the approved settings configuration reference.
    pub const fn environment(&self) -> &EnvironmentRef {
        &self.environment
    }

    #[must_use]
    /// Returns the host-supplied session mode bound at construction.
    pub const fn mode(&self) -> &ZcodeAppServerMode {
        &self.mode
    }
}

impl StructuredRunDriver for ZcodeAppServerDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        Box::pin(async move { self.start(plan, request, services).await })
    }
}

impl ZcodeAppServerDriver {
    async fn start(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> Result<Box<dyn RunHandle>, RuntimeFailure> {
        crate::validation::validate(&plan, &request, &services)?;
        let task_service = services.task().cloned().expect("validated task service");
        let process_service = services
            .process()
            .cloned()
            .expect("validated process service");
        let time_service = services.time().cloned().expect("validated time service");
        let model = plan.model_id().cloned().expect("validated model route");
        let working_resource = request
            .working_resource()
            .cloned()
            .expect("validated working resource");
        let deadline = request.deadline().expect("validated deadline");
        let run_id = runtime_run_id(request.request_id().as_str())?;
        let scope = runtime_scope(request.request_id().as_str())?;
        let (event_sender, event_stream) = runtime_event_channel(EVENT_CAPACITY)?;
        let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
            plan.instance_target_ref(),
        ))
        .with_arguments(crate::command::arguments())
        .with_environment([self.environment.clone()])
        .with_working_resource(working_resource.clone());
        let process: Arc<dyn ProcessHandle> = Arc::from(
            process_service
                .start(scope.clone(), process_request)
                .await?,
        );
        if let Err(error) = event_sender.send(RuntimeEvent::new(0, RuntimeEventKind::Started)) {
            crate::pump::cleanup_failed_start(process.as_ref()).await;
            return Err(error);
        }
        let (terminal_sender, terminal_future) = terminal_outcome_channel();
        let cancellation = Arc::new(ZcodeCancellation::new(Arc::clone(&process)));
        let task = task_service.spawn(
            scope,
            Box::pin({
                let cancellation = Arc::clone(&cancellation);
                let process = Arc::clone(&process);
                let operation_id = ActivityOperationId::Run(run_id.clone());
                let services = services.clone();
                let cwd = working_resource.as_host_value().to_owned();
                let model = model.as_str().to_owned();
                let provider = plan
                    .provider_id()
                    .expect("validated provider")
                    .as_str()
                    .to_owned();
                let prompt = request.content().as_str().to_owned();
                let mode = self.mode.as_str().to_owned();
                async move {
                    let outcome = crate::pump::pump(
                        process,
                        event_sender.clone(),
                        cancellation,
                        time_service.wait_until(deadline),
                        operation_id,
                        cwd,
                        provider,
                        model,
                        prompt,
                        mode,
                        services,
                    )
                    .await;
                    let _ = terminal_sender.complete(outcome);
                    event_sender.mark_terminal();
                }
            }),
        );
        let task = match task {
            Ok(task) => task,
            Err(error) => {
                crate::pump::cleanup_failed_start(process.as_ref()).await;
                return Err(error);
            }
        };
        Ok(Box::new(ZcodeRunHandle::new(
            request.request_id().clone(),
            run_id,
            Box::pin(event_stream),
            Box::pin(terminal_future),
            cancellation,
            task,
        )))
    }
}

fn runtime_run_id(request_id: &str) -> Result<RuntimeRunId, RuntimeFailure> {
    RuntimeRunId::new(format!("zcode:{request_id}")).map_err(|_| invalid_request())
}

fn runtime_scope(request_id: &str) -> Result<ScopeId, RuntimeFailure> {
    ScopeId::new(format!("zcode:{request_id}")).map_err(|_| invalid_request())
}

fn invalid_request() -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.zcode.app_server.invalid_request",
        "ZCode app-server request identity is invalid",
    )
}

#[cfg(test)]
mod tests {
    use super::zcode_app_server_descriptor;
    use swallowtail_core::{DriverRole, ExecutionLayer, HostServiceKind, OperationShape};

    #[test]
    fn descriptor_is_one_separate_structured_harness_route() {
        let descriptor = zcode_app_server_descriptor();
        assert_eq!(descriptor.integration_family().as_str(), "zcode");
        assert_eq!(
            descriptor.transport_family().as_str(),
            "zcode-app-server-ndjson-stdio"
        );
        assert_eq!(descriptor.identity().id().as_str(), crate::DRIVER_ID);
        assert!(descriptor.supports_role(DriverRole::Discovery));
        assert!(descriptor.supports_role(DriverRole::StructuredRun));
        assert!(descriptor.supports_execution_layer(ExecutionLayer::HarnessInteraction));
        assert!(descriptor.supports_operation_shape(OperationShape::StructuredRun));
        assert!(!descriptor.supports_role(DriverRole::InteractiveSession));
        assert!(!descriptor.supports_operation_shape(OperationShape::InteractiveSession));
        let services: Vec<_> = descriptor
            .required_host_services(DriverRole::StructuredRun)
            .collect();
        assert!(services.contains(&HostServiceKind::Task));
        assert!(services.contains(&HostServiceKind::Process));
        assert!(services.contains(&HostServiceKind::Time));
    }
}
