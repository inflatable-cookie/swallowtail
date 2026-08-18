use serde_json::Value;
use swallowtail_adapter_opencode::{
    OPENCODE_BASELINE_VERSION, OPENCODE_LATEST_QUALIFIED_VERSION, OPENCODE_SERVER_AXIS,
    opencode_http_claim, opencode_server_binding,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/opencode-1.18.18/identity.json");
const PROTOCOL: &str = include_str!("fixtures/opencode-1.18.18/protocol.json");

#[test]
fn identity_and_claim_qualify_1_18_18_as_compatible_extension() {
    let identity: Value =
        serde_json::from_str(IDENTITY).expect("OpenCode 1.18.18 identity corpus is valid JSON");
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("OpenCode 1.18.18 protocol corpus is valid JSON");

    assert_eq!(identity["axis"], OPENCODE_SERVER_AXIS);
    assert_eq!(identity["version"], "1.18.18");
    assert_eq!(identity["npm_package"], "opencode-ai");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(
        identity["npm_integrity"],
        "sha512-J+5HFq8tf+wPBBpBpMPSNjSytF2/EkNWYfFZh4si1d9auFbQriqDyqZv+vFUsLWERfdMU32Eajwuiq3rKBvZLQ=="
    );
    assert_eq!(identity["local_cli"], "1.18.18");
    assert!(is_sha256(
        identity["local_executable_sha256"]
            .as_str()
            .expect("host executable digest is text")
    ));
    assert_eq!(
        identity["openapi_sha256_1_18_11_through_1_18_18"],
        "5bbd6493a1a488ef4294889341c896e420f814ecea95822100aaa9f3f95ab2d1"
    );
    assert_eq!(identity["unpublished_gap_in_1_18_11_through_1_18_18"], false);

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(decision["add_private_surface"], "surface-19");
    assert_eq!(decision["raise_latest_qualified_to"], "1.18.18");
    assert_eq!(decision["keep_baseline"], "1.14.48");
    assert_eq!(decision["keep_unpublished_gaps"], true);
    assert_eq!(decision["flatten_to_single_interval"], false);
    assert_eq!(decision["new_public_operation"], false);
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
    assert_eq!(protocol["decoder_corpus"], "opencode-1.14.48");
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["attached_server_started"], false);

    assert_eq!(OPENCODE_BASELINE_VERSION, "1.14.48");
    assert_eq!(OPENCODE_LATEST_QUALIFIED_VERSION, "1.18.18");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "1.18.10"
    );

    let claim = opencode_http_claim();
    assert!(claim.supports(&version("1.18.10")));
    assert!(claim.supports(&version("1.18.11")));
    assert!(matches!(
        claim.assess(&version("1.18.18")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained
                && matched.behavior_revision().as_str() == "opencode.http-sse.surface-19"
    ));
    assert!(matches!(
        claim.assess(&version("1.18.19")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert!(!claim.permits(&version("1.15.8")));
    assert_eq!(
        opencode_server_binding("1.18.18")
            .expect("version binds")
            .axis()
            .as_str(),
        OPENCODE_SERVER_AXIS
    );
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
