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
    let load_services = support::host_services(process);
    let loaded = block_on(
        profile
            .load_session(
                RequestId::new("bound-load").unwrap(),
                binding.clone(),
                load_services.clone(),
            )
            .expect("bound load prepares"),
    )
    .expect("bound load opens");
    assert_eq!(
        loaded
            .replay()
            .filter_map(|item| item.content().map(|content| content.as_str()))
            .collect::<Vec<_>>(),
        ["Earlier question.", "Earlier answer."]
    );
    let (_, loaded_handle) = loaded.into_parts();
    assert_eq!(
        loaded_handle
            .management_binding()
            .expect("prepared load returns management authority")
            .origin(),
        swallowtail_core::ProviderSessionBindingOrigin::Loaded
    );
    assert_eq!(
        block_on(support::close_session(loaded_handle, load_services)),
        CleanupOutcome::Clean
    );
    assert!(state.methods().contains(&"thread/resume".to_owned()));
    assert!(state.waited());

    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let resume_services = support::host_services(process);
    let handle = block_on(
        profile
            .resume_session(
                RequestId::new("bound-resume").unwrap(),
                binding,
                resume_services.clone(),
            )
            .expect("bound resume prepares"),
    )
    .expect("bound resume opens");
    assert_eq!(
        handle
            .management_binding()
            .expect("prepared resume returns management authority")
            .origin(),
        swallowtail_core::ProviderSessionBindingOrigin::Resumed
    );
    assert_eq!(
        block_on(support::close_session(handle, resume_services)),
        CleanupOutcome::Clean
    );
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
