#[cfg(test)]
mod tests {
    use super::{DRIVER_ID, opencode_http_descriptor};
    use swallowtail_core::{DriverRole, ExecutionLayer, HostServiceKind, OperationShape};

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
    }
}

