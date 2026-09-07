//! Provider-free Contract 063 selected-skill binding for the Claude SDK route.
//!
//! These fixtures cover the resolved bundle's typed boundary, the additive
//! profile/preparation/driver surfaces, the distinct open label, continuation
//! refusal, and the route-local Contract 061 projection. No provider, auth,
//! desktop, or workspace file is involved.

use crate::host_id;
use crate::sdk_support::{
    SdkFixtureHost, SdkScenario, cleanup_request, preparation, prepared_session, turn_request,
};
use futures_executor::block_on;
use std::sync::Arc;
use swallowtail_adapter_claude_agent::sdk::registered_tool::{
    ClaudeAgentSdkRegisteredToolCarrier,
    project_claude_agent_sdk_registered_tool_with_selected_skill,
};
use swallowtail_adapter_claude_agent::sdk::{
    ClaudeAgentSdkSelectedSkillBinding, ClaudeAgentSdkSessionProfile,
    prepare_claude_agent_sdk_session,
};
use swallowtail_core::{
    ConfiguredInstanceId, ExecutionHostId, ModelId, ModelRouteId, ResourceAccess,
    SessionAccessPolicy, SessionRef,
};
use swallowtail_runtime::{
    ConsumerRouteAvailability, ConsumerRouteEnumerableValue, ConsumerRouteProjectionSourceId,
    ConsumerRouteValueDomain, InteractiveSessionHandle, MAX_SELECTED_SKILL_CONTENT_BYTES,
    RegisteredServerId, RegisteredServerRevision, RegisteredToolBounds, RegisteredToolDeclaration,
    RegisteredToolEffectPosture, RegisteredToolExecutionKind, RegisteredToolId,
    RegisteredToolLocalName, RegisteredToolNamespace, RegisteredToolProtocolVersion,
    RegisteredToolReadiness, RegisteredToolRetryPosture, RegisteredToolSchema,
    RegisteredToolSchemaDialect, RegisteredToolSchemaDigest, RegisteredToolSchemaDocument,
    RegisteredToolSchemaMediaType, RegisteredToolSelection, RegisteredToolSnapshot,
    RegisteredToolSnapshotInput, RegisteredToolSource, RegisteredToolSourceId,
    RegisteredToolTransport, RegisteredToolTransportSupport, RequiredReferenceDescriptor,
    RequiredReferenceId, ResolvedSkillBundle, SelectedContentDescriptor, SelectedContentDigest,
    SelectedContentRef, SelectedContentResolution, SelectedSkillBundle, SelectedSkillBundleBounds,
    SelectedSkillHostResources, SelectedSkillId, SelectedSkillIdentity, SelectedSkillProvenance,
    SelectedSkillRevision, SessionResumeBinding,
};

const SKILL_BODY: &[u8] = b"# review skill\nreview the bounded workspace\n";
const REFERENCE_BODY: &[u8] = b"review-checklist-body";

fn media_type() -> RegisteredToolSchemaMediaType {
    RegisteredToolSchemaMediaType::new("text/markdown").expect("media type")
}

fn content_ref(label: &str) -> SelectedContentRef {
    SelectedContentRef::new(label).expect("content reference")
}

fn descriptor(label: &str, bytes: usize) -> SelectedContentDescriptor {
    SelectedContentDescriptor::new(content_ref(label), media_type(), bytes)
        .expect("content descriptor")
}

fn payload(body: &[u8]) -> swallowtail_runtime::RegisteredToolPayload {
    swallowtail_runtime::RegisteredToolPayload::new(
        media_type(),
        body.to_vec(),
        MAX_SELECTED_SKILL_CONTENT_BYTES,
    )
    .expect("payload")
}

fn resolved(body: &[u8]) -> SelectedContentResolution {
    SelectedContentResolution::Resolved(payload(body))
}

fn reference_id(value: &str) -> RequiredReferenceId {
    RequiredReferenceId::new(value).expect("reference id")
}

fn selected_bundle() -> SelectedSkillBundle {
    SelectedSkillBundle::new(
        SelectedSkillIdentity::new(
            SelectedSkillId::new("desktop.skill.review").expect("skill id"),
            SelectedSkillProvenance::ProjectBound,
            descriptor("host://skill", SKILL_BODY.len()),
        ),
        SelectedSkillRevision::new("7").expect("revision"),
        SelectedContentDigest::of_bytes(SKILL_BODY),
        [RequiredReferenceDescriptor::new(
            reference_id("checklist"),
            SelectedContentDigest::of_bytes(REFERENCE_BODY),
            descriptor("host://reference", REFERENCE_BODY.len()),
        )],
        SelectedSkillBundleBounds::ceiling(),
    )
    .expect("selected bundle")
}

