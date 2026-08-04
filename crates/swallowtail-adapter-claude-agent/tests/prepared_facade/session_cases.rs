#[test]
fn prepared_sessions_bind_version_access_model_and_ambient_read_policy() {
    for host_value in ["fixture.prepared.local", "fixture.prepared.remote"] {
        let host_id = ExecutionHostId::new(host_value).expect("valid host");
        let preparation_host = FixtureHost::new(Scenario::Version, "0.61.0");
        let prepared = block_on(prepare_claude_agent(
            preparation_input(host_id.clone()),
            probe(),
            preparation_host.services(host_id.clone()),
        ))
        .expect("Claude Agent prepares");
        let profile = prepared
            .prepare_session(ClaudeAgentSessionProfileInput::new(
                RequestId::new("claude-agent-prepared-open").expect("valid request"),
                ClaudeAgentModelSelection::new(
                    ModelRouteId::new("claude-agent.prepared.route").expect("valid route"),
                    ModelRouteRevision::new("1").expect("valid route revision"),
                    ModelId::new("claude-sonnet-4-6").expect("valid model"),
                ),
                WorkingResourceRef::new("claude-agent.prepared.workspace").expect("valid resource"),
                SessionOptions::default().with_reasoning_mode(
                    swallowtail_core::ReasoningMode::new("high").expect("valid reasoning mode"),
                )
                .with_harness_mode(HarnessMode::Plan),
            ))
            .expect("session profile prepares");

        assert_eq!(
            profile
                .evidence()
                .observation()
                .version()
                .version()
                .as_str(),
            "0.61.0"
        );
        assert_eq!(
            profile.plan().requirements().harness_isolation(),
            Some(HarnessIsolation::AmbientHost)
        );
        assert_eq!(
            profile.plan().harness_configuration_posture(),
            Some(HarnessConfigurationPosture::Ambient)
        );
        assert_eq!(
            profile.request().access_policy(),
            &SessionAccessPolicy::ambient_harness(ResourceAccess::Read)
        );
        assert_eq!(
            profile.plan().model_id().map(ModelId::as_str),
            Some("claude-sonnet-4-6")
        );
        assert_prepared_operation_evidence_matches_plan(
            profile.evidence().operation(),
            profile.plan(),
        );
        assert_eq!(
            profile
                .evidence()
                .operation()
                .observable_activity()
                .availability(),
            ObservableActivityAvailability::Available
        );
        let operation_host = FixtureHost::new(Scenario::Success, "0.61.0");
        let session = block_on(profile.open_session(operation_host.services(host_id.clone())))
            .expect("prepared session opens");
        let binding = session
            .management_binding()
            .expect("prepared session returns lifecycle binding")
            .clone();
        assert!(binding.supports(Capability::ProviderNativeSessionClose));
        assert!(binding.supports(Capability::ProviderSessionDelete));
        assert!(session.resume_binding().is_some());
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        let writes = operation_host.writes();
        let config = writes
            .iter()
            .filter(|message| {
                message.get("method").and_then(serde_json::Value::as_str)
                    == Some("session/set_config_option")
            })
            .collect::<Vec<_>>();
        assert!(profile.plan().requirements().capabilities().any(|requirement| {
            requirement.capability() == Capability::HarnessModeSelection
                && requirement
                    .constraints()
                    .any(|constraint| constraint == &CapabilityConstraint::HarnessMode(HarnessMode::Plan))
        }));
        assert_eq!(config.len(), 3);
        assert_eq!(config[0]["params"]["configId"], "model");
        assert_eq!(config[0]["params"]["value"], "claude-sonnet-4-6");
        assert_eq!(config[1]["params"]["configId"], "effort");
        assert_eq!(config[1]["params"]["value"], "high");
        assert_eq!(config[2]["params"]["configId"], "mode");
        assert_eq!(config[2]["params"]["value"], "plan");
        assert!(writes.iter().any(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("session/close")
        }));
        assert_eq!(operation_host.credential_acquires(), 1);
        assert_eq!(operation_host.credential_releases(), 1);
        assert_eq!(operation_host.resource_releases(), 1);

        let delete = prepared
            .prepare_delete_session(ClaudeAgentSessionManagementInput::new(
                RequestId::new(format!("claude-agent-delete-{host_value}"))
                    .expect("valid delete request"),
                binding,
            ))
            .expect("qualified Claude Agent delete prepares");
        assert_prepared_operation_evidence_matches_plan(
            delete.evidence().operation(),
            delete.plan().preflight(),
        );
        let delete_host = FixtureHost::new(Scenario::Success, "0.61.0");
        let outcome = block_on(delete.execute(delete_host.services(host_id)))
            .expect("prepared Claude Agent delete executes");
        assert_eq!(
            outcome.effect().truth(),
            ProviderSessionEffectTruth::Applied
        );
        assert_eq!(
            outcome.effect().confirmed_deletion_strength(),
            Some(ProviderSessionDeletionStrength::ProviderDataDeleted)
        );
        assert_eq!(
            outcome.effect().affected_scope(),
            Some(ProviderSessionAffectedScope::ProviderDefinedDescendants)
        );
        let writes = delete_host.writes();
        assert!(writes.iter().any(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("session/delete")
        }));
        assert!(!writes.iter().any(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("session/new")
        }));
        assert_eq!(delete_host.credential_acquires(), 1);
        assert_eq!(delete_host.credential_releases(), 1);
        assert_eq!(delete_host.resource_releases(), 1);
    }
}

