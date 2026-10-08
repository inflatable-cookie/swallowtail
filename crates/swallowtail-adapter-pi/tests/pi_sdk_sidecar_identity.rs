use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use swallowtail_adapter_pi::{
    PI_SDK_SIDECAR_NODE_AXIS, PI_SDK_SIDECAR_PACKAGE_AXIS, PI_SDK_SIDECAR_SIDECAR_AXIS,
    PI_SDK_SIDECAR_WIRE_AXIS, pi_sdk_sidecar_node_claim, pi_sdk_sidecar_package_claim,
    pi_sdk_sidecar_sidecar_claim, pi_sdk_sidecar_wire_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const PROTOCOL: &str = include_str!("fixtures/pi-sdk-sidecar-v1/protocol.json");
const SIDECAR: &str = include_str!("../sidecar/pi-sdk-sidecar.mjs");
const NODE_TLS_PROOF: &str =
    include_str!("../../../docs/research/396-pi-sdk-sidecar-node-22-23-3-tls-offline-proof.json");
const NODE_HOP_REVIEW: &str =
    include_str!("../../../docs/research/396-pi-sdk-sidecar-node-22-23-3-hop-review.json");
const NODE_TREE_DIFF: &str =
    include_str!("../../../docs/research/396-pi-sdk-sidecar-node-22-23-3-tree-diff.json");
const NODE_ROOT_DIFF: &str = include_str!(
    "../../../docs/research/396-pi-sdk-sidecar-node-22-23-3-root-certificate-diff.json"
);

#[test]
fn sidecar_identity_and_claims_match_the_frozen_corpus() {
    let protocol: Value =
        serde_json::from_str(PROTOCOL).expect("Pi SDK sidecar protocol corpus is valid JSON");

    assert_eq!(protocol["wire"], "swallowtail-pi-sdk-jsonl-v1");
    assert_eq!(protocol["behavior_revision"], "pi.sdk-sidecar-v1");
    assert_eq!(protocol["sdk_package"], "@earendil-works/pi-coding-agent");
    assert_eq!(protocol["sdk_version"], "0.84.2");
    assert_eq!(protocol["node_runtime"], "22.23.3");
    assert_eq!(protocol["node_requirement"], ">=22.19.0");
    assert_eq!(
        protocol["node_qualified_points"],
        serde_json::json!(["22.23.2", "22.23.3"])
    );
    assert_eq!(protocol["compatibility_claim"], "qualified_only_segment");
    assert_eq!(protocol["sidecar_entry_file"], "pi-sdk-sidecar.mjs");

    for (claim, axis, version) in [
        (
            pi_sdk_sidecar_package_claim(),
            PI_SDK_SIDECAR_PACKAGE_AXIS,
            "0.84.2",
        ),
        (
            pi_sdk_sidecar_node_claim(),
            PI_SDK_SIDECAR_NODE_AXIS,
            "22.23.3",
        ),
        (
            pi_sdk_sidecar_wire_claim(),
            PI_SDK_SIDECAR_WIRE_AXIS,
            "swallowtail-pi-sdk-jsonl-v1",
        ),
        (
            pi_sdk_sidecar_sidecar_claim(),
            PI_SDK_SIDECAR_SIDECAR_AXIS,
            swallowtail_adapter_pi::sidecar::PI_SDK_SIDECAR_SOURCE_TAG,
        ),
    ] {
        assert_eq!(claim.axis().as_str(), axis);
        assert!(matches!(
            claim.assess(&InterfaceVersion::new(version).expect("valid version")),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
                    && matched.behavior_revision().as_str() == "pi.sdk-sidecar-v1"
        ));
    }

    // The sidecar claims inherit nothing from the RPC window: later stable
    // points are rejected, not unverified-newer.
    let package = pi_sdk_sidecar_package_claim();
    assert!(!matches!(
        package.assess(&InterfaceVersion::new("0.84.3").expect("valid version")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert!(!package.permits(&InterfaceVersion::new("0.84.3").expect("valid version")));
    assert!(!package.permits(&InterfaceVersion::new("0.84.4").expect("valid version")));
    assert!(
        swallowtail_adapter_pi::sidecar::PI_SDK_SIDECAR_SOURCE_TAG
            .starts_with(protocol["sidecar_source_tag_prefix"].as_str().unwrap())
    );

    let node_claim = pi_sdk_sidecar_node_claim();
    assert!(node_claim.permits(&InterfaceVersion::new("22.23.2").expect("valid version")));
    assert!(!node_claim.permits(&InterfaceVersion::new("22.23.4").expect("valid version")));
}

#[test]
fn sidecar_keeps_session_paths_inside_the_approved_directory() {
    assert!(SIDECAR.contains("state.sessionManager.listAll(state.sessionDir)"));
    assert!(SIDECAR.contains("realpath(matches[0].path)"));
    assert!(SIDECAR.contains("sessionRef: session.sessionId"));
    assert!(!SIDECAR.contains("sessionRef: session.sessionFile"));
    assert!(!SIDECAR.contains("existsSync(sessionRef)"));
}

#[test]
fn node_hop_freezes_exact_distribution_classes_and_default_tls_failure_boundary() {
    let proof: Value = serde_json::from_str(NODE_TLS_PROOF).expect("Node TLS proof is valid JSON");
    assert_eq!(
        proof["schema"],
        "swallowtail-node22-pi-tls-offline-proof-v1"
    );
    assert_eq!(proof["network_scope"], "loopback-only");
    assert_eq!(proof["provider_requests"], false);
    assert_eq!(
        proof["tls_policy"],
        "default verification and hostname checks retained"
    );
    assert_eq!(proof["roots"]["22.23.2"]["count"], 145);
    assert_eq!(
        proof["roots"]["22.23.2"]["sorted_der_fingerprint_set_sha256"],
        "198226aedca48a2d2d256da63ffb5836367954a983de81a6a3482b0a3e34ec50"
    );
    assert_eq!(proof["roots"]["22.23.3"]["count"], 119);
    assert_eq!(
        proof["roots"]["22.23.3"]["sorted_der_fingerprint_set_sha256"],
        "f9509629db3d7460eb77ac966b63bd2894aebaecdef37a3ee07433131a1ec5ab"
    );
    for version in ["22.23.2", "22.23.3"] {
        assert_eq!(
            proof["offline_fetch"][version]["failure"],
            "DEPTH_ZERO_SELF_SIGNED_CERT"
        );
        assert_eq!(
            proof["offline_fetch"][version]["hostname_failure"],
            "ERR_TLS_CERT_ALTNAME_INVALID"
        );
    }

    let review: Value =
        serde_json::from_str(NODE_HOP_REVIEW).expect("Node hop review is valid JSON");
    assert_eq!(review["distribution"]["changed_file_count"], 393);
    assert_eq!(review["distribution"]["added_file_count"], 0);
    assert_eq!(review["distribution"]["removed_file_count"], 0);
    let categories = review["distribution"]["categories"]
        .as_array()
        .expect("distribution categories are an array");
    assert_eq!(categories.len(), 7);
    let recorded_categories = categories
        .iter()
        .map(|category| {
            (
                category["id"]
                    .as_str()
                    .expect("distribution category id is a string")
                    .to_owned(),
                category["count"]
                    .as_u64()
                    .expect("distribution category count is an integer") as usize,
            )
        })
        .collect::<BTreeMap<_, _>>();
    assert_eq!(
        recorded_categories,
        BTreeMap::from([
            ("node_executable".to_owned(), 1),
            ("distribution_readme_changelog".to_owned(), 2),
            ("native_addon_build_metadata".to_owned(), 1),
            ("native_addon_public_headers".to_owned(), 3),
            ("openssl_native_build_headers".to_owned(), 172),
            ("embedded_corepack".to_owned(), 4),
            ("embedded_npm".to_owned(), 210),
        ])
    );
    let tree_diff: Value =
        serde_json::from_str(NODE_TREE_DIFF).expect("Node tree diff is valid JSON");
    assert_eq!(
        format!("{:x}", Sha256::digest(NODE_TREE_DIFF.as_bytes())),
        "7d27b45bd8822d0ce0dfd92191a3531b43f65d4e8a0e81601f4544fb3928d776"
    );
    assert_eq!(
        tree_diff["compared"],
        serde_json::json!(["22.23.2", "22.23.3"])
    );
    assert_eq!(tree_diff["added"].as_array().unwrap().len(), 0);
    assert_eq!(tree_diff["removed"].as_array().unwrap().len(), 0);
    assert_eq!(tree_diff["changed"].as_array().unwrap().len(), 393);
    assert_eq!(tree_diff["identical"].as_array().unwrap().len(), 5472);
    let mut classified = BTreeMap::new();
    for path in tree_diff["changed"].as_array().unwrap() {
        let path = path.as_str().expect("distribution path is a string");
        let category = node_distribution_category(path)
            .unwrap_or_else(|| panic!("unclassified Node distribution path: {path}"));
        *classified.entry(category).or_insert(0_usize) += 1;
    }
    assert_eq!(
        classified,
        BTreeMap::from([
            ("node_executable", 1),
            ("distribution_readme_changelog", 2),
            ("native_addon_build_metadata", 1),
            ("native_addon_public_headers", 3),
            ("openssl_native_build_headers", 172),
            ("embedded_corepack", 4),
            ("embedded_npm", 210),
        ])
    );
    let root_diff: Value =
        serde_json::from_str(NODE_ROOT_DIFF).expect("Node root diff is valid JSON");
    assert_eq!(
        format!("{:x}", Sha256::digest(NODE_ROOT_DIFF.as_bytes())),
        "e7d58cb8378359eb44646abfa925048a843b7a754af8704fc64ea27aa2ca7778"
    );
    assert_eq!(root_diff["baseline_count"], 145);
    assert_eq!(root_diff["target_count"], 119);
    assert_eq!(
        root_diff["removed_or_changed"].as_array().unwrap().len(),
        26
    );
    assert_eq!(root_diff["added_or_changed"].as_array().unwrap().len(), 0);
    assert_eq!(
        review["selected_source_changes"]["undici"]["from"],
        "6.28.0"
    );
    assert_eq!(review["selected_source_changes"]["undici"]["to"], "6.28.1");
    assert_eq!(
        review["selected_source_changes"]["tls"]["root_set_change"],
        "26 prior root identities removed or changed; no target additions"
    );
    assert_eq!(review["pi_source_path"]["tag"], "v0.84.2");
    assert_eq!(
        format!("{:x}", Sha256::digest(SIDECAR.as_bytes())),
        review["pi_source_path"]["sidecar_asset_sha256"]
            .as_str()
            .expect("frozen sidecar asset digest is a string")
    );
    assert_eq!(
        review["pi_source_path"]["commit"],
        "914cf1472e715297caa30db4b9535d534a9eb718"
    );
    assert_eq!(
        review["pi_source_path"]["route_call_path"]["provider_http"],
        "Pi provider HTTP defaults to options.fetch or globalThis.fetch; the sidecar supplies no custom fetch, so Node global Fetch uses its bundled Undici and TLS roots."
    );
    assert_eq!(
        review["pi_source_path"]["route_call_path"]["provider_websocket"],
        "Pi Codex auto transport uses globalThis.WebSocket; Node global WebSocket uses its bundled Undici implementation."
    );
    assert_eq!(
        review["qualification_decision"]["qualified_segment"],
        "22.23.2..=22.23.3"
    );
    assert_eq!(
        review["qualification_decision"]["other_axes_changed"],
        false
    );
}

#[test]
fn sidecar_uses_the_runtime_default_tls_path_without_tls_overrides() {
    assert_eq!(SIDECAR.matches("allowModelNetwork: false").count(), 2);
    assert_eq!(SIDECAR.matches("modelsPath: null").count(), 2);
    for forbidden in [
        "NODE_EXTRA_CA_CERTS",
        "NODE_USE_SYSTEM_CA",
        "NODE_TLS_REJECT_UNAUTHORIZED",
        "NODE_OPTIONS",
        "rejectUnauthorized",
        "setGlobalDispatcher",
        "ca:",
        "dispatcher:",
    ] {
        assert!(
            !SIDECAR.contains(forbidden),
            "sidecar must not override the Node TLS trust path with {forbidden}"
        );
    }
}

fn node_distribution_category(path: &str) -> Option<&'static str> {
    match path {
        "bin/node" => Some("node_executable"),
        "CHANGELOG.md" | "README.md" => Some("distribution_readme_changelog"),
        "include/node/common.gypi" => Some("native_addon_build_metadata"),
        "include/node/js_native_api.h"
        | "include/node/js_native_api_types.h"
        | "include/node/node_version.h" => Some("native_addon_public_headers"),
        _ if path.starts_with("include/node/openssl/") => Some("openssl_native_build_headers"),
        _ if path.starts_with("lib/node_modules/corepack/") => Some("embedded_corepack"),
        _ if path.starts_with("lib/node_modules/npm/") => Some("embedded_npm"),
        _ => None,
    }
}
