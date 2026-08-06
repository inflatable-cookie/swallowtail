use crate::headless_command::{HeadlessCommand, arguments};
use crate::headless_handle::{AntigravityHeadlessCancellation, AntigravityHeadlessRunHandle};
use crate::headless_pump::{cleanup_failed_start, pump};
use std::sync::Arc;
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    ActivityOperationId, BoxFuture, EnvironmentRef, ExecutableRef, HostServices, ProcessHandle,
    ProcessRequest, RunHandle, RuntimeEvent, RuntimeEventKind, RuntimeFailure, RuntimeRunId,
    ScopeId, StructuredRunDriver, StructuredRunRequest, runtime_event_channel,
    terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 4098;

/// Low-level driver for Antigravity stream-JSON headless execution.
pub struct AntigravityHeadlessDriver {
    environment: EnvironmentRef,
}

impl AntigravityHeadlessDriver {
    #[must_use]
    /// Creates a headless driver with one approved process environment.
    pub const fn new(environment: EnvironmentRef) -> Self {
        Self { environment }
    }

    #[must_use]
    /// Returns the approved process environment.
    pub const fn environment(&self) -> &EnvironmentRef {
        &self.environment
    }
}

impl StructuredRunDriver for AntigravityHeadlessDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        Box::pin(async move { self.start(plan, request, services).await })
    }
}

impl AntigravityHeadlessDriver {
    async fn start(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> Result<Box<dyn RunHandle>, RuntimeFailure> {
        let validated = crate::headless_validation::validate(&plan, &request, &services)?;
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
        let run_id = RuntimeRunId::new(format!(
            "antigravity-headless:{}",
            request.request_id().as_str()
        ))
        .map_err(|_| {
            crate::failure::failure(
                "swallowtail.antigravity.headless.invalid_request",
                "Antigravity headless request identity is invalid",
            )
        })?;
        let scope = ScopeId::new(format!(
            "antigravity-headless:{}",
            request.request_id().as_str()
        ))
        .map_err(|_| {
            crate::failure::failure(
                "swallowtail.antigravity.headless.invalid_request",
                "Antigravity headless request identity is invalid",
            )
        })?;
        let (event_sender, event_stream) = runtime_event_channel(EVENT_CAPACITY)?;
        let schema_expected = validated.schema.is_some();
        let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
            plan.instance_target_ref(),
        ))
        .with_arguments(arguments(HeadlessCommand {
            prompt: request.content().as_str(),
            model: &model,
            access: validated.access,
            isolation: validated.isolation,
            effort: validated.effort.as_ref(),
            schema: validated.schema.as_deref(),
            conversation_id: None,
        }))
        .with_environment([self.environment.clone()])
        .with_working_resource(working_resource);
        let process: Arc<dyn ProcessHandle> = Arc::from(
            process_service
                .start(scope.clone(), process_request)
                .await?,
        );
        if process.close_stdin().await.is_err() {
            cleanup_failed_start(process.as_ref()).await;
            return Err(crate::failure::failure(
                "swallowtail.antigravity.headless.stdin_close_failed",
                "Antigravity headless process stdin could not be closed",
            ));
        }
        if let Err(error) = event_sender.send(RuntimeEvent::new(0, RuntimeEventKind::Started)) {
            cleanup_failed_start(process.as_ref()).await;
            return Err(error);
        }
        let (terminal_sender, terminal_future) = terminal_outcome_channel();
        let cancellation = Arc::new(AntigravityHeadlessCancellation::new(Arc::clone(&process)));
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
                        time_service.wait_until(deadline),
                        model,
                        schema_expected,
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
        Ok(Box::new(AntigravityHeadlessRunHandle::new(
            request.request_id().clone(),
            run_id,
            Box::pin(event_stream),
            Box::pin(terminal_future),
            cancellation,
            task,
        )))
    }
}
