use crate::fixtures::{AGENT_HOST, agent_session};
use crate::naming::{rows, semantic_id, source};
use crate::support::{FixtureHost, Scenario};
use futures_executor::block_on;
use swallowtail_adapter_claude_agent::ClaudeAgentProjectionOpenFailure;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{CleanupOutcome, ConsumerRouteValueDomain};

const PREPARED: &str = "claude-agent.projection.prepared";
const ACTIVE: &str = "claude-agent.projection.active";

#[test]
fn matching_reasoning_opens_with_exact_provider_effective_state() {
    let fixture = FixtureHost::new(Scenario::Success, "0.61.0");
    let outcome = block_on(
        agent_session(Some("low"), false).open_session_with_projection(
            source(PREPARED),
            source(ACTIVE),
            fixture.cleanup_request(),
            fixture.services(ExecutionHostId::new(AGENT_HOST).expect("host is valid")),
        ),
    )
    .map_err(|failure| failure.failure().diagnostic().code())
    .expect("matching acknowledgement opens");
    assert!(rows(outcome.contribution()).contains("feature.active-session-reasoning-ack"));
    let acknowledgement = outcome
        .contribution()
        .active_session_rows()
        .find(|row| semantic_id(row.identity()) == "feature.active-session-reasoning-ack")
        .expect("acknowledgement row exists");
    assert!(acknowledgement.state_support().provider_effective());
    assert!(!acknowledgement.state_support().rejected());
    assert_eq!(acknowledgement.source().id().as_str(), ACTIVE);
    let ConsumerRouteValueDomain::Enumerated(domain) = acknowledgement
        .control_value()
        .expect("acknowledgement carries a value")
        .domain()
    else {
        panic!("acknowledgement value is enumerated");
    };
    assert_eq!(
        domain
            .values()
            .map(|value| value.as_str())
            .collect::<Vec<_>>(),
        ["low"]
    );
    let (session, _) = outcome.into_parts();
    assert_eq!(
        block_on(session.close(
            fixture.cleanup_request(),
            fixture.services(ExecutionHostId::new(AGENT_HOST).expect("host is valid")),
        )),
        CleanupOutcome::Clean
    );
}

#[test]
fn exact_advertised_mismatch_returns_rejected_state_and_no_session() {
    let fixture = FixtureHost::new(Scenario::ReasoningMismatchAdvertised, "0.61.0");
    let failure = match block_on(
        agent_session(Some("low"), false).open_session_with_projection(
            source(PREPARED),
            source(ACTIVE),
            fixture.cleanup_request(),
            fixture.services(ExecutionHostId::new(AGENT_HOST).expect("host is valid")),
        ),
    ) {
        Ok(_) => panic!("different admitted acknowledgement must reject"),
        Err(failure) => failure,
    };
    let ClaudeAgentProjectionOpenFailure::Rejected { .. } = &failure else {
        panic!("exact mismatch carries rejected projection");
    };
    let acknowledgement = failure
        .rejected_contribution()
        .expect("rejected contribution exists")
        .active_session_rows()
        .find(|row| semantic_id(row.identity()) == "feature.active-session-reasoning-ack")
        .expect("rejected acknowledgement exists");
    assert!(acknowledgement.state_support().rejected());
    assert!(!acknowledgement.state_support().provider_effective());
    let ConsumerRouteValueDomain::Enumerated(domain) = acknowledgement
        .control_value()
        .expect("rejected value exists")
        .domain()
    else {
        panic!("rejected value is enumerated");
    };
    assert_eq!(
        domain
            .values()
            .map(|value| value.as_str())
            .collect::<Vec<_>>(),
        ["high"]
    );
    assert_eq!(fixture.credential_releases(), 1);
    assert_eq!(fixture.resource_releases(), 1);
}

#[test]
fn ambiguous_or_unbounded_confirmations_return_runtime_without_contribution() {
    for scenario in [
        Scenario::ReasoningMismatchUnadvertised,
        Scenario::ReasoningMismatchUnqualified,
        Scenario::ReasoningConfirmationMissing,
        Scenario::ReasoningConfirmationMalformed,
        Scenario::ReasoningConfirmationDuplicate,
        Scenario::ReasoningConfirmationUnbounded,
    ] {
        let fixture = FixtureHost::new(scenario, "0.61.0");
        let failure = match block_on(
            agent_session(Some("low"), false).open_session_with_projection(
                source(PREPARED),
                source(ACTIVE),
                fixture.cleanup_request(),
                fixture.services(ExecutionHostId::new(AGENT_HOST).expect("host is valid")),
            ),
        ) {
            Ok(_) => panic!("ambiguous acknowledgement must reject"),
            Err(failure) => failure,
        };
        assert!(matches!(
            failure,
            ClaudeAgentProjectionOpenFailure::Runtime(_)
        ));
        assert!(failure.rejected_contribution().is_none());
        assert_eq!(fixture.credential_releases(), 1);
        assert_eq!(fixture.resource_releases(), 1);
    }
}

