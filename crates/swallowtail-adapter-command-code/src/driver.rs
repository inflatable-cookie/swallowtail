use crate::command::arguments;
use crate::handle::{CommandCodeCancellation, CommandCodeRunHandle};
use std::sync::Arc;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, DriverDescriptor, DriverRole, ExecutionLayer,
    HostServiceKind, IntegrationFamilyId, OperationShape, PreflightPlan, TransportFamilyId,
};
use swallowtail_runtime::{
    ActivityOperationId, BoxFuture, EnvironmentRef, ExecutableRef, HostServices, ProcessHandle,
    ProcessInputChunk, ProcessRequest, RunHandle, RuntimeEvent, RuntimeEventKind, RuntimeFailure,
    RuntimeRunId, ScopeId, StructuredRunDriver, StructuredRunRequest, runtime_event_channel,
    terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 4098;

#[must_use]
/// Returns the exact installed Command Code discovery and headless-run descriptor.
pub fn command_code_headless_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(crate::DRIVER_ID).expect("static Command Code driver id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("command-code").expect("static Command Code family id is valid"),
        TransportFamilyId::new("command-code-agent-event-ndjson-stdio")
            .expect("static Command Code transport id is valid"),
    )
    .with_roles([
        DriverRole::Discovery,
        DriverRole::StructuredRun,
        DriverRole::InteractiveSession,
    ])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([
        OperationShape::StructuredRun,
        OperationShape::InteractiveSession,
    ])
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
    .with_required_host_services(
        DriverRole::InteractiveSession,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::Time,
            HostServiceKind::WorkingResource,
        ],
    )
    .with_discovery_actions([swallowtail_core::DiscoveryAction::Probe])
    .with_interface_compatibility(crate::selection::command_code_headless_claim())
}

/// Low-level exact Command Code headless driver.
pub struct CommandCodeHeadlessDriver {
    environment: EnvironmentRef,
}

impl CommandCodeHeadlessDriver {
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

impl StructuredRunDriver for CommandCodeHeadlessDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        Box::pin(async move { self.start(plan, request, services).await })
    }
}

impl CommandCodeHeadlessDriver {
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
        .with_arguments(arguments(&model))
        .with_environment([self.environment.clone()])
        .with_working_resource(working_resource);
        let process: Arc<dyn ProcessHandle> = Arc::from(
            process_service
                .start(scope.clone(), process_request)
                .await?,
        );
        if let Err(error) = write_prompt(process.as_ref(), request.content()).await {
            crate::pump::cleanup_failed_start(process.as_ref()).await;
            return Err(error);
        }
        if let Err(error) = event_sender.send(RuntimeEvent::new(0, RuntimeEventKind::Started)) {
            crate::pump::cleanup_failed_start(process.as_ref()).await;
            return Err(error);
        }
        let (terminal_sender, terminal_future) = terminal_outcome_channel();
        let cancellation = Arc::new(CommandCodeCancellation::new(Arc::clone(&process)));
        let task = task_service.spawn(
            scope,
            Box::pin({
                let cancellation = Arc::clone(&cancellation);
                let process = Arc::clone(&process);
                let operation_id = ActivityOperationId::Run(run_id.clone());
                let services = services.clone();
                async move {
                    let outcome = crate::pump::pump(
                        process,
                        event_sender.clone(),
                        cancellation,
                        time_service.wait_until(deadline),
                        operation_id,
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
        Ok(Box::new(CommandCodeRunHandle::new(
            request.request_id().clone(),
            run_id,
            Box::pin(event_stream),
            Box::pin(terminal_future),
            cancellation,
            task,
        )))
    }
}

pub(crate) async fn write_prompt(
    process: &dyn ProcessHandle,
    content: &swallowtail_runtime::OperationContent,
) -> Result<(), RuntimeFailure> {
    process
        .write_stdin(ProcessInputChunk::new(
            content.as_str().as_bytes().to_vec(),
        ))
        .await?;
    process.close_stdin().await.map_err(|_| {
        crate::failure::failure(
            "swallowtail.command_code.headless.stdin_close_failed",
            "Command Code process stdin could not be closed",
        )
    })
}

fn runtime_run_id(request_id: &str) -> Result<RuntimeRunId, RuntimeFailure> {
    RuntimeRunId::new(format!("command-code-headless:{request_id}")).map_err(|_| invalid_request())
}

fn runtime_scope(request_id: &str) -> Result<ScopeId, RuntimeFailure> {
    ScopeId::new(format!("command-code-headless:{request_id}")).map_err(|_| invalid_request())
}

fn invalid_request() -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.command_code.headless.invalid_request",
        "Command Code request identity is invalid",
    )
}

#[cfg(test)]
mod tests {
    use super::command_code_headless_descriptor;
    use swallowtail_core::{DriverRole, ExecutionLayer, OperationShape};

    #[test]
    fn descriptor_is_one_separate_exact_harness_route() {
        let descriptor = command_code_headless_descriptor();
        assert_eq!(descriptor.integration_family().as_str(), "command-code");
        assert_eq!(
            descriptor.transport_family().as_str(),
            "command-code-agent-event-ndjson-stdio"
        );
        assert!(descriptor.supports_role(DriverRole::Discovery));
        assert!(descriptor.supports_role(DriverRole::StructuredRun));
        assert!(descriptor.supports_role(DriverRole::InteractiveSession));
        assert!(descriptor.supports_execution_layer(ExecutionLayer::HarnessInteraction));
        assert!(descriptor.supports_operation_shape(OperationShape::StructuredRun));
        assert!(descriptor.supports_operation_shape(OperationShape::InteractiveSession));
    }
}