fn resolved_bundle() -> ResolvedSkillBundle {
    selected_bundle()
        .resolve(
            &SelectedSkillHostResources::new()
                .with_skill_body(resolved(SKILL_BODY))
                .with_reference(reference_id("checklist"), resolved(REFERENCE_BODY)),
        )
        .expect("bundle resolves")
}

fn open_params(fixture: &SdkFixtureHost) -> serde_json::Value {
    fixture
        .inputs()
        .into_iter()
        .find(|input| input["command"] == "open")
        .expect("open command exists")["params"]
        .clone()
}

#[test]
fn a_resolved_bundle_crosses_as_one_distinct_labelled_input() {
    let host = host_id("claude-agent-sdk.fixture.selected-skill-present");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let profile_binding: ClaudeAgentSdkSelectedSkillBinding =
        ClaudeAgentSdkSessionProfile::read_only().with_selected_skill_bundle(resolved_bundle());
    let prepared = prepare_claude_agent_sdk_session(
        preparation(host.clone()).with_selected_skill_binding(profile_binding),
        swallowtail_runtime::SessionOptions::default(),
    )
    .expect("selected bundle preparation succeeds");
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let mut session = block_on(prepared.open_route_session(services.clone())).expect("opens");
    let mut turn = block_on(session.start_turn(
        turn_request("turn-selected-skill", "review the workspace"),
        services,
    ))
    .expect("turn starts");
    let _ = block_on(turn.take_terminal_outcome().expect("terminal"));
    let _ = block_on(turn.close());
    let _ = block_on(Box::new(session).close(cleanup_request(), cleanup_services));

    let params = open_params(&fixture);
    let bundle = &params["selectedSkillBundle"];
    assert_eq!(bundle["identity"], "desktop.skill.review");
    assert_eq!(bundle["provenance"], "project-bound");
    assert_eq!(bundle["revision"], "7");
    assert_eq!(
        bundle["digest"],
        SelectedContentDigest::of_bytes(SKILL_BODY).as_str()
    );
    assert_eq!(bundle["body"]["mediaType"], "text/markdown");
    assert_eq!(
        bundle["body"]["content"],
        std::str::from_utf8(SKILL_BODY).expect("skill body text")
    );
    assert_eq!(
        bundle["requiredReferences"].as_array().map(Vec::len),
        Some(1)
    );
    assert_eq!(bundle["requiredReferences"][0]["id"], "checklist");
    assert_eq!(
        bundle["requiredReferences"][0]["digest"],
        SelectedContentDigest::of_bytes(REFERENCE_BODY).as_str()
    );
    assert_eq!(
        bundle["requiredReferences"][0]["content"]["content"],
        std::str::from_utf8(REFERENCE_BODY).expect("reference body text")
    );
    assert!(params.get("developerInstructions").is_none());
    assert!(
        fixture
            .inputs()
            .iter()
            .filter(|input| input["command"] == "query")
            .all(|input| input["params"].get("text").is_some())
    );
}

#[test]
fn an_absent_bundle_keeps_the_open_shape_unchanged() {
    let host = host_id("claude-agent-sdk.fixture.selected-skill-absent");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let session = block_on(prepared.open_route_session(services)).expect("opens");
    let params = open_params(&fixture);
    assert!(params.get("selectedSkillBundle").is_none());
    assert!(params.get("developerInstructions").is_none());
    let _ = block_on(Box::new(session).close(cleanup_request(), cleanup_services));
}

#[test]
fn tampered_digest_fails_closed_before_provider_work() {
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let tampered = SelectedSkillBundle::new(
        SelectedSkillIdentity::new(
            SelectedSkillId::new("desktop.skill.review").expect("skill id"),
            SelectedSkillProvenance::ProjectBound,
            descriptor("host://skill", SKILL_BODY.len()),
        ),
        SelectedSkillRevision::new("7").expect("revision"),
        SelectedContentDigest::of_bytes(b"tampered"),
        [],
        SelectedSkillBundleBounds::ceiling(),
    )
    .expect("tampered declaration");
    let failure = tampered
        .resolve(&SelectedSkillHostResources::new().with_skill_body(resolved(SKILL_BODY)))
        .expect_err("digest mismatch fails closed");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.registered_tool.selected_content_mismatch"
    );
    assert!(fixture.inputs().is_empty());
}

