use crate::claude_code_command::arguments;
use crate::claude_code_handle::{ClaudeCodeCancellation, ClaudeCodeRunHandle};
use crate::claude_code_pump::{PumpHost, cleanup_failed_start, pump};
use crate::failure::failure;
use std::sync::Arc;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, CredentialMechanism, DriverDescriptor, DriverRole,
    ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation, HostServiceKind,
    IntegrationFamilyId, OperationShape, PreflightPlan, TransportFamilyId,
};
use swallowtail_runtime::{
    ActivityOperationId, BoxFuture, EnvironmentRef, ExecutableRef, HostServices, ProcessHandle,
    ProcessInputChunk, ProcessRequest, RunHandle, RuntimeEvent, RuntimeEventKind, RuntimeFailure,
    RuntimeRunId, RuntimeTurnId, ScopeId, StructuredRunDriver, StructuredRunRequest,
    runtime_event_channel, terminal_outcome_channel,
};

pub(crate) const DRIVER_ID: &str = "swallowtail.claude-code.headless";
const EVENT_CAPACITY: usize = 4098;
pub(crate) const ENDPOINT_AUDIENCE: &str = "anthropic-claude-code";

/// Low-level driver for native one-shot `claude -p` stream-JSON runs.
pub struct ClaudeCodeHeadlessDriver {
    environment: EnvironmentRef,
    maximum_turns: Option<crate::ClaudeCodeMaximumTurns>,
    watchers: bool,
}

impl ClaudeCodeHeadlessDriver {
    /// Creates a headless driver using the approved execution environment.
    #[must_use]
    pub const fn new(environment: EnvironmentRef) -> Self {
        Self {
            environment,
            maximum_turns: None,
            watchers: false,
        }
    }

    /// Configures one admitted `--max-turns` value for low-level dispatch.
    ///
    /// Deliberately crate-private, and reached only from
    /// [`crate::ClaudeCodePreparedRun::start_run`], which pairs the bound with
    /// its own plan and request. No public seam attaches a bound to a driver:
    /// a hand-built driver cannot be given one, and
    /// [`crate::ClaudeCodePreparedRun::low_level_driver`] deliberately returns
    /// an unbound driver.
    ///
    /// That is what keeps prepared and dispatched state in agreement. Neither
    /// `PreflightPlan` nor `StructuredRunRequest` records a maximum-turn bound,
    /// so a bound driver reachable from outside its own prepared run could be
    /// handed another run's plan and silently dispatch the wrong value. The
    /// bound and its `(plan, request)` pair are only ever brought together in
    /// one place, so they cannot disagree.
    #[must_use]
    pub(crate) const fn with_maximum_turns(
        mut self,
        maximum_turns: crate::ClaudeCodeMaximumTurns,
    ) -> Self {
        self.maximum_turns = Some(maximum_turns);
        self
    }

    /// Opts the driver into exact `2.1.251` watcher composition.
    ///
    /// Crate-private and reached only from prepared `start_run`.
    #[must_use]
    pub(crate) const fn with_watchers(mut self) -> Self {
        self.watchers = true;
        self
    }
}

#[must_use]
/// Describes the native Claude Code headless route.
pub fn claude_code_headless_descriptor() -> DriverDescriptor {
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
    .with_interface_compatibility(crate::claude_code_headless_claim())
}

impl StructuredRunDriver for ClaudeCodeHeadlessDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        Box::pin(async move { self.start(plan, request, services).await })
    }
}

