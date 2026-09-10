//! Provider-free Contract 063 mediation evidence for the `claude-agent.sdk`
//! route.
//!
//! The frozen transcript in
//! `tests/fixtures/claude-agent-sdk-v1/registered-tool-mcp.jsonl` is the oracle:
//! every case names its setup, its recorded `canUseTool` decisions, the exact
//! record the provider sends, and the exact reply the carrier must produce.
//! Each case runs against a real mounted local host composition and a real
//! Contract 063 lease, so admission, correlation, deadlines, settlement, and
//! the honest execution disposition come from the kernel rather than a stub.
//!
//! No provider process, credential, network call, or live MCP server is used.

use crate::sdk_support;
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use swallowtail_adapter_claude_agent::sdk::registered_tool::{
    CLAUDE_AGENT_SDK_MCP_PROTOCOL_VERSION, CLAUDE_AGENT_SDK_MCP_SUPPORTED_PROTOCOL_VERSIONS,
    CLAUDE_AGENT_SDK_MEDIATION_KIND_SEMANTIC_ID, CLAUDE_AGENT_SDK_REGISTERED_TOOL_CARRIER_AXIS,
    CLAUDE_AGENT_SDK_REGISTERED_TOOL_CARRIER_REVISION, CLAUDE_AGENT_SDK_REGISTERED_TOOL_MEDIATION,
    CLAUDE_AGENT_SDK_REGISTERED_TOOL_NATIVE_VERSION, CLAUDE_AGENT_SDK_REGISTERED_TOOL_SDK_VERSION,
    CLAUDE_AGENT_SDK_REGISTERED_TOOL_SERVER, ClaudeAgentSdkMcpReply,
    ClaudeAgentSdkRegisteredToolBinding, ClaudeAgentSdkRegisteredToolCarrier,
    ClaudeAgentSdkRegisteredToolDecision, ClaudeAgentSdkRegisteredToolMediator,
    claude_agent_sdk_mcp_protocol_version_admitted, claude_agent_sdk_mcp_protocol_version_known,
    claude_agent_sdk_registered_tool_carrier_binding,
    claude_agent_sdk_registered_tool_carrier_claim, claude_agent_sdk_registered_tool_qualification,
    project_claude_agent_sdk_registered_tool,
};
use swallowtail_core::{ConfiguredInstanceId, ExecutionHostId};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    ConsumerAdmissionBinding, Deadline, HostServices, MonotonicInstant, RegisteredServerId,
    RegisteredServerRevision, RegisteredToolBounds, RegisteredToolBridgeLease,
    RegisteredToolDeclaration, RegisteredToolEffectPosture, RegisteredToolExecutionDisposition,
    RegisteredToolExecutionKind, RegisteredToolFailure, RegisteredToolFailureKind,
    RegisteredToolId, RegisteredToolLimits, RegisteredToolLocalName, RegisteredToolNamespace,
    RegisteredToolOutcome, RegisteredToolPayload, RegisteredToolPreparation,
    RegisteredToolProtocolVersion, RegisteredToolReadiness, RegisteredToolResult,
    RegisteredToolRetryPosture, RegisteredToolSchema, RegisteredToolSchemaDialect,
    RegisteredToolSchemaDigest, RegisteredToolSchemaDocument, RegisteredToolSchemaMediaType,
    RegisteredToolSchemaNamespace, RegisteredToolSelection, RegisteredToolSnapshot,
    RegisteredToolSnapshotInput, RegisteredToolSource, RegisteredToolSourceId,
    RegisteredToolTransport, RegisteredToolTransportSupport, RuntimeTurnId, ScopeId,
};
use swallowtail_testkit::{
    FakeClock, ScriptedAdmissionPort, ScriptedRegisteredToolDispatcher, drive_fixture,
    fixture_admission,
};

const TRANSCRIPT: &str = include_str!("../fixtures/claude-agent-sdk-v1/registered-tool-mcp.jsonl");

const RECONCILE: &str = "reconcile";
const READ_NOTE: &str = "read-note";
const NAMESPACE: &str = "desktop";

fn host_id() -> ExecutionHostId {
    ExecutionHostId::new("fixture.host.local").expect("host id is valid")
}

fn tool_id(local_name: &str) -> RegisteredToolId {
    RegisteredToolId::new(
        RegisteredToolNamespace::new(NAMESPACE).expect("namespace is valid"),
        RegisteredToolLocalName::new(local_name).expect("local name is valid"),
    )
}

fn schema(digest: &str) -> RegisteredToolSchema {
    RegisteredToolSchema::new(
        RegisteredToolSchemaNamespace::new("desktop.registered-tools.schema")
            .expect("schema namespace is valid"),
        RegisteredToolSchemaMediaType::new("application/json").expect("media type is valid"),
        RegisteredToolSchemaDialect::new("json-schema-2020-12").expect("dialect is valid"),
        RegisteredServerRevision::new("1").expect("schema revision is valid"),
        RegisteredToolSchemaDigest::new(digest).expect("digest is valid"),
        RegisteredToolSchemaDocument::new("{\"type\":\"object\"}").expect("document is valid"),
    )
}

