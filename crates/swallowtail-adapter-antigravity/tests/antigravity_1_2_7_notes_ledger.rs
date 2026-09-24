use serde_json::Value;
use std::collections::BTreeSet;
use swallowtail_adapter_antigravity::{
    ANTIGRAVITY_BASELINE_VERSION, ANTIGRAVITY_CATALOGUE_LATEST_QUALIFIED_VERSION,
    ANTIGRAVITY_HEADLESS_LATEST_QUALIFIED_VERSION, ANTIGRAVITY_RELEASE_AXIS,
    antigravity_catalogue_claim, antigravity_headless_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const IDENTITY: &str = include_str!("fixtures/antigravity-cli-1.2.7/identity.json");
const PROTOCOL: &str = include_str!("fixtures/antigravity-cli-1.2.7/protocol.json");

const NEW_RELEASES: &[&str] = &["1.2.3", "1.2.4", "1.2.5", "1.2.6", "1.2.7"];
const CATALOGUE_INTERMEDIATES: &[&str] = &["1.2.3", "1.2.4", "1.2.5", "1.2.6"];
const HOPS: &[&str] = &[
    "from_1_2_2_to_1_2_3",
    "from_1_2_3_to_1_2_4",
    "from_1_2_4_to_1_2_5",
    "from_1_2_5_to_1_2_6",
    "from_1_2_6_to_1_2_7",
];

struct Artifact {
    version: &'static str,
    published_at: &'static str,
    commit: &'static str,
    linux_tarball: &'static str,
    linux_tarball_size: u64,
    mac_tarball: &'static str,
    mac_tarball_size: u64,
    linux_binary: &'static str,
    linux_binary_size: u64,
    build_id: &'static str,
    mac_binary: &'static str,
    mac_binary_size: u64,
}

const ARTIFACTS: &[Artifact] = &[
    Artifact {
        version: "1.2.3",
        published_at: "2026-09-15T02:03:04Z",
        commit: "444063c79f81a36f124ad613b30caf113f093c0b",
        linux_tarball: "57afb34f2a4be9296beb477e600761b6ac7401eb3a54a64a0014d573b7fc3af4",
        linux_tarball_size: 57_891_892,
        mac_tarball: "c244ec966f5d8c22d845117332a6858f3abaf0fb5c8058e337d51b77b2c93e50",
        mac_tarball_size: 50_217_630,
        linux_binary: "c4c8a6722f9b570e370941b0953ba29051336307d7999ec842bdf7500b0ca7c8",
        linux_binary_size: 215_384_320,
        build_id: "41c0722061557995df58308202bf857a",
        mac_binary: "e0c2743e3fd062e11c13d2eb5fabaa244936094bbe39c30fdfbac619bb78e554",
        mac_binary_size: 181_740_432,
    },
    Artifact {
        version: "1.2.4",
        published_at: "2026-09-16T03:54:29Z",
        commit: "e5dcb8247e3c364671f2bae4c64971c8632590ed",
        linux_tarball: "dcd3e4d8c8afb1902d59c1ae52812458d2ddab67a5d2db44810c512910d918fe",
        linux_tarball_size: 58_151_273,
        mac_tarball: "f59c12c289e74bbb48178f827702c6224bd0aa920914319f85b42760dfc72f4c",
        mac_tarball_size: 50_449_780,
        linux_binary: "5c19ea964509bc4fd33c3789860f017a8ca22ef93cb48d333c7b95b118bbb6a7",
        linux_binary_size: 217_022_720,
        build_id: "4c44c1cba03957b96aa5127baafa441d",
        mac_binary: "a939016cfb86e3862112ee57124f1bc14f30defdd69e181a8526b6e7c80a4471",
        mac_binary_size: 182_974_368,
    },
    Artifact {
        version: "1.2.5",
        published_at: "2026-09-17T04:10:06Z",
        commit: "48e88e0723bc8f3c7b6bdae85d34067b04fc9277",
        linux_tarball: "e450caab5682acc920721b04cf0f6860c313d1f5296bc6e49d79cd3843802e65",
        linux_tarball_size: 58_151_164,
        mac_tarball: "b37495eebe53e1c565dd6d77bdd1d3ba603216b5d3acddafb185f97d35fdf9eb",
        mac_tarball_size: 50_431_921,
        linux_binary: "84808e105f643f135d9b347b36005d5bbe6cfe09c5ffc61e3311700adb5a3286",
        linux_binary_size: 217_284_864,
        build_id: "cd38e0524c59a16f19d3d459ae25311d",
        mac_binary: "cdf6bcff6840b12184b7ff3210c156a522f9d855ce32f4e27ce34c40091a8409",
        mac_binary_size: 183_156_336,
    },
    Artifact {
        version: "1.2.6",
        published_at: "2026-09-18T04:21:05Z",
        commit: "d39491f6f98a62aaf29af76964aa4fe75bc044d4",
        linux_tarball: "3d4973187c4c074e70894068053eeef1b7dfa9c0a16bfe9c3b81954de0d2cc5c",
        linux_tarball_size: 58_626_363,
        mac_tarball: "14e1be7ed9b35e512b6aa2f2b2ba3ad79877dbde1b2d487892d3721806ff9e59",
        mac_tarball_size: 50_904_209,
        linux_binary: "312eb057d8b8155383e74242e948333c6556adc779e9398b121a863390691fe9",
        linux_binary_size: 218_132_688,
        build_id: "15cff90bb5b1dfa7429a148ef393c21f",
        mac_binary: "7e1a1036b68cd8e2ad66097ddc8f43dc2081079d51d340863647dc8de2a1bc84",
        mac_binary_size: 184_043_424,
    },
    Artifact {
        version: "1.2.7",
        published_at: "2026-09-19T01:01:46Z",
        commit: "7bb195acaec9e7788df5210d0dc3e15f3cefc6b3",
        linux_tarball: "e410dd56d8c213ef12643d3ff5eaaab57a17e05bbf72e9415322f23879fc4a18",
        linux_tarball_size: 61_763_170,
        mac_tarball: "ce9fe3f4d6f44a2b1c83b334fc5c8f2975079959e24dd805e10eb49ab8c76a7e",
        mac_tarball_size: 53_945_901,
        linux_binary: "9991515b6d5307bcf701069622b0537b6b206e605f3c891c0cf3a3d208dea8b0",
        linux_binary_size: 224_268_544,
        build_id: "4666c9380200e7538aab7b73748b9341",
        mac_binary: "8c01ef82307dc01455418eb2e6e82f2989a3fa8b815c4b192efaaa89a55bef8d",
        mac_binary_size: 189_658_880,
    },
];

#[test]
fn identity_freezes_every_new_official_release_and_both_platform_assets() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], ANTIGRAVITY_RELEASE_AXIS);
    assert_eq!(
        identity["github_repo"],
        "google-antigravity/antigravity-cli"
    );
    assert_eq!(identity["official_latest"], "1.2.7");
    assert_eq!(identity["not_gemini_cli"], true);
    assert_eq!(identity["not_antigravity_acp"], true);
    assert_exact_string_array(
        &identity["published_stables_from_previous_ceiling"],
        NEW_RELEASES,
    );
    assert_exact_keys(&identity["artifacts"], NEW_RELEASES);

    for expected in ARTIFACTS {
        let actual = &identity["artifacts"][expected.version];
        assert_eq!(
            actual["published_at"], expected.published_at,
            "{}",
            expected.version
        );
        assert_eq!(
            actual["github_commit"], expected.commit,
            "{}",
            expected.version
        );
        assert_sha256(&actual["linux_x64_tarball_sha256"], expected.linux_tarball);
        assert_eq!(
            actual["linux_x64_tarball_size"], expected.linux_tarball_size,
            "{}",
            expected.version
        );
        assert_sha256(&actual["mac_arm64_tarball_sha256"], expected.mac_tarball);
        assert_eq!(
            actual["mac_arm64_tarball_size"], expected.mac_tarball_size,
            "{}",
            expected.version
        );
        assert_sha256(
            &actual["linux_x64_extracted_cli_sha256"],
            expected.linux_binary,
        );
        assert_eq!(
            actual["linux_x64_extracted_cli_size"], expected.linux_binary_size,
            "{}",
            expected.version
        );
        assert_eq!(
            actual["linux_x64_elf_build_id"], expected.build_id,
            "{}",
            expected.version
        );
        assert_sha256(
            &actual["mac_arm64_extracted_cli_sha256"],
            expected.mac_binary,
        );
        assert_eq!(
            actual["mac_arm64_extracted_cli_size"], expected.mac_binary_size,
            "{}",
            expected.version
        );
        assert_eq!(
            actual["version_literal_in_binary"], expected.version,
            "{}",
            expected.version
        );
    }

    let official = &identity["official"];
    assert_eq!(official["version"], "1.2.7");
    assert_eq!(official["github_commit"], ARTIFACTS[4].commit);
    assert_sha256(
        &official["assets"]["linux_x64"]["sha256"],
        ARTIFACTS[4].linux_tarball,
    );
    assert_sha256(
        &official["assets"]["linux_x64"]["extracted_cli_sha256"],
        ARTIFACTS[4].linux_binary,
    );
    assert_sha256(
        &official["assets"]["mac_arm64"]["sha256"],
        ARTIFACTS[4].mac_tarball,
    );
    assert_sha256(
        &official["assets"]["mac_arm64"]["extracted_cli_sha256"],
        ARTIFACTS[4].mac_binary,
    );

    let host = &identity["host"];
    assert_eq!(host["installed"], false);
    assert_eq!(host["install_attempted"], false);
    assert_eq!(host["executed_during_probe"], false);

    let boundary = &identity["frozen_1_2_2_boundary_reproduction"];
    assert_sha256(
        &boundary["1.2.2_linux_x64_tarball_sha256"],
        "2cfa5c9a4a1edd96db6d4058f34970be60d3bcacda866e2bdce6aefb2451b48e",
    );
    assert_sha256(
        &boundary["1.2.2_linux_x64_binary_sha256"],
        "e8f90ef67943b56c1148d73bc0e102d0b44d18935ffb11d49a2d870a095f416b",
    );
    assert_eq!(boundary["matches_research_323"], true);
    assert_eq!(boundary["digest_disagreement"], false);

    assert_eq!(identity["public_git_hops"], "CHANGELOG.md-only");
    assert_eq!(identity["first_unpublished_later_stable"], "1.2.8");
    assert_eq!(identity["first_unpublished_release_absent"], true);
    assert_eq!(identity["first_unpublished_tag_absent"], true);
    assert_eq!(identity["keep_1_1_8_incompatible"], true);
    assert_eq!(identity["decoder_corpus"], "antigravity-cli-1.1.9");
    assert_eq!(
        identity["claim_at_observation"]["catalogue_latest_qualified"],
        "1.2.2"
    );
    assert_eq!(
        identity["claim_at_observation"]["headless_latest_qualified"],
        "1.1.17"
    );
}

