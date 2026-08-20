//! Adapter-local Contract 057 addable-route descriptor for Codex app-server.
//!
//! This is the installed ChatGPT subscription route. Consumers assemble an
//! addable-route catalog from this descriptor the same way they assemble
//! prepared facades. ChatGPT access is a cached local login, so the route
//! advertises no credential field and no sign-in action; Swallowtail never
//! extracts ChatGPT tokens.

use swallowtail_core::{
    AddableRouteAvailability, AddableRouteDescriptor, AddableRouteId,
    AddableRouteMissingRequirement, ConfigFieldDescriptor, ConfigFieldId, ConfigFieldKind,
    FieldLabel, RouteTopology,
};
use swallowtail_runtime::HostServices;

/// Addable-route id for the installed Codex app-server route.
pub const CODEX_APP_SERVER_ADDABLE_ROUTE_ID: &str = "codex.app-server";
/// Config-field id for the opaque host-owned binary path.
pub const CODEX_APP_SERVER_BINARY_PATH_FIELD_ID: &str = "binary_path";
/// Config-field id for the opaque host-owned environment.
pub const CODEX_APP_SERVER_ENVIRONMENT_FIELD_ID: &str = "environment";

/// Returns the installed addable-route descriptor for Codex app-server.
///
/// The route is `Available` when the host exposes the Process service that
/// admission, discovery, and preparation spawn the executable through.
/// Without it the row is `Unavailable(HostService)`. Absence of the
/// descriptor still means this crate is unlinked. Topology is installed; it
/// is not [`swallowtail_core::ExecutionLayer`]. Discovery of the executable
/// stays Contract 008 on the selected driver; the addable row does not probe.
#[must_use]
pub fn codex_app_server_addable_route_descriptor(
    services: &HostServices,
) -> AddableRouteDescriptor {
    let availability = if services.process().is_some() {
        AddableRouteAvailability::Available
    } else {
        AddableRouteAvailability::Unavailable(AddableRouteMissingRequirement::HostService)
    };
    AddableRouteDescriptor::new(
        AddableRouteId::new(CODEX_APP_SERVER_ADDABLE_ROUTE_ID)
            .expect("static addable route id is valid"),
        crate::codex_app_server_descriptor().identity().clone(),
        RouteTopology::Installed,
        availability,
    )
    .with_config_fields([
        ConfigFieldDescriptor::new(
            ConfigFieldId::new(CODEX_APP_SERVER_BINARY_PATH_FIELD_ID)
                .expect("static config field id is valid"),
            FieldLabel::new("Binary path").expect("static field label is valid"),
            ConfigFieldKind::BinaryPath,
        ),
        ConfigFieldDescriptor::new(
            ConfigFieldId::new(CODEX_APP_SERVER_ENVIRONMENT_FIELD_ID)
                .expect("static config field id is valid"),
            FieldLabel::new("Environment").expect("static field label is valid"),
            ConfigFieldKind::Environment,
        ),
    ])
}