fn declaration(
    local_name: &str,
    kind: RegisteredToolExecutionKind,
    effect: RegisteredToolEffectPosture,
) -> RegisteredToolDeclaration {
    RegisteredToolDeclaration::new(
        tool_id(local_name),
        kind,
        schema("sha256:input"),
        schema("sha256:output"),
        effect,
        RegisteredToolRetryPosture::ConsumerRetryable,
        RegisteredToolBounds::ceiling(),
    )
    .expect("declaration is valid")
}

fn protocol_version() -> RegisteredToolProtocolVersion {
    RegisteredToolProtocolVersion::new(
        swallowtail_runtime::REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION,
    )
    .expect("conformance protocol version is valid")
}

fn snapshot_input(declarations: Vec<RegisteredToolDeclaration>) -> RegisteredToolSnapshotInput {
    RegisteredToolSnapshotInput {
        server_id: RegisteredServerId::new("desktop.registered-tools").expect("server id is valid"),
        revision: RegisteredServerRevision::new("2026-09-07.1").expect("revision is valid"),
        execution_host_id: host_id(),
        declarations,
        transports: vec![
            RegisteredToolTransportSupport::new(
                RegisteredToolTransport::HostMediatedCallback,
                [protocol_version()],
            )
            .expect("callback transport is valid"),
        ],
        required_services: [].into_iter().collect(),
        credential_references: Vec::new(),
        executable_recipes: Vec::new(),
        environment_recipes: Vec::new(),
        bounds: RegisteredToolBounds::ceiling(),
        source: RegisteredToolSource::new(
            RegisteredToolSourceId::new("desktop.registration.1").expect("source id is valid"),
            MonotonicInstant::from_ticks(1),
        ),
    }
}

fn route_selection() -> RegisteredToolSelection {
    let snapshot = Arc::new(
        RegisteredToolSnapshot::new(snapshot_input(vec![
            declaration(
                RECONCILE,
                RegisteredToolExecutionKind::Mcp,
                RegisteredToolEffectPosture::Mutating,
            ),
            declaration(
                READ_NOTE,
                RegisteredToolExecutionKind::Mcp,
                RegisteredToolEffectPosture::ReadOnly,
            ),
        ]))
        .expect("route snapshot is valid"),
    );
    RegisteredToolSelection::new(
        snapshot,
        [tool_id(RECONCILE), tool_id(READ_NOTE)],
        RegisteredToolTransport::HostMediatedCallback,
        protocol_version(),
    )
    .expect("route selection is valid")
}

/// Linked dispatcher behaviours the transcript names.
fn dispatcher(kind: &str) -> Arc<ScriptedRegisteredToolDispatcher> {
    match kind {
        "server-error" => Arc::new(ScriptedRegisteredToolDispatcher::new(Arc::new(
            |call, _context| {
                Ok(RegisteredToolOutcome::failed(
                    call,
                    RegisteredToolFailure::new(RegisteredToolFailureKind::ServerExecutionFailed),
                    RegisteredToolExecutionDisposition::Executed,
                ))
            },
        ))),
        "unknown-outcome" => Arc::new(ScriptedRegisteredToolDispatcher::new(Arc::new(
            |call, _context| {
                Ok(RegisteredToolOutcome::failed(
                    call,
                    RegisteredToolFailure::new(RegisteredToolFailureKind::TransportLost),
                    RegisteredToolExecutionDisposition::Unknown,
                ))
            },
        ))),
        _ => Arc::new(ScriptedRegisteredToolDispatcher::new(Arc::new(
            |call, _context| {
                Ok(RegisteredToolOutcome::completed(
                    call,
                    RegisteredToolResult::new(
                        RegisteredToolPayload::new(
                            RegisteredToolSchemaMediaType::new("application/json")
                                .expect("media type is valid"),
                            b"{\"ok\":true}".to_vec(),
                            call.binding().effective_bounds().max_result_bytes(),
                        )
                        .expect("bounded result payload"),
                        RegisteredToolSchemaDigest::new("sha256:output").expect("digest is valid"),
                    ),
                ))
            },
        ))),
    }
}

fn mounted_hosts(dispatcher: Arc<ScriptedRegisteredToolDispatcher>) -> HostServices {
    LocalProcessHost::builder(LocalProcessLimits::default())
        .with_registered_tool_dispatcher(dispatcher)
        .with_registered_tool_clock(Arc::new(FakeClock::default()))
        .build_services(host_id())
        .services()
        .clone()
}

struct Harness {
    mediator: ClaudeAgentSdkRegisteredToolMediator,
    dispatcher: Arc<ScriptedRegisteredToolDispatcher>,
}

