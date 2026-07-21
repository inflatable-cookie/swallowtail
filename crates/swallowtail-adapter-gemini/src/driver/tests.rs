mod tests {
    use super::{
        DRIVER_ID, gemini_acp_descriptor, gemini_process_request, parse_new_session,
        validate_initialize,
    };
    use serde_json::json;
    use swallowtail_core::{DriverRole, ExecutionLayer, HostServiceKind, OperationShape};
    use swallowtail_runtime::{EnvironmentRef, ExecutableRef, WorkingResourceRef};

    #[test]
    fn descriptor_claims_only_the_pinned_interactive_acp_shape() {
        let descriptor = gemini_acp_descriptor();
        assert_eq!(descriptor.identity().id().as_str(), DRIVER_ID);
        assert!(descriptor.supports_role(DriverRole::InteractiveSession));
        assert!(!descriptor.supports_role(DriverRole::ModelCatalog));
        assert!(!descriptor.supports_role(DriverRole::StructuredRun));
        assert!(descriptor.supports_execution_layer(ExecutionLayer::HarnessInteraction));
        assert!(!descriptor.supports_execution_layer(ExecutionLayer::DirectModelInference));
        assert!(descriptor.supports_operation_shape(OperationShape::InteractiveSession));
        let required = descriptor
            .required_host_services(DriverRole::InteractiveSession)
            .collect::<Vec<_>>();
        assert!(required.contains(&HostServiceKind::WorkingResourceIo));
        assert!(!required.contains(&HostServiceKind::Credential));
        assert!(!required.contains(&HostServiceKind::Network));
    }

    #[test]
    fn process_mapping_uses_only_isolated_environment_and_read_only_mode() {
        let environment = EnvironmentRef::new("gemini.isolated").expect("valid environment");
        let resource = WorkingResourceRef::new("workspace.main").expect("valid resource");
        let request = gemini_process_request(
            ExecutableRef::new("gemini.pinned").expect("valid executable"),
            environment.clone(),
            resource.clone(),
        );
        assert_eq!(
            request.arguments().collect::<Vec<_>>(),
            ["--acp", "--approval-mode", "plan"]
        );
        assert_eq!(request.environment().collect::<Vec<_>>(), [&environment]);
        assert_eq!(request.working_resource(), Some(&resource));
    }

    #[test]
    fn initialize_and_new_session_fail_closed_on_version_access_or_mode_drift() {
        let initialize = json!({
            "protocolVersion": 1,
            "agentInfo": {"name": "gemini-cli", "version": "0.51.0"},
            "authMethods": [{"id": "gemini-api-key", "name": "Gemini API key"}]
        });
        validate_initialize(&initialize).expect("pinned initialization is accepted");
        assert!(
            validate_initialize(&json!({
                "agentInfo": {"name": "gemini-cli", "version": "0.52.0"},
                "authMethods": [{"id": "gemini-api-key"}]
            }))
            .is_err()
        );
        assert_eq!(
            parse_new_session(&json!({
                "sessionId": "fixture-session",
                "modes": {"currentModeId": "plan"}
            }))
            .expect("plan session is accepted"),
            "fixture-session"
        );
        assert!(
            parse_new_session(&json!({
                "sessionId": "fixture-session",
                "modes": {"currentModeId": "yolo"}
            }))
            .is_err()
        );
    }
}
