#![allow(dead_code)]

//! Compose one admitted `claude-agent.sdk` sidecar instance with registered
//! tools, end to end, without touching a live provider.
//!
//! The application provisions the exact approved Node runtime, the
//! source-tagged sidecar entry point, the exact
//! `@anthropic-ai/claude-agent-sdk` package with its peer dependencies, and
//! the exact platform package that carries the native `claude` binary, all
//! through a host-approved interpreted-script launch recipe. Swallowtail
//! never installs, vendors, updates, repairs, or redistributes any of them,
//! and never holds the subscription credential: the user runs the official
//! Claude login out of band and the native binary authenticates itself.
//!
//! This example is the full admission composition that the guide's Explicit
//! Inputs section describes:
//!
//! 1. `admit_claude_agent_sdk` admits the `claude-agent.sdk` addable route
//!    with opaque host-owned references only — the launch recipe, the
//!    environment reference whose approved body carries
//!    `CLAUDE_AGENT_SDK_SIDECAR_SDK_MODULE`, `_NATIVE_BINARY`, and
//!    `_MANIFEST`, and the delegated subscription credential — and writes one
//!    `AdmittedInstanceRecord`;
//! 2. `ClaudeAgentSdkSessionPreparation::from_admitted` lifts that record
//!    with the explicit per-session model route and open deadline, without
//!    exposing paths, environment values, or credential bytes;
//! 3. `mediated_registered_tool_preparation` builds the card 125 registered
//!    tool opt-in: one immutable registration snapshot, one selection over
//!    the private-loopback carrier carrying the mediated-stdio proxy
//!    attachment with the host-approved courier proxy recipe — the exact
//!    shape `ClaudeAgentSdkRegisteredToolBinding::qualify` admits — and one
//!    consumer admission binding;
//! 4. `prepare_admitted_session_with_registered_tools` binds that preparation
//!    with `with_registered_tools(preparation, host)` and prepares the
//!    session; `open_and_run_turn` then opens, starts one turn, admits tool
//!    use, observes the terminal outcome, closes the turn, and reads the
//!    cleanup outcome.
//!
//! Provider-free: nothing here starts Node, the SDK, the native binary, a
//! courier, or a provider session. The example composes the same types and
//! sequence a consumer host runs, and spends no live route.

use std::sync::Arc;
use swallowtail_adapter_claude_agent::sdk::{
    CLAUDE_AGENT_SDK_CREDENTIAL_FIELD_ID, CLAUDE_AGENT_SDK_ENVIRONMENT_FIELD_ID,
    CLAUDE_AGENT_SDK_LAUNCH_RECIPE_FIELD_ID, ClaudeAgentSdkPreparedSession,
    ClaudeAgentSdkSessionPreparation, claude_agent_sdk_addable_route_descriptor,
    prepare_claude_agent_sdk_session,
};
use swallowtail_core::{
    AccessProfileId, AdmittedInstanceRecord, ConfigFieldId, ConfigFieldRef, ConfiguredInstanceId,
    CredentialFieldId, CredentialRef, ExecutionHostId, InstanceRevision, IntegrationFamilyId,
    ModelId, ModelRouteId, ModelRouteRevision,
};
use swallowtail_host_local::{LocalHostServices, MemoryConnectionLifecycleStore};
use swallowtail_runtime::{
    AddableRouteCatalog, AdmissionPhase, AdmissionVerdict, AdmittedAttemptId, AdmittedSessionId,
    AdmittedTaskId, BoxFuture, CallbackResponse, CleanupOutcome, ConsumerAdmissionBinding,
    ConsumerAdmissionHostService, ConsumerProcessIncarnation, ConsumerTaskGeneration,
    ConsumerWorkspaceGeneration, Deadline, EnvironmentRef, ExecutableRef, HostServices,
    InstanceAdmissionFailure, InstanceAdmissionRequest, MonotonicInstant, OperationContent,
    PreparationFailure, REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION,
    REGISTERED_TOOL_PROXY_WIRE_TAG, RegisteredServerId, RegisteredServerRevision,
    RegisteredToolAttachment, RegisteredToolBounds, RegisteredToolDeclaration,
    RegisteredToolEffectPosture, RegisteredToolExecutionKind, RegisteredToolId,
    RegisteredToolLimits, RegisteredToolLocalName, RegisteredToolNamespace,
    RegisteredToolPreparation, RegisteredToolProtocolVersion, RegisteredToolProxyRecipe,
    RegisteredToolRetryPosture, RegisteredToolSchema, RegisteredToolSchemaDialect,
    RegisteredToolSchemaDigest, RegisteredToolSchemaDocument, RegisteredToolSchemaMediaType,
    RegisteredToolSchemaNamespace, RegisteredToolSelection, RegisteredToolSnapshot,
    RegisteredToolSnapshotInput, RegisteredToolSource, RegisteredToolSourceId,
    RegisteredToolTransport, RegisteredToolTransportSupport, RequestId, RuntimeFailure,
    RuntimeTurnId, SessionCleanupRequest, SessionOptions, TerminalOutcome, TurnRequest,
    WorkingResourceRef, admit_instance,
};

