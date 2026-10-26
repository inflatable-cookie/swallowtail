//! Identity stop for official Mistral Vibe `3.0.2`.
//!
//! The frozen corpus records every published stable hop after the `2.26.0`
//! ceiling. The production claim stays on that ceiling. Downloaded upstream
//! artifacts were inspected statically and never executed.

use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use swallowtail_adapter_mistral_vibe::{MISTRAL_VIBE_RELEASE_VERSION, mistral_vibe_headless_claim};
use swallowtail_core::InterfaceVersion;

const IDENTITY: &str = include_str!("fixtures/mistral-vibe-headless-3.0.2/identity.json");
const LEDGER: &str = include_str!("fixtures/mistral-vibe-headless-3.0.2/surface-ledger.json");
const INVENTORY: &str = include_str!("fixtures/mistral-vibe-headless-3.0.2/dist-inventory.json");

const COMPARED: [&str; 5] = ["2.26.0", "2.26.1", "3.0.0", "3.0.1", "3.0.2"];
const LAUNCHER_2_26: &str = "db6a00e04258fd74c25665dbe959f84325e6cb06bbb808f18525d68aab295ccd";
const LAUNCHER_3_0: &str = "8903c0fc8749ae8b4724a016ab6e2d258446b389915b8a62de96ef07bb21fac7";
const VIBE_HOME_SHA: &str = "df7191d3ea02bf2e7db1de8263c0f17016482338705622f07ef4384b35f423df";
const AGENTS_2_26: &str = "28015a29ad37b759c67b89d75e1cd38985ef50b3351f546dc801184debd65b68";
const AGENTS_3_0: &str = "3e240ce910b557c7553be16e4d86031023ce5d94bc9ad601dd9953dab6eaf063";
const CLI_RS_2_26: &str = "00138109403583a7d426de4d959240a2e24c199c672f64d16a87790771b54e68";
const HEADLESS_2_26: &str = "b2fae96dfc27d5f3e8a597e9ed3b35b8c286883ef51a440667dcb89ca32f6e17";
const HEADLESS_3_0_0: &str = "7b5b1d3675b9a328d7248de76195f8fc9a86ba1a92753d55e1d8c88f280d39d1";
const HEADLESS_3_0_1: &str = "989951ea8eac7ac5c9949061547f8650faa0858430169b20221b28d977f8f7aa";

const SDISTS: [(&str, &str, u64, usize, &str, u64, &str); 5] = [
    (
        "2.26.0",
        "86ee13da13f9ca6b2f023cc5ca254a81bd99eeb5d2caa7b9e791421531bb2f5c",
        2_993_316,
        1480,
        "371d9cdd99dc6b8e8eebd6af8890472c090efa6db0bea24d1f76e488c3eddd39",
        616,
        "5d7cb2b07593c2cac7f6d28068f9c9c29c7eac08e717e0c9b54a7beb7019e6df",
    ),
    (
        "2.26.1",
        "e92402b2a1705c66463cad9d1f33d549ea2d8fc1bb608ec228c7acaf49ef4332",
        3_194_290,
        1583,
        "09063384bee6abdd9fe8701e7000a7967302db2956580f876eec0db42b0d6c3a",
        633,
        "f3d7491f48977a306dca2565c5e31b3e04c480eb63393d511e7c6bbbbc038084",
    ),
    (
        "3.0.0",
        "49d90b1bce61ae2a2835e19bb24237958fa1323f9a2f12ce1968b9333bb3fee7",
        3_003_149,
        1533,
        "bbb99a6e1792613da6c75a76e07857cb1d1caa03cf631fbdd73ab504d25faffb",
        475,
        "614c66e07c651223513a84aa79581fdf2b2e3b519c721a4596f2eaa647402304",
    ),
    (
        "3.0.1",
        "b4d1ab55d6ccfc65c658fb19b09b46734e3bf14fd70ca90a5b227a5c640c5882",
        3_166_935,
        1596,
        "11ed8bae67c56ac43588743a8d645d81afe87947786689b77fc0d49fff7efc08",
        473,
        "df4339e72dd03f50b1d0abd40e65f1762fe19df119a4e58dfbdfbd7c1369276e",
    ),
    (
        "3.0.2",
        "cb62be5743194f84a63d3e955afeeedd3e3741442fd19b118ec43ec532ddacd3",
        3_220_310,
        1617,
        "d70dd7cc378d90f716a6c4e982b5cf721ac0eec08bbc1ae324a7a4b178723a5d",
        480,
        "384b49e9f0955f1aca4389a26ed944b3a586ae3fb2efb0da01272ddf462bb1c1",
    ),
];

