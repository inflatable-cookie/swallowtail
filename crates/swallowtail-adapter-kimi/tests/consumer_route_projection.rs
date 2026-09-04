use crate::fixtures::{prepared, profile_input};
use crate::support::{FixtureHost, Scenario, close_session};
use futures_executor::block_on;
use swallowtail_adapter_kimi::{KimiPreparedSession, KimiProjectionOpenFailure};
use swallowtail_core::{ExecutionHostId, HarnessMode, ReasoningMode};
use swallowtail_runtime::{
    ConsumerRouteAcknowledgementState, ConsumerRouteControlId, ConsumerRouteFeatureId,
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionSourceId, ConsumerRouteRowIdentity,
    SessionOptions,
};

#[test]
fn prepared_profiles_emit_only_constructible_session_controls() {
    let host_id = host_id("profiles");
    let host = FixtureHost::new(Scenario::ReasoningEffortSuccess);
    let integration = prepared(&host, host_id, "0.29.0");
    let minimal = integration
        .prepare_session(profile_input(
            "projection-minimal",
            SessionOptions::default(),
        ))
        .expect("minimal session prepares")
        .consumer_route_projection_contribution(source("kimi.prepared.minimal"))
        .expect("minimal contribution is admitted");
    assert!(!has_feature(
        &minimal,
        ConsumerRouteFeatureId::ReasoningSelection
    ));
    assert!(!has_control(
        &minimal,
        ConsumerRouteControlId::ReasoningSelection
    ));
    assert!(has_control(&minimal, ConsumerRouteControlId::LoadSession));
    assert!(has_control(&minimal, ConsumerRouteControlId::ResumeSession));

    let options = SessionOptions::default()
        .with_reasoning_mode(ReasoningMode::new("high").expect("reasoning"))
        .with_harness_mode(HarnessMode::Plan);
    let maximal = integration
        .prepare_session(profile_input("projection-maximal", options))
        .expect("maximal session prepares")
        .consumer_route_projection_contribution(source("kimi.prepared.maximal"))
        .expect("maximal contribution is admitted");
    assert!(has_feature(
        &maximal,
        ConsumerRouteFeatureId::ReasoningSelection
    ));
    assert!(has_control(
        &maximal,
        ConsumerRouteControlId::ReasoningSelection
    ));
    assert!(!has_control(&maximal, ConsumerRouteControlId::LoadSession));
    assert!(!has_control(
        &maximal,
        ConsumerRouteControlId::ResumeSession
    ));
    assert!(!has_feature(
        &maximal,
        ConsumerRouteFeatureId::PersistentSessionPosture
    ));
}

#[test]
fn projected_open_requires_distinct_sources_before_host_work() {
    let host_id = host_id("sources");
    let host = FixtureHost::new(Scenario::Complete);
    let session = session(&host, host_id.clone(), SessionOptions::default());
    let same = source("kimi.open.same");
    let failure = match block_on(session.open_session_with_projection(
        same.clone(),
        same,
        host.services(host_id),
    )) {
        Ok(_) => panic!("equal source identities must reject"),
        Err(failure) => failure,
    };
    assert_eq!(
        failure.failure().diagnostic().code(),
        "swallowtail.kimi.projection_source_identity_invalid"
    );
    assert!(!host.process_started());
}

#[test]
fn maximal_reasoning_rejection_records_terminally_undispatched_plan() {
    let host_id = host_id("compound-rejection");
    let host = FixtureHost::new(Scenario::ReasoningDrift);
    let options = SessionOptions::default()
        .with_reasoning_mode(ReasoningMode::new("high").expect("reasoning"))
        .with_harness_mode(HarnessMode::Plan);
    let prepared = session(&host, host_id.clone(), options);
    let failure = match block_on(prepared.open_session_with_projection(
        source("kimi.open.prepared"),
        source("kimi.open.active"),
        host.services(host_id),
    )) {
        Ok(_) => panic!("mismatched reasoning must reject"),
        Err(failure) => failure,
    };
    assert!(matches!(
        failure,
        KimiProjectionOpenFailure::Rejected { .. }
    ));
    assert_eq!(
        failure.failure().diagnostic().code(),
        "swallowtail.negotiated_reasoning.effective_mismatch"
    );
    let row = failure
        .rejected_contribution()
        .expect("admitted rejection has a contribution")
        .active_session_rows()
        .find(|row| {
            row.identity()
                == &ConsumerRouteRowIdentity::Feature(
                    ConsumerRouteFeatureId::ActiveSessionReasoningAcknowledgement,
                )
        })
        .expect("compound acknowledgement row exists");
    let acknowledgement = row
        .compound_acknowledgement()
        .expect("compound value exists");
    assert!(matches!(
        acknowledgement.reasoning(),
        ConsumerRouteAcknowledgementState::Rejected(value) if value.as_str() == "medium"
    ));
    assert_eq!(
        acknowledgement.plan(),
        &ConsumerRouteAcknowledgementState::RequestedNotDispatched
    );
    assert!(!row.state_support().pending());
    let sets = host
        .wire_messages()
        .into_iter()
        .filter(|message| message["method"] == "session/set_config_option")
        .collect::<Vec<_>>();
    assert_eq!(sets.len(), 1);
    assert_eq!(sets[0]["params"]["configId"], "thinking");
}

