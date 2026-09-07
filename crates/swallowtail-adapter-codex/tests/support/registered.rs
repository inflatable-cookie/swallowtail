//! Shared Contract 063 registered-tool fixtures for the Codex route suites.
//!
//! These fixtures mount the real local registered-tool bridge port beside the
//! scripted Codex app-server, so a case exercises the mounted wiring rather
//! than an isolated type fixture.

use super::app_server::AppServerState;
use super::{app_server_plan_with, host_services};
use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use swallowtail_adapter_codex::CodexRegisteredToolBinding;
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityRequirement, DriverRole, ExecutionHostId,
    HostServiceKind, PreflightPlan,
};
use swallowtail_host_local::{LocalHostServices, LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    BoxFuture, HostServices, MonotonicInstant, ProcessService,
    REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION, RegisteredServerId, RegisteredServerRevision,
    RegisteredToolBounds, RegisteredToolCall, RegisteredToolDeclaration,
    RegisteredToolDispatchContext, RegisteredToolDispatcher, RegisteredToolEffectPosture,
    RegisteredToolExecutionKind, RegisteredToolId, RegisteredToolLimits, RegisteredToolLocalName,
    RegisteredToolNamespace, RegisteredToolOutcome, RegisteredToolPayload,
    RegisteredToolPreparation, RegisteredToolProtocolVersion, RegisteredToolResult,
    RegisteredToolRetryPosture, RegisteredToolSchema, RegisteredToolSchemaDialect,
    RegisteredToolSchemaDigest, RegisteredToolSchemaDocument, RegisteredToolSchemaMediaType,
    RegisteredToolSchemaNamespace, RegisteredToolSelection, RegisteredToolSnapshot,
    RegisteredToolSnapshotInput, RegisteredToolSource, RegisteredToolSourceId,
    RegisteredToolTransport, RegisteredToolTransportSupport, RuntimeFailure, SessionOptions,
};
use swallowtail_testkit::{ScriptedAdmissionPort, fixture_admission};

/// The one flat dynamic tool name the namespaced identity renders as.
pub const REGISTERED_TOOL_WIRE_NAME: &str = "desktop__task_ledger";
const REGISTERED_SCHEMA: &str = r#"{"type":"object","properties":{"operation":{"type":"string"}}}"#;
const FIXTURE_CLEANUP_BUDGET: Duration = Duration::from_millis(25);

/// Returns the namespaced identity every registered fixture selects.
pub fn registered_tool_id() -> RegisteredToolId {
    RegisteredToolId::new(
        RegisteredToolNamespace::new("desktop").expect("namespace is valid"),
        RegisteredToolLocalName::new("task_ledger").expect("local name is valid"),
    )
}

fn registered_schema() -> RegisteredToolSchema {
    RegisteredToolSchema::new(
        RegisteredToolSchemaNamespace::new("desktop.tools").expect("schema namespace is valid"),
        RegisteredToolSchemaMediaType::new("application/schema+json").expect("media type is valid"),
        RegisteredToolSchemaDialect::new("json-schema-2020-12").expect("dialect is valid"),
        RegisteredServerRevision::new("rev-1").expect("revision is valid"),
        RegisteredToolSchemaDigest::new("sha256:input").expect("digest is valid"),
        RegisteredToolSchemaDocument::new(REGISTERED_SCHEMA).expect("schema body is bounded"),
    )
}

fn registered_snapshot(kind: RegisteredToolExecutionKind) -> Arc<RegisteredToolSnapshot> {
    let declaration = RegisteredToolDeclaration::new(
        registered_tool_id(),
        kind,
        registered_schema(),
        registered_schema(),
        RegisteredToolEffectPosture::Mutating,
        RegisteredToolRetryPosture::ConsumerRetryable,
        RegisteredToolBounds::ceiling(),
    )
    .expect("declaration is valid");
    Arc::new(
        RegisteredToolSnapshot::new(RegisteredToolSnapshotInput {
            server_id: RegisteredServerId::new("desktop.tools").expect("server id is valid"),
            revision: RegisteredServerRevision::new("rev-1").expect("revision is valid"),
            execution_host_id: ExecutionHostId::new("host.local").expect("host id is valid"),
            declarations: vec![declaration],
            transports: vec![
                RegisteredToolTransportSupport::new(
                    RegisteredToolTransport::HostMediatedCallback,
                    [protocol_version()],
                )
                .expect("transport support is valid"),
            ],
            required_services: BTreeSet::from([HostServiceKind::Task]),
            credential_references: Vec::new(),
            executable_recipes: Vec::new(),
            environment_recipes: Vec::new(),
            bounds: RegisteredToolBounds::ceiling(),
            source: RegisteredToolSource::new(
                RegisteredToolSourceId::new("desktop.registration").expect("source id is valid"),
                MonotonicInstant::from_ticks(0),
            ),
        })
        .expect("snapshot is valid"),
    )
}

fn protocol_version() -> RegisteredToolProtocolVersion {
    RegisteredToolProtocolVersion::new(REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION)
        .expect("protocol version is valid")
}