fn open_lease(
    hosts: &HostServices,
    selection: &RegisteredToolSelection,
    admission: ConsumerAdmissionBinding,
    limits: RegisteredToolLimits,
) -> RegisteredToolBridgeLease {
    let preparation = RegisteredToolPreparation::new(
        Arc::clone(selection.snapshot()),
        selection.clone(),
        admission,
        limits,
    );
    let prepared = preparation
        .prepare(
            hosts,
            ConfiguredInstanceId::new("fixture.instance.claude-agent-sdk")
                .expect("instance id is valid"),
            ScopeId::new("fixture.claude-agent-sdk.scope").expect("scope id is valid"),
            RuntimeTurnId::new("turn-registered-tool").expect("turn id is valid"),
            Deadline::at(MonotonicInstant::from_ticks(1_000)),
        )
        .expect("prepare succeeds on the mounted registry");
    drive_fixture(prepared.open()).expect("open succeeds on a prepared binding")
}

fn harness(setup: &str, dispatcher_kind: &str) -> Harness {
    let selection = route_selection();
    let dispatcher = dispatcher(dispatcher_kind);
    let hosts = mounted_hosts(Arc::clone(&dispatcher));
    let port = Arc::new(ScriptedAdmissionPort::current());
    if setup == "revoked" {
        port.revoke_from(swallowtail_runtime::AdmissionPhase::BeforeDispatch);
    }
    let limits = if setup == "narrow-limits" {
        RegisteredToolLimits::new(
            RegisteredToolBounds::new(1, 32, 1024, 1024, 8, std::time::Duration::from_secs(60))
                .expect("narrowed bounds are positive"),
        )
    } else {
        RegisteredToolLimits::ceiling()
    };
    let lease = open_lease(&hosts, &selection, fixture_admission(port), limits);
    let carrier =
        ClaudeAgentSdkRegisteredToolCarrier::new(&selection).expect("carrier admits the selection");
    let mediator = ClaudeAgentSdkRegisteredToolMediator::new(carrier, lease)
        .expect("mediator binds its own lease")
        .with_consumer_limits(limits);
    if setup != "fresh" {
        mediate(
            &mediator,
            &json!({
                "jsonrpc": "2.0",
                "id": 0,
                "method": "initialize",
                "params": {"protocolVersion": CLAUDE_AGENT_SDK_MCP_PROTOCOL_VERSION},
            }),
        )
        .expect("handshake is admissible");
        mediate(
            &mediator,
            &json!({"jsonrpc": "2.0", "method": "notifications/initialized"}),
        )
        .expect("initialized notification is admissible");
    }
    Harness {
        mediator,
        dispatcher,
    }
}

fn mediate(
    mediator: &ClaudeAgentSdkRegisteredToolMediator,
    record: &Value,
) -> Result<ClaudeAgentSdkMcpReply, swallowtail_runtime::RuntimeFailure> {
    let encoded = serde_json::to_vec(record).expect("record encodes");
    futures_executor::block_on(mediator.mediate(&encoded))
}

#[test]
fn the_frozen_mcp_transcript_is_reproduced_exactly() {
    let mut cases = 0_usize;
    for line in TRANSCRIPT.lines().filter(|line| !line.trim().is_empty()) {
        let entry: Value = serde_json::from_str(line).expect("transcript line is JSON");
        let case = entry["case"].as_str().expect("case name");
        let harness = harness(
            entry["setup"].as_str().expect("setup name"),
            entry["dispatcher"].as_str().expect("dispatcher name"),
        );
        for admission in entry["admissions"].as_array().expect("admissions array") {
            let pair = admission.as_array().expect("admission pair");
            let decision = match pair[1].as_str().expect("decision") {
                "allow" => ClaudeAgentSdkRegisteredToolDecision::Allow,
                _ => ClaudeAgentSdkRegisteredToolDecision::Deny,
            };
            harness
                .mediator
                .record_admission(pair[0].as_str().expect("tool name"), decision)
                .unwrap_or_else(|_| panic!("{case}: admission is recordable"));
        }
        for record in entry["prelude"].as_array().expect("prelude array") {
            mediate(&harness.mediator, record)
                .unwrap_or_else(|_| panic!("{case}: prelude record is admissible"));
        }
        let expect = entry["expect"].as_str().expect("expectation");
        if expect == "decode-failure" {
            let outcome = match entry.get("raw").and_then(Value::as_str) {
                Some(raw) => futures_executor::block_on(harness.mediator.mediate(raw.as_bytes())),
                None => mediate(&harness.mediator, &entry["request"]),
            };
            let failure = outcome
                .err()
                .unwrap_or_else(|| panic!("{case}: an undecodable record must fail closed"));
            assert_eq!(
                failure.diagnostic().code(),
                "swallowtail.claude-agent.sdk.registered_tool.record_invalid",
                "{case}: decode failures carry the carrier's own safe code"
            );
        } else {
            let reply = mediate(&harness.mediator, &entry["request"])
                .unwrap_or_else(|_| panic!("{case}: the record decodes"));
            match expect {
                "accepted" => assert_eq!(
                    reply,
                    ClaudeAgentSdkMcpReply::Accepted,
                    "{case}: a notification writes nothing back"
                ),
                _ => {
                    let encoded = reply
                        .response_bytes()
                        .unwrap_or_else(|| panic!("{case}: a request is answered"));
                    let decoded: Value =
                        serde_json::from_slice(encoded).expect("the reply is valid JSON");
                    assert_eq!(
                        decoded, entry["response"],
                        "{case}: the reply matches the frozen transcript"
                    );
                }
            }
        }
        if let Some(expected) = entry.get("dispatches").and_then(Value::as_u64) {
            assert_eq!(
                harness.dispatcher.dispatches(),
                usize::try_from(expected).expect("dispatch count fits"),
                "{case}: the linked dispatcher is reached exactly this often"
            );
        }
        cases += 1;
    }
    assert!(cases >= 20, "the frozen transcript covers every gate case");
}

