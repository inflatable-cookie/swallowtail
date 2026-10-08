use serde_json::Value;
use swallowtail_adapter_copilot_cli::COPILOT_CLI_PACKAGE_VERSION;

const INVENTORY: &str =
    include_str!("fixtures/copilot-cli-acp-offline-proof/artifact-inventory.json");
const EXECUTION: &str =
    include_str!("fixtures/copilot-cli-acp-offline-proof/execution-record.json");

fn version_record<'a>(package: &'a Value, version: &str) -> &'a Value {
    package["versions"]
        .as_array()
        .expect("versions")
        .iter()
        .find(|record| record["version"] == version)
        .expect("pinned version")
}

fn package_record<'a>(inventory: &'a Value, name: &str) -> &'a Value {
    inventory["packages"]
        .as_array()
        .expect("packages")
        .iter()
        .find(|record| record["name"] == name)
        .expect("package identity")
}

#[test]
fn official_stable_artifacts_have_complete_frozen_inventories() {
    let inventory: Value = serde_json::from_str(INVENTORY).expect("artifact inventory JSON");
    assert_eq!(
        inventory["schema"],
        "copilot-cli-acp-offline-artifact-inventory.v1"
    );
    assert_eq!(inventory["platform"], "darwin-arm64");
    assert_eq!(
        inventory["stable_channel_cross_check"]["npm_latest"],
        "1.0.93"
    );
    assert_eq!(
        inventory["stable_channel_cross_check"]["native_latest"],
        "1.0.93"
    );
    assert_eq!(
        inventory["stable_channel_cross_check"]["github_latest_release"]["tag"],
        "v1.0.93"
    );
    assert_eq!(
        inventory["stable_channel_cross_check"]["npm_prerelease"],
        "1.0.94-4"
    );

    let wrapper = package_record(&inventory, "@github/copilot");
    let native = package_record(&inventory, "@github/copilot-darwin-arm64");
    for (package, expected_counts, expected_archive_hashes, expected_tree_hashes) in [
        (
            wrapper,
            [4, 4, 4],
            [
                "799457937f8f87de6fdc95599380de5f5a0f761ab2fdfbba7f8d1c82d2988892",
                "0b0205dbb634579e3edc876d2f6facc5fde67c041dedc4eb037ef6f531fda08d",
                "a8e704fb6874364af1b268aed2170bb597e0ca8086f3182b8fe5cb86ca3e43e1",
            ],
            [
                "ed5319d2137d30af224067bfadb163b9a2852de67201b767034c7989c2ccc42f",
                "d4d5fbf7f15dd91a158a5f0ef97f6fdb4338434852ae521eb61cc245cbb3f755",
                "5334a96e0e54b33578ede41a5f478654a4014bcfedf2bad54cb8988b97507b7e",
            ],
        ),
        (
            native,
            [241, 225, 4],
            [
                "98640ca0de6576807f369c533c839b5742b038f105a970bdd7cb0d7efc8a7a71",
                "be324ba249b5744cb07fb7cfc275d52a0694136bc476d0299a9391bb3c0aa299",
                "f254651a3195e125b91d723c800e71e6541f8db3832d269854ae982254263eeb",
            ],
            [
                "bce437715d370d3e7baf81351c3376c7051b17ae61085b7cf25af727ee336817",
                "8e46b7ff27be330301d24a7ff2dd1998a2fc5010206ebac6311a1f1f45ecf36e",
                "67666c08135a9cbe4e433fd45653018ed17e8131a932d8317ea77991d3732fd5",
            ],
        ),
    ] {
        for (index, version) in ["1.0.80", "1.0.81", "1.0.93"].iter().enumerate() {
            let record = version_record(package, version);
            assert_eq!(record["dist"]["fileCount"], expected_counts[index]);
            assert_eq!(record["archive_sha256"], expected_archive_hashes[index]);
            assert_eq!(record["inventory_sha256"], expected_tree_hashes[index]);
            assert_eq!(
                record["files"].as_array().expect("files").len(),
                expected_counts[index]
            );
            assert!(
                record["dist"]["integrity"]
                    .as_str()
                    .unwrap()
                    .starts_with("sha512-")
            );
            assert!(record["dist"]["signatures"].as_array().is_some());
        }
    }

    for (version, binary_hash) in [
        (
            "1.0.80",
            "fe779da7dd2342c1d23f0744873fa27d0251eaaee4dc6637fa53093639c0f3c9",
        ),
        (
            "1.0.81",
            "0f2ba6429dbee9f5adcdc2ad09ded7f5a0511f5da9af10c3a0dbc6ed070f004f",
        ),
        (
            "1.0.93",
            "df347f272793e735629a91eea0285a736aeeb38821dd2b056234f7f47b58aef1",
        ),
    ] {
        let record = version_record(native, version);
        let binary = record["files"]
            .as_array()
            .expect("native package files")
            .iter()
            .find(|file| file["path"] == "package/copilot")
            .expect("selected copilot binary");
        assert_eq!(binary["sha256"], binary_hash);
    }

    // Evidence preparation does not alter the existing exact 1.0.80 claim.
    assert_eq!(COPILOT_CLI_PACKAGE_VERSION, "1.0.80");
}