#[test]
fn oversized_reference_fails_closed_before_provider_work() {
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let oversized = [REFERENCE_BODY, b"-and-more"].concat();
    let failure = selected_bundle()
        .resolve(
            &SelectedSkillHostResources::new()
                .with_skill_body(resolved(SKILL_BODY))
                .with_reference(reference_id("checklist"), resolved(&oversized)),
        )
        .expect_err("oversized reference fails closed");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.registered_tool.limit_exceeded"
    );
    assert!(fixture.inputs().is_empty());
}

#[test]
fn foreign_reference_fails_closed_before_provider_work() {
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let failure = selected_bundle()
        .resolve(
            &SelectedSkillHostResources::new()
                .with_skill_body(resolved(SKILL_BODY))
                .with_reference(reference_id("checklist"), resolved(REFERENCE_BODY))
                .with_reference(reference_id("unrelated"), resolved(b"foreign")),
        )
        .expect_err("foreign reference fails closed");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.registered_tool.foreign_selected_content"
    );
    assert!(fixture.inputs().is_empty());
}

#[test]
fn a_bound_bundle_cannot_redeclare_on_resume_or_listing() {
    let host = host_id("claude-agent-sdk.fixture.selected-skill-continuation");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = preparation(host.clone()).with_selected_skill_bundle(resolved_bundle());
    let prepared =
        prepare_claude_agent_sdk_session(prepared, swallowtail_runtime::SessionOptions::default())
            .expect("preparation succeeds");
    let binding = SessionResumeBinding::new(
        SessionRef::new("provider-session-existing").expect("provider session"),
        ConfiguredInstanceId::new("claude-agent-sdk.fixture").expect("instance"),
        host.clone(),
        ModelRouteId::new("claude-agent-sdk.fixture.route").expect("route"),
        ModelId::new("claude-sonnet-5").expect("model"),
        swallowtail_runtime::WorkingResourceRef::new("claude-agent-sdk.fixture.workspace")
            .expect("workspace"),
        SessionAccessPolicy::ambient_harness(ResourceAccess::Read),
    );
    let failure = prepared
        .resume_request(
            swallowtail_runtime::RequestId::new("resume-selected-skill").expect("request"),
            binding,
        )
        .expect_err("resume refuses selected bundle");
    assert_eq!(
        failure.diagnostic().safe().code(),
        "swallowtail.claude-agent.sdk.preparation.resume_selected_skill_unsupported"
    );
    assert!(fixture.inputs().is_empty());
}

#[test]
fn selected_skill_projection_is_available_without_qualifying_registered_tools() {
    let selection = projection_selection();
    let carrier = ClaudeAgentSdkRegisteredToolCarrier::new(&selection).expect("carrier");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let services = fixture.services(selection.snapshot().execution_host_id().clone());
    let readiness = RegisteredToolReadiness::evaluate(&services, &selection);
    let prepared = prepared_session(selection.snapshot().execution_host_id().clone());
    let applicability = swallowtail_runtime::ConsumerRouteApplicability::from_plan(prepared.plan());
    let contribution = project_claude_agent_sdk_registered_tool_with_selected_skill(
        &applicability,
        &carrier,
        &readiness,
        Some(&resolved_bundle()),
    )
    .expect("projection");
    let row = contribution
        .session_start_rows()
        .find(|row| {
            row.identity()
                .namespaced_extension()
                .is_some_and(|extension| {
                    extension.semantic_id()
                        == swallowtail_runtime::SELECTED_SKILL_BUNDLE_SEMANTIC_ID
                })
        })
        .expect("selected skill row");
    assert_eq!(row.availability(), ConsumerRouteAvailability::Available);
    assert_eq!(
        row.control_value().expect("control value").domain(),
        &ConsumerRouteValueDomain::Enumerated(
            swallowtail_runtime::ConsumerRouteEnumeratedValues::new(
                [
                    "desktop.skill.review".to_owned(),
                    "project-bound".to_owned(),
                    "7".to_owned(),
                    SelectedContentDigest::of_bytes(SKILL_BODY)
                        .as_str()
                        .to_owned(),
                    "references=1".to_owned(),
                    format!("bytes={}", SKILL_BODY.len() + REFERENCE_BODY.len()),
                ]
                .into_iter()
                .map(|value| ConsumerRouteEnumerableValue::new(value).expect("value"))
            )
            .expect("values"),
        )
    );
    assert!(
        contribution.selection_rows().any(|row| {
            row.availability() != ConsumerRouteAvailability::Available
                && row
                    .identity()
                    .namespaced_extension()
                    .is_some_and(|extension| {
                        extension.semantic_id()
                            != swallowtail_runtime::SELECTED_SKILL_BUNDLE_SEMANTIC_ID
                    })
        }),
        "registered-tool rows retain the Unqualified/live-gate posture"
    );
}

