//! Identity stop for official Mistral Vibe `3.0.2`.
//!
//! The frozen corpus records every published stable hop after the `2.26.0`
//! ceiling. The production claim stays on that ceiling.

use serde_json::Value;
use swallowtail_adapter_mistral_vibe::{MISTRAL_VIBE_RELEASE_VERSION, mistral_vibe_headless_claim};
use swallowtail_core::InterfaceVersion;

const IDENTITY: &str = include_str!("fixtures/mistral-vibe-headless-3.0.2/identity.json");
const LEDGER: &str = include_str!("fixtures/mistral-vibe-headless-3.0.2/surface-ledger.json");

const LAUNCHER_2_26: &str = "db6a00e04258fd74c25665dbe959f84325e6cb06bbb808f18525d68aab295ccd";
const LAUNCHER_3_0: &str = "8903c0fc8749ae8b4724a016ab6e2d258446b389915b8a62de96ef07bb21fac7";
const VIBE_HOME_SHA: &str = "df7191d3ea02bf2e7db1de8263c0f17016482338705622f07ef4384b35f423df";

fn json(source: &str) -> Value {
    serde_json::from_str(source).expect("frozen identity fixture is valid JSON")
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("version")
}

#[test]
fn official_3_0_2_is_an_identity_stop_and_the_claim_stays_at_2_26_0() {
    let identity = json(IDENTITY);
    let ledger = json(LEDGER);
    assert_eq!(
        identity["schema"],
        "swallowtail.mistral-vibe.headless.identity-stop.v1"
    );
    assert_eq!(identity["axis"], "mistral-vibe.release");
    assert_eq!(identity["route"], "mistral-vibe.headless");
    assert_eq!(identity["observed_at"], "2026-10-26");
    assert_eq!(identity["official"]["version"], "3.0.2");
    assert_eq!(identity["official"]["pypi_latest_stable"], "3.0.2");
    assert_eq!(
        identity["official"]["github_latest_stable_release"],
        "v3.0.2"
    );
    assert_eq!(identity["official"]["channels_agree"], true);
    assert_eq!(identity["host"]["vibe_command_found"], false);
    assert_eq!(identity["host"]["artifact_execution"], false);
    assert_eq!(identity["identity_decision"]["shape"], "identity-stop");
    assert_eq!(
        identity["identity_decision"]["latest_qualified_unchanged"],
        "2.26.0"
    );
    assert_eq!(
        identity["identity_decision"]["first_selected_break"],
        "2.26.1"
    );
    assert_eq!(identity["identity_decision"]["major_line_reset"], "3.0.0");
    assert_eq!(identity["identity_decision"]["extend_ceiling"], false);
    assert_eq!(identity["identity_decision"]["claim_id_change"], "none");
    assert_eq!(
        identity["identity_decision"]["behavior_revision_change"],
        "none"
    );
    assert_eq!(identity["identity_decision"]["config_home_unchanged"], true);
    assert_eq!(
        identity["identity_decision"]["downloaded_artifact_executed"],
        false
    );
    assert_eq!(identity["identity_decision"]["provider_prompt_sent"], false);
    assert_eq!(
        identity["provenance"]["selected_sdist_files_match_github_tag"],
        true
    );
    assert_eq!(ledger["ceiling"], "2.26.0");
    assert_eq!(ledger["official"], "3.0.2");
    assert_eq!(ledger["hops"].as_array().expect("hops").len(), 4);
    assert_eq!(ledger["hops"][0]["kind"], "turn-limit-failure");
    assert_eq!(ledger["hops"][0]["selected_break"], true);
    assert_eq!(ledger["hops"][1]["kind"], "major-line-reset");
    assert_eq!(ledger["hops"][1]["selected_break"], true);
    assert_eq!(ledger["hops"][2]["selected_break"], false);
    assert_eq!(ledger["hops"][3]["selected_break"], false);

    assert_eq!(MISTRAL_VIBE_RELEASE_VERSION, "2.26.0");
    let claim = mistral_vibe_headless_claim();
    for qualified in ["2.25.4", "2.25.5", "2.25.7", "2.25.8", "2.26.0"] {
        let point = version(qualified);
        assert!(claim.assess(&point).is_permitted(), "{qualified}");
        assert!(claim.classify(&point).is_some(), "{qualified}");
    }
    for gap in ["2.25.6", "2.25.9"] {
        let point = version(gap);
        assert!(!claim.assess(&point).is_permitted(), "{gap}");
        assert!(claim.classify(&point).is_none(), "{gap}");
    }
    for unverified in ["2.26.1", "3.0.0", "3.0.1", "3.0.2"] {
        let point = version(unverified);
        assert!(claim.assess(&point).is_permitted(), "{unverified}");
        assert!(claim.classify(&point).is_none(), "{unverified}");
    }

    let rows = identity["versions"].as_array().expect("versions");
    assert_eq!(
        rows.iter()
            .map(|row| row["version"].as_str().expect("version"))
            .collect::<Vec<_>>(),
        ["2.26.0", "2.26.1", "3.0.0", "3.0.1", "3.0.2"]
    );
    for row in rows {
        let version_name = row["version"].as_str().expect("version");
        assert_eq!(
            row["selected_files"]["vibe/utils/vibe_home.py"]["sdist_sha256"], VIBE_HOME_SHA,
            "{version_name}"
        );
        assert_eq!(
            row["config_home"]["default_is_dot_vibe"], true,
            "{version_name}"
        );
        assert_eq!(
            row["config_home"]["honors_vibe_home_env"], true,
            "{version_name}"
        );
        assert_eq!(
            row["rust_cli"]["has_legacy_harness_flag"], false,
            "{version_name}"
        );
        assert_eq!(
            row["rust_cli"]["has_output_streaming_variant"], true,
            "{version_name}"
        );
        assert_eq!(row["rust_cli"]["has_max_turns"], true, "{version_name}");
        assert_eq!(row["rust_cli"]["has_agent_flag"], true, "{version_name}");
        assert_eq!(
            row["linux_x86_64_wheel"]["vibe_rs_phrase_counts"]["--legacy-harness"], 0,
            "{version_name}"
        );
        let verified = row["pypi_wheels"]
            .as_array()
            .expect("wheels")
            .iter()
            .find(|wheel| wheel["locally_verified"] == true)
            .expect("linux wheel");
        assert_eq!(verified["sha256"], row["linux_x86_64_wheel"]["sha256"]);
        assert!(
            verified["filename"]
                .as_str()
                .expect("name")
                .contains("manylinux_2_28_x86_64")
        );
        for (path, file) in row["selected_files"].as_object().expect("files") {
            if file["present_in_sdist"] == true {
                assert_eq!(file["matches_github_tag"], true, "{version_name} {path}");
            }
        }
        match version_name {
            "2.26.0" => {
                assert_eq!(row["launcher_source"]["sha256"], LAUNCHER_2_26);
                assert_eq!(row["launcher_source"]["always_execs_rust"], false);
                assert_eq!(row["launcher_source"]["mentions_vibe_cli"], true);
                assert_eq!(row["plan_agent"]["builtin_plan_member"], true);
                assert_eq!(row["plan_agent"]["removed_builtin_maps_plan_to_ask"], false);
                assert_eq!(row["config_home"]["plans_dir_constant"], true);
                assert_eq!(row["turn_limit"]["stderr_phrase_in_programmatic"], true);
                assert_eq!(row["turn_limit"]["programmatic_limit_error_exits_1"], true);
                assert_eq!(row["turn_limit"]["run_outcome_turn_limit_exit_3"], false);
                assert_eq!(
                    row["turn_limit"]["python_entrypoint_has_legacy_harness"],
                    true
                );
                assert_eq!(
                    row["linux_x86_64_wheel"]["python_limit_phrase_files"][0],
                    "vibe/cli/programmatic.py"
                );
                assert_eq!(
                    row["pypi_sdist"]["sha256"],
                    "86ee13da13f9ca6b2f023cc5ca254a81bd99eeb5d2caa7b9e791421531bb2f5c"
                );
            }
            "2.26.1" => {
                assert_eq!(row["launcher_source"]["sha256"], LAUNCHER_2_26);
                assert_eq!(row["launcher_source"]["always_execs_rust"], false);
                assert_eq!(row["plan_agent"]["builtin_plan_member"], true);
                assert_eq!(row["config_home"]["plans_dir_constant"], true);
                assert_eq!(row["turn_limit"]["stderr_phrase_in_programmatic"], false);
                assert_eq!(row["turn_limit"]["programmatic_limit_error_exits_1"], false);
                assert_eq!(row["turn_limit"]["run_outcome_turn_limit_exit_3"], true);
                assert_eq!(
                    row["turn_limit"]["python_entrypoint_has_legacy_harness"],
                    true
                );
                assert_eq!(row["linux_x86_64_wheel"]["programmatic_present"], true);
                assert!(
                    row["linux_x86_64_wheel"]["python_limit_phrase_files"]
                        .as_array()
                        .expect("limit files")
                        .is_empty()
                );
                assert_eq!(
                    row["linux_x86_64_wheel"]["python_legacy_harness_files"]
                        .as_array()
                        .expect("legacy files")
                        .len(),
                    8
                );
                assert_eq!(
                    row["pypi_sdist"]["sha256"],
                    "e92402b2a1705c66463cad9d1f33d549ea2d8fc1bb608ec228c7acaf49ef4332"
                );
                assert_eq!(
                    row["github_tag_commit"],
                    "4ae5c5955da8e761318916bdb1a7f46f0b50c7d0"
                );
            }
            "3.0.0" | "3.0.1" | "3.0.2" => {
                assert_eq!(row["launcher_source"]["sha256"], LAUNCHER_3_0);
                assert_eq!(row["launcher_source"]["always_execs_rust"], true);
                assert_eq!(row["launcher_source"]["mentions_vibe_cli"], false);
                assert_eq!(row["launcher_source"]["can_reach_python_entrypoint"], false);
                assert_eq!(row["plan_agent"]["builtin_plan_member"], false);
                assert_eq!(row["plan_agent"]["removed_builtin_maps_plan_to_ask"], true);
                assert_eq!(row["config_home"]["plans_dir_constant"], false);
                assert_eq!(row["turn_limit"]["programmatic_py_present"], false);
                assert_eq!(row["turn_limit"]["stderr_phrase_in_programmatic"], false);
                assert_eq!(
                    row["turn_limit"]["python_entrypoint_has_legacy_harness"],
                    false
                );
                assert_eq!(row["linux_x86_64_wheel"]["programmatic_present"], false);
                assert!(
                    row["linux_x86_64_wheel"]["python_legacy_harness_files"]
                        .as_array()
                        .expect("legacy files")
                        .is_empty()
                );
                assert_eq!(
                    row["linux_x86_64_wheel"]["vibe_rs_phrase_counts"]["VIBE_CLI"],
                    0
                );
                assert_eq!(
                    row["linux_x86_64_wheel"]["vibe_rs_phrase_counts"]["The configured conversation limit was reached"],
                    0
                );
            }
            other => panic!("unexpected hop {other}"),
        }
    }
    let latest = rows.last().expect("3.0.2");
    assert_eq!(latest["version"], "3.0.2");
    assert_eq!(
        latest["github_tag_commit"],
        "36f7b3a4e10440e90f98080492530b8f4d46960d"
    );
    assert_eq!(
        latest["pypi_sdist"]["sha256"],
        "cb62be5743194f84a63d3e955afeeedd3e3741442fd19b118ec43ec532ddacd3"
    );
    assert_eq!(latest["requires_python"], ">=3.12");
    assert_eq!(rows[2]["requires_python"], ">=3.14");
    assert_eq!(rows[2]["version"], "3.0.0");
}
