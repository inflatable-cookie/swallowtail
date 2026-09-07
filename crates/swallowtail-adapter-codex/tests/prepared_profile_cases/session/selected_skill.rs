// Contract 063 selected-skill bundle binding for the Codex app-server route.
//
// Every fixture drives the mounted wiring: the fail-closed runtime resolver,
// the additive `CodexSessionProfileInput::with_selected_skill_bundle` API,
// and the real app-server driver against the scripted fixture server. The
// labelled input is asserted on the `thread/start` wire, never inferred.
use swallowtail_runtime::{
    ConsumerRouteAvailability, ConsumerRouteProjectionContribution, ConsumerRouteProjectionRow,
    ConsumerRouteValueDomain, MAX_SELECTED_SKILL_CONTENT_BYTES, RegisteredToolFailureKind,
    RequiredReferenceDescriptor, RequiredReferenceId, ResolvedSkillBundle,
    SELECTED_SKILL_BUNDLE_SEMANTIC_ID, SelectedContentDescriptor, SelectedContentDigest,
    SelectedContentRef, SelectedContentResolution, SelectedSkillBundle, SelectedSkillBundleBounds,
    SelectedSkillHostResources, SelectedSkillId, SelectedSkillIdentity, SelectedSkillProvenance,
    SelectedSkillRevision,
};

const SKILL_BODY: &[u8] = b"# review skill\nreview the bounded workspace\n";
const REFERENCE_BODY: &[u8] = b"review-checklist-body";
const TURN_TEXT: &str = "review the workspace";

fn media_type() -> swallowtail_runtime::RegisteredToolSchemaMediaType {
    swallowtail_runtime::RegisteredToolSchemaMediaType::new("text/markdown").expect("media type")
}

fn content_ref(label: &str) -> SelectedContentRef {
    SelectedContentRef::new(label).expect("selected content reference")
}

fn descriptor(label: &str, declared_bytes: usize) -> SelectedContentDescriptor {
    SelectedContentDescriptor::new(content_ref(label), media_type(), declared_bytes)
        .expect("content descriptor")
}

fn reference_id(label: &str) -> RequiredReferenceId {
    RequiredReferenceId::new(label).expect("reference id")
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
    .expect("bundle")
}

fn host_resources() -> SelectedSkillHostResources {
    SelectedSkillHostResources::new()
        .with_skill_body(resolved(SKILL_BODY))
        .with_reference(reference_id("checklist"), resolved(REFERENCE_BODY))
}

fn resolved_bundle() -> ResolvedSkillBundle {
    selected_bundle()
        .resolve(&host_resources())
        .expect("bundle resolves")
}

fn skill_profile_input(label: &str) -> CodexSessionProfileInput {
    CodexSessionProfileInput::new(
        RequestId::new(format!("selected-skill-{label}")).unwrap(),
        model(),
        working_resource(),
        None,
        SessionOptions::default(),
    )
}

fn thread_start_params(state: &support::app_server::AppServerState) -> serde_json::Value {
    state
        .messages()
        .into_iter()
        .find(|message| message["method"] == "thread/start")
        .expect("thread/start was sent")["params"]
        .clone()
}

fn turn_start_params(state: &support::app_server::AppServerState) -> serde_json::Value {
    state
        .messages()
        .into_iter()
        .find(|message| message["method"] == "turn/start")
        .expect("turn/start was sent")["params"]
        .clone()
}

fn skill_row(
    contribution: &ConsumerRouteProjectionContribution,
) -> &ConsumerRouteProjectionRow {
    contribution
        .session_start_rows()
        .find(|row| {
            row.identity()
                .namespaced_extension()
                .is_some_and(|extension| extension.semantic_id() == SELECTED_SKILL_BUNDLE_SEMANTIC_ID)
        })
        .expect("selected-skill-bundle row is published")
}

fn skill_row_values(contribution: &ConsumerRouteProjectionContribution) -> Vec<String> {
    match skill_row(contribution)
        .control_value()
        .expect("skill row publishes a value")
        .domain()
    {
        ConsumerRouteValueDomain::Enumerated(values) => values
            .values()
            .map(|value| value.as_str().to_owned())
            .collect(),
        _ => Vec::new(),
    }
}

