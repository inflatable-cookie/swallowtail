#[test]
fn plan_agrees_across_input_capability_policy_and_argv() {
    let host_id = ExecutionHostId::new("fixture.prepared.headless.plan").expect("host");
    let discovery = DiscoveryHost::new(CLINE_PACKAGE_VERSION);
    let operation = FixtureHost::scripted([SUCCESS]);
    let mut services = discovery.services(host_id.clone());
    services = services.with_working_resource(
        operation
            .services(host_id.clone())
            .working_resource()
            .expect("resource service")
            .clone(),
    );
    let prepared = block_on(prepare_cline_headless(
        preparation_input(host_id.clone()),
        probe(),
        services,
    ))
    .expect("Cline headless prepares");
    let input = run_input("plan").with_harness_mode(HarnessMode::Plan);
    assert_eq!(input.harness_mode(), Some(HarnessMode::Plan));
    let run = prepared.prepare_run(input).expect("plan run prepares");
    assert_prepared_operation_evidence_matches_plan(run.evidence(), run.plan());
    assert_eq!(run.harness_mode(), Some(HarnessMode::Plan));
    assert_eq!(
        run.request().policy().harness_mode(),
        Some(HarnessMode::Plan)
    );
    assert!(
        run.plan().requirements().capabilities().any(|required| {
            required.capability() == Capability::HarnessModeSelection
                && required.constraints().any(|constraint| {
                    *constraint == CapabilityConstraint::HarnessMode(HarnessMode::Plan)
                })
        }),
        "prepared Plan requires HarnessModeSelection(Plan)"
    );

    let mut handle = block_on(run.start_run(operation.services(host_id))).expect("plan run starts");
    let events = block_on(handle.take_events().expect("events").collect::<Vec<_>>())
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .expect("events parse");
    let terminal = block_on(handle.take_terminal_outcome().expect("terminal"));
    assert!(!events.is_empty());
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert!(!format!("{events:?}").contains("private fixture prompt"));
    let observed = operation.observed();
    assert_eq!(
        observed.arguments,
        [
            "--json",
            "--auto-approve",
            "false",
            "--plan",
            "-c",
            FIXTURE_CWD,
            "private fixture prompt"
        ]
    );
    for forbidden in ["--acp", "--id", "--yolo", "--zen", "-p"] {
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