#[test]
fn the_carrier_projection_is_reversible_and_reserved() {
    let selection = route_selection();
    let carrier = ClaudeAgentSdkRegisteredToolCarrier::new(&selection).expect("carrier is valid");

    assert_eq!(
        carrier.server_name(),
        CLAUDE_AGENT_SDK_REGISTERED_TOOL_SERVER
    );
    assert_eq!(
        carrier.mediation_kind(),
        CLAUDE_AGENT_SDK_REGISTERED_TOOL_MEDIATION
    );
    assert_eq!(
        carrier.admitted_tool_names(),
        [
            "mcp__swallowtail-registered-tools__desktop_reconcile",
            "mcp__swallowtail-registered-tools__desktop_read-note",
        ]
    );
    for tool in carrier.tools() {
        assert_eq!(
            carrier
                .resolve_provider_tool(tool.provider_tool_name())
                .map(|resolved| resolved.id()),
            Some(tool.id()),
            "one wire name resolves to exactly one namespaced identity"
        );
    }
    assert!(carrier.resolve_carrier_tool("desktop_absent").is_none());
}

#[test]
fn a_non_mcp_kind_cannot_reach_the_provider_through_this_carrier() {
    let snapshot = Arc::new(
        RegisteredToolSnapshot::new(snapshot_input(vec![declaration(
            RECONCILE,
            RegisteredToolExecutionKind::NativeClient,
            RegisteredToolEffectPosture::Mutating,
        )]))
        .expect("snapshot is valid"),
    );
    let selection = RegisteredToolSelection::new(
        snapshot,
        [tool_id(RECONCILE)],
        RegisteredToolTransport::HostMediatedCallback,
        protocol_version(),
    )
    .expect("selection is valid");

    let failure = ClaudeAgentSdkRegisteredToolCarrier::new(&selection)
        .expect_err("a native-client identity is not an MCP tool");

    assert_eq!(
        failure.diagnostic().safe().code(),
        "swallowtail.claude-agent.sdk.registered_tool.kind_unsupported"
    );
}

#[test]
fn an_identity_the_wire_cannot_spell_unambiguously_is_refused() {
    for namespace in ["desk_top", "desk.top"] {
        let id = RegisteredToolId::new(
            RegisteredToolNamespace::new(namespace).expect("namespace is admissible"),
            RegisteredToolLocalName::new(RECONCILE).expect("local name is admissible"),
        );
        let snapshot = Arc::new(
            RegisteredToolSnapshot::new(RegisteredToolSnapshotInput {
                declarations: vec![
                    RegisteredToolDeclaration::new(
                        id.clone(),
                        RegisteredToolExecutionKind::Mcp,
                        schema("sha256:input"),
                        schema("sha256:output"),
                        RegisteredToolEffectPosture::ReadOnly,
                        RegisteredToolRetryPosture::ConsumerRetryable,
                        RegisteredToolBounds::ceiling(),
                    )
                    .expect("declaration is valid"),
                ],
                ..snapshot_input(Vec::new())
            })
            .expect("snapshot is valid"),
        );
        let selection = RegisteredToolSelection::new(
            snapshot,
            [id],
            RegisteredToolTransport::HostMediatedCallback,
            protocol_version(),
        )
        .expect("selection is valid");

        let failure = ClaudeAgentSdkRegisteredToolCarrier::new(&selection)
            .expect_err("an ambiguous namespace cannot be spelled on the wire");

        assert_eq!(
            failure.diagnostic().safe().code(),
            "swallowtail.claude-agent.sdk.registered_tool.identity_unprojectable"
        );
    }
}

