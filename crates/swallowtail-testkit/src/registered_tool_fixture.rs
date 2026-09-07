//! Provider-free fixtures for the Contract 063 registered-tool profile.
//!
//! Every fixture is deterministic and provider-free. Nothing here starts a
//! process, binds a listener, reads credentials, or claims route support.

use std::future::ready;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::Duration;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    AdmissionPhase, AdmissionVerdict, AdmittedAttemptId, AdmittedSessionId, AdmittedTaskId,
    BoxFuture, ConsumerAdmissionBinding, ConsumerAdmissionHostService, ConsumerProcessIncarnation,
    ConsumerTaskGeneration, ConsumerWorkspaceGeneration, Deadline, DeadlineObservation,
    MonotonicInstant, REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION, RegisteredServerId,
    RegisteredServerRevision, RegisteredToolBounds, RegisteredToolCall, RegisteredToolDeclaration,
    RegisteredToolDispatchContext, RegisteredToolDispatcher, RegisteredToolEffectPosture,
    RegisteredToolExecutionKind, RegisteredToolId, RegisteredToolLocalName,
    RegisteredToolNamespace, RegisteredToolOutcome, RegisteredToolPayload,
    RegisteredToolProtocolVersion, RegisteredToolReasonCode, RegisteredToolRetryPosture,
    RegisteredToolSchema, RegisteredToolSchemaDialect, RegisteredToolSchemaDigest,
    RegisteredToolSchemaDocument, RegisteredToolSchemaMediaType, RegisteredToolSchemaNamespace,
    RegisteredToolSelection, RegisteredToolSnapshot, RegisteredToolSnapshotInput,
    RegisteredToolSource, RegisteredToolSourceId, RegisteredToolTransport,
    RegisteredToolTransportSupport, RuntimeFailure, TimeService,
};

/// Namespace used by every provider-free registered-tool fixture.
pub const FIXTURE_TOOL_NAMESPACE: &str = "swallowtail.conformance";
/// Native client tool declared by the canonical fixture snapshot.
pub const FIXTURE_NATIVE_TOOL: &str = "echo";
/// MCP tool declared by the canonical fixture snapshot.
pub const FIXTURE_MCP_TOOL: &str = "reconcile";
/// Provider-owned tool declared by the canonical fixture snapshot.
pub const FIXTURE_PROVIDER_TOOL: &str = "observe";
/// Media type used by fixture schemas and payloads.
pub const FIXTURE_MEDIA_TYPE: &str = "application/json";

/// Builds one bounded fixture media type.
#[must_use]
pub fn fixture_media_type() -> RegisteredToolSchemaMediaType {
    RegisteredToolSchemaMediaType::new(FIXTURE_MEDIA_TYPE).expect("fixture media type")
}

/// Builds one namespaced fixture tool identity.
#[must_use]
pub fn fixture_tool_id(local_name: &str) -> RegisteredToolId {
    RegisteredToolId::new(
        RegisteredToolNamespace::new(FIXTURE_TOOL_NAMESPACE).expect("fixture namespace"),
        RegisteredToolLocalName::new(local_name).expect("fixture local name"),
    )
}

/// Builds one bounded fixture schema of an exact byte length.
#[must_use]
pub fn fixture_schema(revision: &str, digest: &str, body: &str) -> RegisteredToolSchema {
    RegisteredToolSchema::new(
        RegisteredToolSchemaNamespace::new("swallowtail.conformance.schema").expect("namespace"),
        fixture_media_type(),
        RegisteredToolSchemaDialect::new("json-schema-2020-12").expect("dialect"),
        RegisteredServerRevision::new(revision).expect("revision"),
        RegisteredToolSchemaDigest::new(digest).expect("digest"),
        RegisteredToolSchemaDocument::new(body).expect("document"),
    )
}

/// Builds one fixture declaration binding one identity to one execution kind.
#[must_use]
pub fn fixture_declaration(
    local_name: &str,
    kind: RegisteredToolExecutionKind,
) -> RegisteredToolDeclaration {
    RegisteredToolDeclaration::new(
        fixture_tool_id(local_name),
        kind,
        fixture_schema("1", "sha256:input", "{\"type\":\"object\"}"),
        fixture_schema("1", "sha256:output", "{\"type\":\"object\"}"),
        RegisteredToolEffectPosture::Mutating,
        RegisteredToolRetryPosture::ConsumerRetryable,
        RegisteredToolBounds::ceiling(),
    )
    .expect("fixture declaration")
}

/// Builds the canonical provider-free registration snapshot.
#[must_use]
pub fn fixture_snapshot(execution_host_id: &ExecutionHostId) -> RegisteredToolSnapshot {
    RegisteredToolSnapshot::new(fixture_snapshot_input(execution_host_id))
        .expect("fixture snapshot")
}

