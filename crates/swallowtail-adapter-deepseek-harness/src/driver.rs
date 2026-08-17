use crate::handle::{DeepSeekHarnessCancellation, DeepSeekHarnessRunHandle};
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
/// Returns the exact installed DeepSeek Harness JSON-RPC descriptor.
pub fn deepseek_harness_jsonrpc_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(crate::DRIVER_ID).expect("static DeepSeek Harness driver id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("deepseek-harness")
            .expect("static DeepSeek Harness family id is valid"),
        TransportFamilyId::new("deepseek-harness-jsonrpc-ndjson-stdio")
            .expect("static DeepSeek Harness transport id is valid"),
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
    .with_interface_compatibility(crate::deepseek_harness_jsonrpc_claim())
}

/// Low-level exact DeepSeek Harness JSON-RPC driver.
pub struct DeepSeekHarnessJsonRpcDriver {
    environment: EnvironmentRef,
}

impl DeepSeekHarnessJsonRpcDriver {
    #[must_use]
    /// Creates a driver with one host-approved Cordis configuration reference.
    pub const fn new(environment: EnvironmentRef) -> Self {
        Self { environment }
    }

    #[must_use]
    /// Returns the approved Cordis configuration reference.
    pub const fn environment(&self) -> &EnvironmentRef {
        &self.environment
    }
}

impl StructuredRunDriver for DeepSeekHarnessJsonRpcDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        Box::pin(async move { self.start(plan, request, services).await })
    }
}

impl DeepSeekHarnessJsonRpcDriver {
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
        let provider = plan
            .provider_id()
            .expect("validated provider route")
            .as_str()
            .to_owned();
        let model = plan.model_id().cloned().expect("validated model route");
        let working_resource = request
            .working_resource()
            .cloned()
            .expect("validated working resource");
        let deadline = request.deadline().expect("validated deadline");
        let run_id = runtime_run_id(request.request_id().as_str())?;
        let scope = runtime_scope(request.request_id().as_str())?;
        let session_id = format!(
            "swallowtail.deepseek-harness.{}",
            request.request_id().as_str()
        );
        let (event_sender, event_stream) = runtime_event_channel(EVENT_CAPACITY)?;
        let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
            plan.instance_target_ref(),
        ))
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
        let cancellation = Arc::new(DeepSeekHarnessCancellation::new(Arc::clone(&process)));
        let task = task_service.spawn(
            scope,
            Box::pin({
                let cancellation = Arc::clone(&cancellation);
                let process = Arc::clone(&process);
                let operation_id = ActivityOperationId::Run(run_id.clone());
                let services = services.clone();
                let cwd = working_resource.as_host_value().to_owned();
                let provider = provider.clone();
                let model = model.as_str().to_owned();
                let prompt = request.content().as_str().to_owned();
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
                        session_id,
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
        Ok(Box::new(DeepSeekHarnessRunHandle::new(
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
    RuntimeRunId::new(format!("deepseek-harness:{request_id}")).map_err(|_| invalid_request())
}

fn runtime_scope(request_id: &str) -> Result<ScopeId, RuntimeFailure> {
    ScopeId::new(format!("deepseek-harness:{request_id}")).map_err(|_| invalid_request())
}

fn invalid_request() -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.deepseek_harness.invalid_request",
        "DeepSeek Harness request identity is invalid",
    )
}

#[cfg(test)]
mod tests {
    use super::deepseek_harness_jsonrpc_descriptor;
    use swallowtail_core::{DriverRole, ExecutionLayer, OperationShape};

    #[test]
    fn descriptor_is_one_separate_structured_harness_route() {
        let descriptor = deepseek_harness_jsonrpc_descriptor();
        assert_eq!(descriptor.integration_family().as_str(), "deepseek-harness");
        assert_eq!(
            descriptor.transport_family().as_str(),
            "deepseek-harness-jsonrpc-ndjson-stdio"
        );
        assert!(descriptor.supports_role(DriverRole::Discovery));
        assert!(descriptor.supports_role(DriverRole::StructuredRun));
        assert!(descriptor.supports_execution_layer(ExecutionLayer::HarnessInteraction));
        assert!(descriptor.supports_operation_shape(OperationShape::StructuredRun));
        assert!(!descriptor.supports_role(DriverRole::InteractiveSession));
    }
}