impl ClaudeCodeHeadlessDriver {
    async fn start(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> Result<Box<dyn RunHandle>, RuntimeFailure> {
        crate::claude_code_validation::validate(&plan, &request, &services)?;
        // A bound only reaches this driver through prepared `start_run`, which
        // always supplies its own plan, so this cannot fire through any public
        // path today. It is kept as a fail-closed guard on the internal seam:
        // no future construction may dispatch a bound onto a version Research
        // 226 never probed.
        if self.maximum_turns.is_some()
            && !crate::claude_code_selection::plan_admits_maximum_turns(&plan)
        {
            return Err(failure(
                "swallowtail.claude_code.headless.maximum_turns_unqualified",
                "Claude Code headless maximum turns requires an exactly probed Claude Code version",
            ));
        }
        if self.watchers && !crate::claude_code_selection::plan_admits_watchers(&plan) {
            return Err(failure(
                "swallowtail.claude_code.headless.watchers_unqualified",
                "Claude Code headless watchers require exact Claude Code 2.1.251",
            ));
        }
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
        let run_id = RuntimeRunId::new(format!(
            "claude-code-headless:{}",
            request.request_id().as_str()
        ))
        .map_err(|_| {
            failure(
                "swallowtail.claude_code.headless.run_id_invalid",
                "Claude Code headless runtime run identity was invalid",
            )
        })?;
        let scope = ScopeId::new(format!(
            "claude-code-headless:{}",
            request.request_id().as_str()
        ))
        .map_err(|_| {
            failure(
                "swallowtail.claude_code.headless.scope_invalid",
                "Claude Code headless operation scope was invalid",
            )
        })?;
        let (event_sender, event_stream) = runtime_event_channel(EVENT_CAPACITY)?;
        let watcher_turn = if self.watchers {
            Some(watcher_turn(&request)?)
        } else {
            None
        };
        let watcher_feed = if let Some(turn) = watcher_turn.as_ref() {
            let watcher = services.watcher().cloned().ok_or_else(|| {
                failure(
                    "swallowtail.claude_code.headless.host_service_missing",
                    "Claude Code headless requires the preflight-bound watcher service",
                )
            })?;
            Some(watcher.open_lifecycle_feed(turn.clone()).await?)
        } else {
            None
        };
        let watcher_binding = if let Some(turn) = watcher_turn.as_ref() {
            Some(
                crate::claude_code_watcher::open_binding(&services, scope.clone(), turn.clone())
                    .await?,
            )
        } else {
            None
        };
        let executable = ExecutableRef::from_instance_target(plan.instance_target_ref());
        let process_request = ProcessRequest::new(executable)
            .with_arguments(arguments(
                &model,
                request.policy().reasoning_mode(),
                self.maximum_turns,
                watcher_binding
                    .as_ref()
                    .map(crate::claude_code_watcher::WatcherBinding::files),
            ))
            .with_environment([self.environment.clone()])
            .with_working_resource(working_resource);
        let process: Arc<dyn ProcessHandle> =
            match process_service.start(scope.clone(), process_request).await {
                Ok(process) => Arc::from(process),
                Err(error) => {
                    if let Some(binding) = watcher_binding {
                        let _ = binding.close(swallowtail_core::WatcherCleanupCause::Failed);
                    }
                    return Err(error);
                }
            };
        if let Err(error) = write_prompt(process.as_ref(), &request).await {
            cleanup_failed_start(process.as_ref()).await;
            if let Some(binding) = watcher_binding {
                let _ = binding.close(swallowtail_core::WatcherCleanupCause::Failed);
            }
            return Err(error);
        }
        let deadline = time_service.wait_until(deadline);
        if let Err(error) = event_sender.send(RuntimeEvent::new(0, RuntimeEventKind::Started)) {
            cleanup_failed_start(process.as_ref()).await;
            if let Some(binding) = watcher_binding {
                let _ = binding.close(swallowtail_core::WatcherCleanupCause::Failed);
            }
            return Err(error);
        }
        let (terminal_sender, terminal_future) = terminal_outcome_channel();
        let cancellation = Arc::new(ClaudeCodeCancellation::new(Arc::clone(&process)));
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
                        deadline,
                        model,
                        operation_id,
                        PumpHost {
                            services,
                            watcher_binding,
                            watcher_feed,
                            watcher_turn,
                        },
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
        Ok(Box::new(ClaudeCodeRunHandle::new(
            request.request_id().clone(),
            run_id,
            Box::pin(event_stream),
            Box::pin(terminal_future),
            cancellation,
            task,
        )))
    }
}

fn watcher_turn(request: &StructuredRunRequest) -> Result<RuntimeTurnId, RuntimeFailure> {
    RuntimeTurnId::new(format!(
        "claude-code-headless:{}",
        request.request_id().as_str()
    ))
    .map_err(|_| {
        failure(
            "swallowtail.claude_code.headless.turn_invalid",
            "Claude Code headless watcher turn identity was invalid",
        )
    })
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
            "swallowtail.claude_code.headless.stdin_close_failed",
            "Claude Code headless process stdin could not be closed",
        )
    })
}

pub(crate) fn validate_headless_plan(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
    if plan.driver_identity().id().as_str() != DRIVER_ID {
        return Err(failure(
            "swallowtail.claude_code.headless.plan_driver_mismatch",
            "Claude Code headless plan is bound to a different driver",
        ));
    }
    if plan.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
        || plan.credential_reference().is_some()
        || plan.endpoint_audience().as_str() != ENDPOINT_AUDIENCE
    {
        return Err(failure(
            "swallowtail.claude_code.headless.access_profile_rejected",
            "Claude Code headless requires its local subscription access profile",
        ));
    }
    if plan.harness_configuration_posture() != Some(HarnessConfigurationPosture::Ambient)
        || plan.requirements().harness_isolation() != Some(HarnessIsolation::AmbientHost)
    {
        return Err(failure(
            "swallowtail.claude_code.headless.ambient_authority_rejected",
            "Claude Code headless requires explicit ambient configuration and isolation authority",
        ));
    }
    crate::claude_code_selection::select_claude_code_headless_plan(plan)
}
