//! Adapter-local Contract 057 addable-route descriptor for Kimi Platform Chat.
//!
//! This is the hosted public Platform API-key route. Consumers assemble an
//! addable-route catalog from this descriptor the same way they assemble
//! prepared facades. The descriptor never carries secret bytes and does not
//! invent an environment-variable name.

use swallowtail_core::{
    AddableRouteAvailability, AddableRouteDescriptor, AddableRouteId,
    AddableRouteMissingRequirement, ConfigFieldDescriptor, ConfigFieldId, ConfigFieldKind,
    CredentialFieldDescriptor, CredentialFieldId, CredentialFieldVisibility, FieldLabel,
    RouteTopology,
};
use swallowtail_runtime::HostServices;

/// Addable-route id for the hosted Kimi Platform Chat route.
pub const KIMI_PLATFORM_CHAT_ADDABLE_ROUTE_ID: &str = "kimi-platform.chat";
/// Credential-field id for the secret Kimi Platform API key.
pub const KIMI_PLATFORM_CHAT_API_KEY_FIELD_ID: &str = "api_key";
/// Config-field id for the opaque host-owned API endpoint.
pub const KIMI_PLATFORM_CHAT_ENDPOINT_FIELD_ID: &str = "endpoint";

/// Returns the hosted addable-route descriptor for Kimi Platform Chat.
///
/// The route is `Available` when the host exposes the Credential service that
/// admission and preparation lease API-key secrets through. Without it the
/// row is `Unavailable(HostService)`. Absence of the descriptor still means
/// this crate is unlinked. Topology is hosted; it is not
/// [`swallowtail_core::ExecutionLayer`]. The credential field has no
/// environment name.
#[must_use]
pub fn kimi_platform_chat_addable_route_descriptor(
    services: &HostServices,
) -> AddableRouteDescriptor {
    let availability = if services.credential().is_some() {
        AddableRouteAvailability::Available
    } else {
        AddableRouteAvailability::Unavailable(AddableRouteMissingRequirement::HostService)
    };
    AddableRouteDescriptor::new(
        AddableRouteId::new(KIMI_PLATFORM_CHAT_ADDABLE_ROUTE_ID)
            .expect("static addable route id is valid"),
        crate::kimi_platform_direct_descriptor().identity().clone(),
        RouteTopology::Hosted,
        availability,
    )
    .with_credential_fields([CredentialFieldDescriptor::new(
        CredentialFieldId::new(KIMI_PLATFORM_CHAT_API_KEY_FIELD_ID)
            .expect("static credential field id is valid"),
        FieldLabel::new("Kimi Platform API key").expect("static field label is valid"),
        CredentialFieldVisibility::Secret,
    )])
    .with_config_fields([ConfigFieldDescriptor::new(
        ConfigFieldId::new(KIMI_PLATFORM_CHAT_ENDPOINT_FIELD_ID)
            .expect("static config field id is valid"),
        FieldLabel::new("API endpoint").expect("static field label is valid"),
        ConfigFieldKind::ApiEndpoint,
    )])
}
