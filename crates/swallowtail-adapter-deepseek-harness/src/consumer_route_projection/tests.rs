use std::collections::{BTreeMap, BTreeSet};
use swallowtail_testkit::{
    ConsumerRouteLedgerClaim, assert_consumer_route_ledger_emitted_by_facade,
};

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

    const JSONRPC: &str = "jsonrpc.run";
    const WEB_RUN: &str = "web.run";
    const WEB_CATALOGUE: &str = "web.catalogue";
    const WEB_HISTORY: &str = "web.history";
    const WEB_FORK: &str = "web.fork";
    const WEB_ARCHIVE: &str = "web.archive";
    const WEB_ALL: &[&str] = &[WEB_RUN, WEB_CATALOGUE, WEB_HISTORY, WEB_FORK, WEB_ARCHIVE];
    const LEDGER: [(&str, &str, &str, &[&str]); 26] = [
        (
            JSONRPC_ROUTE,
            "structured-run",
            "feature.structured-run",
            &[JSONRPC],
        ),
        (
            JSONRPC_ROUTE,
            "route-observation",
            "feature.streaming-events",
            &[JSONRPC],
        ),
        (
            JSONRPC_ROUTE,
            "route-observation",
            "feature.usage-evidence",
            &[JSONRPC],
        ),
        (
            JSONRPC_ROUTE,
            "route-capability",
            "feature.cancellation-or-interruption",
            &[JSONRPC],
        ),
        (
            JSONRPC_ROUTE,
            "route-capability",
            "feature.working-resource",
            &[JSONRPC],
        ),
        (
            JSONRPC_ROUTE,
            "route-capability",
            "feature.prepared-facade",
            &[JSONRPC],
        ),
        (
            JSONRPC_ROUTE,
            "route-observation",
            "feature.activity-observation",
            &[JSONRPC],
        ),
        (
            JSONRPC_ROUTE,
            "structured-run",
            "control.model-selection",
            &[JSONRPC],
        ),
        (
            WEB_ROUTE,
            "structured-run",
            "feature.structured-run",
            &[WEB_RUN],
        ),
        (
            WEB_ROUTE,
            "route-observation",
            "feature.streaming-events",
            &[WEB_RUN],
        ),
        (
            WEB_ROUTE,
            "route-observation",
            "feature.usage-evidence",
            &[WEB_RUN],
        ),
        (
            WEB_ROUTE,
            "route-capability",
            "feature.cancellation-or-interruption",
            &[WEB_RUN],
        ),
        (
            WEB_ROUTE,
            "route-capability",
            "feature.working-resource",
            &[WEB_RUN, WEB_CATALOGUE],
        ),
        (
            WEB_ROUTE,
            "route-capability",
            "feature.prepared-facade",
            WEB_ALL,
        ),
        (
            WEB_ROUTE,
            "route-observation",
            "feature.activity-observation",
            &[WEB_RUN],
        ),
        (
            WEB_ROUTE,
            "structured-run",
            "control.model-selection",
            &[WEB_RUN],
        ),
        (
            WEB_ROUTE,
            "session-lifecycle",
            "feature.provider-session-catalogue",
            &[WEB_CATALOGUE],
        ),
        (
            WEB_ROUTE,
            "session-lifecycle",
            "feature.provider-session-archive",
            &[WEB_ARCHIVE],
        ),
        (
            WEB_ROUTE,
            "session-management",
            "control.provider-session-fork",
            &[WEB_FORK],
        ),
        (
            WEB_ROUTE,
            "session-management",
            "control.provider-session-archive",
            &[WEB_ARCHIVE],
        ),
        (
            JSONRPC_ROUTE,
            "model-catalogue",
            "feature.model-catalogue",
            &[],
        ),
        (
            JSONRPC_ROUTE,
            "session-lifecycle",
            "feature.persistent-session-posture",
            &[],
        ),
        (
            JSONRPC_ROUTE,
            "route-capability",
            "feature.owned-runtime-lifecycle",
            &[],
        ),
        (WEB_ROUTE, "model-catalogue", "feature.model-catalogue", &[]),
        (
            WEB_ROUTE,
            "session-lifecycle",
            "feature.persistent-session-posture",
            &[],
        ),
        (
            WEB_ROUTE,
            "route-capability",
            "feature.owned-runtime-lifecycle",
            &[],
        ),
    ];
    let observed = BTreeMap::from([
        (JSONRPC, observed_tuples(JSONRPC_ROUTE, &jsonrpc)),
        (WEB_RUN, observed_tuples(WEB_ROUTE, &run)),
        (WEB_CATALOGUE, observed_tuples(WEB_ROUTE, &catalogue_rows)),
        (WEB_HISTORY, observed_tuples(WEB_ROUTE, &history_rows)),
        (WEB_FORK, observed_tuples(WEB_ROUTE, &fork)),
        (WEB_ARCHIVE, observed_tuples(WEB_ROUTE, &archive)),
    ]);
    assert_consumer_route_ledger_emitted_by_facade(
        &observed,
        LEDGER.iter().map(
            |(route, shape, semantic, emitted_by)| ConsumerRouteLedgerClaim {
                identity: (*route, *shape, *semantic),
                emitted_by,
            },
        ),
    );
    let prepared_emitted = 19 + jsonrpc_rows.len() + web_rows.len();
    assert_eq!(prepared_emitted, 39);
    assert_eq!(prepared_emitted + 2, 41);
    assert_eq!(prepared_emitted + 2 + 6, 47);
}