const LINUX_WHEELS: [(&str, &str, u64, usize, &str, u64, &str); 5] = [
    (
        "2.26.0",
        "d8230b8abd7c39a8e90d60884b1f5ee8c95ffb459c62c21f3ea0e3041117a114",
        66_340_569,
        755,
        "9976e9a749dbb8208c2a09b3d10226019ae3196afb4895136869c83fe130d26c",
        616,
        "e95c75ea7505321bc9c34252f9705e0428fb2adf795a179c21c482c7dae25bfe",
    ),
    (
        "2.26.1",
        "1ceb43e38a2e82cb8c54e1032b011874f8422e998cf87b4a3fdf58704133de55",
        67_124_844,
        779,
        "2f7ad4fc868d3905272e9edaa80f2fac9688d196f3291d27e90d110b795d2a3e",
        633,
        "64aa7c228d1e6a9bb1c235debc978b63924f62ecee19c069f77cf0b502c47664",
    ),
    (
        "3.0.0",
        "b45de79af6f496257583c8817a785bbc8578955331b69c4c36d4b80990ca8a2b",
        67_917_476,
        614,
        "124e5f9cfe21a8f96641644d6a536d52ed894f23941747e4e38ab8559bf8b2e7",
        475,
        "bb7f67fdc6d61f621549db31c736a5e4e10a559830c8c5bcd653fc7105c4c9b0",
    ),
    (
        "3.0.1",
        "3016ea464719ac99a62f6c5ff4ffa013f3ac0c5ddf03c3def486133ebdc7094f",
        69_485_024,
        615,
        "8d79eb17d1aa7f243e782be70bd68636810ba82eb5d9b0afecdbfe7951425d05",
        473,
        "e61c43c8ec0f539a5945ad982780263d41c01857ec91a584a62d04040da6b9f7",
    ),
    (
        "3.0.2",
        "9dae259db0651a3978653bed1a87005ded40532942f3c143e2d8228fce0f7563",
        69_553_575,
        622,
        "7a059a1d877b620313b260d67d1b05d54d8aade073b79900f5e524b329bcf0a5",
        480,
        "250dd755b535f874f35b248a9be3c218e1161de5e7fc8a2fb345151a062c400e",
    ),
];

const VIBE_RS: [(&str, &str, u64); 5] = [
    (
        "2.26.0",
        "a70f1a340d1692cd9190ad84f0a5d6507eff09b3082b933d4b1c057667ecf7cc",
        148_888_208,
    ),
    (
        "2.26.1",
        "18b0620bec1dd9b1e3917258c7ecf54b831d58e3e58e9d6526195e8eb1b2bddc",
        151_765_032,
    ),
    (
        "3.0.0",
        "e9c202c64fb933daabfe23f6f5d5a89924d06e71144724a4fb6ad20c759e5c1a",
        156_163_552,
    ),
    (
        "3.0.1",
        "21c59a93184a406bd71a90ef8386c84566729699ff9d3a818d087ce657b82eca",
        162_319_864,
    ),
    (
        "3.0.2",
        "fed77ae5e39ee741d0ee7ac03d847698ee3ff1bc4150f79547fa1dd16844537e",
        162_419_000,
    ),
];

