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
    let selection = ClaudeCodeMaximumTurns::from_u64(3).expect("value is admitted");

    // `2.1.242` and `2.1.252` are now route-qualified but still outside
    // Research 226's probed set. `2.1.230` sits inside the semantic window
    // but was never published. `2.1.253` is permitted `UnverifiedNewer`.
    // None may admit the feature.
    for unprobed in ["2.1.242", "2.1.230", "2.1.252", "2.1.253"] {
        let integration = prepared_at(topology.execution_host_id().clone(), unprobed);
        let error = integration
            .prepare_run(
                run_profile_input(topology.working_resource().clone(), "turns-unqualified")
                    .with_maximum_turns(selection),
            )
            .expect_err("unprobed version rejects the selection");
        assert_eq!(
            error.diagnostic().safe().code(),
            "swallowtail.claude_code.headless.preparation.maximum_turns_unqualified"
        );

        // The same version still runs without a maximum-turn selection.
        let omitted = integration
            .prepare_run(run_profile_input(
                topology.working_resource().clone(),
                "turns-unqualified-omitted",
            ))
            .expect("omission still prepares on a permitted version");
        assert_eq!(omitted.maximum_turns(), None);
    }

    for probed in ["2.1.220", "2.1.229", "2.1.231", "2.1.241"] {
        let integration = prepared_at(topology.execution_host_id().clone(), probed);
        assert_eq!(
            integration
                .prepare_run(
                    run_profile_input(topology.working_resource().clone(), "turns-qualified")
                        .with_maximum_turns(selection),
                )
                .expect("probed version admits the selection")
                .maximum_turns(),
            Some(selection)
        );
    }
}

#[test]
fn an_extracted_driver_never_carries_another_runs_bound() {
    let topology = ExecutionTopologyFixture::local();
    let prepared = prepared(topology.execution_host_id().clone());

    let run_a = prepared
        .prepare_run(
            run_profile_input(topology.working_resource().clone(), "turns-a")
                .with_maximum_turns(ClaudeCodeMaximumTurns::from_u64(1).expect("value is admitted")),
        )
        .expect("run A prepares with maximum 1");
    let run_b = prepared
        .prepare_run(
            run_profile_input(topology.working_resource().clone(), "turns-b")
                .with_maximum_turns(
                    ClaudeCodeMaximumTurns::from_u64(30).expect("value is admitted"),
                ),
        )
        .expect("run B prepares with maximum 30");
    let run_c = prepared
        .prepare_run(run_profile_input(
            topology.working_resource().clone(),
            "turns-c",
        ))
        .expect("run C prepares with no selection");

    // Neither `PreflightPlan` nor `StructuredRunRequest` records a bound, so a
    // driver that escaped its own prepared run could be handed another run's
    // plan and silently dispatch the wrong value. Extraction is unbound instead,
    // so the cross-pairings below cannot contradict the plan they run.
    for (donor, host_run, label) in [
        (&run_a, &run_b, "A(1) driver with B(30) plan"),
        (&run_b, &run_a, "B(30) driver with A(1) plan"),
        (&run_a, &run_c, "A(1) driver with C(omitted) plan"),
        (&run_c, &run_a, "C(omitted) driver with A(1) plan"),
    ] {
        let (process, state) = FakeProcessService::with_exit(
            &fixture("headless-complete.jsonl"),
            ProcessExit::new(true, Some(0)),
        );
        let (services, task) = host_services(
            topology.execution_host_id().clone(),
            process,
            Arc::new(PendingTimeService),
        );
        let mut run = block_on(donor.low_level_driver().start_run(
            host_run.plan().clone(),
            host_run.request().clone(),
            services,
        ))
        .expect("low-level run starts");
        let _ = block_on(
            run.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
        assert!(task.joined());
        assert!(
            !state
                .request()
                .arguments
                .iter()
                .any(|argument| argument == "--max-turns"),
            "{label} dispatched a maximum-turn bound"
        );
    }

    // Prepared `start_run` still dispatches each run's own bound exactly.
    for (run, expected) in [(&run_a, Some("1")), (&run_b, Some("30")), (&run_c, None)] {
        let dispatched = execute(
            run,
            topology.execution_host_id().clone(),
            &fixture("headless-complete.jsonl"),
            ProcessExit::new(true, Some(0)),
        );
        match expected {
            Some(value) => assert_eq!(
                dispatched.request.arguments[dispatched.request.arguments.len() - 2..],
                ["--max-turns", value]
            ),
            None => assert!(
                !dispatched
                    .request
                    .arguments
                    .iter()
                    .any(|argument| argument == "--max-turns")
            ),
        }
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