/// Returns the canonical snapshot input so a case can vary exactly one field.
#[must_use]
pub fn fixture_snapshot_input(execution_host_id: &ExecutionHostId) -> RegisteredToolSnapshotInput {
    RegisteredToolSnapshotInput {
        server_id: RegisteredServerId::new("swallowtail.conformance.server").expect("server id"),
        revision: RegisteredServerRevision::new("2026-09-07.1").expect("revision"),
        execution_host_id: execution_host_id.clone(),
        declarations: vec![
            fixture_declaration(
                FIXTURE_NATIVE_TOOL,
                RegisteredToolExecutionKind::NativeClient,
            ),
            fixture_declaration(FIXTURE_MCP_TOOL, RegisteredToolExecutionKind::Mcp),
            fixture_declaration(
                FIXTURE_PROVIDER_TOOL,
                RegisteredToolExecutionKind::ProviderOwned,
            ),
        ],
        transports: vec![
            RegisteredToolTransportSupport::new(
                RegisteredToolTransport::HostMediatedCallback,
                [fixture_protocol_version()],
            )
            .expect("callback transport"),
        ],
        required_services: [].into_iter().collect(),
        credential_references: Vec::new(),
        executable_recipes: Vec::new(),
        environment_recipes: Vec::new(),
        bounds: RegisteredToolBounds::ceiling(),
        source: RegisteredToolSource::new(
            RegisteredToolSourceId::new("consumer.registration.1").expect("source id"),
            MonotonicInstant::from_ticks(1),
        ),
    }
}

/// Returns the one explicitly named provider-free conformance protocol version.
#[must_use]
pub fn fixture_protocol_version() -> RegisteredToolProtocolVersion {
    RegisteredToolProtocolVersion::new(REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION)
        .expect("conformance protocol version")
}

/// Builds the canonical selection over the fixture snapshot.
#[must_use]
pub fn fixture_selection(snapshot: Arc<RegisteredToolSnapshot>) -> RegisteredToolSelection {
    RegisteredToolSelection::new(
        snapshot,
        [
            fixture_tool_id(FIXTURE_NATIVE_TOOL),
            fixture_tool_id(FIXTURE_MCP_TOOL),
            fixture_tool_id(FIXTURE_PROVIDER_TOOL),
        ],
        RegisteredToolTransport::HostMediatedCallback,
        fixture_protocol_version(),
    )
    .expect("fixture selection")
}

/// Builds one bounded fixture payload of an exact byte length.
#[must_use]
pub fn fixture_payload(bytes: usize, max_bytes: usize) -> RegisteredToolPayload {
    RegisteredToolPayload::new(fixture_media_type(), vec![b'x'; bytes], max_bytes)
        .expect("fixture payload")
}

/// Deterministic host clock a case advances by exact ticks.
///
/// Ticks are nanoseconds, matching the local host composition, so a case can
/// prove deadline behavior without waiting on wall-clock time.
#[derive(Default)]
pub struct FakeClock {
    ticks: Mutex<u64>,
}

impl FakeClock {
    /// Creates a clock at an exact tick.
    #[must_use]
    pub fn at(ticks: u64) -> Self {
        Self {
            ticks: Mutex::new(ticks),
        }
    }

    /// Advances the clock by an exact number of ticks.
    pub fn advance(&self, ticks: u64) {
        let mut current = self.ticks.lock().expect("fake clock lock");
        *current = current.saturating_add(ticks);
    }
}

impl TimeService for FakeClock {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(*self.ticks.lock().expect("fake clock lock"))
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        let observed = self.now();
        Box::pin(ready(DeadlineObservation::new(deadline, observed)))
    }
}

/// Consumer admission port whose live verdict a case can script exactly.
///
/// A case may also hold validations pending, so it can force the exact race
/// where one live verdict is in flight while another admission sequence tries
/// to start.
pub struct ScriptedAdmissionPort {
    revoke_at: Mutex<Option<AdmissionPhase>>,
    revoke_after: Mutex<Option<usize>>,
    observed: Mutex<Vec<AdmissionPhase>>,
    held: Mutex<bool>,
    waiters: Mutex<Vec<Waker>>,
}

impl Default for ScriptedAdmissionPort {
    fn default() -> Self {
        Self::current()
    }
}

impl ScriptedAdmissionPort {
    /// Creates a port that always answers `Current`.
    #[must_use]
    pub fn current() -> Self {
        Self {
            revoke_at: Mutex::new(None),
            revoke_after: Mutex::new(None),
            observed: Mutex::new(Vec::new()),
            held: Mutex::new(false),
            waiters: Mutex::new(Vec::new()),
        }
    }

