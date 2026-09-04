use serde_json::Value;
use std::collections::BTreeSet;
use swallowtail_adapter_antigravity::{
    ANTIGRAVITY_BASELINE_VERSION, ANTIGRAVITY_LATEST_QUALIFIED_VERSION, ANTIGRAVITY_RELEASE_AXIS,
    antigravity_catalogue_claim, antigravity_headless_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const IDENTITY: &str = include_str!("fixtures/antigravity-cli-1.1.26/identity.json");
const PROTOCOL: &str = include_str!("fixtures/antigravity-cli-1.1.26/protocol.json");
const DIST_INVENTORY: &str = include_str!("fixtures/antigravity-cli-1.1.26/dist-inventory.json");

const VERSIONS: &[&str] = &[
    "1.1.17", "1.1.18", "1.1.19", "1.1.20", "1.1.21", "1.1.22", "1.1.23", "1.1.24", "1.1.25",
    "1.1.26",
];
const PUBLISHED_NEWER: &[&str] = &[
    "1.1.18", "1.1.19", "1.1.20", "1.1.21", "1.1.22", "1.1.23", "1.1.24", "1.1.25", "1.1.26",
];
const SELECTED_SURFACES: &[&str] = &[
    "--print",
    "--output-format",
    "--model",
    "--mode",
    "--sandbox",
    "--effort",
    "--json-schema",
    "--conversation",
    "models",
];
const HOPS: &[&str] = &[
    "from_1_1_17_to_1_1_18",
    "from_1_1_18_to_1_1_19",
    "from_1_1_19_to_1_1_20",
    "from_1_1_20_to_1_1_21",
    "from_1_1_21_to_1_1_22",
    "from_1_1_22_to_1_1_23",
    "from_1_1_23_to_1_1_24",
    "from_1_1_24_to_1_1_25",
    "from_1_1_25_to_1_1_26",
];

struct Artifact {
    version: &'static str,
    published_at: &'static str,
    commit: &'static str,
    tarball: &'static str,
    tarball_size: u64,
    binary: &'static str,
    binary_size: u64,
    build_id: &'static str,
}

const ARTIFACTS: &[Artifact] = &[
    Artifact {
        version: "1.1.17",
        published_at: "2026-08-20T22:13:58Z",
        commit: "efa16f096dc02fb654b7e86958d268195284d014",
        tarball: "15443966494cd62938320900acfd16df906cf4da56279e4dd8f4846c09f849df",
        tarball_size: 55_607_296,
        binary: "d1ea7370fce2ae229a370d8cc42e91d4eeb971344c5f07918e55ce05a4e19579",
        binary_size: 205_574_400,
        build_id: "de253ec6ade81ddac11b45a8558af46a",
    },
    Artifact {
        version: "1.1.18",
        published_at: "2026-08-22T01:46:57Z",
        commit: "f09d6b583d0f902d3f0f63736af23d34f0a5ddbe",
        tarball: "1aa7e3c1f5ba02372d24ba2f99ed015c7135016becc7dcbb18bf8332f513a818",
        tarball_size: 55_721_724,
        binary: "60eb243a68bfbc1bffa3823c7fb90df27a72502550b333c6248fc55f20d02564",
        binary_size: 206_024_960,
        build_id: "dd44555625e546bec8bd0befba77123a",
    },
    Artifact {
        version: "1.1.19",
        published_at: "2026-08-22T23:30:26Z",
        commit: "ee5766c17fce8f27ea85185f97183575058218ec",
        tarball: "a02132a7c6c647ef0ad483ecbe767619adf6b660a5589cba5c937b0c83909b97",
        tarball_size: 55_763_391,
        binary: "68d229d37aeabde76d15af0003d4c1ce07b211414e7452fb0309be9714ae7dd4",
        binary_size: 206_188_800,
        build_id: "708bc379032a11dfc939a59a9b461991",
    },
    Artifact {
        version: "1.1.20",
        published_at: "2026-08-25T02:58:27Z",
        commit: "ade702a5439c2bc67de2f9cfcb83c5370768f0c6",
        tarball: "6ceeb0ac91df6dca60a4fa02856807ed2e2fc6d3d70bb734d1ad61a9e44ef4da",
        tarball_size: 56_295_623,
        binary: "d743ebe97c822b07d010a5a836804528119f926de9b136c9f5b2c0925fe710cb",
        binary_size: 208_085_248,
        build_id: "917b0a8cc7c33aec77e9e26374c01659",
    },
    Artifact {
        version: "1.1.21",
        published_at: "2026-08-26T02:21:06Z",
        commit: "7cc1925c8cbe021699038606ada488618dbda5a2",
        tarball: "4806a347119d36be6d8ab5cc3f03319bc6aa8407a8d9203de7976a42954cabde",
        tarball_size: 56_329_878,
        binary: "ca7ffc496be6c24bb908aab478ec5be2b8fbad76507085b885163475613332c5",
        binary_size: 208_183_552,
        build_id: "54865ceacda7fc2f3ff8071b1b8180fb",
    },
    Artifact {
        version: "1.1.22",
        published_at: "2026-08-27T04:03:21Z",
        commit: "556846a4bb94117222f53846896c7eb0d645307e",
        tarball: "1e1a219a86e75d7c6351f96d182ca2105302d5c34d8fa9c31265dc0adf24145f",
        tarball_size: 56_399_106,
        binary: "2822292f90deea4556938a8728fe4ed02a1d66d1525cf75fa07a171e36a38c25",
        binary_size: 208_429_312,
        build_id: "a9f978445e9528435a7fcaa6983687aa",
    },
    Artifact {
        version: "1.1.23",
        published_at: "2026-09-01T04:47:50Z",
        commit: "4c150a22f7f68061e8af35412b05b9f8974e4c56",
        tarball: "379693509ca4d68d74f75def6c95996739aa6c1dc38b120c399035c108f1a39a",
        tarball_size: 56_593_215,
        binary: "caf4a5f9ae0f02e0ac3db01600a7dd4a9697354e3f4dc3f0a08b2de30d3aefbc",
        binary_size: 208_986_368,
        build_id: "7d25d7790a3dc495ec30cb513b299199",
    },
    Artifact {
        version: "1.1.24",
        published_at: "2026-09-02T02:38:18Z",
        commit: "bf27ce1134b4ead2f7bfa0a4fb3cb5fcbebcaa5a",
        tarball: "cff1fb7ed735da72c35658645a4f916cf74f020d4cd30ab95ebe8c2a49a4d569",
        tarball_size: 56_692_103,
        binary: "22c6ddeb06d2da6049ff861e44954bf232b77bd791986104326e9500f5327193",
        binary_size: 209_273_088,
        build_id: "0d87e8b60bfaf0d76a8d5e6f838dddae",
    },
    Artifact {
        version: "1.1.25",
        published_at: "2026-09-03T02:30:18Z",
        commit: "7e1316ca775dc3805aac13b2db5cd37d89d5aae8",
        tarball: "45ab4a99884de17af76565a4ff8d9762d6e960067bd008fde9b050ec8fc9e421",
        tarball_size: 56_770_237,
        binary: "e552463e7cd479e342cfec3487f7b2de048b89548df74c610e3a58d1c2c9735b",
        binary_size: 210_436_352,
        build_id: "64216c04e5d62b5257e3e40bc500defd",
    },
    Artifact {
        version: "1.1.26",
        published_at: "2026-09-04T03:28:48Z",
        commit: "3bc5795ff561c9d71bf1ce272f185aec6013e5e4",
        tarball: "c47c0726266b3513660b7094bceceecbd03d8ae907786aa269c507ceb7e4ee54",
        tarball_size: 56_691_683,
        binary: "a0a6a8044d01accd39e6f5926d29648d212a2e519ff14102f09e1c061e6171dd",
        binary_size: 210_247_936,
        build_id: "ffbd3e994b91095d2d1ff46e3b54b6c4",
    },
];

struct Hop {
    key: &'static str,
    mapped: &'static [&'static str],
    unmapped: &'static [&'static str],
    classification: &'static str,
}

