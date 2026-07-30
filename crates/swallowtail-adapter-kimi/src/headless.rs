use crate::failure::failure;
use crate::headless_command::arguments;
use crate::headless_handle::{KimiHeadlessCancellation, KimiHeadlessRunHandle};
use crate::headless_pump::{cleanup_failed_start, pump};
use std::sync::Arc;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, CredentialRef, DriverDescriptor, DriverRole,
    ExecutionLayer, HostServiceKind, IntegrationFamilyId, OperationShape, PreflightPlan,
    TransportFamilyId,
};
use swallowtail_runtime::{
    ActivityOperationId, BoxFuture, DiscoveryDriver, DiscoveryRequest, EnvironmentRef,
    ExecutableRef, HostServices, InstalledExecutableDiscoveryRequest, ProcessHandle,
    ProcessRequest, RunHandle, RuntimeEvent, RuntimeEventKind, RuntimeFailure, RuntimeRunId,
    ScopeId, StructuredRunDriver, StructuredRunRequest, runtime_event_channel,
    terminal_outcome_channel,
};

pub(crate) const DRIVER_ID: &str = "swallowtail.kimi.headless";
const EVENT_CAPACITY: usize = 4098;

pub struct KimiHeadlessDriver {
    environment: EnvironmentRef,
    credential: CredentialRef,
}

impl KimiHeadlessDriver {
    #[must_use]
    pub const fn new(environment: EnvironmentRef, credential: CredentialRef) -> Self {
        Self {
            environment,
            credential,
        }
    }
}

#[must_use]
pub fn kimi_headless_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("kimi-code").expect("static family id is valid"),
        TransportFamilyId::new("kimi-stream-json-stdio").expect("static transport id is valid"),
    )
    .with_roles([DriverRole::Discovery, DriverRole::StructuredRun])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([OperationShape::StructuredRun])
    .with_required_host_services(
        DriverRole::StructuredRun,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::Time,
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
    .with_interface_compatibility(crate::kimi_headless_claim())
}

impl DiscoveryDriver for KimiHeadlessDriver {
    fn discover(
        &self,
        _request: DiscoveryRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Vec<swallowtail_core::DiscoveryOutcome>, RuntimeFailure>> {
        Box::pin(async {
            Err(failure(
                "swallowtail.kimi.headless.discovery_target_required",
                "Kimi headless discovery requires one explicit host-approved executable target",
            ))
        })
    }

    fn discover_installed_executable(
        &self,
        request: InstalledExecutableDiscoveryRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<swallowtail_core::DiscoveryOutcome, RuntimeFailure>> {
        Box::pin(crate::discovery::probe_joined(
            request,
            services,
            crate::kimi_headless_claim(),
        ))
    }
}

impl StructuredRunDriver for KimiHeadlessDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        Box::pin(async move { self.start(plan, request, services).await })
    }
}

impl KimiHeadlessDriver {
    async fn start(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> Result<Box<dyn RunHandle>, RuntimeFailure> {
        crate::headless_validation::validate(&plan, &request, &services, &self.credential)?;
        let task_service = services.task().cloned().expect("validated task service");
        let process_service = services
            .process()
            .cloned()
            .expect("validated process service");
        let time_service = services.time().cloned().expect("validated time service");
        let model = plan.model_id().expect("validated model binding");
        let working_resource = request
            .working_resource()
            .cloned()
            .expect("validated working resource");
        let deadline = request.deadline().expect("validated deadline");
        let run_id = RuntimeRunId::new(format!("kimi-headless:{}", request.request_id().as_str()))
            .map_err(|_| {
                failure(
                    "swallowtail.kimi.headless.run_id_invalid",
                    "Kimi headless runtime run identity was invalid",
                )
            })?;
        let scope = ScopeId::new(format!("kimi-headless:{}", request.request_id().as_str()))
            .map_err(|_| {
                failure(
                    "swallowtail.kimi.headless.scope_invalid",
                    "Kimi headless operation scope was invalid",
                )
            })?;
        let (event_sender, event_stream) = runtime_event_channel(EVENT_CAPACITY)?;
        let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
            plan.instance_target_ref(),
        ))
        .with_arguments(arguments(model, request.content()))
        .with_environment([self.environment.clone()])
        .with_working_resource(working_resource);
        let process: Arc<dyn ProcessHandle> = Arc::from(
            process_service
                .start(scope.clone(), process_request)
                .await?,
        );
        if let Err(error) = process.close_stdin().await {
            cleanup_failed_start(process.as_ref()).await;
            return Err(error);
        }
        let deadline = time_service.wait_until(deadline);
        if let Err(error) = event_sender.send(RuntimeEvent::new(0, RuntimeEventKind::Started)) {
            cleanup_failed_start(process.as_ref()).await;
            return Err(error);
        }
        let (terminal_sender, terminal_future) = terminal_outcome_channel();
        let cancellation = Arc::new(KimiHeadlessCancellation::new(Arc::clone(&process)));
        let task = task_service.spawn(
            scope,
            Box::pin({
                let cancellation = Arc::clone(&cancellation);
                let process = Arc::clone(&process);
                let operation_id = ActivityOperationId::Run(run_id.clone());
                async move {
                    let outcome = pump(
                        process,
                        event_sender.clone(),
                        cancellation,
                        deadline,
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
                cleanup_failed_start(process.as_ref()).await;
                return Err(error);
            }
        };
        Ok(Box::new(KimiHeadlessRunHandle::new(
            request.request_id().clone(),
            run_id,
            Box::pin(event_stream),
            Box::pin(terminal_future),
            cancellation,
            task,
        )))
    }
}
