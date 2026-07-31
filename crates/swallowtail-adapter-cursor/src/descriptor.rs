use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, DriverDescriptor, DriverRole, ExecutionLayer,
    HostServiceKind, IntegrationFamilyId, OperationShape, TransportFamilyId,
};

#[must_use]
pub fn cursor_catalogue_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(crate::CATALOGUE_DRIVER_ID).expect("static Cursor adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("cursor").expect("static Cursor family id is valid"),
        TransportFamilyId::new("cursor-cli-models-stdio")
            .expect("static Cursor catalogue transport id is valid"),
    )
    .with_roles([DriverRole::Discovery, DriverRole::ModelCatalog])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([OperationShape::StructuredRun])
    .with_required_host_services(
        DriverRole::Discovery,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::Time,
        ],
    )
    .with_required_host_services(
        DriverRole::ModelCatalog,
        [HostServiceKind::Process, HostServiceKind::Time],
    )
    .with_discovery_actions([swallowtail_core::DiscoveryAction::Probe])
    .with_interface_compatibility(crate::cursor_catalogue_claim())
}

#[must_use]
pub fn cursor_acp_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(crate::ACP_DRIVER_ID).expect("static Cursor ACP adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("cursor").expect("static Cursor family id is valid"),
        TransportFamilyId::new("acp-v1-stdio").expect("static ACP transport id is valid"),
    )
    .with_roles([DriverRole::Discovery, DriverRole::InteractiveSession])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([OperationShape::InteractiveSession])
    .with_required_host_services(
        DriverRole::Discovery,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::Time,
        ],
    )
    .with_required_host_services(
        DriverRole::InteractiveSession,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::WorkingResource,
            HostServiceKind::WorkingResourceIo,
        ],
    )
    .with_discovery_actions([swallowtail_core::DiscoveryAction::Probe])
    .with_interface_compatibility(crate::cursor_acp_claim())
}

#[must_use]
pub fn cursor_headless_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(crate::HEADLESS_DRIVER_ID)
                .expect("static Cursor headless adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("cursor").expect("static Cursor family id is valid"),
        TransportFamilyId::new("cursor-stream-json-stdio")
            .expect("static Cursor headless transport id is valid"),
    )
    .with_roles([DriverRole::Discovery, DriverRole::StructuredRun])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([OperationShape::StructuredRun])
    .with_required_host_services(
        DriverRole::Discovery,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::Time,
        ],
    )
    .with_required_host_services(
        DriverRole::StructuredRun,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::Time,
        ],
    )
    .with_discovery_actions([swallowtail_core::DiscoveryAction::Probe])
    .with_interface_compatibility(crate::cursor_headless_claim())
}

#[cfg(test)]
mod tests {
    use super::{cursor_acp_descriptor, cursor_catalogue_descriptor, cursor_headless_descriptor};
    use swallowtail_core::{DriverRole, ExecutionLayer, HostServiceKind, OperationShape};

    #[test]
    fn catalogue_descriptor_keeps_cursor_identity_and_roles_explicit() {
        let descriptor = cursor_catalogue_descriptor();
        assert_eq!(descriptor.integration_family().as_str(), "cursor");
        assert_eq!(
            descriptor.transport_family().as_str(),
            "cursor-cli-models-stdio"
        );
        assert!(descriptor.supports_role(DriverRole::Discovery));
        assert!(descriptor.supports_role(DriverRole::ModelCatalog));
        assert!(descriptor.supports_execution_layer(ExecutionLayer::HarnessInteraction));
        assert!(descriptor.supports_operation_shape(OperationShape::StructuredRun));
        assert_eq!(
            descriptor
                .required_host_services(DriverRole::ModelCatalog)
                .collect::<Vec<_>>(),
            [HostServiceKind::Time, HostServiceKind::Process]
        );
    }

    #[test]
    fn acp_descriptor_is_interactive_only() {
        let descriptor = cursor_acp_descriptor();
        assert_eq!(descriptor.integration_family().as_str(), "cursor");
        assert_eq!(descriptor.transport_family().as_str(), "acp-v1-stdio");
        assert!(descriptor.supports_role(DriverRole::Discovery));
        assert!(descriptor.supports_role(DriverRole::InteractiveSession));
        assert!(!descriptor.supports_role(DriverRole::StructuredRun));
        assert!(descriptor.supports_operation_shape(OperationShape::InteractiveSession));
        assert!(!descriptor.supports_operation_shape(OperationShape::StructuredRun));
    }

    #[test]
    fn headless_descriptor_is_structured_only() {
        let descriptor = cursor_headless_descriptor();
        assert_eq!(
            descriptor.transport_family().as_str(),
            "cursor-stream-json-stdio"
        );
        assert!(descriptor.supports_role(DriverRole::StructuredRun));
        assert!(!descriptor.supports_role(DriverRole::InteractiveSession));
        assert!(descriptor.supports_operation_shape(OperationShape::StructuredRun));
        assert!(!descriptor.supports_operation_shape(OperationShape::InteractiveSession));
    }
}