const HOP_EXPECTATIONS: &[Hop] = &[
    Hop {
        key: "from_1_1_17_to_1_1_18",
        mapped: &[
            "print-dropped-stream-now-surfaces-error-and-exits-nonzero",
            "valueless-print-argument-now-rejects-instead-of-consuming-the-next-flag",
        ],
        unmapped: &[
            "project-name-widening",
            "conversation-picker-rename-delete-keybindings",
            "audio-attachment-format-expansion",
        ],
        classification: "compatible-extension-repair",
    },
    Hop {
        key: "from_1_1_18_to_1_1_19",
        mapped: &[],
        unmapped: &[
            "remote-control-selects-a-free-operating-system-port",
            "logo-and-renderer-environment-controls",
        ],
        classification: "unmapped-only",
    },
    Hop {
        key: "from_1_1_19_to_1_1_20",
        mapped: &[
            "review-mode-autoapproves-workspace-scoped-read-within-the-selected-read-profile",
            "print-benign-tool-errors-and-permission-denials-no-longer-become-cascade-level-failure",
        ],
        unmapped: &[
            "skill-icons",
            "customization-listing-views",
            "settings-file-preservation",
        ],
        classification: "compatible-extension-repair",
    },
    Hop {
        key: "from_1_1_20_to_1_1_21",
        mapped: &[
            "invalid-utf8-tool-results-no-longer-stall-the-selected-stream",
            "readwrite-file-repairs-remain-under-the-selected-resource-authority",
            "embedded-ripgrep-removes-reliance-on-an-ambient-ripgrep-child",
        ],
        unmapped: &[
            "voice-and-mic-serve",
            "always-proceed-autoapproval-for-mcp-and-page-reads",
            "cost-status-line",
            "mcp-google-credential-diagnostic",
        ],
        classification: "compatible-extension-repair",
    },
    Hop {
        key: "from_1_1_21_to_1_1_22",
        mapped: &[
            "http-502-now-retries-instead-of-ending-the-run",
            "self-subagent-setup-now-preserves-parent-configuration",
            "transient-http-502-no-longer-has-the-previous-terminal-failure-shape",
        ],
        unmapped: &[
            "interactive-model-name-selection",
            "gemini-api-key-effort-repair",
            "headless-daemon-banner-removal",
        ],
        classification: "authority-stop",
    },
    Hop {
        key: "from_1_1_22_to_1_1_23",
        mapped: &[
            "models-subcommand-no-longer-hangs-on-inherited-stdin",
            "gemini-history-reconstruction-retains-tool-call-identities",
            "cancelled-subagent-status-no-longer-remains-running",
        ],
        unmapped: &[
            "mcp-subagent-dispatcher-repair",
            "google-cloud-onboarding-and-token-refresh-repairs",
        ],
        classification: "compatible-extension-repair-after-blocking-hop",
    },
    Hop {
        key: "from_1_1_23_to_1_1_24",
        mapped: &[
            "headless-piped-stdio-now-closes-on-exit-with-fd-cloexec",
            "startup-from-inaccessible-working-directory-now-uses-absolute-schema-uris",
        ],
        unmapped: &[
            "mcp-config-json5-parsing",
            "conversation-delete-cleans-annotation-files",
            "interactive-goal-side-question-repair",
        ],
        classification: "compatible-extension-repair-after-blocking-hop",
    },
    Hop {
        key: "from_1_1_24_to_1_1_25",
        mapped: &[
            "duplicate-permission-grants-no-longer-accumulate-across-session-reloads-and-subagents",
            "background-summary-update-no-longer-panics-after-trajectory-closure",
        ],
        unmapped: &[
            "resume-picker-workspace-grouping",
            "gemini-api-key-model-catalogue-addition",
            "markdown-custom-agent-ambient-skill-rule-and-subagent-inheritance",
            "mcp-oauth-code-length-repair",
            "remote-control-reverse-tunnel-hardening",
        ],
        classification: "compatible-extension-repair-after-blocking-hop",
    },
    Hop {
        key: "from_1_1_25_to_1_1_26",
        mapped: &["sqlite-wal-checkpoint-on-exit-flushes-trailing-session-metadata"],
        unmapped: &[
            "interactive-picker-effort-default",
            "always-proceed-subagent-approval-repair",
            "subagent-kill-and-conversation-delete-worktree-cleanup",
            "logout-storage-shortcut",
            "customization-discovery-log-suppression",
        ],
        classification: "compatible-extension-repair-after-blocking-hop",
    },
];