#[test]
fn release_note_classification_advances_catalogue_and_keeps_the_headless_stop() {
    let identity = json(IDENTITY);
    let decision = &identity["identity_decision"];

    assert_eq!(decision["behavioural_authority"], "official-release-notes");
    assert!(
        decision["authority_ruling"]
            .as_str()
            .expect("authority ruling is text")
            .contains("official release notes are the behavioural authority")
    );

    let catalogue = &decision["catalogue"];
    assert_exact_keys(
        catalogue,
        &[
            "shape",
            "advance_to",
            "qualify_intermediates",
            "keep_catalogue_behavior",
            "keep_baseline",
            "reason",
        ],
    );
    assert_eq!(catalogue["shape"], "compatible-extension");
    assert_eq!(catalogue["advance_to"], "1.2.7");
    assert_exact_string_array(&catalogue["qualify_intermediates"], CATALOGUE_INTERMEDIATES);
    assert_eq!(
        catalogue["keep_catalogue_behavior"],
        "antigravity.catalogue.cli-1.1.8-artifact-1.1.9-v1"
    );
    assert_eq!(catalogue["keep_baseline"], "1.1.9");

    let headless = &decision["headless"];
    assert_exact_keys(
        headless,
        &[
            "shape",
            "blocking_hop",
            "keep_latest_qualified",
            "unqualified_gap",
            "keep_headless_behavior",
            "keep_baseline",
            "reason",
        ],
    );
    assert_eq!(headless["shape"], "stop");
    assert_eq!(headless["blocking_hop"], "1.1.22");
    assert_eq!(headless["keep_latest_qualified"], "1.1.17");
    assert_eq!(headless["unqualified_gap"], "1.1.18..=1.2.7");
    assert_eq!(
        headless["keep_headless_behavior"],
        "antigravity.stream-json.cli-1.1.8-artifact-1.1.9-v1"
    );
    assert_eq!(headless["keep_baseline"], "1.1.9");

    for flag in [
        "new_public_operation",
        "new_private_milestone",
        "headless_must_move_with_catalogue",
        "headless_stop_reopened",
        "provider_prompt_sent",
        "live_catalogue",
        "live_print_run",
        "host_install_changed",
        "downloaded_binaries_executed",
        "exhaustive_binary_forensics_after_ruling",
        "official_latest_moved_during_run",
    ] {
        assert_eq!(decision[flag], false, "{flag}");
    }
}

