use crate::DRIVER_ID;
use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, DriverDescriptor, DriverRole, ExecutionLayer,
    HostServiceKind, IntegrationFamilyId, OperationShape, TransportFamilyId,
};

/// Describes Pi discovery, catalogue, run, and session roles.
#[must_use]
pub fn pi_rpc_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("pi").expect("static family id is valid"),
        TransportFamilyId::new("strict-lf-jsonl-stdio").expect("static transport id is valid"),
    )
    .with_roles([
        DriverRole::Discovery,
        DriverRole::ModelCatalog,
        DriverRole::InteractiveSession,
        DriverRole::StructuredRun,
    ])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([
        OperationShape::InteractiveSession,
        OperationShape::StructuredRun,
    ])
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
    .with_required_host_services(
        DriverRole::StructuredRun,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::Credential,
            HostServiceKind::WorkingResource,
            HostServiceKind::Time,
        ],
    )
    .with_required_host_services(
        DriverRole::Discovery,
        [
            HostServiceKind::Task,
            HostServiceKind::Time,
            HostServiceKind::Process,
        ],
    )
    .with_discovery_actions([swallowtail_core::DiscoveryAction::Probe])
    .with_interface_compatibility(crate::pi_rpc_claim())
}