#[test]
fn identity_freezes_every_official_hop_and_the_observation_only_host() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], ANTIGRAVITY_RELEASE_AXIS);
    assert_eq!(
        identity["github_repo"],
        "google-antigravity/antigravity-cli"
    );
    assert_eq!(identity["official_latest"], "1.1.26");
    assert_eq!(identity["not_gemini_cli"], true);
    assert_eq!(identity["not_antigravity_acp"], true);
    assert_exact_string_array(
        &identity["published_stables_from_previous_ceiling"],
        PUBLISHED_NEWER,
    );
    assert_exact_keys(&identity["artifacts"], VERSIONS);

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
        assert_sha256(&actual["tarball_sha256"], expected.tarball);
        assert_eq!(
            actual["tarball_size"], expected.tarball_size,
            "{}",
            expected.version
        );
        assert_sha256(&actual["extracted_cli_sha256"], expected.binary);
        assert_eq!(
            actual["extracted_cli_size"], expected.binary_size,
            "{}",
            expected.version
        );
        assert_eq!(
            actual["elf_build_id"], expected.build_id,
            "{}",
            expected.version
        );
        assert_eq!(actual["version_literal_in_binary"], expected.version);
    }

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
    assert_eq!(
        host["signature_authority"],
        "Developer ID Application: Google LLC (EQHXZ8M8AV)"
    );
    assert_eq!(host["executed_during_probe"], false);

    let official = &identity["official"];
    assert_eq!(official["version"], "1.1.26");
    assert_eq!(official["github_commit"], ARTIFACTS[9].commit);
    assert_eq!(official["tarball_sha256"], ARTIFACTS[9].tarball);
    assert_eq!(official["extracted_cli_sha256"], ARTIFACTS[9].binary);
    assert_eq!(official["elf_build_id"], ARTIFACTS[9].build_id);
    assert_eq!(identity["first_unpublished_later_stable"], "1.1.27");
    assert_eq!(identity["first_unpublished_release_absent"], true);
    assert_eq!(identity["first_unpublished_tag_absent"], true);
}

