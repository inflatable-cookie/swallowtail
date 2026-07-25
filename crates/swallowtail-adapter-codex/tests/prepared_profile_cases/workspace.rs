use super::*;

#[test]
fn bounded_workspace_is_separate_version_gated_and_plan_derived() {
    let recording = RecordingHostServices::default();
    let prepared_app = prepared(CodexPreparedDriver::AppServer, "0.145.0", &recording, true);
    let profile = prepared_app
        .prepare_bounded_workspace_session(CodexSessionProfileInput::new(
            RequestId::new("workspace-session").unwrap(),
            model(),
            working_resource(),
            None,
            SessionOptions::default(),
        ))
        .expect("bounded workspace prepares");
    assert_eq!(profile.kind(), CodexPreparedSessionKind::BoundedWorkspace);
    assert_eq!(
        profile.request().access_policy().resource_access(),
        Some(swallowtail_core::ResourceAccess::ReadWrite)
    );
    assert!(
        profile
            .plan()
            .requirements()
            .host_services()
            .any(|kind| kind == HostServiceKind::WorkingResource)
    );
    assert!(
        profile
            .plan()
            .requirements()
            .capabilities()
            .any(|requirement| requirement.capability() == Capability::WorkingResource)
    );

    let (process, state) = ScriptedAppServer::gate_enforcing(AppServerMode::CompleteTurn);
    let handle = block_on(profile.open_session(host_services_with(
        process,
        &recording,
        [HostServiceKind::WorkingResource],
    )))
    .expect("bounded workspace opens");
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    let start = state
        .messages()
        .into_iter()
        .find(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("thread/start")
        })
        .expect("thread start was sent");
    assert_eq!(
        start
            .pointer("/params/sandbox")
            .and_then(serde_json::Value::as_str),
        Some("workspace-write")
    );

    let legacy = prepared(CodexPreparedDriver::AppServer, "0.130.0", &recording, true);
    let failure = legacy
        .prepare_bounded_workspace_session(CodexSessionProfileInput::new(
            RequestId::new("legacy-workspace").unwrap(),
            model(),
            working_resource(),
            None,
            SessionOptions::default(),
        ))
        .expect_err("pre-workspace version fails");
    assert_eq!(failure.stage(), PreparationStage::Preflight);
}
