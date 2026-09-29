use super::ledger::{ACP, observed_tuples};
use super::{FixtureHost, Scenario, host_id, prepared, profile_input, source};
use crate::provider_session_import::catalogue_input;
use futures_executor::block_on;
use std::collections::{BTreeMap, BTreeSet};
use swallowtail_core::{HarnessMode, OperationShape, ReasoningMode};
use swallowtail_runtime::SessionOptions;
use swallowtail_testkit::{
    ConsumerRouteLedgerClaim, assert_consumer_route_ledger_emitted_by_facade,
};

#[test]
fn acp_ledger_matches_the_prepared_and_observed_facades() {
    const ROUTE: &str = "kimi-code.acp";
    const MINIMAL: &str = "session.minimal";
    const MAXIMAL: &str = "session.maximal-open";
    const CATALOGUE: &str = "catalogue";
    const IMPORT: &str = "import";
    const CATALOGUE_OUTCOME: &str = "catalogue.outcome";

    let host_id = host_id("ledger");
    let preparation_host = FixtureHost::new(Scenario::ReasoningEffortSuccess);
    let integration = prepared(&preparation_host, host_id.clone(), "0.29.0");
    let minimal = integration
        .prepare_session(profile_input("ledger-minimal", SessionOptions::default()))
        .expect("minimal session prepares")
        .consumer_route_projection_contribution(source("kimi.ledger.minimal"))
        .expect("minimal projection");
    let options = SessionOptions::default()
        .with_reasoning_mode(ReasoningMode::new("high").expect("reasoning"))
        .with_harness_mode(HarnessMode::Plan);
    let maximal = integration
        .prepare_session(profile_input("ledger-maximal", options))
        .expect("maximal session prepares");
    let open_host = FixtureHost::new(Scenario::ReasoningEffortSuccess);
    let open_services = open_host.services(host_id.clone());
    let opened = block_on(maximal.open_session_with_projection(
        source("kimi.ledger.maximal.prepared"),
        source("kimi.ledger.maximal.active"),
        open_services.clone(),
    ))
    .unwrap_or_else(|failure| panic!("maximal open: {}", failure.failure()));
    let maximal_rows = observed_tuples(ROUTE, opened.contribution());
    let (handle, _) = opened.into_parts();
    assert_eq!(
        block_on(crate::support::close_session(handle, open_services)),
        swallowtail_runtime::CleanupOutcome::Clean
    );

    let catalogue = integration
        .prepare_session_catalogue(catalogue_input("ledger"))
        .expect("catalogue prepares");
    let catalogue_rows = catalogue
        .consumer_route_projection_contribution(source("kimi.ledger.catalogue"))
        .expect("catalogue projection");
    let catalogue_host = FixtureHost::new(Scenario::ReasoningEffortSuccess);
    let outcome = block_on(catalogue.list_sessions(catalogue_host.services(host_id)))
        .expect("catalogue completes");
    let observation = catalogue
        .consumer_route_provider_operation_observation(
            &outcome,
            source("kimi.ledger.catalogue.outcome"),
        )
        .expect("catalogue observation");
    let observed_control = observation
        .rows()
        .map(|row| {
            assert_eq!(
                row.applicability().operation_shape(),
                OperationShape::ProviderSessionCatalogue
            );
            let semantic = row
                .identity()
                .namespaced_extension()
                .expect("namespaced catalogue control")
                .semantic_id();
            (
                ROUTE.to_owned(),
                "session-management".to_owned(),
                semantic.to_owned(),
            )
        })
        .collect::<BTreeSet<_>>();
    let candidate = outcome.candidates().next().expect("candidate").clone();
    let imported = integration
        .prepare_session_import(
            &catalogue,
            candidate,
            profile_input("ledger-import", SessionOptions::default()),
        )
        .expect("import prepares")
        .consumer_route_projection_contribution(source("kimi.ledger.import"))
        .expect("import projection");

    let observed = BTreeMap::from([
        (MINIMAL, observed_tuples(ROUTE, &minimal)),
        (MAXIMAL, maximal_rows),
        (CATALOGUE, observed_tuples(ROUTE, &catalogue_rows)),
        (IMPORT, observed_tuples(ROUTE, &imported)),
        (CATALOGUE_OUTCOME, observed_control),
    ]);
    assert_consumer_route_ledger_emitted_by_facade(
        &observed,
        ACP.iter()
            .map(|(shape, semantic, emitted)| ConsumerRouteLedgerClaim {
                identity: (
                    ROUTE.to_owned(),
                    (*shape).to_owned(),
                    (*semantic).to_owned(),
                ),
                emitted_by: if *emitted {
                    acp_emitters(semantic)
                } else {
                    &[]
                },
            }),
    );
}

fn acp_emitters(semantic: &str) -> &'static [&'static str] {
    const MINIMAL: &str = "session.minimal";
    const MAXIMAL: &str = "session.maximal-open";
    const CATALOGUE: &str = "catalogue";
    const IMPORT: &str = "import";
    const CATALOGUE_OUTCOME: &str = "catalogue.outcome";
    match semantic {
        "feature.interactive-session"
        | "feature.streaming-events"
        | "feature.cancellation-or-interruption"
        | "feature.bounded-workspace-text-write"
        | "feature.activity-observation"
        | "feature.load-session"
        | "feature.resume-session"
        | "control.model-selection"
        | "control.session-options" => &[MINIMAL, MAXIMAL],
        "feature.reasoning-selection"
        | "feature.active-session-reasoning-and-plan-ack"
        | "feature.negotiated-model-options-observation"
        | "control.reasoning-selection" => &[MAXIMAL],
        "control.load-session" | "control.resume-session" => &[MINIMAL],
        "feature.working-resource" | "feature.prepared-facade" => {
            &[MINIMAL, MAXIMAL, CATALOGUE, IMPORT]
        }
        "feature.persistent-session-posture" => &[IMPORT],
        "feature.provider-session-catalogue" => &[CATALOGUE],
        "feature.provider-session-import" | "control.provider-session-import" => &[IMPORT],
        "control.provider-session-catalogue" => &[CATALOGUE_OUTCOME],
        other => panic!("unmapped ACP ledger semantic {other}"),
    }
}
