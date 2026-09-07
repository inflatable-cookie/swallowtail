//! Qualification counterexamples for the Codex dynamic native tool seam.

use super::{CODEX_REGISTERED_TOOL_ROUTE, CodexRegisteredToolBinding};
use std::collections::BTreeSet;
use std::sync::Arc;
use swallowtail_core::{ExecutionHostId, HostServiceKind};
use swallowtail_runtime::{
    AdmissionPhase, AdmissionVerdict, AdmittedAttemptId, AdmittedSessionId, AdmittedTaskId,
    BoxFuture, ConsumerAdmissionBinding, ConsumerAdmissionHostService, ConsumerProcessIncarnation,
    ConsumerTaskGeneration, ConsumerWorkspaceGeneration, MonotonicInstant,
    REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION, RegisteredServerId, RegisteredServerRevision,
    RegisteredToolBounds, RegisteredToolDeclaration, RegisteredToolEffectPosture,
    RegisteredToolExecutionKind, RegisteredToolId, RegisteredToolLimits, RegisteredToolLocalName,
    RegisteredToolNamespace, RegisteredToolPreparation, RegisteredToolProgressMode,
    RegisteredToolProtocolVersion, RegisteredToolRetryPosture, RegisteredToolSchema,
    RegisteredToolSchemaDialect, RegisteredToolSchemaDigest, RegisteredToolSchemaDocument,
    RegisteredToolSchemaMediaType, RegisteredToolSchemaNamespace, RegisteredToolSelection,
    RegisteredToolSkillDelivery, RegisteredToolSnapshot, RegisteredToolSnapshotInput,
    RegisteredToolSource, RegisteredToolSourceId, RegisteredToolTransport,
    RegisteredToolTransportSupport, RuntimeFailure,
};

const NAMESPACE: &str = "desktop";
const SCHEMA: &str = r#"{"type":"object","properties":{"operation":{"type":"string"}}}"#;

struct CurrentAdmission;

impl ConsumerAdmissionHostService for CurrentAdmission {
    fn validate(
        &self,
        _binding: &ConsumerAdmissionBinding,
        _phase: AdmissionPhase,
    ) -> BoxFuture<'_, Result<AdmissionVerdict, RuntimeFailure>> {
        Box::pin(async { Ok(AdmissionVerdict::Current) })
    }
}

pub(crate) fn tool_id(namespace: &str, local_name: &str) -> RegisteredToolId {
    RegisteredToolId::new(
        RegisteredToolNamespace::new(namespace).expect("namespace is valid"),
        RegisteredToolLocalName::new(local_name).expect("local name is valid"),
    )
}

fn declaration(
    id: RegisteredToolId,
    kind: RegisteredToolExecutionKind,
) -> RegisteredToolDeclaration {
    declaration_with(id, kind, "application/schema+json", SCHEMA)
}

fn declaration_with(
    id: RegisteredToolId,
    kind: RegisteredToolExecutionKind,
    media_type: &str,
    body: &str,
) -> RegisteredToolDeclaration {
    let schema = RegisteredToolSchema::new(
        RegisteredToolSchemaNamespace::new("desktop.tools").expect("schema namespace is valid"),
        RegisteredToolSchemaMediaType::new(media_type).expect("media type is valid"),
        RegisteredToolSchemaDialect::new("json-schema-2020-12").expect("dialect is valid"),
        RegisteredServerRevision::new("rev-1").expect("revision is valid"),
        RegisteredToolSchemaDigest::new("sha256:input").expect("digest is valid"),
        RegisteredToolSchemaDocument::new(body).expect("schema body is bounded"),
    );
    RegisteredToolDeclaration::new(
        id,
        kind,
        schema.clone(),
        schema,
        RegisteredToolEffectPosture::Mutating,
        RegisteredToolRetryPosture::ConsumerRetryable,
        RegisteredToolBounds::ceiling(),
    )
    .expect("declaration is valid")
}