#[test]
fn frozen_1_1_17_and_parked_pr_182_digests_were_recomputed() {
    let identity = json(IDENTITY);
    let frozen = &identity["frozen_1_1_17_recomputation"];
    assert_exact_keys(
        frozen,
        &[
            "1.1.16_linux_x64_tarball_sha256",
            "1.1.16_linux_x64_binary_sha256",
            "1.1.16_linux_x64_binary_size",
            "1.1.17_linux_x64_tarball_sha256",
            "1.1.17_linux_x64_binary_sha256",
            "1.1.17_linux_x64_binary_size",
            "1.1.17_mac_arm64_tarball_sha256",
            "matches_frozen_identity",
        ],
    );
    assert_sha256(
        &frozen["1.1.16_linux_x64_tarball_sha256"],
        "7742953b7835b457e9102f1357a493913657dfd147435584f609d58356ec085a",
    );
    assert_sha256(
        &frozen["1.1.16_linux_x64_binary_sha256"],
        "b233e6a4f38564a06a0d3220aa79f6a7c8f11da2b85fc8f0957f8a14d46e6cc9",
    );
    assert_sha256(
        &frozen["1.1.17_mac_arm64_tarball_sha256"],
        "60fe89d3aef472ddf6c7048032f7585fae732d879f3700fc3188c68c46b35cdd",
    );
    assert_eq!(
        frozen["1.1.17_linux_x64_tarball_sha256"],
        ARTIFACTS[0].tarball
    );
    assert_eq!(
        frozen["1.1.17_linux_x64_binary_sha256"],
        ARTIFACTS[0].binary
    );
    assert_eq!(frozen["matches_frozen_identity"], true);

    let parked = &identity["parked_pr_182_cross_check"];
    assert_eq!(parked["head"], "562225db6e2a77986e5f1504a70f767ccb3fe82d");
    assert_exact_string_array(&parked["compared_versions"], &VERSIONS[..8]);
    assert_exact_keys(&parked["recorded_hashes"]["tarball"], &VERSIONS[..8]);
    assert_exact_keys(&parked["recorded_hashes"]["antigravity"], &VERSIONS[..8]);
    for expected in &ARTIFACTS[..8] {
        assert_sha256(
            &parked["recorded_hashes"]["tarball"][expected.version],
            expected.tarball,
        );
        assert_sha256(
            &parked["recorded_hashes"]["antigravity"][expected.version],
            expected.binary,
        );
    }
    assert_sha256(
        &parked["1.1.24_mac_arm64_tarball_sha256"],
        "189af288ed9527f567ab3a53b35a6da2fc0c3812c6245f266c75a2a3604bdec3",
    );
    assert_eq!(parked["every_recorded_digest_recomputed"], true);
    assert_eq!(parked["digest_disagreement"], false);
    assert_eq!(parked["branch_merged_or_replayed"], false);
}