const GITHUB_COMMITS: [(&str, &str); 5] = [
    ("2.26.0", "376f6a33413a3eec9b3795b0c0e004066c47b5c1"),
    ("2.26.1", "4ae5c5955da8e761318916bdb1a7f46f0b50c7d0"),
    ("3.0.0", "a0c78d0a3f3f822c275f1dec7054e40d1050f966"),
    ("3.0.1", "dd2c04ce717c417731ab89c333cd0a72541e33ba"),
    ("3.0.2", "36f7b3a4e10440e90f98080492530b8f4d46960d"),
];

const HOP_COUNTS: [(&str, usize, usize, usize, usize, usize, usize); 4] = [
    ("2.26.0..2.26.1", 106, 3, 366, 30, 6, 108),
    ("2.26.1..3.0.0", 152, 202, 721, 39, 204, 216),
    ("3.0.0..3.0.1", 166, 103, 381, 17, 16, 191),
    ("3.0.1..3.0.2", 21, 0, 141, 13, 6, 49),
];

fn json(source: &str) -> Value {
    serde_json::from_str(source).expect("frozen identity fixture is valid JSON")
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("version")
}

fn sha256(text: &str) -> String {
    format!("{:x}", Sha256::digest(text.as_bytes()))
}

fn tree_manifest(entries: &[Value]) -> String {
    let body = entries
        .iter()
        .map(|row| {
            format!(
                "{}\0{}\0{}\n",
                row["path"].as_str().expect("path"),
                row["sha256"].as_str().expect("digest"),
                row["size"].as_u64().expect("size")
            )
        })
        .collect::<String>();
    sha256(&body)
}

fn python_manifest(entries: &[Value]) -> (String, usize) {
    let rows = entries
        .iter()
        .filter(|row| {
            row["path"]
                .as_str()
                .is_some_and(|path| path.starts_with("vibe/") && path.ends_with(".py"))
        })
        .cloned()
        .collect::<Vec<_>>();
    (tree_manifest(&rows), rows.len())
}

fn paths(value: &Value) -> Vec<&str> {
    value
        .as_array()
        .expect("path list")
        .iter()
        .map(|entry| entry.as_str().expect("path"))
        .collect()
}

fn hop_delta(
    from_entries: &[Value],
    to_entries: &[Value],
) -> (Vec<String>, Vec<String>, Vec<String>) {
    let from_map = from_entries
        .iter()
        .map(|row| (row["path"].as_str().expect("path"), row))
        .collect::<BTreeMap<_, _>>();
    let to_map = to_entries
        .iter()
        .map(|row| (row["path"].as_str().expect("path"), row))
        .collect::<BTreeMap<_, _>>();
    let added = to_map
        .keys()
        .filter(|path| !from_map.contains_key(*path))
        .map(|path| (*path).to_owned())
        .collect::<Vec<_>>();
    let removed = from_map
        .keys()
        .filter(|path| !to_map.contains_key(*path))
        .map(|path| (*path).to_owned())
        .collect::<Vec<_>>();
    let changed = from_map
        .keys()
        .filter(|path| to_map.contains_key(*path))
        .filter(|path| {
            from_map[*path]["sha256"] != to_map[*path]["sha256"]
                || from_map[*path]["size"] != to_map[*path]["size"]
        })
        .map(|path| (*path).to_owned())
        .collect::<Vec<_>>();
    (added, removed, changed)
}

