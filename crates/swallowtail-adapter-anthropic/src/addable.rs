//! Adapter-local Contract 057 addable-route descriptor for Anthropic Messages.
//!
//! This is the hosted public API-key route. Consumers assemble an
//! addable-route catalog from this descriptor the same way they assemble
//! prepared facades. The descriptor never carries secret bytes; the
//! environment name is a name, not a resolved value.

use swallowtail_core::{
    AddableRouteAvailability, AddableRouteDescriptor, AddableRouteId,
    AddableRouteMissingRequirement, ConfigFieldDescriptor, ConfigFieldId, ConfigFieldKind,
    CredentialFieldDescriptor, CredentialFieldId, CredentialFieldVisibility,
    EnvironmentVariableName, FieldLabel, RouteTopology,
};
use swallowtail_runtime::HostServices;

/// Addable-route id for the hosted Anthropic Messages route.
pub const ANTHROPIC_MESSAGES_ADDABLE_ROUTE_ID: &str = "anthropic.messages";
/// Credential-field id for the secret Anthropic API key.
pub const ANTHROPIC_MESSAGES_API_KEY_FIELD_ID: &str = "api_key";
/// Config-field id for the opaque host-owned API endpoint.
pub const ANTHROPIC_MESSAGES_ENDPOINT_FIELD_ID: &str = "endpoint";
/// Environment-variable name a host may use for the API key. Name only.
pub const ANTHROPIC_MESSAGES_API_KEY_ENVIRONMENT_NAME: &str = "ANTHROPIC_API_KEY";

/// Returns the hosted addable-route descriptor for Anthropic Messages.
///
/// The route is `Available` when the host exposes the Credential service that
/// admission and preparation lease API-key secrets through. Without it the
/// row is `Unavailable(HostService)`. Absence of the descriptor still means
/// this crate is unlinked. Topology is hosted; it is not
/// [`swallowtail_core::ExecutionLayer`].
#[must_use]
pub fn anthropic_messages_addable_route_descriptor(
    services: &HostServices,
) -> AddableRouteDescriptor {
    let availability = if services.credential().is_some() {
        AddableRouteAvailability::Available
    } else {
        AddableRouteAvailability::Unavailable(AddableRouteMissingRequirement::HostService)
    };
    AddableRouteDescriptor::new(
        AddableRouteId::new(ANTHROPIC_MESSAGES_ADDABLE_ROUTE_ID)
            .expect("static addable route id is valid"),
        crate::anthropic_direct_descriptor().identity().clone(),
        RouteTopology::Hosted,
        availability,
    )
    .with_credential_fields([CredentialFieldDescriptor::new(
        CredentialFieldId::new(ANTHROPIC_MESSAGES_API_KEY_FIELD_ID)
            .expect("static credential field id is valid"),
        FieldLabel::new("Anthropic API key").expect("static field label is valid"),
        CredentialFieldVisibility::Secret,
    )
    .with_environment_name(
        EnvironmentVariableName::new(ANTHROPIC_MESSAGES_API_KEY_ENVIRONMENT_NAME)
            .expect("static environment name is valid"),
    )])
    .with_config_fields([ConfigFieldDescriptor::new(
        ConfigFieldId::new(ANTHROPIC_MESSAGES_ENDPOINT_FIELD_ID)
            .expect("static config field id is valid"),
        FieldLabel::new("API endpoint").expect("static field label is valid"),
        ConfigFieldKind::ApiEndpoint,
    )])
}
