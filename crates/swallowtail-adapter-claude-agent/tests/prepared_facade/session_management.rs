use super::*;

#[path = "session_management/conformance.rs"]
mod conformance;

#[test]
fn unverified_newer_delete_requires_explicit_acceptance_and_close_is_not_promoted() {
    let host_id = ExecutionHostId::new("fixture.prepared.unverified").expect("valid host");
    let preparation_host = FixtureHost::new(Scenario::Version, "0.65.0");
    let prepared = block_on(prepare_claude_agent(
        preparation_input(host_id.clone()),
        probe(),
        preparation_host.services(host_id.clone()),
    ))
    .expect("unverified newer Claude Agent prepares visibly");
    assert!(!prepared.observation().is_qualified());
    let profile = prepared
        .prepare_session(ClaudeAgentSessionProfileInput::new(
            RequestId::new("claude-agent-unverified-open").expect("valid request"),
            ClaudeAgentModelSelection::new(
                ModelRouteId::new("claude-agent.prepared.route").expect("valid route"),
                ModelRouteRevision::new("1").expect("valid route revision"),
                ModelId::new("claude-sonnet-4-6").expect("valid model"),
            ),
            WorkingResourceRef::new("claude-agent.prepared.workspace").expect("valid resource"),
            SessionOptions::default(),
        ))
        .expect("unverified session profile prepares");
    let session_host = FixtureHost::new(Scenario::Success, "0.65.0");
    let session = block_on(profile.open_session(session_host.services(host_id.clone())))
        .expect("unverified session opens");
    let binding = session
        .management_binding()
        .expect("unverified binding remains visible")
        .clone();
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert!(!session_host.writes().iter().any(|message| {
        message.get("method").and_then(serde_json::Value::as_str) == Some("session/close")
    }));

    let rejected = prepared.prepare_delete_session(ClaudeAgentSessionManagementInput::new(
        RequestId::new("claude-agent-unverified-delete-rejected").expect("valid request"),
        binding.clone(),
    ));
    assert!(rejected.is_err());
    let accepted = prepared
        .prepare_delete_session(
            ClaudeAgentSessionManagementInput::new(
                RequestId::new("claude-agent-unverified-delete-accepted").expect("valid request"),
                binding,
            )
            .allow_unverified_newer(),
        )
        .expect("explicit unverified-newer acceptance prepares deletion");
    let delete_host = FixtureHost::new(Scenario::Success, "0.65.0");
    let outcome = block_on(accepted.execute(delete_host.services(host_id)))
        .expect("accepted unverified deletion executes latest mapped behavior");
    assert_eq!(
        outcome.effect().truth(),
        ProviderSessionEffectTruth::Applied
    );
}
