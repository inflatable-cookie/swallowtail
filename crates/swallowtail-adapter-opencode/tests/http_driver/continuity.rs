const CONTINUITY_CORPUS: &str =
    include_str!("../fixtures/opencode-v1.14.48-v1.18.10/session-continuity.json");

#[test]
fn every_continuity_segment_loads_oldest_first_and_resumes_without_history() {
    let corpus: serde_json::Value =
        serde_json::from_str(CONTINUITY_CORPUS).expect("continuity corpus parses");
    for segment in corpus["segments"].as_array().expect("segments") {
        let version = segment["minimum"].as_str().expect("version");
        let server = FixtureServer::start_with_version(StreamFixture::Success, version);
        let fixture = Fixture::new_with_version(server.endpoint(), "host.continuity", version);
        let plan = fixture.plan(DriverRole::InteractiveSession);
        let binding = SessionResumeBinding::new(
            swallowtail_core::SessionRef::new("ses_fixture").unwrap(),
            plan.instance_id().clone(),
            plan.execution_host_id().clone(),
            plan.model_route_id().unwrap().clone(),
            plan.model_id().unwrap().clone(),
            fixture.resource.clone(),
            SessionAccessPolicy::ambient_harness(swallowtail_core::ResourceAccess::Read),
        );
        let loaded = block_on(OpenCodeHttpDriver::new().load_session(
            plan.clone(),
            LoadSessionRequest::from_plan(
                &plan,
                RequestId::new(format!("load-{version}")).unwrap(),
                binding.clone(),
                fixture.resource.clone(),
                None,
            )
            .unwrap(),
            fixture.services(),
        ))
        .expect("session loads");
        assert_eq!(
            loaded
                .replay()
                .filter_map(|item| item.content().map(OperationContent::as_str))
                .collect::<Vec<_>>(),
            [
                "Earlier question.",
                "Earlier answer.",
                "Later question.",
                "Later answer."
            ]
        );
        let (_, handle) = loaded.into_parts();
        assert_eq!(handle.resume_binding(), Some(&binding));
        assert!(matches!(
            block_on(handle.close()),
            swallowtail_runtime::CleanupOutcome::Clean
        ));

        let history_requests = server
            .requests()
            .iter()
            .filter(|request| request.contains("/message?"))
            .count();
        let resumed = block_on(OpenCodeHttpDriver::new().resume_session(
            plan.clone(),
            ResumeSessionRequest::from_plan(
                &plan,
                RequestId::new(format!("resume-{version}")).unwrap(),
                binding.clone(),
                fixture.resource.clone(),
                None,
            )
            .unwrap(),
            fixture.services(),
        ))
        .expect("session resumes");
        assert_eq!(resumed.resume_binding(), Some(&binding));
        assert_eq!(
            server
                .requests()
                .iter()
                .filter(|request| request.contains("/message?"))
                .count(),
            history_requests,
            "resume must not fetch replay"
        );
        assert!(matches!(
            block_on(resumed.close()),
            swallowtail_runtime::CleanupOutcome::Clean
        ));
    }
}

