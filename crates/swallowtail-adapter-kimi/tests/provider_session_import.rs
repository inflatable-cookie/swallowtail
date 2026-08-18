use crate::fixtures::{prepared, prepared_with_state_root, profile_input};
use crate::support::{FixtureHost, Scenario};
use futures_executor::block_on;
use std::num::NonZeroU32;
use swallowtail_adapter_kimi::KimiSessionCatalogueInput;
use swallowtail_core::{
    ExecutionHostId, ProviderSessionBindingOrigin, ProviderSessionCatalogueBounds,
    ProviderSessionImportAvailability,
};
use swallowtail_runtime::{CleanupOutcome, ProviderSessionCatalogueId, RequestId, SessionOptions};

#[test]
fn exact_session_list_range_fixture_names_every_qualified_milestone() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(
        "fixtures/kimi-code-acp-v0.28.1-v0.31.0/session-list-range.json"
    ))
    .expect("range fixture parses");
    assert_eq!(fixture["operation"]["method"], "session/list");
    assert_eq!(fixture["operation"]["scope_field"], "cwd");
    let versions = fixture["qualified_versions"]
        .as_array()
        .expect("qualified version records")
        .iter()
        .map(|record| record["version"].as_str().expect("exact version"))
        .collect::<Vec<_>>();
    assert_eq!(
        versions,
        ["0.28.1", "0.29.0", "0.29.1", "0.29.2", "0.30.0", "0.31.0", "0.31.1"]
    );
    assert_eq!(
        fixture["later_stable_posture"]["session_import"],
        "not-inherited"
    );
}

#[test]
fn prepared_catalogue_import_and_load_preserve_state_and_replay_boundaries() {
    for version in [
        "0.28.1", "0.29.0", "0.29.1", "0.29.2", "0.30.0", "0.31.0", "0.31.1",
    ] {
        let host_id = ExecutionHostId::new(format!("fixture.kimi.import.{version}")).unwrap();
        let preparation_host = FixtureHost::new(scenario(version));
        let prepared = prepared(&preparation_host, host_id.clone(), version);
        let catalogue = prepared
            .prepare_session_catalogue(catalogue_input(&format!("catalogue-{version}")))
            .expect("qualified Kimi catalogue prepares");
        let catalogue_host = FixtureHost::new(scenario(version));
        let outcome = block_on(catalogue.list_sessions(catalogue_host.services(host_id.clone())))
            .expect("resource-scoped catalogue succeeds");
        let candidate = outcome
            .candidates()
            .next()
            .expect("fixture candidate exists")
            .clone();
        assert_eq!(
            candidate.import_availability(),
            ProviderSessionImportAvailability::Available
        );
        assert_eq!(candidate.display().title(), Some("Kimi fixture session"));
        let initialize = &catalogue_host.wire_messages()[0];
        assert_eq!(
            initialize["params"]["clientCapabilities"]["fs"]["writeTextFile"],
            false
        );
        assert_eq!(
            catalogue_host.wire_methods(),
            ["initialize".to_owned(), "session/list".to_owned()]
        );

        let imported = prepared
            .prepare_session_import(
                &catalogue,
                candidate,
                profile_input(&format!("import-{version}"), SessionOptions::default()),
            )
            .expect("selected candidate import prepares");
        let import_host = FixtureHost::new(scenario(version));
        let imported_outcome =
            block_on(imported.import_session(import_host.services(host_id.clone())))
                .expect("candidate revalidation issues a binding");
        assert_eq!(
            imported_outcome.binding().origin(),
            ProviderSessionBindingOrigin::ExplicitlyImported
        );

        let session = prepared
            .prepare_session(profile_input(
                &format!("load-profile-{version}"),
                SessionOptions::default(),
            ))
            .expect("ordinary attachment profile prepares");
        let load_host = FixtureHost::new(scenario(version));
        let loaded = block_on(
            session
                .load_session(
                    RequestId::new(format!("load-{version}")).unwrap(),
                    imported_outcome.binding().clone(),
                    load_host.services(host_id),
                )
                .expect("imported binding derives a load request"),
        )
        .expect("imported session loads with replay");
        let (replay, handle) = loaded.into_parts();
        assert_eq!(replay.len(), 2);
        assert_eq!(
            replay[1].content().expect("agent replay").as_str(),
            "Previous answer."
        );
        assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
        assert!(load_host
            .wire_methods()
            .contains(&"session/load".to_owned()));
    }
}

