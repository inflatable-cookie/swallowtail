use super::SIDECAR_DRIVER_ID;
use crate::sidecar::{
    pi_sdk_sidecar_node_claim, pi_sdk_sidecar_package_claim, pi_sdk_sidecar_sidecar_claim,
    pi_sdk_sidecar_wire_claim,
};
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, DriverDescriptor, DriverRole, ExecutionLayer,
    HostServiceKind, IntegrationFamilyId, OperationShape, TransportFamilyId,
};

/// Describes the Pi SDK sidecar catalogue and fresh interactive-session roles.
#[must_use]
pub fn pi_sdk_sidecar_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(SIDECAR_DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("pi").expect("static family id is valid"),
        TransportFamilyId::new(crate::sidecar::PI_SDK_SIDECAR_WIRE)
            .expect("static transport id is valid"),
    )
    .with_roles([DriverRole::ModelCatalog, DriverRole::InteractiveSession])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([OperationShape::InteractiveSession])
    .with_required_host_services(
        DriverRole::ModelCatalog,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::Credential,
            HostServiceKind::Time,
        ],
    )
    .with_required_host_services(
        DriverRole::InteractiveSession,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::Credential,
            HostServiceKind::WorkingResource,
            HostServiceKind::Time,
        ],
    )
    .with_interface_compatibility(pi_sdk_sidecar_package_claim())
    .with_interface_compatibility(pi_sdk_sidecar_node_claim())
    .with_interface_compatibility(pi_sdk_sidecar_wire_claim())
    .with_interface_compatibility(pi_sdk_sidecar_sidecar_claim())
}