/// Opaque host-owned references carried by the admitted record. Values stay
/// private to the host that owns the launch recipe, environment body, and
/// credential store; nothing here opens or inspects them.
const LAUNCH_RECIPE_REF: &str = "claude-agent.sdk.launch-recipe";
const SIDECAR_ENVIRONMENT_REF: &str = "claude-agent.sdk.sidecar-environment";
const DELEGATED_SUBSCRIPTION_REF: &str = "claude-agent.sdk.delegated-subscription";

/// Admits the `claude-agent.sdk` addable route with the three opaque
/// references the route requires: the launch recipe, the sidecar environment,
/// and the delegated subscription credential.
fn admit_claude_agent_sdk(
    services: &HostServices,
    store: &MemoryConnectionLifecycleStore,
    instance_id: ConfiguredInstanceId,
) -> Result<AdmittedInstanceRecord, InstanceAdmissionFailure> {
    let descriptor = claude_agent_sdk_addable_route_descriptor(services);
    let route_id = descriptor.id().clone();
    let catalog = AddableRouteCatalog::from_descriptors([descriptor]).expect("catalog assembles");
    admit_instance(
        &catalog,
        store,
        InstanceAdmissionRequest::new(
            instance_id,
            IntegrationFamilyId::new("claude-agent").expect("family id is valid"),
            route_id,
        )
        .with_config_refs([
            (
                ConfigFieldId::new(CLAUDE_AGENT_SDK_LAUNCH_RECIPE_FIELD_ID)
                    .expect("config id is valid"),
                ConfigFieldRef::new(LAUNCH_RECIPE_REF).expect("config ref is valid"),
            ),
            (
                ConfigFieldId::new(CLAUDE_AGENT_SDK_ENVIRONMENT_FIELD_ID)
                    .expect("config id is valid"),
                ConfigFieldRef::new(SIDECAR_ENVIRONMENT_REF).expect("config ref is valid"),
            ),
        ])
        .with_credential_refs([(
            CredentialFieldId::new(CLAUDE_AGENT_SDK_CREDENTIAL_FIELD_ID)
                .expect("credential id is valid"),
            CredentialRef::new(DELEGATED_SUBSCRIPTION_REF).expect("credential ref is valid"),
        )]),
    )
}

/// Lifts one admitted record with the explicit per-session model route and
/// open deadline, then prepares the fresh sidecar session. The open deadline
/// is caller-supplied and mandatory: it bounds open and every startup await
/// against the host clock.
fn prepare_admitted_session(
    admitted: &AdmittedInstanceRecord,
    execution_host: ExecutionHostId,
    deadline: Deadline,
) -> Result<ClaudeAgentSdkPreparedSession, PreparationFailure> {
    let preparation = ClaudeAgentSdkSessionPreparation::from_admitted(
        admitted,
        InstanceRevision::new("1").expect("revision is valid"),
        execution_host,
        AccessProfileId::new("claude-agent.sdk.subscription").expect("access id is valid"),
        ModelRouteId::new("claude-agent.sdk.models").expect("route id is valid"),
        ModelRouteRevision::new("2026-09-08.1").expect("route revision is valid"),
        ModelId::new("claude-sonnet-5").expect("model id is valid"),
        WorkingResourceRef::new("claude-agent.sdk.workspace").expect("workspace ref is valid"),
        RequestId::new("claude-agent-sdk.admitted-open").expect("request id is valid"),
        deadline,
    )?;
    prepare_claude_agent_sdk_session(preparation, SessionOptions::default())
}