#[test]
fn a_resolved_bundle_opens_as_one_distinct_labelled_input() {
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.145.0",
        &RecordingHostServices::default(),
        false,
    );
    let profile = prepared_app
        .prepare_read_only_session(
            skill_profile_input("present").with_selected_skill_bundle(resolved_bundle()),
        )
        .expect("a bundle-bound preparation prepares");
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let services = host_services(process);
    let mut session = block_on(profile.open_session(services.clone()))
        .expect("the bundle-bound session opens");
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("turn-selected-skill").unwrap(),
            OperationContent::new(TURN_TEXT).unwrap(),
        ),
        services.clone(),
    ))
    .expect("turn starts");
    let _terminal = block_on(turn.take_terminal_outcome().expect("terminal is available"));
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(
        block_on(support::close_session(session, services)),
        CleanupOutcome::Clean
    );

    let params = thread_start_params(&state);
    // The bundle crosses as one distinct labelled input, byte-identical to
    // what Swallowtail validated, never inside the instructions input.
    let bundle = &params["selectedSkillBundle"];
    assert_eq!(bundle["identity"], "desktop.skill.review");
    assert_eq!(bundle["provenance"], "project-bound");
    assert_eq!(bundle["revision"], "7");
    assert_eq!(
        bundle["digest"],
        SelectedContentDigest::of_bytes(SKILL_BODY).as_str()
    );
    assert_eq!(
        bundle["body"]["content"],
        std::str::from_utf8(SKILL_BODY).expect("skill body is text")
    );
    assert_eq!(bundle["body"]["mediaType"], "text/markdown");
    assert_eq!(bundle["requiredReferences"].as_array().map(Vec::len), Some(1));
    assert_eq!(bundle["requiredReferences"][0]["id"], "checklist");
    assert_eq!(
        bundle["requiredReferences"][0]["digest"],
        SelectedContentDigest::of_bytes(REFERENCE_BODY).as_str()
    );
    assert_eq!(
        bundle["requiredReferences"][0]["content"]["content"],
        std::str::from_utf8(REFERENCE_BODY).expect("reference body is text")
    );
    // The instructions input stays exactly what it was: absent here, because
    // no developer instructions were configured for this session.
    assert!(params.get("developerInstructions").is_none());
    // The per-turn user text carries only the turn text.
    let turn_params = turn_start_params(&state);
    assert_eq!(
        turn_params["input"][0]["text"],
        serde_json::Value::String(TURN_TEXT.to_owned())
    );
}

#[test]
fn an_absent_bundle_keeps_the_open_shape_unchanged() {
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.145.0",
        &RecordingHostServices::default(),
        false,
    );
    let profile = prepared_app
        .prepare_read_only_session(skill_profile_input("absent"))
        .expect("an ordinary preparation prepares");
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let services = host_services(process);
    let session = block_on(profile.open_session(services.clone()))
        .expect("the ordinary session opens");
    assert_eq!(
        block_on(support::close_session(session, services)),
        CleanupOutcome::Clean
    );

    let params = thread_start_params(&state);
    assert!(params.get("selectedSkillBundle").is_none());
    assert!(params.get("developerInstructions").is_none());
}

#[test]
fn a_tampered_body_digest_fails_typed_before_provider_work() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let mismatched = SelectedSkillBundle::new(
        SelectedSkillIdentity::new(
            SelectedSkillId::new("desktop.skill.review").expect("skill id"),
            SelectedSkillProvenance::ProjectBound,
            descriptor("host://skill", SKILL_BODY.len()),
        ),
        SelectedSkillRevision::new("7").expect("revision"),
        SelectedContentDigest::of_bytes(b"declared-but-never-read"),
        [],
        SelectedSkillBundleBounds::ceiling(),
    )
    .expect("bundle");

    let failure = mismatched
        .resolve(&SelectedSkillHostResources::new().with_skill_body(resolved(SKILL_BODY)))
        .expect_err("a changed body fails against its declared digest");

    assert_eq!(
        failure.diagnostic().code(),
        RegisteredToolFailureKind::SelectedContentMismatch.code()
    );
    assert!(!state.started(), "no provider process ever starts");
    let _ = process;
}