fn observed_tuples(
    route: &'static str,
    contribution: &ConsumerRouteProjectionContribution,
) -> BTreeSet<(&'static str, &'static str, &'static str)> {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
        .map(|row| {
            let (shape, semantic) = match row.identity() {
                ConsumerRouteRowIdentity::Feature(feature) => match feature {
                    ConsumerRouteFeatureId::StructuredRun => {
                        ("structured-run", "feature.structured-run")
                    }
                    ConsumerRouteFeatureId::StreamingEvents => {
                        ("route-observation", "feature.streaming-events")
                    }
                    ConsumerRouteFeatureId::UsageEvidence => {
                        ("route-observation", "feature.usage-evidence")
                    }
                    ConsumerRouteFeatureId::CancellationOrInterruption => {
                        ("route-capability", "feature.cancellation-or-interruption")
                    }
                    ConsumerRouteFeatureId::WorkingResource => {
                        ("route-capability", "feature.working-resource")
                    }
                    ConsumerRouteFeatureId::PreparedFacade => {
                        ("route-capability", "feature.prepared-facade")
                    }
                    ConsumerRouteFeatureId::ActivityObservation => {
                        ("route-observation", "feature.activity-observation")
                    }
                    ConsumerRouteFeatureId::ProviderSessionCatalogue => {
                        ("session-lifecycle", "feature.provider-session-catalogue")
                    }
                    ConsumerRouteFeatureId::ProviderSessionArchive => {
                        ("session-lifecycle", "feature.provider-session-archive")
                    }
                    other => panic!("unexpected Harness feature {other:?}"),
                },
                ConsumerRouteRowIdentity::Control(control) => match control {
                    ConsumerRouteControlId::ModelSelection => {
                        ("structured-run", "control.model-selection")
                    }
                    ConsumerRouteControlId::Namespaced(extension) => {
                        match extension.semantic_id() {
                            "control.provider-session-fork" => {
                                ("session-management", "control.provider-session-fork")
                            }
                            "control.provider-session-archive" => {
                                ("session-management", "control.provider-session-archive")
                            }
                            other => panic!("unexpected Harness extension {other}"),
                        }
                    }
                    other => panic!("unexpected Harness control {other:?}"),
                },
            };
            (route, shape, semantic)
        })
        .collect()
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