    /// Holds every later validation pending until it is released.
    pub fn hold(&self) {
        *self.held.lock().expect("hold lock") = true;
    }

    /// Completes every held validation.
    pub fn release(&self) {
        *self.held.lock().expect("hold lock") = false;
        for waker in std::mem::take(&mut *self.waiters.lock().expect("waiter lock")) {
            waker.wake();
        }
    }

    /// Returns how many validations this port has been asked for.
    #[must_use]
    pub fn validation_count(&self) -> usize {
        self.observed.lock().expect("observed lock").len()
    }

    /// Revokes from the next validation at one exact phase onwards.
    pub fn revoke_from(&self, phase: AdmissionPhase) {
        *self.revoke_at.lock().expect("revocation lock") = Some(phase);
    }

    /// Revokes every validation from now on, including any held pending one.
    pub fn revoke_now(&self) {
        *self.revoke_after.lock().expect("revocation lock") = Some(0);
    }

    /// Revokes every validation after an exact number of current verdicts.
    ///
    /// This places a revocation between dispatch and a later progress or
    /// delivery check without any timing dependency.
    pub fn revoke_after_validations(&self, count: usize) {
        *self.revoke_after.lock().expect("revocation lock") = Some(count);
    }

    /// Returns every phase the kernel validated, in order.
    #[must_use]
    pub fn observed_phases(&self) -> Vec<AdmissionPhase> {
        self.observed.lock().expect("observed lock").clone()
    }
}

struct ScriptedVerdict<'port> {
    port: &'port ScriptedAdmissionPort,
    phase: AdmissionPhase,
    recorded: bool,
}

impl Future for ScriptedVerdict<'_> {
    type Output = Result<AdmissionVerdict, RuntimeFailure>;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.recorded {
            // The request is recorded when it is first made, so a case can see
            // that a live verdict is in flight while it is still pending.
            self.port
                .observed
                .lock()
                .expect("observed lock")
                .push(self.phase);
            self.recorded = true;
        }
        if *self.port.held.lock().expect("hold lock") {
            let mut waiters = self.port.waiters.lock().expect("waiter lock");
            let waker = context.waker();
            if !waiters.iter().any(|waiting| waiting.will_wake(waker)) {
                waiters.push(waker.clone());
            }
            return Poll::Pending;
        }
        Poll::Ready(Ok(self.port.verdict_for(self.phase)))
    }
}

impl ScriptedAdmissionPort {
    fn verdict_for(&self, phase: AdmissionPhase) -> AdmissionVerdict {
        let seen = self.observed.lock().expect("observed lock").len();
        let revoked_by_phase = matches!(
            *self.revoke_at.lock().expect("revocation lock"),
            Some(revoke_at) if revoke_at == phase
                || (revoke_at == AdmissionPhase::BeforeDispatch
                    && phase == AdmissionPhase::BeforeDelivery)
        );
        let revoked_by_count = matches!(
            *self.revoke_after.lock().expect("revocation lock"),
            Some(after) if seen > after
        );
        if revoked_by_phase || revoked_by_count {
            AdmissionVerdict::Revoked(
                RegisteredToolReasonCode::new("consumer.task_superseded").expect("reason code"),
            )
        } else {
            AdmissionVerdict::Current
        }
    }
}

impl ConsumerAdmissionHostService for ScriptedAdmissionPort {
    fn validate(
        &self,
        _binding: &ConsumerAdmissionBinding,
        phase: AdmissionPhase,
    ) -> BoxFuture<'_, Result<AdmissionVerdict, RuntimeFailure>> {
        Box::pin(ScriptedVerdict {
            port: self,
            phase,
            recorded: false,
        })
    }
}

/// Builds one trusted admission binding over a scripted port.
#[must_use]
pub fn fixture_admission(port: Arc<ScriptedAdmissionPort>) -> ConsumerAdmissionBinding {
    ConsumerAdmissionBinding::new(
        ConsumerProcessIncarnation::new("incarnation-1").expect("incarnation"),
        ConsumerWorkspaceGeneration::initial(),
        ConsumerTaskGeneration::initial(),
        AdmittedTaskId::new("task-1").expect("task"),
        AdmittedSessionId::new("session-1").expect("session"),
        AdmittedAttemptId::new("attempt-1").expect("attempt"),
        port,
    )
}

type DispatchBehaviour = dyn Fn(
        &RegisteredToolCall,
        &RegisteredToolDispatchContext,
    ) -> Result<RegisteredToolOutcome, RuntimeFailure>
    + Send
    + Sync;

/// Linked dispatcher whose exact behavior one case scripts.
pub struct ScriptedRegisteredToolDispatcher {
    behaviour: Arc<DispatchBehaviour>,
    dispatches: AtomicUsize,
}

