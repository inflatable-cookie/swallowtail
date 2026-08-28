#[test]
fn prepared_catalogue_uses_qwen_control_protocol_and_joins_the_ephemeral_process() {
    let host_id = ExecutionHostId::new("fixture.qwen.catalogue").expect("valid host");
    let (discovery_process, _) = FakeProcessService::completed("0.19.11\n");
    let (discovery_services, _) = host_services_for(
        host_id.clone(),
        discovery_process,
        Arc::new(PendingTimeService),
    );
    let prepared = block_on(prepare_qwen_headless(
        preparation_input(host_id.clone()),
        probe(),
        discovery_services,
    ))
    .expect("Qwen prepares");
    let catalogue = prepare_qwen_catalogue(
        &prepared,
        QwenCatalogueProfileInput::new(RequestId::new("qwen-catalogue").expect("valid request"))
            .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_000))),
    )
    .expect("catalogue prepares");
    assert_eq!(
        catalogue.evidence().observable_activity().availability(),
        ObservableActivityAvailability::NotApplicable
    );
    let output = concat!(
        "{\"type\":\"control_response\",\"response\":{\"subtype\":\"success\",\"request_id\":\"swallowtail-initialize\",\"response\":{\"subtype\":\"initialize\",\"session_id\":\"fixture\",\"capabilities\":{\"can_get_available_models\":true}}}}\n",
        "{\"type\":\"control_response\",\"response\":{\"subtype\":\"success\",\"request_id\":\"swallowtail-models\",\"response\":{\"subtype\":\"get_available_models\",\"models\":[{\"id\":\"qwen-fixture\",\"label\":\"Qwen Fixture\",\"contextWindowSize\":131072}]}}}\n"
    );
    let (process, state) = FakeProcessService::completed(output);
    let (services, _) = host_services_for(host_id, process, Arc::new(PendingTimeService));
    let models = block_on(catalogue.list_models(services)).expect("catalogue succeeds");

    assert_eq!(models[0].id().as_str(), "qwen-fixture");
    assert!(state.stdin_closed());
    assert!(state.force_stopped());
    assert!(state.waited());
    let request = state.request();
    assert_eq!(
        request.arguments,
        [
            "--input-format",
            "stream-json",
            "--output-format",
            "stream-json",
            "--safe-mode",
            "--approval-mode",
            "default",
        ]
    );
    let stdin = String::from_utf8(state.stdin()).expect("stdin is UTF-8");
    assert!(stdin.contains("\"subtype\":\"initialize\""));
    assert!(stdin.contains("\"subtype\":\"get_available_models\""));
}

#[test]
fn latest_qualified_qwen_binds_its_exact_runtime_stream_version() {
    let host_id = ExecutionHostId::new("fixture.qwen.prepared.latest").expect("valid host");
    let (process, _) = FakeProcessService::completed("0.22.3\n");
    let (services, _) = host_services_for(host_id.clone(), process, Arc::new(PendingTimeService));
    let prepared = block_on(prepare_qwen_headless(
        preparation_input(host_id.clone()),
        probe(),
        services,
    ))
    .expect("latest Qwen prepares");
    let InstalledExecutableCompatibility::Qualified(assessment) =
        prepared.observation().compatibility()
    else {
        panic!("latest Qwen must be qualified");
    };
    assert_eq!(
        assessment.behavior_revision().as_str(),
        "qwen-code.headless.v0.21.15-reasoning-control"
    );

    let profile = prepared
        .prepare_run(QwenRunProfileInput::new(
            RequestId::new("qwen-latest-run").expect("valid request"),
            QwenModelSelection::new(
                ModelRouteId::new("qwen.latest.route").expect("valid route"),
                ModelRouteRevision::new("1").expect("valid route revision"),
                ProviderId::new("alibaba-modelstudio").expect("valid provider"),
                ModelId::new("qwen3-coder-plus").expect("valid model"),
            ),
            OperationContent::new("latest private prompt").expect("valid prompt"),
            WorkingResourceRef::new("qwen.latest.workspace").expect("valid resource"),
            Deadline::at(MonotonicInstant::from_ticks(1_000)),
        ))
        .expect("latest run profile prepares");
    let output = include_str!("../fixtures/qwen-code-v0.19.11/success.jsonl").replace(
        "\"qwen_code_version\":\"0.19.11\"",
        "\"qwen_code_version\":\"0.22.3\"",
    );
    let (process, _) = FakeProcessService::completed(&output);
    let (services, _) = host_services_for(host_id, process, Arc::new(PendingTimeService));
    let mut run = block_on(profile.start_run(services)).expect("latest run starts");
    let terminal = block_on(run.take_terminal_outcome().expect("terminal is available"));
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
}

#[test]
fn later_stable_qwen_is_visible_and_executable_as_unverified_newer() {
    let host_id = ExecutionHostId::new("fixture.qwen.prepared.newer").expect("valid host");
    let (process, _) = FakeProcessService::completed("0.22.4\n");
    let (services, _) = host_services_for(host_id.clone(), process, Arc::new(PendingTimeService));
    let prepared = block_on(prepare_qwen_headless(
        preparation_input(host_id),
        probe(),
        services,
    ))
    .expect("newer Qwen remains executable");
    assert!(matches!(
        prepared.observation().compatibility(),
        InstalledExecutableCompatibility::UnverifiedNewer(_)
    ));
}
