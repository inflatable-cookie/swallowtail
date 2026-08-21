//! Adapter-local Contract 057 addable-route descriptor for the Pi SDK
//! sidecar route.
//!
//! This is the installed host-provisioned route: the consuming application
//! owns the exact Node runtime, the source-tagged sidecar entry point, and
//! the exact SDK package, and admits them only as opaque host-owned
//! references. The descriptor never carries Node paths, session paths,
//! environment bodies, or credential bytes, and the route advertises no
//! sign-in action: the delegated harness credential is provisioned, never
//! collected. There is no discovery probe; admission evidence is the explicit
//! launch recipe plus the driver's bootstrap identity verification.

use swallowtail_core::{
    AddableRouteAvailability, AddableRouteDescriptor, AddableRouteId,
    AddableRouteMissingRequirement, ConfigFieldDescriptor, ConfigFieldId, ConfigFieldKind,
    CredentialFieldDescriptor, CredentialFieldId, CredentialFieldVisibility, FieldLabel,
    RouteTopology,
};
use swallowtail_runtime::HostServices;

/// Addable-route id for the Pi SDK sidecar route.
pub const PI_SDK_SIDECAR_ADDABLE_ROUTE_ID: &str = "pi.sdk-sidecar";
/// Config-field id for the opaque host-owned interpreted-script launch recipe
/// (approved Node runtime plus the sidecar entry point).
pub const PI_SDK_SIDECAR_LAUNCH_RECIPE_FIELD_ID: &str = "launch_recipe";
/// Config-field id for the opaque host-owned environment carrying the
/// provisioned SDK module, agent directory, and session directory.
pub const PI_SDK_SIDECAR_ENVIRONMENT_FIELD_ID: &str = "environment";
/// Credential-field id for the delegated harness credential.
pub const PI_SDK_SIDECAR_CREDENTIAL_FIELD_ID: &str = "harness_credential";

/// Returns the installed addable-route descriptor for the Pi SDK sidecar.
///
/// The route is `Available` when the host exposes the Process and Credential
/// services that admission and preparation spawn the sidecar and lease the
/// delegated credential through. Without either the row is
/// `Unavailable(HostService)`. Absence of the descriptor still means this
/// crate is unlinked. Topology is installed; the addable row does not probe.
#[must_use]
pub fn pi_sdk_sidecar_addable_route_descriptor(services: &HostServices) -> AddableRouteDescriptor {
    let availability = if services.process().is_some() && services.credential().is_some() {
        AddableRouteAvailability::Available
    } else {
        AddableRouteAvailability::Unavailable(AddableRouteMissingRequirement::HostService)
    };
    AddableRouteDescriptor::new(
        AddableRouteId::new(PI_SDK_SIDECAR_ADDABLE_ROUTE_ID)
            .expect("static addable route id is valid"),
        super::pi_sdk_sidecar_descriptor().identity().clone(),
        RouteTopology::Installed,
        availability,
    )
    .with_config_fields([
        ConfigFieldDescriptor::new(
            ConfigFieldId::new(PI_SDK_SIDECAR_LAUNCH_RECIPE_FIELD_ID)
                .expect("static config field id is valid"),
            FieldLabel::new("Sidecar launch recipe").expect("static field label is valid"),
            ConfigFieldKind::BinaryPath,
        ),
        ConfigFieldDescriptor::new(
            ConfigFieldId::new(PI_SDK_SIDECAR_ENVIRONMENT_FIELD_ID)
                .expect("static config field id is valid"),
            FieldLabel::new("Sidecar environment").expect("static field label is valid"),
            ConfigFieldKind::Environment,
        ),
    ])
    .with_credential_fields([CredentialFieldDescriptor::new(
        CredentialFieldId::new(PI_SDK_SIDECAR_CREDENTIAL_FIELD_ID)
            .expect("static credential field id is valid"),
        FieldLabel::new("Delegated harness credential").expect("static field label is valid"),
        CredentialFieldVisibility::Secret,
    )])
}
