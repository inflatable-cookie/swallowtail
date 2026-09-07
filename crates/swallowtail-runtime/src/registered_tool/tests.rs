//! Provider-free conformance for the registered-tool vocabulary itself.

use super::*;
use crate::{Deadline, HostServices, MonotonicInstant, RuntimeTurnId, ScopeId, TimeService};
use std::future::ready;
use std::sync::Arc;
use std::time::Duration;
use swallowtail_core::{ConfiguredInstanceId, ExecutionHostId, HostServiceKind};

/// Admission port that holds one exact validation pending until released.
///
/// `hold_from(n)` suspends the nth live verdict and every later one, so a test
/// can stop the kernel precisely inside its terminal verdict.
#[derive(Default)]
struct HoldableAdmission {
    hold_from: std::sync::Mutex<Option<usize>>,
    waiters: std::sync::Mutex<Vec<std::task::Waker>>,
    requests: std::sync::Mutex<usize>,
}

impl HoldableAdmission {
    fn hold_from(&self, index: usize) {
        *self.hold_from.lock().expect("hold lock") = Some(index);
    }

    fn release(&self) {
        *self.hold_from.lock().expect("hold lock") = None;
        for waker in std::mem::take(&mut *self.waiters.lock().expect("waiter lock")) {
            waker.wake();
        }
    }

    fn requests(&self) -> usize {
        *self.requests.lock().expect("request lock")
    }
}

struct HeldVerdict<'port> {
    port: &'port HoldableAdmission,
    index: Option<usize>,
}

impl Future for HeldVerdict<'_> {
    type Output = Result<AdmissionVerdict, crate::RuntimeFailure>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        context: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let index = match self.index {
            Some(index) => index,
            None => {
                let mut requests = self.port.requests.lock().expect("request lock");
                *requests += 1;
                let index = *requests;
                drop(requests);
                self.index = Some(index);
                index
            }
        };
        let held = matches!(
            *self.port.hold_from.lock().expect("hold lock"),
            Some(from) if index >= from
        );
        if held {
            let mut waiters = self.port.waiters.lock().expect("waiter lock");
            let waker = context.waker();
            if !waiters.iter().any(|waiting| waiting.will_wake(waker)) {
                waiters.push(waker.clone());
            }
            return std::task::Poll::Pending;
        }
        std::task::Poll::Ready(Ok(AdmissionVerdict::Current))
    }
}

impl ConsumerAdmissionHostService for HoldableAdmission {
    fn validate(
        &self,
        _binding: &ConsumerAdmissionBinding,
        _phase: AdmissionPhase,
    ) -> crate::BoxFuture<'_, Result<AdmissionVerdict, crate::RuntimeFailure>> {
        Box::pin(HeldVerdict {
            port: self,
            index: None,
        })
    }
}

fn holdable_admission(port: Arc<HoldableAdmission>) -> ConsumerAdmissionBinding {
    ConsumerAdmissionBinding::new(
        ConsumerProcessIncarnation::new("incarnation").expect("incarnation"),
        ConsumerWorkspaceGeneration::initial(),
        ConsumerTaskGeneration::initial(),
        AdmittedTaskId::new("task").expect("task"),
        AdmittedSessionId::new("session").expect("session"),
        AdmittedAttemptId::new("attempt").expect("attempt"),
        port,
    )
}

/// Opens one kernel whose live verdicts a test controls exactly.
fn held_kernel(
    port: &Arc<HoldableAdmission>,
    clock: &Arc<FixedClock>,
    deadline: Deadline,
) -> (
    Arc<RegisteredToolOperationKernel>,
    RegisteredToolBridgeLease,
) {
    let hosts = ready_hosts();
    let proof = RegisteredToolReadiness::evaluate(&hosts, &selection())
        .require_ready()
        .expect("proof");
    let request = RegisteredToolOpenRequest::new(
        host(),
        ConfiguredInstanceId::new("instance").expect("instance"),
        ScopeId::new("scope").expect("scope"),
        RuntimeTurnId::new("turn").expect("turn"),
        selection(),
        holdable_admission(Arc::clone(port)),
        deadline,
    );
    RegisteredToolOperationKernel::open(
        request,
        &proof,
        Arc::new(EchoDispatcher),
        Arc::clone(clock) as Arc<dyn TimeService>,
        RegisteredToolLeaseGeneration::initial(),
        RegisteredToolTransportGeneration::initial(),
    )
    .expect("open")
}

