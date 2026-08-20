//! Contract 057 addable-route descriptor tests for local Ollama attach.

use std::sync::Arc;
use swallowtail_adapter_ollama::{
    OLLAMA_ATTACHED_ADDABLE_ROUTE_ID, OLLAMA_ATTACHED_ENDPOINT_FIELD_ID,
    ollama_attached_addable_route_descriptor, ollama_native_descriptor,
};
use swallowtail_core::{
    AddableRouteAvailability, AddableRouteMissingRequirement, ConfigFieldId, ConfigFieldKind,
    ExecutionHostId, ExecutionLayer, RouteTopology,
};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{AddableRouteCatalog, HostServices, NetworkPolicyService};

fn host_id() -> ExecutionHostId {
    ExecutionHostId::new("ollama.addable.host").expect("host id is valid")
}

fn services_with_network() -> HostServices {
    let host = LocalProcessHost::builder(LocalProcessLimits::default()).build();
    HostServices::new(host_id()).with_network(Arc::new(host) as Arc<dyn NetworkPolicyService>)
}

#[test]
fn descriptor_is_local_runtime_and_matches_the_native_driver() {
    let descriptor = ollama_attached_addable_route_descriptor(&services_with_network());

    assert_eq!(descriptor.id().as_str(), OLLAMA_ATTACHED_ADDABLE_ROUTE_ID);
    assert_eq!(descriptor.id().as_str(), "ollama.attached");
    assert_eq!(descriptor.topology(), RouteTopology::LocalRuntime);
    assert_eq!(
        descriptor.driver(),
        ollama_native_descriptor().identity()
    );
    assert_eq!(
        descriptor.availability(),
        AddableRouteAvailability::Available
    );
    assert_eq!(descriptor.sign_in_actions().len(), 0);
}

#[test]
fn missing_network_service_marks_the_route_unavailable() {
    let services = HostServices::new(host_id());
    let descriptor = ollama_attached_addable_route_descriptor(&services);

    assert_eq!(
        descriptor.availability(),
        AddableRouteAvailability::Unavailable(AddableRouteMissingRequirement::HostService)
    );
}

#[test]
fn local_unauthenticated_path_advertises_no_secret_credential_field() {
    let descriptor = ollama_attached_addable_route_descriptor(&services_with_network());

    assert_eq!(descriptor.credential_fields().len(), 0);
    let debug = format!("{descriptor:?}");
    assert!(!debug.contains("sk-"));
    assert!(!debug.contains("token"));
}

#[test]
fn endpoint_config_field_is_an_opaque_reference_not_a_url() {
    let descriptor = ollama_attached_addable_route_descriptor(&services_with_network());
    let endpoint = descriptor
        .config_field(
            &ConfigFieldId::new(OLLAMA_ATTACHED_ENDPOINT_FIELD_ID).expect("config id is valid"),
        )
        .expect("endpoint field is advertised");

    assert_eq!(endpoint.kind(), ConfigFieldKind::ApiEndpoint);
    let debug = format!("{descriptor:?}");
    assert!(!debug.contains("://"));
    assert!(!debug.contains('/'));
}

#[test]
fn addable_row_does_not_probe_the_runtime() {
    // No Ollama server is listening behind this network service; the row is
    // still Available because reachability stays with prepare_ollama_attached.
    let descriptor = ollama_attached_addable_route_descriptor(&services_with_network());

    assert_eq!(
        descriptor.availability(),
        AddableRouteAvailability::Available
    );
}

#[test]
fn direct_model_inference_execution_layer_is_unchanged() {
    assert!(
        ollama_native_descriptor().supports_execution_layer(ExecutionLayer::DirectModelInference)
    );
    assert!(!ollama_native_descriptor().supports_execution_layer(ExecutionLayer::HarnessInteraction));
}

#[test]
fn catalog_assembles_without_a_registry_crate() {
    let descriptor = ollama_attached_addable_route_descriptor(&services_with_network());
    let catalog = AddableRouteCatalog::from_descriptors([descriptor]).expect("catalog assembles");

    assert_eq!(catalog.routes().len(), 1);
    assert_eq!(
        catalog
            .routes_with_topology(RouteTopology::LocalRuntime)
            .count(),
        1
    );
}