#[test]
fn prepared_session_load_and_resume_preserve_replay_and_attachment_truth() {
    let host_id = ExecutionHostId::new("fixture.prepared.continuity").expect("valid host");
    let preparation_host = FixtureHost::new(Scenario::Version, "0.61.0");
    let prepared = block_on(prepare_claude_agent(
        preparation_input(host_id.clone()),
        probe(),
        preparation_host.services(host_id.clone()),
    ))
    .expect("Claude Agent prepares");
    let profile = prepared
        .prepare_session(ClaudeAgentSessionProfileInput::new(
            RequestId::new("claude-agent-continuity-open").expect("valid request"),
            ClaudeAgentModelSelection::new(
                ModelRouteId::new("claude-agent.prepared.route").expect("valid route"),
                ModelRouteRevision::new("1").expect("valid route revision"),
                ModelId::new("claude-sonnet-4-6").expect("valid model"),
            ),
            WorkingResourceRef::new("claude-agent.prepared.workspace").expect("valid resource"),
            SessionOptions::default(),
        ))
        .expect("session profile prepares");

    let open_host = FixtureHost::new(Scenario::Success, "0.61.0");
    let opened = block_on(profile.open_session(open_host.services(host_id.clone())))
        .expect("prepared session opens");
    let binding = opened
        .resume_binding()
        .expect("prepared session returns resume binding")
        .clone();
    assert_eq!(block_on(opened.close()), CleanupOutcome::Clean);

    let load_host = FixtureHost::new(Scenario::Success, "0.61.0");
    let loaded = block_on(
        profile
            .load_session(
                RequestId::new("claude-agent-continuity-load").expect("valid request"),
                binding.clone(),
                load_host.services(host_id.clone()),
            )
            .expect("prepared load operation derives"),
    )
    .expect("prepared session loads");
    assert_eq!(
        loaded
            .replay()
            .map(swallowtail_runtime::SessionReplayItem::sequence)
            .collect::<Vec<_>>(),
        [0, 1]
    );
    let (_, loaded_handle) = loaded.into_parts();
    assert_eq!(
        loaded_handle
            .management_binding()
            .expect("loaded session returns management authority")
            .origin(),
        swallowtail_core::ProviderSessionBindingOrigin::Loaded
    );
    assert_eq!(block_on(loaded_handle.close()), CleanupOutcome::Clean);

    let recovery_host = FixtureHost::new(Scenario::Success, "0.61.0");
    let restoration = profile
        .prepare_working_state_restoration(
            RequestId::new("claude-agent-continuity-recovery").expect("valid request"),
            binding.clone(),
            RuntimeTurnId::new("claude-agent-interrupted-turn").expect("valid turn"),
        )
        .expect("working-state restoration prepares");
    assert_eq!(
        restoration.method(),
        WorkingStateRestorationMethod::ProviderSessionContinuationRecovery
    );
    let recovered = block_on(restoration.restore(recovery_host.services(host_id.clone())))
        .expect("working-state restoration loads the exact session");
    let WorkingStateRestorationOutcome::SessionRecovered(recovered) = recovered else {
        panic!("Claude Agent ACP must report continuation recovery");
    };
    assert_eq!(
        recovered.interrupted_turn_id().as_str(),
        "claude-agent-interrupted-turn"
    );
    assert_eq!(
        recovered
            .replay()
            .map(swallowtail_runtime::SessionReplayItem::sequence)
            .collect::<Vec<_>>(),
        [0, 1]
    );
    let (_, loaded) = recovered.into_parts();
    let (_, recovered_handle) = loaded.into_parts();
    assert_eq!(
        recovered_handle
            .management_binding()
            .expect("recovered session returns management authority")
            .origin(),
        swallowtail_core::ProviderSessionBindingOrigin::Loaded
    );
    assert_eq!(block_on(recovered_handle.close()), CleanupOutcome::Clean);
    assert!(recovery_host.writes().iter().any(|message| {
        message.get("method").and_then(serde_json::Value::as_str) == Some("session/load")
    }));

    let resume_host = FixtureHost::new(Scenario::Success, "0.61.0");
    let resumed = block_on(
        profile
            .resume_session(
                RequestId::new("claude-agent-continuity-resume").expect("valid request"),
                binding,
                resume_host.services(host_id),
            )
            .expect("prepared resume operation derives"),
    )
    .expect("prepared session resumes");
    assert_eq!(
        resumed
            .management_binding()
            .expect("resumed session returns management authority")
            .origin(),
        swallowtail_core::ProviderSessionBindingOrigin::Resumed
    );
    assert_eq!(block_on(resumed.close()), CleanupOutcome::Clean);
}

