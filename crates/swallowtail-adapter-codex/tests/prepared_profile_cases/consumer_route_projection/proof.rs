use super::super::*;
use std::collections::BTreeSet;
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteControlId, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteRowIdentity, ConsumerRouteSourceClass,
};

use super::fixtures::{observed_dispositions, session_profile};
use super::ledger::*;
use super::naming::*;

#[test]
fn the_coverage_ledger_dispositions_exactly_the_thirty_six_app_server_rows() {
    let mut ids = BTreeSet::new();
    for entry in &CODEX_FIRST_TRANCHE {
        assert!(
            ids.insert(entry.semantic_id),
            "the ledger repeats {}",
            entry.semantic_id
        );
        assert!(
            entry.semantic_id.starts_with("feature.") || entry.semantic_id.starts_with("control."),
            "{} is not a census row identity",
            entry.semantic_id
        );
        if entry.emitted_by.is_empty() {
            assert!(
                !entry.withheld_because.is_empty(),
                "{} is withheld without a reason",
                entry.semantic_id
            );
        } else {
            assert!(
                entry.withheld_because.is_empty(),
                "{} is emitted and withheld at once",
                entry.semantic_id
            );
        }
    }
    assert_eq!(CODEX_FIRST_TRANCHE.len(), 36);
    assert_eq!(ids.len(), 36);
}

#[test]
fn every_prepared_facade_emits_exactly_its_ledger_rows() {
    let observed = observed_dispositions();
    assert_eq!(observed.len(), CODEX_FACADES.len());
    for facade in CODEX_FACADES {
        let expected = CODEX_FIRST_TRANCHE
            .iter()
            .filter(|entry| entry.emitted_by.contains(&facade))
            .map(|entry| entry.semantic_id)
            .collect::<BTreeSet<_>>();
        let published = observed.get(facade).expect("every facade contributes");
        assert_eq!(
            published, &expected,
            "{facade} emitted rows differ from the coverage ledger"
        );
    }
}

#[test]
fn withheld_rows_are_emitted_by_no_prepared_facade() {
    let observed = observed_dispositions();
    let emitted = observed
        .values()
        .flatten()
        .copied()
        .collect::<BTreeSet<_>>();
    let ledger = CODEX_FIRST_TRANCHE
        .iter()
        .map(|entry| entry.semantic_id)
        .collect::<BTreeSet<_>>();
    for published in &emitted {
        assert!(
            ledger.contains(published),
            "{published} is published without a recorded disposition"
        );
    }
    for entry in &CODEX_FIRST_TRANCHE {
        if entry.emitted_by.is_empty() {
            assert!(
                !emitted.contains(entry.semantic_id),
                "{} is withheld but was published",
                entry.semantic_id
            );
        } else {
            assert!(
                emitted.contains(entry.semantic_id),
                "{} is claimed but was never published",
                entry.semantic_id
            );
        }
    }
}

#[test]
fn out_of_tranche_feature_rows_are_withheld_at_construction() {
    let observed = observed_dispositions();
    let emitted = observed
        .values()
        .flatten()
        .copied()
        .collect::<BTreeSet<_>>();
    for withheld in WITHHELD_OUT_OF_TRANCHE {
        assert!(
            !emitted.contains(withheld),
            "{withheld} is outside the tranche and must never be constructed"
        );
        assert!(
            !CODEX_FIRST_TRANCHE
                .iter()
                .any(|entry| entry.semantic_id == withheld),
            "{withheld} must not appear in the 36-row ledger"
        );
    }
    assert_eq!(
        emitted.len(),
        CODEX_FIRST_TRANCHE
            .iter()
            .filter(|entry| !entry.emitted_by.is_empty())
            .count(),
        "the published set is exactly the ledger's emitted rows"
    );
}

#[test]
fn the_per_turn_exchange_stays_per_turn_and_claims_no_provider_mutation() {
    let recording = RecordingHostServices::default();
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        FIXTURE_VERSION,
        &recording,
        true,
    );
    let session = prepared_app
        .prepare_read_only_session(session_profile("per-turn"))
        .expect("read-only session prepares");
    let contribution = session
        .consumer_route_projection_contribution(source("codex.session.per-turn"))
        .expect("session contributes");
    let exchange = contribution
        .session_start_rows()
        .find(|row| {
            row.identity()
                == &ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::UserInputExchange)
        })
        .expect("the per-turn exchange is published");
    assert_eq!(exchange.lifecycle(), ConsumerRouteLifecycle::PerTurn);
    assert!(
        exchange
            .mutation_authority()
            .is_consumer_mediated_per_turn(),
        "the per-turn exchange carries consumer-mediated authority"
    );
    assert!(
        !exchange.mutation_authority().is_prepared_session_start(),
        "the per-turn exchange never claims prepared session-start authority"
    );
    assert!(!exchange.state_support().prepared());
    assert!(!exchange.state_support().provider_effective());
    assert!(!exchange.state_support().rejected());
    assert!(!exchange.mutation_authority().is_acknowledged());

    let observation = contribution
        .active_session_rows()
        .find(|row| {
            row.identity()
                == &ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::ActivityObservation)
        })
        .expect("activity observation stays post-open");
    assert_eq!(
        observation.lifecycle(),
        ConsumerRouteLifecycle::PostOpenObservationOnly
    );
    assert_eq!(
        observation.actor_posture(),
        ConsumerRouteActorPosture::ObservationOnly
    );
    assert!(observation.state_support().observed());
    assert!(!observation.state_support().provider_effective());
}

#[test]
fn every_published_row_carries_exact_runtime_or_prepared_authority() {
    let recording = RecordingHostServices::default();
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        FIXTURE_VERSION,
        &recording,
        true,
    );
    let session = prepared_app
        .prepare_read_only_session(session_profile("authority"))
        .expect("read-only session prepares");
    let contribution = session
        .consumer_route_projection_contribution(source("codex.session.authority"))
        .expect("session contributes");
    for row in contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
    {
        assert!(
            matches!(
                row.source_class(),
                ConsumerRouteSourceClass::PreparedOperationRecord
                    | ConsumerRouteSourceClass::CapabilityProfile
                    | ConsumerRouteSourceClass::AdapterPreparedInput
            ),
            "{:?} does not carry exact runtime or prepared authority",
            row.identity()
        );
        assert!(row.safe_reason().is_none());
    }
    let rendered = format!("{contribution:?}");
    for forbidden in ["codex-app-server-executable", "private instructions"] {
        assert!(
            !rendered.contains(forbidden),
            "a projected row must not carry raw target, command, or content data"
        );
    }
}
