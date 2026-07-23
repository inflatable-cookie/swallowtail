use crate::exec_handle::{CodexExecRunHandle, ProcessCancellation};
use crate::exec_input::{SharedExecMaterializations, prepare};
use crate::exec_pump::{cleanup_failed_start, pump};
use crate::selection::{classify_exec_plan, codex_exec_claim};
use std::sync::Arc;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, DriverDescriptor, DriverRole, ExecutionLayer,
    HostServiceKind, IntegrationFamilyId, OperationShape, PreflightPlan, SafeDiagnostic,
    TransportFamilyId,
};
use swallowtail_runtime::{
    BoxFuture, EnvironmentRef, ExecutableRef, HostServices, ProcessHandle, ProcessInputChunk,
    ProcessRequest, RunHandle, RuntimeEvent, RuntimeEventKind, RuntimeFailure, RuntimeRunId,
    ScopeId, StructuredRunDriver, StructuredRunRequest, runtime_event_channel,
    terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 256;

pub struct CodexExecDriver {
    environment: EnvironmentRef,
}

impl CodexExecDriver {
    #[must_use]
    pub const fn new(environment: EnvironmentRef) -> Self {
        Self { environment }
    }
}

#[must_use]
pub fn codex_exec_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new("swallowtail.codex.exec").expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("codex").expect("static family id is valid"),
        TransportFamilyId::new("structured-cli").expect("static transport id is valid"),
    )
    .with_roles([DriverRole::Discovery, DriverRole::StructuredRun])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([OperationShape::StructuredRun])
    .with_required_host_services(
        DriverRole::StructuredRun,
        [HostServiceKind::Task, HostServiceKind::Process],
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
    .with_interface_compatibility(codex_exec_claim())
}

impl StructuredRunDriver for CodexExecDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        Box::pin(async move { self.start(plan, request, services).await })
    }
}

impl CodexExecDriver {
    async fn start(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> Result<Box<dyn RunHandle>, RuntimeFailure> {
        if plan.driver_identity().id().as_str() != "swallowtail.codex.exec" {
            return Err(failure(
                "swallowtail.codex.exec.plan_driver_mismatch",
                "Preflight plan is bound to a different driver",
            ));
        }
        let behavior = classify_exec_plan(&plan)?;
        services.require_execution_host(plan.execution_host_id())?;
        let task_service = services.task().cloned().ok_or_else(|| {
            failure(
                "swallowtail.codex.exec.task_service_missing",
                "Codex exec requires a scoped task service",
            )
        })?;
        let process_service = services.process().cloned().ok_or_else(|| {
            failure(
                "swallowtail.codex.exec.process_service_missing",
                "Codex exec requires a process service",
            )
        })?;
        let model = plan.model_id().ok_or_else(|| {
            failure(
                "swallowtail.codex.exec.model_missing",
                "Codex exec requires a preflight-bound model",
            )
        })?;
        let working_resource = request.working_resource().ok_or_else(|| {
            failure(
                "swallowtail.codex.exec.working_resource_missing",
                "Codex exec requires a working resource",
            )
        })?;
        let scope = ScopeId::new(format!("codex-exec:{}", request.request_id().as_str()))
            .expect("request id produces a non-empty scope id");
        let (event_sender, event_stream) = runtime_event_channel(EVENT_CAPACITY)?;
        let prepared = prepare(&plan, &request, &services, &scope, model, behavior).await?;
        let (arguments, materializations) = prepared.into_parts();
        let materializations = SharedExecMaterializations::new(materializations);
        let executable = ExecutableRef::from_instance_target(plan.instance_target_ref());
        let process_request = ProcessRequest::new(executable)
            .with_arguments(arguments)
            .with_environment([self.environment.clone()])
            .with_working_resource(working_resource.clone());
        let process: Arc<dyn ProcessHandle> =
            match process_service.start(scope.clone(), process_request).await {
                Ok(process) => Arc::from(process),
                Err(error) => {
                    let _ = materializations.release().await;
                    return Err(error);
                }
            };
        if let Err(error) = write_prompt(process.as_ref(), &request).await {
            cleanup_failed_start(process.as_ref()).await;
            let _ = materializations.release().await;
            return Err(error);
        }

        let deadline = request.deadline().map(|deadline| {
            services
                .time()
                .expect("validated time service is present")
                .wait_until(deadline)
        });
        if let Err(error) = event_sender.send(RuntimeEvent::new(0, RuntimeEventKind::Started)) {
            cleanup_failed_start(process.as_ref()).await;
            let _ = materializations.release().await;
            return Err(error);
        }
        let (terminal_sender, terminal_future) = terminal_outcome_channel();
        let cancellation = Arc::new(ProcessCancellation::new(Arc::clone(&process)));
        let task = task_service.spawn(
            scope,
            Box::pin({
                let cancellation = Arc::clone(&cancellation);
                let process = Arc::clone(&process);
                let materializations = materializations.clone();
                async move {
                    let outcome = pump(
                        process,
                        event_sender.clone(),
                        cancellation,
                        deadline,
                        materializations,
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
                let _ = materializations.release().await;
                return Err(error);
            }
        };
        let run_id = RuntimeRunId::new(format!("codex-exec:{}", request.request_id().as_str()))
            .expect("request id produces a non-empty run id");
        Ok(Box::new(CodexExecRunHandle::new(
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
    process.close_stdin().await
}

pub(crate) fn failure(code: &'static str, message: impl Into<String>) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}
