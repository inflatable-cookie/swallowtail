#[test]
fn prepared_route_executes_exact_local_subscription_invocation_in_both_topologies() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let prepared = prepared(topology.execution_host_id().clone());
        let profile = profile(
            &prepared,
            topology.working_resource().clone(),
            "prepared",
            Some("high"),
        );
        assert_eq!(
            profile.plan().harness_configuration_posture(),
            Some(HarnessConfigurationPosture::Ambient)
        );
        assert_eq!(
            profile.plan().requirements().harness_isolation(),
            Some(HarnessIsolation::AmbientHost)
        );
        assert_eq!(
            profile.request().policy().provider_retention(),
            ProviderRetentionPolicy::Prohibited
        );
        assert_eq!(
            profile.request().policy().harness_mode(),
            Some(HarnessMode::Plan)
        );
        assert!(
            profile
                .plan()
                .requirements()
                .capabilities()
                .any(|requirement| {
                    requirement.capability() == Capability::HarnessModeSelection
                        && requirement.constraints().any(|constraint| {
                            constraint == &CapabilityConstraint::HarnessMode(HarnessMode::Plan)
                        })
                })
        );
        assert_prepared_operation_evidence_matches_plan(
            profile.evidence().operation(),
            profile.plan(),
        );
        assert_eq!(
            profile.evidence().observable_activity().availability(),
            ObservableActivityAvailability::Available
        );

        let evidence = execute(
            &profile,
            topology.execution_host_id().clone(),
            &fixture("headless-complete.jsonl"),
            ProcessExit::new(true, Some(0)),
        );
        assert_eq!(evidence.outcome.status(), &TerminalStatus::Completed);
        assert_eq!(
            evidence.outcome.output().map(OperationContent::as_str),
            Some("fixture result")
        );
        assert!(evidence.events.iter().any(|event| matches!(
            event.kind(),
            RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(usage))
                if usage.input_tokens() == Some(12)
                    && usage.output_tokens() == Some(3)
                    && usage.cache_read_input_tokens() == Some(4)
                    && usage.cache_write_input_tokens() == Some(1)
        )));
        assert_eq!(
            evidence.request.arguments,
            [
                "-p",
                "--input-format",
                "text",
                "--output-format",
                "stream-json",
                "--verbose",
                "--no-session-persistence",
                "--model",
                "claude-opus-5",
                "--effort",
                "high",
                "--permission-mode",
                "plan",
                "--tools",
                "Read,Glob,Grep",
                "--setting-sources",
                "user,project,local",
                "--mcp-config",
                r#"{"mcpServers":{}}"#,
                "--strict-mcp-config",
            ]
        );
        for forbidden in [
            "--bare",
            "--dangerously-skip-permissions",
            "--resume",
            "--continue",
        ] {
            assert!(
                !evidence
                    .request
                    .arguments
                    .iter()
                    .any(|argument| argument == forbidden)
            );
        }
        assert_eq!(
            evidence.request.environments,
            ["claude.fixture.local-subscription-environment"]
        );
        assert_eq!(
            evidence.request.working_resource.as_deref(),
            Some(topology.working_resource().as_host_value())
        );
        assert_eq!(evidence.stdin, b"private Claude fixture prompt");
        assert!(evidence.stdin_closed);
        assert!(
            !format!("{:?}{:?}", evidence.events, evidence.outcome)
                .contains("private Claude fixture prompt")
        );
    }
}

