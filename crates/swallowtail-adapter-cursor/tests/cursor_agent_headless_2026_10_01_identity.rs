use serde_json::Value;
use sha2::{Digest, Sha256};

const IDENTITY: &str = include_str!("fixtures/cursor-agent-headless-2026.10.01/identity.json");
const PROTOCOL: &str = include_str!("fixtures/cursor-agent-headless-2026.10.01/protocol.json");
const ARTIFACT_IDENTITY: &str = include_str!("fixtures/cursor-agent-2026.10.01/identity.json");
const DIST_INVENTORY: &str = include_str!("fixtures/cursor-agent-2026.10.01/dist-inventory.json");

fn json(source: &str) -> Value {
    serde_json::from_str(source).expect("Cursor Agent headless identity fixture is valid JSON")
}

fn sha256_hex(source: &str) -> String {
    Sha256::digest(source.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<Vec<_>>()
        .join("")
}

fn archive_record<'a>(identity: &'a Value, version: &str) -> &'a Value {
    if identity["previous_ceiling"]["version"] == version {
        return &identity["previous_ceiling"];
    }
    identity["hops"]
        .as_array()
        .expect("artifact hops are an array")
        .iter()
        .find(|hop| hop["version"] == version)
        .expect("every headless artifact has a source identity record")
}

fn find_file<'a>(tree: &'a Value, path: &str) -> &'a Value {
    tree["files"]
        .as_array()
        .expect("complete artifact tree has a file array")
        .iter()
        .find(|file| file["path"] == path)
        .expect("selected headless file is present in its complete tree")
}

