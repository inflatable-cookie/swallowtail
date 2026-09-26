use serde_json::Value;
use std::collections::BTreeSet;
use swallowtail_adapter_antigravity::{
    ANTIGRAVITY_BASELINE_VERSION, ANTIGRAVITY_CATALOGUE_LATEST_QUALIFIED_VERSION,
    ANTIGRAVITY_HEADLESS_LATEST_QUALIFIED_VERSION, ANTIGRAVITY_RELEASE_AXIS,
    antigravity_catalogue_claim, antigravity_headless_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const IDENTITY: &str = include_str!("fixtures/antigravity-cli-1.2.11/identity.json");
const PROTOCOL: &str = include_str!("fixtures/antigravity-cli-1.2.11/protocol.json");

const NEW_RELEASES: &[&str] = &["1.2.8", "1.2.9", "1.2.10", "1.2.11"];
const CATALOGUE_INTERMEDIATES: &[&str] = &["1.2.8", "1.2.9", "1.2.10"];
const HOPS: &[&str] = &[
    "from_1_2_7_to_1_2_8",
    "from_1_2_8_to_1_2_9",
    "from_1_2_9_to_1_2_10",
    "from_1_2_10_to_1_2_11",
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
        version: "1.2.8",
        published_at: "2026-09-22T04:12:17Z",
        commit: "ad7d70342a108687a1b36db7573050de3e9c2c3e",
        linux_tarball: "244752206d1f65c01aff489628f1df51f1a3fddacaa8ed74984661ebb6d09136",
        linux_tarball_size: 59_525_457,
        mac_tarball: "f77d57d99ca83e50a3a767d70621c99962ce4242e5470220f3adbf1c50844538",
        mac_tarball_size: 52_219_853,
        linux_binary: "c20434f0b9278196498069dac5a0a2e72bc0b5f8aebdf17c5d535b5369b76f67",
        linux_binary_size: 216_797_392,
        build_id: "65997a9e3a37b8a6666307959500a78e",
        mac_binary: "62913fb38f14d376e67b62014e701063f4061aba60b0e049cb6af9cab62763ef",
        mac_binary_size: 184_085_952,
    },
    Artifact {
        version: "1.2.9",
        published_at: "2026-09-23T04:42:15Z",
        commit: "818089f390e240921bb597b7a22ce9c96cdf7fe6",
        linux_tarball: "d9850373f3df866011024a961fa9740cc4adaac060eebe9c70fbf263ac6b2624",
        linux_tarball_size: 59_668_209,
        mac_tarball: "2b2671c846f62cb1159817517e4a9fff3e9a2ffe01f9dd0b3e227298d88b46f6",
        mac_tarball_size: 52_354_867,
        linux_binary: "1dbb10f8295cc1ad2e558bd006c7808fe53b6c7f678a887eb557b576bb591711",
        linux_binary_size: 217_424_080,
        build_id: "268d50b56fcfeb754baa4ec32408b0b4",
        mac_binary: "0ff346ae903f15d863bd6a3e401638b612848210d28b174461acdb8780ac8c22",
        mac_binary_size: 184_515_040,
    },
    Artifact {
        version: "1.2.10",
        published_at: "2026-09-24T06:26:54Z",
        commit: "59a894a009e1ec73129112c8d744ff6e9eb5a81d",
        linux_tarball: "77cb69251292aa35b0b662f91f704f06dd787b72f7902a62db8c6d692989203e",
        linux_tarball_size: 60_286_675,
        mac_tarball: "95d5d8ab8870b849a157f647bb4d9953184f97855cbfbedebdedcc421ca5b435",
        mac_tarball_size: 52_941_197,
        linux_binary: "aea7ed8df1e79b716c0ccd14c7d8086db75b14da535ffb7febdec9a488deff67",
        linux_binary_size: 218_566_864,
        build_id: "5d98c91992eba8921435a28d79f35d87",
        mac_binary: "1e43262d55f69e20bf4ba4f087252d65dd4ed37c0a980fae50a6bd5bc3637650",
        mac_binary_size: 185_534_944,
    },
    Artifact {
        version: "1.2.11",
        published_at: "2026-09-25T04:22:17Z",
        commit: "6dadd6227a49905f475d22b7f0afe59493229595",
        linux_tarball: "c91c62c5e6fa954f5a7e1d7b9ad417d749db4aa60a4ba0b3d604dec1b645d190",
        linux_tarball_size: 60_414_410,
        mac_tarball: "437a813cd7c606ccbb3180886887fc69361c28fe8e880327b3b82201afa900cc",
        mac_tarball_size: 53_058_630,
        linux_binary: "ec7cf797ecb0e1d91ddf3b6d9d6c1d616bb89f78a5b0e43536b72a7fce695f56",
        linux_binary_size: 219_545_808,
        build_id: "257bedb917787ceb25bbe0d36687c663",
        mac_binary: "42e76bedafb5896bc6a6eefb61902162f6ba08ddf59efd3357767102e3a59a0c",
        mac_binary_size: 186_406_752,
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
    assert_eq!(identity["official_latest"], "1.2.11");
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
    }

    let official = &identity["official"];
    assert_eq!(official["version"], "1.2.11");
    assert_eq!(official["github_commit"], ARTIFACTS[3].commit);
    assert_sha256(
        &official["assets"]["linux_x64"]["sha256"],
        ARTIFACTS[3].linux_tarball,
    );
    assert_sha256(
        &official["assets"]["linux_x64"]["extracted_cli_sha256"],
        ARTIFACTS[3].linux_binary,
    );
    assert_sha256(
        &official["assets"]["mac_arm64"]["sha256"],
        ARTIFACTS[3].mac_tarball,
    );
    assert_sha256(
        &official["assets"]["mac_arm64"]["extracted_cli_sha256"],
        ARTIFACTS[3].mac_binary,
    );

    let host = &identity["host"];
    assert_eq!(host["installed"], false);
    assert_eq!(host["install_attempted"], false);
    assert_eq!(host["executed_during_probe"], false);

    let boundary = &identity["frozen_1_2_7_boundary_reproduction"];
    assert_sha256(
        &boundary["1.2.7_linux_x64_tarball_sha256"],
        "e410dd56d8c213ef12643d3ff5eaaab57a17e05bbf72e9415322f23879fc4a18",
    );
    assert_sha256(
        &boundary["1.2.7_mac_arm64_tarball_sha256"],
        "ce9fe3f4d6f44a2b1c83b334fc5c8f2975079959e24dd805e10eb49ab8c76a7e",
    );
    assert_eq!(boundary["matches_research_346"], true);
    assert_eq!(boundary["digest_disagreement"], false);

    let bounded = &identity["bounded_binary_observations"];
    assert_eq!(
        bounded["model_retry_control_lead"],
        "AGY_CLI_MODEL_API_MAX_RETRIES"
    );
    assert_eq!(
        bounded["model_retry_control_published_in_official_notes"],
        false
    );
    assert_eq!(bounded["model_retry_control_semantics_verified"], false);
    assert_exact_keys(&bounded["version_literal"], NEW_RELEASES);

    let docs = &identity["official_docs_cross_check"];
    assert_eq!(docs["docs_published_print_timeout_default"], "5m");
    assert_eq!(
        docs["release_notes_print_timeout_default_from_1_2_6"],
        "unlimited"
    );
    assert_eq!(docs["behavioural_authority"], "official-release-notes");
    assert_eq!(docs["docs_publish_any_model_retry_control"], false);
    assert_eq!(
        docs["artifact_identity_disagreement_across_channels"],
        false
    );

    assert_eq!(identity["public_git_hops"], "CHANGELOG.md-only");
    assert_eq!(identity["first_unpublished_later_stable"], "1.2.12");
    assert_eq!(identity["first_unpublished_release_absent"], true);
    assert_eq!(identity["first_unpublished_tag_absent"], true);
    assert_eq!(identity["keep_1_1_8_incompatible"], true);
    assert_eq!(identity["decoder_corpus"], "antigravity-cli-1.1.9");
    assert_eq!(
        identity["claim_at_observation"]["catalogue_latest_qualified"],
        "1.2.7"
    );
    assert_eq!(
        identity["claim_at_observation"]["headless_latest_qualified"],
        "1.1.17"
    );
    assert_eq!(
        identity["claim_at_observation"]["classification_of_1_2_11"],
        "unverified_newer"
    );
}