#[test]
fn execution_record_is_secret_free_and_keeps_the_permission_stop_explicit() {
    let record: Value = serde_json::from_str(EXECUTION).expect("execution record JSON");
    assert_eq!(
        record["schema"],
        "copilot-cli-acp-offline-execution-record.v1"
    );
    assert_eq!(record["pre_execution_record_persisted"], true);
    assert_eq!(record["artifact_execution_started"], true);
    assert_eq!(record["permission_boundary_proven_for_all_targets"], false);
    assert_eq!(record["exact_permission_path_reached"], false);
    assert_eq!(
        record["artifact_inventory_sha256_at_execution"],
        "2d122117ccbb52dd547a783117ea3b1699df8e15357bca86a4d27a65416c8b0f"
    );
    assert_eq!(
        record["artifact_inventory_sha256_committed"],
        "2d122117ccbb52dd547a783117ea3b1699df8e15357bca86a4d27a65416c8b0f"
    );
    assert_eq!(
        record["harness_source"],
        "scripts/copilot-acp-offline-proof.py"
    );
    assert_eq!(
        record["harness_sha256"],
        "65e8357d2942c8dd554b8973d35e065ed7a4e7a9596bf1c103298396338acb2d"
    );
    assert_eq!(record["scope"]["network"], "denied, including loopback");
    assert_eq!(
        record["limitation"],
        "All exact stable artifacts advertised copilot-login and rejected session/new with Authentication required under the synthetic placeholder. No real authentication or network access was used, so no permission request, cancellation, or tool-effect boundary is claimed."
    );
    assert_eq!(
        record["scope"]["authentication"],
        "synthetic placeholder only"
    );

    let preflight = &record["preflight"];
    assert_eq!(preflight["status"], "passed");
    assert_eq!(preflight["record_before_execution"], true);
    assert_eq!(preflight["fake_acp"]["permission_request"], "observed");
    assert_eq!(preflight["fake_acp"]["permission_reply"], "cancelled");
    assert_eq!(preflight["fake_acp"]["tool_effect"], "absent");

    let executions = record["executions"]
        .as_array()
        .expect("exact artifact runs");
    assert_eq!(executions.len(), 3);
    for (run, version) in executions.iter().zip(["1.0.80", "1.0.81", "1.0.93"]) {
        assert_eq!(run["version"], version);
        assert_eq!(run["recorded_before_process_start"], true);
        assert_eq!(
            run["selected_argv"],
            serde_json::json!(["copilot", "--acp", "--stdio"])
        );
        assert_eq!(run["network"], "sandbox-denied");
        assert_eq!(run["credentials"], "synthetic-placeholder-only");
        assert_eq!(run["permission_request_observed"], false);
        assert_eq!(run["permission_evidence_complete"], false);
        assert_eq!(run["effect_marker_present"], false);
    }
    for (run, version) in executions.iter().zip(["1.0.80", "1.0.81", "1.0.93"]) {
        assert_eq!(run["initialize"], "success");
        assert_eq!(run["reported_version_matches"], true);
        assert_eq!(run["auth_method_ids"], serde_json::json!(["copilot-login"]));
        assert_eq!(run["session_new"], "authentication-required", "{version}");
        assert_eq!(run["session_new_error"]["code"], -32000);
        assert_eq!(
            run["session_new_error"]["message"],
            "Authentication required"
        );
        assert_eq!(run["session_new_error"].get("data"), None);
        assert_eq!(run["session_new_error"]["data_omitted"], false);
        assert_eq!(run["session_prompt"], "not-reached");
        assert_eq!(run["timed_out"], false);
        assert_eq!(run["forced_process_group_kill"], false);
    }

    let serialized = record.to_string();
    for forbidden in ["/Users/", "github_pat_", "ghp_", "gho_", "ghs_", "xoxb-"] {
        assert!(
            !serialized.contains(forbidden),
            "record contains {forbidden}"
        );
    }
}
