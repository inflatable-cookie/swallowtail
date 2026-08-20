use swallowtail_core::{
    AddableRouteAvailability, AddableRouteDescriptor, AddableRouteId,
    AddableRouteMissingRequirement, ConfigFieldKind, DiscoveryOutcome, DiscoveryStatus,
    ExecutionLayer, RouteTopology,
};
use swallowtail_runtime::AddableRouteCatalog;
use swallowtail_testkit::{
    fixture_hosted_available_descriptor, fixture_hosted_missing_host_service_descriptor,
    fixture_installed_available_with_config_descriptor,
    fixture_installed_missing_install_descriptor, fixture_installed_unsupported_descriptor,
    fixture_local_runtime_missing_runtime_descriptor,
};

#[test]
fn consumer_assembles_a_catalog_from_fixture_descriptors_without_a_registry() {
    let catalog = AddableRouteCatalog::from_descriptors([
        fixture_hosted_available_descriptor(),
        fixture_installed_missing_install_descriptor(),
        fixture_local_runtime_missing_runtime_descriptor(),
        fixture_hosted_missing_host_service_descriptor(),
        fixture_installed_unsupported_descriptor(),
        fixture_installed_available_with_config_descriptor(),
    ])
    .expect("fixture catalog assembles");

    assert_eq!(catalog.routes().len(), 6);
    assert_eq!(
        catalog.routes_with_topology(RouteTopology::Hosted).count(),
        2
    );
    assert_eq!(
        catalog
            .routes_with_topology(RouteTopology::Installed)
            .count(),
        3
    );
    assert_eq!(
        catalog
            .routes_with_topology(RouteTopology::LocalRuntime)
            .count(),
        1
    );
    assert_ne!(
        format!("{:?}", RouteTopology::Hosted),
        format!("{:?}", ExecutionLayer::DirectModelInference)
    );
}

#[test]
fn absence_of_a_descriptor_means_the_consumer_did_not_link_that_adapter() {
    let catalog = AddableRouteCatalog::from_descriptors([fixture_hosted_available_descriptor()])
        .expect("partial catalog assembles");
    let unlinked = AddableRouteId::new("fixture-local-runtime").expect("route id is valid");

    assert!(catalog.get(&unlinked).is_none());
    assert_eq!(
        catalog
            .get(&AddableRouteId::new("fixture-hosted-messages").expect("route id is valid"))
            .map(AddableRouteDescriptor::availability),
        Some(AddableRouteAvailability::Available)
    );
}

#[test]
fn unavailable_and_unsupported_fixture_observations_stay_distinct() {
    let catalog = AddableRouteCatalog::from_descriptors([
        fixture_installed_missing_install_descriptor(),
        fixture_local_runtime_missing_runtime_descriptor(),
        fixture_hosted_missing_host_service_descriptor(),
        fixture_installed_unsupported_descriptor(),
    ])
    .expect("observation catalog assembles");

    assert_eq!(
        catalog
            .get(&AddableRouteId::new("fixture-installed-harness").expect("route id is valid"))
            .map(AddableRouteDescriptor::availability),
        Some(AddableRouteAvailability::Unavailable(
            AddableRouteMissingRequirement::Install
        ))
    );
    assert_eq!(
        catalog
            .get(&AddableRouteId::new("fixture-local-runtime").expect("route id is valid"))
            .map(AddableRouteDescriptor::availability),
        Some(AddableRouteAvailability::Unavailable(
            AddableRouteMissingRequirement::Runtime
        ))
    );
    assert_eq!(
        catalog
            .get(&AddableRouteId::new("fixture-hosted-subscription").expect("route id is valid"))
            .map(AddableRouteDescriptor::availability),
        Some(AddableRouteAvailability::Unavailable(
            AddableRouteMissingRequirement::HostService
        ))
    );
    assert_eq!(
        catalog
            .get(&AddableRouteId::new("fixture-installed-unsupported").expect("route id is valid"))
            .map(AddableRouteDescriptor::availability),
        Some(AddableRouteAvailability::Unsupported)
    );
}

#[test]
fn discovery_candidates_are_not_fixture_catalog_rows() {
    let discovery = DiscoveryOutcome::new(DiscoveryStatus::Discovered, None);
    let catalog = AddableRouteCatalog::from_descriptors([fixture_hosted_available_descriptor()])
        .expect("fixture catalog assembles");

    assert_eq!(discovery.status(), DiscoveryStatus::Discovered);
    assert!(
        catalog
            .get(&AddableRouteId::new("discovered-candidate").expect("route id is valid"))
            .is_none()
    );
}

#[test]
fn installed_config_fixture_advertises_host_private_field_kinds() {
    let descriptor = fixture_installed_available_with_config_descriptor();
    let kinds: Vec<_> = descriptor
        .config_fields()
        .map(swallowtail_core::ConfigFieldDescriptor::kind)
        .collect();

    assert_eq!(
        kinds,
        vec![
            ConfigFieldKind::BinaryPath,
            ConfigFieldKind::ApiEndpoint,
            ConfigFieldKind::Environment,
        ]
    );
}
