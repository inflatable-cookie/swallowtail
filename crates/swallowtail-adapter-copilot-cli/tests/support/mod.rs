use futures_executor::block_on;
use serde_json::{Value, json};
use std::{
    collections::VecDeque,
    sync::{
        Arc, Condvar, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    thread::JoinHandle,
};
use swallowtail_adapter_copilot_cli::{
    copilot_cli_acp_descriptor, copilot_cli_host_account_access_profile,
    copilot_cli_package_binding,
};
use swallowtail_core::{
    AccessProfileId, AccessRequirement, AccessStatus, AdapterId, Capability, CapabilityConstraint,
    CapabilityProfile, CapabilityRequirement, ConfiguredInstance, ConfiguredInstanceId,
    CredentialState, DriverRole, EndpointAuthorization, EntitlementState, ExecutionHostId,
    ExecutionLayer, HarnessConfigurationPosture, HarnessIsolation, InstanceOwnership,
    InstancePolicyId, InstanceRevision, InstanceTargetRef, OperationRequirements, OperationShape,
    PreflightContext, PreflightPlan, ProtocolFacadeId, ResourceAccess, ResourceRepresentation,
    RuntimeReadiness, SessionAccessPolicy, SessionProviderStatePolicy, SupportAuthority, preflight,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, Deadline, DeadlineObservation, HostServices, JoinedTask,
    MonotonicInstant, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk,
    ProcessOutputStream, ProcessRequest, ProcessService, ResourceLease, RuntimeFailure, ScopeId,
    ScopedTaskService, SessionCleanupRequest, TimeService, WorkingResourceRef,
    WorkingResourceService,
};

include!("agent.rs");

#[derive(Clone)]
pub struct FixtureHost {
    agent: Arc<SharedAgent>,
    process: Arc<Mutex<Option<ObservedProcess>>>,
    releases: Arc<AtomicUsize>,
}

impl FixtureHost {
    pub fn new(scenario: Scenario) -> Self {
        Self::with_version(scenario, "1.0.80")
    }

    pub fn with_version(scenario: Scenario, version: &str) -> Self {
        Self {
            agent: Arc::new(SharedAgent {
                state: Mutex::new(AgentState::default()),
                changed: Condvar::new(),
                scenario,
                version: version.to_owned(),
            }),
            process: Arc::new(Mutex::new(None)),
            releases: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn services(&self, host: ExecutionHostId) -> HostServices {
        HostServices::new(host)
            .with_task(Arc::new(ThreadTaskService))
            .with_time(Arc::new(FixtureTime))
            .with_process(Arc::new(self.clone()))
            .with_working_resource(Arc::new(self.clone()))
    }

    pub fn cleanup_request(&self) -> SessionCleanupRequest {
        SessionCleanupRequest::new(Deadline::at(MonotonicInstant::from_ticks(10_000)))
    }

    pub fn observed_process(&self) -> ObservedProcess {
        self.process
            .lock()
            .expect("fixture process lock poisoned")
            .clone()
            .expect("process was observed")
    }

    pub fn process_started(&self) -> bool {
        self.process
            .lock()
            .expect("fixture process lock poisoned")
            .is_some()
    }

    pub fn releases(&self) -> usize {
        self.releases.load(Ordering::SeqCst)
    }

    pub fn writes(&self) -> Vec<Value> {
        self.agent
            .state
            .lock()
            .expect("fixture agent lock poisoned")
            .writes
            .clone()
    }
}

struct FixtureTime;

impl TimeService for FixtureTime {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(0)
    }

    fn wait_until(&self, _deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(std::future::pending())
    }
}

impl ProcessService for FixtureHost {
    fn start(
        &self,
        _scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        *self.process.lock().expect("fixture process lock poisoned") = Some(ObservedProcess {
            arguments: request.arguments().map(str::to_owned).collect(),
            environment_count: request.environment().len(),
            working_resource: request.working_resource().cloned(),
        });
        let handle =
            Box::new(FixtureProcessHandle(Arc::clone(&self.agent))) as Box<dyn ProcessHandle>;
        Box::pin(async move { Ok(handle) })
    }
}

impl WorkingResourceService for FixtureHost {
    fn resolve(
        &self,
        scope: ScopeId,
        reference: WorkingResourceRef,
        access: ResourceAccess,
        representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>> {
        let lease = ResourceLease::consumer_owned(scope, reference, access, representation)
            .with_filesystem(
                swallowtail_runtime::MaterializedResourceRef::new("/private/fixture")
                    .expect("fixture path is valid"),
            );
        Box::pin(async move { Ok(lease) })
    }

    fn create_temporary(
        &self,
        _scope: ScopeId,
        _access: ResourceAccess,
        _representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>> {
        Box::pin(async { Err(fixture_failure()) })
    }

    fn release(&self, _lease: ResourceLease) -> BoxFuture<'static, CleanupOutcome> {
        self.releases.fetch_add(1, Ordering::SeqCst);
        Box::pin(async { CleanupOutcome::NotApplicable })
    }
}

struct ThreadTaskService;

struct ThreadTask(Option<JoinHandle<()>>);

impl ScopedTaskService for ThreadTaskService {
    fn spawn(
        &self,
        _scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        Ok(Box::new(ThreadTask(Some(std::thread::spawn(move || {
            block_on(task);
        })))))
    }
}

impl JoinedTask for ThreadTask {
    fn join(mut self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let handle = self.0.take().expect("fixture task joins once");
        Box::pin(async move { handle.join().map_err(|_| fixture_failure()) })
    }
}

include!("selection.rs");

fn fixture_failure() -> RuntimeFailure {
    RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
        "fixture.copilot_cli_acp.failed",
        "Copilot CLI ACP fixture failed",
    ))
}