#[test]
fn a_mediator_refuses_a_lease_opened_for_another_selection() {
    let selection = route_selection();
    let other = RegisteredToolSelection::new(
        Arc::clone(selection.snapshot()),
        [tool_id(RECONCILE)],
        RegisteredToolTransport::HostMediatedCallback,
        protocol_version(),
    )
    .expect("a narrower selection is valid");
    let hosts = mounted_hosts(dispatcher("ok"));
    let lease = open_lease(
        &hosts,
        &other,
        fixture_admission(Arc::new(ScriptedAdmissionPort::current())),
        RegisteredToolLimits::ceiling(),
    );
    let carrier = ClaudeAgentSdkRegisteredToolCarrier::new(&selection).expect("carrier is valid");

    let failure = ClaudeAgentSdkRegisteredToolMediator::new(carrier, lease)
        .err()
        .expect("a foreign lease is refused");

    assert_eq!(
        failure.diagnostic().code(),
        RegisteredToolFailureKind::ForeignCorrelation.code()
    );
}

#[test]
fn an_admission_for_a_tool_this_carrier_never_presents_is_refused() {
    let harness = harness("handshaken", "ok");

    let failure = harness
        .mediator
        .record_admission(
            "mcp__swallowtail-watchers__start",
            ClaudeAgentSdkRegisteredToolDecision::Allow,
        )
        .expect_err("a foreign tool name is refused");

    assert_eq!(
        failure.diagnostic().code(),
        RegisteredToolFailureKind::UnsupportedTool.code()
    );
}

#[test]
fn the_carrier_version_axis_is_qualified_only_at_one_exact_point() {
    let claim = claude_agent_sdk_registered_tool_carrier_claim();

    assert_eq!(
        claim.axis().as_str(),
        CLAUDE_AGENT_SDK_REGISTERED_TOOL_CARRIER_AXIS
    );
    assert!(
        claude_agent_sdk_registered_tool_carrier_binding(
            CLAUDE_AGENT_SDK_REGISTERED_TOOL_CARRIER_REVISION
        )
        .is_some()
    );
    assert!(claude_agent_sdk_registered_tool_carrier_binding("some-other-revision").is_none());
    assert_eq!(
        CLAUDE_AGENT_SDK_REGISTERED_TOOL_SDK_VERSION,
        swallowtail_adapter_claude_agent::sdk::CLAUDE_AGENT_SDK_VERSION
    );
    assert_eq!(
        CLAUDE_AGENT_SDK_REGISTERED_TOOL_NATIVE_VERSION,
        swallowtail_adapter_claude_agent::sdk::CLAUDE_AGENT_SDK_NATIVE_VERSION
    );
}

#[test]
fn the_carrier_answers_one_mcp_protocol_version_and_knows_the_pinned_set() {
    assert!(claude_agent_sdk_mcp_protocol_version_admitted(
        CLAUDE_AGENT_SDK_MCP_PROTOCOL_VERSION
    ));
    assert!(!claude_agent_sdk_mcp_protocol_version_admitted(
        "2024-11-05"
    ));
    assert!(claude_agent_sdk_mcp_protocol_version_known("2024-11-05"));
    assert!(!claude_agent_sdk_mcp_protocol_version_known("1999-01-01"));
    assert_eq!(
        CLAUDE_AGENT_SDK_MCP_SUPPORTED_PROTOCOL_VERSIONS[0],
        CLAUDE_AGENT_SDK_MCP_PROTOCOL_VERSION
    );
}

fn row_with_semantic_id<'a>(
    contribution: &'a swallowtail_runtime::ConsumerRouteProjectionContribution,
    wanted: &str,
) -> &'a swallowtail_runtime::ConsumerRouteProjectionRow {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .find(|row| {
            row.identity()
                .namespaced_extension()
                .is_some_and(|extension| extension.semantic_id() == wanted)
        })
        .unwrap_or_else(|| panic!("missing projected row {wanted}"))
}

