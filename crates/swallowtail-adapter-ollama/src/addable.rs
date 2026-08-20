//! Adapter-local Contract 057 addable-route descriptor for Ollama attach.
//!
//! This is the local-runtime route. Consumers assemble an addable-route
//! catalog from this descriptor the same way they assemble prepared facades.
//! The attached runtime is externally managed: the route advertises no
//! credential field and no sign-in action, and Swallowtail never installs,
//! starts, or pulls Ollama.

use swallowtail_core::{
    AddableRouteAvailability, AddableRouteDescriptor, AddableRouteId,
    AddableRouteMissingRequirement, ConfigFieldDescriptor, ConfigFieldId, ConfigFieldKind,
    FieldLabel, RouteTopology,
};
use swallowtail_runtime::HostServices;

/// Addable-route id for the local Ollama attach route.
pub const OLLAMA_ATTACHED_ADDABLE_ROUTE_ID: &str = "ollama.attached";
/// Config-field id for the opaque host-owned API endpoint.
pub const OLLAMA_ATTACHED_ENDPOINT_FIELD_ID: &str = "endpoint";

/// Returns the local-runtime addable-route descriptor for Ollama attach.
///
/// The route is `Available` when the host exposes the Network service that
/// admission and preparation reach the attached runtime through. Without it
/// the row is `Unavailable(HostService)`. Absence of the descriptor still
/// means this crate is unlinked. Topology is local-runtime; it is not
/// [`swallowtail_core::ExecutionLayer`]. Runtime reachability stays
/// `prepare_ollama_attached`; the addable row does not probe the runtime.
#[must_use]
pub fn ollama_attached_addable_route_descriptor(services: &HostServices) -> AddableRouteDescriptor {
    let availability = if services.network().is_some() {
        AddableRouteAvailability::Available
    } else {
        AddableRouteAvailability::Unavailable(AddableRouteMissingRequirement::HostService)
    };
    AddableRouteDescriptor::new(
        AddableRouteId::new(OLLAMA_ATTACHED_ADDABLE_ROUTE_ID)
            .expect("static addable route id is valid"),
        crate::ollama_native_descriptor().identity().clone(),
        RouteTopology::LocalRuntime,
        availability,
    )
    .with_config_fields([ConfigFieldDescriptor::new(
        ConfigFieldId::new(OLLAMA_ATTACHED_ENDPOINT_FIELD_ID)
            .expect("static config field id is valid"),
        FieldLabel::new("API endpoint").expect("static field label is valid"),
        ConfigFieldKind::ApiEndpoint,
    )])
}
