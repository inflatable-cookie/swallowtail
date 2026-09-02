use super::*;
use std::sync::Arc;
use swallowtail_core::{ProviderSessionActivityEvidence, ProviderSessionEffectTruth};
use swallowtail_runtime::CancellationControl;

#[test]
fn qualified_behavior_segments_share_one_delete_and_cleanup_contract() {
    for version in [
        "0.53.0", "0.54.0", "0.60.0", "0.61.0", "0.62.0", "0.63.0", "0.64.0", "0.69.0", "0.70.0",
        "0.71.0", "0.72.0", "0.73.0",
    ] {
        let (prepared, binding, host_id) = prepared_binding(version, version);
        let delete = prepared
            .prepare_delete_session(ClaudeAgentSessionManagementInput::new(
                request(&format!("qualified-delete-{version}")),
                binding,
            ))
            .expect("qualified deletion prepares");
        assert_eq!(
            delete.plan().agreement().activity(),
            ProviderSessionActivityEvidence::CallerAssertedInactive
        );
        let host = FixtureHost::new(Scenario::Success, version);
        let outcome =
            block_on(delete.execute(host.services(host_id))).expect("qualified deletion executes");
        assert_eq!(
            outcome.effect().truth(),
            ProviderSessionEffectTruth::Applied
        );
        assert_eq!(
            host.cleanup_events(),
            vec!["process_joined", "resource_released", "credential_released"]
        );
    }
}

#[test]
fn unpublished_exclusion_and_missing_capability_stop_before_delete_dispatch() {
    let host_id = ExecutionHostId::new("fixture.prepared.excluded").expect("valid host");
    let host = FixtureHost::new(Scenario::Version, "0.58.0");
    let failure = block_on(prepare_claude_agent(
        preparation_input(host_id.clone()),
        probe(),
        host.services(host_id),
    ))
    .expect_err("excluded package version must not prepare");
    assert_eq!(
        failure.diagnostic().safe().code(),
        "swallowtail.claude_agent.preparation.discovery_rejected"
    );

    let (prepared, binding, host_id) = prepared_binding("0.61.0", "capability-drift");
    let delete = prepared
        .prepare_delete_session(ClaudeAgentSessionManagementInput::new(
            request("missing-delete-capability"),
            binding,
        ))
        .expect("qualified binding prepares");
    let host = FixtureHost::new(Scenario::LifecycleDrift, "0.61.0");
    let failure = block_on(delete.execute(host.services(host_id)))
        .expect_err("missing negotiated delete capability fails");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.claude_agent.acp.lifecycle_capability_drift"
    );
    assert!(!saw(&host, "session/delete"));
    assert_eq!(host.credential_releases(), 1);
    assert_eq!(host.resource_releases(), 1);
}

#[test]
fn missing_provider_failure_disconnect_and_malformed_reply_remain_unconfirmed() {
    for (scenario, suffix) in [
        (Scenario::DeleteMissing, "missing"),
        (Scenario::DeleteProviderFailure, "provider-failure"),
        (Scenario::DeleteDisconnect, "disconnect"),
        (Scenario::DeleteMalformed, "malformed"),
    ] {
        let (prepared, binding, host_id) = prepared_binding("0.61.0", suffix);
        let delete = prepared
            .prepare_delete_session(ClaudeAgentSessionManagementInput::new(
                request(&format!("delete-{suffix}")),
                binding,
            ))
            .expect("qualified deletion prepares");
        let host = FixtureHost::new(scenario, "0.61.0");
        let outcome =
            block_on(delete.execute(host.services(host_id))).expect("attempt returns effect truth");
        assert_eq!(
            outcome.effect().truth(),
            ProviderSessionEffectTruth::UnconfirmedAfterEffect
        );
        assert_eq!(outcome.effect().confirmed_deletion_strength(), None);
        assert!(outcome.diagnostic().is_some());
        assert!(!format!("{outcome:?}").contains("private"));
        assert_eq!(host.credential_releases(), 1);
        assert_eq!(host.resource_releases(), 1);
    }
}

#[test]
fn cancellation_and_deadline_preserve_effect_boundary_and_release_access() {
    let (prepared, binding, host_id) = prepared_binding("0.61.0", "cancel-before");
    let delete = prepared
        .prepare_delete_session(ClaudeAgentSessionManagementInput::new(
            request("delete-cancel-before"),
            binding,
        ))
        .expect("deletion prepares");
    block_on(delete.request().cancellation().request()).expect("cancellation is accepted");
    let host = FixtureHost::new(Scenario::DeletePending, "0.61.0");
    let outcome = block_on(delete.execute(host.services(host_id))).expect("cancelled outcome");
    assert_eq!(
        outcome.effect().truth(),
        ProviderSessionEffectTruth::FailedBeforeEffect
    );
    assert!(!saw(&host, "initialize"));
    assert_eq!(host.credential_acquires(), 0);

    assert_after_dispatch_cancellation();
    assert_after_dispatch_deadline();
}

