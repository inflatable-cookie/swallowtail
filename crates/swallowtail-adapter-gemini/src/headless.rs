use crate::failure::failure;
use crate::headless_command::arguments;
use crate::headless_handle::{GeminiHeadlessCancellation, GeminiHeadlessRunHandle};
use crate::headless_pump::{HeadlessProjection, TranscriptCleanup, cleanup_failed_start, pump};
use std::sync::Arc;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, CredentialMechanism, CredentialRef,
    DriverDescriptor, DriverRole, ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation,
    HostServiceKind, IntegrationFamilyId, OperationShape, PreflightPlan, RunRef, TransportFamilyId,
};
use swallowtail_runtime::{
    ActivityOperationId, BoxFuture, EnvironmentRef, ExecutableRef, HostServices, ProcessHandle,
    ProcessInputChunk, ProcessRequest, RunHandle, RuntimeEvent, RuntimeEventKind, RuntimeFailure,
    RuntimeRunId, ScopeId, StructuredRunDriver, StructuredRunRequest, runtime_event_channel,
    terminal_outcome_channel,
};

pub(crate) const DRIVER_ID: &str = "swallowtail.gemini.headless";
const EVENT_CAPACITY: usize = 4098;

pub struct GeminiHeadlessDriver {
    pub(crate) environment: EnvironmentRef,
    pub(crate) credential: CredentialRef,
}

impl GeminiHeadlessDriver {
    #[must_use]
    pub const fn new(environment: EnvironmentRef, credential: CredentialRef) -> Self {
        Self {
            environment,
            credential,
        }
    }
}

#[must_use]
pub fn gemini_headless_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("gemini-cli").expect("static family id is valid"),
        TransportFamilyId::new("gemini-stream-json-stdio").expect("static transport id is valid"),
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
    .with_interface_compatibility(crate::gemini_cli_headless_claim())
}

impl StructuredRunDriver for GeminiHeadlessDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        Box::pin(async move { self.start(plan, request, services).await })
    }
}

impl GeminiHeadlessDriver {
    async fn start(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> Result<Box<dyn RunHandle>, RuntimeFailure> {
        crate::headless_validation::validate(&plan, &request, &services, &self.credential)?;
        let owned_transcript_cleanup = crate::headless_validation::owns_transcript_cleanup(&plan)?;
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
        let run_id =
            RuntimeRunId::new(format!("gemini-headless:{}", request.request_id().as_str()))
                .map_err(|_| {
                    failure(
                        "swallowtail.gemini.headless.run_id_invalid",
                        "Gemini headless runtime run identity was invalid",
                    )
                })?;
        let provider_id = provider_session_id(request.request_id());
        let provider_run_ref = RunRef::new(&provider_id).map_err(|_| {
            failure(
                "swallowtail.gemini.headless.provider_run_id_invalid",
                "Gemini headless provider-run identity was invalid",
            )
        })?;
        let scope = ScopeId::new(format!("gemini-headless:{}", request.request_id().as_str()))
            .map_err(|_| {
                failure(
                    "swallowtail.gemini.headless.scope_invalid",
                    "Gemini headless operation scope was invalid",
                )
            })?;
        let (event_sender, event_stream) = runtime_event_channel(EVENT_CAPACITY)?;
        let executable = ExecutableRef::from_instance_target(plan.instance_target_ref());
        let process_request = ProcessRequest::new(executable.clone())
            .with_arguments(arguments(&model, &provider_id))
            .with_environment([self.environment.clone()])
            .with_working_resource(working_resource.clone());
        let process: Arc<dyn ProcessHandle> = Arc::from(
            process_service
                .start(scope.clone(), process_request)
                .await?,
        );
        if let Err(error) = write_prompt(process.as_ref(), &request).await {
            cleanup_failed_start(process.as_ref()).await;
            return Err(error);
        }
        let run_deadline = time_service.wait_until(deadline);
        let cleanup_deadline = time_service.wait_until(deadline);
        if let Err(error) = event_sender.send(RuntimeEvent::new(0, RuntimeEventKind::Started)) {
            cleanup_failed_start(process.as_ref()).await;
            return Err(error);
        }
        let (terminal_sender, terminal_future) = terminal_outcome_channel();
        let cancellation = Arc::new(GeminiHeadlessCancellation::new(Arc::clone(&process)));
        let cleanup_environment = self.environment.clone();
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
                        run_deadline,
                        HeadlessProjection {
                            model,
                            session_id: provider_id.clone(),
                            operation_id,
                        },
                        owned_transcript_cleanup.then_some(TranscriptCleanup {
                            process_service,
                            executable,
                            environment: cleanup_environment,
                            working_resource,
                            session_id: provider_id,
                            deadline: cleanup_deadline,
                        }),
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
        Ok(Box::new(GeminiHeadlessRunHandle::new(
            request.request_id().clone(),
            run_id,
            provider_run_ref,
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
            "swallowtail.gemini.headless.stdin_close_failed",
            "Gemini headless process stdin could not be closed",
        )
    })
}

pub(crate) fn provider_session_id(request_id: &swallowtail_runtime::RequestId) -> String {
    use std::fmt::Write;

    let mut value = String::from("swallowtail-");
    for byte in request_id.as_str().bytes() {
        write!(&mut value, "{byte:02x}").expect("writing to a string cannot fail");
    }
    value
}

pub(crate) fn validate_headless_plan(
    plan: &PreflightPlan,
    credential: &CredentialRef,
) -> Result<(), RuntimeFailure> {
    if plan.driver_identity().id().as_str() != DRIVER_ID {
        return Err(failure(
            "swallowtail.gemini.headless.plan_driver_mismatch",
            "Gemini headless plan is bound to a different driver",
        ));
    }
    if plan.credential_mechanism() != &CredentialMechanism::ApiKey
        || plan.credential_reference() != Some(credential)
        || plan.endpoint_audience().as_str() != "gemini-developer-api"
    {
        return Err(failure(
            "swallowtail.gemini.headless.access_profile_rejected",
            "Gemini headless requires its configured Developer API key profile",
        ));
    }
    if plan.harness_configuration_posture() != Some(HarnessConfigurationPosture::Ambient)
        || plan.requirements().harness_isolation() != Some(HarnessIsolation::AmbientHost)
    {
        return Err(failure(
            "swallowtail.gemini.headless.ambient_authority_rejected",
            "Gemini headless requires explicit ambient configuration and isolation authority",
        ));
    }
    crate::selection::select_gemini_headless_plan(plan)?;
    Ok(())
}
