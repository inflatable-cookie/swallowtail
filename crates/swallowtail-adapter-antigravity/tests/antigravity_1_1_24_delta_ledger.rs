use serde_json::Value;
use std::collections::BTreeSet;
use swallowtail_adapter_antigravity::{
    ANTIGRAVITY_BASELINE_VERSION, ANTIGRAVITY_LATEST_QUALIFIED_VERSION, ANTIGRAVITY_RELEASE_AXIS,
    antigravity_catalogue_claim, antigravity_headless_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const IDENTITY: &str = include_str!("fixtures/antigravity-cli-1.1.24/identity.json");
const PROTOCOL: &str = include_str!("fixtures/antigravity-cli-1.1.24/protocol.json");
const DIST_INVENTORY: &str = include_str!("fixtures/antigravity-cli-1.1.24/dist-inventory.json");

const COMPARED: &[&str] = &[
    "1.1.17", "1.1.18", "1.1.19", "1.1.20", "1.1.21", "1.1.22", "1.1.23", "1.1.24",
];
const HOPS: &[&str] = &[
    "from_1_1_17_to_1_1_18",
    "from_1_1_18_to_1_1_19",
    "from_1_1_19_to_1_1_20",
    "from_1_1_20_to_1_1_21",
    "from_1_1_21_to_1_1_22",
    "from_1_1_22_to_1_1_23",
    "from_1_1_23_to_1_1_24",
];
const SELECTED_FLAGS: &[&str] = &[
    "--print",
    "--output-format",
    "--model",
    "--mode",
    "--sandbox",
    "--effort",
    "--json-schema",
    "--conversation",
];
const UNMAPPED_KEYS: &[&str] = &[
    "--input-format",
    "mcp",
    "mic-serve",
    "voice",
    "--remote-control",
    "--project-name",
    "GEMINI_API_KEY",
    "enterprise-sign-in",
    "models --output-format",
    "slash-model-name",
    "skill-icons",
    "always-proceed-mcp",
    "embedded-ripgrep",
    "cost-status-line",
    "AGY_CLI_HIDE_LOGO",
    "AGY_CLI_DISABLE_ESCAPE_SEQUENCE_OPTIMIZATIONS",
    "tui-mcp-agents-btw-goal",
];
const LIFECYCLE_REPAIRS: &[&str] = &[
    "1.1.18-print-mode-dropped-stream-nonzero-exit",
    "1.1.20-print-mode-benign-tool-error-nonzero-exit",
    "1.1.23-models-agents-inherited-stdin-hang",
    "1.1.24-headless-piped-stdio-fd-cloexec",
];
const TARBALL_HASHES: &[(&str, &str)] = &[
    (
        "1.1.17",
        "15443966494cd62938320900acfd16df906cf4da56279e4dd8f4846c09f849df",
    ),
    (
        "1.1.18",
        "1aa7e3c1f5ba02372d24ba2f99ed015c7135016becc7dcbb18bf8332f513a818",
    ),
    (
        "1.1.19",
        "a02132a7c6c647ef0ad483ecbe767619adf6b660a5589cba5c937b0c83909b97",
    ),
    (
        "1.1.20",
        "6ceeb0ac91df6dca60a4fa02856807ed2e2fc6d3d70bb734d1ad61a9e44ef4da",
    ),
    (
        "1.1.21",
        "4806a347119d36be6d8ab5cc3f03319bc6aa8407a8d9203de7976a42954cabde",
    ),
    (
        "1.1.22",
        "1e1a219a86e75d7c6351f96d182ca2105302d5c34d8fa9c31265dc0adf24145f",
    ),
    (
        "1.1.23",
        "379693509ca4d68d74f75def6c95996739aa6c1dc38b120c399035c108f1a39a",
    ),
    (
        "1.1.24",
        "cff1fb7ed735da72c35658645a4f916cf74f020d4cd30ab95ebe8c2a49a4d569",
    ),
];
const BINARY_HASHES: &[(&str, &str)] = &[
    (
        "1.1.17",
        "d1ea7370fce2ae229a370d8cc42e91d4eeb971344c5f07918e55ce05a4e19579",
    ),
    (
        "1.1.18",
        "60eb243a68bfbc1bffa3823c7fb90df27a72502550b333c6248fc55f20d02564",
    ),
    (
        "1.1.19",
        "68d229d37aeabde76d15af0003d4c1ce07b211414e7452fb0309be9714ae7dd4",
    ),
    (
        "1.1.20",
        "d743ebe97c822b07d010a5a836804528119f926de9b136c9f5b2c0925fe710cb",
    ),
    (
        "1.1.21",
        "ca7ffc496be6c24bb908aab478ec5be2b8fbad76507085b885163475613332c5",
    ),
    (
        "1.1.22",
        "2822292f90deea4556938a8728fe4ed02a1d66d1525cf75fa07a171e36a38c25",
    ),
    (
        "1.1.23",
        "caf4a5f9ae0f02e0ac3db01600a7dd4a9697354e3f4dc3f0a08b2de30d3aefbc",
    ),
    (
        "1.1.24",
        "22c6ddeb06d2da6049ff861e44954bf232b77bd791986104326e9500f5327193",
    ),
];

#[test]
fn identity_freezes_official_1_1_24_without_executing_binaries() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], ANTIGRAVITY_RELEASE_AXIS);
    assert_eq!(
        identity["github_repo"],
        "google-antigravity/antigravity-cli"
    );
    assert_eq!(identity["github_latest"], true);
    assert_eq!(identity["not_gemini_cli"], true);
    assert_eq!(identity["not_antigravity_acp"], true);
    assert_eq!(identity["host"]["installed"], false);
    assert_eq!(identity["official"]["version"], "1.1.24");
    assert_eq!(
        identity["official"]["github_commit"],
        "bf27ce1134b4ead2f7bfa0a4fb3cb5fcbebcaa5a"
    );
    assert_sha256(
        &identity["official"]["tarball_sha256"],
        "cff1fb7ed735da72c35658645a4f916cf74f020d4cd30ab95ebe8c2a49a4d569",
    );
    assert_sha256(
        &identity["official"]["extracted_cli_sha256"],
        "22c6ddeb06d2da6049ff861e44954bf232b77bd791986104326e9500f5327193",
    );
    assert_eq!(identity["official"]["extracted_cli_size"], 209273088);
    assert_eq!(
        identity["official"]["elf_build_id"],
        "0d87e8b60bfaf0d76a8d5e6f838dddae"
    );
    assert_eq!(identity["official"]["version_literal_in_binary"], "1.1.24");
    assert_eq!(identity["keep_1_1_8_incompatible"], true);
    assert_eq!(identity["unpublished_1_1_25"], true);
    assert_eq!(identity["public_git_1_1_17_to_1_1_24"], "changelog-only");
    assert_eq!(identity["flatten_to_antigravity_acp"], false);
    assert_exact_string_set(
        &identity["published_stables_from_previous_ceiling"],
        &[
            "1.1.18", "1.1.19", "1.1.20", "1.1.21", "1.1.22", "1.1.23", "1.1.24",
        ],
    );
    assert_eq!(
        identity["previous_ceiling_1_1_17"]["matches_frozen_1_1_17_identity"],
        true
    );
    assert_eq!(
        identity["previous_ceiling_1_1_17"]["extracted_cli_sha256"],
        "d1ea7370fce2ae229a370d8cc42e91d4eeb971344c5f07918e55ce05a4e19579"
    );
    assert_ne!(
        identity["official"]["extracted_cli_sha256"],
        identity["previous_ceiling_1_1_17"]["extracted_cli_sha256"]
    );

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["reuse_catalogue_behavior"],
        "antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1"
    );
    assert_eq!(
        decision["reuse_headless_behavior"],
        "antigravity.stream-json.cli-1.1.8-artifact-1.1.9-v1"
    );
    assert_eq!(decision["raise_latest_qualified_to"], "1.1.24");
    assert_eq!(decision["keep_baseline"], "1.1.9");
    assert_exact_string_set(
        &decision["qualify_intermediates"],
        &["1.1.18", "1.1.19", "1.1.20", "1.1.21", "1.1.22", "1.1.23"],
    );
    assert_eq!(decision["new_milestone"], false);
    assert_eq!(decision["later_unverified_after_qualification"], "1.1.25");
    assert_true_object(
        decision,
        &[
            "keep_1_1_8_incompatible",
            "map_input_format",
            "map_mcp_subcommand",
            "map_mic_serve",
            "map_voice",
            "map_remote_control",
            "map_project_name",
            "flatten_to_gemini_api_key",
            "flatten_to_antigravity_acp",
            "provider_prompt_sent",
            "live_catalogue",
            "live_print_run",
            "host_install_changed",
            "downloaded_binaries_executed",
        ],
        &["keep_1_1_8_incompatible"],
    );
}