impl ScriptedRegisteredToolDispatcher {
    /// Creates a dispatcher from one exact scripted behavior.
    #[must_use]
    pub fn new(behaviour: Arc<DispatchBehaviour>) -> Self {
        Self {
            behaviour,
            dispatches: AtomicUsize::new(0),
        }
    }

    /// Creates a dispatcher that echoes bounded arguments as its result.
    #[must_use]
    pub fn echoing() -> Self {
        Self::new(Arc::new(|call, _context| {
            Ok(RegisteredToolOutcome::completed(
                call,
                swallowtail_runtime::RegisteredToolResult::new(
                    fixture_payload(8, call.binding().effective_bounds().max_result_bytes()),
                    RegisteredToolSchemaDigest::new("sha256:output").expect("digest"),
                ),
            ))
        }))
    }

    /// Returns how many times the kernel committed a call to this dispatcher.
    #[must_use]
    pub fn dispatches(&self) -> usize {
        self.dispatches.load(Ordering::SeqCst)
    }
}

impl RegisteredToolDispatcher for ScriptedRegisteredToolDispatcher {
    fn dispatch(
        &self,
        call: RegisteredToolCall,
        context: RegisteredToolDispatchContext,
    ) -> BoxFuture<'_, Result<RegisteredToolOutcome, RuntimeFailure>> {
        self.dispatches.fetch_add(1, Ordering::SeqCst);
        let outcome = (self.behaviour)(&call, &context);
        Box::pin(ready(outcome))
    }
}

/// Deliberately uncooperative dispatcher that never settles a call.
#[derive(Default)]
pub struct UncooperativeRegisteredToolDispatcher {
    dispatches: AtomicUsize,
}

impl UncooperativeRegisteredToolDispatcher {
    /// Returns how many calls the kernel committed to this dispatcher.
    #[must_use]
    pub fn dispatches(&self) -> usize {
        self.dispatches.load(Ordering::SeqCst)
    }
}

impl RegisteredToolDispatcher for UncooperativeRegisteredToolDispatcher {
    fn dispatch(
        &self,
        _call: RegisteredToolCall,
        _context: RegisteredToolDispatchContext,
    ) -> BoxFuture<'_, Result<RegisteredToolOutcome, RuntimeFailure>> {
        self.dispatches.fetch_add(1, Ordering::SeqCst);
        Box::pin(std::future::pending())
    }
}

/// Bounded cleanup budget every deterministic fixture uses.
pub const FIXTURE_CLEANUP_BUDGET: Duration = Duration::from_millis(25);

const MAX_FIXTURE_POLLS: usize = 4096;

/// Drives one provider-free future to completion without an executor.
///
/// Fixture futures never block on real I/O, so a bounded poll loop is exact.
///
/// # Panics
///
/// Panics when a fixture future never becomes ready.
pub fn drive_fixture<T>(future: BoxFuture<'_, T>) -> T {
    let mut future = future;
    let mut context = Context::from_waker(Waker::noop());
    for _ in 0..MAX_FIXTURE_POLLS {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value,
            Poll::Pending => {}
        }
    }
    panic!("fixture future never settled");
}

/// Polls one future exactly once and reports whether it stayed pending.
///
/// This models an issued call whose dispatcher has not settled it yet.
pub fn poll_fixture_once<T>(future: &mut Pin<Box<dyn Future<Output = T> + Send + '_>>) -> Poll<T> {
    let mut context = Context::from_waker(Waker::noop());
    future.as_mut().poll(&mut context)
}

/// Drives two futures round-robin so their admission sequences must interleave.
///
/// This forces the exact concurrency the serialized admission gate must
/// linearize: both futures are in flight at once and neither completes before
/// the other starts.
///
/// # Panics
///
/// Panics when either fixture future never settles.
pub fn interleave_fixtures<T>(
    first: &mut Pin<Box<dyn Future<Output = T> + Send + '_>>,
    second: &mut Pin<Box<dyn Future<Output = T> + Send + '_>>,
) -> (T, T) {
    let mut first_done = None;
    let mut second_done = None;
    for _ in 0..MAX_FIXTURE_POLLS {
        if first_done.is_none()
            && let Poll::Ready(value) = poll_fixture_once(first)
        {
            first_done = Some(value);
        }
        if second_done.is_none()
            && let Poll::Ready(value) = poll_fixture_once(second)
        {
            second_done = Some(value);
        }
        if first_done.is_some() && second_done.is_some() {
            break;
        }
    }
    (
        first_done.expect("the first fixture future never settled"),
        second_done.expect("the second fixture future never settled"),
    )
}