/// Opaque host references for the registered-tool courier. The host approves
/// the exact courier executable and allowlisted environment; the recipe never
/// carries a path or environment body.
const COURIER_EXECUTABLE_REF: &str = "desktop.registered-tools.courier";
const COURIER_ENVIRONMENT_REF: &str = "desktop.registered-tools.courier-environment";

/// Shape-only stand-in for the consumer's live Contract 063 admission port.
///
/// The real consumer kernel answers `BeforeDispatch` and `BeforeDelivery`
/// revalidation from its own admitted attempt state; this always-current port
/// exists so the composition compiles and reads without that kernel.
struct CurrentAdmissionPort;

impl ConsumerAdmissionHostService for CurrentAdmissionPort {
    fn validate(
        &self,
        _binding: &ConsumerAdmissionBinding,
        _phase: AdmissionPhase,
    ) -> BoxFuture<'_, Result<AdmissionVerdict, RuntimeFailure>> {
        Box::pin(std::future::ready(Ok(AdmissionVerdict::Current)))
    }
}

fn consumer_admission_binding() -> ConsumerAdmissionBinding {
    ConsumerAdmissionBinding::new(
        ConsumerProcessIncarnation::new("desktop-incarnation-1").expect("incarnation is valid"),
        ConsumerWorkspaceGeneration::initial(),
        ConsumerTaskGeneration::initial(),
        AdmittedTaskId::new("desktop-task-1").expect("task id is valid"),
        AdmittedSessionId::new("desktop-session-1").expect("session id is valid"),
        AdmittedAttemptId::new("desktop-attempt-1").expect("attempt id is valid"),
        Arc::new(CurrentAdmissionPort),
    )
}

fn courier_executable() -> ExecutableRef {
    ExecutableRef::new(COURIER_EXECUTABLE_REF).expect("executable ref is valid")
}

fn courier_environment() -> EnvironmentRef {
    EnvironmentRef::new(COURIER_ENVIRONMENT_REF).expect("environment ref is valid")
}

/// One disposable MCP registration shaped like the card 132 gate's: server
/// `desktop.registered-tools`, tool `desktop/reconcile`, and the courier
/// executable and environment declared as recipes.
fn registered_tool_schema(digest: &str) -> RegisteredToolSchema {
    RegisteredToolSchema::new(
        RegisteredToolSchemaNamespace::new("desktop.registered-tools.schema")
            .expect("schema namespace is valid"),
        RegisteredToolSchemaMediaType::new("application/json").expect("media type is valid"),
        RegisteredToolSchemaDialect::new("json-schema-2020-12").expect("dialect is valid"),
        RegisteredServerRevision::new("1").expect("schema revision is valid"),
        RegisteredToolSchemaDigest::new(digest).expect("schema digest is valid"),
        RegisteredToolSchemaDocument::new("{\"type\":\"object\"}").expect("document is valid"),
    )
}

fn registered_tool_id() -> RegisteredToolId {
    RegisteredToolId::new(
        RegisteredToolNamespace::new("desktop").expect("namespace is valid"),
        RegisteredToolLocalName::new("reconcile").expect("local name is valid"),
    )
}

fn registered_tool_snapshot(execution_host: &ExecutionHostId) -> RegisteredToolSnapshot {
    RegisteredToolSnapshot::new(RegisteredToolSnapshotInput {
        server_id: RegisteredServerId::new("desktop.registered-tools").expect("server is valid"),
        revision: RegisteredServerRevision::new("2026-09-07.1").expect("revision is valid"),
        execution_host_id: execution_host.clone(),
        declarations: vec![
            RegisteredToolDeclaration::new(
                registered_tool_id(),
                RegisteredToolExecutionKind::Mcp,
                registered_tool_schema("sha256:input"),
                registered_tool_schema("sha256:output"),
                RegisteredToolEffectPosture::Mutating,
                RegisteredToolRetryPosture::ConsumerRetryable,
                RegisteredToolBounds::ceiling(),
            )
            .expect("declaration is valid"),
        ],
        transports: vec![
            RegisteredToolTransportSupport::new(
                RegisteredToolTransport::PrivateLoopbackHttp,
                [
                    RegisteredToolProtocolVersion::new(
                        REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION,
                    )
                    .expect("protocol version is valid"),
                ],
            )
            .expect("transport support is valid"),
        ],
        required_services: [].into_iter().collect(),
        credential_references: Vec::new(),
        executable_recipes: vec![courier_executable()],
        environment_recipes: vec![courier_environment()],
        bounds: RegisteredToolBounds::ceiling(),
        source: RegisteredToolSource::new(
            RegisteredToolSourceId::new("desktop.registration.1").expect("source id is valid"),
            MonotonicInstant::from_ticks(1),
        ),
    })
    .expect("snapshot is valid")
}