#[test]
fn protocol_keeps_selected_mapped_subset_and_bounds_unmapped_keys() {
    let protocol = json(PROTOCOL);
    assert_exact_string_set(&protocol["help_selected_flags_present"], SELECTED_FLAGS);
    assert_eq!(protocol["selected_catalogue_command"], "models");
    assert_eq!(protocol["selected_output_format"], "stream-json");
    assert_eq!(protocol["selected_permission_mode"], "request-review");
    assert_eq!(protocol["selected_mapped_subset_unchanged"], true);
    assert_eq!(
        protocol["selected_flag_literals_present_1_1_17_through_1_1_24"],
        true
    );
    assert_eq!(protocol["package_tree_unchanged"], true);
    assert_eq!(protocol["binaries_byte_identical_across_hops"], false);
    assert_eq!(
        protocol["already_mapped_lifecycle_repairs_are_compatible_extension"],
        true
    );
    assert_exact_string_set(
        &protocol["already_mapped_lifecycle_repairs"],
        LIFECYCLE_REPAIRS,
    );
    assert_exact_string_set(&protocol["help_delta_from_1_1_17"], &["mic-serve"]);
    assert_exact_string_set(&protocol["unused_deltas"], UNMAPPED_KEYS);
    assert_eq!(protocol["decoder_corpus"], "antigravity-cli-1.1.9");
    assert_eq!(protocol["map_input_format"], false);
    assert_eq!(protocol["downloaded_binaries_executed"], false);
}

