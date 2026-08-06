const DRIVER_ID: &str = "swallowtail.kimi.acp";

#[must_use]
/// Describes the installed Kimi Code ACP route.
pub fn kimi_acp_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(DRIVER_ID).expect("static adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("kimi-code").expect("static family id is valid"),
        TransportFamilyId::new("acp-v1-stdio").expect("static transport id is valid"),
    )
    .with_roles([
        DriverRole::Discovery,
        DriverRole::InteractiveSession,
        DriverRole::ProviderSessionCatalogue,
        DriverRole::ProviderSessionImport,
    ])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([
        OperationShape::InteractiveSession,
        OperationShape::ProviderSessionCatalogue,
        OperationShape::ProviderSessionImport,
    ])
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
    .with_required_host_services(
        DriverRole::Discovery,
        [
            HostServiceKind::Task,
            HostServiceKind::Time,
            HostServiceKind::Process,
        ],
    )
    .with_required_host_services(
        DriverRole::ProviderSessionCatalogue,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::Credential,
            HostServiceKind::WorkingResource,
        ],
    )
    .with_required_host_services(
        DriverRole::ProviderSessionImport,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::Credential,
            HostServiceKind::WorkingResource,
        ],
    )
    .with_discovery_actions([swallowtail_core::DiscoveryAction::Probe])
    .with_interface_compatibility(crate::kimi_acp_claim())
}
