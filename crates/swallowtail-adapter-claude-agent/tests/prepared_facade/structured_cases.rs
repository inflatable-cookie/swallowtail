#[test]
fn prepared_structured_run_binds_one_prompt_and_durable_retention_on_both_hosts() {
    for host_value in ["fixture.run.local", "fixture.run.remote-authoritative"] {
        let host_id = ExecutionHostId::new(host_value).expect("valid host");
        let preparation_host = FixtureHost::new(Scenario::Version, "0.61.0");
        let prepared = block_on(prepare_claude_agent(
            preparation_input(host_id.clone()),
            probe(),
            preparation_host.services(host_id.clone()),
        ))
        .expect("Claude Agent prepares");
        let profile = prepared
            .prepare_run(
                ClaudeAgentRunProfileInput::new(
                    RequestId::new(format!("claude-agent-run-{host_value}"))
                        .expect("valid request"),
                    ClaudeAgentModelSelection::new(
                        ModelRouteId::new("claude-agent.prepared.route").expect("valid route"),
                        ModelRouteRevision::new("1").expect("valid route revision"),
                        ModelId::new("claude-sonnet-4-6").expect("valid model"),
                    ),
                    OperationContent::new("one private prepared prompt").expect("valid prompt"),
                    WorkingResourceRef::new("claude-agent.prepared.workspace")
                        .expect("valid resource"),
                    None,
                )
                .with_reasoning_mode(
                    swallowtail_core::ReasoningMode::new("high").expect("valid reasoning mode"),
                ),
            )
            .expect("structured run prepares");
        assert_eq!(
            profile.plan().requirements().driver_role(),
            swallowtail_core::DriverRole::StructuredRun
        );
        assert!(
            profile
                .plan()
                .requirements()
                .capabilities()
                .any(|required| {
                    required.capability() == Capability::WorkingResource
                        && required.constraints().any(|constraint| {
                            constraint
                                == &CapabilityConstraint::ResourceAccess(ResourceAccess::ReadWrite)
                        })
                })
        );
        assert_eq!(
            profile.request().policy().provider_retention(),
            ProviderRetentionPolicy::DurableAllowed
        );
        assert_eq!(
            profile
                .request()
                .policy()
                .reasoning_mode()
                .map(swallowtail_core::ReasoningMode::as_str),
            Some("high")
        );
        assert_prepared_operation_evidence_matches_plan(
            profile.evidence().operation(),
            profile.plan(),
        );
        let activity_profile = profile.evidence().operation().observable_activity().clone();

        let operation_host = FixtureHost::new(Scenario::Success, "0.61.0");
        let mut run = block_on(profile.start_run(operation_host.services(host_id)))
            .expect("structured run starts");
        assert!(run.provider_run_ref().is_none());
        assert!(run.take_callbacks().is_some());
        let mut events = run.take_events().expect("events");
        let terminal = run.take_terminal_outcome().expect("terminal");
        let (observed_events, outcome) = block_on(async {
            let mut observed_events = Vec::new();
            while let Some(event) = events.next().await {
                observed_events.push(event.expect("event succeeds"));
            }
            (observed_events, terminal.await)
        });
        assert_observable_activity_trace(&activity_profile, &observed_events);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
        let writes = operation_host.writes();
        let config = writes
            .iter()
            .filter(|message| {
                message.get("method").and_then(serde_json::Value::as_str)
                    == Some("session/set_config_option")
            })
            .collect::<Vec<_>>();
        assert_eq!(config.len(), 2);
        assert_eq!(config[0]["params"]["configId"], "model");
        assert_eq!(config[1]["params"]["configId"], "effort");
        assert_eq!(config[1]["params"]["value"], "high");
        assert_eq!(
            writes
                .iter()
                .filter(|message| {
                    message.get("method").and_then(serde_json::Value::as_str)
                        == Some("session/prompt")
                })
                .count(),
            1
        );
        assert!(writes.iter().any(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("session/close")
        }));
        assert!(!writes.iter().any(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("session/delete")
        }));
        assert_eq!(operation_host.credential_releases(), 1);
        assert_eq!(operation_host.resource_releases(), 1);
    }
}

#[test]
fn prepared_structured_run_can_opt_into_operation_owned_session_cleanup() {
    for scenario in [
        Scenario::Success,
        Scenario::Cancellation,
        Scenario::RunDeleteDisconnect,
    ] {
        let host_id = ExecutionHostId::new("fixture.run.owned-cleanup").expect("valid host");
        let preparation_host = FixtureHost::new(Scenario::Version, "0.61.0");
        let prepared = block_on(prepare_claude_agent(
            preparation_input(host_id.clone()),
            probe(),
            preparation_host.services(host_id.clone()),
        ))
        .expect("Claude Agent prepares");
        let profile = prepared
            .prepare_run(
                ClaudeAgentRunProfileInput::new(
                    RequestId::new("claude-agent-owned-cleanup").expect("valid request"),
                    ClaudeAgentModelSelection::new(
                        ModelRouteId::new("claude-agent.prepared.route").expect("valid route"),
                        ModelRouteRevision::new("1").expect("valid route revision"),
                        ModelId::new("claude-sonnet-4-6").expect("valid model"),
                    ),
                    OperationContent::new("one temporary prompt").expect("valid prompt"),
                    WorkingResourceRef::new("claude-agent.prepared.workspace")
                        .expect("valid resource"),
                    None,
                )
                .with_owned_session_cleanup(),
            )
            .expect("temporary structured run prepares");
        assert_eq!(
            profile.request().policy().provider_retention(),
            ProviderRetentionPolicy::TemporaryAllowed
        );
        assert!(
            profile
                .plan()
                .requirements()
                .capabilities()
                .any(|required| {
                    required.capability() == Capability::OwnedRemoteResourceDeletion
                        && required
                            .constraints()
                            .eq([&CapabilityConstraint::OwnedRemoteResource(
                                OwnedRemoteResourceKind::Session,
                            )])
                })
        );

        let operation_host = FixtureHost::new(scenario, "0.61.0");
        let mut run = block_on(profile.start_run(operation_host.services(host_id.clone())))
            .expect("temporary structured run starts");
        if scenario == Scenario::Cancellation {
            block_on(run.cancellation().request()).expect("run cancellation is accepted");
        }
        let mut events = run.take_events().expect("events");
        let terminal = run.take_terminal_outcome().expect("terminal");
        let outcome = block_on(async {
            while let Some(event) = events.next().await {
                event.expect("event succeeds");
            }
            terminal.await
        });
        assert_eq!(
            outcome.status(),
            if scenario == Scenario::Cancellation {
                &TerminalStatus::Cancelled
            } else {
                &TerminalStatus::Completed
            }
        );
        assert_eq!(
            outcome.remote_resource_deletion(OwnedRemoteResourceKind::Session),
            Some(if scenario == Scenario::RunDeleteDisconnect {
                RemoteResourceDeletionOutcome::Unconfirmed
            } else {
                RemoteResourceDeletionOutcome::Confirmed
            })
        );
        assert_eq!(
            matches!(outcome.cleanup(), CleanupOutcome::Degraded(_)),
            scenario == Scenario::RunDeleteDisconnect
        );
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
        let writes = operation_host.writes();
        let close = writes
            .iter()
            .position(|message| message["method"] == "session/close")
            .expect("native close dispatched");
        let delete = writes
            .iter()
            .position(|message| message["method"] == "session/delete")
            .expect("owned delete dispatched");
        assert!(close < delete);
        assert_eq!(operation_host.credential_releases(), 1);
        assert_eq!(operation_host.resource_releases(), 1);
    }
}