#[test]
fn dist_inventory_is_one_changed_binary_per_hop() {
    let inventory = json(DIST_INVENTORY);
    assert_exact_string_set(&inventory["compared"], COMPARED);
    assert_eq!(inventory["not_a_complete_semantic_changelog"], true);
    assert_eq!(inventory["extracted_cli_name"], "antigravity");
    let counts = inventory["package_file_counts"]
        .as_object()
        .expect("package file counts");
    assert_eq!(
        counts.keys().map(String::as_str).collect::<BTreeSet<_>>(),
        COMPARED.iter().copied().collect()
    );
    for version in COMPARED {
        assert_eq!(counts[*version], 1, "{version}");
    }
    assert_exact_string_set(&inventory["identical_through_1_1_17_to_1_1_24"], &[]);
    for hop in HOPS {
        assert_exact_string_set(&inventory[hop]["added"], &[]);
        assert_exact_string_set(&inventory[hop]["removed"], &[]);
        assert_exact_string_set(&inventory[hop]["changed"], &["antigravity"]);
        assert_exact_string_set(&inventory[hop]["identical"], &[]);
    }
    for (version, digest) in TARBALL_HASHES {
        assert_sha256(&inventory["hashes"]["tarball"][version], digest);
    }
    for (version, digest) in BINARY_HASHES {
        assert_sha256(&inventory["hashes"]["antigravity"][version], digest);
    }
    assert_ne!(
        inventory["hashes"]["antigravity"]["1.1.17"],
        inventory["hashes"]["antigravity"]["1.1.24"]
    );
}

#[test]
fn claim_at_observation_still_ends_at_1_1_17() {
    let identity = json(IDENTITY);
    assert_eq!(identity["claim_at_observation"]["baseline"], "1.1.9");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "1.1.17"
    );
    assert_eq!(ANTIGRAVITY_BASELINE_VERSION, "1.1.9");
    assert_eq!(ANTIGRAVITY_LATEST_QUALIFIED_VERSION, "1.1.17");

    let catalogue = antigravity_catalogue_claim();
    let headless = antigravity_headless_claim();
    for candidate in ["1.1.9", "1.1.17"] {
        assert!(matches!(
            catalogue.assess(&version(candidate)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
                    && matched.behavior_revision().as_str()
                        == "antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1"
        ));
        assert!(matches!(
            headless.assess(&version(candidate)),
            InterfaceCompatibilityAssessment::Qualified(matched)
                if matched.support_status() == InterfaceSupportStatus::Maintained
                    && matched.behavior_revision().as_str()
                        == "antigravity.stream-json.cli-1.1.8-artifact-1.1.9-v1"
        ));
    }
    assert!(!catalogue.permits(&version("1.1.8")));
    assert!(!headless.permits(&version("1.1.8")));
    for newer in ["1.1.18", "1.1.24"] {
        assert!(matches!(
            catalogue.assess(&version(newer)),
            InterfaceCompatibilityAssessment::UnverifiedNewer(_)
        ));
        assert!(matches!(
            headless.assess(&version(newer)),
            InterfaceCompatibilityAssessment::UnverifiedNewer(_)
        ));
    }
}

fn json(value: &str) -> Value {
    serde_json::from_str(value).expect("frozen corpus JSON is valid")
}

fn assert_sha256(value: &Value, expected: &str) {
    let value = value.as_str().expect("digest is text");
    assert_eq!(value.len(), 64);
    assert!(value.bytes().all(u8::is_ascii_hexdigit));
    assert_eq!(value, expected);
}

fn assert_exact_string_set(value: &Value, expected: &[&str]) {
    assert_eq!(string_set(value), expected.iter().copied().collect());
}

fn assert_true_object(value: &Value, keys: &[&str], true_keys: &[&str]) {
    let actual = value
        .as_object()
        .expect("object")
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    for key in keys {
        assert!(actual.contains(key), "missing key {key}");
    }
    for key in true_keys {
        assert_eq!(value[key], true, "{key}");
    }
    for key in keys {
        if !true_keys.contains(key) {
            assert_eq!(value[key], false, "{key}");
        }
    }
}

fn string_set(value: &Value) -> BTreeSet<&str> {
    value
        .as_array()
        .expect("string array")
        .iter()
        .map(|item| item.as_str().expect("string"))
        .collect()
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