#[test]
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn the_projection_publishes_the_qualified_route_on_the_accepted_platform() {
    let selection = route_selection();
    let carrier = ClaudeAgentSdkRegisteredToolCarrier::new(&selection).expect("carrier is valid");
    let hosts = mounted_hosts(dispatcher("ok"));
    let readiness = RegisteredToolReadiness::evaluate(&hosts, &selection);
    let prepared = sdk_support::prepared_session(host_id());
    let applicability = swallowtail_runtime::ConsumerRouteApplicability::from_plan(prepared.plan());

    let contribution =
        project_claude_agent_sdk_registered_tool(&applicability, &carrier, &readiness)
            .expect("the contribution composes");

    let route = swallowtail_adapter_claude_agent::sdk::registered_tool::
        CLAUDE_AGENT_SDK_REGISTERED_TOOL_ROUTE;
    assert_eq!(
        claude_agent_sdk_registered_tool_qualification(),
        swallowtail_runtime::RegisteredToolRouteQualification::Qualified(route),
        "the accepted live tuple qualifies the route on its own platform"
    );
    assert_eq!(
        route.permission(),
        swallowtail_runtime::RegisteredToolPermissionStrength::ExactOneShot
    );
    assert_eq!(
        route.progress(),
        swallowtail_runtime::RegisteredToolProgressMode::NoProgress
    );
    assert_eq!(
        route.skill_delivery(),
        swallowtail_runtime::RegisteredToolSkillDelivery::NotCarried
    );
    let capability = row_with_semantic_id(&contribution, "registered-tool.capability");
    assert_eq!(
        capability.support(),
        swallowtail_runtime::ConsumerRouteSupportPosture::Supported
    );
    assert_eq!(
        capability.availability(),
        swallowtail_runtime::ConsumerRouteAvailability::Available
    );
    let mediation =
        row_with_semantic_id(&contribution, CLAUDE_AGENT_SDK_MEDIATION_KIND_SEMANTIC_ID);
    assert_eq!(
        mediation.support(),
        swallowtail_runtime::ConsumerRouteSupportPosture::Supported
    );
    assert_eq!(
        mediation.availability(),
        swallowtail_runtime::ConsumerRouteAvailability::Available
    );
    assert_eq!(
        mediation.evidence_strength(),
        swallowtail_runtime::ConsumerRouteEvidenceStrength::RouteValidation
    );
    assert!(
        mediation.safe_reason().is_none(),
        "a route-validation row carries no unavailable reason"
    );
    let permission = row_with_semantic_id(&contribution, "registered-tool.one-shot-permission");
    assert_eq!(
        permission.support(),
        swallowtail_runtime::ConsumerRouteSupportPosture::Supported
    );
    assert_eq!(
        permission.availability(),
        swallowtail_runtime::ConsumerRouteAvailability::Available
    );
    for dimension in [
        "registered-tool.progress-delivery",
        "registered-tool.selected-skill-bundle",
    ] {
        let row = row_with_semantic_id(&contribution, dimension);
        assert_eq!(
            row.support(),
            swallowtail_runtime::ConsumerRouteSupportPosture::Unsupported,
            "{dimension} stays exactly what the capsule proved"
        );
        assert_eq!(
            row.availability(),
            swallowtail_runtime::ConsumerRouteAvailability::Unavailable
        );
        assert_eq!(
            row.safe_reason()
                .expect("dimension reason")
                .diagnostic()
                .code(),
            "swallowtail.registered_tool.route_dimension_unsupported"
        );
    }
    let skill = row_with_semantic_id(&contribution, "registered-tool.selected-skill-bundle");
    assert_eq!(
        skill.actor_posture(),
        swallowtail_runtime::ConsumerRouteActorPosture::Informational
    );
    assert!(skill.mutation_authority().source().is_none());
    let scheduling = row_with_semantic_id(&contribution, "registered-tool.scheduling");
    assert_eq!(
        scheduling.support(),
        swallowtail_runtime::ConsumerRouteSupportPosture::Unsupported,
        "scheduling stays withheld on the qualified route"
    );
}

#[test]
#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
fn the_projection_publishes_the_unqualified_truth_off_the_accepted_platform() {
    let selection = route_selection();
    let carrier = ClaudeAgentSdkRegisteredToolCarrier::new(&selection).expect("carrier is valid");
    let hosts = mounted_hosts(dispatcher("ok"));
    let readiness = RegisteredToolReadiness::evaluate(&hosts, &selection);
    let prepared = sdk_support::prepared_session(host_id());
    let applicability = swallowtail_runtime::ConsumerRouteApplicability::from_plan(prepared.plan());

    let contribution =
        project_claude_agent_sdk_registered_tool(&applicability, &carrier, &readiness)
            .expect("the contribution composes");

    assert!(matches!(
        claude_agent_sdk_registered_tool_qualification(),
        swallowtail_runtime::RegisteredToolRouteQualification::Unqualified
    ));
    let mediation =
        row_with_semantic_id(&contribution, CLAUDE_AGENT_SDK_MEDIATION_KIND_SEMANTIC_ID);
    assert_eq!(
        mediation.availability(),
        swallowtail_runtime::ConsumerRouteAvailability::Unavailable
    );
    assert_eq!(
        mediation
            .safe_reason()
            .map(|reason| reason.diagnostic().code().to_owned()),
        Some(
            swallowtail_adapter_claude_agent::sdk::registered_tool::
                CLAUDE_AGENT_SDK_REGISTERED_TOOL_PLATFORM_NOT_ADMITTED_CODE
                .to_owned()
        ),
    );
    assert_eq!(
        mediation
            .safe_reason()
            .map(|reason| reason.diagnostic().message().to_owned()),
        Some("the accepted live gate ran only on exact Darwin arm64".to_owned()),
    );
    for row in contribution.selection_rows() {
        assert_ne!(
            row.availability(),
            swallowtail_runtime::ConsumerRouteAvailability::Available,
            "an unqualified route publishes no available registered-capability row"
        );
    }
}