/// Suspends one call precisely inside its terminal verdict.
///
/// The dispatch verdict completes, the dispatcher settles, and the second live
/// verdict — the terminal `BeforeDelivery` check — is left in flight.
fn suspend_at_terminal_verdict<'call>(
    call: &mut crate::BoxFuture<'call, Result<RegisteredToolOutcome, crate::RuntimeFailure>>,
    port: &HoldableAdmission,
) {
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    assert!(
        call.as_mut().poll(&mut context).is_pending(),
        "the call is suspended inside its terminal verdict, not settled"
    );
    assert_eq!(
        port.requests(),
        2,
        "the dispatch verdict completed and the terminal verdict is pending"
    );
}

#[test]
fn a_freeze_during_the_terminal_verdict_rejects_a_stale_success() {
    let port = Arc::new(HoldableAdmission::default());
    let clock = Arc::new(FixedClock::default());
    let (kernel, lease) = held_kernel(
        &port,
        &clock,
        Deadline::at(MonotonicInstant::from_ticks(10_000)),
    );
    // Let the dispatch verdict through and hold the terminal one.
    port.hold_from(2);
    let mut call = lease.call(call_request("frozen-terminal"));
    suspend_at_terminal_verdict(&mut call, &port);

    // A concurrent terminal freeze lands while that verdict is pending.
    kernel.freeze();
    port.release();
    let outcome = block(call).expect("the kernel settles the call");

    assert!(
        outcome.result().is_none(),
        "a stale success never commits after a terminal freeze"
    );
    assert_eq!(
        outcome.failure().map(RegisteredToolFailure::kind),
        Some(RegisteredToolFailureKind::PostTerminalCorrelation)
    );
    assert_eq!(
        outcome.disposition(),
        RegisteredToolExecutionDisposition::Executed,
        "the dispatcher produced a result, so the disposition stays honest"
    );
    assert!(!outcome.permits_explicit_retry());
}

#[test]
fn a_deadline_elapsing_during_the_terminal_verdict_rejects_a_stale_success() {
    let port = Arc::new(HoldableAdmission::default());
    let clock = Arc::new(FixedClock::default());
    let (_kernel, lease) = held_kernel(
        &port,
        &clock,
        Deadline::at(MonotonicInstant::from_ticks(10_000)),
    );
    port.hold_from(2);
    let mut call = lease.call(call_request("expired-terminal"));
    suspend_at_terminal_verdict(&mut call, &port);

    // The effective deadline elapses while the terminal verdict is pending.
    clock.set(10_001);
    port.release();
    let outcome = block(call).expect("the kernel settles the call");

    assert!(
        outcome.result().is_none(),
        "a stale success never commits after the deadline elapses"
    );
    assert_eq!(
        outcome.failure().map(RegisteredToolFailure::kind),
        Some(RegisteredToolFailureKind::DeadlineExceeded)
    );
    assert_eq!(
        outcome.disposition(),
        RegisteredToolExecutionDisposition::Unknown,
        "an expired terminal never claims the effect did or did not land"
    );
    assert!(!outcome.permits_explicit_retry());
}

struct AlwaysCurrent;

impl ConsumerAdmissionHostService for AlwaysCurrent {
    fn validate(
        &self,
        _binding: &ConsumerAdmissionBinding,
        _phase: AdmissionPhase,
    ) -> crate::BoxFuture<'_, Result<AdmissionVerdict, crate::RuntimeFailure>> {
        Box::pin(ready(Ok(AdmissionVerdict::Current)))
    }
}

fn host() -> ExecutionHostId {
    ExecutionHostId::new("fixture.host.local").expect("host id")
}

fn tool(local: &str) -> RegisteredToolId {
    RegisteredToolId::new(
        RegisteredToolNamespace::new("swallowtail.conformance").expect("namespace"),
        RegisteredToolLocalName::new(local).expect("local name"),
    )
}

fn schema() -> RegisteredToolSchema {
    RegisteredToolSchema::new(
        RegisteredToolSchemaNamespace::new("swallowtail.conformance.schema").expect("namespace"),
        RegisteredToolSchemaMediaType::new("application/json").expect("media type"),
        RegisteredToolSchemaDialect::new("json-schema-2020-12").expect("dialect"),
        RegisteredServerRevision::new("1").expect("revision"),
        RegisteredToolSchemaDigest::new("sha256:body").expect("digest"),
        RegisteredToolSchemaDocument::new("{\"secret\":\"body\"}").expect("document"),
    )
}

