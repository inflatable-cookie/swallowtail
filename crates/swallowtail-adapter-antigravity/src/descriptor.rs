use swallowtail_core::{
    AdapterId, AdapterIdentity, AdapterVersion, DriverDescriptor, DriverRole, ExecutionLayer,
    HostServiceKind, IntegrationFamilyId, OperationShape, TransportFamilyId,
};

#[must_use]
/// Returns the installed discovery and model-catalogue descriptor.
pub fn antigravity_catalogue_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(crate::CATALOGUE_DRIVER_ID)
                .expect("static Antigravity adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("antigravity").expect("static Antigravity family id is valid"),
        TransportFamilyId::new("antigravity-cli-models-stdio")
            .expect("static Antigravity catalogue transport id is valid"),
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
    .with_interface_compatibility(crate::antigravity_catalogue_claim())
}

#[must_use]
/// Returns the separate headless run and continuation descriptor.
pub fn antigravity_headless_descriptor() -> DriverDescriptor {
    DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new(crate::HEADLESS_DRIVER_ID)
                .expect("static Antigravity headless adapter id is valid"),
            AdapterVersion::new(env!("CARGO_PKG_VERSION"))
                .expect("package version is a valid adapter version"),
        ),
        IntegrationFamilyId::new("antigravity").expect("static Antigravity family id is valid"),
        TransportFamilyId::new("antigravity-stream-json-stdio")
            .expect("static Antigravity headless transport id is valid"),
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
    .with_required_host_services(
        DriverRole::InteractiveSession,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::Time,
            HostServiceKind::WorkingResource,
        ],
    )
    .with_discovery_actions([swallowtail_core::DiscoveryAction::Probe])
    .with_interface_compatibility(crate::antigravity_headless_claim())
}

#[cfg(test)]
mod tests {
    use super::{antigravity_catalogue_descriptor, antigravity_headless_descriptor};
    use swallowtail_core::{DriverRole, ExecutionLayer, HostServiceKind, OperationShape};

    #[test]
    fn catalogue_identity_does_not_inherit_gemini() {
        let descriptor = antigravity_catalogue_descriptor();
        assert_eq!(descriptor.integration_family().as_str(), "antigravity");
        assert_ne!(descriptor.integration_family().as_str(), "gemini");
        assert_eq!(
            descriptor.transport_family().as_str(),
            "antigravity-cli-models-stdio"
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
    fn headless_descriptor_is_a_separate_structured_transport() {
        let descriptor = antigravity_headless_descriptor();
        assert_eq!(descriptor.integration_family().as_str(), "antigravity");
        assert_eq!(
            descriptor.transport_family().as_str(),
            "antigravity-stream-json-stdio"
        );
        assert!(descriptor.supports_role(DriverRole::Discovery));
        assert!(descriptor.supports_role(DriverRole::StructuredRun));
        assert!(descriptor.supports_role(DriverRole::InteractiveSession));
        assert!(descriptor.supports_operation_shape(OperationShape::InteractiveSession));
    }
}
