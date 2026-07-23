use crate::{DRIVER_ID, PINNED_PI_VERSION};
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, DriverDescriptor, DriverRole, ExecutionLayer,
    HostServiceKind, IntegrationFamilyId, InterfaceBehaviorRevision, InterfaceCompatibilityClaim,
    InterfaceCompatibilityClaimId, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionScheme, InterfaceVersionSegment, OperationShape, TransportFamilyId,
};

#[must_use]
pub fn pi_rpc_descriptor() -> DriverDescriptor {
    let version = InterfaceVersion::new(PINNED_PI_VERSION).expect("static Pi version is valid");
    let claim = InterfaceCompatibilityClaim::new(
        InterfaceCompatibilityClaimId::new("pi.rpc.package-window-1")
            .expect("static claim id is valid"),
        InterfaceVersionAxis::new("pi.package").expect("static version axis is valid"),
        InterfaceVersionScheme::Semantic,
        [InterfaceVersionSegment::exact(
            version,
            InterfaceBehaviorRevision::new("pi.rpc.strict-lf-v1")
                .expect("static behavior revision is valid"),
            InterfaceSupportStatus::Maintained,
        )],
        [],
    )
    .expect("static Pi compatibility claim is valid");
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("pi").expect("static family id is valid"),
        TransportFamilyId::new("strict-lf-jsonl-stdio").expect("static transport id is valid"),
    )
    .with_roles([DriverRole::InteractiveSession])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([OperationShape::InteractiveSession])
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
    .with_interface_compatibility(claim)
}
