use super::*;
use crate::provider_session_import::catalogue_input;

#[test]
fn catalogue_control_requires_a_completed_provider_operation_outcome() {
    let host_id = host_id("catalogue");
    let preparation_host = FixtureHost::new(Scenario::Complete);
    let integration = prepared(&preparation_host, host_id.clone(), "0.29.0");
    let catalogue = integration
        .prepare_session_catalogue(catalogue_input("projection"))
        .expect("catalogue prepares");
    let prepared_contribution = catalogue
        .consumer_route_projection_contribution(source("kimi.catalogue.prepared"))
        .expect("prepared catalogue contribution is admitted");
    assert!(has_feature(
        &prepared_contribution,
        ConsumerRouteFeatureId::ProviderSessionCatalogue
    ));
    assert!(!prepared_contribution.session_start_rows().any(|row| {
        row.identity()
            .namespaced_extension()
            .is_some_and(|extension| {
                extension.semantic_id() == "control.provider-session-catalogue"
            })
    }));

    let operation_host = FixtureHost::new(Scenario::ReasoningEffortSuccess);
    let outcome = block_on(catalogue.list_sessions(operation_host.services(host_id)))
        .expect("catalogue completes");
    let observation = catalogue
        .consumer_route_provider_operation_observation(&outcome, source("kimi.catalogue.outcome"))
        .expect("completed outcome admits an observation");
    assert!(observation.rows().any(|row| {
        row.identity()
            .namespaced_extension()
            .is_some_and(|extension| {
                extension.semantic_id() == "control.provider-session-catalogue"
            })
    }));
}