#[test]
fn missing_state_root_and_unverified_newer_fail_before_catalogue_dispatch() {
    let host_id = ExecutionHostId::new("fixture.kimi.import.newer").unwrap();
    let host = FixtureHost::new(Scenario::ReasoningNewerSuccess);
    let prepared = prepared(&host, host_id, "0.37.0");
    let failure = prepared
        .prepare_session_catalogue(catalogue_input("unverified"))
        .expect_err("unverified newer cannot inherit catalogue support");
    assert_eq!(
        failure.diagnostic().safe().code(),
        "swallowtail.kimi.preparation.session_catalogue_version_unsupported"
    );
    assert!(!host.process_started());

    let host_id = ExecutionHostId::new("fixture.kimi.import.no-state").unwrap();
    let host = FixtureHost::new(Scenario::Complete);
    let prepared = prepared_with_state_root(&host, host_id, "0.28.1", None);
    let failure = prepared
        .prepare_session_catalogue(catalogue_input("no-state"))
        .expect_err("catalogue requires explicit state-root identity");
    assert_eq!(
        failure.diagnostic().safe().code(),
        "swallowtail.kimi.preparation.session_catalogue_state_root_missing"
    );
    assert!(!host.process_started());
}

#[test]
fn state_root_drift_and_changed_candidates_issue_no_binding() {
    let host_id = ExecutionHostId::new("fixture.kimi.import.drift").unwrap();
    let source_host = FixtureHost::new(Scenario::Complete);
    let source = prepared_with_state_root(
        &source_host,
        host_id.clone(),
        "0.28.1",
        Some("fixture.kimi.state-a"),
    );
    let catalogue = source
        .prepare_session_catalogue(catalogue_input("drift"))
        .expect("source catalogue prepares");
    let list_host = FixtureHost::new(Scenario::Complete);
    let outcome = block_on(catalogue.list_sessions(list_host.services(host_id.clone())))
        .expect("source catalogue lists");
    let candidate = outcome.candidates().next().unwrap().clone();

    let other_host = FixtureHost::new(Scenario::Complete);
    let other = prepared_with_state_root(
        &other_host,
        host_id.clone(),
        "0.28.1",
        Some("fixture.kimi.state-b"),
    );
    let mismatch = other
        .prepare_session_import(
            &catalogue,
            candidate.clone(),
            profile_input("state-drift", SessionOptions::default()),
        )
        .expect_err("candidate cannot cross Kimi state roots");
    assert_eq!(
        mismatch.diagnostic().safe().code(),
        "swallowtail.kimi.preparation.session_import_source_mismatch"
    );
    assert!(!other_host.process_started());

    let import = source
        .prepare_session_import(
            &catalogue,
            candidate,
            profile_input("candidate-drift", SessionOptions::default()),
        )
        .expect("matching source import prepares");
    let changed_host = FixtureHost::new(Scenario::CatalogueChanged);
    let failure = block_on(import.import_session(changed_host.services(host_id)))
        .expect_err("changed candidate issues no binding");
    assert_eq!(
        failure.stage(),
        swallowtail_runtime::ProviderSessionOperationFailureStage::ImportRevalidation
    );
}

pub(crate) fn catalogue_input(suffix: &str) -> KimiSessionCatalogueInput {
    KimiSessionCatalogueInput::new(
        RequestId::new(format!("kimi-list-{suffix}")).unwrap(),
        ProviderSessionCatalogueId::new(format!("kimi-catalogue-{suffix}")).unwrap(),
        swallowtail_runtime::WorkingResourceRef::new("kimi.prepared.workspace").unwrap(),
        ProviderSessionCatalogueBounds::new(
            NonZeroU32::new(32).unwrap(),
            NonZeroU32::new(128).unwrap(),
            NonZeroU32::new(512).unwrap(),
            NonZeroU32::new(4_096).unwrap(),
            NonZeroU32::new(512).unwrap(),
        )
        .unwrap(),
    )
}

fn scenario(version: &str) -> Scenario {
    match version {
        "0.28.1" => Scenario::Complete,
        "0.29.0" => Scenario::ReasoningEffortSuccess,
        "0.29.1" => Scenario::ReasoningEffort291Success,
        "0.29.2" => Scenario::ReasoningEffort292Success,
        "0.30.0" => Scenario::ReasoningEffort300Success,
        "0.31.0" => Scenario::ReasoningEffort310Success,
        "0.31.1" => Scenario::ReasoningEffort311Success,
        _ => panic!("unsupported fixture version"),
    }
}
