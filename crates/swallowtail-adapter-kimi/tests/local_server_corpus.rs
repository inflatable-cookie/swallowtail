use swallowtail_adapter_kimi::{
    KIMI_CODE_AXIS, kimi_acp_descriptor, kimi_code_binding, kimi_local_server_claim,
    kimi_local_server_descriptor,
};
use swallowtail_core::{
    DriverRole, ExecutionLayer, InterfaceCompatibilityAssessment, OperationShape,
};

#[test]
fn local_server_and_acp_descriptors_cannot_substitute_for_each_other() {
    let acp = kimi_acp_descriptor();
    let local = kimi_local_server_descriptor();

    assert_ne!(acp.identity().id(), local.identity().id());
    assert_ne!(acp.transport_family(), local.transport_family());
    assert_eq!(acp.integration_family(), local.integration_family());
    assert_eq!(
        local.transport_family().as_str(),
        "kimi-local-server-rest-ws-v2"
    );
    assert!(local.supports_execution_layer(ExecutionLayer::HarnessInteraction));
    assert!(local.supports_operation_shape(OperationShape::ProviderSessionManagement));
    assert!(local.supports_role(DriverRole::ProviderSessionManagement));
    assert!(local.supports_role(DriverRole::InteractiveSession));
    assert!(acp.supports_role(DriverRole::InteractiveSession));
    assert!(!acp.supports_role(DriverRole::ProviderSessionManagement));

    for role in [
        DriverRole::Discovery,
        DriverRole::ModelCatalog,
        DriverRole::StructuredRun,
        DriverRole::RealtimeMediaSession,
        DriverRole::ServingInstanceLifecycle,
    ] {
        assert!(!local.supports_role(role));
    }
}

#[test]
fn local_server_claim_is_separate_and_forward_permissive() {
    let claim = kimi_local_server_claim();
    assert_eq!(claim.axis().as_str(), KIMI_CODE_AXIS);
    assert_ne!(
        claim.id().as_str(),
        kimi_acp_descriptor()
            .interface_compatibility(claim.axis())
            .expect("ACP claim exists")
            .id()
            .as_str()
    );

    for exact in ["0.28.1", "0.29.0"] {
        let binding = kimi_code_binding(exact).expect("exact version binds");
        assert!(matches!(
            claim.assess(binding.version()),
            InterfaceCompatibilityAssessment::Qualified(_)
        ));
    }
    let newer = kimi_code_binding("0.30.0").expect("newer version binds");
    assert!(matches!(
        claim.assess(newer.version()),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
}
