#[cfg(test)]
mod tests {
    use super::{DRIVER_ID, opencode_http_descriptor};
    use crate::selection::{
        OPENCODE_BASELINE_VERSION, OPENCODE_LATEST_QUALIFIED_VERSION, opencode_server_binding,
    };
    use swallowtail_core::{
        DriverRole, ExecutionLayer, HostServiceKind, InterfaceCompatibilityAssessment,
        OperationShape,
    };

    #[test]
    fn descriptor_claims_only_attached_harness_roles() {
        let descriptor = opencode_http_descriptor();
        assert_eq!(descriptor.identity().id().as_str(), DRIVER_ID);
        assert!(descriptor.supports_role(DriverRole::ModelCatalog));
        assert!(descriptor.supports_role(DriverRole::InteractiveSession));
        assert!(!descriptor.supports_role(DriverRole::StructuredRun));
        assert!(descriptor.supports_execution_layer(ExecutionLayer::HarnessInteraction));
        assert!(!descriptor.supports_execution_layer(ExecutionLayer::DirectModelInference));
        assert!(descriptor.supports_operation_shape(OperationShape::InteractiveSession));
        assert!(
            descriptor
                .required_host_services(DriverRole::InteractiveSession)
                .any(|service| service == HostServiceKind::BlockingWork)
        );
        for version in [OPENCODE_BASELINE_VERSION, OPENCODE_LATEST_QUALIFIED_VERSION] {
            assert!(
                descriptor.supports_interface_version(
                    &opencode_server_binding(version).expect("version is safe")
                )
            );
        }
        assert!(!descriptor.supports_interface_version(
            &opencode_server_binding("1.15.8").expect("gap version is safe")
        ));
        let newer = opencode_server_binding("1.18.5").expect("newer version is safe");
        assert!(!descriptor.supports_interface_version(&newer));
        assert!(descriptor.permits_interface_version(&newer));
        assert!(matches!(
            descriptor.assess_interface_version(&newer),
            InterfaceCompatibilityAssessment::UnverifiedNewer(_)
        ));
    }
}