#[test]
fn identity_freezes_each_official_artifact_and_headless_only_qualification() {
    assert_eq!(
        sha256_hex(IDENTITY),
        "3393a931fe7d76b4ac93c76446868e6bb29a1f4d5863015b0d18cceddc46755d"
    );
    assert_eq!(
        sha256_hex(ARTIFACT_IDENTITY),
        "55a51a3d3192285fe07ff115150a2008aba94f66fe2f705e398b353a902af231"
    );
    assert_eq!(
        sha256_hex(DIST_INVENTORY),
        "84473ea98c85ed1474b9d9ea14921030a48821ac9ae99d8343d6dd259994d4fc"
    );

    let identity = json(IDENTITY);
    let artifact_identity = json(ARTIFACT_IDENTITY);
    let inventory = json(DIST_INVENTORY);
    assert_eq!(identity["axis"], "cursor-agent.release-date");
    assert_eq!(identity["qualified_route"], "cursor-agent.headless");
    assert_eq!(identity["current_official_stable"], "2026.10.01-14929f9");
    assert_eq!(identity["registry"]["latest_stable"], "2026.10.01-14929f9");
    assert_eq!(
        identity["published_stables_after_ceiling"],
        serde_json::json!([
            "2026.09.26-dd393fe",
            "2026.09.28-64d2043",
            "2026.10.01-14929f9"
        ])
    );
    assert_eq!(
        identity["independently_unqualified_older_published"],
        serde_json::json!(["2026.08.25-3e8eec8", "2026.09.08-6caf4ff"])
    );
    assert_eq!(
        identity["complete_artifact_evidence"]["artifact_identity_sha256"],
        "55a51a3d3192285fe07ff115150a2008aba94f66fe2f705e398b353a902af231"
    );
    assert_eq!(
        identity["complete_artifact_evidence"]["complete_tree_inventory_sha256"],
        "84473ea98c85ed1474b9d9ea14921030a48821ac9ae99d8343d6dd259994d4fc"
    );
    assert_eq!(
        identity["complete_artifact_evidence"]["versions"],
        serde_json::json!([
            "2026.09.18-9a7762b",
            "2026.09.26-dd393fe",
            "2026.09.28-64d2043",
            "2026.10.01-14929f9"
        ])
    );
    assert_eq!(
        identity["complete_artifact_evidence"]["platforms"],
        serde_json::json!(["darwin-arm64", "linux-x64"])
    );

    let artifacts = identity["artifacts"]
        .as_array()
        .expect("headless artifacts are an array");
    assert_eq!(artifacts.len(), 8);
    let expected = [
        ("2026.09.18-9a7762b", "darwin-arm64", "1218.index.js"),
        ("2026.09.18-9a7762b", "linux-x64", "7021.index.js"),
        ("2026.09.26-dd393fe", "darwin-arm64", "9352.index.js"),
        ("2026.09.26-dd393fe", "linux-x64", "7021.index.js"),
        ("2026.09.28-64d2043", "darwin-arm64", "6949.index.js"),
        ("2026.09.28-64d2043", "linux-x64", "7000.index.js"),
        ("2026.10.01-14929f9", "darwin-arm64", "1962.index.js"),
        ("2026.10.01-14929f9", "linux-x64", "7000.index.js"),
    ];
    for (artifact, (version, platform, writer_chunk)) in artifacts.iter().zip(expected) {
        assert_eq!(artifact["version"], version);
        assert_eq!(artifact["platform"], platform);
        let source = archive_record(&artifact_identity, version);
        let (archive_sha_field, archive_url_field) = match platform {
            "darwin-arm64" => ("darwin_arm64_archive_sha256", "darwin_arm64_archive_url"),
            "linux-x64" => ("linux_x64_archive_sha256", "linux_x64_archive_url"),
            _ => unreachable!("expected platform is fixed above"),
        };
        assert_eq!(artifact["archive_sha256"], source[archive_sha_field]);
        assert_eq!(artifact["archive_url"], source[archive_url_field]);

        let tree = &inventory["archive_trees"][version][platform];
        assert_eq!(artifact["archive_sha256"], tree["archive_sha256"]);
        assert_eq!(artifact["package_file_count"], tree["file_count"]);
        assert_eq!(
            artifact["complete_tree_manifest_sha256"],
            tree["tree_manifest_sha256"]
        );
        let file_count =
            usize::try_from(tree["file_count"].as_u64().expect("file count is numeric"))
                .expect("package file count fits usize");
        assert_eq!(
            tree["files"].as_array().expect("files are an array").len(),
            file_count
        );

        let runtime_index = &artifact["runtime_index"];
        assert_eq!(runtime_index["path"], "dist-package/index.js");
        let runtime_file = find_file(tree, "dist-package/index.js");
        assert_eq!(runtime_index["sha256"], runtime_file["sha256"]);
        assert_eq!(runtime_index["size"], runtime_file["size"]);

        let writer = &artifact["headless_writer_chunk"];
        assert_eq!(writer["path"], format!("dist-package/{writer_chunk}"));
        let writer_file = find_file(tree, writer["path"].as_str().expect("writer path is text"));
        assert_eq!(writer["sha256"], writer_file["sha256"]);
        assert_eq!(writer["size"], writer_file["size"]);
    }

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["claim_id"],
        "cursor-agent.headless.release-window-3"
    );
    assert_eq!(
        decision["behavior_revision"],
        "cursor-agent.stream-json.structured-v1"
    );
    assert_eq!(decision["keep_baseline"], "2026.07.01-41b2de7");
    assert_eq!(decision["raise_headless_ceiling_to"], "2026.10.01-14929f9");
    assert_eq!(decision["keep_acp_ceiling"], "2026.09.18-9a7762b");
    for unchanged in [
        "infer_calendar_gap",
        "new_behavior_revision",
        "new_public_operation",
        "consumer_visible_narrowing",
        "new_authority_or_security_behavior",
        "host_install_or_update",
        "artifact_execution",
        "provider_prompt_sent",
        "credentials_used",
        "authenticated_catalogue_or_session_called",
    ] {
        assert!(
            !decision[unchanged]
                .as_bool()
                .expect("decision flag is boolean"),
            "{unchanged}"
        );
    }
}