#[test]
fn protocol_classifies_every_published_selected_path_change_per_claim() {
    let protocol = json(PROTOCOL);
    assert_eq!(protocol["selected_surfaces_unchanged_in_notes"], true);
    let inventory = &protocol["archive_inventory"];
    assert_exact_string_array(&inventory["compared"], NEW_RELEASES);
    assert_exact_string_array(&inventory["files_per_archive"], &["antigravity"]);
    assert_exact_string_array(&inventory["identical_through_1_2_3_to_1_2_7"], &[]);
    assert_eq!(inventory["mapped_binary_byte_identical_across_hops"], false);
    for version in NEW_RELEASES {
        assert_eq!(inventory["package_file_counts"][version], 1, "{version}");
    }
    assert_exact_string_array(&inventory["every_hop"]["added"], &[]);
    assert_exact_string_array(&inventory["every_hop"]["removed"], &[]);
    assert_exact_string_array(&inventory["every_hop"]["changed"], &["antigravity"]);
    assert_exact_string_array(&inventory["every_hop"]["identical"], &[]);

    let changes = &protocol["published_selected_path_changes"];
    assert_exact_keys(changes, HOPS);

    for hop in HOPS {
        let hop = &changes[hop];
        assert_exact_keys(
            hop,
            &[
                "catalogue_changes",
                "headless_changes",
                "catalogue_classification",
                "headless_classification",
            ],
        );
        assert_exact_string_array(&hop["catalogue_changes"], &[]);
        assert_eq!(hop["catalogue_classification"], "unchanged");
    }

    assert_eq!(
        changes["from_1_2_2_to_1_2_3"]["headless_classification"],
        "authority-stop-maintained"
    );
    assert_exact_string_array(&changes["from_1_2_2_to_1_2_3"]["headless_changes"], &[]);
    assert_eq!(
        changes["from_1_2_3_to_1_2_4"]["headless_classification"],
        "authority-stop-maintained"
    );
    assert_exact_string_set(
        &changes["from_1_2_3_to_1_2_4"]["headless_changes"],
        &["agent-turns-no-longer-terminate-on-tool-schema-validation-failure"],
    );
    assert_eq!(
        changes["from_1_2_5_to_1_2_6"]["headless_classification"],
        "authority-stop-deepens"
    );
    assert_exact_string_set(
        &changes["from_1_2_5_to_1_2_6"]["headless_changes"],
        &[
            "headless-default-print-timeout-changes-from-5-minutes-to-unlimited",
            "headless-agent-or-model-api-failure-prints-agy-error-json-and-exits-3",
        ],
    );
    assert_eq!(
        changes["from_1_2_6_to_1_2_7"]["headless_classification"],
        "authority-stop-maintained"
    );
    assert_exact_string_set(
        &changes["from_1_2_6_to_1_2_7"]["headless_changes"],
        &[
            "model-api-retry-per-attempt-backoff-capped-at-30-seconds",
            "headless-background-waiting-notice-occasionally-skipped",
        ],
    );

    let retry = &protocol["headless_retry_trace"];
    let hop_1_1_22 = &retry["hop_1_1_22_http_502_retry"];
    assert_true_object(
        hop_1_1_22,
        &[
            "published_finite_bound_present",
            "public_disable_control_present",
            "separate_operator_acceptance_present",
            "stop",
        ],
        &["stop"],
    );
    let backoff = &retry["hop_1_2_7_backoff_cap"];
    assert_exact_keys(
        backoff,
        &[
            "published_finite_attempt_bound_present",
            "public_disable_control_present",
            "published_per_attempt_backoff_cap",
            "admissible_under_contract_023",
        ],
    );
    assert_eq!(backoff["published_finite_attempt_bound_present"], false);
    assert_eq!(backoff["public_disable_control_present"], false);
    assert_eq!(backoff["admissible_under_contract_023"], false);
    assert_eq!(backoff["published_per_attempt_backoff_cap"], "30 seconds");
    assert_eq!(retry["host_deadline_is_not_a_provider_retry_policy"], true);
    assert_eq!(retry["inferred_bound_recorded"], false);

    let timeout = &protocol["print_timeout_trace"];
    assert_eq!(
        timeout["default_from_1_2_6"],
        "unlimited unless --print-timeout is passed"
    );
    assert_eq!(timeout["selected_deadline_authority_unchanged"], true);
    assert_eq!(timeout["catalogue_affected"], false);
    assert_eq!(
        protocol["headless_error_trace"]["catalogue_affected"],
        false
    );

    let authority = &protocol["authority_trace"];
    assert_true_object(
        &authority["contract_023"],
        &[
            "host_deadline_does_not_replace_provider_native_retry_policy",
            "provider_managed_retry_requires_separate_acceptance",
            "no_separate_acceptance_exists_for_this_lane",
        ],
        &[
            "host_deadline_does_not_replace_provider_native_retry_policy",
            "provider_managed_retry_requires_separate_acceptance",
            "no_separate_acceptance_exists_for_this_lane",
        ],
    );
    assert_true_object(
        &authority["catalogue_isolation"],
        &[
            "notes_name_no_models_selected_path_change",
            "retry_notes_name_the_agent_model_request_loop",
            "print_timeout_and_agy_error_name_headless_prompt_path",
            "catalogue_advance_independent_of_headless_stop",
        ],
        &[
            "notes_name_no_models_selected_path_change",
            "retry_notes_name_the_agent_model_request_loop",
            "print_timeout_and_agy_error_name_headless_prompt_path",
            "catalogue_advance_independent_of_headless_stop",
        ],
    );

    assert_exact_string_set(
        &protocol["material_unmapped_published_changes"],
        &[
            "interactive-question-and-copy-btw",
            "remote-control-session-flag",
            "plugin-and-skill-management",
            "interactive-model-picker-and-slash-model",
            "gemini-api-key-headless-daemon-background",
            "customization-token-budget",
            "kitty-graphics-artifact-viewer",
            "default-agent-legacy-toolset-retirement",
        ],
    );
    for flag in [
        "provider_prompt_sent",
        "live_catalogue",
        "live_print_run",
        "host_install_changed",
        "downloaded_binaries_executed",
    ] {
        assert_eq!(protocol[flag], false, "{flag}");
    }
}