#[test]
fn selected_literals_hold_but_1_1_22_retry_is_an_authority_stop() {
    let protocol = json(PROTOCOL);
    assert_exact_string_set(&protocol["selected_surfaces"], SELECTED_SURFACES);
    assert_exact_string_set(
        &protocol["selected_values"],
        &["stream-json", "plan", "request-review"],
    );
    assert_exact_keys(&protocol["literal_presence_versions"], SELECTED_SURFACES);
    for surface in SELECTED_SURFACES {
        assert_exact_string_array(&protocol["literal_presence_versions"][surface], VERSIONS);
    }
    assert_exact_string_set(
        &protocol["material_unmapped_literals"],
        &[
            "--agent",
            "--continue",
            "--dangerously-skip-permissions",
            "--input-format",
            "--remote-control",
            "GEMINI_API_KEY",
            "mcp",
            "mic-serve",
        ],
    );
    assert_exact_string_set(
        &protocol["retry_control_literals_absent"],
        &[
            "--retry",
            "--disable-retry",
            "--max-retries",
            "--retry-count",
            "--retry-backoff",
            "--max-attempts",
        ],
    );
    assert_exact_string_array(
        &protocol["retry_control_literals_absent_in"],
        &["1.1.21", "1.1.22", "1.1.26"],
    );

    let hops = &protocol["hop_classification"];
    assert_exact_keys(hops, HOPS);
    for expected in HOP_EXPECTATIONS {
        let hop = &hops[expected.key];
        assert_exact_keys(
            hop,
            &[
                "mapped_changes",
                "material_unmapped_changes",
                "classification",
            ],
        );
        assert_exact_string_set(&hop["mapped_changes"], expected.mapped);
        assert_exact_string_set(&hop["material_unmapped_changes"], expected.unmapped);
        assert_eq!(hop["classification"], expected.classification);
    }

    let authority = &protocol["authority_trace"];
    assert_exact_keys(
        authority,
        &[
            "contract_017",
            "contract_023",
            "hop_1_1_20_workspace_read",
            "hop_1_1_22_http_502_retry",
            "hop_1_1_25_custom_agents",
            "hop_1_1_26_worktree_cleanup",
        ],
    );
    assert_true_object(
        &authority["contract_017"],
        &[
            "private_exact_conversation_id_is_not_public_load_or_resume",
            "provider_permission_observation_grants_no_write_authority",
            "ambient_route_claims_no_filesystem_or_descendant_containment",
        ],
        &[
            "private_exact_conversation_id_is_not_public_load_or_resume",
            "provider_permission_observation_grants_no_write_authority",
            "ambient_route_claims_no_filesystem_or_descendant_containment",
        ],
    );
    assert_true_object(
        &authority["contract_023"],
        &[
            "ambient_host_keeps_process_and_descendant_authority_ambient",
            "provider_permission_modes_do_not_prove_containment",
            "host_deadline_does_not_replace_provider_native_retry_policy",
            "provider_managed_retry_requires_separate_acceptance",
        ],
        &[
            "ambient_host_keeps_process_and_descendant_authority_ambient",
            "provider_permission_modes_do_not_prove_containment",
            "host_deadline_does_not_replace_provider_native_retry_policy",
            "provider_managed_retry_requires_separate_acceptance",
        ],
    );
    assert_true_object(
        &authority["hop_1_1_20_workspace_read"],
        &[
            "within_selected_resource_access_read",
            "changes_isolation_claim",
            "widening_beyond_selected_root_proved",
        ],
        &["within_selected_resource_access_read"],
    );
    let retry = &authority["hop_1_1_22_http_502_retry"];
    assert_true_object(
        retry,
        &[
            "applies_to_selected_headless_model_request",
            "separate_operator_acceptance_present",
            "published_retry_bound_present",
            "public_disable_control_present",
            "deterministic_provider_neutral_mapping_present",
            "stop",
        ],
        &["applies_to_selected_headless_model_request", "stop"],
    );
    assert_true_object(
        &authority["hop_1_1_25_custom_agents"],
        &[
            "adapter_selects_agent_flag",
            "default_agents_already_inherited_ambient_capabilities",
            "selected_authority_widening",
        ],
        &["default_agents_already_inherited_ambient_capabilities"],
    );
    assert_true_object(
        &authority["hop_1_1_26_worktree_cleanup"],
        &[
            "adapter_selects_subagent_kill_or_conversation_delete",
            "grants_swallowtail_delete_authority",
            "proves_descendant_containment",
        ],
        &[],
    );
    assert_eq!(protocol["selected_literal_set_unchanged"], true);
    assert_eq!(protocol["selected_semantics_unchanged"], false);
    assert_eq!(protocol["downloaded_binaries_executed"], false);
}

