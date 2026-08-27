#[test]
fn zero_overflow_and_unqualified_versions_reject_before_process_work() {
    for rejected in [0_u64, u64::from(u32::MAX) + 1, u64::MAX] {
        let error = ClaudeCodeMaximumTurns::from_u64(rejected).expect_err("value is not admitted");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.claude_code.headless.maximum_turns_invalid"
        );
    }

    let topology = ExecutionTopologyFixture::local();
    let unverified = prepared_at(topology.execution_host_id().clone(), "2.1.242");
    let selection = ClaudeCodeMaximumTurns::from_u64(3).expect("value is admitted");
    let error = unverified
        .prepare_run(
            run_profile_input(topology.working_resource().clone(), "turns-unqualified")
                .with_maximum_turns(selection),
        )
        .expect_err("unqualified version rejects the selection");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.claude_code.headless.preparation.maximum_turns_unqualified"
    );

    // The same unqualified version still runs without a maximum-turn selection.
    let omitted = unverified
        .prepare_run(run_profile_input(
            topology.working_resource().clone(),
            "turns-unqualified-omitted",
        ))
        .expect("omission still prepares on a permitted newer version");
    assert_eq!(omitted.maximum_turns(), None);

    for qualified in ["2.1.220", "2.1.230", "2.1.241"] {
        let integration = prepared_at(topology.execution_host_id().clone(), qualified);
        assert_eq!(
            integration
                .prepare_run(
                    run_profile_input(topology.working_resource().clone(), "turns-qualified")
                        .with_maximum_turns(selection),
                )
                .expect("qualified version admits the selection")
                .maximum_turns(),
            Some(selection)
        );
    }
}

#[test]
fn native_limit_reached_maps_to_provider_failure_with_no_output_and_joined_cleanup() {
    let topology = ExecutionTopologyFixture::local();
    let prepared = prepared(topology.execution_host_id().clone());
    let selected = prepared
        .prepare_run(
            run_profile_input(topology.working_resource().clone(), "turns-reached")
                .with_maximum_turns(ClaudeCodeMaximumTurns::from_u64(1).expect("value is admitted")),
        )
        .expect("Claude Code run prepares");

    // Research 226: reaching the native bound emits one `error_max_turns`
    // result with no `result` field and exits the process with code 1.
    let reached = execute(
        &selected,
        topology.execution_host_id().clone(),
        &fixture("headless-max-turns.jsonl"),
        ProcessExit::new(false, Some(1)),
    );
    assert_status(
        &reached.outcome,
        "swallowtail.claude_code.headless.provider_failed",
        true,
    );
    assert_eq!(
        reached
            .outcome
            .failure()
            .expect("provider failure")
            .diagnostic()
            .failure_classification()
            .origin(),
        swallowtail_core::FailureOrigin::Provider
    );
    assert!(reached.outcome.output().is_none());
    assert!(!reached.events.iter().any(|event| matches!(
        event.kind(),
        RuntimeEventKind::OutputAvailable | RuntimeEventKind::OutputDelta
    )));
    assert!(reached.events.iter().any(|event| matches!(
        event.kind(),
        RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(usage))
            if usage.input_tokens() == Some(14) && usage.output_tokens() == Some(2)
    )));
    assert_eq!(reached.outcome.cleanup(), &CleanupOutcome::Clean);
    assert!(reached.events.iter().any(|event| matches!(
        event.kind(),
        RuntimeEventKind::Activity(activity)
            if activity.kind() == &swallowtail_runtime::ActivityKind::ProviderOwnedTool
    )));
    assert!(!format!("{:?}", reached.events).contains("private fixture match"));

    // Native bound reached is not a harness process failure and not completion.
    assert_ne!(reached.outcome.status(), &TerminalStatus::Completed);
}