/// Builds one opt-in preparation selecting the fixture tool at one exact kind.
pub fn registered_preparation(
    admission: &Arc<ScriptedAdmissionPort>,
    kind: RegisteredToolExecutionKind,
) -> RegisteredToolPreparation {
    let snapshot = registered_snapshot(kind);
    let selection = RegisteredToolSelection::new(
        Arc::clone(&snapshot),
        [registered_tool_id()],
        RegisteredToolTransport::HostMediatedCallback,
        protocol_version(),
    )
    .expect("selection is valid");
    RegisteredToolPreparation::new(
        snapshot,
        selection,
        fixture_admission(Arc::clone(admission)),
        RegisteredToolLimits::ceiling(),
    )
}

/// Qualifies the native fixture preparation for the Codex route.
pub fn registered_binding(admission: &Arc<ScriptedAdmissionPort>) -> CodexRegisteredToolBinding {
    CodexRegisteredToolBinding::qualify(registered_preparation(
        admission,
        RegisteredToolExecutionKind::NativeClient,
    ))
    .expect("Codex qualifies a native registered tool")
}

/// Mounts the real local registered-tool port beside the scripted provider.
pub fn registered_services(
    process: Arc<dyn ProcessService>,
    dispatcher: Arc<dyn RegisteredToolDispatcher>,
) -> (HostServices, LocalHostServices) {
    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .with_registered_tool_dispatcher(dispatcher)
        .with_registered_tool_cleanup_budget(FIXTURE_CLEANUP_BUDGET)
        .build_services(ExecutionHostId::new("host.local").expect("host id is valid"));
    let port = local
        .services()
        .registered_tool_bridge()
        .expect("the local composition mounts the registered-tool port")
        .clone();
    (
        host_services(process).with_registered_tool_bridge(port),
        local,
    )
}

/// Returns the session options a low-level registered open must transport.
pub fn registered_options(binding: &CodexRegisteredToolBinding) -> SessionOptions {
    SessionOptions::default().with_tools(binding.declarations().to_vec())
}

/// Returns the preflight plan the registered fixture session agrees to.
pub fn registered_plan() -> PreflightPlan {
    app_server_plan_with(
        DriverRole::InteractiveSession,
        [CapabilityRequirement::new(
            Capability::ToolCalls,
            [
                CapabilityConstraint::ToolMaximumCount(4),
                CapabilityConstraint::ToolMaximumSchemaBytes(4096),
                CapabilityConstraint::tool_schema_dialect("json-schema-2020-12")
                    .expect("dialect is valid"),
            ],
        )],
        [],
    )
}

/// Waits for the exact provider response to the fixture tool call.
pub fn tool_response(state: &AppServerState) -> serde_json::Value {
    state.wait_for_message(|message| {
        message.get("id").and_then(serde_json::Value::as_str) == Some("callback-900")
            && message.get("result").is_some()
    })
}

/// Returns the bounded text one provider tool result carries.
pub fn result_text(response: &serde_json::Value) -> String {
    response["result"]["contentItems"][0]["text"]
        .as_str()
        .expect("the tool result carries bounded text")
        .to_owned()
}

type ScriptedOutcome =
    dyn Fn(&RegisteredToolCall) -> Result<RegisteredToolOutcome, RuntimeFailure> + Send + Sync;

/// Linked dispatcher that returns one exact scripted outcome.
pub struct RouteDispatcher {
    outcome: Box<ScriptedOutcome>,
    dispatches: AtomicUsize,
}

impl RouteDispatcher {
    /// Creates a dispatcher from one exact scripted outcome.
    pub fn new(
        outcome: impl Fn(&RegisteredToolCall) -> Result<RegisteredToolOutcome, RuntimeFailure>
        + Send
        + Sync
        + 'static,
    ) -> Arc<Self> {
        Arc::new(Self {
            outcome: Box::new(outcome),
            dispatches: AtomicUsize::new(0),
        })
    }

    /// Returns how many calls the kernel committed to this dispatcher.
    pub fn dispatches(&self) -> usize {
        self.dispatches.load(Ordering::SeqCst)
    }
}

impl RegisteredToolDispatcher for RouteDispatcher {
    fn dispatch(
        &self,
        call: RegisteredToolCall,
        _context: RegisteredToolDispatchContext,
    ) -> BoxFuture<'_, Result<RegisteredToolOutcome, RuntimeFailure>> {
        self.dispatches.fetch_add(1, Ordering::SeqCst);
        let outcome = (self.outcome)(&call);
        Box::pin(async move { outcome })
    }
}

/// Builds one bounded text result for a settled call.
pub fn text_result(call: &RegisteredToolCall, text: &str) -> RegisteredToolOutcome {
    RegisteredToolOutcome::completed(
        call,
        RegisteredToolResult::new(
            RegisteredToolPayload::new(
                RegisteredToolSchemaMediaType::new("application/json")
                    .expect("media type is valid"),
                text.as_bytes().to_vec(),
                call.binding().effective_bounds().max_result_bytes(),
            )
            .expect("result payload is bounded"),
            RegisteredToolSchemaDigest::new("sha256:output").expect("digest is valid"),
        ),
    )
}

/// Spins until a fixture condition holds, or fails the case.
pub fn wait_until(condition: impl Fn() -> bool) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while !condition() {
        assert!(Instant::now() < deadline, "fixture condition never held");
        std::thread::yield_now();
    }
}
