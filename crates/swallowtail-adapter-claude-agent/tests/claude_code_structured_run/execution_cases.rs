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