#[test]
fn release_note_classification_advances_catalogue_and_raises_the_headless_ruling_request() {
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
    assert_eq!(catalogue["advance_to"], "1.2.11");
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
    assert_eq!(headless["shape"], "stop-pending-operator-ruling");
    assert_eq!(headless["blocking_hop"], "1.1.22");
    assert_eq!(headless["keep_latest_qualified"], "1.1.17");
    assert_eq!(headless["unqualified_gap"], "1.1.18..=1.2.11");
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
    assert_eq!(
        protocol["selected_surface_presence_unchanged_in_notes"],
        true
    );
    let inventory = &protocol["archive_inventory"];
    assert_exact_string_array(&inventory["compared"], NEW_RELEASES);
    assert_exact_string_array(&inventory["files_per_archive"], &["antigravity"]);
    assert_exact_string_array(&inventory["identical_through_1_2_8_to_1_2_11"], &[]);
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
        changes["from_1_2_7_to_1_2_8"]["headless_classification"],
        "authority-stop-maintained"
    );
    assert_exact_string_set(
        &changes["from_1_2_7_to_1_2_8"]["headless_changes"],
        &["shutdown-cancels-open-streaming-connections-immediately"],
    );
    assert_eq!(
        changes["from_1_2_8_to_1_2_9"]["headless_classification"],
        "authority-stop-deepens"
    );
    assert_exact_string_set(
        &changes["from_1_2_8_to_1_2_9"]["headless_changes"],
        &[
            "headless-daemon-background-processes-terminate-when-the-run-ends",
            "headless-background-tasks-wait-until-print-timeout-deadline-up-to-30-minute-cap",
        ],
    );
    assert_eq!(
        changes["from_1_2_9_to_1_2_10"]["headless_classification"],
        "authority-stop-deepens"
    );
    assert_exact_string_set(
        &changes["from_1_2_9_to_1_2_10"]["headless_changes"],
        &["headless-partial-response-then-model-or-agent-error-exits-3-with-agy-error"],
    );
    assert_eq!(
        changes["from_1_2_10_to_1_2_11"]["headless_classification"],
        "authority-stop-maintained"
    );
    assert_exact_string_set(
        &changes["from_1_2_10_to_1_2_11"]["headless_changes"],
        &[
            "project-custom-agents-resolved-in-headless-runs",
            "reasoning-effort-level-selection-improved-for-models-with-different-support",
        ],
    );

    let mapping = &protocol["selected_mapping_changes_by_hop"];
    assert_exact_keys(mapping, &["from_1_2_10_to_1_2_11"]);
    assert_exact_string_array(
        &mapping["from_1_2_10_to_1_2_11"],
        &["--effort model-support-dependent reasoning level improvement"],
    );

    let retry = &protocol["headless_retry_trace"];
    assert_true_object(
        &retry["hop_1_1_22_http_502_retry"],
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

    let latest = &retry["hop_1_2_11_retry_bound"];
    assert_exact_keys(
        latest,
        &[
            "published_finite_attempt_bound_present",
            "public_disable_control_present",
            "published_per_attempt_backoff_cap",
            "unpublished_control_lead",
            "unpublished_control_semantics_verified",
        ],
    );
    assert_eq!(latest["published_finite_attempt_bound_present"], false);
    assert_eq!(latest["public_disable_control_present"], false);
    assert_eq!(
        latest["unpublished_control_lead"],
        "AGY_CLI_MODEL_API_MAX_RETRIES"
    );
    assert_eq!(latest["unpublished_control_semantics_verified"], false);
    assert_eq!(retry["host_deadline_is_not_a_provider_retry_policy"], true);
    assert_eq!(retry["inferred_bound_recorded"], false);

    let timeout = &protocol["print_timeout_trace"];
    assert_eq!(
        timeout["default_from_1_2_6"],
        "unlimited unless --print-timeout is passed"
    );
    assert_eq!(timeout["official_docs_page_still_publishes"], "5m");
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
            "effort_note_names_headless_effort_selection",
            "print_timeout_and_agy_error_name_headless_prompt_path",
            "catalogue_advance_independent_of_headless_stop",
        ],
        &[
            "notes_name_no_models_selected_path_change",
            "effort_note_names_headless_effort_selection",
            "print_timeout_and_agy_error_name_headless_prompt_path",
            "catalogue_advance_independent_of_headless_stop",
        ],
    );

    assert_exact_string_set(
        &protocol["material_unmapped_published_changes"],
        &[
            "interactive-tui-and-vim-editing",
            "artifact-viewer-and-kitty-graphics",
            "context-compaction-budget-sizing",
            "remote-control-and-enterprise-sign-in",
            "plugin-skill-agent-registry-scanning",
            "voice-dictation-deadline",
            "gemini-api-key-session-parity",
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
fn ruling_request_names_every_option_and_its_required_ruling() {
    let protocol = json(PROTOCOL);
    let request = &protocol["ruling_request"];
    assert_exact_keys(
        request,
        &[
            "raised_by",
            "blocked_on",
            "current_official_stable",
            "current_headless_ceiling",
            "no_claim_change_in_this_record",
            "options",
        ],
    );
    assert_eq!(request["raised_by"], "swallowtail#065");
    assert_eq!(
        request["blocked_on"],
        "contract-023-provider-managed-retry-separate-acceptance"
    );
    assert_eq!(request["current_official_stable"], "1.2.11");
    assert_eq!(request["current_headless_ceiling"], "1.1.17");
    assert_eq!(request["no_claim_change_in_this_record"], true);

    let options = &request["options"];
    assert_exact_keys(
        options,
        &[
            "accept_published_retry",
            "require_finite_retry_pin",
            "keep_ceiling",
        ],
    );

    let accept = &options["accept_published_retry"];
    assert_exact_keys(
        accept,
        &[
            "operator_ruling_required",
            "contract",
            "effect",
            "consumer_visible_change",
            "publishes_attempt_bound",
            "published_per_attempt_backoff_cap",
            "host_deadline_remains_the_only_terminal_bound",
        ],
    );
    assert_eq!(accept["operator_ruling_required"], true);
    assert_eq!(accept["contract"], "023-provider-managed-retry-exception");
    assert_eq!(accept["publishes_attempt_bound"], false);
    assert_eq!(accept["consumer_visible_change"], true);

    let pin = &options["require_finite_retry_pin"];
    assert_exact_keys(
        pin,
        &[
            "operator_ruling_required",
            "contract",
            "effect",
            "control_published_in_official_notes",
            "control_present_in_official_artifact_strings",
            "control_semantics_verified",
            "needs_its_own_evidence_before_qualification",
        ],
    );
    assert_eq!(pin["operator_ruling_required"], true);
    assert_eq!(pin["contract"], "029-pinned-settings");
    assert_eq!(pin["control_published_in_official_notes"], false);
    assert_eq!(pin["control_present_in_official_artifact_strings"], true);
    assert_eq!(pin["control_semantics_verified"], false);
    assert_eq!(pin["needs_its_own_evidence_before_qualification"], true);

    let keep = &options["keep_ceiling"];
    assert_exact_keys(
        keep,
        &[
            "operator_ruling_required",
            "contract",
            "effect",
            "terminal_stop",
        ],
    );
    assert_eq!(keep["operator_ruling_required"], true);
    assert_eq!(keep["contract"], "029-no-terminal-stop");
    assert_eq!(keep["terminal_stop"], true);
}

#[test]
fn per_claim_segments_split_catalogue_from_the_headless_stop() {
    assert_eq!(ANTIGRAVITY_BASELINE_VERSION, "1.1.9");
    assert_eq!(ANTIGRAVITY_CATALOGUE_LATEST_QUALIFIED_VERSION, "1.2.11");
    assert_eq!(ANTIGRAVITY_HEADLESS_LATEST_QUALIFIED_VERSION, "1.1.17");

    let catalogue = antigravity_catalogue_claim();
    for candidate in [
        "1.1.9", "1.1.17", "1.2.2", "1.2.7", "1.2.8", "1.2.9", "1.2.10", "1.2.11",
    ] {
        assert!(matches!(
            catalogue.assess(&version(candidate)),
            InterfaceCompatibilityAssessment::Qualified(_)
        ));
    }
    assert!(!catalogue.permits(&version("1.1.8")));
    assert!(matches!(
        catalogue.assess(&version("1.2.12")),
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
        "1.1.18", "1.1.22", "1.2.2", "1.2.7", "1.2.8", "1.2.9", "1.2.10", "1.2.11", "1.2.12",
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
