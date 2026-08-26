use crate::headless_command::{CursorHeadlessReadMode, arguments};
use crate::headless_handle::{CursorHeadlessCancellation, CursorHeadlessRunHandle};
use crate::headless_pump::{cleanup_failed_start, pump};
use std::sync::Arc;
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    ActivityOperationId, BoxFuture, EnvironmentRef, ExecutableRef, HostServices, ProcessHandle,
    ProcessInputChunk, ProcessRequest, RunHandle, RuntimeEvent, RuntimeEventKind, RuntimeFailure,
    RuntimeRunId, ScopeId, StructuredRunDriver, StructuredRunRequest, runtime_event_channel,
    terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 4098;

/// Low-level driver for one-shot Cursor stream-JSON runs.
pub struct CursorHeadlessDriver {
    environment: EnvironmentRef,
    read_mode: Option<CursorHeadlessReadMode>,
}

impl CursorHeadlessDriver {
    /// Creates a headless driver using the approved execution environment.
    ///
    /// Without an explicit read mode the driver keeps the exact default argv:
    /// `ResourceAccess::Read` dispatches `--mode plan` and
    /// `ResourceAccess::ReadWrite` dispatches no mode.
    #[must_use]
    pub const fn new(environment: EnvironmentRef) -> Self {
        Self {
            environment,
            read_mode: None,
        }
    }

    /// Binds one exact Cursor-local read mode to this driver.
    ///
    /// The selection is rejected before process work unless the preflight plan
    /// carries `ResourceAccess::Read`, and `Ask` additionally requires an
    /// exactly qualified Cursor release. A newer unverified release, a
    /// read-write plan, or a mismatched low-level use fails closed; the driver
    /// never falls back to another mode.
    #[must_use]
    pub fn with_read_mode(mut self, read_mode: CursorHeadlessReadMode) -> Self {
        self.read_mode = Some(read_mode);
        self
    }

    /// Returns the approved execution environment.
    #[must_use]
    pub const fn environment(&self) -> &EnvironmentRef {
        &self.environment
    }

    /// Returns the exact Cursor-local read mode bound to this driver.
    #[must_use]
    pub const fn read_mode(&self) -> Option<CursorHeadlessReadMode> {
        self.read_mode
    }
}

impl StructuredRunDriver for CursorHeadlessDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        Box::pin(async move { self.start(plan, request, services).await })
    }
}

impl CursorHeadlessDriver {
    async fn start(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> Result<Box<dyn RunHandle>, RuntimeFailure> {
        let access = crate::headless_validation::validate(&plan, &request, &services)?;
        let read_mode =
            crate::headless_validation::validate_read_mode(&plan, access, self.read_mode)?;
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
        let run_id =
            RuntimeRunId::new(format!("cursor-headless:{}", request.request_id().as_str()))
                .map_err(|_| crate::failure::malformed())?;
        let scope = ScopeId::new(format!("cursor-headless:{}", request.request_id().as_str()))
            .map_err(|_| crate::failure::malformed())?;
        let (event_sender, event_stream) = runtime_event_channel(EVENT_CAPACITY)?;
        let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
            plan.instance_target_ref(),
        ))
        .with_arguments(arguments(&model, read_mode))
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
        if let Err(error) = event_sender.send(RuntimeEvent::new(0, RuntimeEventKind::Started)) {
            cleanup_failed_start(process.as_ref()).await;
            return Err(error);
        }
        let (terminal_sender, terminal_future) = terminal_outcome_channel();
        let cancellation = Arc::new(CursorHeadlessCancellation::new(Arc::clone(&process)));
        let task = task_service.spawn(
            scope,
            Box::pin({
                let cancellation = Arc::clone(&cancellation);
                let process = Arc::clone(&process);
                let operation_id = ActivityOperationId::Run(run_id.clone());
                let services = services.clone();
                async move {
                    let outcome = pump(
                        process,
                        event_sender.clone(),
                        cancellation,
                        time_service.wait_until(deadline),
                        model,
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
                cleanup_failed_start(process.as_ref()).await;
                return Err(error);
            }
        };
        Ok(Box::new(CursorHeadlessRunHandle::new(
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
        crate::failure::failure(
            "swallowtail.cursor.headless.stdin_close_failed",
            "Cursor headless process stdin could not be closed",
        )
    })
}
