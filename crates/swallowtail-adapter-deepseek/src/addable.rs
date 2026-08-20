//! Adapter-local Contract 057 addable-route descriptor for DeepSeek continuation.
//!
//! This is the hosted public API-key route. Consumers assemble an
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

/// Addable-route id for the hosted DeepSeek continuation route.
pub const DEEPSEEK_CONTINUATION_ADDABLE_ROUTE_ID: &str = "deepseek.continuation";
/// Credential-field id for the secret DeepSeek API key.
pub const DEEPSEEK_CONTINUATION_API_KEY_FIELD_ID: &str = "api_key";
/// Config-field id for the opaque host-owned API endpoint.
pub const DEEPSEEK_CONTINUATION_ENDPOINT_FIELD_ID: &str = "endpoint";

/// Returns the hosted addable-route descriptor for DeepSeek continuation.
///
/// The route is `Available` when the host exposes the Credential service that
/// admission and preparation lease API-key secrets through. Without it the
/// row is `Unavailable(HostService)`. Absence of the descriptor still means
/// this crate is unlinked. Topology is hosted; it is not
/// [`swallowtail_core::ExecutionLayer`]. The credential field has no
/// environment name.
#[must_use]
pub fn deepseek_continuation_addable_route_descriptor(
    services: &HostServices,
) -> AddableRouteDescriptor {
    let availability = if services.credential().is_some() {
        AddableRouteAvailability::Available
    } else {
        AddableRouteAvailability::Unavailable(AddableRouteMissingRequirement::HostService)
    };
    AddableRouteDescriptor::new(
        AddableRouteId::new(DEEPSEEK_CONTINUATION_ADDABLE_ROUTE_ID)
            .expect("static addable route id is valid"),
        crate::deepseek_direct_descriptor().identity().clone(),
        RouteTopology::Hosted,
        availability,
    )
    .with_credential_fields([CredentialFieldDescriptor::new(
        CredentialFieldId::new(DEEPSEEK_CONTINUATION_API_KEY_FIELD_ID)
            .expect("static credential field id is valid"),
        FieldLabel::new("DeepSeek API key").expect("static field label is valid"),
        CredentialFieldVisibility::Secret,
    )])
    .with_config_fields([ConfigFieldDescriptor::new(
        ConfigFieldId::new(DEEPSEEK_CONTINUATION_ENDPOINT_FIELD_ID)
            .expect("static config field id is valid"),
        FieldLabel::new("API endpoint").expect("static field label is valid"),
        ConfigFieldKind::ApiEndpoint,
    )])
}
