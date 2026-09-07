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
    ConsumerTaskGeneration, ConsumerWorkspaceGeneration, MonotonicInstant,
    REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION, RegisteredServerId, RegisteredServerRevision,
    RegisteredToolBounds, RegisteredToolCall, RegisteredToolDeclaration,
    RegisteredToolDispatchContext, RegisteredToolDispatcher, RegisteredToolEffectPosture,
    RegisteredToolExecutionKind, RegisteredToolId, RegisteredToolLocalName,
    RegisteredToolNamespace, RegisteredToolOutcome, RegisteredToolPayload,
    RegisteredToolProtocolVersion, RegisteredToolReasonCode, RegisteredToolRetryPosture,
    RegisteredToolSchema, RegisteredToolSchemaDialect, RegisteredToolSchemaDigest,
    RegisteredToolSchemaDocument, RegisteredToolSchemaMediaType, RegisteredToolSchemaNamespace,
    RegisteredToolSelection, RegisteredToolSnapshot, RegisteredToolSnapshotInput,
    RegisteredToolSource, RegisteredToolSourceId, RegisteredToolTransport,
    RegisteredToolTransportSupport, RuntimeFailure,
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

/// Consumer admission port whose live verdict a case can script exactly.
pub struct ScriptedAdmissionPort {
    revoke_at: Mutex<Option<AdmissionPhase>>,
    observed: Mutex<Vec<AdmissionPhase>>,
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
            observed: Mutex::new(Vec::new()),
        }
    }

    /// Revokes from the next validation at one exact phase onwards.
    pub fn revoke_from(&self, phase: AdmissionPhase) {
        *self.revoke_at.lock().expect("revocation lock") = Some(phase);
    }

    /// Returns every phase the kernel validated, in order.
    #[must_use]
    pub fn observed_phases(&self) -> Vec<AdmissionPhase> {
        self.observed.lock().expect("observed lock").clone()
    }
}

impl ConsumerAdmissionHostService for ScriptedAdmissionPort {
    fn validate(
        &self,
        _binding: &ConsumerAdmissionBinding,
        phase: AdmissionPhase,
    ) -> BoxFuture<'_, Result<AdmissionVerdict, RuntimeFailure>> {
        self.observed.lock().expect("observed lock").push(phase);
        let revoked = matches!(
            *self.revoke_at.lock().expect("revocation lock"),
            Some(revoke_at) if revoke_at == phase
                || (revoke_at == AdmissionPhase::BeforeDispatch
                    && phase == AdmissionPhase::BeforeDelivery)
        );
        let verdict = if revoked {
            AdmissionVerdict::Revoked(
                RegisteredToolReasonCode::new("consumer.task_superseded").expect("reason code"),
            )
        } else {
            AdmissionVerdict::Current
        };
        Box::pin(ready(Ok(verdict)))
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
