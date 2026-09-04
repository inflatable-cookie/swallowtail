use std::collections::BTreeSet;

use super::*;
use crate::{
    DeepSeekHarnessModelSelection, DeepSeekHarnessRunProfileInput, DeepSeekHarnessWebForkInput,
    DeepSeekHarnessWebModelSelection, DeepSeekHarnessWebRunProfileInput,
    DeepSeekHarnessWebSessionCatalogueInput, DeepSeekHarnessWebSessionHistoryInput,
    DeepSeekHarnessWebSessionManagementInput,
};
use swallowtail_core::{
    ModelId, ModelRouteId, ModelRouteRevision, ProviderId, ProviderSessionBindingOrigin, SessionRef,
};
use swallowtail_runtime::{
    CleanupOutcome, ConsumerRouteProjectionContribution, Deadline, MonotonicInstant,
    OperationContent, ProviderSessionCatalogueId, ProviderSessionHistoryId,
    ProviderSessionHistoryTotal, RequestId, ResourceAccess, SessionAccessPolicy,
    SessionResumeBinding, WorkingResourceRef, page_provider_session_history_window,
};

#[path = "tests/support.rs"]
mod support;
use support::*;

#[test]
fn candidate_i_harness_contributions_reconcile_the_actual_ledger() {
    let jsonrpc = crate::prepared::tests::prepared_integration()
        .prepare_run(DeepSeekHarnessRunProfileInput::new(
            request("jsonrpc-run"),
            DeepSeekHarnessModelSelection::new(route(), revision(), provider(), model()),
            content(),
            resource(),
            deadline(),
        ))
        .unwrap()
        .consumer_route_projection_contribution(source("jsonrpc.prepared"))
        .unwrap();
    let jsonrpc_rows = rows(&jsonrpc).collect::<BTreeSet<_>>();
    assert_eq!(
        jsonrpc_rows,
        [
            feature(ConsumerRouteFeatureId::StructuredRun),
            feature(ConsumerRouteFeatureId::StreamingEvents),
            feature(ConsumerRouteFeatureId::UsageEvidence),
            feature(ConsumerRouteFeatureId::CancellationOrInterruption),
            feature(ConsumerRouteFeatureId::WorkingResource),
            feature(ConsumerRouteFeatureId::PreparedFacade),
            feature(ConsumerRouteFeatureId::ActivityObservation),
            control(ConsumerRouteControlId::ModelSelection),
        ]
        .into_iter()
        .collect()
    );

    let web = crate::web_prepared::tests::prepared();
    let run = web
        .prepare_run(DeepSeekHarnessWebRunProfileInput::new(
            request("web-run"),
            web_model(),
            content(),
            resource(),
            deadline(),
        ))
        .unwrap()
        .consumer_route_projection_contribution(source("web.run.prepared"))
        .unwrap();
    let catalogue = catalogue(&web, "ledger");
    let catalogue_rows = catalogue
        .consumer_route_projection_contribution(source("web.catalogue.prepared"))
        .unwrap();
    let session = SessionRef::new("projection-session").unwrap();
    let history = history(&web, session.clone(), "ledger");
    let history_rows = history
        .consumer_route_projection_contribution(source("web.history.prepared"))
        .unwrap();
    let fork = catalogue
        .prepare_fork(DeepSeekHarnessWebForkInput::new(
            request("fork"),
            session.clone(),
        ))
        .consumer_route_projection_contribution(source("web.fork.prepared"))
        .unwrap();
    let archive = web
        .prepare_archive_session(DeepSeekHarnessWebSessionManagementInput::new(
            request("archive"),
            web.management_binding(
                session,
                Some(resource()),
                ProviderSessionBindingOrigin::Loaded,
            )
            .unwrap(),
        ))
        .unwrap()
        .consumer_route_projection_contribution(source("web.archive.prepared"))
        .unwrap();

    let web_rows = [&run, &catalogue_rows, &history_rows, &fork, &archive]
        .into_iter()
        .flat_map(rows)
        .collect::<BTreeSet<_>>();
    assert_eq!(web_rows.len(), 12);
    let standard_web_rows = web_rows
        .iter()
        .filter(|identity| identity.namespaced_extension().is_none())
        .cloned()
        .collect::<BTreeSet<_>>();
    assert_eq!(
        standard_web_rows,
        [
            feature(ConsumerRouteFeatureId::StructuredRun),
            feature(ConsumerRouteFeatureId::StreamingEvents),
            feature(ConsumerRouteFeatureId::UsageEvidence),
            feature(ConsumerRouteFeatureId::CancellationOrInterruption),
            feature(ConsumerRouteFeatureId::ProviderSessionCatalogue),
            feature(ConsumerRouteFeatureId::WorkingResource),
            feature(ConsumerRouteFeatureId::ProviderSessionArchive),
            feature(ConsumerRouteFeatureId::PreparedFacade),
            feature(ConsumerRouteFeatureId::ActivityObservation),
            control(ConsumerRouteControlId::ModelSelection),
        ]
        .into_iter()
        .collect()
    );
    assert_eq!(
        web_rows
            .iter()
            .filter_map(ConsumerRouteRowIdentity::namespaced_extension)
            .map(ConsumerRouteNamespacedExtension::semantic_id)
            .collect::<BTreeSet<_>>(),
        [
            "control.provider-session-archive",
            "control.provider-session-fork"
        ]
        .into_iter()
        .collect()
    );
    for withheld in [
        ConsumerRouteFeatureId::ModelCatalogue,
        ConsumerRouteFeatureId::PersistentSessionPosture,
    ] {
        assert!(!jsonrpc_rows.contains(&ConsumerRouteRowIdentity::Feature(withheld.clone())));
        assert!(!web_rows.contains(&ConsumerRouteRowIdentity::Feature(withheld)));
    }
    assert!(!jsonrpc_rows.iter().any(owned_runtime_lifecycle));
    assert!(!web_rows.iter().any(owned_runtime_lifecycle));

    let prepared_emitted = 19 + jsonrpc_rows.len() + web_rows.len();
    assert_eq!(prepared_emitted, 39);
    assert_eq!(prepared_emitted + 2, 41);
    assert_eq!(prepared_emitted + 2 + 6, 47);
}

