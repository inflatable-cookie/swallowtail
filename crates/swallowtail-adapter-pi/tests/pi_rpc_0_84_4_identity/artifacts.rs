use super::support::{
    ARGS_BLOB, FROZEN_CLI_SHA256, GITHUB_TAG_COMMIT, HOST_SHA256, IDENTITY, JSON_EVENT_BLOB,
    JSONL_BLOB, PRIOR_IDENTITY, RPC_MODE_BLOB, RPC_TYPES_BLOB, SESSION_CWD_BLOB, TARBALL_SHA256,
    assert_blob, assert_sha256, json, strings,
};
use swallowtail_adapter_pi::PI_PACKAGE_AXIS;

#[test]
fn official_package_release_and_artifact_identity_is_exact() {
    let prior = json(PRIOR_IDENTITY);
    let identity = json(IDENTITY);
    assert_eq!(prior["official"]["version"], "0.84.3");
    assert_eq!(identity["axis"], PI_PACKAGE_AXIS);
    assert_eq!(identity["npm_package"], "@earendil-works/pi-coding-agent");
    assert_eq!(identity["npm_latest"], true);
    assert_eq!(identity["not_oh_my_pi"], true);
    assert_eq!(identity["oh_my_pi_latest_at_observation"], "18.1.0");
    assert_eq!(identity["official"]["version"], "0.84.4");
    assert_eq!(
        identity["official"]["published_at"],
        "2026-08-28T22:07:57.753Z"
    );
    assert_eq!(identity["official"]["github_tag"], "v0.84.4");
    assert_eq!(
        identity["official"]["github_release_published_at"],
        "2026-08-28T22:08:23Z"
    );
    assert_eq!(
        identity["official"]["npm_integrity"],
        "sha512-jmOlrqUmvhh/siNWFRXjYLJzhKFIHNsAQaysRwzQPQFnPAaV/vhqHsLH/MBsIISA1Rjj7WTUFR3nJrpXoLx39w=="
    );
    assert_eq!(
        identity["official"]["npm_shasum"],
        "3a2f04bfc5e463b4cfa36b174a586d11a0bdf9ad"
    );
    assert_eq!(identity["official"]["npm_git_head"], GITHUB_TAG_COMMIT);
    assert_eq!(identity["official"]["github_tag_commit"], GITHUB_TAG_COMMIT);
    assert_eq!(identity["npm_git_head_matches_github_tag"], true);
    assert_sha256(&identity["official"]["tarball_sha256"], TARBALL_SHA256);
    assert_sha256(
        &identity["official"]["package_json_sha256"],
        "db9fead11bd2ddf7a327d2c2d11b535f30d059241c251d376837d5ab638a5576",
    );
    assert_sha256(
        &identity["official"]["extracted_cli_sha256"],
        FROZEN_CLI_SHA256,
    );
    assert_eq!(
        identity["official"]["extracted_cli_size"].as_u64(),
        Some(710)
    );
    assert_sha256(
        &prior["official"]["extracted_cli_sha256"],
        FROZEN_CLI_SHA256,
    );
    assert_sha256(
        &identity["official"]["extracted_bundle_cli_sha256"],
        "5406c369954516fb56879d685e082ff9095cd6e06e41af406f394942377fd4bf",
    );
    assert_ne!(
        identity["official"]["extracted_bundle_cli_sha256"],
        prior["official"]["extracted_bundle_cli_sha256"]
    );
    assert_sha256(
        &identity["official"]["extracted_bundle_index_sha256"],
        "d6744208473f5f0f0a199377165922340e105cb9a98693031c3fd5bbbee3d484",
    );
    assert_eq!(
        identity["official"]["extracted_bundle_index_size"].as_u64(),
        Some(6260)
    );
}

#[test]
fn host_0_83_0_matches_frozen_0_84_2_digest_and_was_not_replaced() {
    let identity = json(IDENTITY);
    assert_eq!(identity["host"]["version"], "0.83.0");
    assert_eq!(identity["host"]["matches_frozen_0_84_2_host_digest"], true);
    assert_sha256(&identity["host"]["executable_sha256"], HOST_SHA256);
    assert_eq!(identity["host"]["executable_size"].as_u64(), Some(681));
    assert_eq!(identity["identity_decision"]["host_install_changed"], false);
    assert_eq!(
        identity["identity_decision"]["official_binary_executed"],
        false
    );
}

#[test]
fn selected_source_blobs_are_exact() {
    let identity = json(IDENTITY);
    let blobs = &identity["selected_blobs_at_0_84_4"];
    assert_blob(
        &blobs["rpc_docs"],
        "52dbf884f53c281329e83444574c74c142564181",
    );
    assert_blob(&blobs["rpc_types"], RPC_TYPES_BLOB);
    assert_blob(&blobs["rpc_mode"], RPC_MODE_BLOB);
    assert_blob(&blobs["jsonl"], JSONL_BLOB);
    assert_blob(&blobs["session_cwd"], SESSION_CWD_BLOB);
    assert_blob(&blobs["json_event"], JSON_EVENT_BLOB);
    assert_blob(&blobs["args"], ARGS_BLOB);
    assert_eq!(
        strings(&identity["selected_blobs_identical_to_0_84_3"]),
        ["jsonl", "session_cwd", "json_event", "args"]
    );
    let prior = json(PRIOR_IDENTITY);
    assert_eq!(prior["selected_blobs_at_0_84_3"]["jsonl"], blobs["jsonl"]);
    assert_eq!(
        prior["selected_blobs_at_0_84_3"]["session_cwd"],
        blobs["session_cwd"]
    );
    assert_eq!(
        prior["selected_blobs_at_0_84_3"]["json_event"],
        blobs["json_event"]
    );
    assert_eq!(prior["selected_blobs_at_0_84_3"]["args"], blobs["args"]);
    assert_ne!(
        prior["selected_blobs_at_0_84_3"]["rpc_types"],
        blobs["rpc_types"]
    );
    assert_ne!(
        prior["selected_blobs_at_0_84_3"]["rpc_mode"],
        blobs["rpc_mode"]
    );
}
