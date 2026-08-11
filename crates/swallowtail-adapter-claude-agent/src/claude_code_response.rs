use crate::claude_code_handle::{ClaudeCodeCancellation, ClaudeCodeRunHandle};
use crate::claude_code_response_command::arguments;
use crate::claude_code_response_pump::{cleanup_failed_start, pump};
use crate::failure::failure;
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

pub(crate) const DRIVER_ID: &str = "swallowtail.claude-code.response-only";
const EVENT_CAPACITY: usize = 32;

/// Low-level driver for tool-free one-shot `claude -p` text responses.
pub struct ClaudeCodeResponseOnlyDriver {
    environment: EnvironmentRef,
}

impl ClaudeCodeResponseOnlyDriver {
    /// Creates a response-only driver using the approved subscription environment.
    #[must_use]
    pub const fn new(environment: EnvironmentRef) -> Self {
        Self { environment }
    }
}

#[must_use]
/// Describes the exact Claude Code response-only route.
pub fn claude_code_response_only_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("claude-code").expect("static family id is valid"),
        TransportFamilyId::new("claude-code-stream-json-stdio")
            .expect("static transport id is valid"),
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
    .with_interface_compatibility(crate::claude_code_response_only_claim())
}

impl StructuredRunDriver for ClaudeCodeResponseOnlyDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        Box::pin(async move { self.start(plan, request, services).await })
    }
}

impl ClaudeCodeResponseOnlyDriver {
    async fn start(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> Result<Box<dyn RunHandle>, RuntimeFailure> {
        crate::claude_code_response_validation::validate(&plan, &request, &services)?;
        let task_service = services.task().cloned().expect("validated task service");
        let process_service = services
            .process()
            .cloned()
            .expect("validated process service");
        let time_service = services.time().cloned().expect("validated time service");
        let model = plan.model_id().cloned().expect("validated model binding");
        let deadline = request.deadline().expect("validated deadline");
        let run_id = RuntimeRunId::new(format!(
            "claude-code-response-only:{}",
            request.request_id().as_str()
        ))
        .map_err(|_| {
            failure(
                "swallowtail.claude_code.response_only.run_id_invalid",
                "Claude Code response-only runtime run identity was invalid",
            )
        })?;
        let scope = ScopeId::new(format!(
            "claude-code-response-only:{}",
            request.request_id().as_str()
        ))
        .map_err(|_| {
            failure(
                "swallowtail.claude_code.response_only.scope_invalid",
                "Claude Code response-only operation scope was invalid",
            )
        })?;
        let (event_sender, event_stream) = runtime_event_channel(EVENT_CAPACITY)?;
        let process_request = ProcessRequest::new(ExecutableRef::from_instance_target(
            plan.instance_target_ref(),
        ))
        .with_arguments(arguments(&model, request.policy().reasoning_mode()))
        .with_environment([self.environment.clone()]);
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
        let cancellation = Arc::new(ClaudeCodeCancellation::new(Arc::clone(&process)));
        let pump_run_id = run_id.clone();
        let task = task_service.spawn(
            scope,
            Box::pin({
                let cancellation = Arc::clone(&cancellation);
                let process = Arc::clone(&process);
                let services = services.clone();
                async move {
                    let outcome = pump(
                        process,
                        event_sender.clone(),
                        cancellation,
                        deadline,
                        model,
                        pump_run_id,
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
        Ok(Box::new(ClaudeCodeRunHandle::new_response_only(
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
            "swallowtail.claude_code.response_only.stdin_close_failed",
            "Claude Code response-only process stdin could not be closed",
        )
    })
}
