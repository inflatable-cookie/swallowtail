use serde_json::Value;
use std::collections::BTreeSet;
use swallowtail_adapter_claude_agent::{
    CLAUDE_CODE_RESPONSE_ONLY_AXIS, CLAUDE_CODE_RESPONSE_ONLY_BASELINE_VERSION,
    CLAUDE_CODE_RESPONSE_ONLY_DENIED_VERSIONS, CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION,
    claude_code_response_only_binding, claude_code_response_only_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const PACKAGE: &str = include_str!("fixtures/claude-code-2.1.251/identity.json");
const RESPONSE_ONLY: &str = include_str!("fixtures/claude-code-2.1.251/response-only.json");
const CURRENT_IDENTITY: &str =
    include_str!("fixtures/claude-code-response-only-2.1.293/identity.json");
const CURRENT_PROTOCOL: &str =
    include_str!("fixtures/claude-code-response-only-2.1.293/protocol.json");
const CURRENT_INVENTORY: &str =
    include_str!("fixtures/claude-code-response-only-2.1.293/dist-inventory.json");
const CURRENT_HOOKS: &str =
    include_str!("fixtures/claude-code-response-only-2.1.293/builtin-hook-ledger.json");

#[test]
fn response_only_qualifies_2_1_251_as_compatible_extension() {
    let package: Value =
        serde_json::from_str(PACKAGE).expect("Claude Code 2.1.251 package identity is valid JSON");
    let identity: Value = serde_json::from_str(RESPONSE_ONLY)
        .expect("Claude Code 2.1.251 response-only identity is valid JSON");

    assert_eq!(package["version"], "2.1.251");
    assert_eq!(package["npm_latest"], true);
    assert_eq!(package["host"]["not_installed"], false);
    assert_eq!(identity["axis"], CLAUDE_CODE_RESPONSE_ONLY_AXIS);
    assert_eq!(identity["version"], "2.1.251");
    assert_eq!(identity["provider_prompt_sent"], false);
    assert_eq!(identity["selected_mapped_subset_unchanged"], true);

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["reuse_behavior_revision"],
        "claude-code.response-only.stream-json.v1"
    );
    assert_eq!(decision["raise_latest_qualified"], true);
    assert_eq!(decision["raise_latest_qualified_to"], "2.1.251");
    assert_eq!(decision["keep_baseline"], "2.1.227");
    assert_eq!(decision["qualify_intermediates"], true);
    assert_eq!(decision["keep_unpublished_2_1_244_incompatible"], true);
    assert_eq!(decision["keep_unpublished_2_1_249_incompatible"], true);
    assert_eq!(decision["deny_2_1_251"], false);
    assert_eq!(decision["mix_headless_axis"], false);
    assert_eq!(decision["flatten_to_claude_agent_acp"], false);
    assert_eq!(decision["later_unverified_after_qualification"], "2.1.252");

    let flags = identity["help_selected_flags_present"]
        .as_array()
        .expect("selected flags are an array");
    for required in [
        "-p",
        "--output-format",
        "--no-session-persistence",
        "--tools",
        "--safe-mode",
        "--disable-slash-commands",
        "--no-chrome",
        "--prompt-suggestions",
        "--mcp-config",
        "--strict-mcp-config",
    ] {
        assert!(
            flags.iter().any(|flag| flag == required),
            "missing selected flag {required}"
        );
    }
    assert_eq!(identity["selected_tools"], "");
    assert_eq!(identity["selected_prompt_suggestions"], "false");

    assert_eq!(CLAUDE_CODE_RESPONSE_ONLY_BASELINE_VERSION, "2.1.227");
    assert_eq!(
        CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION,
        "2.1.293"
    );
    assert_eq!(
        CLAUDE_CODE_RESPONSE_ONLY_DENIED_VERSIONS,
        &[
            "2.1.244", "2.1.249", "2.1.253", "2.1.254", "2.1.255", "2.1.256", "2.1.262", "2.1.264",
            "2.1.279",
        ]
    );
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "2.1.241"
    );

    let claim = claude_code_response_only_claim();
    assert!(claim.supports(&version("2.1.227")));
    assert!(claim.supports(&version("2.1.228")));
    assert!(claim.supports(&version("2.1.229")));
    assert!(claim.supports(&version("2.1.241")));
    assert!(claim.supports(&version("2.1.242")));
    assert!(claim.supports(&version("2.1.250")));
    assert!(!claim.permits(&version("2.1.226")));
    assert!(!claim.permits(&version("2.1.244")));
    assert!(!claim.permits(&version("2.1.249")));
    assert!(matches!(
        claim.assess(&version("2.1.251")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Deprecated
    ));
    assert!(matches!(
        claim.assess(&version("2.1.252")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Deprecated
    ));
    assert!(matches!(
        claim.assess(&version("2.1.257")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Deprecated
    ));
    assert!(!claim.permits(&version("2.1.253")));
    assert!(!claim.permits(&version("2.1.262")));
    assert!(!claim.permits(&version("2.1.264")));
    for published in [
        "2.1.258", "2.1.259", "2.1.260", "2.1.261", "2.1.263", "2.1.265", "2.1.266", "2.1.267",
        "2.1.268", "2.1.269", "2.1.270", "2.1.271", "2.1.272", "2.1.273", "2.1.274", "2.1.275",
        "2.1.276", "2.1.277", "2.1.278", "2.1.280", "2.1.281",
    ] {
        assert!(
            matches!(
                claim.assess(&version(published)),
                InterfaceCompatibilityAssessment::Qualified(matched)
                    if matched.support_status() == InterfaceSupportStatus::Deprecated
            ),
            "{published}"
        );
    }
    assert!(!claim.permits(&version("2.1.279")));
    assert!(matches!(
        claim.assess(&version("2.1.282")),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.behavior_revision().as_str() == "claude-code.response-only.stream-json.v3"
                && matched.support_status() == InterfaceSupportStatus::Maintained
    ));
    assert_eq!(
        claude_code_response_only_binding("2.1.251")
            .expect("version binds")
            .axis()
            .as_str(),
        CLAUDE_CODE_RESPONSE_ONLY_AXIS
    );
}

#[test]
fn current_response_only_identity_freezes_v3_and_every_published_hop() {
    let identity: Value =
        serde_json::from_str(CURRENT_IDENTITY).expect("current identity is valid JSON");
    let protocol: Value =
        serde_json::from_str(CURRENT_PROTOCOL).expect("current protocol is valid JSON");
    let inventory: Value =
        serde_json::from_str(CURRENT_INVENTORY).expect("current inventory is valid JSON");
    let hooks: Value = serde_json::from_str(CURRENT_HOOKS).expect("hook ledger is valid JSON");

    assert_eq!(identity["family"], "claude-code.response-only");
    assert_eq!(identity["axis"], CLAUDE_CODE_RESPONSE_ONLY_AXIS);
    assert_eq!(identity["observed_at"], "2026-10-08T00:26:41Z");
    assert_eq!(
        identity["claim_at_observation"]["latest_qualified"],
        "2.1.281"
    );
    assert_eq!(
        identity["official_channels"]["npm"]["dist_tags"]["latest"],
        "2.1.293"
    );
    assert_eq!(
        identity["official_channels"]["npm"]["dist_tags"]["stable"],
        "2.1.285"
    );
    assert_eq!(
        identity["official_channels"]["github_latest_non_prerelease"]["tag"],
        "v2.1.293"
    );
    assert_eq!(
        identity["first_unpublished_after_latest"]["version"],
        "2.1.294"
    );
    assert_eq!(
        identity["first_unpublished_after_latest"]["npm_version_endpoint_status"],
        404
    );
    assert_eq!(
        identity["first_unpublished_after_latest"]["github_release_endpoint_status"],
        404
    );
    assert_eq!(
        identity["host_observation"]["executable_available_on_path"],
        true
    );
    assert_eq!(
        identity["host_observation"]["version_and_digest"],
        "not_observed"
    );
    assert_eq!(identity["downloaded_artifacts_executed"], false);
    assert_eq!(identity["identity_decision"]["shape"], "private-milestone");
    assert_eq!(
        identity["identity_decision"]["new_behavior"],
        "claude-code.response-only.stream-json.v3"
    );
    assert_eq!(
        identity["identity_decision"]["new_segment"]["baseline"],
        "2.1.282"
    );
    assert_eq!(
        identity["identity_decision"]["new_segment"]["latest"],
        "2.1.293"
    );
    assert_eq!(
        identity["identity_decision"]["public_response_only_contract_unchanged"],
        true
    );
    assert_eq!(
        identity["identity_decision"]["source_to_npm_runtime_provenance_claimed"],
        false
    );
    assert_eq!(
        CLAUDE_CODE_RESPONSE_ONLY_LATEST_QUALIFIED_VERSION,
        "2.1.293"
    );

    let versions = (281..=293)
        .map(|patch| format!("2.1.{patch}"))
        .collect::<Vec<_>>();
    assert_eq!(
        string_set(&identity["published_hops_after_previous_ceiling"]),
        versions.iter().skip(1).cloned().collect()
    );
    let source_tags = identity["source_tags"]
        .as_array()
        .expect("source tags are an array");
    assert_eq!(source_tags.len(), versions.len());
    assert_eq!(
        source_tags
            .iter()
            .map(|tag| tag["version"]
                .as_str()
                .expect("tag version is text")
                .to_owned())
            .collect::<Vec<_>>(),
        versions
    );
    assert!(source_tags.iter().all(|tag| {
        tag["tag"]
            .as_str()
            .is_some_and(|value| value.starts_with('v'))
            && tag["commit"]
                .as_str()
                .is_some_and(|value| value.len() == 40)
            && tag["release_published_at"].as_str().is_some()
    }));
    assert_eq!(inventory["compared_versions"], serde_json::json!(versions));
    assert_eq!(
        json_keys(&inventory["packages"]),
        BTreeSet::from([
            "claude-code".to_owned(),
            "claude-code-darwin-arm64".to_owned(),
            "claude-code-linux-x64".to_owned(),
        ])
    );
    assert_eq!(
        json_keys(&hooks["versions"]),
        versions.iter().cloned().collect::<BTreeSet<_>>()
    );

    let file_sets = [
        (
            "claude-code",
            [
                "package/LICENSE.md",
                "package/README.md",
                "package/bin/claude.exe",
                "package/cli-wrapper.cjs",
                "package/install.cjs",
                "package/package.json",
                "package/sdk-tools.d.ts",
            ]
            .as_slice(),
        ),
        (
            "claude-code-darwin-arm64",
            [
                "package/LICENSE.md",
                "package/README.md",
                "package/claude",
                "package/package.json",
            ]
            .as_slice(),
        ),
        (
            "claude-code-linux-x64",
            [
                "package/LICENSE.md",
                "package/README.md",
                "package/claude",
                "package/package.json",
            ]
            .as_slice(),
        ),
    ];
    for (package, expected_paths) in file_sets {
        let package_inventory = &inventory["packages"][package]["versions"];
        assert_eq!(
            json_keys(package_inventory),
            versions.iter().cloned().collect()
        );
        let expected_paths = expected_paths
            .iter()
            .map(|path| (*path).to_owned())
            .collect::<BTreeSet<_>>();
        for version in &versions {
            let record = &package_inventory[version];
            assert_eq!(record["file_count"], expected_paths.len());
            assert_eq!(
                json_keys(&record["files"]),
                expected_paths,
                "{package} {version}"
            );
            for (path, file) in record["files"]
                .as_object()
                .expect("file inventory is an object")
            {
                assert_eq!(
                    file["sha256"].as_str().expect("file digest is text").len(),
                    64
                );
                assert!(
                    file["size"].as_u64().is_some(),
                    "{package} {version} {path}"
                );
            }
        }
    }

    let expected_platform_hashes = [
        (
            "claude-code-darwin-arm64",
            "a922981f6f3b55a251ef9f9dbaa0621a5f99cbcb5ca67f8a797476ccfc83f626",
            "fcfd837103965c64de34a6b9b94370d77a347ea71819715a27d5f0ef01775ea4",
            "4e21122a227857da1178aca3299700c1fd7f2b77c93f12e73c2c76db796a105e",
        ),
        (
            "claude-code-linux-x64",
            "56fe3da88458465fb27d7e9299dddb3fead55750fb9c2de795f233b5eea6dce1",
            "3afe8535c0cc33f0e24f7b25dab7a1727b8b592196f8496a8bc302ba2161eed3",
            "8968405e26db478af44eabc4635ab5ca557057b702a54460a59c13e1b253e978",
        ),
    ];
    for (package, baseline, first_milestone, latest) in expected_platform_hashes {
        for (version, expected) in [
            ("2.1.281", baseline),
            ("2.1.282", first_milestone),
            ("2.1.293", latest),
        ] {
            assert_eq!(
                inventory["packages"][package]["versions"][version]["files"]["package/claude"]["sha256"],
                expected,
                "{package} {version}"
            );
        }
    }
    assert_eq!(
        inventory["packages"]["claude-code"]["versions"]["2.1.293"]["tarball"]["sha256"],
        "a96c76dfce0fd4b0449201ac16e6ac4f6517330a9a046b0f53197088f9a1dff4"
    );
    assert_eq!(
        inventory["packages"]["claude-code-darwin-arm64"]["versions"]["2.1.293"]["tarball"]["sha256"],
        "0c7bbfb7c571161caa5283a61056bc6e4414222c9488e01beed77f59c9210966"
    );
    assert_eq!(
        inventory["packages"]["claude-code-linux-x64"]["versions"]["2.1.293"]["tarball"]["sha256"],
        "7b6d6842a9e9de1fa0f5dfc18967696b91883b9d82be333a05b35bcf21f7d70f"
    );

    let adjacent_hops = inventory["adjacent_hops"]
        .as_array()
        .expect("adjacent hops are an array");
    assert_eq!(adjacent_hops.len(), 12);
    for (index, hop) in adjacent_hops.iter().enumerate() {
        let from = &versions[index];
        let to = &versions[index + 1];
        assert_eq!(hop["from"], from.as_str());
        assert_eq!(hop["to"], to.as_str());
        assert_eq!(
            json_keys(&hop["packages"]),
            BTreeSet::from([
                "claude-code".to_owned(),
                "claude-code-darwin-arm64".to_owned(),
                "claude-code-linux-x64".to_owned(),
            ])
        );
        for (package, expected_paths) in file_sets {
            let delta = &hop["packages"][package];
            assert_eq!(
                string_set(&delta["added"]),
                BTreeSet::new(),
                "{package} {to}"
            );
            assert_eq!(
                string_set(&delta["removed"]),
                BTreeSet::new(),
                "{package} {to}"
            );
            let expected_changed = expected_changed_files(package, to);
            assert_eq!(
                string_set(&delta["changed"]),
                expected_changed,
                "{package} {to}"
            );
            let expected_identical = expected_paths
                .iter()
                .filter(|path| !expected_changed.contains(**path))
                .map(|path| (*path).to_owned())
                .collect::<BTreeSet<_>>();
            assert_eq!(
                string_set(&delta["identical"]),
                expected_identical,
                "{package} {to}"
            );
            assert_eq!(
                package_inventory_delta(&inventory, package, to),
                delta.clone(),
                "{package} {to} inventory and hop ledger agree"
            );
        }
    }

    let markers = serde_json::json!([
        "prompt.context",
        "session.start",
        "fs.ancestors",
        "instructionFiles",
        "AGENTS_NAMES",
        "CLAUDE_NAMES",
        "projectDirOf",
        "registerPlugin",
        "--safe-mode",
        "--disable-slash-commands",
        "--strict-mcp-config"
    ]);
    assert_eq!(hooks["route_facing_symbols"], markers);
    assert_eq!(
        hooks["route_facing_symbols_are_not_internal_function_names"],
        true
    );
    for version in &versions {
        assert_eq!(
            json_keys(&hooks["versions"][version]),
            BTreeSet::from([
                "claude-code-darwin-arm64".to_owned(),
                "claude-code-linux-x64".to_owned(),
            ])
        );
        for package in ["claude-code-darwin-arm64", "claude-code-linux-x64"] {
            let entry = &hooks["versions"][version][package];
            assert_eq!(entry["artifact_file"], "package/claude");
            assert_eq!(entry["static_markers_observed"], markers);
            assert_eq!(entry["all_route_facing_markers_observed"], true);
            assert_eq!(
                entry["artifact_sha256"],
                inventory["packages"][package]["versions"][version]["files"]["package/claude"]["sha256"]
            );
        }
    }

    let hook_mapping = &protocol["route_facing_hook_mapping"];
    assert_eq!(hook_mapping["built_in_plugin"], "agents-md");
    assert_eq!(hook_mapping["empty_tools_block_read_tool_hook"], true);
    assert_eq!(hook_mapping["prompt_context_hook_remains_selected"], true);
    assert_eq!(
        hook_mapping["no_private_resolver_function_identified"],
        true
    );
    assert_eq!(
        hook_mapping["static_strings_do_not_establish_resolver_conformance"],
        true
    );
    assert_eq!(
        protocol["linked_instruction_read_limit"]["starts_at"],
        "2.1.282"
    );
    assert_eq!(
        protocol["linked_instruction_read_limit"]["resolved_function_level_behavior_proven"],
        false
    );
    assert_eq!(
        protocol["host_process_lifecycle"]["host_local_unix_force_stop"],
        "SIGKILL; crates/swallowtail-host-local/src/process_exit.rs selects SIGKILL when force=true."
    );
    assert!(
        protocol["host_process_lifecycle"]["2.1.288_release_change"]
            .as_str()
            .expect("release note summary is text")
            .contains("not the host-local force_stop signal")
    );
    assert_eq!(
        protocol["artifact_and_scope_limits"]["source_to_runtime_provenance"],
        "not established by the public repository; no claim made"
    );

    let claim = claude_code_response_only_claim();
    for point in [
        "2.1.282", "2.1.283", "2.1.284", "2.1.285", "2.1.286", "2.1.287", "2.1.288", "2.1.289",
        "2.1.290", "2.1.291", "2.1.292", "2.1.293",
    ] {
        assert!(
            matches!(
                claim.assess(&version(point)),
                InterfaceCompatibilityAssessment::Qualified(matched)
                    if matched.behavior_revision().as_str() == "claude-code.response-only.stream-json.v3"
                        && matched.support_status() == InterfaceSupportStatus::Maintained
            ),
            "{point}"
        );
    }
    assert!(matches!(
        claim.assess(&version("2.1.294")),
        InterfaceCompatibilityAssessment::UnverifiedNewer(_)
    ));
    assert!(!claim.permits(&version("2.1.244")));
    assert!(!claim.permits(&version("2.1.249")));
    assert!(!claim.permits(&version("2.1.253")));
    assert!(!claim.permits(&version("2.1.279")));
}

fn json_keys(value: &Value) -> BTreeSet<String> {
    value
        .as_object()
        .expect("value is a JSON object")
        .keys()
        .cloned()
        .collect()
}

fn string_set(value: &Value) -> BTreeSet<String> {
    value
        .as_array()
        .expect("value is a JSON array")
        .iter()
        .map(|entry| entry.as_str().expect("array entry is text").to_owned())
        .collect()
}

fn expected_changed_files(package: &str, to: &str) -> BTreeSet<String> {
    let mut changed = BTreeSet::from(["package/package.json".to_owned()]);
    match (package, to) {
        ("claude-code", "2.1.284" | "2.1.285" | "2.1.290" | "2.1.292") => {
            changed.insert("package/sdk-tools.d.ts".to_owned());
        }
        ("claude-code", "2.1.288") => {
            changed.insert("package/install.cjs".to_owned());
        }
        ("claude-code-darwin-arm64" | "claude-code-linux-x64", _) => {
            changed.insert("package/claude".to_owned());
        }
        _ => {}
    }
    changed
}

fn package_inventory_delta(inventory: &Value, package: &str, version: &str) -> Value {
    inventory["packages"][package]["versions"][version]["delta_from_previous"].clone()
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}