#[test]
fn per_claim_segments_split_catalogue_from_the_headless_stop() {
    assert_eq!(ANTIGRAVITY_BASELINE_VERSION, "1.1.9");
    assert_eq!(ANTIGRAVITY_CATALOGUE_LATEST_QUALIFIED_VERSION, "1.2.7");
    assert_eq!(ANTIGRAVITY_HEADLESS_LATEST_QUALIFIED_VERSION, "1.1.17");

    let catalogue = antigravity_catalogue_claim();
    for candidate in [
        "1.1.9", "1.1.17", "1.2.2", "1.2.3", "1.2.4", "1.2.5", "1.2.6", "1.2.7",
    ] {
        assert!(matches!(
            catalogue.assess(&version(candidate)),
            InterfaceCompatibilityAssessment::Qualified(_)
        ));
    }
    assert!(!catalogue.permits(&version("1.1.8")));
    assert!(matches!(
        catalogue.assess(&version("1.2.8")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));

    let headless = antigravity_headless_claim();
    for candidate in ["1.1.9", "1.1.17"] {
        assert!(matches!(
            headless.assess(&version(candidate)),
            InterfaceCompatibilityAssessment::Qualified(_)
        ));
    }
    for candidate in [
        "1.1.18", "1.1.22", "1.2.2", "1.2.3", "1.2.6", "1.2.7", "1.2.8",
    ] {
        assert!(matches!(
            headless.assess(&version(candidate)),
            InterfaceCompatibilityAssessment::UnverifiedNewer(_)
        ));
    }
    assert!(!headless.permits(&version("1.1.8")));
}

fn json(value: &str) -> Value {
    serde_json::from_str(value).expect("frozen corpus JSON is valid")
}

fn assert_sha256(value: &Value, expected: &str) {
    let value = value.as_str().expect("digest is text");
    assert_eq!(value.len(), 64);
    assert!(value.bytes().all(|byte| byte.is_ascii_hexdigit()));
    assert_eq!(value, expected);
}

fn assert_exact_keys(value: &Value, expected: &[&str]) {
    let actual = value
        .as_object()
        .expect("object")
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    assert_eq!(actual, expected.iter().copied().collect());
}

fn assert_exact_string_array(value: &Value, expected: &[&str]) {
    let actual = value
        .as_array()
        .expect("string array")
        .iter()
        .map(|item| item.as_str().expect("string"))
        .collect::<Vec<_>>();
    assert_eq!(actual, expected);
}

fn assert_exact_string_set(value: &Value, expected: &[&str]) {
    let actual = value
        .as_array()
        .expect("string array")
        .iter()
        .map(|item| item.as_str().expect("string"))
        .collect::<BTreeSet<_>>();
    assert_eq!(actual, expected.iter().copied().collect());
}

fn assert_true_object(value: &Value, keys: &[&str], true_keys: &[&str]) {
    assert_exact_keys(value, keys);
    for key in keys {
        assert_eq!(value[key], true_keys.contains(key), "{key}");
    }
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
