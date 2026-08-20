//! Adapter-local Contract 057 addable-route descriptor for Claude Agent ACP.
//!
//! This is the installed local-subscription route. Consumers assemble an
//! addable-route catalog from this descriptor the same way they assemble
//! prepared facades. Local Claude subscription is inherited process login
//! state, so the route advertises no credential field and no sign-in action;
//! Swallowtail never extracts keychain bytes. Do not advertise
//! `claude-code.headless` or `claude-code.response-only` from this row.

use swallowtail_core::{
    AddableRouteAvailability, AddableRouteDescriptor, AddableRouteId,
    AddableRouteMissingRequirement, ConfigFieldDescriptor, ConfigFieldId, ConfigFieldKind,
    FieldLabel, RouteTopology,
};
use swallowtail_runtime::HostServices;

/// Addable-route id for the installed Claude Agent ACP route.
pub const CLAUDE_AGENT_ACP_ADDABLE_ROUTE_ID: &str = "claude-agent.acp";
/// Config-field id for the opaque host-owned binary path.
pub const CLAUDE_AGENT_ACP_BINARY_PATH_FIELD_ID: &str = "binary_path";
/// Config-field id for the opaque host-owned environment.
pub const CLAUDE_AGENT_ACP_ENVIRONMENT_FIELD_ID: &str = "environment";

/// Returns the installed addable-route descriptor for Claude Agent ACP.
///
/// The route is `Available` when the host exposes the Process service that
/// admission, discovery, and preparation spawn the executable through.
/// Without it the row is `Unavailable(HostService)`. Absence of the
/// descriptor still means this crate is unlinked. Topology is installed; it
/// is not [`swallowtail_core::ExecutionLayer`]. Discovery of the executable
/// stays Contract 008 on the selected driver; the addable row does not probe.
#[must_use]
pub fn claude_agent_acp_addable_route_descriptor(
    services: &HostServices,
) -> AddableRouteDescriptor {
    let availability = if services.process().is_some() {
        AddableRouteAvailability::Available
    } else {
        AddableRouteAvailability::Unavailable(AddableRouteMissingRequirement::HostService)
    };
    AddableRouteDescriptor::new(
        AddableRouteId::new(CLAUDE_AGENT_ACP_ADDABLE_ROUTE_ID)
            .expect("static addable route id is valid"),
        crate::claude_agent_acp_descriptor().identity().clone(),
        RouteTopology::Installed,
        availability,
    )
    .with_config_fields([
        ConfigFieldDescriptor::new(
            ConfigFieldId::new(CLAUDE_AGENT_ACP_BINARY_PATH_FIELD_ID)
                .expect("static config field id is valid"),
            FieldLabel::new("Binary path").expect("static field label is valid"),
            ConfigFieldKind::BinaryPath,
        ),
        ConfigFieldDescriptor::new(
            ConfigFieldId::new(CLAUDE_AGENT_ACP_ENVIRONMENT_FIELD_ID)
                .expect("static config field id is valid"),
            FieldLabel::new("Environment").expect("static field label is valid"),
            ConfigFieldKind::Environment,
        ),
    ])
}
