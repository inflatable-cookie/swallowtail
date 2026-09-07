//! Registration, selection, and preflight counterexamples.

use super::support::{RegisteredToolHarness, conformance_host_id};
use crate::registered_tool_fixture::{
    FIXTURE_MCP_TOOL, FIXTURE_NATIVE_TOOL, fixture_declaration, fixture_protocol_version,
    fixture_schema, fixture_selection, fixture_snapshot, fixture_snapshot_input, fixture_tool_id,
};
use std::sync::Arc;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    HostServices, MAX_REGISTERED_TOOL_SCHEMA_BYTES, RegisteredToolExecutionKind,
    RegisteredToolFailureKind, RegisteredToolPortAvailability, RegisteredToolProtocolVersion,
    RegisteredToolReadiness, RegisteredToolSchemaDocument, RegisteredToolSelection,
    RegisteredToolSnapshot, RegisteredToolTransport,
};

pub(super) fn snapshot_binds_one_kind_per_identity() {
    let host = conformance_host_id();
    let mut input = fixture_snapshot_input(&host);
    input.declarations.push(fixture_declaration(
        FIXTURE_NATIVE_TOOL,
        RegisteredToolExecutionKind::Mcp,
    ));

    let error = RegisteredToolSnapshot::new(input).expect_err("kind substitution must fail");

    assert_eq!(error.kind(), RegisteredToolFailureKind::IdentityRejected);
}

pub(super) fn snapshot_rejects_oversized_schema() {
    let oversized = "x".repeat(MAX_REGISTERED_TOOL_SCHEMA_BYTES + 1);

    let error = RegisteredToolSchemaDocument::new(oversized).expect_err("oversized schema fails");

    assert_eq!(error.kind(), RegisteredToolFailureKind::LimitExceeded);
}

pub(super) fn snapshot_requires_a_declared_transport() {
    let host = conformance_host_id();
    let mut input = fixture_snapshot_input(&host);
    input.transports.clear();

    let error = RegisteredToolSnapshot::new(input).expect_err("no transport must fail");

    assert_eq!(
        error.kind(),
        RegisteredToolFailureKind::UnsupportedRegistration
    );
}

pub(super) fn selection_rejects_unknown_and_duplicate_identities() {
    let snapshot = Arc::new(fixture_snapshot(&conformance_host_id()));

    let unknown = RegisteredToolSelection::new(
        Arc::clone(&snapshot),
        [fixture_tool_id("absent")],
        RegisteredToolTransport::HostMediatedCallback,
        fixture_protocol_version(),
    )
    .expect_err("unknown tool must fail");
    let duplicate = RegisteredToolSelection::new(
        Arc::clone(&snapshot),
        [
            fixture_tool_id(FIXTURE_NATIVE_TOOL),
            fixture_tool_id(FIXTURE_NATIVE_TOOL),
        ],
        RegisteredToolTransport::HostMediatedCallback,
        fixture_protocol_version(),
    )
    .expect_err("duplicate selection must fail");

    assert_eq!(unknown.kind(), RegisteredToolFailureKind::UnsupportedTool);
    assert_eq!(
        duplicate.kind(),
        RegisteredToolFailureKind::IdentityRejected
    );
}

pub(super) fn selection_rejects_undeclared_transport_and_version() {
    let snapshot = Arc::new(fixture_snapshot(&conformance_host_id()));

    let transport = RegisteredToolSelection::new(
        Arc::clone(&snapshot),
        [fixture_tool_id(FIXTURE_MCP_TOOL)],
        RegisteredToolTransport::PrivateLoopbackSse,
        fixture_protocol_version(),
    )
    .expect_err("undeclared transport must fail");
    let version = RegisteredToolSelection::new(
        Arc::clone(&snapshot),
        [fixture_tool_id(FIXTURE_MCP_TOOL)],
        RegisteredToolTransport::HostMediatedCallback,
        RegisteredToolProtocolVersion::new("latest").expect("version text"),
    )
    .expect_err("substituted version must fail");

    assert_eq!(
        transport.kind(),
        RegisteredToolFailureKind::UnsupportedTransport
    );
    assert_eq!(
        version.kind(),
        RegisteredToolFailureKind::UnsupportedProtocolVersion
    );
}

pub(super) fn absent_port_is_typed_unavailable() {
    let hosts = HostServices::new(conformance_host_id());
    let snapshot = Arc::new(fixture_snapshot(&conformance_host_id()));
    let selection = fixture_selection(snapshot);

    let readiness = RegisteredToolReadiness::evaluate(&hosts, &selection);
    let error = readiness
        .require_ready()
        .expect_err("an absent port is not ready");

    assert_eq!(readiness.port(), RegisteredToolPortAvailability::Absent);
    assert!(!readiness.is_ready());
    assert_eq!(error.kind(), RegisteredToolFailureKind::MissingHostService);
    assert!(
        !hosts
            .available_kinds()
            .iter()
            .any(|kind| format!("{kind:?}").contains("Registered")),
        "HostServiceKind stays exhaustive and gains no registered-tool variant"
    );
}

pub(super) fn foreign_execution_host_fails_before_open(harness: &RegisteredToolHarness) {
    let foreign = ExecutionHostId::new("fixture.host.remote").expect("host id is valid");
    let snapshot = Arc::new(fixture_snapshot(&foreign));
    let selection = fixture_selection(snapshot);

    let readiness = RegisteredToolReadiness::evaluate(&harness.hosts, &selection);

    assert!(!readiness.execution_host_matches());
    assert_eq!(
        readiness
            .require_ready()
            .expect_err("foreign host is not ready")
            .kind(),
        RegisteredToolFailureKind::UnsupportedRegistration
    );
}

pub(super) fn ready_registry_reports_every_dimension(harness: &RegisteredToolHarness) {
    let readiness = RegisteredToolReadiness::evaluate(&harness.hosts, &harness.selection);

    assert_eq!(readiness.port(), RegisteredToolPortAvailability::Registered);
    assert!(readiness.execution_host_matches());
    assert!(readiness.missing_services().is_empty());
    assert!(readiness.transport_qualified());
    assert!(readiness.protocol_qualified());
    assert!(readiness.is_ready());
    readiness.require_ready().expect("ready registry passes");
}

pub(super) fn schema_and_snapshot_hold_no_runtime_resource() {
    let schema = fixture_schema("1", "sha256:input", "{\"type\":\"object\"}");

    assert_eq!(
        schema.document().expose_for_execution(),
        "{\"type\":\"object\"}"
    );
    assert!(!format!("{schema:?}").contains("\"type\""));
    assert!(!format!("{}", schema.document()).contains("\"type\""));
}