fn snapshot() -> RegisteredToolSnapshot {
    RegisteredToolSnapshot::new(snapshot_input()).expect("snapshot")
}

fn snapshot_input() -> RegisteredToolSnapshotInput {
    RegisteredToolSnapshotInput {
        server_id: RegisteredServerId::new("server").expect("server id"),
        revision: RegisteredServerRevision::new("1").expect("revision"),
        execution_host_id: host(),
        declarations: vec![
            RegisteredToolDeclaration::new(
                tool("echo"),
                RegisteredToolExecutionKind::NativeClient,
                schema(),
                schema(),
                RegisteredToolEffectPosture::ReadOnly,
                RegisteredToolRetryPosture::NeverRetry,
                RegisteredToolBounds::ceiling(),
            )
            .expect("declaration"),
        ],
        transports: vec![
            RegisteredToolTransportSupport::new(
                RegisteredToolTransport::HostMediatedCallback,
                [protocol_version()],
            )
            .expect("transport support"),
        ],
        required_services: [HostServiceKind::Time].into_iter().collect(),
        credential_references: Vec::new(),
        executable_recipes: Vec::new(),
        environment_recipes: Vec::new(),
        bounds: RegisteredToolBounds::ceiling(),
        source: RegisteredToolSource::new(
            RegisteredToolSourceId::new("source").expect("source id"),
            MonotonicInstant::from_ticks(1),
        ),
    }
}

fn protocol_version() -> RegisteredToolProtocolVersion {
    RegisteredToolProtocolVersion::new(REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION)
        .expect("protocol version")
}

fn admission() -> ConsumerAdmissionBinding {
    ConsumerAdmissionBinding::new(
        ConsumerProcessIncarnation::new("incarnation").expect("incarnation"),
        ConsumerWorkspaceGeneration::initial(),
        ConsumerTaskGeneration::initial(),
        AdmittedTaskId::new("task").expect("task"),
        AdmittedSessionId::new("session").expect("session"),
        AdmittedAttemptId::new("attempt").expect("attempt"),
        Arc::new(AlwaysCurrent),
    )
}

/// A selection over a different snapshot revision of the same server.
fn other_selection() -> RegisteredToolSelection {
    let mut input = snapshot_input();
    input.revision = RegisteredServerRevision::new("2").expect("revision");
    RegisteredToolSelection::new(
        Arc::new(RegisteredToolSnapshot::new(input).expect("snapshot")),
        [tool("echo")],
        RegisteredToolTransport::HostMediatedCallback,
        protocol_version(),
    )
    .expect("selection")
}

fn ready_hosts() -> HostServices {
    struct AbsentPort;
    impl RegisteredToolBridgeHostService for AbsentPort {
        fn open(
            &self,
            _request: RegisteredToolOpenRequest,
        ) -> crate::BoxFuture<'_, Result<RegisteredToolBridgeLease, crate::RuntimeFailure>>
        {
            Box::pin(ready(Err(RegisteredToolFailure::new(
                RegisteredToolFailureKind::NotReady,
            )
            .into_runtime_failure())))
        }

        fn completion_gate(
            &self,
            _lease: &RegisteredToolBridgeLease,
        ) -> crate::BoxFuture<'_, Result<RegisteredToolCompletionState, crate::RuntimeFailure>>
        {
            Box::pin(ready(Err(RegisteredToolFailure::new(
                RegisteredToolFailureKind::NotReady,
            )
            .into_runtime_failure())))
        }

        fn close(
            &self,
            _lease: RegisteredToolBridgeLease,
            _cause: RegisteredToolCleanupCause,
        ) -> crate::BoxFuture<'_, Result<crate::CleanupOutcome, crate::RuntimeFailure>> {
            Box::pin(ready(Ok(crate::CleanupOutcome::NotApplicable)))
        }
    }

    HostServices::new(host())
        .with_time(Arc::new(FixedClock::default()))
        .with_registered_tool_bridge(Arc::new(AbsentPort))
}

fn open_request(
    selection: RegisteredToolSelection,
    deadline: Deadline,
) -> RegisteredToolOpenRequest {
    RegisteredToolOpenRequest::new(
        host(),
        ConfiguredInstanceId::new("instance").expect("instance"),
        ScopeId::new("scope").expect("scope"),
        RuntimeTurnId::new("turn").expect("turn"),
        selection,
        admission(),
        deadline,
    )
}