#[test]
fn forked_skill_frames_and_stop_reentry_preserve_stream_and_join() {
    let topology = ExecutionTopologyFixture::local();
    let prepared = prepared_at(topology.execution_host_id().clone(), "2.1.294");
    let profile = profile(
        &prepared,
        topology.working_resource().clone(),
        "forked-skill-stop-reentry",
        None,
    );

    let evidence = execute(
        &profile,
        topology.execution_host_id().clone(),
        &fixture_at("2.1.294", "headless-forked-skill-and-stop.jsonl"),
        ProcessExit::new(true, Some(0)),
    );
    assert_eq!(evidence.outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        evidence.outcome.output().map(OperationContent::as_str),
        Some("fixture final answer")
    );

    let activities = evidence
        .events
        .iter()
        .filter_map(|event| match event.kind() {
            RuntimeEventKind::Activity(activity) => Some(activity),
            _ => None,
        })
        .collect::<Vec<_>>();
    let forked = activities
        .iter()
        .find(|activity| {
            activity.kind() == &ActivityKind::AssistantMessage
                && activity
                    .provider_activity_ref()
                    .is_some_and(|reference| reference.as_provider_value() == "msg_forked_skill")
        })
        .expect("forked assistant frame is projected");
    assert_eq!(
        forked.assistant_phase(),
        Some(swallowtail_runtime::ActivityAssistantPhase::ProviderUnspecified)
    );
    assert!(forked.content().is_none());
    assert_eq!(
        activities
            .iter()
            .filter(|activity| {
                activity.kind() == &ActivityKind::AssistantMessage
                    && activity.assistant_phase()
                        == Some(swallowtail_runtime::ActivityAssistantPhase::Final)
            })
            .count(),
        2
    );
    assert!(activities.iter().all(|activity| {
        activity.phase() == ActivityLifecyclePhase::Completed
    }));
    assert!(evidence.events.iter().any(|event| {
        matches!(event.kind(), RuntimeEventKind::OutputDelta)
            && event.content().is_some_and(|content| content.as_str() == "forked skill update")
    }));

    let usage = evidence
        .events
        .iter()
        .filter_map(|event| match event.kind() {
            RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(usage)) => Some(usage),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(usage.len(), 1);
    assert_eq!(usage[0].input_tokens(), Some(24));
    assert_eq!(usage[0].output_tokens(), Some(8));
    assert_eq!(usage[0].cache_read_input_tokens(), Some(6));
    assert_eq!(usage[0].cache_write_input_tokens(), Some(2));

    assert_eq!(
        argument_after(&evidence.request.arguments, "--permission-mode"),
        "plan"
    );
    assert_eq!(
        argument_after(&evidence.request.arguments, "--tools"),
        "Read,Glob,Grep"
    );
    assert_eq!(
        argument_after(&evidence.request.arguments, "--setting-sources"),
        "user,project,local"
    );
    for forbidden in ["--disable-hooks", "--dangerously-skip-permissions", "--bare"] {
        assert!(!evidence.request.arguments.iter().any(|argument| argument == forbidden));
    }
}

#[test]
fn hook_and_post_rewrite_denial_stays_failed_without_a_tool_success() {
    let topology = ExecutionTopologyFixture::local();
    let prepared = prepared_at(topology.execution_host_id().clone(), "2.1.294");
    let profile = profile(
        &prepared,
        topology.working_resource().clone(),
        "hook-denial",
        None,
    );

    let evidence = execute(
        &profile,
        topology.execution_host_id().clone(),
        &fixture_at("2.1.294", "headless-hook-denied.jsonl"),
        ProcessExit::new(true, Some(0)),
    );
    assert_eq!(evidence.outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        evidence.outcome.output().map(OperationContent::as_str),
        Some("The read was blocked; no file content was returned.")
    );
    let tool_results = evidence
        .events
        .iter()
        .filter_map(|event| match event.kind() {
            RuntimeEventKind::Activity(activity)
                if activity.kind() == &ActivityKind::ProviderOwnedTool =>
            {
                Some(activity)
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(tool_results.len(), 2);
    assert_eq!(tool_results[0].status(), swallowtail_runtime::ActivityStatus::Completed);
    assert_eq!(tool_results[1].status(), swallowtail_runtime::ActivityStatus::Failed);
    assert_eq!(
        tool_results[1]
            .provider_activity_ref()
            .map(|reference| reference.as_provider_value()),
        Some("tool-denied-read")
    );
    assert!(!format!("{:?}{:?}", evidence.events, evidence.outcome)
        .contains("/approved-workspace/fixture.txt"));
    assert_eq!(
        argument_after(&evidence.request.arguments, "--permission-mode"),
        "plan"
    );
    assert_eq!(
        argument_after(&evidence.request.arguments, "--setting-sources"),
        "user,project,local"
    );
    for forbidden in ["--disable-hooks", "--dangerously-skip-permissions", "--bare"] {
        assert!(!evidence.request.arguments.iter().any(|argument| argument == forbidden));
    }
}
