use crate::DRIVER_ID;
use crate::command::arguments;
use crate::handle::{QwenProcessCancellation, QwenRunHandle};
use crate::pump::{cleanup_failed_start, pump};
use crate::validation::{failure, validate};
use std::sync::Arc;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, DriverDescriptor, DriverRole, ExecutionLayer,
    HostServiceKind, IntegrationFamilyId, OperationShape, PreflightPlan, TransportFamilyId,
};
use swallowtail_runtime::{
    BoxFuture, EnvironmentRef, ExecutableRef, HostServices, ProcessHandle, ProcessInputChunk,
    ProcessRequest, RunHandle, RuntimeEvent, RuntimeEventKind, RuntimeFailure, RuntimeRunId,
    ScopeId, StructuredRunDriver, StructuredRunRequest, runtime_event_channel,
    terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 4098;

pub struct QwenHeadlessDriver {
    environment: EnvironmentRef,
}

impl QwenHeadlessDriver {
    #[must_use]
    pub const fn new(environment: EnvironmentRef) -> Self {
        Self { environment }
    }
}

#[must_use]
pub fn qwen_headless_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("qwen-code").expect("static family id is valid"),
        TransportFamilyId::new("structured-cli-stream-json").expect("static transport id is valid"),
    )
    .with_roles([DriverRole::StructuredRun])
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
}

impl StructuredRunDriver for QwenHeadlessDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        Box::pin(async move { self.start(plan, request, services).await })
    }
}

impl QwenHeadlessDriver {
    async fn start(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> Result<Box<dyn RunHandle>, RuntimeFailure> {
        validate(&plan, &request, &services)?;
        let task_service = services
            .task()
            .cloned()
            .expect("validated task service is present");
        let process_service = services
            .process()
            .cloned()
            .expect("validated process service is present");
        let time_service = services
            .time()
            .cloned()
            .expect("validated time service is present");
        let model = plan
            .model_id()
            .cloned()
            .expect("validated model binding is present");
        let working_resource = request
            .working_resource()
            .cloned()
            .expect("validated working resource is present");
        let deadline = request.deadline().expect("validated deadline is present");
        let scope = ScopeId::new(format!("qwen-headless:{}", request.request_id().as_str()))
            .expect("request id produces a non-empty scope id");
        let (event_sender, event_stream) = runtime_event_channel(EVENT_CAPACITY)?;
        let executable = ExecutableRef::from_instance_target(plan.instance_target_ref());
        let process_request = ProcessRequest::new(executable)
            .with_arguments(arguments(&model))
            .with_environment([self.environment.clone()])
            .with_working_resource(working_resource);
        let process: Arc<dyn ProcessHandle> = Arc::from(
            process_service
                .start(scope.clone(), process_request)
                .await?,
        );
        if let Err(error) = write_prompt(process.as_ref(), &request).await {
            cleanup_failed_start(process.as_ref()).await;
            return Err(error);
        }
        let deadline = time_service.wait_until(deadline);
        if let Err(error) = event_sender.send(RuntimeEvent::new(0, RuntimeEventKind::Started)) {
            cleanup_failed_start(process.as_ref()).await;
            return Err(error);
        }
        let (terminal_sender, terminal_future) = terminal_outcome_channel();
        let cancellation = Arc::new(QwenProcessCancellation::new(Arc::clone(&process)));
        let task = task_service.spawn(
            scope,
            Box::pin({
                let cancellation = Arc::clone(&cancellation);
                let process = Arc::clone(&process);
                async move {
                    let outcome =
                        pump(process, event_sender.clone(), cancellation, deadline, model).await;
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
        let run_id = RuntimeRunId::new(format!("qwen-headless:{}", request.request_id().as_str()))
            .expect("request id produces a non-empty run id");
        Ok(Box::new(QwenRunHandle::new(
            request.request_id().clone(),
            run_id,
            Box::pin(event_stream),
            Box::pin(terminal_future),
            cancellation,
            task,
        )))
    }
}

async fn write_prompt(
    process: &dyn ProcessHandle,
    request: &StructuredRunRequest,
) -> Result<(), RuntimeFailure> {
    process
        .write_stdin(ProcessInputChunk::new(
            request.content().as_str().as_bytes().to_vec(),
        ))
        .await?;
    process.close_stdin().await.map_err(|_| {
        failure(
            "swallowtail.qwen.headless.stdin_close_failed",
            "Qwen headless process stdin could not be closed",
        )
    })
}