#[test]
fn completed_catalogue_and_history_outcomes_are_the_only_observation_admission() {
    let web = crate::web_prepared::tests::prepared();
    let prepared_catalogue = catalogue(&web, "matching");
    let catalogue_outcome = ProviderSessionCatalogueOutcome::new(
        prepared_catalogue.plan(),
        prepared_catalogue.request(),
        vec![],
        None,
        CleanupOutcome::NotApplicable,
    )
    .unwrap();
    let catalogue_observation = prepared_catalogue
        .consumer_route_provider_operation_observation(
            &catalogue_outcome,
            source("web.catalogue.outcome"),
        )
        .unwrap();
    assert_observation(
        &catalogue_observation,
        "control.provider-session-catalogue",
        &prepared_catalogue
            .consumer_route_projection_contribution(source("web.catalogue.prepared"))
            .unwrap(),
    );

    let prepared_history = history(
        &web,
        SessionRef::new("projection-history").unwrap(),
        "matching",
    );
    let window = page_provider_session_history_window(
        prepared_history.plan(),
        prepared_history.request(),
        vec![],
        ProviderSessionHistoryTotal::Exact(0),
    )
    .unwrap();
    let history_outcome = ProviderSessionHistoryPage::new(
        prepared_history.plan(),
        prepared_history.request(),
        window,
        CleanupOutcome::NotApplicable,
    )
    .unwrap();
    let history_observation = prepared_history
        .consumer_route_provider_operation_observation(
            &history_outcome,
            source("web.history.outcome"),
        )
        .unwrap();
    assert_observation(
        &history_observation,
        "control.provider-session-history",
        &prepared_history
            .consumer_route_projection_contribution(source("web.history.prepared"))
            .unwrap(),
    );

    let other_web = crate::web_prepared::tests::prepared_with_suffix("mismatch");
    assert!(
        catalogue(&other_web, "mismatch")
            .consumer_route_provider_operation_observation(
                &catalogue_outcome,
                source("web.catalogue.mismatch"),
            )
            .is_err()
    );
    assert!(
        history(
            &other_web,
            SessionRef::new("projection-history-other").unwrap(),
            "mismatch"
        )
        .consumer_route_provider_operation_observation(
            &history_outcome,
            source("web.history.mismatch"),
        )
        .is_err()
    );
}

#[test]
fn matrix_only_capabilities_have_no_construction_mapping() {
    assert_eq!(super::builder::feature_for(Capability::ModelCatalog), None);
    assert_eq!(
        super::builder::feature_for(Capability::ProviderSessionHistory),
        None
    );
}
