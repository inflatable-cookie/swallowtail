//! Currentness and mutation-sensitive reopen evidence for official Qoder npm `1.1.54`.

use serde_json::Value;
use swallowtail_adapter_qoder::{QODER_PACKAGE_VERSION, qoder_headless_claim};
use swallowtail_core::InterfaceVersion;

const IDENTITY: &str = include_str!("fixtures/qoder-headless-1.1.54/identity.json");
const INVENTORY: &str = include_str!("fixtures/qoder-headless-1.1.54/dist-inventory.json");
const PROTOCOL: &str = include_str!("fixtures/qoder-headless-1.1.54/protocol.json");

fn fixture(body: &str, name: &str) -> Value {
    serde_json::from_str(body).unwrap_or_else(|error| panic!("{name}: {error}"))
}

#[test]
fn current_identity_is_one_exact_private_milestone_point() {
    let identity = fixture(IDENTITY, "identity");
    assert_eq!(QODER_PACKAGE_VERSION, "1.1.54");
    assert_eq!(identity["npm_channel"]["latest"], "1.1.54");
    assert_eq!(identity["npm_channel"]["beta"], "1.1.54-beta.1");
    assert_eq!(identity["previous_ceiling"], "1.1.52");
    assert_eq!(
        identity["compared"],
        serde_json::json!(["1.1.52", "1.1.53", "1.1.54"])
    );
    assert_eq!(
        identity["published_stables_since_previous_ceiling"],
        serde_json::json!(["1.1.53", "1.1.54"])
    );
    assert_eq!(identity["first_unpublished_later_stable"], "1.1.55");
    assert_eq!(identity["first_unpublished_verified"], true);
    assert_eq!(identity["official"]["file_count"], 30);
    assert_eq!(identity["official"]["unpacked_size"], 69_199_769_i64);
    assert_eq!(identity["identity_decision"]["shape"], "private-milestone");
    assert_eq!(identity["identity_decision"]["chosen_max_turns"], 8);
    assert_eq!(
        identity["identity_decision"]["previous_point_retained"],
        false
    );
    assert_eq!(
        identity["identity_decision"]["new_behavior_revision"],
        "qoder.headless.stdio-stream-json-v2"
    );

    let claim = qoder_headless_claim();
    assert_eq!(claim.id().as_str(), "qoder.headless.package-window-2");
    assert!(
        claim
            .assess(&InterfaceVersion::new("1.1.54").expect("current"))
            .is_permitted()
    );
    assert!(
        !claim
            .assess(&InterfaceVersion::new("1.1.25").expect("historical"))
            .is_permitted()
    );
    assert!(
        !claim
            .assess(&InterfaceVersion::new("1.1.55").expect("next"))
            .is_permitted()
    );
}

#[test]
fn inventory_and_protocol_bind_the_explicit_eight_turn_terminal() {
    let inventory = fixture(INVENTORY, "dist inventory");
    assert_eq!(
        inventory["compared"],
        serde_json::json!(["1.1.52", "1.1.53", "1.1.54"])
    );
    assert_eq!(inventory["package_file_counts"]["1.1.54"], 30);
    assert_eq!(
        inventory["from_hop_to_hop"]["1.1.53..1.1.54"]["added"],
        serde_json::json!([
            "package/bundle/vendor/sites/artifact.json",
            "package/bundle/vendor/sites/sites.zip"
        ])
    );
    assert_eq!(
        inventory["hashes"]["package/bundle/qoder-npm-dispatcher.cjs"]["1.1.54"],
        "37cc389f07b046d78a2c80ff8d9ab54f433d38f94bf1f221a9e7e1dea297d120"
    );
    assert!(
        inventory["from_hop_to_hop"]["1.1.53..1.1.54"]["identical"]
            .as_array()
            .expect("identical paths")
            .iter()
            .any(|path| path == "package/bundle/qoder-npm-dispatcher.cjs")
    );

    let protocol = fixture(PROTOCOL, "protocol");
    assert_eq!(protocol["artifact_revision"], "1.1.54");
    assert_eq!(protocol["selected_invocation"]["argv"][6], "--max-turns");
    assert_eq!(protocol["selected_invocation"]["argv"][7], "8");
    assert_eq!(protocol["max_turns_authority"]["adapter_owned"], true);
    assert_eq!(protocol["max_turns_authority"]["explicitly_declared"], true);
    assert_eq!(protocol["max_turns_authority"]["bound_turns"], 8);
    assert_eq!(
        protocol["max_turns_authority"]["terminal_subtype"],
        "error_max_turns"
    );
    assert_eq!(protocol["max_turns_authority"]["terminal_is_error"], true);
    assert_eq!(
        protocol["max_turns_authority"]["terminal_num_turns_equals_bound"],
        true
    );
    assert_eq!(
        protocol["max_turns_authority"]["terminal_mapping"],
        "swallowtail.qoder.headless.max_turns"
    );
    assert_eq!(
        protocol["live_evidence_disposition"]["provider_operation"],
        false
    );
    assert_eq!(
        protocol["live_evidence_disposition"]["downloaded_artifact_executed"],
        false
    );
}
