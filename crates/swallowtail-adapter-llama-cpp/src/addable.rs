//! Adapter-local Contract 057 addable-route descriptor for llama.cpp attach.
//!
//! This is the local-runtime route. Consumers assemble an addable-route
//! catalog from this descriptor the same way they assemble prepared facades.
//! The attached runtime is externally managed: the route advertises no
//! credential field and no sign-in action, and Swallowtail never starts or
//! stops the operator-owned server. Do not advertise `llama-cpp.owned` from
//! this row.

use swallowtail_core::{
    AddableRouteAvailability, AddableRouteDescriptor, AddableRouteId,
    AddableRouteMissingRequirement, ConfigFieldDescriptor, ConfigFieldId, ConfigFieldKind,
    FieldLabel, RouteTopology,
};
use swallowtail_runtime::HostServices;

/// Addable-route id for the local llama.cpp attach route.
pub const LLAMA_CPP_ATTACHED_ADDABLE_ROUTE_ID: &str = "llama-cpp.attached";
/// Config-field id for the opaque host-owned API endpoint.
pub const LLAMA_CPP_ATTACHED_ENDPOINT_FIELD_ID: &str = "endpoint";

/// Returns the local-runtime addable-route descriptor for llama.cpp attach.
///
/// The route is `Available` when the host exposes the Network service that
/// admission and preparation reach the attached runtime through. Without it
/// the row is `Unavailable(HostService)`. Absence of the descriptor still
/// means this crate is unlinked. Topology is local-runtime; it is not
/// [`swallowtail_core::ExecutionLayer`]. Runtime reachability stays
/// `prepare_llama_cpp_attached`; the addable row does not probe `/health`.
#[must_use]
pub fn llama_cpp_attached_addable_route_descriptor(
    services: &HostServices,
) -> AddableRouteDescriptor {
    let availability = if services.network().is_some() {
        AddableRouteAvailability::Available
    } else {
        AddableRouteAvailability::Unavailable(AddableRouteMissingRequirement::HostService)
    };
    AddableRouteDescriptor::new(
        AddableRouteId::new(LLAMA_CPP_ATTACHED_ADDABLE_ROUTE_ID)
            .expect("static addable route id is valid"),
        crate::llama_cpp_attached_descriptor().identity().clone(),
        RouteTopology::LocalRuntime,
        availability,
    )
    .with_config_fields([ConfigFieldDescriptor::new(
        ConfigFieldId::new(LLAMA_CPP_ATTACHED_ENDPOINT_FIELD_ID)
            .expect("static config field id is valid"),
        FieldLabel::new("API endpoint").expect("static field label is valid"),
        ConfigFieldKind::ApiEndpoint,
    )])
}
