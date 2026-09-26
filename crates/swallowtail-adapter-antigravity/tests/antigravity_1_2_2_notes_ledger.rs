use serde_json::Value;
use std::collections::BTreeSet;
use swallowtail_adapter_antigravity::{
    ANTIGRAVITY_BASELINE_VERSION, ANTIGRAVITY_CATALOGUE_LATEST_QUALIFIED_VERSION,
    ANTIGRAVITY_HEADLESS_LATEST_QUALIFIED_VERSION, ANTIGRAVITY_RELEASE_AXIS,
    antigravity_catalogue_claim, antigravity_headless_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const IDENTITY: &str = include_str!("fixtures/antigravity-cli-1.2.2/identity.json");
const PROTOCOL: &str = include_str!("fixtures/antigravity-cli-1.2.2/protocol.json");

const NEW_RELEASES: &[&str] = &["1.1.27", "1.1.28", "1.2.0", "1.2.1", "1.2.2"];
const CATALOGUE_INTERMEDIATES: &[&str] = &[
    "1.1.18", "1.1.19", "1.1.20", "1.1.21", "1.1.22", "1.1.23", "1.1.24", "1.1.25", "1.1.26",
    "1.1.27", "1.1.28", "1.2.0", "1.2.1",
];
const HOPS: &[&str] = &[
    "from_1_1_26_to_1_1_27",
    "from_1_1_27_to_1_1_28",
    "from_1_1_28_to_1_2_0",
    "from_1_2_0_to_1_2_1",
    "from_1_2_1_to_1_2_2",
];

struct Artifact {
    version: &'static str,
    published_at: &'static str,
    commit: &'static str,
    linux_tarball: &'static str,
    linux_tarball_size: u64,
    mac_tarball: &'static str,
    linux_binary: &'static str,
    linux_binary_size: u64,
    build_id: &'static str,
    mac_binary: &'static str,
    mac_binary_size: u64,
}

const ARTIFACTS: &[Artifact] = &[
    Artifact {
        version: "1.1.27",
        published_at: "2026-09-05T04:23:25Z",
        commit: "1ae9cb7b51667192c051b73a91099c71e816ca5f",
        linux_tarball: "f874d4f6b8a73c2df660f580f25fb656fcb6e64adbfd746e6692e837fd9a20be",
        linux_tarball_size: 56_789_301,
        mac_tarball: "e901e5c8fd20ab4c21c01df306030079286d08e6d372cdb535d5ccc7a3f565f4",
        linux_binary: "93eb2118b778a4005700b54cdd7e08b896fbe665d5ff338e38e9e53da9a091ea",
        linux_binary_size: 210_551_040,
        build_id: "f9e9161520c4891e4727593e196cba3d",
        mac_binary: "d583be1344ea9cfa0c45cff2c1342af7837f4833c4edb65e69bee84776a45caa",
        mac_binary_size: 177_426_912,
    },
    Artifact {
        version: "1.1.28",
        published_at: "2026-09-09T00:43:52Z",
        commit: "baef32d9f7d91aa5f8f1747b4a8a938723132a19",
        linux_tarball: "074ff4f732a750ad727aeed5fc82ed34b1fb72fda2a6ceba6c8e652ffd0a94b0",
        linux_tarball_size: 56_914_961,
        mac_tarball: "8f642cffce8bc14aa3e49d1a75780bb2bd99fe7a3016389627476d9e3ec911eb",
        linux_binary: "a8793092fbe6eea0b8228fc20582ec306f7f7698d1151be526902b1556a76f3a",
        linux_binary_size: 210_923_776,
        build_id: "b3f253d9868abc95dc081fe2fcf9f702",
        mac_binary: "2a63488c33221627f1798ff4ccb1d5131b6abe3160286c84b357a7456905f763",
        mac_binary_size: 177_756_720,
    },
    Artifact {
        version: "1.2.0",
        published_at: "2026-09-10T01:43:29Z",
        commit: "34406bef8e87fc103783c0c9715e5e2cce3c3e1b",
        linux_tarball: "d9bfee1ae6e4329562cb87da1f5fc3c886d18594837e73e25c3aae00a49499b9",
        linux_tarball_size: 57_224_342,
        mac_tarball: "8fee3c120142490f2eddd286ff7d532b3e660c92d17e60a3612cc0ad1122478f",
        linux_binary: "195bf11b249deebe67028305a9b7b1d19ac38e9ab281b786a163a7d2fc8ff428",
        linux_binary_size: 212_304_128,
        build_id: "734ed857e46e56054b0ce7d40927d3b7",
        mac_binary: "f3671863b53ecef2c45a41673677fe603fbe5d73df33d744feec3e88d5af0199",
        mac_binary_size: 178_935_744,
    },
    Artifact {
        version: "1.2.1",
        published_at: "2026-09-11T06:46:48Z",
        commit: "e4afe6b6f3aa115b1ba31e26db6508a23b5e42e5",
        linux_tarball: "6a2c53db6c681fc114f9a1e499e7b4771357ab2852242e56acbd43197d4807f9",
        linux_tarball_size: 57_458_208,
        mac_tarball: "b80425e10a7b92f20679eee5df3bb18e3f9154b3653a373fad47e2f72614248e",
        linux_binary: "38f130cdd0757e1d22e151baa48ace4074a5bd3d960eb2dcc7f44bdf2ad4c0fd",
        linux_binary_size: 213_422_336,
        build_id: "c61f1d72352c28d668f23f19c79fd7b9",
        mac_binary: "98724c5370d91a2fc99a1b42cccb8cda5075dba61615e18aa71daaacb7a71f86",
        mac_binary_size: 179_956_448,
    },
    Artifact {
        version: "1.2.2",
        published_at: "2026-09-12T03:51:08Z",
        commit: "ba985e6b5de2ac8aa09860a154a102831eb7722b",
        linux_tarball: "2cfa5c9a4a1edd96db6d4058f34970be60d3bcacda866e2bdce6aefb2451b48e",
        linux_tarball_size: 57_596_853,
        mac_tarball: "f90ff6094a196f1be3854ac45d999a542d47ec66a1513aa882a8505b947b9a0f",
        linux_binary: "e8f90ef67943b56c1148d73bc0e102d0b44d18935ffb11d49a2d870a095f416b",
        linux_binary_size: 213_582_080,
        build_id: "fa00846652bf39ea18e012d43e3e161f",
        mac_binary: "cabadc15a61944372bede1fdff186701c17467dd9d718e97dc79283055d3c101",
        mac_binary_size: 180_089_984,
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
    assert_eq!(identity["official_latest"], "1.2.2");
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
    assert_eq!(official["version"], "1.2.2");
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
    assert_eq!(host["version"], "1.1.19");
    assert_eq!(
        host["version_authority"],
        "byte-identical-to-official-1.1.19-mac-arm64"
    );
    assert_sha256(
        &host["executable_sha256"],
        "96fae3fccfb444c7fb2c6d8d70426e5c978e4f21cfc4507a541f612a8b8ffeef",
    );
    assert_eq!(host["executable_size"], 178_046_224);
    assert_eq!(host["executed_during_probe"], false);

    let boundary = &identity["frozen_1_1_26_boundary_reproduction"];
    assert_sha256(
        &boundary["1.1.26_linux_x64_tarball_sha256"],
        "c47c0726266b3513660b7094bceceecbd03d8ae907786aa269c507ceb7e4ee54",
    );
    assert_sha256(
        &boundary["1.1.26_linux_x64_binary_sha256"],
        "a0a6a8044d01accd39e6f5926d29648d212a2e519ff14102f09e1c061e6171dd",
    );
    assert_sha256(
        &boundary["1.1.21_linux_x64_binary_sha256"],
        "ca7ffc496be6c24bb908aab478ec5be2b8fbad76507085b885163475613332c5",
    );
    assert_sha256(
        &boundary["1.1.22_linux_x64_binary_sha256"],
        "2822292f90deea4556938a8728fe4ed02a1d66d1525cf75fa07a171e36a38c25",
    );
    assert_eq!(boundary["matches_research_283"], true);
    assert_eq!(boundary["digest_disagreement"], false);

    assert_eq!(identity["first_unpublished_later_stable"], "1.2.3");
    assert_eq!(identity["first_unpublished_release_absent"], true);
    assert_eq!(identity["first_unpublished_tag_absent"], true);
    assert_eq!(identity["keep_1_1_8_incompatible"], true);
    assert_eq!(identity["decoder_corpus"], "antigravity-cli-1.1.9");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
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
    assert_eq!(catalogue["advance_to"], "1.2.2");
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
    assert_eq!(headless["unqualified_gap"], "1.1.18..=1.2.2");
    assert_eq!(
        headless["keep_headless_behavior"],
        "antigravity.stream-json.cli-1.1.8-artifact-1.1.9-v1"
    );
    assert_eq!(headless["keep_baseline"], "1.1.9");

    for flag in [
        "new_public_operation",
        "new_private_milestone",
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
        changes["from_1_1_27_to_1_1_28"]["headless_classification"],
        "authority-stop-deepens"
    );
    assert_exact_string_set(
        &changes["from_1_1_27_to_1_1_28"]["headless_changes"],
        &[
            "transient-model-api-errors-retry-for-much-longer-with-exponential-backoff",
            "print-timeout-expiry-now-returns-partial-output-and-exits-zero",
            "print-mode-waits-for-background-tasks-bounded-by-print-timeout-and-leaves-daemon-tasks-running",
            "print-fatal-errors-report-on-stderr-with-a-stable-error-marker-and-truncation-note",
            "headless-plan-review-proceeds-automatically-instead-of-stalling",
            "url-fetch-default-permission-changes-from-always-allowed-to-asking-first",
        ],
    );
    assert_eq!(
        changes["from_1_1_28_to_1_2_0"]["headless_classification"],
        "authority-stop-maintained"
    );
    assert_exact_string_set(
        &changes["from_1_1_28_to_1_2_0"]["headless_changes"],
        &[
            "content-safety-filter-blocks-surface-a-clear-content-filter-stop-reason-instead-of-silent-or-retried-ends",
        ],
    );
    assert_eq!(
        changes["from_1_2_0_to_1_2_1"]["headless_classification"],
        "authority-stop-maintained"
    );
    assert_exact_string_set(
        &changes["from_1_2_0_to_1_2_1"]["headless_changes"],
        &[
            "transient-genai-api-errors-502-503-504-per-minute-429-and-mid-stream-interruptions-automatically-retry-in-process-with-exponential-backoff",
        ],
    );
    assert_eq!(
        changes["from_1_2_1_to_1_2_2"]["headless_classification"],
        "authority-stop-maintained"
    );
    assert_exact_string_array(&changes["from_1_2_1_to_1_2_2"]["headless_changes"], &[]);

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
    let much_longer = &retry["hop_1_1_28_much_longer_retry"];
    assert_exact_keys(
        much_longer,
        &[
            "published_finite_bound_present",
            "public_disable_control_present",
            "published_backoff_shape",
            "admissible_under_contract_023",
        ],
    );
    assert_eq!(much_longer["published_finite_bound_present"], false);
    assert_eq!(much_longer["public_disable_control_present"], false);
    assert_eq!(much_longer["admissible_under_contract_023"], false);
    assert_eq!(
        much_longer["published_backoff_shape"],
        "exponential, duration unbounded in the notes"
    );
    let broader = &retry["hop_1_2_1_broader_automatic_retry"];
    assert_exact_keys(
        broader,
        &[
            "published_finite_bound_present",
            "public_disable_control_present",
            "published_codes",
            "admissible_under_contract_023",
        ],
    );
    assert_eq!(broader["published_finite_bound_present"], false);
    assert_eq!(broader["public_disable_control_present"], false);
    assert_eq!(broader["admissible_under_contract_023"], false);
    assert_exact_string_set(
        &broader["published_codes"],
        &[
            "502",
            "503",
            "504",
            "429-per-minute",
            "mid-stream-interruptions",
        ],
    );
    assert_eq!(retry["host_deadline_is_not_a_provider_retry_policy"], true);
    assert_eq!(retry["inferred_bound_recorded"], false);

    let timeout = &protocol["print_timeout_trace"];
    assert_eq!(
        timeout["expiry_before_1_1_28"],
        "timeout error terminal failure"
    );
    assert_eq!(
        timeout["expiry_from_1_1_28"],
        "returns partial output and exits zero with a stderr warning"
    );
    assert_eq!(
        timeout["published_default_or_caller_bounds"],
        "not stated in the notes"
    );
    assert_eq!(timeout["selected_deadline_authority_unchanged"], true);

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
        &authority["hop_1_2_0_content_filter_stop"],
        &[
            "applies_to_selected_headless_stream",
            "new_stop_reason_published",
            "catalogue_affected",
        ],
        &[
            "applies_to_selected_headless_stream",
            "new_stop_reason_published",
        ],
    );
    assert_true_object(
        &authority["catalogue_isolation"],
        &[
            "notes_name_no_models_selected_path_change",
            "retry_notes_name_the_agent_model_request_loop",
            "catalogue_advance_independent_of_headless_stop",
        ],
        &[
            "notes_name_no_models_selected_path_change",
            "retry_notes_name_the_agent_model_request_loop",
            "catalogue_advance_independent_of_headless_stop",
        ],
    );

    assert_exact_string_set(
        &protocol["material_unmapped_published_changes"],
        &[
            "interactive-model-picker-and-effort-defaults",
            "remote-control-services",
            "mcp-and-plugin-management",
            "custom-agents-and-skills",
            "gemini-api-key-sign-in",
            "conversation-delete-and-worktree-cleanup",
            "url-fetch-permission-default",
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
    assert_eq!(ANTIGRAVITY_CATALOGUE_LATEST_QUALIFIED_VERSION, "1.2.11");
    assert_eq!(ANTIGRAVITY_HEADLESS_LATEST_QUALIFIED_VERSION, "1.1.17");

    let catalogue = antigravity_catalogue_claim();
    for candidate in [
        "1.1.9", "1.1.17", "1.1.21", "1.1.22", "1.1.27", "1.2.0", "1.2.2", "1.2.7", "1.2.8",
        "1.2.9", "1.2.10", "1.2.11",
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
        "1.1.18", "1.1.22", "1.1.27", "1.2.0", "1.2.1", "1.2.2", "1.2.7", "1.2.8", "1.2.11",
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