fn call_request(label: &str) -> crate::RegisteredToolCallRequest {
    crate::RegisteredToolCallRequest::new(
        crate::RegisteredToolCallId::new(label).expect("call id"),
        tool("echo"),
        RegisteredToolPayload::new(
            RegisteredToolSchemaMediaType::new("application/json").expect("media type"),
            b"{}".to_vec(),
            64,
        )
        .expect("payload"),
        Deadline::at(MonotonicInstant::from_ticks(u64::MAX)),
    )
}

#[derive(Default)]
struct FixedClock {
    ticks: std::sync::Mutex<u64>,
}

impl FixedClock {
    fn set(&self, ticks: u64) {
        *self.ticks.lock().expect("clock lock") = ticks;
    }
}

impl TimeService for FixedClock {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(*self.ticks.lock().expect("clock lock"))
    }

    fn wait_until(
        &self,
        deadline: Deadline,
    ) -> crate::BoxFuture<'static, crate::DeadlineObservation> {
        let observed = self.now();
        Box::pin(ready(crate::DeadlineObservation::new(deadline, observed)))
    }
}

struct EchoDispatcher;

impl RegisteredToolDispatcher for EchoDispatcher {
    fn dispatch(
        &self,
        call: RegisteredToolCall,
        _context: RegisteredToolDispatchContext,
    ) -> crate::BoxFuture<'_, Result<RegisteredToolOutcome, crate::RuntimeFailure>> {
        let outcome = RegisteredToolOutcome::completed(
            &call,
            RegisteredToolResult::new(
                RegisteredToolPayload::new(
                    RegisteredToolSchemaMediaType::new("application/json").expect("media type"),
                    b"{}".to_vec(),
                    64,
                )
                .expect("payload"),
                RegisteredToolSchemaDigest::new("sha256:body").expect("digest"),
            ),
        );
        Box::pin(ready(Ok(outcome)))
    }
}

fn block<T>(future: crate::BoxFuture<'_, T>) -> T {
    let mut future = future;
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    for _ in 0..1024 {
        if let std::task::Poll::Ready(value) = future.as_mut().poll(&mut context) {
            return value;
        }
    }
    panic!("fixture future never settled");
}

fn selection() -> RegisteredToolSelection {
    RegisteredToolSelection::new(
        Arc::new(snapshot()),
        [tool("echo")],
        RegisteredToolTransport::HostMediatedCallback,
        protocol_version(),
    )
    .expect("selection")
}

#[test]
fn host_service_kind_stays_exhaustive_for_the_registered_profile() {
    let hosts = HostServices::new(host());

    assert!(hosts.registered_tool_bridge().is_none());
    assert!(!hosts.available_kinds().iter().any(|kind| {
        format!("{kind:?}")
            .to_ascii_lowercase()
            .contains("registeredtool")
    }));
}

#[test]
fn missing_required_services_are_reported_before_provider_work() {
    let hosts = HostServices::new(host());
    let readiness = RegisteredToolReadiness::evaluate(&hosts, &selection());

    assert!(
        readiness
            .missing_services()
            .contains(&HostServiceKind::Time)
    );
    assert_eq!(readiness.port(), RegisteredToolPortAvailability::Absent);
    assert_eq!(
        readiness
            .require_ready()
            .expect_err("an absent port fails first")
            .kind(),
        RegisteredToolFailureKind::MissingHostService
    );
}

#[test]
fn consumer_limits_only_narrow_effective_bounds() {
    let narrow = RegisteredToolBounds::new(1, 1_024, 2_048, 512, 4, Duration::from_secs(5))
        .expect("narrow bounds");
    let effective = selection().effective_bounds().narrowed(narrow);

    assert_eq!(effective.max_argument_bytes(), 1_024);
    assert_eq!(effective.max_queued_progress_items(), 4);
    assert_eq!(effective.max_call_duration(), Duration::from_secs(5));
    assert_eq!(
        effective.max_outstanding_calls(),
        MAX_REGISTERED_TOOL_OUTSTANDING_CALLS
    );
}

