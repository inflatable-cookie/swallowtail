use std::collections::BTreeSet;
use swallowtail_core::OperationShape;

use super::fixtures::{
    catalogue_contribution, inference_contribution, observed_attached, observed_owned,
    serving_contribution,
};
use super::ledger::*;
use super::naming::RowIdentity;

fn identity(entry: &LedgerEntry) -> RowIdentity {
    (
        entry.route_id.to_owned(),
        entry.operation_shape,
        entry.semantic_id.to_owned(),
    )
}

fn claimed(tranche: &[LedgerEntry], profile: &str) -> BTreeSet<RowIdentity> {
    tranche
        .iter()
        .filter(|entry| entry.emitted_by.contains(&profile))
        .map(identity)
        .collect()
}

fn assert_tranche(
    tranche: &[LedgerEntry],
    route: &str,
    shapes: &[&str],
    profiles: &[&str],
    expected_len: usize,
) {
    let mut tuples = BTreeSet::new();
    let mut semantics = BTreeSet::new();
    for entry in tranche {
        assert_eq!(entry.route_id, route, "{} is off-route", entry.semantic_id);
        assert!(
            shapes.contains(&entry.operation_shape),
            "{} names an operation shape outside this census",
            entry.semantic_id
        );
        assert!(
            tuples.insert(identity(entry)),
            "the ledger repeats the exact identity of {}",
            entry.semantic_id
        );
        assert!(
            semantics.insert(entry.semantic_id),
            "the ledger repeats {}",
            entry.semantic_id
        );
        assert!(
            entry.semantic_id.starts_with("feature.") || entry.semantic_id.starts_with("control."),
            "{} is not a census row identity",
            entry.semantic_id
        );
        assert_eq!(
            entry.emitted_by.is_empty(),
            !entry.withheld_because.is_empty(),
            "{} must be either emitted or withheld with a reason",
            entry.semantic_id
        );
        for profile in entry.emitted_by {
            assert!(
                profiles.contains(profile),
                "{} names an unknown prepared profile",
                entry.semantic_id
            );
        }
    }
    assert_eq!(tranche.len(), expected_len);
    assert_eq!(tuples.len(), expected_len);
    assert_eq!(semantics.len(), expected_len);
}

fn assert_observed(
    observed: &std::collections::BTreeMap<&str, BTreeSet<RowIdentity>>,
    tranche: &[LedgerEntry],
    profiles: &[&str],
    off_route: &[&str],
) {
    assert_eq!(observed.len(), profiles.len());
    for profile in profiles {
        assert_eq!(
            observed.get(profile).expect("every profile contributes"),
            &claimed(tranche, profile),
            "{profile} emitted identities differ from the coverage ledger"
        );
    }
    let emitted = observed
        .values()
        .flatten()
        .cloned()
        .collect::<BTreeSet<_>>();
    let ledger = tranche.iter().map(identity).collect::<BTreeSet<_>>();
    for published in &emitted {
        assert!(
            ledger.contains(published),
            "{published:?} is published without a recorded disposition"
        );
    }
    for entry in tranche {
        if entry.emitted_by.is_empty() {
            assert!(
                !emitted.contains(&identity(entry)),
                "{} is withheld but was published",
                entry.semantic_id
            );
        } else {
            assert!(
                emitted.contains(&identity(entry)),
                "{} is claimed but was never published",
                entry.semantic_id
            );
        }
    }
    let published_semantics = emitted
        .iter()
        .map(|(_, _, semantic)| semantic.as_str())
        .collect::<BTreeSet<_>>();
    for off_route in off_route {
        assert!(
            !published_semantics.contains(off_route),
            "{off_route} has no census row and must never be constructed"
        );
        assert!(
            !tranche.iter().any(|entry| entry.semantic_id == *off_route),
            "{off_route} must not appear in this ledger"
        );
    }
}

#[test]
fn the_coverage_ledgers_disposition_exactly_the_sixteen_llama_cpp_rows() {
    assert_tranche(
        &LLAMA_CPP_ATTACHED_TRANCHE,
        ATTACHED_ROUTE,
        &ATTACHED_SHAPES,
        &ATTACHED_PROFILES,
        10,
    );
    assert_tranche(
        &LLAMA_CPP_OWNED_TRANCHE,
        OWNED_ROUTE,
        &OWNED_SHAPES,
        &OWNED_PROFILES,
        6,
    );
    assert_eq!(
        LLAMA_CPP_ATTACHED_TRANCHE
            .iter()
            .filter(|entry| entry.emitted_by.is_empty())
            .count(),
        1
    );
    assert_eq!(
        LLAMA_CPP_OWNED_TRANCHE
            .iter()
            .filter(|entry| entry.emitted_by.is_empty())
            .count(),
        1
    );
}

#[test]
fn every_prepared_profile_emits_exactly_its_ledger_identities() {
    assert_observed(
        &observed_attached(),
        &LLAMA_CPP_ATTACHED_TRANCHE,
        &ATTACHED_PROFILES,
        &ATTACHED_OFF_ROUTE,
    );
    assert_observed(
        &observed_owned(),
        &LLAMA_CPP_OWNED_TRANCHE,
        &OWNED_PROFILES,
        &OWNED_OFF_ROUTE,
    );
}

#[test]
fn attached_inference_rows_carry_structured_run_and_catalogue_stays_model_catalog() {
    let catalogue = catalogue_contribution("llama-cpp.attached.shape.catalogue");
    let inference = inference_contribution("llama-cpp.attached.shape.inference");
    assert_eq!(
        catalogue.applicability().operation_shape(),
        OperationShape::StructuredRun
    );
    assert_eq!(
        catalogue.applicability().driver_role(),
        swallowtail_core::DriverRole::ModelCatalog
    );
    assert_eq!(
        inference.applicability().operation_shape(),
        OperationShape::StructuredRun
    );
    assert_eq!(
        inference.applicability().driver_role(),
        swallowtail_core::DriverRole::StructuredRun
    );
    for row in super::naming::all_rows(&catalogue).chain(super::naming::all_rows(&inference)) {
        let contribution =
            if row.applicability().driver_role() == swallowtail_core::DriverRole::ModelCatalog {
                &catalogue
            } else {
                &inference
            };
        assert_eq!(row.applicability(), contribution.applicability());
    }
}

#[test]
fn owned_rows_carry_the_serving_lifecycle_role_and_never_emit_activity() {
    let serving = serving_contribution("llama-cpp.owned.shape");
    assert_eq!(
        serving.applicability().driver_role(),
        swallowtail_core::DriverRole::ServingInstanceLifecycle
    );
    assert_eq!(serving.active_session_rows().len(), 0);
}
