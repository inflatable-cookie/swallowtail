use super::support::{
    AGENT_SESSION_BLOB, ARGS_BLOB, FROZEN_ARGS_JS, FROZEN_RPC_MODE_JS, FROZEN_RPC_TYPES_JS,
    GITHUB_TAG_COMMIT, HOST_SHA256, IDENTITY, INTERMEDIATE_GITHUB_TAG_COMMIT, JSON_EVENT_BLOB,
    JSONL_BLOB, PRIOR_IDENTITY, RPC_DOCS_BLOB, RPC_MODE_BLOB, RPC_TYPES_BLOB, SESSION_CWD_BLOB,
    TARBALL_SHA256, assert_blob, assert_sha256, json, strings,
};
use swallowtail_adapter_pi::PI_PACKAGE_AXIS;

#[test]
fn official_package_release_and_artifact_identity_is_exact() {
    let prior = json(PRIOR_IDENTITY);
    let identity = json(IDENTITY);
    assert_eq!(prior["official"]["version"], "0.84.4");
    assert_eq!(identity["axis"], PI_PACKAGE_AXIS);
    assert_eq!(identity["npm_package"], "@earendil-works/pi-coding-agent");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["not_oh_my_pi"], true);
    assert_eq!(identity["oh_my_pi_latest_at_observation"], "18.1.15");
    assert_eq!(identity["dist_inventory"], "dist-inventory.json");
    assert_eq!(identity["official"]["version"], "0.85.1");
    assert_eq!(
        identity["official"]["published_at"],
        "2026-09-05T12:17:19.281Z"
    );
    assert_eq!(identity["official"]["github_tag"], "v0.85.1");
    assert_eq!(
        identity["official"]["github_release_published_at"],
        "2026-09-05T12:29:01Z"
    );
    assert_eq!(
        identity["official"]["npm_integrity"],
        "sha512-FGRN+OHbWaefBPGaTggAdLjrIHW+s2PzLyglz/5dfLzb9of7uuXMXYC0fJIeZTw+shS32o2cuQ9jF7YSDuL/oQ=="
    );
    assert_eq!(
        identity["official"]["npm_shasum"],
        "4cd00f653c3dabeb193b46f511044e7fbfe0f947"
    );
    assert_eq!(identity["official"]["npm_git_head"], GITHUB_TAG_COMMIT);
    assert_eq!(identity["official"]["github_tag_commit"], GITHUB_TAG_COMMIT);
    assert_eq!(identity["npm_git_head_matches_github_tag"], true);
    assert_sha256(&identity["official"]["tarball_sha256"], TARBALL_SHA256);
    assert_sha256(
        &identity["official"]["package_json_sha256"],
        "f1738e4b42203e5f22bcb513f13fb2fb224f1e98d1f129ff042f87048665a94c",
    );
    assert_sha256(
        &identity["official"]["extracted_cli_sha256"],
        "8189b66abc4f9f431dbb70941dcba690d76d040de1fbfff212886be35a53639d",
    );
    assert_eq!(
        identity["official"]["extracted_cli_size"].as_u64(),
        Some(169)
    );
    assert_sha256(
        &identity["official"]["extracted_setup_js_sha256"],
        "4a2a7a0dbf82e2e5d18cec90896b36cbedce78b3d09e78d4040ac94fa3fbeba8",
    );
    assert_sha256(
        &identity["official"]["extracted_bundle_cli_sha256"],
        HOST_SHA256,
    );
    assert_eq!(
        identity["official"]["extracted_bundle_cli_size"].as_u64(),
        Some(660)
    );
    assert_sha256(
        &identity["official"]["extracted_rpc_types_js_sha256"],
        FROZEN_RPC_TYPES_JS,
    );
    assert_sha256(
        &identity["official"]["extracted_rpc_mode_js_sha256"],
        FROZEN_RPC_MODE_JS,
    );
    assert_sha256(
        &identity["official"]["extracted_args_js_sha256"],
        FROZEN_ARGS_JS,
    );
    assert_sha256(
        &prior["official"]["extracted_rpc_types_js_sha256"],
        FROZEN_RPC_TYPES_JS,
    );
    assert_sha256(
        &prior["official"]["extracted_rpc_mode_js_sha256"],
        FROZEN_RPC_MODE_JS,
    );
    assert_sha256(
        &prior["official"]["extracted_args_js_sha256"],
        FROZEN_ARGS_JS,
    );
}

