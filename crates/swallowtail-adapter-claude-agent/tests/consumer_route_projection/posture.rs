use std::collections::BTreeSet;
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteAvailability, ConsumerRouteLifecycle,
    ConsumerRouteOmissionSemantics, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionRow, ConsumerRouteSupportPosture,
};

use super::fixtures::{agent_run, agent_session, code_run, profile_contributions, response_run};
use super::ledger::AGENT_OBSERVED;
use super::naming::{rows, semantic_id, source};

fn all_rows(
    contribution: &ConsumerRouteProjectionContribution,
) -> impl Iterator<Item = &ConsumerRouteProjectionRow> {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
}

#[test]
fn every_row_keeps_exact_view_lifecycle_actor_support_and_availability() {
    for (profile, contribution) in profile_contributions() {
        let selection = contribution.selection_rows().map(|row| (row, "selection"));
        let start = contribution
            .session_start_rows()
            .map(|row| (row, "session-start"));
        let active = contribution
            .active_session_rows()
            .map(|row| (row, "active"));
        for (row, view) in selection.chain(start).chain(active) {
            let semantic = semantic_id(row.identity());
            let is_control = semantic.starts_with("control.");
            let is_activity = semantic == "feature.activity-observation";
            let is_ack = semantic == "feature.active-session-reasoning-ack";
            let is_model = semantic == "feature.negotiated-model-options-observation";
            let expected_view = if is_activity || is_ack || is_model {
                "active"
            } else if is_control && semantic != "control.model-selection" {
                "session-start"
            } else {
                "selection"
            };
            assert_eq!(view, expected_view, "{profile}: {semantic}");
            assert_eq!(
                row.lifecycle(),
                if is_activity || is_ack || is_model {
                    ConsumerRouteLifecycle::PostOpenObservationOnly
                } else if expected_view == "session-start" {
                    ConsumerRouteLifecycle::SessionStartOnly
                } else {
                    ConsumerRouteLifecycle::SelectionSummary
                },
                "{profile}: {semantic}"
            );
            assert_eq!(
                row.actor_posture(),
                if is_activity || is_ack || is_model {
                    ConsumerRouteActorPosture::ObservationOnly
                } else if is_control {
                    ConsumerRouteActorPosture::ConsumerSelectable
                } else {
                    ConsumerRouteActorPosture::Informational
                },
                "{profile}: {semantic}"
            );
            assert_eq!(row.support(), ConsumerRouteSupportPosture::Supported);
            assert_eq!(row.availability(), ConsumerRouteAvailability::Available);
        }
    }
}

#[test]
fn activity_is_descriptor_only_and_prepared_rows_claim_no_effective_or_rejected_state() {
    for (profile, contribution) in profile_contributions() {
        for row in all_rows(&contribution) {
            let semantic = semantic_id(row.identity());
            if semantic == "feature.activity-observation" {
                assert!(row.state_support().is_descriptor_only(), "{profile}");
            }
            if profile != AGENT_OBSERVED || semantic != "feature.active-session-reasoning-ack" {
                assert!(
                    !row.state_support().provider_effective(),
                    "{profile}: {semantic}"
                );
                assert!(!row.state_support().rejected(), "{profile}: {semantic}");
            }
        }
    }
}

#[test]
fn prepared_reasoning_keeps_exact_requested_prepared_and_pending_truth() {
    for (label, contribution, pending) in [
        (
            "agent-run",
            agent_run(Some("low"), true, true)
                .consumer_route_projection_contribution(source("posture.agent.run"))
                .expect("contributes"),
            true,
        ),
        (
            "agent-session",
            agent_session(Some("low"), true)
                .consumer_route_projection_contribution(source("posture.agent.session"))
                .expect("contributes"),
            true,
        ),
        (
            "code",
            code_run(Some("low"), Some(3))
                .consumer_route_projection_contribution(source("posture.code"))
                .expect("contributes"),
            false,
        ),
        (
            "response",
            response_run(Some("low"))
                .consumer_route_projection_contribution(source("posture.response"))
                .expect("contributes"),
            false,
        ),
    ] {
        let row = all_rows(&contribution)
            .find(|row| semantic_id(row.identity()) == "control.reasoning-selection")
            .expect("reasoning control exists");
        let state = row.state_support();
        assert!(state.requested(), "{label}");
        assert!(state.prepared(), "{label}");
        assert_eq!(state.pending(), pending, "{label}");
        assert!(!state.provider_effective(), "{label}");
        assert!(!state.rejected(), "{label}");
    }
}

#[test]
fn each_control_keeps_its_exact_omission_semantics() {
    for (profile, contribution) in profile_contributions() {
        for row in all_rows(&contribution) {
            let semantic = semantic_id(row.identity());
            let Some(value) = row.control_value() else {
                assert!(!semantic.starts_with("control."));
                continue;
            };
            let expected = match semantic.as_str() {
                "control.model-selection"
                | "control.permission-handling"
                | "control.run-retention" => ConsumerRouteOmissionSemantics::Required,
                "control.reasoning-selection"
                | "control.session-options"
                | "control.maximum-agentic-turns" => {
                    ConsumerRouteOmissionSemantics::PreservesRouteBehavior
                }
                "control.load-session"
                | "control.resume-session"
                | "control.provider-session-delete" => {
                    ConsumerRouteOmissionSemantics::SuppliesNothing
                }
                "feature.active-session-reasoning-ack"
                | "feature.negotiated-model-options-observation" => {
                    ConsumerRouteOmissionSemantics::NotSelectable
                }
                other => panic!("unexpected control {other}"),
            };
            assert_eq!(value.omission(), expected, "{profile}: {semantic}");
        }
    }
}

#[test]
fn optional_rows_are_absent_until_the_exact_prepared_input_exists() {
    let cases = [
        (
            rows(
                &agent_run(None, false, false)
                    .consumer_route_projection_contribution(source("omission.agent.run.min"))
                    .expect("contributes"),
            ),
            rows(
                &agent_run(Some("low"), true, true)
                    .consumer_route_projection_contribution(source("omission.agent.run.max"))
                    .expect("contributes"),
            ),
        ),
        (
            rows(
                &agent_session(None, false)
                    .consumer_route_projection_contribution(source("omission.agent.session.min"))
                    .expect("contributes"),
            ),
            rows(
                &agent_session(Some("low"), true)
                    .consumer_route_projection_contribution(source("omission.agent.session.max"))
                    .expect("contributes"),
            ),
        ),
        (
            rows(
                &code_run(None, None)
                    .consumer_route_projection_contribution(source("omission.code.min"))
                    .expect("contributes"),
            ),
            rows(
                &code_run(Some("low"), Some(3))
                    .consumer_route_projection_contribution(source("omission.code.max"))
                    .expect("contributes"),
            ),
        ),
        (
            rows(
                &response_run(None)
                    .consumer_route_projection_contribution(source("omission.response.min"))
                    .expect("contributes"),
            ),
            rows(
                &response_run(Some("low"))
                    .consumer_route_projection_contribution(source("omission.response.max"))
                    .expect("contributes"),
            ),
        ),
    ];
    for (minimal, maximal) in &cases {
        assert!(minimal.is_subset(maximal));
        assert_ne!(minimal, maximal);
    }
    let absent = BTreeSet::from([
        "control.reasoning-selection".to_owned(),
        "feature.reasoning-selection".to_owned(),
    ]);
    assert!(absent.is_disjoint(&cases[3].0));
}
