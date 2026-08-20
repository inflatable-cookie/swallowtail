use super::{AddableRouteCatalog, AddableRouteCatalogFailureKind};
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, AddableRouteAvailability, AddableRouteDescriptor,
    AddableRouteId, AddableRouteMissingRequirement, ConfigFieldDescriptor, ConfigFieldId,
    ConfigFieldKind, DiscoveryOutcome, DiscoveryStatus, ExecutionLayer, FieldLabel, RouteTopology,
};

fn fixture_driver(adapter: &str) -> AdapterIdentity {
    AdapterIdentity::new(
        AdapterId::new(adapter).expect("fixture adapter id is valid"),
        AdapterVersion::new("0.0.0").expect("fixture adapter version is valid"),
    )
}

fn hosted_available() -> AddableRouteDescriptor {
    AddableRouteDescriptor::new(
        AddableRouteId::new("fixture-hosted-messages").expect("route id is valid"),
        fixture_driver("swallowtail-adapter-fixture-hosted"),
        RouteTopology::Hosted,
        AddableRouteAvailability::Available,
    )
}

fn installed_missing_install() -> AddableRouteDescriptor {
    AddableRouteDescriptor::new(
        AddableRouteId::new("fixture-installed-harness").expect("route id is valid"),
        fixture_driver("swallowtail-adapter-fixture-installed"),
        RouteTopology::Installed,
        AddableRouteAvailability::Unavailable(AddableRouteMissingRequirement::Install),
    )
}

fn local_runtime_missing_runtime() -> AddableRouteDescriptor {
    AddableRouteDescriptor::new(
        AddableRouteId::new("fixture-local-runtime").expect("route id is valid"),
        fixture_driver("swallowtail-adapter-fixture-local"),
        RouteTopology::LocalRuntime,
        AddableRouteAvailability::Unavailable(AddableRouteMissingRequirement::Runtime),
    )
}

fn hosted_missing_host_service() -> AddableRouteDescriptor {
    AddableRouteDescriptor::new(
        AddableRouteId::new("fixture-hosted-subscription").expect("route id is valid"),
        fixture_driver("swallowtail-adapter-fixture-hosted"),
        RouteTopology::Hosted,
        AddableRouteAvailability::Unavailable(AddableRouteMissingRequirement::HostService),
    )
}

fn installed_unsupported() -> AddableRouteDescriptor {
    AddableRouteDescriptor::new(
        AddableRouteId::new("fixture-installed-unsupported").expect("route id is valid"),
        fixture_driver("swallowtail-adapter-fixture-installed"),
        RouteTopology::Installed,
        AddableRouteAvailability::Unsupported,
    )
}

fn installed_available_with_config() -> AddableRouteDescriptor {
    AddableRouteDescriptor::new(
        AddableRouteId::new("fixture-installed-binary").expect("route id is valid"),
        fixture_driver("swallowtail-adapter-fixture-installed"),
        RouteTopology::Installed,
        AddableRouteAvailability::Available,
    )
    .with_config_fields([
        ConfigFieldDescriptor::new(
            ConfigFieldId::new("binary").expect("config id is valid"),
            FieldLabel::new("Binary").expect("label is valid"),
            ConfigFieldKind::BinaryPath,
        ),
        ConfigFieldDescriptor::new(
            ConfigFieldId::new("endpoint").expect("config id is valid"),
            FieldLabel::new("Endpoint").expect("label is valid"),
            ConfigFieldKind::ApiEndpoint,
        ),
        ConfigFieldDescriptor::new(
            ConfigFieldId::new("environment").expect("config id is valid"),
            FieldLabel::new("Environment").expect("label is valid"),
            ConfigFieldKind::Environment,
        ),
    ])
}

#[test]
fn catalog_assembles_adapter_local_descriptors_without_a_registry() {
    let catalog = AddableRouteCatalog::from_descriptors([
        hosted_available(),
        installed_missing_install(),
        local_runtime_missing_runtime(),
    ])
    .expect("catalog assembles");

    assert_eq!(catalog.routes().len(), 3);
    assert!(
        catalog
            .get(&AddableRouteId::new("fixture-hosted-messages").expect("route id is valid"))
            .is_some()
    );
}

#[test]
fn topology_groups_are_hosted_installed_and_local_runtime_not_execution_layer() {
    let catalog = AddableRouteCatalog::from_descriptors([
        hosted_available(),
        installed_missing_install(),
        local_runtime_missing_runtime(),
    ])
    .expect("catalog assembles");

    let hosted: Vec<_> = catalog
        .routes_with_topology(RouteTopology::Hosted)
        .map(|route| route.id().as_str())
        .collect();
    let installed: Vec<_> = catalog
        .routes_with_topology(RouteTopology::Installed)
        .map(|route| route.id().as_str())
        .collect();
    let local: Vec<_> = catalog
        .routes_with_topology(RouteTopology::LocalRuntime)
        .map(|route| route.id().as_str())
        .collect();

    assert_eq!(hosted, vec!["fixture-hosted-messages"]);
    assert_eq!(installed, vec!["fixture-installed-harness"]);
    assert_eq!(local, vec!["fixture-local-runtime"]);
    assert_ne!(
        format!("{:?}", RouteTopology::Installed),
        format!("{:?}", ExecutionLayer::HarnessInteraction)
    );
}

#[test]
fn unavailable_names_a_missing_install_runtime_or_host_service() {
    let catalog = AddableRouteCatalog::from_descriptors([
        installed_missing_install(),
        local_runtime_missing_runtime(),
        hosted_missing_host_service(),
    ])
    .expect("catalog assembles");

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
}

#[test]
fn unsupported_is_distinct_from_unavailable_and_from_an_unlinked_adapter() {
    let catalog = AddableRouteCatalog::from_descriptors([installed_unsupported()])
        .expect("catalog assembles");
    let unsupported_id =
        AddableRouteId::new("fixture-installed-unsupported").expect("route id is valid");
    let unlinked_id = AddableRouteId::new("fixture-local-runtime").expect("route id is valid");

    assert_eq!(
        catalog
            .get(&unsupported_id)
            .map(AddableRouteDescriptor::availability),
        Some(AddableRouteAvailability::Unsupported)
    );
    assert!(catalog.get(&unlinked_id).is_none());
}

#[test]
fn discovery_candidates_are_not_catalog_rows() {
    let discovery = DiscoveryOutcome::new(DiscoveryStatus::Discovered, None);
    let catalog =
        AddableRouteCatalog::from_descriptors([hosted_available()]).expect("catalog assembles");
    let discovered_id = AddableRouteId::new("discovered-candidate").expect("route id is valid");

    assert_eq!(discovery.status(), DiscoveryStatus::Discovered);
    assert!(catalog.get(&discovered_id).is_none());
    assert_eq!(catalog.routes().len(), 1);
    assert_eq!(
        catalog.routes().next().map(|route| route.id().as_str()),
        Some("fixture-hosted-messages")
    );
}

#[test]
fn duplicate_route_ids_fail_closed() {
    let error = AddableRouteCatalog::from_descriptors([hosted_available(), hosted_available()])
        .expect_err("duplicate route must fail");

    assert_eq!(error.kind(), AddableRouteCatalogFailureKind::DuplicateRoute);
}

#[test]
fn available_installed_descriptor_can_advertise_config_fields() {
    let catalog = AddableRouteCatalog::from_descriptors([installed_available_with_config()])
        .expect("catalog assembles");
    assert_eq!(
        catalog
            .get(&AddableRouteId::new("fixture-installed-binary").expect("route id is valid"))
            .map(|route| route.config_fields().len()),
        Some(3)
    );
}
