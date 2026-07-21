use futures_executor::block_on;
use serde_json::{Value, json};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::JoinHandle;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, AdapterId, Capability,
    CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, CredentialMechanism, CredentialRef, CredentialState, DriverRole,
    EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
    ExecutionHostId, ExecutionLayer, InstanceOwnership, InstancePolicyId, InstanceRevision,
    InstanceTargetRef, ModelId, ModelRoute, ModelRouteId, ModelRouteRevision,
    OperationRequirements, OperationShape, PreflightContext, PreflightPlan, ProtocolFacadeId,
    ResourceAccess, ResourceRepresentation, RuntimeReadiness, SessionAccessPolicy,
    SupportAuthority, preflight,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, HostServices, JoinedTask, ProcessExit, ProcessHandle,
    ProcessInputChunk, ProcessOutputChunk, ProcessOutputStream, ProcessRequest, ProcessService,
    ResourceLease, RuntimeFailure, ScopeId, ScopedTaskService, WorkingResourceIoService,
    WorkingResourceReadRequest, WorkingResourceRef, WorkingResourceService, WorkingResourceText,
};

include!("agent.rs");
#[derive(Clone)]
pub struct FixtureHost {
    agent: Arc<SharedAgent>,
    process: Arc<Mutex<Option<ObservedProcess>>>,
    reads: Arc<AtomicUsize>,
    releases: Arc<AtomicUsize>,
}

impl FixtureHost {
    pub fn new(scenario: Scenario) -> Self {
        Self {
            agent: Arc::new(SharedAgent {
                state: Mutex::new(AgentState::default()),
                changed: Condvar::new(),
                scenario,
            }),
            process: Arc::new(Mutex::new(None)),
            reads: Arc::new(AtomicUsize::new(0)),
            releases: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn services(&self, host: ExecutionHostId) -> HostServices {
        HostServices::new(host)
            .with_task(Arc::new(ThreadTaskService))
            .with_process(Arc::new(self.clone()))
            .with_working_resource(Arc::new(self.clone()))
            .with_working_resource_io(Arc::new(self.clone()))
    }

    pub fn observed_process(&self) -> ObservedProcess {
        self.process
            .lock()
            .expect("fixture process lock poisoned")
            .clone()
            .expect("process was observed")
    }

    pub fn reads(&self) -> usize {
        self.reads.load(Ordering::SeqCst)
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

impl WorkingResourceIoService for FixtureHost {
    fn read_text(
        &self,
        _lease: &ResourceLease,
        request: WorkingResourceReadRequest,
    ) -> BoxFuture<'static, Result<WorkingResourceText, RuntimeFailure>> {
        if request.locator().as_host_value() != "/private/fixture/src/lib.rs" {
            return Box::pin(async { Err(fixture_failure()) });
        }
        self.reads.fetch_add(1, Ordering::SeqCst);
        let content = WorkingResourceText::new("fixture file".to_owned(), request.maximum_bytes())
            .map_err(|_| fixture_failure());
        Box::pin(async move { content })
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
        "fixture.gemini_acp.failed",
        "Gemini ACP fixture failed",
    ))
}
