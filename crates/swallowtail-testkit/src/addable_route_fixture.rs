//! Fixture addable-route descriptors for Contract 057 catalog assembly.
//!
//! These stand in for adapter-local descriptor constructors until first-proof
//! routes exist. They are not a registry of production adapters.

use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, AddableRouteAvailability, AddableRouteDescriptor,
    AddableRouteId, AddableRouteMissingRequirement, ConfigFieldDescriptor, ConfigFieldId,
    ConfigFieldKind, CredentialFieldDescriptor, CredentialFieldId, CredentialFieldVisibility,
    FieldLabel, RouteTopology,
};

fn fixture_driver(adapter: &str) -> AdapterIdentity {
    AdapterIdentity::new(
        AdapterId::new(adapter).expect("fixture adapter id is valid"),
        AdapterVersion::new("0.0.0").expect("fixture adapter version is valid"),
    )
}

/// Hosted, available fixture route with an API-key field and endpoint config.
#[must_use]
pub fn fixture_hosted_available_descriptor() -> AddableRouteDescriptor {
    AddableRouteDescriptor::new(
        AddableRouteId::new("fixture-hosted-messages").expect("fixture route id is valid"),
        fixture_driver("swallowtail-adapter-fixture-hosted"),
        RouteTopology::Hosted,
        AddableRouteAvailability::Available,
    )
    .with_credential_fields([CredentialFieldDescriptor::new(
        CredentialFieldId::new("api_key").expect("fixture field id is valid"),
        FieldLabel::new("API key").expect("fixture label is valid"),
        CredentialFieldVisibility::Secret,
    )])
    .with_config_fields([ConfigFieldDescriptor::new(
        ConfigFieldId::new("endpoint").expect("fixture config id is valid"),
        FieldLabel::new("Endpoint").expect("fixture label is valid"),
        ConfigFieldKind::ApiEndpoint,
    )])
}

/// Installed fixture route unavailable because the install is missing.
#[must_use]
pub fn fixture_installed_missing_install_descriptor() -> AddableRouteDescriptor {
    AddableRouteDescriptor::new(
        AddableRouteId::new("fixture-installed-harness").expect("fixture route id is valid"),
        fixture_driver("swallowtail-adapter-fixture-installed"),
        RouteTopology::Installed,
        AddableRouteAvailability::Unavailable(AddableRouteMissingRequirement::Install),
    )
}

/// Local-runtime fixture route unavailable because the runtime is missing.
#[must_use]
pub fn fixture_local_runtime_missing_runtime_descriptor() -> AddableRouteDescriptor {
    AddableRouteDescriptor::new(
        AddableRouteId::new("fixture-local-runtime").expect("fixture route id is valid"),
        fixture_driver("swallowtail-adapter-fixture-local"),
        RouteTopology::LocalRuntime,
        AddableRouteAvailability::Unavailable(AddableRouteMissingRequirement::Runtime),
    )
}

/// Hosted fixture route unavailable because a host service is missing.
#[must_use]
pub fn fixture_hosted_missing_host_service_descriptor() -> AddableRouteDescriptor {
    AddableRouteDescriptor::new(
        AddableRouteId::new("fixture-hosted-subscription").expect("fixture route id is valid"),
        fixture_driver("swallowtail-adapter-fixture-hosted"),
        RouteTopology::Hosted,
        AddableRouteAvailability::Unavailable(AddableRouteMissingRequirement::HostService),
    )
}

/// Installed fixture route the adapter will not offer on this host.
#[must_use]
pub fn fixture_installed_unsupported_descriptor() -> AddableRouteDescriptor {
    AddableRouteDescriptor::new(
        AddableRouteId::new("fixture-installed-unsupported").expect("fixture route id is valid"),
        fixture_driver("swallowtail-adapter-fixture-installed"),
        RouteTopology::Installed,
        AddableRouteAvailability::Unsupported,
    )
}

/// Installed, available fixture route with binary, endpoint, and environment fields.
#[must_use]
pub fn fixture_installed_available_with_config_descriptor() -> AddableRouteDescriptor {
    AddableRouteDescriptor::new(
        AddableRouteId::new("fixture-installed-binary").expect("fixture route id is valid"),
        fixture_driver("swallowtail-adapter-fixture-installed"),
        RouteTopology::Installed,
        AddableRouteAvailability::Available,
    )
    .with_config_fields([
        ConfigFieldDescriptor::new(
            ConfigFieldId::new("binary").expect("fixture config id is valid"),
            FieldLabel::new("Binary").expect("fixture label is valid"),
            ConfigFieldKind::BinaryPath,
        ),
        ConfigFieldDescriptor::new(
            ConfigFieldId::new("endpoint").expect("fixture config id is valid"),
            FieldLabel::new("Endpoint").expect("fixture label is valid"),
            ConfigFieldKind::ApiEndpoint,
        ),
        ConfigFieldDescriptor::new(
            ConfigFieldId::new("environment").expect("fixture config id is valid"),
            FieldLabel::new("Environment").expect("fixture label is valid"),
            ConfigFieldKind::Environment,
        ),
    ])
}
