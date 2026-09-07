//! Shared harness for the registered-tool conformance oracle.

use crate::registered_tool_fixture::{
    FakeClock, ScriptedAdmissionPort, fixture_admission, fixture_selection, fixture_snapshot,
};
use std::sync::Arc;
use std::time::Duration;
use swallowtail_core::{ConfiguredInstanceId, ExecutionHostId};
use swallowtail_runtime::{
    Deadline, HostServices, MonotonicInstant, RegisteredToolBridgeLease, RegisteredToolDispatcher,
    RegisteredToolLimits, RegisteredToolPreparation, RegisteredToolSelection,
    RegisteredToolSnapshot, RuntimeFailure, RuntimeTurnId, ScopeId,
};

/// Exact composition inputs one conformance case asks a host to mount.
pub struct RegisteredToolHostSpec {
    /// Linked dispatcher the mounted port must dispatch through.
    pub dispatcher: Arc<dyn RegisteredToolDispatcher>,
    /// Bounded joined-cleanup budget for registered-tool leases.
    pub cleanup_budget: Duration,
    /// Virtual clock the mounted port enforces call deadlines against.
    pub clock: Arc<FakeClock>,
}

impl RegisteredToolHostSpec {
    /// Builds a spec with a fresh virtual clock at tick zero.
    #[must_use]
    pub fn new(dispatcher: Arc<dyn RegisteredToolDispatcher>, cleanup_budget: Duration) -> Self {
        Self {
            dispatcher,
            cleanup_budget,
            clock: Arc::new(FakeClock::default()),
        }
    }
}

/// Composes one host registry that mounts a linked registered-tool dispatcher.
pub type ComposeRegisteredToolHost = dyn Fn(RegisteredToolHostSpec) -> HostServices;

/// Execution host every registered-tool conformance case uses.
#[must_use]
pub fn conformance_host_id() -> ExecutionHostId {
    ExecutionHostId::new("fixture.host.local").expect("host id is valid")
}

/// Configured instance every registered-tool conformance case uses.
#[must_use]
pub fn conformance_instance() -> ConfiguredInstanceId {
    ConfiguredInstanceId::new("fixture.instance.registered-tool").expect("instance id is valid")
}

/// Operation scope every registered-tool conformance case uses.
#[must_use]
pub fn conformance_scope() -> ScopeId {
    ScopeId::new("fixture.registered-tool.scope").expect("scope id is valid")
}

/// Returns one bounded conformance turn attempt.
#[must_use]
pub fn conformance_turn(label: &str) -> RuntimeTurnId {
    RuntimeTurnId::new(label).expect("turn id is valid")
}

/// Returns the fixed conformance deadline.
#[must_use]
pub fn conformance_deadline() -> Deadline {
    Deadline::at(MonotonicInstant::from_ticks(1_000))
}

/// One prepared conformance harness bound to a scripted admission port.
pub struct RegisteredToolHarness {
    /// Host registry under test.
    pub hosts: HostServices,
    /// Scripted consumer admission port bound into the preparation.
    pub admission: Arc<ScriptedAdmissionPort>,
    /// Immutable registration snapshot.
    pub snapshot: Arc<RegisteredToolSnapshot>,
    /// Immutable selection over that snapshot.
    pub selection: RegisteredToolSelection,
    /// Immutable opt-in preparation.
    pub preparation: RegisteredToolPreparation,
    /// Virtual clock the mounted port enforces call deadlines against.
    pub clock: Arc<FakeClock>,
}

impl RegisteredToolHarness {
    /// Mounts one host registry for an exact dispatcher and cleanup budget.
    #[must_use]
    pub fn mount(
        compose: &ComposeRegisteredToolHost,
        dispatcher: Arc<dyn RegisteredToolDispatcher>,
        cleanup_budget: Duration,
    ) -> Self {
        let spec = RegisteredToolHostSpec::new(dispatcher, cleanup_budget);
        let clock = Arc::clone(&spec.clock);
        Self::with_clock(compose(spec), clock)
    }

    /// Builds one harness over a composed host registry and its virtual clock.
    #[must_use]
    pub fn with_clock(hosts: HostServices, clock: Arc<FakeClock>) -> Self {
        let admission = Arc::new(ScriptedAdmissionPort::current());
        let snapshot = Arc::new(fixture_snapshot(&conformance_host_id()));
        let selection = fixture_selection(Arc::clone(&snapshot));
        let preparation = RegisteredToolPreparation::new(
            Arc::clone(&snapshot),
            selection.clone(),
            fixture_admission(Arc::clone(&admission)),
            RegisteredToolLimits::ceiling(),
        );
        Self {
            hosts,
            admission,
            snapshot,
            selection,
            preparation,
            clock,
        }
    }

    /// Attempts to prepare and open one lease on an exact turn attempt.
    pub fn try_open(&self, turn: &str) -> Result<RegisteredToolBridgeLease, RuntimeFailure> {
        let prepared = self.preparation.prepare(
            &self.hosts,
            conformance_instance(),
            conformance_scope(),
            conformance_turn(turn),
            conformance_deadline(),
        )?;
        crate::registered_tool_fixture::drive_fixture(prepared.open())
    }

    /// Prepares and opens one lease on an exact turn attempt.
    #[must_use]
    pub fn open(&self, turn: &str) -> RegisteredToolBridgeLease {
        let prepared = self
            .preparation
            .prepare(
                &self.hosts,
                conformance_instance(),
                conformance_scope(),
                conformance_turn(turn),
                conformance_deadline(),
            )
            .expect("prepare succeeds on a ready registry");
        crate::registered_tool_fixture::drive_fixture(prepared.open())
            .expect("open succeeds on a prepared binding")
    }
}