#[test]
fn selected_headless_files_keep_the_event_and_cli_contract_at_every_hop() {
    assert_eq!(
        sha256_hex(PROTOCOL),
        "3032ea2edb66c992b1ab04e4f09ceaed3f06ed89b5028dcb1d0ce7f8e6a1ba61"
    );
    let protocol = json(PROTOCOL);
    assert_eq!(protocol["route"], "cursor-agent.headless");
    assert_eq!(
        protocol["wire"]["framing"],
        "JSON event per line, each serialized event terminated by LF."
    );
    assert!(protocol["wire"]["system_init"].is_string());
    assert!(protocol["wire"]["assistant"].is_string());
    assert!(protocol["wire"]["thinking"].is_string());
    assert!(protocol["wire"]["tool_call"].is_string());
    assert!(protocol["wire"]["result"].is_string());
    assert!(protocol["wire"]["usage"].is_string());
    assert_eq!(
        protocol["static_review"]["versions"],
        serde_json::json!([
            "2026.09.18-9a7762b",
            "2026.09.26-dd393fe",
            "2026.09.28-64d2043",
            "2026.10.01-14929f9"
        ])
    );
    assert!(
        protocol["static_review"]["selected_wire_shape_unchanged"]
            .as_bool()
            .expect("selected wire decision is boolean")
    );
    assert!(
        protocol["static_review"]["usage_mapping_unchanged"]
            .as_bool()
            .expect("usage decision is boolean")
    );
    assert!(
        protocol["static_review"]["only_headless_selected_surface_qualified"]
            .as_bool()
            .expect("route scope decision is boolean")
    );
    assert!(
        !protocol["static_review"]["catalogue_or_acp_evidence_transferred"]
            .as_bool()
            .expect("evidence transfer decision is boolean")
    );

    let inventory = json(DIST_INVENTORY);
    let identity = json(IDENTITY);
    let transitions = identity["selected_file_hop_classification"]
        .as_array()
        .expect("headless hop classifications are an array");
    assert_eq!(transitions.len(), 6);
    for transition in transitions {
        let platform = transition["platform"].as_str().expect("platform is text");
        let key = transition["complete_package_delta_reference"]["transition_key"]
            .as_str()
            .expect("transition key is text");
        let delta = &inventory["hop_deltas"][platform][key];
        assert_eq!(transition["runtime_index"]["category"], "changed");
        assert!(
            delta["changed"]["paths"]
                .as_array()
                .expect("changed paths are an array")
                .iter()
                .any(|path| path == "dist-package/index.js")
        );
        for category in ["added", "removed", "changed", "identical"] {
            assert_eq!(
                transition["complete_package_delta_reference"]["counts"][category],
                delta[category]["count"]
            );
            assert_eq!(
                transition["complete_package_delta_reference"]["path_set_sha256"][category],
                delta[category]["path_set_sha256"]
            );
        }

        let old_path = transition["headless_writer_chunk"]["from"]["path"]
            .as_str()
            .expect("old writer path is text");
        let new_path = transition["headless_writer_chunk"]["to"]["path"]
            .as_str()
            .expect("new writer path is text");
        match transition["headless_writer_chunk"]["category"]
            .as_str()
            .expect("writer classification is text")
        {
            "changed" => assert!(
                delta["changed"]["paths"]
                    .as_array()
                    .expect("changed paths are an array")
                    .iter()
                    .any(|path| path == old_path && old_path == new_path)
            ),
            "removed-and-added" => {
                assert!(
                    delta["removed"]["paths"]
                        .as_array()
                        .expect("removed paths are an array")
                        .iter()
                        .any(|path| path == old_path)
                );
                assert!(
                    delta["added"]["paths"]
                        .as_array()
                        .expect("added paths are an array")
                        .iter()
                        .any(|path| path == new_path)
                );
            }
            other => panic!("unexpected headless writer classification {other}"),
        }
    }
}