#[test]
fn persisted_binding_resumes_and_prompts_the_exact_session_without_fresh_creation() {
    let server = FixtureServer::start_with_version(StreamFixture::Compaction, "1.18.10");
    let fixture = Fixture::new_with_version(server.endpoint(), "host.restart", "1.18.10");
    let driver = OpenCodeHttpDriver::new();
    let plan = fixture.plan(DriverRole::InteractiveSession);
    let access = SessionAccessPolicy::ambient_harness(swallowtail_core::ResourceAccess::Read);

    let session = block_on(driver.open_session(
        plan.clone(),
        open_session_request("restart-open", fixture.resource.clone()),
        fixture.services(),
    ))
    .expect("session opens");
    let persisted = session
        .resume_binding()
        .expect("open returns a resume binding")
        .export_persisted(&plan)
        .expect("binding exports");
    let stored = persisted.as_bytes().to_vec();
    assert!(matches!(
        block_on(session.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    ));

    let persisted = PersistedSessionResumeBinding::from_bytes(stored)
        .expect("stored binding reconstructs after process boundary");
    let restored = SessionResumeBinding::restore_persisted(
        &persisted,
        &plan,
        &fixture.resource,
        &access,
    )
    .expect("matching attachment restores");
    assert_eq!(
        restored.provider_session_ref().as_provider_value(),
        "ses_fixture"
    );
    assert_eq!(
        restored.origin(),
        swallowtail_core::ProviderSessionBindingOrigin::Created
    );

    let requests_before_rejection = server.requests().len();
    let wrong_access = SessionAccessPolicy::ambient_harness(
        swallowtail_core::ResourceAccess::ReadWrite,
    );
    let mismatch = SessionResumeBinding::restore_persisted(
        &persisted,
        &plan,
        &fixture.resource,
        &wrong_access,
    )
    .expect_err("access drift rejects");
    assert_eq!(
        mismatch.kind(),
        SessionResumeBindingPersistenceFailureKind::AttachmentMismatch
    );
    assert_eq!(server.requests().len(), requests_before_rejection);

    let drifted_resource = WorkingResourceRef::new("fixture-workspace-moved").unwrap();
    let mismatch =
        SessionResumeBinding::restore_persisted(&persisted, &plan, &drifted_resource, &access)
            .expect_err("working-resource drift rejects");
    assert_eq!(
        mismatch.kind(),
        SessionResumeBindingPersistenceFailureKind::AttachmentMismatch
    );
    assert_eq!(server.requests().len(), requests_before_rejection);

    let drifted_plan = fixture.plan_with_route(
        DriverRole::InteractiveSession,
        "opencode-anthropic-sonnet-reconfigured",
    );
    let mismatch = SessionResumeBinding::restore_persisted(
        &persisted,
        &drifted_plan,
        &fixture.resource,
        &access,
    )
    .expect_err("model-route drift rejects");
    assert_eq!(
        mismatch.kind(),
        SessionResumeBindingPersistenceFailureKind::AttachmentMismatch
    );
    assert_eq!(server.requests().len(), requests_before_rejection);

    let drifted_plan = fixture.plan_with_versions(DriverRole::InteractiveSession, &["1.18.9"]);
    let mismatch = SessionResumeBinding::restore_persisted(
        &persisted,
        &drifted_plan,
        &fixture.resource,
        &access,
    )
    .expect_err("interface-version drift rejects");
    assert_eq!(
        mismatch.kind(),
        SessionResumeBindingPersistenceFailureKind::AttachmentMismatch
    );
    assert_eq!(server.requests().len(), requests_before_rejection);

    let mut corrupted = persisted.as_bytes().to_vec();
    corrupted[22] ^= 0x01;
    let corruption = PersistedSessionResumeBinding::from_bytes(corrupted)
        .expect_err("corrupted record rejects");
    assert_eq!(
        corruption.kind(),
        SessionResumeBindingPersistenceFailureKind::IntegrityMismatch
    );
    assert_eq!(server.requests().len(), requests_before_rejection);

    let mut session = block_on(driver.resume_session(
        plan.clone(),
        ResumeSessionRequest::from_plan(
            &plan,
            RequestId::new("restart-resume").unwrap(),
            restored,
            fixture.resource.clone(),
            None,
        )
        .unwrap(),
        fixture.services(),
    ))
    .expect("original session resumes");
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("restart-turn").unwrap(),
            OperationContent::new("continue exact session").unwrap(),
        ),
        fixture.services(),
    ))
    .expect("resumed turn starts");
    let mut events = turn.take_events().expect("event stream is available");
    let outcome = turn
        .take_terminal_outcome()
        .expect("terminal outcome is available");
    let outcome = block_on(async {
        while let Some(event) = events.next().await {
            event.expect("compaction lifecycle remains accepted");
        }
        outcome.await
    });
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert!(matches!(
        block_on(turn.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    ));
    assert!(matches!(
        block_on(session.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    ));

    let requests = server.requests();
    assert_eq!(
        requests
            .iter()
            .filter(|request| request.starts_with("POST /session?"))
            .count(),
        1,
        "restart continuation must not create a replacement session"
    );
    assert!(requests.iter().any(|request| {
        request.starts_with("GET /session/ses_fixture?directory=")
    }));
    assert!(requests.iter().any(|request| {
        request.starts_with("POST /session/ses_fixture/prompt_async?directory=")
    }));
}