#[test]
fn the_qualified_route_binds_the_accepted_live_evidence_identities() {
    // Research 301 freezes the accepted Desktop Card 318 gate: exactly four
    // fresh opens and four prompt turns ran against source-linked Swallowtail,
    // with no retry, reconnect, respawn, repeated attempt, or model fallback.
    // The Allow case dispatched `desktop/reconcile` exactly once with unchanged
    // `{}` and correlated the fixed `{"ok":true}` result; Deny, cancellation
    // while pending, and stale/foreign callback rejection dispatched zero
    // times. Cleanup was the accepted Contract 019 route-qualified degraded
    // macOS posture — reapers joined and no lease, listener, or process
    // survived — never `Clean`. These identities are the qualification's only
    // live evidence.
    const SWALLOWTAIL_SOURCE: &str = "24f88fb8a1328aa0e85b9c91989962ba32b9c590";
    const DESKTOP_TASK: &str = "4356b461-cea3-4079-85cf-d9e63a1bb178";
    const DESKTOP_PR: &str = "181";
    const DESKTOP_PR_HEAD: &str = "18b70c917ccbc551f94596d364bf2f1916cc3065";
    const DESKTOP_REVIEW_COMMENT: &str = "5599408741";
    const DESKTOP_MERGE: &str = "807f7a3f916605e170b73f25812663fc0d9c295c";
    const DESKTOP_CLOSEOUT: &str = "03e71e90d5280550c175c85a4ac4202cca7d9da6";
    const CAPSULE_SHA256: &str = "c4de15a8a8b4a47996dc94c9cf7610e56a311a1b52266d156309cf944231c1f6";
    const COURIER: &str = "sha256:12db9fe39fe6928c179ee0e83afc96a7005717e0ef23a00fc25300c886f69720";
    const NATIVE_BINARY_SHA256: &str =
        "884baa38fe1a624be25c4a91568bf5a08b5cf4e7d7acf29b7760e3525d964898";
    const SDK_VERSION: &str = "0.3.259";
    const NATIVE_VERSION: &str = "2.1.259";
    const NODE_VERSION: &str = "22.23.2";
    const SIDECAR_TAG: &str = "swallowtail-claude-agent-sdk-sidecar@0.4.4";
    const MCP_PROTOCOL_VERSION: &str = "2025-11-25";
    const MODEL: &str = "claude-sonnet-5";
    const ATTEMPT_COUNT: usize = 4;
    const ALLOW_DISPATCH_COUNT: usize = 1;
    const CONTROL_DISPATCH_COUNT: usize = 0;

    // The claim is bound to exactly the tuple the capsules ran, and to nothing
    // adjacent: the route pins every version axis at this exact point.
    assert_eq!(
        CLAUDE_AGENT_SDK_REGISTERED_TOOL_SDK_VERSION,
        swallowtail_adapter_claude_agent::sdk::CLAUDE_AGENT_SDK_VERSION
    );
    assert_eq!(
        CLAUDE_AGENT_SDK_REGISTERED_TOOL_NATIVE_VERSION,
        swallowtail_adapter_claude_agent::sdk::CLAUDE_AGENT_SDK_NATIVE_VERSION
    );
    assert_eq!(
        SDK_VERSION,
        swallowtail_adapter_claude_agent::sdk::CLAUDE_AGENT_SDK_VERSION
    );
    assert_eq!(
        NATIVE_VERSION,
        swallowtail_adapter_claude_agent::sdk::CLAUDE_AGENT_SDK_NATIVE_VERSION
    );
    assert_eq!(
        NODE_VERSION,
        swallowtail_adapter_claude_agent::sdk::CLAUDE_AGENT_SDK_NODE_RUNTIME
    );
    // The sidecar source tag moves with every coordinated release, so the
    // live tag at this head is not the tag the capsules ran. The evidence
    // stays bound to the exact 0.4.4 source build and shares only the asset
    // lineage prefix with the live sidecar identity.
    assert_eq!(SIDECAR_TAG, "swallowtail-claude-agent-sdk-sidecar@0.4.4");
    assert!(
        swallowtail_adapter_claude_agent::sdk::CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG
            .starts_with("swallowtail-claude-agent-sdk-sidecar@")
    );
    assert_eq!(MCP_PROTOCOL_VERSION, CLAUDE_AGENT_SDK_MCP_PROTOCOL_VERSION);
    // The qualified mediation is exactly the private-loopback mediated-stdio
    // carrier shape the capsules ran, and no other transport.
    assert_eq!(
        CLAUDE_AGENT_SDK_REGISTERED_TOOL_MEDIATION,
        "route-local-stdio-mcp-mediation"
    );
    // Whole-identity receipts are pinned so a future head cannot silently
    // rebase the claim onto different evidence.
    assert_eq!(SWALLOWTAIL_SOURCE.len(), 40);
    assert_eq!(CAPSULE_SHA256.len(), 64);
    assert_eq!(NATIVE_BINARY_SHA256.len(), 64);
    assert!(COURIER.starts_with("sha256:"));
    assert_eq!(DESKTOP_TASK.len(), 36);
    assert_eq!(DESKTOP_PR, "181");
    assert_eq!(DESKTOP_PR_HEAD.len(), 40);
    assert!(!DESKTOP_REVIEW_COMMENT.is_empty());
    assert_eq!(DESKTOP_MERGE.len(), 40);
    assert_eq!(DESKTOP_CLOSEOUT.len(), 40);
    assert_eq!(MODEL, "claude-sonnet-5");
    assert_eq!(ATTEMPT_COUNT, 4);
    assert_eq!(ALLOW_DISPATCH_COUNT, 1);
    assert_eq!(CONTROL_DISPATCH_COUNT, 0);
    // The compiled platform decides which truth the projection publishes:
    // qualified only on the exact Darwin arm64 target the capsules ran on.
    let accepted_platform = cfg!(all(target_os = "macos", target_arch = "aarch64"));
    assert_eq!(
        matches!(
            claude_agent_sdk_registered_tool_qualification(),
            swallowtail_runtime::RegisteredToolRouteQualification::Qualified(_)
        ),
        accepted_platform,
        "the qualification never widens past the accepted platform"
    );
}