#[test]
fn intermediate_0_85_0_identity_is_exact() {
    let identity = json(IDENTITY);
    let hop = &identity["published_intermediate_0_85_0"];
    assert_eq!(hop["version"], "0.85.0");
    assert_eq!(
        hop["role"],
        "intermediate_supporting_evidence_not_standalone_ceiling"
    );
    assert_eq!(hop["published_at"], "2026-09-04T10:18:05.208Z");
    assert_eq!(hop["github_tag"], "v0.85.0");
    assert_eq!(hop["npm_git_head"], INTERMEDIATE_GITHUB_TAG_COMMIT);
    assert_eq!(hop["github_tag_commit"], INTERMEDIATE_GITHUB_TAG_COMMIT);
    assert_sha256(
        &hop["tarball_sha256"],
        "a0895f70a9efd9dde2a69b9cee04cb3b7c5aab68f5d47aad92b63f27a4ca13c8",
    );
    assert_sha256(
        &hop["extracted_args_js_sha256"],
        "05f30cc6545ef472fd0d8519c9e26b80b7a03432440ce42c356b65206f1077f9",
    );
    assert_ne!(
        hop["extracted_args_js_sha256"],
        identity["official"]["extracted_args_js_sha256"]
    );
    assert_eq!(
        hop["extracted_agent_session_js_sha256"],
        identity["official"]["extracted_agent_session_js_sha256"]
    );
}

#[test]
fn host_0_85_1_matches_official_bundle_cli_and_was_not_replaced() {
    let identity = json(IDENTITY);
    assert_eq!(identity["host"]["version"], "0.85.1");
    assert_eq!(identity["host"]["matches_official_0_85_1_bundle_cli"], true);
    assert_sha256(&identity["host"]["executable_sha256"], HOST_SHA256);
    assert_eq!(identity["host"]["executable_size"].as_u64(), Some(660));
    assert_eq!(identity["identity_decision"]["host_install_changed"], false);
    assert_eq!(
        identity["identity_decision"]["official_binary_executed"],
        false
    );
}

#[test]
fn selected_source_blobs_are_exact() {
    let identity = json(IDENTITY);
    let blobs = &identity["selected_blobs_at_0_85_1"];
    assert_blob(&blobs["rpc_docs"], RPC_DOCS_BLOB);
    assert_blob(&blobs["rpc_types"], RPC_TYPES_BLOB);
    assert_blob(&blobs["rpc_mode"], RPC_MODE_BLOB);
    assert_blob(&blobs["jsonl"], JSONL_BLOB);
    assert_blob(&blobs["session_cwd"], SESSION_CWD_BLOB);
    assert_blob(&blobs["json_event"], JSON_EVENT_BLOB);
    assert_blob(&blobs["args"], ARGS_BLOB);
    assert_blob(&blobs["agent_session"], AGENT_SESSION_BLOB);
    assert_eq!(
        strings(&identity["selected_blobs_identical_to_0_84_4"]),
        [
            "rpc_types",
            "rpc_mode",
            "jsonl",
            "session_cwd",
            "json_event"
        ]
    );
    assert_eq!(
        strings(&identity["selected_blobs_identical_0_84_4_and_0_85_1"]),
        [
            "rpc_types",
            "rpc_mode",
            "jsonl",
            "session_cwd",
            "json_event",
            "args"
        ]
    );
    let prior = json(PRIOR_IDENTITY);
    let prior_blobs = &prior["selected_blobs_at_0_84_4"];
    assert_eq!(prior_blobs["rpc_types"], blobs["rpc_types"]);
    assert_eq!(prior_blobs["rpc_mode"], blobs["rpc_mode"]);
    assert_eq!(prior_blobs["jsonl"], blobs["jsonl"]);
    assert_eq!(prior_blobs["session_cwd"], blobs["session_cwd"]);
    assert_eq!(prior_blobs["json_event"], blobs["json_event"]);
    assert_eq!(prior_blobs["args"], blobs["args"]);
    assert_ne!(prior_blobs["rpc_docs"], blobs["rpc_docs"]);
    let hop = &identity["selected_blobs_at_0_85_0"];
    assert_eq!(hop["rpc_docs"], blobs["rpc_docs"]);
    assert_eq!(hop["agent_session"], blobs["agent_session"]);
    assert_ne!(hop["args"], blobs["args"]);
}