/// Builds the registered-tool opt-in in the exact shape card 125's route
/// binding admits: the mediated-stdio proxy attachment over the
/// private-loopback HTTP carrier, with the host-approved courier proxy
/// recipe. Host-mediated callback selections are a different route seam.
fn mediated_registered_tool_preparation(
    execution_host: &ExecutionHostId,
) -> RegisteredToolPreparation {
    let snapshot = Arc::new(registered_tool_snapshot(execution_host));
    let recipe = RegisteredToolProxyRecipe::new(
        courier_executable(),
        courier_environment(),
        REGISTERED_TOOL_PROXY_WIRE_TAG,
    )
    .expect("proxy recipe binds the fixed courier wire");
    let selection = RegisteredToolSelection::new(
        Arc::clone(&snapshot),
        [registered_tool_id()],
        RegisteredToolTransport::PrivateLoopbackHttp,
        RegisteredToolProtocolVersion::new(REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION)
            .expect("selection protocol version is valid"),
    )
    .expect("selection is valid")
    .with_attachment(RegisteredToolAttachment::MediatedStdioProxy)
    .with_proxy_recipe(recipe);
    RegisteredToolPreparation::new(
        snapshot,
        selection,
        consumer_admission_binding(),
        RegisteredToolLimits::ceiling(),
    )
}

/// Lifts the same admitted record and binds the mediated registered-tool
/// preparation through the openable card 125 entry. The local host resolves
/// the approved courier path and environment at bind time; absence of the
/// binding keeps the previous open unchanged.
fn prepare_admitted_session_with_registered_tools(
    admitted: &AdmittedInstanceRecord,
    execution_host: ExecutionHostId,
    local: LocalHostServices,
    registered: RegisteredToolPreparation,
    deadline: Deadline,
) -> Result<ClaudeAgentSdkPreparedSession, PreparationFailure> {
    let preparation = ClaudeAgentSdkSessionPreparation::from_admitted(
        admitted,
        InstanceRevision::new("1").expect("revision is valid"),
        execution_host,
        AccessProfileId::new("claude-agent.sdk.subscription").expect("access id is valid"),
        ModelRouteId::new("claude-agent.sdk.models").expect("route id is valid"),
        ModelRouteRevision::new("2026-09-08.1").expect("route revision is valid"),
        ModelId::new("claude-sonnet-5").expect("model id is valid"),
        WorkingResourceRef::new("claude-agent.sdk.workspace").expect("workspace ref is valid"),
        RequestId::new("claude-agent-sdk.admitted-registered-open").expect("request id is valid"),
        deadline,
    )?
    .with_registered_tools(registered, local)?;
    prepare_claude_agent_sdk_session(preparation, SessionOptions::default())
}

/// Opens the prepared session, runs one turn under the caller's services,
/// admits tool use through the turn's bounded callback exchange, observes the
/// terminal outcome, and reads the cleanup outcome from the caller's single
/// cleanup deadline.
async fn open_and_run_turn(
    prepared: &ClaudeAgentSdkPreparedSession,
    services: HostServices,
    turn_id: RuntimeTurnId,
    content: OperationContent,
    cleanup: SessionCleanupRequest,
) -> Result<(TerminalOutcome, CleanupOutcome), RuntimeFailure> {
    let services_for_cleanup = services.clone();
    let mut session = prepared.open_session(services.clone()).await?;
    let mut turn = session
        .start_turn(TurnRequest::new(turn_id, content), services)
        .await?;
    // Read-only tool admission is the consumer's decision, delivered through
    // the turn's bounded callback exchange.
    let _callbacks = turn.take_callbacks();
    let outcome = turn
        .take_terminal_outcome()
        .expect("sidecar turns expose one terminal outcome")
        .await;
    let _ = turn.close().await;
    Ok((outcome, session.close(cleanup, services_for_cleanup).await))
}

async fn admit_tool_use(
    exchange: &swallowtail_runtime::CallbackExchange,
    response: CallbackResponse,
) -> Result<(), RuntimeFailure> {
    exchange.responder().respond(response).await
}

fn main() {}