#[test]
fn an_oversized_reference_fails_typed_before_provider_work() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let oversized = [REFERENCE_BODY, b"-and-more"].concat();
    let failure = selected_bundle()
        .resolve(
            &SelectedSkillHostResources::new()
                .with_skill_body(resolved(SKILL_BODY))
                .with_reference(reference_id("checklist"), resolved(oversized.as_slice())),
        )
        .expect_err("content beyond its declared bound is rejected");

    assert_eq!(
        failure.diagnostic().code(),
        RegisteredToolFailureKind::LimitExceeded.code()
    );
    assert!(!state.started(), "no provider process ever starts");
    let _ = process;
}

#[test]
fn a_foreign_reference_fails_typed_before_provider_work() {
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let failure = selected_bundle()
        .resolve(
            &SelectedSkillHostResources::new()
                .with_skill_body(resolved(SKILL_BODY))
                .with_reference(reference_id("checklist"), resolved(REFERENCE_BODY))
                .with_reference(reference_id("unrelated"), resolved(b"unrelated-body")),
        )
        .expect_err("content the bundle never declared is foreign");

    assert_eq!(
        failure.diagnostic().code(),
        RegisteredToolFailureKind::ForeignSelectedContent.code()
    );
    assert!(!state.started(), "no provider process ever starts");
    let _ = process;
}

#[test]
fn a_bound_bundle_cannot_redeclare_on_resume_or_load() {
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.145.0",
        &RecordingHostServices::default(),
        false,
    );
    let profile = prepared_app
        .prepare_read_only_session(
            skill_profile_input("continuation").with_selected_skill_bundle(resolved_bundle()),
        )
        .expect("a bundle-bound preparation prepares");
    let binding = support::session_resume_binding(profile.plan(), "thread-provider-existing");

    let resume_failure = profile
        .resume_request(RequestId::new("resume").unwrap(), binding.clone())
        .expect_err("a resumed thread cannot redeclare the bundle");
    assert_eq!(
        resume_failure.diagnostic().safe().code(),
        "swallowtail.codex.preparation.resume_selected_skill_unsupported"
    );
    let load_failure = profile
        .load_request(RequestId::new("load").unwrap(), binding)
        .expect_err("a loaded thread cannot redeclare the bundle");
    assert_eq!(
        load_failure.diagnostic().safe().code(),
        "swallowtail.codex.preparation.load_selected_skill_unsupported"
    );
}

#[test]
fn the_selected_skill_bundle_row_publishes_with_this_route() {
    let admission = std::sync::Arc::new(swallowtail_testkit::ScriptedAdmissionPort::current());
    let dispatcher = support::registered::RouteDispatcher::new(|call| {
        Ok(support::registered::text_result(call, "unused"))
    });
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.145.0",
        &RecordingHostServices::default(),
        false,
    );
    let profile = prepared_app
        .prepare_read_only_session(
            skill_profile_input("projection").with_selected_skill_bundle(resolved_bundle()),
        )
        .expect("a registered bundle-bound preparation prepares");
    assert!(profile.registered_tools().is_none());

    let registered_profile = prepared_app
        .prepare_read_only_session(
            skill_profile_input("registered-projection")
                .with_selected_skill_bundle(resolved_bundle())
                .with_registered_tools(support::registered::registered_preparation(
                    &admission,
                    swallowtail_runtime::RegisteredToolExecutionKind::NativeClient,
                )),
        )
        .expect("a registered bundle-bound preparation prepares");
    let (process, _state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let (services, _local) = support::registered::registered_services(process, dispatcher);
    let contribution = registered_profile
        .registered_capability_projection_contribution(
            registered_source_id(),
            &services,
        )
        .expect("a registered session publishes its capability")
        .expect("the contribution is valid");

    let values = skill_row_values(&contribution);
    assert!(
        values.contains(&"desktop.skill.review".to_owned()),
        "the row carries the bundle identity: {values:?}"
    );
    assert!(
        values.contains(&SelectedContentDigest::of_bytes(SKILL_BODY).as_str().to_owned()),
        "the row carries the bundle digest: {values:?}"
    );
    assert_eq!(
        skill_row(&contribution).availability(),
        ConsumerRouteAvailability::Available
    );
}
