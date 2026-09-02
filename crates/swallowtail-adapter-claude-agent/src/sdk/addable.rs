//! Adapter-local Contract 057 addable-route descriptor for the Claude Agent
//! SDK sidecar route.
//!
//! The consuming application owns the exact Node runtime, the source-tagged
//! sidecar entry point, the exact SDK package, and the platform package that
//! carries the native binary, and admits them only as opaque host-owned
//! references. The descriptor never carries Node paths, binary paths,
//! environment bodies, or credential bytes. The route advertises no sign-in
//! action: the user runs the official Claude login out of band, the
//! credential stays in the official store, and Swallowtail leases a delegated
//! reference that exposes no secret.

use swallowtail_core::{
    AddableRouteAvailability, AddableRouteDescriptor, AddableRouteId,
    AddableRouteMissingRequirement, ConfigFieldDescriptor, ConfigFieldId, ConfigFieldKind,
    CredentialFieldDescriptor, CredentialFieldId, CredentialFieldVisibility, FieldLabel,
    RouteTopology,
};
use swallowtail_runtime::HostServices;

/// Addable-route id for the Claude Agent SDK sidecar route.
pub const CLAUDE_AGENT_SDK_ADDABLE_ROUTE_ID: &str = "claude-agent.sdk";
/// Config-field id for the opaque host-owned interpreted-script launch recipe
/// (approved Node runtime plus the sidecar entry point).
pub const CLAUDE_AGENT_SDK_LAUNCH_RECIPE_FIELD_ID: &str = "launch_recipe";
/// Config-field id for the opaque host-owned environment carrying the
/// provisioned SDK module, native binary, and shipped manifest.
pub const CLAUDE_AGENT_SDK_ENVIRONMENT_FIELD_ID: &str = "environment";
/// Credential-field id for the delegated subscription credential.
pub const CLAUDE_AGENT_SDK_CREDENTIAL_FIELD_ID: &str = "delegated_subscription";

/// Returns the installed addable-route descriptor for the Claude Agent SDK
/// sidecar.
///
/// The route is `Available` when the host exposes the Process and Credential
/// services that admission and preparation start the sidecar and lease the
/// delegated credential through. Without either the row is
/// `Unavailable(HostService)`. Topology is installed; the row does not probe.
#[must_use]
pub fn claude_agent_sdk_addable_route_descriptor(
    services: &HostServices,
) -> AddableRouteDescriptor {
    let availability = if services.process().is_some() && services.credential().is_some() {
        AddableRouteAvailability::Available
    } else {
        AddableRouteAvailability::Unavailable(AddableRouteMissingRequirement::HostService)
    };
    AddableRouteDescriptor::new(
        AddableRouteId::new(CLAUDE_AGENT_SDK_ADDABLE_ROUTE_ID)
            .expect("static addable route id is valid"),
        super::claude_agent_sdk_descriptor().identity().clone(),
        RouteTopology::Installed,
        availability,
    )
    .with_config_fields([
        ConfigFieldDescriptor::new(
            ConfigFieldId::new(CLAUDE_AGENT_SDK_LAUNCH_RECIPE_FIELD_ID)
                .expect("static config field id is valid"),
            FieldLabel::new("Sidecar launch recipe").expect("static field label is valid"),
            ConfigFieldKind::BinaryPath,
        ),
        ConfigFieldDescriptor::new(
            ConfigFieldId::new(CLAUDE_AGENT_SDK_ENVIRONMENT_FIELD_ID)
                .expect("static config field id is valid"),
            FieldLabel::new("Sidecar environment").expect("static field label is valid"),
            ConfigFieldKind::Environment,
        ),
    ])
    .with_credential_fields([CredentialFieldDescriptor::new(
        CredentialFieldId::new(CLAUDE_AGENT_SDK_CREDENTIAL_FIELD_ID)
            .expect("static credential field id is valid"),
        FieldLabel::new("Delegated Claude subscription").expect("static field label is valid"),
        CredentialFieldVisibility::Secret,
    )])
}