#[test]
fn unsupported_options_fail_before_session_process_effects() {
    let host_id = ExecutionHostId::new("fixture.prepared.options").expect("valid host");
    let preparation_host = FixtureHost::new(Scenario::Version, "0.61.0");
    let prepared = block_on(prepare_claude_agent(
        preparation_input(host_id.clone()),
        probe(),
        preparation_host.services(host_id),
    ))
    .expect("Claude Agent prepares");
    let result = prepared.prepare_session(ClaudeAgentSessionProfileInput::new(
        RequestId::new("claude-agent-options").expect("valid request"),
        ClaudeAgentModelSelection::new(
            ModelRouteId::new("claude-agent.prepared.route").expect("valid route"),
            ModelRouteRevision::new("1").expect("valid route revision"),
            ModelId::new("claude-sonnet-4-6").expect("valid model"),
        ),
        WorkingResourceRef::new("claude-agent.prepared.workspace").expect("valid resource"),
        SessionOptions::default().with_developer_instructions(
            OperationContent::new("unsupported developer instruction").expect("valid content"),
        ),
    ));
    assert!(result.is_err());

    let result = prepared.prepare_session(ClaudeAgentSessionProfileInput::new(
        RequestId::new("claude-agent-unsupported-reasoning").expect("valid request"),
        ClaudeAgentModelSelection::new(
            ModelRouteId::new("claude-agent.prepared.route").expect("valid route"),
            ModelRouteRevision::new("1").expect("valid route revision"),
            ModelId::new("claude-sonnet-4-6").expect("valid model"),
        ),
        WorkingResourceRef::new("claude-agent.prepared.workspace").expect("valid resource"),
        SessionOptions::default().with_reasoning_mode(
            swallowtail_core::ReasoningMode::new("ultra").expect("valid reasoning mode"),
        ),
    ));
    assert!(result.is_err());
}