fn assert_after_dispatch_cancellation() {
    let (prepared, binding, host_id) = prepared_binding("0.61.0", "cancel-after");
    let delete = prepared
        .prepare_delete_session(ClaudeAgentSessionManagementInput::new(
            request("delete-cancel-after"),
            binding,
        ))
        .expect("deletion prepares");
    let cancellation = Arc::clone(delete.request().cancellation());
    let host = FixtureHost::new(Scenario::DeletePending, "0.61.0");
    let operation_host = host.clone();
    let execution = std::thread::spawn(move || {
        block_on(delete.execute(operation_host.services(host_id))).expect("cancelled outcome")
    });
    host.wait_for_write("session/delete");
    block_on(cancellation.request()).expect("cancellation is accepted");
    let outcome = execution.join().expect("execution joins");
    assert_unconfirmed_and_released(&host, &outcome);
}

fn assert_after_dispatch_deadline() {
    let (prepared, binding, host_id) = prepared_binding("0.61.0", "deadline-after");
    let delete = prepared
        .prepare_delete_session(
            ClaudeAgentSessionManagementInput::new(request("delete-deadline-after"), binding)
                .with_deadline(Deadline::at(MonotonicInstant::from_ticks(100))),
        )
        .expect("deadline-bound deletion prepares");
    let host = FixtureHost::new(Scenario::DeletePending, "0.61.0").with_deadline_after_waits(2);
    let outcome = block_on(delete.execute(host.services(host_id))).expect("deadline outcome");
    assert!(saw(&host, "session/delete"));
    assert_unconfirmed_and_released(&host, &outcome);
}

fn assert_unconfirmed_and_released(
    host: &FixtureHost,
    outcome: &swallowtail_runtime::ProviderSessionManagementOutcome,
) {
    assert_eq!(
        outcome.effect().truth(),
        ProviderSessionEffectTruth::UnconfirmedAfterEffect
    );
    assert_eq!(host.credential_releases(), 1);
    assert_eq!(host.resource_releases(), 1);
    assert_eq!(
        host.cleanup_events(),
        vec!["process_joined", "resource_released", "credential_released"]
    );
}

fn prepared_binding(
    version: &str,
    suffix: &str,
) -> (
    swallowtail_adapter_claude_agent::ClaudeAgentPreparedIntegration,
    swallowtail_runtime::ProviderSessionManagementBinding,
    ExecutionHostId,
) {
    let host_id = ExecutionHostId::new(format!("fixture.prepared.{suffix}")).expect("valid host");
    let preparation_host = FixtureHost::new(Scenario::Version, version);
    let prepared = block_on(prepare_claude_agent(
        preparation_input(host_id.clone()),
        probe(),
        preparation_host.services(host_id.clone()),
    ))
    .expect("Claude Agent prepares");
    let profile = prepared
        .prepare_session(ClaudeAgentSessionProfileInput::new(
            request(&format!("open-{suffix}")),
            ClaudeAgentModelSelection::new(
                ModelRouteId::new("claude-agent.prepared.route").expect("valid route"),
                ModelRouteRevision::new("1").expect("valid revision"),
                ModelId::new("claude-sonnet-4-6").expect("valid model"),
            ),
            WorkingResourceRef::new("claude-agent.prepared.workspace").expect("valid resource"),
            SessionOptions::default(),
        ))
        .expect("session prepares");
    let session_host = FixtureHost::new(Scenario::Success, version);
    let session = block_on(profile.open_session(session_host.services(host_id.clone())))
        .expect("session opens");
    let binding = session
        .management_binding()
        .expect("management binding exists")
        .clone();
    assert_eq!(
        block_on(session.close(
            session_host.cleanup_request(),
            session_host.services(host_id.clone()),
        )),
        CleanupOutcome::Clean
    );
    (prepared, binding, host_id)
}

fn request(value: &str) -> RequestId {
    RequestId::new(value).expect("valid request")
}

fn saw(host: &FixtureHost, method: &str) -> bool {
    host.writes()
        .iter()
        .any(|message| message.get("method").and_then(serde_json::Value::as_str) == Some(method))
}