#[test]
fn distribution_inventory_is_one_changed_closed_binary_per_hop() {
    let inventory = json(DIST_INVENTORY);
    assert_exact_string_array(&inventory["compared"], VERSIONS);
    assert_exact_string_set(&inventory["identical_through_1_1_17_to_1_1_26"], &[]);
    assert_exact_keys(&inventory["package_file_counts"], VERSIONS);
    for version in VERSIONS {
        assert_eq!(inventory["package_file_counts"][version], 1);
    }
    for hop in HOPS {
        assert_exact_keys(
            &inventory[hop],
            &["added", "removed", "changed", "identical"],
        );
        assert_exact_string_set(&inventory[hop]["added"], &[]);
        assert_exact_string_set(&inventory[hop]["removed"], &[]);
        assert_exact_string_set(&inventory[hop]["changed"], &["antigravity"]);
        assert_exact_string_set(&inventory[hop]["identical"], &[]);
    }
    for expected in ARTIFACTS {
        assert_sha256(
            &inventory["hashes"]["tarball"][expected.version],
            expected.tarball,
        );
        assert_sha256(
            &inventory["hashes"]["antigravity"][expected.version],
            expected.binary,
        );
    }
    assert_eq!(
        inventory["named_closed_binary_boundary"]["public_git_hops"],
        "CHANGELOG.md-only"
    );
}

#[test]
fn stop_keeps_the_production_ceiling_and_gates_card_072() {
    let identity = json(IDENTITY);
    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "stop");
    assert_eq!(decision["blocking_hop"], "1.1.22");
    assert_eq!(decision["admitted_segment"], false);
    assert_eq!(decision["claim_card_072_admitted"], false);
    assert_eq!(decision["keep_latest_qualified"], "1.1.17");
    assert_eq!(decision["official_latest_moved_during_run"], false);
    assert_eq!(decision["downloaded_binaries_executed"], false);
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "1.1.17"
    );
    assert_eq!(ANTIGRAVITY_BASELINE_VERSION, "1.1.9");
    assert_eq!(ANTIGRAVITY_LATEST_QUALIFIED_VERSION, "1.1.17");

    for claim in [antigravity_catalogue_claim(), antigravity_headless_claim()] {
        assert!(!claim.permits(&version("1.1.8")));
        for candidate in ["1.1.18", "1.1.22", "1.1.26"] {
            assert!(matches!(
                claim.assess(&version(candidate)),
                InterfaceCompatibilityAssessment::UnverifiedNewer(_)
            ));
        }
    }
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
