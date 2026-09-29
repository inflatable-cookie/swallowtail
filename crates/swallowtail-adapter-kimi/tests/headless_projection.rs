use crate::support::{local_topology, prepared, profile};
use std::collections::BTreeMap;
use swallowtail_testkit::{
    ConsumerRouteLedgerClaim, assert_consumer_route_ledger_emitted_by_facade,
};
#[path = "consumer_route_projection/ledger.rs"]
mod ledger;
use swallowtail_runtime::{
    ConsumerRouteFeatureId, ConsumerRouteProjectionSourceId, ConsumerRouteRowIdentity,
};

#[test]
fn headless_projection_matches_the_ten_emitted_row_ledger() {
    let topology = local_topology();
    let prepared = prepared(topology.execution_host_id().clone());
    let run = profile(&prepared, topology.working_resource().clone(), "projection");
    let contribution = run
        .consumer_route_projection_contribution(
            ConsumerRouteProjectionSourceId::new("kimi.headless.projection").expect("source"),
        )
        .expect("headless contribution is admitted");
    let rows = contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
        .collect::<Vec<_>>();
    assert_eq!(
        rows.len(),
        10,
        "unexpected rows: {:?}",
        rows.iter().map(|row| row.identity()).collect::<Vec<_>>()
    );
    const ROUTE: &str = "kimi-code.headless";
    const RUN: &str = "run";
    let observed = BTreeMap::from([(RUN, ledger::observed_tuples(ROUTE, &contribution))]);
    assert_consumer_route_ledger_emitted_by_facade(
        &observed,
        ledger::HEADLESS
            .iter()
            .map(|(shape, semantic, emitted)| ConsumerRouteLedgerClaim {
                identity: (
                    ROUTE.to_owned(),
                    (*shape).to_owned(),
                    (*semantic).to_owned(),
                ),
                emitted_by: if *emitted { &[RUN] } else { &[] },
            }),
    );
    for withheld in [
        ConsumerRouteFeatureId::ModelCatalogue,
        ConsumerRouteFeatureId::InteractiveSession,
        ConsumerRouteFeatureId::ReasoningSelection,
        ConsumerRouteFeatureId::LoadSession,
        ConsumerRouteFeatureId::ResumeSession,
        ConsumerRouteFeatureId::BoundedWorkspaceTextWrite,
        ConsumerRouteFeatureId::ProviderSessionCatalogue,
        ConsumerRouteFeatureId::ProviderSessionImport,
    ] {
        assert!(
            !rows.iter().any(|row| {
                row.identity() == &ConsumerRouteRowIdentity::Feature(withheld.clone())
            })
        );
    }
}