#[test]
fn an_accepted_allow_dispatches_one_unchanged_empty_object_and_correlates_its_result() {
    // Research 301's Allow case, bound deterministically: exactly one
    // `desktop/reconcile` dispatch whose arguments reach the linked host
    // unchanged as `{}`, and exactly one correlated `{"ok":true}` result
    // written back to the carrier wire.
    let observed: Arc<Mutex<Vec<Vec<u8>>>> = Arc::new(Mutex::new(Vec::new()));
    let capture = Arc::clone(&observed);
    let dispatcher = Arc::new(ScriptedRegisteredToolDispatcher::new(Arc::new(
        move |call, _context| {
            capture
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .push(call.arguments().expose_for_execution().to_vec());
            Ok(RegisteredToolOutcome::completed(
                call,
                RegisteredToolResult::new(
                    RegisteredToolPayload::new(
                        RegisteredToolSchemaMediaType::new("application/json")
                            .expect("media type is valid"),
                        b"{\"ok\":true}".to_vec(),
                        call.binding().effective_bounds().max_result_bytes(),
                    )
                    .expect("bounded result payload"),
                    RegisteredToolSchemaDigest::new("sha256:output").expect("digest is valid"),
                ),
            ))
        },
    )));
    let selection = route_selection();
    let hosts = mounted_hosts(Arc::clone(&dispatcher));
    let lease = open_lease(
        &hosts,
        &selection,
        fixture_admission(Arc::new(ScriptedAdmissionPort::current())),
        RegisteredToolLimits::ceiling(),
    );
    let carrier =
        ClaudeAgentSdkRegisteredToolCarrier::new(&selection).expect("carrier admits the selection");
    let mediator = ClaudeAgentSdkRegisteredToolMediator::new(carrier, lease)
        .expect("mediator binds its own lease");
    mediate(
        &mediator,
        &json!({
            "jsonrpc": "2.0",
            "id": 0,
            "method": "initialize",
            "params": {"protocolVersion": CLAUDE_AGENT_SDK_MCP_PROTOCOL_VERSION},
        }),
    )
    .expect("handshake is admissible");
    mediate(
        &mediator,
        &json!({"jsonrpc": "2.0", "method": "notifications/initialized"}),
    )
    .expect("initialized notification is admissible");
    mediator
        .record_admission(
            "mcp__swallowtail-registered-tools__desktop_reconcile",
            ClaudeAgentSdkRegisteredToolDecision::Allow,
        )
        .expect("the Allow decision is recordable");

    let reply = mediate(
        &mediator,
        &json!({
            "jsonrpc": "2.0",
            "id": 3,
            "method": "tools/call",
            "params": {"name": "desktop_reconcile", "arguments": {}},
        }),
    )
    .expect("the admitted call mediates");

    assert_eq!(
        dispatcher.dispatches(),
        1,
        "one Allow authorizes exactly one dispatch"
    );
    assert_eq!(
        observed
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .as_slice(),
        [b"{}".to_vec()],
        "the dispatch carries the unchanged empty arguments object"
    );
    let encoded = reply
        .response_bytes()
        .expect("the admitted call is answered");
    let decoded: Value = serde_json::from_slice(encoded).expect("the reply is valid JSON");
    assert_eq!(
        decoded["result"]["content"][0]["text"],
        json!("{\"ok\":true}"),
        "the fixed correlated result crosses unchanged"
    );
    assert_eq!(decoded["result"]["isError"], json!(false));
}

#[test]
fn qualify_rejects_the_host_mediated_callback_selection() {
    let selection = route_selection();
    let preparation = RegisteredToolPreparation::new(
        Arc::clone(selection.snapshot()),
        selection,
        fixture_admission(Arc::new(ScriptedAdmissionPort::current())),
        RegisteredToolLimits::ceiling(),
    );
    let Err(failure) = ClaudeAgentSdkRegisteredToolBinding::qualify(preparation) else {
        panic!("route binding is the mediated stdio proxy");
    };
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.claude-agent.sdk.registered_tool.transport_unsupported"
    );
}
