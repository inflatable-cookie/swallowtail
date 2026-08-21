use super::{driver, make_host_id};
use crate::support::{
    CleanupEvent, SidecarFixtureHost, SidecarScenario, sidecar_catalogue_selection,
};
use futures_executor::block_on;
use swallowtail_runtime::{ModelCatalogDriver, ModelCatalogRequest, RequestId};

#[test]
fn catalogue_lists_bounded_models_from_the_explicit_runtime() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.catalogue");
    let fixture = SidecarFixtureHost::new(SidecarScenario::Complete);
    let selected = sidecar_catalogue_selection(host_id.clone());
    let models = block_on(driver(selected.credential.clone()).list_models(
        selected.plan,
        ModelCatalogRequest::new(RequestId::new("sidecar-catalogue").expect("valid request")),
        fixture.services(host_id),
    ))
    .expect("sidecar catalogue loads");
    assert_eq!(models.len(), 2);
    assert_eq!(models[0].id().as_str(), "fixture-model");
    assert_eq!(
        models[0].provider_id().map(|provider| provider.as_str()),
        Some("fixture-provider")
    );
    assert_eq!(fixture.process_arguments(), Vec::<String>::new());
    let inputs = fixture.inputs();
    assert_eq!(inputs[0]["command"], "bootstrap");
    assert_eq!(inputs[0]["params"]["catalogueOnly"], true);
    assert!(
        inputs.iter().any(|value| value["command"] == "close"),
        "catalogue sidecar is closed and joined"
    );
    assert_eq!(
        fixture.cleanup_events(),
        [CleanupEvent::ProcessWait, CleanupEvent::CredentialRelease,]
    );
    assert!(!format!("{models:?}").contains("fixture-private"));
}
