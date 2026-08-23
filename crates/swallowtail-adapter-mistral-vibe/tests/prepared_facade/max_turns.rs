#[test]
fn selected_max_turns_agree_across_input_prepared_run_and_argv() {
    for turns in [1_u8, 8] {
        let host_id =
            ExecutionHostId::new(format!("fixture.prepared.headless.turns.{turns}")).expect("host");
        let discovery = DiscoveryHost::new(MISTRAL_VIBE_RELEASE_VERSION);
        let operation = FixtureHost::scripted([SUCCESS]);
        let mut services = discovery.services(host_id.clone());
        services = services.with_working_resource(
            operation
                .services(host_id.clone())
                .working_resource()
                .expect("resource service")
                .clone(),
        );
        let prepared = block_on(prepare_mistral_vibe_headless(
            preparation_input(host_id.clone()),
            probe(),
            services,
        ))
        .expect("Mistral Vibe headless prepares");
        let selected = MistralVibeMaxTurns::try_new(turns).expect("admitted");
        let run = prepared
            .prepare_run(run_input(&format!("turns-{turns}")).with_max_turns(selected))
            .expect("run prepares");
        assert_eq!(run.max_turns(), Some(selected));
        assert_prepared_operation_evidence_matches_plan(run.evidence(), run.plan());

        let mut handle = block_on(run.start_run(operation.services(host_id))).expect("run starts");
        let _events = block_on(handle.take_events().expect("events").collect::<Vec<_>>())
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
            .expect("events parse");
        let terminal = block_on(handle.take_terminal_outcome().expect("terminal"));
        assert_eq!(terminal.status(), &TerminalStatus::Completed);
        let observed = operation.observed();
        let expected_turns = turns.to_string();
        assert_eq!(
            observed.arguments,
            [
                "--prompt".to_owned(),
                "private fixture prompt".to_owned(),
                "--output".to_owned(),
                "streaming".to_owned(),
                "--max-turns".to_owned(),
                expected_turns,
                "--trust".to_owned(),
                "--agent".to_owned(),
                "plan".to_owned(),
                "--workdir".to_owned(),
                FIXTURE_CWD.to_owned(),
            ]
        );
        for forbidden in [
            "vibe-acp",
            "--continue",
            "--resume",
            "--teleport",
            "--auto-approve",
            "--yolo",
        ] {
            assert!(
                !observed
                    .arguments
                    .iter()
                    .any(|argument| argument == forbidden)
            );
        }
        assert!(operation.stdin().is_empty());
        assert!(operation.stdin_closed());
        assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
        assert!(operation.joined());
    }
}

#[test]
fn omitted_max_turns_stays_unselected_and_keeps_argv_eight() {
    let host_id = ExecutionHostId::new("fixture.prepared.headless.turns.omit").expect("host");
    let discovery = DiscoveryHost::new(MISTRAL_VIBE_RELEASE_VERSION);
    let operation = FixtureHost::scripted([SUCCESS]);
    let mut services = discovery.services(host_id.clone());
    services = services.with_working_resource(
        operation
            .services(host_id.clone())
            .working_resource()
            .expect("resource service")
            .clone(),
    );
    let prepared = block_on(prepare_mistral_vibe_headless(
        preparation_input(host_id.clone()),
        probe(),
        services,
    ))
    .expect("Mistral Vibe headless prepares");
    let run = prepared
        .prepare_run(run_input("omit"))
        .expect("run prepares");
    assert_eq!(run.max_turns(), None);

    let mut handle = block_on(run.start_run(operation.services(host_id))).expect("run starts");
    let _events = block_on(handle.take_events().expect("events").collect::<Vec<_>>())
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .expect("events parse");
    let terminal = block_on(handle.take_terminal_outcome().expect("terminal"));
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert!(
        operation
            .observed()
            .arguments
            .windows(2)
            .any(|pair| pair == ["--max-turns", "8"])
    );
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
}

#[test]
fn selected_max_turns_keep_limit_stderr_as_max_turns_failure() {
    let host_id = ExecutionHostId::new("fixture.prepared.headless.turns.limit").expect("host");
    let discovery = DiscoveryHost::new(MISTRAL_VIBE_RELEASE_VERSION);
    let operation = FixtureHost::with_exit(
        [stderr_chunk(LIMIT_STDERR.as_bytes().to_vec())],
        ProcessExit::new(false, Some(1)),
    );
    let mut services = discovery.services(host_id.clone());
    services = services.with_working_resource(
        operation
            .services(host_id.clone())
            .working_resource()
            .expect("resource service")
            .clone(),
    );
    let prepared = block_on(prepare_mistral_vibe_headless(
        preparation_input(host_id.clone()),
        probe(),
        services,
    ))
    .expect("Mistral Vibe headless prepares");
    let selected = MistralVibeMaxTurns::try_new(1).expect("admitted");
    let run = prepared
        .prepare_run(run_input("limit").with_max_turns(selected))
        .expect("run prepares");
    let mut handle = block_on(run.start_run(operation.services(host_id))).expect("run starts");
    let _events = block_on(handle.take_events().expect("events").collect::<Vec<_>>())
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .expect("events parse");
    let terminal = block_on(handle.take_terminal_outcome().expect("terminal"));
    match terminal.status() {
        TerminalStatus::ProviderFailed(diagnostic) => {
            assert_eq!(
                diagnostic.code(),
                "swallowtail.mistral-vibe.headless.max_turns"
            );
        }
        other => panic!("expected provider failed, got {other:?}"),
    }
    assert!(
        operation
            .observed()
            .arguments
            .windows(2)
            .any(|pair| pair == ["--max-turns", "1"])
    );
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
}
