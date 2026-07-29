use swallowtail_adapter_kimi::{
    KIMI_CODE_AXIS, kimi_acp_descriptor, kimi_code_binding, kimi_headless_descriptor,
    kimi_local_server_claim, kimi_local_server_descriptor,
};
use swallowtail_core::{
    DriverRole, ExecutionLayer, InterfaceCompatibilityAssessment, OperationShape,
};

#[test]
fn local_server_and_acp_descriptors_cannot_substitute_for_each_other() {
    let acp = kimi_acp_descriptor();
    let headless = kimi_headless_descriptor();
    let local = kimi_local_server_descriptor();

    assert_ne!(acp.identity().id(), local.identity().id());
    assert_ne!(acp.identity().id(), headless.identity().id());
    assert_ne!(headless.identity().id(), local.identity().id());
    assert_ne!(acp.transport_family(), local.transport_family());
    assert_ne!(acp.transport_family(), headless.transport_family());
    assert_ne!(headless.transport_family(), local.transport_family());
    assert_eq!(acp.integration_family(), local.integration_family());
    assert_eq!(headless.integration_family(), local.integration_family());
    assert_eq!(
        local.transport_family().as_str(),
        "kimi-local-server-rest-ws-v2"
    );
    assert!(local.supports_execution_layer(ExecutionLayer::HarnessInteraction));
    assert!(local.supports_operation_shape(OperationShape::ProviderSessionManagement));
    assert!(local.supports_role(DriverRole::ProviderSessionManagement));
    assert!(local.supports_role(DriverRole::StructuredRun));
    assert!(local.supports_role(DriverRole::InteractiveSession));
    assert!(local.supports_role(DriverRole::ModelCatalog));
    assert!(acp.supports_role(DriverRole::InteractiveSession));
    assert!(!acp.supports_role(DriverRole::ProviderSessionManagement));
    assert!(headless.supports_role(DriverRole::Discovery));
    assert!(headless.supports_role(DriverRole::StructuredRun));
    assert!(!headless.supports_role(DriverRole::InteractiveSession));

    for role in [
        DriverRole::Discovery,
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

    for exact in ["0.28.1", "0.29.0", "0.29.1", "0.29.2"] {
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

#[test]
fn later_currentness_corpus_is_bounded_valid_and_exactly_provenanced() {
    for corpus in [
        include_str!("fixtures/kimi-code-0.29.1-0.29.2/headless-complete.jsonl"),
        include_str!("fixtures/kimi-code-0.29.1-0.29.2/headless-tools.jsonl"),
        include_str!("fixtures/kimi-code-0.29.1-0.29.2/headless-retry.jsonl"),
    ] {
        let lines = corpus.lines().collect::<Vec<_>>();
        assert!(!lines.is_empty());
        assert!(lines.len() <= 8);
        for line in lines {
            assert!(line.len() <= 4_096);
            let value: serde_json::Value =
                serde_json::from_str(line).expect("headless corpus line is JSON");
            assert!(matches!(
                value.get("role").and_then(serde_json::Value::as_str),
                Some("assistant" | "tool" | "meta")
            ));
        }
    }

    let provenance = include_str!("fixtures/kimi-code-0.29.1-0.29.2/README.md");
    for exact in [
        "785c319619ad4cbf87d8598afaea36c989f6cb66",
        "f4c3967a417a539372eadab6c809d27b8a14c005",
        "57503c7c4d854f2c66ea32e10cba28b2c5715e9c",
        "8a45f10eddbb35c317047e82e567cdb59a220b4f",
        "458380a0eb0a2248b79735c3ed48b3f632ad5de6",
    ] {
        assert!(provenance.contains(exact));
    }

    let retained: serde_json::Value = serde_json::from_str(include_str!(
        "fixtures/kimi-code-0.29.1-0.29.2/retained-execution.json"
    ))
    .expect("retained-execution corpus is JSON");
    assert_eq!(
        retained["managed_recovery"]["policy"]["prohibited"],
        "reject_before_effects"
    );
    assert_eq!(
        retained["local_server_reattachment"]["maximum"]
            .as_u64()
            .expect("maximum is numeric"),
        1
    );
    assert_eq!(
        retained["local_server_reattachment"]["last_accepted_cursor"]["seq"],
        11
    );
    assert_eq!(
        retained["local_server_reattachment"]["first_new_event"]["seq"],
        12
    );
    assert!(
        retained["local_server_reattachment"]["forbidden_dispatch"]
            .as_array()
            .expect("forbidden dispatch is bounded")
            .iter()
            .any(|value| value == "prompt_submit")
    );
}