fn assert_sorted_unique_entries(entries: &[Value], label: &str) {
    let mut prior = None;
    let mut seen = BTreeSet::new();
    for entry in entries {
        let path = entry["path"].as_str().expect("archive path");
        assert!(seen.insert(path.to_owned()), "duplicate {label}/{path}");
        if let Some(previous) = prior {
            assert!(previous < path, "unsorted {label}/{path}");
        }
        prior = Some(path);
        assert_eq!(
            entry["sha256"].as_str().expect("digest").len(),
            64,
            "{label}/{path}"
        );
    }
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
    assert_eq!(identity["tree_inventory"], "dist-inventory.json");
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
    assert!(
        ledger["hops"][3]["classification"]
            .as_str()
            .expect("3.0.2 hop")
            .contains("Cli::prompt_given")
    );

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
        COMPARED
    );
    for ((((row, github), sdist), wheel), vibe_rs) in rows
        .iter()
        .zip(GITHUB_COMMITS)
        .zip(SDISTS)
        .zip(LINUX_WHEELS)
        .zip(VIBE_RS)
    {
        let version_name = row["version"].as_str().expect("version");
        assert_eq!(version_name, github.0);
        assert_eq!(version_name, sdist.0);
        assert_eq!(row["github_tag_commit"], github.1, "{version_name}");
        assert_eq!(row["pypi_sdist"]["sha256"], sdist.1, "{version_name}");
        assert_eq!(row["pypi_sdist"]["size"], sdist.2, "{version_name}");
        assert_eq!(row["pypi_sdist"]["file_count"], sdist.3, "{version_name}");
        assert_eq!(
            row["linux_x86_64_wheel"]["sha256"], wheel.1,
            "{version_name}"
        );
        assert_eq!(row["linux_x86_64_wheel"]["size"], wheel.2, "{version_name}");
        assert_eq!(
            row["linux_x86_64_wheel"]["file_count"], wheel.3,
            "{version_name}"
        );
        assert_eq!(
            row["linux_x86_64_wheel"]["vibe_rs_sha256"], vibe_rs.1,
            "{version_name}"
        );
        assert_eq!(
            row["linux_x86_64_wheel"]["vibe_rs_size"], vibe_rs.2,
            "{version_name}"
        );
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
        assert_eq!(
            row["linux_x86_64_wheel"]["vibe_rs_phrase_counts"]["streaming"], 13,
            "{version_name}"
        );
        assert_eq!(
            row["linux_x86_64_wheel"]["vibe_rs_phrase_counts"]["Agent '"], 0,
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
                assert_eq!(row["requires_python"], ">=3.12");
                assert_eq!(
                    row["selected_files"]["vibe/cli/programmatic.py"]["sdist_sha256"],
                    "1dbfd951bba336721cd31aee97a672d219933144e7cde357b485d01908a0a214"
                );
                assert_eq!(
                    row["selected_files"]["vibe/cli/cli.py"]["sdist_sha256"],
                    "6e186997b0e6003486f10cefd38b01e5dc8e5178b7f996b4d7064f71d740d20a"
                );
                assert_eq!(
                    row["selected_files"]["vibe/cli/entrypoint.py"]["sdist_sha256"],
                    "f343aaa95389528037512ae13ba57f005a60bb202d98136dfbe165ef98cc7331"
                );
                assert_eq!(
                    row["selected_files"]["vibe/core/agents/models.py"]["sdist_sha256"],
                    AGENTS_2_26
                );
                assert_eq!(
                    row["selected_files"]["vibe/cli-rust/src/cli.rs"]["sdist_sha256"],
                    CLI_RS_2_26
                );
                assert_eq!(
                    row["selected_files"]["vibe/cli-rust/src/headless_output.rs"]["sdist_sha256"],
                    HEADLESS_2_26
                );
                assert_eq!(
                    row["linux_x86_64_wheel"]["vibe_rs_phrase_counts"]["The configured conversation limit was reached"],
                    1
                );
                assert_eq!(
                    row["linux_x86_64_wheel"]["vibe_rs_phrase_counts"]["VIBE_CLI"],
                    2
                );
                assert_eq!(
                    row["linux_x86_64_wheel"]["vibe_rs_phrase_counts"]["--agent"],
                    0
                );
                assert_eq!(
                    row["linux_x86_64_wheel"]["vibe_rs_phrase_counts"][".vibe"],
                    4
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
                assert_eq!(row["requires_python"], ">=3.12");
                assert_eq!(
                    row["selected_files"]["vibe/cli/programmatic.py"]["sdist_sha256"],
                    "a938d2440ff5599fa1c48f131c8d6d1ac336aec8cd496c1f8a7caa85c92bbbc1"
                );
                assert_eq!(
                    row["selected_files"]["vibe/cli/cli.py"]["sdist_sha256"],
                    "f53bd1000145ab6b033845321691b95f7eae7a4f03c3816561524cac588464e6"
                );
                assert_eq!(
                    row["selected_files"]["vibe/cli/entrypoint.py"]["sdist_sha256"],
                    "fbb033ec73433bc575211680308f287f4bc26a3857f1100f55988de79bad0bb1"
                );
                assert_eq!(
                    row["selected_files"]["vibe/cli-rust/src/cli.rs"]["sdist_sha256"],
                    CLI_RS_2_26
                );
                assert_eq!(
                    row["linux_x86_64_wheel"]["vibe_rs_phrase_counts"]["The configured conversation limit was reached"],
                    1
                );
                assert_eq!(
                    row["linux_x86_64_wheel"]["vibe_rs_phrase_counts"]["VIBE_CLI"],
                    2
                );
                assert_eq!(
                    row["linux_x86_64_wheel"]["vibe_rs_phrase_counts"][".vibe"],
                    5
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
                assert_eq!(
                    row["linux_x86_64_wheel"]["vibe_rs_phrase_counts"]["--agent"],
                    4
                );
                assert_eq!(
                    row["selected_files"]["vibe/core/agents/models.py"]["sdist_sha256"],
                    AGENTS_3_0
                );
                assert_eq!(
                    row["selected_files"]["vibe/cli/programmatic.py"]["present_in_sdist"],
                    false
                );
                match version_name {
                    "3.0.0" => {
                        assert_eq!(row["requires_python"], ">=3.14");
                        assert_eq!(
                            row["selected_files"]["vibe/cli-rust/src/cli.rs"]["sdist_sha256"],
                            "544c7ac52b4e84de068369874428276b745f8ca3923b7458fd7f09b03b26e86a"
                        );
                        assert_eq!(
                            row["selected_files"]["vibe/cli-rust/src/headless_output.rs"]["sdist_sha256"],
                            HEADLESS_3_0_0
                        );
                        assert_eq!(
                            row["linux_x86_64_wheel"]["vibe_rs_phrase_counts"][".vibe"],
                            3
                        );
                    }
                    "3.0.1" => {
                        assert_eq!(row["requires_python"], ">=3.12");
                        assert_eq!(
                            row["selected_files"]["vibe/cli-rust/src/cli.rs"]["sdist_sha256"],
                            "c6ab3ae2b5fe12348aaf08299c103f3e40506dad3d73538d7d5bd08cc34dce76"
                        );
                        assert_eq!(
                            row["selected_files"]["vibe/cli-rust/src/headless_output.rs"]["sdist_sha256"],
                            HEADLESS_3_0_1
                        );
                        assert_eq!(
                            row["linux_x86_64_wheel"]["vibe_rs_phrase_counts"][".vibe"],
                            5
                        );
                    }
                    "3.0.2" => {
                        assert_eq!(row["requires_python"], ">=3.12");
                        assert_eq!(
                            row["selected_files"]["vibe/cli-rust/src/cli.rs"]["sdist_sha256"],
                            "510b8fd6b663b934ffc52a5eb7c5a597cf70280908ac8002229f02b4bebfa6d6"
                        );
                        assert_eq!(
                            row["selected_files"]["vibe/cli-rust/src/headless_output.rs"]["sdist_sha256"],
                            HEADLESS_3_0_1
                        );
                        assert_eq!(
                            row["linux_x86_64_wheel"]["vibe_rs_phrase_counts"][".vibe"],
                            5
                        );
                    }
                    other => panic!("unexpected 3.0 hop {other}"),
                }
            }
            other => panic!("unexpected hop {other}"),
        }
    }
}