#[test]
fn consumer_limits_reach_the_kernel_that_enforces_them() {
    let narrowed = RegisteredToolLimits::new(
        RegisteredToolBounds::new(1, 16, 16, 16, 1, Duration::from_secs(1)).expect("narrow bounds"),
    );
    let request = open_request(
        selection(),
        Deadline::at(MonotonicInstant::from_ticks(10_000)),
    )
    .with_consumer_limits(narrowed);

    let hosts = ready_hosts();
    let proof = RegisteredToolReadiness::evaluate(&hosts, &selection())
        .require_ready()
        .expect("proof");
    let (kernel, _lease) = RegisteredToolOperationKernel::open(
        request,
        &proof,
        Arc::new(EchoDispatcher),
        Arc::new(FixedClock::default()),
        RegisteredToolLeaseGeneration::initial(),
        RegisteredToolTransportGeneration::initial(),
    )
    .expect("open");

    assert_eq!(
        kernel
            .binding()
            .effective_bounds()
            .max_queued_progress_items(),
        1
    );
    assert_eq!(kernel.binding().effective_bounds().max_argument_bytes(), 16);
    assert_eq!(
        kernel.binding().effective_bounds().max_call_duration(),
        Duration::from_secs(1),
        "the kernel enforces the narrowed bounds, not the snapshot ceiling"
    );
}

#[test]
fn bounds_never_widen_the_first_tranche_ceiling() {
    let error = RegisteredToolBounds::new(
        1,
        MAX_REGISTERED_TOOL_ARGUMENT_BYTES + 1,
        1,
        1,
        1,
        Duration::from_secs(1),
    )
    .expect_err("widening must fail");
    let zero = RegisteredToolBounds::new(0, 1, 1, 1, 1, Duration::from_secs(1))
        .expect_err("zero bounds must fail");

    assert_eq!(error.kind(), RegisteredToolFailureKind::LimitExceeded);
    assert_eq!(zero.kind(), RegisteredToolFailureKind::IdentityRejected);
}

#[test]
fn schema_and_payload_bodies_never_reach_formatting() {
    let schema = schema();
    let payload = RegisteredToolPayload::new(
        RegisteredToolSchemaMediaType::new("application/json").expect("media type"),
        b"private-argument".to_vec(),
        64,
    )
    .expect("payload");

    assert!(!format!("{schema:?}").contains("secret"));
    assert!(!format!("{payload:?}").contains("private-argument"));
    assert!(!format!("{payload}").contains("private-argument"));
    assert_eq!(payload.expose_for_execution(), b"private-argument");
    assert_eq!(
        schema.document().expose_for_execution(),
        "{\"secret\":\"body\"}"
    );
}

#[test]
fn admission_binding_compares_fixed_identity_and_redacts_its_port() {
    let binding = admission();
    let rendered = format!("{binding:?}");

    assert!(binding.identity_matches(&admission()));
    assert!(rendered.contains("<opaque>"));
    assert!(rendered.contains("<private consumer admission port>"));
    assert!(!rendered.contains("incarnation\": \"incarnation"));
}

#[test]
fn a_topology_proof_is_required_and_is_not_transferable() {
    let hosts = ready_hosts();
    let proof = RegisteredToolReadiness::evaluate(&hosts, &selection())
        .require_ready()
        .expect("the ready registry issues a proof");
    let other = other_selection();

    assert!(proof.matches(&selection()));
    assert!(
        !proof.matches(&other),
        "a proof issued for one selection never opens another"
    );

    let error = RegisteredToolOperationKernel::open(
        open_request(
            other.clone(),
            Deadline::at(MonotonicInstant::from_ticks(10_000)),
        ),
        &proof,
        Arc::new(EchoDispatcher),
        Arc::new(FixedClock::default()),
        RegisteredToolLeaseGeneration::initial(),
        RegisteredToolTransportGeneration::initial(),
    )
    .expect_err("a mismatched proof mints nothing");

    assert_eq!(
        error.diagnostic().code(),
        RegisteredToolFailureKind::UnsupportedRegistration.code()
    );
}

#[test]
fn the_kernel_is_the_only_lease_and_binding_source() {
    let hosts = ready_hosts();
    let proof = RegisteredToolReadiness::evaluate(&hosts, &selection())
        .require_ready()
        .expect("proof");
    let clock = Arc::new(FixedClock::default());

    let (kernel, lease) = RegisteredToolOperationKernel::open(
        open_request(
            selection(),
            Deadline::at(MonotonicInstant::from_ticks(10_000)),
        ),
        &proof,
        Arc::new(EchoDispatcher),
        clock,
        RegisteredToolLeaseGeneration::initial(),
        RegisteredToolTransportGeneration::initial(),
    )
    .expect("the kernel opens its own lease");

    assert!(
        lease.is_bound_to(&kernel),
        "a lease authenticates only against the kernel that minted it"
    );
    assert_eq!(kernel.binding().lease_generation().get(), 1);
    assert!(format!("{lease:?}").contains("<private operation kernel>"));
}

