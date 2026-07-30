#[test]
fn local_subscription_facade_inherits_harness_auth_without_a_credential_lease() {
    let host_id = ExecutionHostId::new("fixture.run.local-subscription").expect("valid host");
    let preparation_host = FixtureHost::new(Scenario::Version, "0.61.0");
    let prepared = block_on(prepare_claude_agent(
        local_preparation_input(host_id.clone()),
        probe(),
        preparation_host.services_without_credential(host_id.clone()),
    ))
    .expect("locally authenticated Claude Agent prepares");
    let profile = prepared
        .prepare_run(ClaudeAgentRunProfileInput::new(
            RequestId::new("claude-agent-local-subscription-run").expect("valid request"),
            ClaudeAgentModelSelection::new(
                ModelRouteId::new("claude-agent.prepared.route").expect("valid route"),
                ModelRouteRevision::new("1").expect("valid route revision"),
                ModelId::new("claude-sonnet-4-6").expect("valid model"),
            ),
            OperationContent::new("use the local Claude subscription").expect("valid prompt"),
            WorkingResourceRef::new("claude-agent.prepared.workspace").expect("valid resource"),
            None,
        ))
        .expect("local subscription run prepares");

    assert_eq!(
        profile.plan().credential_mechanism(),
        &CredentialMechanism::LocalUnauthenticated
    );
    assert!(profile.plan().credential_reference().is_none());
    assert!(
        !profile
            .plan()
            .requirements()
            .host_services()
            .any(|service| service == HostServiceKind::Credential)
    );

    let operation_host = FixtureHost::new(Scenario::Success, "0.61.0");
    let mut run =
        block_on(profile.start_run(operation_host.services_without_credential(host_id.clone())))
            .expect("local subscription run starts");
    let mut events = run.take_events().expect("events");
    let terminal = run.take_terminal_outcome().expect("terminal");
    let outcome = block_on(async {
        while let Some(event) = events.next().await {
            event.expect("event succeeds");
        }
        terminal.await
    });
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert_eq!(operation_host.credential_acquires(), 0);
    assert_eq!(operation_host.credential_releases(), 0);
    assert_eq!(operation_host.observed_process().environment_count, 1);

    let session_profile = prepared
        .prepare_session(ClaudeAgentSessionProfileInput::new(
            RequestId::new("claude-agent-local-subscription-session").expect("valid request"),
            ClaudeAgentModelSelection::new(
                ModelRouteId::new("claude-agent.prepared.route").expect("valid route"),
                ModelRouteRevision::new("1").expect("valid route revision"),
                ModelId::new("claude-sonnet-4-6").expect("valid model"),
            ),
            WorkingResourceRef::new("claude-agent.prepared.workspace").expect("valid resource"),
            SessionOptions::default(),
        ))
        .expect("local subscription session prepares");
    let session_host = FixtureHost::new(Scenario::Success, "0.61.0");
    let session = block_on(
        session_profile.open_session(session_host.services_without_credential(host_id.clone())),
    )
    .expect("local subscription session opens");
    let binding = session
        .management_binding()
        .expect("session returns a management binding")
        .clone();
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert_eq!(session_host.credential_acquires(), 0);
    assert_eq!(session_host.credential_releases(), 0);

    let delete = prepared
        .prepare_delete_session(ClaudeAgentSessionManagementInput::new(
            RequestId::new("claude-agent-local-subscription-delete").expect("valid request"),
            binding,
        ))
        .expect("local subscription delete prepares");
    let delete_host = FixtureHost::new(Scenario::Success, "0.61.0");
    let outcome = block_on(delete.execute(delete_host.services_without_credential(host_id)))
        .expect("local subscription delete executes");
    assert_eq!(
        outcome.effect().truth(),
        ProviderSessionEffectTruth::Applied
    );
    assert_eq!(delete_host.credential_acquires(), 0);
    assert_eq!(delete_host.credential_releases(), 0);
}

