use serde_json::Value;
use swallowtail_adapter_opencode::{
    OPENCODE_BASELINE_VERSION, OPENCODE_LATEST_QUALIFIED_VERSION, OPENCODE_SERVER_AXIS,
    opencode_http_claim, opencode_server_binding,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/opencode-1.18.20/identity.json");
const PROTOCOL: &str = include_str!("fixtures/opencode-1.18.20/protocol.json");

#[test]
fn identity_and_claim_qualify_1_18_20_as_compatible_extension() {
    let identity: Value =
        serde_json::from_str(IDENTITY).expect("OpenCode 1.18.20 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("OpenCode 1.18.20 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], OPENCODE_SERVER_AXIS);
    assert_eq!(identity["version"], "1.18.20");
    assert_eq!(identity["npm_package"], "opencode-ai");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(
        identity["npm_integrity"],
        "sha512-8c2yJ/Oe1qFi9KYE0KS9WCyy6O1QtI9odzBmBWGOeyOgXTn/hGOwCp/fgcHY2qVQ2TVgkQXze7jXjJ6AFyeU0Q=="
    );
    assert_eq!(identity["local_cli"], Value::Null);
    assert_eq!(identity["openapi_byte_identical_to_1_18_18"], true);
    assert_eq!(
        identity["openapi_sha256_1_18_20"],
        "5bbd6493a1a488ef4294889341c896e420f814ecea95822100aaa9f3f95ab2d1"
    );
    assert_eq!(
        identity["unpublished_gap_in_1_18_19_through_1_18_20"],
        false
    );
    assert_eq!(identity["unpublished_next"], "1.18.21");

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["add_private_surface"], Value::Null);
    assert_eq!(decision["raise_latest_qualified_to"], "1.18.20");
    assert_eq!(decision["keep_baseline"], "1.14.48");
    assert_eq!(decision["keep_unpublished_gaps"], true);
    assert_eq!(decision["flatten_to_single_interval"], false);
    assert_eq!(decision["new_public_operation"], false);
    assert_eq!(
        decision["import_reconcile_history_detach_inherit_on_unverified_newer"],
        false
    );
    assert_eq!(decision["provider_prompt_sent"], false);
    assert_eq!(decision["attached_server_started"], false);
    assert_eq!(decision["host_install_changed"], false);

    let routes = protocol["selected_routes"]
        .as_array()
        .expect("selected routes are an array");
    for required in [
        "global.health",
        "provider.list",
        "session.create",
        "session.prompt_async",
        "event.subscribe",
        "session.abort",
    ] {
        assert!(
            routes.iter().any(|route| route == required),
            "missing selected route {required}"
        );
    }
    assert_eq!(protocol["selected_operation_objects_unchanged"], true);
    assert_eq!(protocol["delete_closure_unchanged"], true);
    assert_eq!(protocol["import_closure_unchanged"], true);
    assert_eq!(protocol["selected_closure_delta"], Value::Null);
    assert_eq!(protocol["decoder_corpus"], "opencode-1.14.48");
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["attached_server_started"], false);

    assert_eq!(OPENCODE_BASELINE_VERSION, "1.14.48");
    assert_eq!(OPENCODE_LATEST_QUALIFIED_VERSION, "1.18.20");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "1.18.18"
    );

    let claim = opencode_http_claim();
    assert!(claim.supports(&version("1.18.18")));
    assert!(claim.supports(&version("1.18.19")));
    assert!(matches!(
        claim.assess(&version("1.18.20")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
                && matched.behavior_revision().as_str() == "opencode.http-sse.surface-19"
    ));
    assert!(matches!(
        claim.assess(&version("1.18.21")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert!(!claim.permits(&version("1.15.8")));
    assert_eq!(
        opencode_server_binding("1.18.20")
            .expect("version binds")
            .axis()
            .as_str(),
        OPENCODE_SERVER_AXIS
    );
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