#[test]
fn an_expired_call_never_reaches_the_dispatcher() {
    let hosts = ready_hosts();
    let proof = RegisteredToolReadiness::evaluate(&hosts, &selection())
        .require_ready()
        .expect("proof");
    let clock = Arc::new(FixedClock::default());
    let dispatcher = Arc::new(EchoDispatcher);
    let (kernel, lease) = RegisteredToolOperationKernel::open(
        open_request(selection(), Deadline::at(MonotonicInstant::from_ticks(100))),
        &proof,
        dispatcher,
        Arc::clone(&clock) as Arc<dyn TimeService>,
        RegisteredToolLeaseGeneration::initial(),
        RegisteredToolTransportGeneration::initial(),
    )
    .expect("open");
    clock.set(100);

    let error = block(lease.call(call_request("expired")))
        .expect_err("an expired call fails before dispatch");

    assert_eq!(
        error.diagnostic().code(),
        RegisteredToolFailureKind::DeadlineExceeded.code()
    );
    assert_eq!(kernel.outstanding_calls(), 0);
}

#[test]
fn the_effective_call_deadline_takes_the_earliest_bound() {
    let hosts = ready_hosts();
    let proof = RegisteredToolReadiness::evaluate(&hosts, &selection())
        .require_ready()
        .expect("proof");
    let (kernel, _lease) = RegisteredToolOperationKernel::open(
        open_request(selection(), Deadline::at(MonotonicInstant::from_ticks(500))),
        &proof,
        Arc::new(EchoDispatcher),
        Arc::new(FixedClock::default()),
        RegisteredToolLeaseGeneration::initial(),
        RegisteredToolTransportGeneration::initial(),
    )
    .expect("open");

    let operation_bound = kernel.effective_call_deadline(
        Deadline::at(MonotonicInstant::from_ticks(900)),
        MonotonicInstant::from_ticks(0),
    );
    let caller_bound = kernel.effective_call_deadline(
        Deadline::at(MonotonicInstant::from_ticks(50)),
        MonotonicInstant::from_ticks(0),
    );
    let (unbounded, _unbounded_lease) = RegisteredToolOperationKernel::open(
        open_request(
            selection(),
            Deadline::at(MonotonicInstant::from_ticks(u64::MAX)),
        ),
        &proof,
        Arc::new(EchoDispatcher),
        Arc::new(FixedClock::default()),
        RegisteredToolLeaseGeneration::initial(),
        RegisteredToolTransportGeneration::initial(),
    )
    .expect("open");
    let duration_bound = unbounded.effective_call_deadline(
        Deadline::at(MonotonicInstant::from_ticks(u64::MAX)),
        MonotonicInstant::from_ticks(0),
    );

    assert_eq!(operation_bound.instant().ticks(), 500);
    assert_eq!(caller_bound.instant().ticks(), 50);
    assert_eq!(
        duration_bound.instant().ticks(),
        u64::try_from(REGISTERED_TOOL_MAX_CALL_DURATION.as_nanos()).expect("bounded"),
        "the maximum call duration bounds a caller deadline that never arrives"
    );
}

#[test]
fn swallowtail_enforced_posture_never_widens() {
    assert!(!RegisteredToolEnforcedPosture::automatic_replay_allowed());
    assert!(!RegisteredToolEnforcedPosture::reconnect_replays_calls());
    assert!(RegisteredToolEnforcedPosture::teardown_joins_issued_work());
}

#[test]
fn only_the_host_mediated_callback_transport_is_qualified() {
    assert_eq!(
        REGISTERED_TOOL_QUALIFIED_TRANSPORTS,
        &[RegisteredToolTransport::HostMediatedCallback]
    );
    assert!(!RegisteredToolTransport::HostMediatedCallback.binds_listener());
    assert!(RegisteredToolTransport::PrivateLoopbackHttp.binds_listener());
    assert!(RegisteredToolTransport::PrivateLoopbackSse.binds_listener());
}

#[test]
fn every_failure_kind_keeps_a_stable_namespaced_code() {
    for kind in [
        RegisteredToolFailureKind::UnsupportedRegistration,
        RegisteredToolFailureKind::UnknownExecutionOutcome,
        RegisteredToolFailureKind::TeardownFailed,
        RegisteredToolFailureKind::Revoked,
    ] {
        let failure = RegisteredToolFailure::new(kind);
        assert!(
            failure
                .diagnostic()
                .code()
                .starts_with("swallowtail.registered_tool.")
        );
        assert_eq!(
            failure.into_runtime_failure().diagnostic().code(),
            kind.code()
        );
    }
}
