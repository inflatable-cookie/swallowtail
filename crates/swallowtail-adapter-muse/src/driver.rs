use crate::command::{MuseCommand, arguments};
use crate::handle::{MuseCancellation, MuseRunHandle};
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

const EVENT_CAPACITY: usize = 12_290;

#[must_use]
/// Returns the exact installed Muse Code discovery and headless-run descriptor.
pub fn muse_headless_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(crate::DRIVER_ID).expect("static Muse driver id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("muse-code").expect("static Muse family id is valid"),
        TransportFamilyId::new("muse-code-event-jsonl-stdio")
            .expect("static Muse transport id is valid"),
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
    .with_interface_compatibility(crate::muse_headless_claim())
}

/// Low-level exact Muse Code headless driver.
pub struct MuseHeadlessDriver {
    environment: EnvironmentRef,
}

impl MuseHeadlessDriver {
    #[must_use]
    /// Creates a driver with one host-approved process environment.
    pub const fn new(environment: EnvironmentRef) -> Self {
        Self { environment }
    }

    #[must_use]
    /// Returns the approved process environment.
    pub const fn environment(&self) -> &EnvironmentRef {
        &self.environment
    }
}

impl StructuredRunDriver for MuseHeadlessDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        Box::pin(async move { self.start(plan, request, services).await })
    }
}

impl MuseHeadlessDriver {
    async fn start(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> Result<Box<dyn RunHandle>, RuntimeFailure> {
        let validated = crate::validation::validate(&plan, &request, &services)?;
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
        .with_arguments(arguments(MuseCommand {
            prompt: request.content().as_str(),
            model: &model,
            effort: &validated.effort,
        }))
        .with_environment([self.environment.clone()])
        .with_working_resource(working_resource);
        let process: Arc<dyn ProcessHandle> = Arc::from(
            process_service
                .start(scope.clone(), process_request)
                .await?,
        );
        if process.close_stdin().await.is_err() {
            crate::pump::cleanup_failed_start(process.as_ref()).await;
            return Err(crate::failure::failure(
                "swallowtail.muse_code.headless.stdin_close_failed",
                "Muse Code process stdin could not be closed",
            ));
        }
        if let Err(error) = event_sender.send(RuntimeEvent::new(0, RuntimeEventKind::Started)) {
            crate::pump::cleanup_failed_start(process.as_ref()).await;
            return Err(error);
        }
        let (terminal_sender, terminal_future) = terminal_outcome_channel();
        let cancellation = Arc::new(MuseCancellation::new(Arc::clone(&process)));
        let task = task_service.spawn(
            scope,
            Box::pin({
                let cancellation = Arc::clone(&cancellation);
                let process = Arc::clone(&process);
                let operation_id = ActivityOperationId::Run(run_id.clone());
                async move {
                    let outcome = crate::pump::pump(
                        process,
                        event_sender.clone(),
                        cancellation,
                        time_service.wait_until(deadline),
                        model,
                        operation_id,
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
        Ok(Box::new(MuseRunHandle::new(
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
    RuntimeRunId::new(format!("muse-headless:{request_id}")).map_err(|_| invalid_request())
}

fn runtime_scope(request_id: &str) -> Result<ScopeId, RuntimeFailure> {
    ScopeId::new(format!("muse-headless:{request_id}")).map_err(|_| invalid_request())
}

fn invalid_request() -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.muse_code.headless.invalid_request",
        "Muse Code request identity is invalid",
    )
}

#[cfg(test)]
mod tests {
    use super::muse_headless_descriptor;
    use swallowtail_core::{DriverRole, ExecutionLayer, OperationShape};

    #[test]
    fn descriptor_is_one_separate_exact_harness_route() {
        let descriptor = muse_headless_descriptor();
        assert_eq!(descriptor.integration_family().as_str(), "muse-code");
        assert_eq!(
            descriptor.transport_family().as_str(),
            "muse-code-event-jsonl-stdio"
        );
        assert!(descriptor.supports_role(DriverRole::Discovery));
        assert!(descriptor.supports_role(DriverRole::StructuredRun));
        assert!(descriptor.supports_execution_layer(ExecutionLayer::HarnessInteraction));
        assert!(descriptor.supports_operation_shape(OperationShape::StructuredRun));
        assert!(!descriptor.supports_role(DriverRole::InteractiveSession));
    }
}