#[test]
fn omitted_reasoning_names_no_acknowledgement_and_keeps_model_observation() {
    let fixture = FixtureHost::new(Scenario::Success, "0.61.0");
    let outcome = block_on(agent_session(None, false).open_session_with_projection(
        source(PREPARED),
        source(ACTIVE),
        fixture.cleanup_request(),
        fixture.services(ExecutionHostId::new(AGENT_HOST).expect("host is valid")),
    ))
    .map_err(|failure| failure.failure().diagnostic().code())
    .expect("omitted reasoning opens");
    assert!(!rows(outcome.contribution()).contains("feature.active-session-reasoning-ack"));
    assert!(rows(outcome.contribution()).contains("feature.negotiated-model-options-observation"));
    let (session, _) = outcome.into_parts();
    assert_eq!(
        block_on(session.close(
            fixture.cleanup_request(),
            fixture.services(ExecutionHostId::new(AGENT_HOST).expect("host is valid")),
        )),
        CleanupOutcome::Clean
    );
}

#[test]
fn equal_source_ids_fail_before_process_work() {
    let fixture = FixtureHost::new(Scenario::Success, "0.61.0");
    let failure = match block_on(
        agent_session(Some("low"), false).open_session_with_projection(
            source(PREPARED),
            source(PREPARED),
            fixture.cleanup_request(),
            fixture.services(ExecutionHostId::new(AGENT_HOST).expect("host is valid")),
        ),
    ) {
        Ok(_) => panic!("collapsed source identity must reject"),
        Err(failure) => failure,
    };
    assert_eq!(
        failure.failure().diagnostic().code(),
        "swallowtail.claude_agent.projection_source_identity_invalid"
    );
    assert!(failure.rejected_contribution().is_none());
    assert_eq!(fixture.credential_acquires(), 0);
    assert_eq!(fixture.resource_releases(), 0);
}

#[test]
fn both_public_open_paths_keep_failure_code_and_cleanup_equal() {
    for scenario in [
        Scenario::ReasoningMismatchAdvertised,
        Scenario::ReasoningConfirmationMissing,
        Scenario::ReasoningConfirmationMalformed,
        Scenario::ReasoningConfirmationDuplicate,
    ] {
        let preserved = FixtureHost::new(scenario, "0.61.0");
        let preserved_failure = match block_on(agent_session(Some("low"), false).open_session(
            preserved.services(ExecutionHostId::new(AGENT_HOST).expect("host is valid")),
        )) {
            Ok(_) => panic!("preserved open should reject"),
            Err(failure) => failure,
        };
        let projected = FixtureHost::new(scenario, "0.61.0");
        let projected_failure = match block_on(
            agent_session(Some("low"), false).open_session_with_projection(
                source(PREPARED),
                source(ACTIVE),
                projected.cleanup_request(),
                projected.services(ExecutionHostId::new(AGENT_HOST).expect("host is valid")),
            ),
        ) {
            Ok(_) => panic!("projected open must reject"),
            Err(failure) => failure,
        };
        assert_eq!(
            preserved_failure.diagnostic().code(),
            projected_failure.failure().diagnostic().code()
        );
        assert_eq!(
            preserved.credential_releases(),
            projected.credential_releases()
        );
        assert_eq!(preserved.resource_releases(), projected.resource_releases());
    }
}

#[test]
fn both_public_open_paths_keep_the_same_managed_handle_shape() {
    let preserved = FixtureHost::new(Scenario::Success, "0.61.0");
    let preserved_handle = block_on(agent_session(Some("low"), false).open_session(
        preserved.services(ExecutionHostId::new(AGENT_HOST).expect("host is valid")),
    ))
    .expect("preserved open succeeds");
    let projected = FixtureHost::new(Scenario::Success, "0.61.0");
    let projected_outcome = match block_on(
        agent_session(Some("low"), false).open_session_with_projection(
            source(PREPARED),
            source(ACTIVE),
            projected.cleanup_request(),
            projected.services(ExecutionHostId::new(AGENT_HOST).expect("host is valid")),
        ),
    ) {
        Ok(outcome) => outcome,
        Err(failure) => panic!(
            "projected open failed: {}",
            failure.failure().diagnostic().code()
        ),
    };
    let (projected_handle, _) = projected_outcome.into_parts();

    assert_eq!(
        preserved_handle.provider_session_ref(),
        projected_handle.provider_session_ref()
    );
    assert_eq!(
        preserved_handle.management_binding(),
        projected_handle.management_binding()
    );
    assert_eq!(
        preserved_handle.negotiated_model_options(),
        projected_handle.negotiated_model_options()
    );
    assert!(preserved_handle.negotiated_model_options().is_some());
    assert_eq!(
        block_on(preserved_handle.close(
            preserved.cleanup_request(),
            preserved.services(ExecutionHostId::new(AGENT_HOST).expect("host is valid")),
        )),
        CleanupOutcome::Clean
    );
    assert_eq!(
        block_on(projected_handle.close(
            projected.cleanup_request(),
            projected.services(ExecutionHostId::new(AGENT_HOST).expect("host is valid")),
        )),
        CleanupOutcome::Clean
    );
    assert_eq!(
        preserved.credential_releases(),
        projected.credential_releases()
    );
    assert_eq!(preserved.resource_releases(), projected.resource_releases());
}