#[test]
fn selected_skill_projection_is_published_for_a_prepared_session_without_registered_tools() {
    let host = host_id("claude-agent-sdk.fixture.selected-skill-only-projection");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepare_claude_agent_sdk_session(
        preparation(host.clone()).with_selected_skill_bundle(resolved_bundle()),
        swallowtail_runtime::SessionOptions::default(),
    )
    .expect("selected-only preparation succeeds");
    let services = fixture.services(host);
    let contribution = prepared
        .registered_capability_projection_contribution(
            ConsumerRouteProjectionSourceId::new("claude-agent.sdk.selected-skill-prepared-input")
                .expect("source id"),
            &services,
        )
        .expect("selected-only contribution is published")
        .expect("selected-only projection succeeds");
    let row = contribution
        .session_start_rows()
        .find(|row| {
            row.identity()
                .namespaced_extension()
                .is_some_and(|extension| {
                    extension.semantic_id()
                        == swallowtail_runtime::SELECTED_SKILL_BUNDLE_SEMANTIC_ID
                })
        })
        .expect("selected skill row");
    assert_eq!(row.availability(), ConsumerRouteAvailability::Available);
    assert!(contribution.selection_rows().next().is_none());
}

fn projection_selection() -> RegisteredToolSelection {
    let host = ExecutionHostId::new("fixture.host.selected-skill").expect("host");
    let tool_id = RegisteredToolId::new(
        RegisteredToolNamespace::new("desktop").expect("namespace"),
        RegisteredToolLocalName::new("read-note").expect("local name"),
    );
    let schema = || {
        RegisteredToolSchema::new(
            swallowtail_runtime::RegisteredToolSchemaNamespace::new("desktop.schema")
                .expect("namespace"),
            RegisteredToolSchemaMediaType::new("application/json").expect("media type"),
            RegisteredToolSchemaDialect::new("json-schema-2020-12").expect("dialect"),
            RegisteredServerRevision::new("1").expect("revision"),
            RegisteredToolSchemaDigest::new("sha256:schema").expect("digest"),
            RegisteredToolSchemaDocument::new("{\"type\":\"object\"}").expect("document"),
        )
    };
    let protocol = RegisteredToolProtocolVersion::new(
        swallowtail_runtime::REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION,
    )
    .expect("protocol");
    let snapshot = Arc::new(
        RegisteredToolSnapshot::new(RegisteredToolSnapshotInput {
            server_id: RegisteredServerId::new("desktop.server").expect("server"),
            revision: RegisteredServerRevision::new("1").expect("revision"),
            execution_host_id: host,
            declarations: vec![
                RegisteredToolDeclaration::new(
                    tool_id,
                    RegisteredToolExecutionKind::Mcp,
                    schema(),
                    schema(),
                    RegisteredToolEffectPosture::ReadOnly,
                    RegisteredToolRetryPosture::ConsumerRetryable,
                    RegisteredToolBounds::ceiling(),
                )
                .expect("declaration"),
            ],
            transports: vec![
                RegisteredToolTransportSupport::new(
                    RegisteredToolTransport::HostMediatedCallback,
                    [protocol.clone()],
                )
                .expect("transport"),
            ],
            required_services: [].into_iter().collect(),
            credential_references: Vec::new(),
            executable_recipes: Vec::new(),
            environment_recipes: Vec::new(),
            bounds: RegisteredToolBounds::ceiling(),
            source: RegisteredToolSource::new(
                RegisteredToolSourceId::new("desktop.source").expect("source"),
                swallowtail_runtime::MonotonicInstant::from_ticks(1),
            ),
        })
        .expect("snapshot"),
    );
    RegisteredToolSelection::new(
        snapshot,
        [RegisteredToolId::new(
            RegisteredToolNamespace::new("desktop").expect("namespace"),
            RegisteredToolLocalName::new("read-note").expect("local name"),
        )],
        RegisteredToolTransport::HostMediatedCallback,
        protocol,
    )
    .expect("selection")
}
