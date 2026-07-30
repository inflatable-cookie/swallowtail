#[test]
fn production_route_preserves_cli_and_host_truth_in_both_topologies() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let id = format!("gemini-success-{}", topology.execution_host_id().as_str());
        let evidence = completed(
            &topology,
            &fixture("success.jsonl", &id),
            ProcessExit::new(true, Some(0)),
            &id,
        );
        assert_eq!(evidence.outcome.status(), &TerminalStatus::Completed);
        assert_eq!(
            evidence.outcome.output().map(|value| value.as_str()),
            Some("fixture answer")
        );
        assert!(evidence.events.iter().any(|event| matches!(
            event.kind(),
            RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(usage))
                if usage.input_tokens() == Some(12)
                    && usage.output_tokens() == Some(2)
                    && usage.cache_read_input_tokens() == Some(3)
                    && usage.cache_miss_input_tokens() == Some(9)
        )));
        assert_redacted(&evidence.events, &evidence.outcome);

        let unknown = completed(
            &topology,
            &fixture("unknown-event.jsonl", "gemini-unknown"),
            ProcessExit::new(true, Some(0)),
            "gemini-unknown",
        );
        assert_eq!(unknown.outcome.status(), &TerminalStatus::Completed);
        assert!(unknown.events.iter().any(|event| matches!(
            event.kind(),
            RuntimeEventKind::Activity(activity)
                if matches!(activity.kind(), swallowtail_runtime::ActivityKind::Unknown(_))
        )));
        assert_redacted(&unknown.events, &unknown.outcome);

        let provider = completed(
            &topology,
            &fixture("provider-failure.jsonl", "gemini-provider-failure"),
            ProcessExit::new(true, Some(0)),
            "gemini-provider-failure",
        );
        assert_status_code(
            &provider.outcome,
            "swallowtail.gemini.headless.provider_failed",
            true,
        );

        let malformed = completed(
            &topology,
            &fixture("malformed.jsonl", "gemini-malformed"),
            ProcessExit::new(true, Some(0)),
            "gemini-malformed",
        );
        assert_status_code(
            &malformed.outcome,
            "swallowtail.gemini.headless.malformed_stream",
            false,
        );

        let incomplete = completed(
            &topology,
            "",
            ProcessExit::new(true, Some(0)),
            "gemini-incomplete",
        );
        assert_status_code(
            &incomplete.outcome,
            "swallowtail.gemini.headless.incomplete_stream",
            false,
        );

        for (exit, code) in [
            (41, "swallowtail.gemini.headless.native_authentication"),
            (42, "swallowtail.gemini.headless.native_input"),
            (44, "swallowtail.gemini.headless.native_sandbox"),
            (52, "swallowtail.gemini.headless.native_configuration"),
            (53, "swallowtail.gemini.headless.native_turn_limit"),
            (54, "swallowtail.gemini.headless.native_tool"),
            (55, "swallowtail.gemini.headless.native_trust"),
            (130, "swallowtail.gemini.headless.process_interrupted"),
            (1, "swallowtail.gemini.headless.process_failed"),
        ] {
            let native = completed(&topology, "", ProcessExit::new(false, Some(exit)), code);
            assert_status_code(&native.outcome, code, true);
        }

        assert_eq!(cancelled(&topology).status(), &TerminalStatus::Cancelled);
        assert_eq!(timed_out(&topology).status(), &TerminalStatus::TimedOut);
    }
}
