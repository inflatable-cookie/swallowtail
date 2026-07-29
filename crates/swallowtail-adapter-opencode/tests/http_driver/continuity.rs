const CONTINUITY_CORPUS: &str =
    include_str!("../fixtures/opencode-v1.14.48-v1.18.4/session-continuity.json");

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