pub(crate) fn snapshot(
    declarations: Vec<RegisteredToolDeclaration>,
) -> Arc<RegisteredToolSnapshot> {
    Arc::new(
        RegisteredToolSnapshot::new(RegisteredToolSnapshotInput {
            server_id: RegisteredServerId::new("desktop.tools").expect("server id is valid"),
            revision: RegisteredServerRevision::new("rev-1").expect("revision is valid"),
            execution_host_id: ExecutionHostId::new("host.local").expect("host id is valid"),
            declarations,
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

pub(crate) fn protocol_version() -> RegisteredToolProtocolVersion {
    RegisteredToolProtocolVersion::new(REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION)
        .expect("conformance protocol version is valid")
}

pub(crate) fn admission() -> ConsumerAdmissionBinding {
    ConsumerAdmissionBinding::new(
        ConsumerProcessIncarnation::new("incarnation-1").expect("incarnation is valid"),
        ConsumerWorkspaceGeneration::initial(),
        ConsumerTaskGeneration::initial(),
        AdmittedTaskId::new("task-1").expect("task id is valid"),
        AdmittedSessionId::new("session-1").expect("session id is valid"),
        AdmittedAttemptId::new("attempt-1").expect("attempt id is valid"),
        Arc::new(CurrentAdmission),
    )
}

fn preparation(
    snapshot: Arc<RegisteredToolSnapshot>,
    selected: Vec<RegisteredToolId>,
    transport: RegisteredToolTransport,
) -> RegisteredToolPreparation {
    let selection = RegisteredToolSelection::new(
        Arc::clone(&snapshot),
        selected,
        transport,
        protocol_version(),
    )
    .expect("selection is valid");
    RegisteredToolPreparation::new(
        snapshot,
        selection,
        admission(),
        RegisteredToolLimits::ceiling(),
    )
}

fn native_preparation() -> RegisteredToolPreparation {
    let id = tool_id(NAMESPACE, "task_ledger");
    let snapshot = snapshot(vec![declaration(
        id.clone(),
        RegisteredToolExecutionKind::NativeClient,
    )]);
    preparation(
        snapshot,
        vec![id],
        RegisteredToolTransport::HostMediatedCallback,
    )
}

fn code(error: &RuntimeFailure) -> String {
    error.diagnostic().code().to_owned()
}

#[test]
fn a_native_tool_declares_one_flat_namespaced_dynamic_tool() {
    let binding =
        CodexRegisteredToolBinding::qualify(native_preparation()).expect("native tools qualify");

    assert_eq!(binding.declarations().len(), 1);
    assert_eq!(binding.declarations()[0].name(), "desktop__task_ledger");
    assert_eq!(
        binding.tool_for("desktop__task_ledger"),
        Some(&tool_id(NAMESPACE, "task_ledger"))
    );
    assert_eq!(binding.tool_for("task_ledger"), None);
}

#[test]
fn provider_direct_mcp_registration_stays_withheld() {
    let id = tool_id(NAMESPACE, "reconcile");
    let snapshot = snapshot(vec![declaration(
        id.clone(),
        RegisteredToolExecutionKind::Mcp,
    )]);
    let error = CodexRegisteredToolBinding::qualify(preparation(
        snapshot,
        vec![id],
        RegisteredToolTransport::HostMediatedCallback,
    ))
    .expect_err("registered MCP tools stay withheld on this route");

    assert_eq!(
        code(&error),
        "swallowtail.codex.app_server.registered_mcp_withheld"
    );
}

#[test]
fn app_and_provider_owned_tools_are_not_this_route() {
    for kind in [
        RegisteredToolExecutionKind::App,
        RegisteredToolExecutionKind::ProviderOwned,
    ] {
        let id = tool_id(NAMESPACE, "observe");
        let snapshot = snapshot(vec![declaration(id.clone(), kind)]);
        let error = CodexRegisteredToolBinding::qualify(preparation(
            snapshot,
            vec![id],
            RegisteredToolTransport::HostMediatedCallback,
        ))
        .expect_err("only native client tools bind here");

        assert_eq!(
            code(&error),
            "swallowtail.codex.app_server.registered_kind_unsupported"
        );
    }
}

#[test]
fn unqualified_carriers_fail_before_any_provider_work() {
    let id = tool_id(NAMESPACE, "task_ledger");
    let declarations = vec![declaration(
        id.clone(),
        RegisteredToolExecutionKind::NativeClient,
    )];
    let snapshot = Arc::new(
        RegisteredToolSnapshot::new(RegisteredToolSnapshotInput {
            server_id: RegisteredServerId::new("desktop.tools").expect("server id is valid"),
            revision: RegisteredServerRevision::new("rev-1").expect("revision is valid"),
            execution_host_id: ExecutionHostId::new("host.local").expect("host id is valid"),
            declarations,
            transports: vec![
                RegisteredToolTransportSupport::new(
                    RegisteredToolTransport::PrivateLoopbackHttp,
                    [protocol_version()],
                )
                .expect("transport support is valid"),
            ],
            required_services: BTreeSet::new(),
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
    );
    let error = CodexRegisteredToolBinding::qualify(preparation(
        snapshot,
        vec![id],
        RegisteredToolTransport::PrivateLoopbackHttp,
    ))
    .expect_err("only the host-mediated carrier is qualified here");

    assert_eq!(
        code(&error),
        "swallowtail.codex.app_server.registered_transport_unsupported"
    );
}

#[test]
fn a_separator_inside_an_identity_cannot_become_an_ambiguous_wire_name() {
    let id = tool_id("desktop__tasks", "ledger");
    let snapshot = snapshot(vec![declaration(
        id.clone(),
        RegisteredToolExecutionKind::NativeClient,
    )]);
    let error = CodexRegisteredToolBinding::qualify(preparation(
        snapshot,
        vec![id],
        RegisteredToolTransport::HostMediatedCallback,
    ))
    .expect_err("an ambiguous namespaced identity is refused");

    assert_eq!(
        code(&error),
        "swallowtail.codex.app_server.registered_identity_unsupported"
    );
}

#[test]
fn a_non_json_schema_input_is_refused_before_declaration() {
    let id = tool_id(NAMESPACE, "task_ledger");
    let snapshot = snapshot(vec![declaration_with(
        id.clone(),
        RegisteredToolExecutionKind::NativeClient,
        "application/xml",
        SCHEMA,
    )]);
    let error = CodexRegisteredToolBinding::qualify(preparation(
        snapshot,
        vec![id],
        RegisteredToolTransport::HostMediatedCallback,
    ))
    .expect_err("Codex dynamic tools carry JSON Schema input only");

    assert_eq!(
        code(&error),
        "swallowtail.codex.app_server.registered_schema_unsupported"
    );
}

#[test]
fn the_published_route_claims_only_what_this_seam_proves() {
    assert_eq!(
        CODEX_REGISTERED_TOOL_ROUTE.progress(),
        RegisteredToolProgressMode::NoProgress
    );
    assert_eq!(
        CODEX_REGISTERED_TOOL_ROUTE.skill_delivery(),
        RegisteredToolSkillDelivery::BoundedSelectedBundle
    );
}