#[test]
fn complete_hop_inventory_freezes_sdist_and_linux_wheel_trees() {
    let identity = json(IDENTITY);
    let inventory = json(INVENTORY);
    assert_eq!(
        inventory["schema"],
        "swallowtail.mistral-vibe.complete-hop-inventory.v1"
    );
    assert_eq!(inventory["retrieved"], "2026-10-26");
    assert_eq!(inventory["sdist_count"], 5);
    assert_eq!(inventory["linux_x86_64_wheel_count"], 5);
    assert_eq!(
        inventory["compared"]
            .as_array()
            .expect("compared")
            .iter()
            .map(|value| value.as_str().expect("version"))
            .collect::<Vec<_>>(),
        COMPARED
    );

    let sdists = inventory["sdists"].as_array().expect("sdists");
    let wheels = inventory["linux_x86_64_wheels"].as_array().expect("wheels");
    assert_eq!(sdists.len(), SDISTS.len());
    assert_eq!(wheels.len(), LINUX_WHEELS.len());

    for (sdist, (version_name, sha, size, count, manifest, python_count, python_manifest_sha)) in
        sdists.iter().zip(SDISTS)
    {
        assert_eq!(sdist["version"], version_name);
        assert_eq!(
            sdist["filename"],
            format!("mistral_vibe-{version_name}.tar.gz")
        );
        assert_eq!(sdist["sha256"], sha);
        assert_eq!(sdist["size"], size);
        assert_eq!(sdist["entry_count"], count);
        assert_eq!(
            inventory["package_file_counts"]["sdist"][version_name],
            count
        );
        let entries = sdist["entries"].as_array().expect("sdist entries");
        assert_eq!(entries.len(), count, "sdist files {version_name}");
        assert_sorted_unique_entries(entries, &format!("sdist {version_name}"));
        assert_eq!(
            tree_manifest(entries),
            manifest,
            "sdist tree {version_name}"
        );
        assert_eq!(sdist["tree_manifest_sha256"], manifest);
        let (computed_python, computed_python_count) = python_manifest(entries);
        assert_eq!(computed_python_count as u64, python_count);
        assert_eq!(sdist["vibe_python_entry_count"], python_count);
        assert_eq!(
            computed_python, python_manifest_sha,
            "sdist python {version_name}"
        );
        assert_eq!(sdist["vibe_python_manifest_sha256"], python_manifest_sha);
    }

    for (wheel, (version_name, sha, size, count, manifest, python_count, python_manifest_sha)) in
        wheels.iter().zip(LINUX_WHEELS)
    {
        assert_eq!(wheel["version"], version_name);
        assert_eq!(
            wheel["filename"],
            format!("mistral_vibe-{version_name}-cp312-abi3-manylinux_2_28_x86_64.whl")
        );
        assert_eq!(wheel["sha256"], sha);
        assert_eq!(wheel["size"], size);
        assert_eq!(wheel["entry_count"], count);
        assert_eq!(
            inventory["package_file_counts"]["linux_x86_64_wheel"][version_name],
            count
        );
        let entries = wheel["entries"].as_array().expect("wheel entries");
        assert_eq!(entries.len(), count, "wheel files {version_name}");
        assert_sorted_unique_entries(entries, &format!("wheel {version_name}"));
        assert_eq!(
            tree_manifest(entries),
            manifest,
            "wheel tree {version_name}"
        );
        assert_eq!(wheel["tree_manifest_sha256"], manifest);
        let (computed_python, computed_python_count) = python_manifest(entries);
        assert_eq!(computed_python_count as u64, python_count);
        assert_eq!(wheel["vibe_python_entry_count"], python_count);
        assert_eq!(
            computed_python, python_manifest_sha,
            "wheel python {version_name}"
        );
        assert_eq!(wheel["vibe_python_manifest_sha256"], python_manifest_sha);
        let vibe_rs = entries
            .iter()
            .find(|entry| entry["path"] == "vibe/_bin/vibe-rs")
            .expect("shipped vibe-rs");
        let expected_rs = VIBE_RS
            .iter()
            .find(|(name, _, _)| *name == version_name)
            .expect("vibe-rs row");
        assert_eq!(vibe_rs["sha256"], expected_rs.1, "vibe-rs {version_name}");
        assert_eq!(vibe_rs["size"], expected_rs.2, "vibe-rs {version_name}");
    }

    let identity_rows = identity["versions"].as_array().expect("versions");
    for row in identity_rows {
        let version_name = row["version"].as_str().expect("version");
        let hashes = &inventory["hashes"];
        assert_eq!(
            hashes["vibe/cli/launcher.py"][version_name]["sdist_sha256"],
            row["selected_files"]["vibe/cli/launcher.py"]["sdist_sha256"],
            "{version_name} launcher"
        );
        assert_eq!(
            hashes["vibe/utils/vibe_home.py"][version_name]["sdist_sha256"], VIBE_HOME_SHA,
            "{version_name} vibe_home"
        );
        if row["selected_files"]["vibe/cli/programmatic.py"]["present_in_sdist"] == true {
            assert_eq!(
                hashes["vibe/cli/programmatic.py"][version_name]["sdist_sha256"],
                row["selected_files"]["vibe/cli/programmatic.py"]["sdist_sha256"],
                "{version_name} programmatic"
            );
        } else {
            assert!(
                hashes["vibe/cli/programmatic.py"]
                    .get(version_name)
                    .is_none()
            );
        }
        assert_eq!(
            hashes["vibe/_bin/vibe-rs"][version_name]["wheel_sha256"],
            row["linux_x86_64_wheel"]["vibe_rs_sha256"],
            "{version_name} vibe-rs inventory"
        );
        assert_eq!(
            hashes["vibe/cli-rust/src/cli.rs"][version_name]["present_in_linux_wheel"], false,
            "{version_name} cli.rs stays sdist-only"
        );
        assert_eq!(
            hashes["vibe/cli-rust/src/main.rs"][version_name]["present_in_sdist"], true,
            "{version_name} main.rs"
        );
    }
    assert_eq!(
        inventory["hashes"]["vibe/cli-rust/src/main.rs"]["3.0.1"]["sdist_sha256"],
        "ed51866052dcb3510f87b4bdd8b206af0ab440fa7d54b06836c43d0d209d767c"
    );
    assert_eq!(
        inventory["hashes"]["vibe/cli-rust/src/main.rs"]["3.0.2"]["sdist_sha256"],
        "59d90ac899f4dfd59a62c0dc38d6738c4d61a6ea908246b62acd590f0918587f"
    );
    assert_eq!(
        inventory["hashes"]["vibe/cli-rust/src/cli.rs"]["3.0.1"]["sdist_sha256"],
        "c6ab3ae2b5fe12348aaf08299c103f3e40506dad3d73538d7d5bd08cc34dce76"
    );
    assert_eq!(
        inventory["hashes"]["vibe/cli-rust/src/cli.rs"]["3.0.2"]["sdist_sha256"],
        "510b8fd6b663b934ffc52a5eb7c5a597cf70280908ac8002229f02b4bebfa6d6"
    );
}