#[test]
fn successful_open_retains_exact_halves_and_model_observation() {
    let host_id = host_id("compound-success");
    let host = FixtureHost::new(Scenario::ReasoningEffortSuccess);
    let options = SessionOptions::default()
        .with_reasoning_mode(ReasoningMode::new("high").expect("reasoning"))
        .with_harness_mode(HarnessMode::Plan);
    let prepared = session(&host, host_id.clone(), options);
    let outcome = match block_on(prepared.open_session_with_projection(
        source("kimi.success.prepared"),
        source("kimi.success.active"),
        host.services(host_id.clone()),
    )) {
        Ok(outcome) => outcome,
        Err(failure) => panic!(
            "projected open failed: {}",
            failure.failure().diagnostic().code()
        ),
    };
    let row = outcome
        .contribution()
        .active_session_rows()
        .find(|row| {
            row.identity()
                == &ConsumerRouteRowIdentity::Feature(
                    ConsumerRouteFeatureId::ActiveSessionReasoningAcknowledgement,
                )
        })
        .expect("compound acknowledgement row exists");
    let acknowledgement = row
        .compound_acknowledgement()
        .expect("compound value exists");
    assert!(matches!(
        acknowledgement.reasoning(),
        ConsumerRouteAcknowledgementState::Effective(value) if value.as_str() == "high"
    ));
    assert!(matches!(
        acknowledgement.plan(),
        ConsumerRouteAcknowledgementState::Effective(value) if value.as_str() == "plan"
    ));
    assert!(outcome.negotiated_model_options().is_some());
    assert!(outcome.contribution().active_session_rows().any(|row| {
        row.identity()
            .namespaced_extension()
            .is_some_and(|extension| {
                extension.semantic_id() == "feature.negotiated-model-options-observation"
            })
    }));
    let (session, _) = outcome.into_parts();
    assert_eq!(
        block_on(close_session(session, host.services(host_id))),
        swallowtail_runtime::CleanupOutcome::Clean
    );
}

fn session(
    host: &FixtureHost,
    host_id: ExecutionHostId,
    options: SessionOptions,
) -> KimiPreparedSession {
    prepared(host, host_id, "0.29.0")
        .prepare_session(profile_input("projection-session", options))
        .expect("session prepares")
}

fn host_id(value: &str) -> ExecutionHostId {
    ExecutionHostId::new(format!("fixture.kimi.projection.{value}")).expect("fixture host identity")
}

fn source(value: &str) -> ConsumerRouteProjectionSourceId {
    ConsumerRouteProjectionSourceId::new(value).expect("projection source")
}

fn has_feature(
    contribution: &ConsumerRouteProjectionContribution,
    feature: ConsumerRouteFeatureId,
) -> bool {
    contribution
        .selection_rows()
        .chain(contribution.active_session_rows())
        .any(|row| row.identity() == &ConsumerRouteRowIdentity::Feature(feature.clone()))
}

fn has_control(
    contribution: &ConsumerRouteProjectionContribution,
    control: ConsumerRouteControlId,
) -> bool {
    contribution
        .session_start_rows()
        .any(|row| row.identity() == &ConsumerRouteRowIdentity::Control(control.clone()))
}
#[path = "consumer_route_projection/catalogue.rs"]
mod catalogue;
#[path = "consumer_route_projection/foreign.rs"]
mod foreign;
#[path = "consumer_route_projection/ledger.rs"]
mod ledger;
