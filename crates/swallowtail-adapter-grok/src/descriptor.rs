use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, DriverDescriptor, DriverRole, ExecutionLayer,
    HostServiceKind, IntegrationFamilyId, OperationShape, TransportFamilyId,
};

const DRIVER_ID: &str = "swallowtail.grok-build.acp";

#[must_use]
/// Describes the installed Grok Build ACP route.
pub fn grok_build_acp_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static Grok adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("grok-build").expect("static Grok family id is valid"),
        TransportFamilyId::new("acp-v1-stdio").expect("static ACP transport id is valid"),
    )
    .with_roles([
        DriverRole::Discovery,
        DriverRole::StructuredRun,
        DriverRole::InteractiveSession,
    ])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([
        OperationShape::StructuredRun,
        OperationShape::InteractiveSession,
    ])
    .with_required_host_services(
        DriverRole::Discovery,
        [
            HostServiceKind::Task,
            HostServiceKind::Time,
            HostServiceKind::Process,
        ],
    )
    .with_required_host_services(
        DriverRole::StructuredRun,
        [
            HostServiceKind::Task,
            HostServiceKind::Time,
            HostServiceKind::Process,
            HostServiceKind::Credential,
            HostServiceKind::WorkingResource,
            HostServiceKind::WorkingResourceIo,
        ],
    )
    .with_required_host_services(
        DriverRole::InteractiveSession,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::Credential,
            HostServiceKind::WorkingResource,
            HostServiceKind::WorkingResourceIo,
        ],
    )
    .with_discovery_actions([swallowtail_core::DiscoveryAction::Probe])
    .with_interface_compatibility(crate::grok_build_acp_claim())
}