#[test]
fn hop_deltas_match_complete_trees_and_selected_breaks() {
    let inventory = json(INVENTORY);
    let hops = inventory["from_hop_to_hop"]
        .as_object()
        .expect("from_hop_to_hop");
    assert_eq!(hops.len(), HOP_COUNTS.len());
    let sdists = inventory["sdists"].as_array().expect("sdists");
    let wheels = inventory["linux_x86_64_wheels"].as_array().expect("wheels");
    let sdist_by_version = sdists
        .iter()
        .map(|row| (row["version"].as_str().expect("version"), row))
        .collect::<BTreeMap<_, _>>();
    let wheel_by_version = wheels
        .iter()
        .map(|row| (row["version"].as_str().expect("version"), row))
        .collect::<BTreeMap<_, _>>();

    for (
        name,
        sdist_added,
        sdist_removed,
        sdist_changed,
        wheel_added,
        wheel_removed,
        wheel_changed,
    ) in HOP_COUNTS
    {
        let (from_version, to_version) = name.split_once("..").expect("hop");
        let hop = &hops[name];
        let sdist_entries_from = sdist_by_version[from_version]["entries"]
            .as_array()
            .expect("from sdist");
        let sdist_entries_to = sdist_by_version[to_version]["entries"]
            .as_array()
            .expect("to sdist");
        let wheel_entries_from = wheel_by_version[from_version]["entries"]
            .as_array()
            .expect("from wheel");
        let wheel_entries_to = wheel_by_version[to_version]["entries"]
            .as_array()
            .expect("to wheel");
        let (computed_sdist_added, computed_sdist_removed, computed_sdist_changed) =
            hop_delta(sdist_entries_from, sdist_entries_to);
        let (computed_wheel_added, computed_wheel_removed, computed_wheel_changed) =
            hop_delta(wheel_entries_from, wheel_entries_to);
        assert_eq!(paths(&hop["sdist"]["added"]), computed_sdist_added);
        assert_eq!(paths(&hop["sdist"]["removed"]), computed_sdist_removed);
        assert_eq!(paths(&hop["sdist"]["changed"]), computed_sdist_changed);
        assert_eq!(
            paths(&hop["linux_x86_64_wheel"]["added"]),
            computed_wheel_added
        );
        assert_eq!(
            paths(&hop["linux_x86_64_wheel"]["removed"]),
            computed_wheel_removed
        );
        assert_eq!(
            paths(&hop["linux_x86_64_wheel"]["changed"]),
            computed_wheel_changed
        );
        assert_eq!(
            hop["sdist"]["added_count"], sdist_added,
            "{name} sdist added"
        );
        assert_eq!(
            hop["sdist"]["removed_count"], sdist_removed,
            "{name} sdist removed"
        );
        assert_eq!(
            hop["sdist"]["changed_count"], sdist_changed,
            "{name} sdist changed"
        );
        assert_eq!(
            hop["linux_x86_64_wheel"]["added_count"], wheel_added,
            "{name} wheel added"
        );
        assert_eq!(
            hop["linux_x86_64_wheel"]["removed_count"], wheel_removed,
            "{name} wheel removed"
        );
        assert_eq!(
            hop["linux_x86_64_wheel"]["changed_count"], wheel_changed,
            "{name} wheel changed"
        );
    }

    let first = &hops["2.26.0..2.26.1"];
    assert!(paths(&first["sdist"]["changed"]).contains(&"vibe/cli/programmatic.py"));
    assert!(paths(&first["linux_x86_64_wheel"]["changed"]).contains(&"vibe/cli/programmatic.py"));
    assert!(!paths(&first["sdist"]["changed"]).contains(&"vibe/cli/launcher.py"));

    let reset = &hops["2.26.1..3.0.0"];
    for path in [
        "vibe/cli/programmatic.py",
        "vibe/cli/cli.py",
        "vibe/cli/entrypoint.py",
    ] {
        assert!(
            paths(&reset["sdist"]["removed"]).contains(&path),
            "sdist removed {path}"
        );
        assert!(
            paths(&reset["linux_x86_64_wheel"]["removed"]).contains(&path),
            "wheel removed {path}"
        );
    }
    assert!(paths(&reset["sdist"]["changed"]).contains(&"vibe/cli/launcher.py"));
    assert!(paths(&reset["linux_x86_64_wheel"]["changed"]).contains(&"vibe/cli/launcher.py"));

    let persist = &hops["3.0.1..3.0.2"];
    assert!(paths(&persist["sdist"]["changed"]).contains(&"vibe/cli-rust/src/cli.rs"));
    assert!(paths(&persist["sdist"]["changed"]).contains(&"vibe/cli-rust/src/main.rs"));
    assert!(!paths(&persist["sdist"]["changed"]).contains(&"vibe/cli/launcher.py"));
    assert!(!paths(&persist["sdist"]["changed"]).contains(&"vibe/cli-rust/src/headless_output.rs"));
    assert!(!paths(&persist["linux_x86_64_wheel"]["changed"]).contains(&"vibe/cli/launcher.py"));
}
