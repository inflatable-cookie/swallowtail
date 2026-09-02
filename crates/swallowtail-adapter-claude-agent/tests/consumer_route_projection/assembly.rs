use swallowtail_runtime::{
    ConsumerRouteApplicability, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailureKind, ConsumerRouteProjectionRow,
    ConsumerRouteProjectionSourceKind,
};

use super::fixtures::{
    agent_run_at_instance, agent_run_at_revision, agent_session, code_run, profile_contributions,
};
use super::ledger::AGENT_OBSERVED;
use super::mixture::contribution_and_drifted_access;
use super::naming::{semantic_id, source};

const SHARED: &str = "projection.shared.assembly";

fn all_rows(
    contribution: &ConsumerRouteProjectionContribution,
) -> impl Iterator<Item = &ConsumerRouteProjectionRow> {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
}

fn contribution(
    applicability: ConsumerRouteApplicability,
    sources: &ConsumerRouteProjectionContribution,
    row: ConsumerRouteProjectionRow,
) -> Result<ConsumerRouteProjectionContribution, swallowtail_runtime::ConsumerRouteProjectionFailure>
{
    ConsumerRouteProjectionContribution::new(
        applicability,
        sources.sources().cloned().collect::<Vec<_>>(),
        [row],
        [],
        [],
    )
}

fn assert_applicability_rejects(
    target: &ConsumerRouteProjectionContribution,
    borrowed: &ConsumerRouteProjectionRow,
) {
    let failure = contribution(target.applicability().clone(), target, borrowed.clone())
        .expect_err("mixed applicability must fail closed");
    assert_eq!(
        failure.kind(),
        ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement
    );
}

#[test]
fn matching_source_cross_route_and_cross_operation_rows_fail_closed() {
    let agent = agent_run_at_revision("1")
        .consumer_route_projection_contribution(source(SHARED))
        .expect("agent contributes");
    let code = code_run(None, None)
        .consumer_route_projection_contribution(source(SHARED))
        .expect("code contributes");
    let session = agent_session(None, false)
        .consumer_route_projection_contribution(source(SHARED))
        .expect("session contributes");
    let portable = all_rows(&agent)
        .find(|row| semantic_id(row.identity()) == "feature.streaming-events")
        .expect("portable row exists");
    assert_applicability_rejects(&code, portable);
    assert_applicability_rejects(&session, portable);

    let interactive = all_rows(&session)
        .find(|row| semantic_id(row.identity()) == "feature.interactive-session")
        .expect("interactive row exists");
    assert_applicability_rejects(&agent, interactive);
}

#[test]
fn matching_source_stale_revision_rows_fail_closed() {
    let current = agent_run_at_revision("1")
        .consumer_route_projection_contribution(source(SHARED))
        .expect("current contributes");
    let stale = agent_run_at_revision("2")
        .consumer_route_projection_contribution(source(SHARED))
        .expect("stale contributes");
    assert_ne!(
        current.applicability().instance_revision(),
        stale.applicability().instance_revision()
    );
    let row = all_rows(&stale)
        .find(|row| semantic_id(row.identity()) == "feature.streaming-events")
        .expect("portable row exists");
    assert_applicability_rejects(&current, row);
}

#[test]
fn matching_source_cross_instance_rows_fail_closed() {
    let current = agent_run_at_instance("projection.agent.instance")
        .consumer_route_projection_contribution(source(SHARED))
        .expect("current contributes");
    let other = agent_run_at_instance("projection.agent.other-instance")
        .consumer_route_projection_contribution(source(SHARED))
        .expect("other instance contributes");
    assert_ne!(
        current.applicability().instance_id(),
        other.applicability().instance_id()
    );
    let row = all_rows(&other)
        .find(|row| semantic_id(row.identity()) == "feature.streaming-events")
        .expect("portable row exists");
    assert_applicability_rejects(&current, row);
}

#[test]
fn matching_source_cross_access_rows_fail_closed() {
    let (ready, shifted) = contribution_and_drifted_access();
    let row = all_rows(&ready)
        .find(|row| semantic_id(row.identity()) == "feature.streaming-events")
        .expect("portable row exists");
    let mut formed = 0;
    for (status, applicability) in shifted {
        let Some(applicability) = applicability else {
            assert_eq!(
                status.support_authority(),
                swallowtail_core::SupportAuthority::ExperimentalObserved,
                "only support-authority drift may fail before a snapshot exists"
            );
            continue;
        };
        formed += 1;
        assert_same_non_access_dimensions(&applicability, ready.applicability());
        let failure = contribution(applicability, &ready, row.clone())
            .expect_err("access drift must reject the prepared row");
        assert_eq!(
            failure.kind(),
            ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement
        );
    }
    assert_eq!(formed, 4);
}

fn assert_same_non_access_dimensions(
    shifted: &ConsumerRouteApplicability,
    ready: &ConsumerRouteApplicability,
) {
    assert_eq!(shifted.instance_id(), ready.instance_id());
    assert_eq!(shifted.instance_revision(), ready.instance_revision());
    assert_eq!(shifted.instance_policy_id(), ready.instance_policy_id());
    assert_eq!(shifted.driver_identity(), ready.driver_identity());
    assert_eq!(shifted.protocol_facade_id(), ready.protocol_facade_id());
    assert_eq!(shifted.execution_host_id(), ready.execution_host_id());
    assert_eq!(shifted.driver_role(), ready.driver_role());
    assert_eq!(shifted.execution_layer(), ready.execution_layer());
    assert_eq!(shifted.operation_shape(), ready.operation_shape());
    assert_eq!(shifted.model(), ready.model());
    assert_eq!(shifted.access_profile_id(), ready.access_profile_id());
    assert_eq!(shifted.credential_mechanism(), ready.credential_mechanism());
    assert_eq!(shifted.resource_access(), ready.resource_access());
    assert_eq!(shifted.filesystem_boundary(), ready.filesystem_boundary());
    assert_ne!(shifted, ready);
}

#[test]
fn acknowledgement_cannot_publish_on_structured_or_claude_code_applicability() {
    let profiles = profile_contributions();
    let observed = profiles.get(AGENT_OBSERVED).expect("observed profile");
    let acknowledgement = all_rows(observed)
        .find(|row| semantic_id(row.identity()) == "feature.active-session-reasoning-ack")
        .expect("acknowledgement exists");
    let model = all_rows(observed)
        .find(|row| semantic_id(row.identity()) == "feature.negotiated-model-options-observation")
        .expect("model observation exists");
    let structured = agent_run_at_revision("1")
        .consumer_route_projection_contribution(source("projection.ack.structured"))
        .expect("structured contributes");
    let code = code_run(None, None)
        .consumer_route_projection_contribution(source("projection.ack.code"))
        .expect("code contributes");
    assert_applicability_rejects(&structured, acknowledgement);
    assert_applicability_rejects(&code, acknowledgement);
    assert_applicability_rejects(&structured, model);
    assert_applicability_rejects(&code, model);
}

#[test]
fn active_observation_source_is_never_attached_to_a_prepared_row() {
    let profiles = profile_contributions();
    let observed = profiles.get(AGENT_OBSERVED).expect("observed profile");
    let active = observed
        .sources()
        .find(|source| source.kind() == ConsumerRouteProjectionSourceKind::ActiveSessionObservation)
        .expect("active source exists");
    for row in observed
        .selection_rows()
        .chain(observed.session_start_rows())
    {
        assert_ne!(row.source().id(), active.id(), "{:?}", row.identity());
        assert_eq!(
            row.source().kind(),
            ConsumerRouteProjectionSourceKind::AdapterContribution
        );
    }
}
