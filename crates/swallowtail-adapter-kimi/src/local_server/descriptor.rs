use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, DriverDescriptor, DriverRole, ExecutionLayer,
    HostServiceKind, IntegrationFamilyId, OperationShape, TransportFamilyId,
};

const DRIVER_ID: &str = "swallowtail.kimi.local-server";
const TRANSPORT_ID: &str = "kimi-local-server-rest-ws-v2";

#[must_use]
pub fn kimi_local_server_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("kimi-code").expect("static family id is valid"),
        TransportFamilyId::new(TRANSPORT_ID).expect("static transport id is valid"),
    )
    .with_roles([
        DriverRole::InteractiveSession,
        DriverRole::ProviderSessionManagement,
    ])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([
        OperationShape::InteractiveSession,
        OperationShape::ProviderSessionManagement,
    ])
    .with_interface_compatibility(crate::kimi_local_server_claim())
    .with_required_host_services(
        DriverRole::InteractiveSession,
        [
            HostServiceKind::Task,
            HostServiceKind::BlockingWork,
            HostServiceKind::Time,
            HostServiceKind::Network,
            HostServiceKind::Credential,
            HostServiceKind::WorkingResource,
        ],
    )
    .with_required_host_services(
        DriverRole::ProviderSessionManagement,
        [
            HostServiceKind::Task,
            HostServiceKind::BlockingWork,
            HostServiceKind::Time,
            HostServiceKind::Network,
            HostServiceKind::Credential,
            HostServiceKind::WorkingResource,
        ],
    )
}
