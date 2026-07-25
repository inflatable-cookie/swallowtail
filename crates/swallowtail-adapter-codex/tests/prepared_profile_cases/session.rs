use super::*;

#[test]
fn prepared_session_derives_tool_bounds_from_bounded_declarations() {
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.145.0",
        &RecordingHostServices::default(),
        false,
    );
    let larger_schema = serde_json::to_vec(&serde_json::json!({
        "type": "object",
        "description": "x".repeat(8 * 1024),
    }))
    .expect("schema serializes");
    let larger_schema_bytes = u64::try_from(larger_schema.len()).expect("schema length fits");
    let larger_tool = ToolDeclaration::new(
        "large_lookup",
        SchemaDocument::inline(larger_schema, 16 * 1024).expect("schema is bounded"),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .expect("tool declaration is valid");
    let profile = prepared_app
        .prepare_read_only_session(CodexSessionProfileInput::new(
            RequestId::new("bounded-tools").unwrap(),
            model(),
            working_resource(),
            None,
            SessionOptions::default().with_tools([tool("lookup"), larger_tool]),
        ))
        .expect("bounded tool declarations prepare");
    let requirement = profile
        .plan()
        .requirements()
        .capabilities()
        .find(|requirement| requirement.capability() == Capability::ToolCalls)
        .expect("tool capability is planned");

    assert!(
        requirement
            .constraints()
            .any(|constraint| constraint == &CapabilityConstraint::ToolMaximumCount(2))
    );
    assert!(requirement.constraints().any(|constraint| {
        constraint == &CapabilityConstraint::ToolMaximumSchemaBytes(larger_schema_bytes)
    }));

    let (process, state) = ScriptedAppServer::gate_enforcing(AppServerMode::CompleteTurn);
    let handle = block_on(profile.open_session(support::host_services(process)))
        .expect("prepared session opens");
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    let thread_start = state
        .messages()
        .into_iter()
        .find(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("thread/start")
        })
        .expect("thread start is captured");
    assert_eq!(
        thread_start
            .pointer("/params/dynamicTools")
            .and_then(serde_json::Value::as_array)
            .map(Vec::len),
        Some(2)
    );
    assert!(state.waited());
}

#[test]
fn session_resume_agreement_is_derived_and_unsupported_deadlines_fail_in_preparation() {
    let recording = RecordingHostServices::default();
    let prepared_app = prepared(CodexPreparedDriver::AppServer, "0.145.0", &recording, false);
    let profile = prepared_app
        .prepare_read_only_session(CodexSessionProfileInput::new(
            RequestId::new("open").unwrap(),
            model(),
            working_resource(),
            None,
            SessionOptions::default(),
        ))
        .expect("read-only session prepares");
    let binding = support::session_resume_binding(profile.plan(), "thread-1");
    let resume = profile
        .resume_request(RequestId::new("resume").unwrap(), binding.clone())
        .expect("resume request derives immutable agreement");
    assert_eq!(resume.access_policy(), profile.request().access_policy());
    assert_eq!(
        resume.harness_configuration_posture(),
        profile.request().harness_configuration_posture()
    );
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let handle = block_on(
        profile
            .resume_session(
                RequestId::new("bound-resume").unwrap(),
                binding,
                support::host_services(process),
            )
            .expect("bound resume prepares"),
    )
    .expect("bound resume opens");
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(state.methods().contains(&"thread/resume".to_owned()));
    assert!(state.waited());

    let failure = prepared_app
        .prepare_read_only_session(CodexSessionProfileInput::new(
            RequestId::new("deadline").unwrap(),
            model(),
            working_resource(),
            Some(Deadline::at(MonotonicInstant::from_ticks(200))),
            SessionOptions::default(),
        ))
        .expect_err("unsupported session deadline fails during preparation");
    assert_eq!(failure.stage(), PreparationStage::Preflight);
}

#[test]
fn prepared_profile_keeps_exact_version_and_access_provenance_visible() {
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.146.0",
        &RecordingHostServices::default(),
        false,
    );
    let profile = prepared_app
        .prepare_catalogue(RequestId::new("catalogue-evidence").unwrap(), None)
        .expect("catalogue prepares");

    assert_eq!(
        profile
            .evidence()
            .observation()
            .version()
            .version()
            .as_str(),
        "0.146.0"
    );
    assert_eq!(
        profile.evidence().access().provenance(),
        &swallowtail_runtime::AccessEvidenceProvenance::CallerAsserted
    );
    swallowtail_testkit::assert_prepared_operation_evidence_matches_plan(
        profile.evidence().operation(),
        profile.plan(),
    );
    assert_eq!(
        profile.evidence().operation().binding().driver_role(),
        DriverRole::ModelCatalog
    );
    assert_eq!(
        profile
            .evidence()
            .operation()
            .interface_compatibility()
            .count(),
        1
    );
    assert!(matches!(
        profile
            .evidence()
            .operation()
            .interface_compatibility()
            .next()
            .expect("Codex version evidence is present")
            .assessment(),
        swallowtail_core::InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}
