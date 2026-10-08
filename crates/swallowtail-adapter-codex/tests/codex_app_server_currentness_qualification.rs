use std::collections::BTreeSet;

use serde_json::Value;
use swallowtail_adapter_codex::{
    codex_app_server_claim, codex_app_server_lifecycle_claim, codex_exec_claim,
};
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion,
};

const ANALYSIS: &str =
    include_str!("fixtures/codex-app-server-0.161.0/qualification-analysis.json");
const ARTIFACTS: &str = include_str!("fixtures/codex-app-server-0.161.0/published-artifacts.json");
const SOURCE_INVENTORY: &str =
    include_str!("fixtures/codex-app-server-0.161.0/source-inventory.json");

const STABLE_VERSIONS: &[&str] = &[
    "0.155.1", "0.156.0", "0.156.1", "0.157.0", "0.157.1", "0.158.0", "0.159.0", "0.159.1",
    "0.159.2", "0.159.3", "0.160.0", "0.160.1", "0.161.0",
];

fn json(source: &str) -> Value {
    serde_json::from_str(source).expect("currentness evidence is valid JSON")
}

fn strings(value: &Value) -> Vec<&str> {
    value
        .as_array()
        .expect("value is an array")
        .iter()
        .map(|entry| entry.as_str().expect("entry is text"))
        .collect()
}

fn assert_exact_keys(value: &Value, expected: &[&str]) {
    let actual = value
        .as_object()
        .expect("value is an object")
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    assert_eq!(actual, expected);
}

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}

#[test]
fn official_package_and_source_chains_are_exact() {
    let artifacts = json(ARTIFACTS);
    assert_exact_keys(&artifacts, &["github_releases", "npm"]);

    let npm = &artifacts["npm"];
    assert_eq!(npm["package"], "@openai/codex");
    assert_eq!(npm["dist_tags"]["latest"], "0.161.0");
    assert_eq!(npm["dist_tags"]["alpha"], "0.162.0-alpha.18");
    let npm_versions = npm["stable_versions"].as_array().expect("stable versions");
    assert_eq!(
        npm_versions
            .iter()
            .map(|entry| entry["version"].as_str().expect("version"))
            .collect::<Vec<_>>(),
        STABLE_VERSIONS.to_vec()
    );
    for entry in npm_versions {
        assert!(entry["published"].as_str().is_some());
        assert!(
            entry["integrity"]
                .as_str()
                .is_some_and(|value| value.starts_with("sha512-"))
        );
        assert_eq!(entry["shasum"].as_str().expect("shasum").len(), 40);
        assert!(
            entry["tarball"]
                .as_str()
                .is_some_and(|value| value.starts_with("https://registry.npmjs.org/"))
        );
    }
    assert_eq!(
        npm_versions.last().expect("latest stable")["integrity"],
        "sha512-+ZnJFGBbQBwYnjUTs+PoacgYVxmNyxMLiadIz6eSJ0AzQW0mRVxuuOQizSbz2qeNAJia9Syxag6j2uGf7guD2Q=="
    );
    let platform_variants = npm["platform_variants"]
        .as_array()
        .expect("platform package variants");
    assert_eq!(platform_variants.len(), 78);
    assert!(platform_variants.iter().any(|entry| {
        entry["version"] == "0.161.0-darwin-arm64"
            && entry["integrity"]
                .as_str()
                .is_some_and(|value| value.starts_with("sha512-"))
    }));
    assert!(platform_variants.iter().any(|entry| {
        entry["version"] == "0.161.0-linux-x64"
            && entry["integrity"] == "sha512-AVkNzeJWyCHHPyg5vxD8UE6aFsYaOwq0/x6mtmkOKy6LdjvsyxLr7Q7UULlk+Ma3+PnrN8uIxG2/XGPQ936hgA=="
    }));
    assert!(platform_variants.iter().any(|entry| {
        entry["version"] == "0.161.0-darwin-arm64"
            && entry["integrity"] == "sha512-arxKLYB3uKjewwOpGFo5/MGe8+4JzNB17KIYBmGfF4k3QU+QC4ieXVUwqHmQX2nXrB1BzNMN0gtAgm1AGBTg8w=="
    }));

    let github = &artifacts["github_releases"];
    assert_eq!(github["latest_non_prerelease"]["tag_name"], "rust-v0.161.0");
    let github_tags = github["selected"]
        .as_array()
        .expect("GitHub release tags")
        .iter()
        .map(|entry| entry["tag_name"].as_str().expect("tag"))
        .collect::<BTreeSet<_>>();
    let expected_github_tags = STABLE_VERSIONS[1..]
        .iter()
        .map(|version| format!("rust-v{version}"))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        github_tags,
        expected_github_tags.iter().map(String::as_str).collect()
    );

    let inventory = json(SOURCE_INVENTORY);
    assert_exact_keys(&inventory, &["hops", "tags", "versions"]);
    assert_eq!(strings(&inventory["versions"]), STABLE_VERSIONS.to_vec());
    let tags = inventory["tags"].as_object().expect("tag inventory");
    assert_eq!(tags.len(), 13);
    assert_eq!(
        tags.keys().map(String::as_str).collect::<BTreeSet<_>>(),
        STABLE_VERSIONS.iter().copied().collect()
    );
    let tag_identities = [
        (
            "0.155.1",
            "be2951ea34f0d295ed0becf97079f92fa5f6950e",
            "4e21628f9ec9ee656650cd2b62ef92225725b5ac",
            7688,
            "1cfc73f029fdc9386e17bff398de562e54403073db8ae298c1f69eb6eb658321",
        ),
        (
            "0.156.0",
            "fe74a774532af67b5a4a3dec03ce9469e17f89af",
            "476ac1aae33835e6e4c2d311c254b9b6ce9b2f4c",
            8346,
            "72321179ab1c0e55dc0b5620e83bae7ca4165d9b3606e368328d7acbcaf7dd2d",
        ),
        (
            "0.156.1",
            "b412ff32c417f855c2b2d1581b77058eed87c84b",
            "81e8e29b2956dfe9b092c63953a9ed282781e77c",
            8346,
            "adee8cf8d08307c1c3cb88b1663b5e8aee3d58385b0e86b1b0b11c7f226549f3",
        ),
        (
            "0.157.0",
            "00c972ed5d6ff6499317fd41b7f23605b8e6850d",
            "ac21625ddf7f9dd5f34b2802212cf20295fdff95",
            8497,
            "9090d38829948d4f14ec38ff31ada7ba720ed511d32e184478eaadcd253dabb5",
        ),
        (
            "0.157.1",
            "36650394c5b38c2990ccf2a3457165ca3e9d9726",
            "ac0e23e5232692b95268583c8278c50b8c436d2b",
            8497,
            "9580103520dc7234c9742d387bfa6f4d7d052564b169db1c0e8dcbd11e8fabfe",
        ),
        (
            "0.158.0",
            "064c6b8c737f5b41d171fdda80bd9ef10ad06eb3",
            "54e1bd264b4122fe9471ee7d54c4d021a76bb8ff",
            8670,
            "b12aaebba7789aabd6127647d3af7d14111e4e1866dc919fc500bec683257087",
        ),
        (
            "0.159.0",
            "687a119f0fcaace47e1f1abcc77cec6c813fd6da",
            "377f7f557a6bdea0f3a2d26d4d899c66db4789d0",
            8697,
            "5c07a9f19c705c2b478fa28a54a251423ffe6b7a1c4d6d98ca37b4c2f326e921",
        ),
        (
            "0.159.1",
            "8e68a98ef03cdde76d2e6800791ebdf1b3b95b24",
            "dd48cd3d094a133f2781ab10343dc665a37e5bc0",
            8697,
            "d3685c50f400e4275ba754f4c12536fea03c54cf02cedaa2bb31fa24e1f65fa1",
        ),
        (
            "0.159.2",
            "ff6aec96948b70d94983af2641a6b67c94faeff5",
            "8b9fa496bbf2c47aebd62e85a080b9a522a455b5",
            8703,
            "a8afc6b4ab6658b9517adc3eda1ee3eb66aae77c7b9652a1ce432dc937f26a32",
        ),
        (
            "0.159.3",
            "01fc69f4026735edfdf6789820549727a4867b11",
            "8e46774a94a745ffdf676bd7a8aa36466bbd4f99",
            8712,
            "52b438771f9c527b50c52504fcc9cf6f6e9374867f954a11fd3f826ce0358713",
        ),
        (
            "0.160.0",
            "a956835d020762cb2b570053af06f643a11c0ecc",
            "79b1b666f2e8551f8abbbca34957227f67f3f553",
            8775,
            "928facc6dccdee5fb009f1bdfb6f385dea8a9a72fd5ecab4b630cb4c22defb3e",
        ),
        (
            "0.160.1",
            "d27764b82f7118f674371e6d6e76271d9d606edb",
            "c3e23d4c4385619ecec78408766e46b7fa7dd9ad",
            8775,
            "58725e927a4f5d3ee8c9065cd4597636fcbd3fa294dc9b3713713c284d1976e4",
        ),
        (
            "0.161.0",
            "979011409de0a60b52f179721948e65531d26144",
            "7e21416b38834816c224ea0dfd135c3de94b2f15",
            8899,
            "ba4c1ff69f36c8b300d7de106b63b1dd20fa6f520c09fc97620b133854c14f42",
        ),
    ];
    for (version, commit, tag_object, file_count, manifest_sha256) in tag_identities {
        let tag = &inventory["tags"][version];
        assert_exact_keys(
            tag,
            &[
                "commit",
                "file_count",
                "tag",
                "tag_object",
                "tree_manifest_sha256",
            ],
        );
        assert_eq!(tag["tag"], format!("rust-v{version}"));
        assert_eq!(tag["commit"], commit);
        assert_eq!(tag["tag_object"], tag_object);
        assert_eq!(tag["file_count"], file_count);
        assert_eq!(tag["tree_manifest_sha256"], manifest_sha256);
    }
    let hops = inventory["hops"].as_array().expect("source hops");
    assert_eq!(hops.len(), 12);

    let expected_counts = [
        (1095, 437, 2128, 249, 5123),
        (0, 0, 21, 0, 8325),
        (155, 4, 928, 138, 7414),
        (0, 0, 10, 0, 8487),
        (192, 19, 1018, 122, 7460),
        (44, 17, 545, 72, 8108),
        (0, 0, 29, 1, 8668),
        (6, 0, 41, 2, 8656),
        (9, 0, 15, 0, 8688),
        (65, 2, 296, 10, 8414),
        (0, 0, 2, 0, 8773),
        (150, 26, 1019, 110, 7730),
    ];
    for (index, hop) in hops.iter().enumerate() {
        assert_exact_keys(
            hop,
            &[
                "added",
                "changed",
                "feeding_candidates",
                "from",
                "identical_count",
                "identical_manifest_sha256",
                "removed",
                "to",
            ],
        );
        assert_eq!(hop["from"], STABLE_VERSIONS[index]);
        assert_eq!(hop["to"], STABLE_VERSIONS[index + 1]);
        let (added, removed, changed, feeding, identical) = expected_counts[index];
        assert_eq!(hop["added"].as_array().expect("added").len(), added);
        assert_eq!(hop["removed"].as_array().expect("removed").len(), removed);
        assert_eq!(hop["changed"].as_array().expect("changed").len(), changed);
        assert_eq!(
            hop["feeding_candidates"]
                .as_array()
                .expect("feeding candidates")
                .len(),
            feeding
        );
        assert_eq!(hop["identical_count"], identical);
        for field in ["added", "removed", "changed"] {
            let entries = strings(&hop[field]);
            assert!(entries.windows(2).all(|pair| pair[0] < pair[1]));
        }
        let feeding_candidates = strings(&hop["feeding_candidates"]);
        assert_eq!(
            feeding_candidates
                .iter()
                .copied()
                .collect::<BTreeSet<_>>()
                .len(),
            feeding_candidates.len()
        );
        let changed = strings(&hop["added"])
            .into_iter()
            .chain(strings(&hop["removed"]))
            .chain(strings(&hop["changed"]))
            .collect::<BTreeSet<_>>();
        assert!(feeding_candidates.iter().all(|path| changed.contains(path)));
        assert_eq!(
            hop["identical_manifest_sha256"]
                .as_str()
                .expect("manifest SHA-256")
                .len(),
            64
        );
    }
    let first_hop_files = strings(&hops[0]["added"])
        .into_iter()
        .chain(strings(&hops[0]["changed"]))
        .collect::<BTreeSet<_>>();
    for path in [
        "codex-rs/app-server/src/request_processors/thread_processor.rs",
        "codex-rs/config/src/loader/mod.rs",
        "codex-rs/config/src/loader/projectless_directory_tests.rs",
    ] {
        assert!(first_hop_files.contains(path));
    }
    for path in [
        "codex-rs/protocol/src/permissions.rs",
        "codex-rs/protocol/src/permissions/local_aliases.rs",
        "codex-rs/protocol/src/permissions/local_aliases_tests.rs",
    ] {
        assert!(
            strings(&hops[4]["added"])
                .into_iter()
                .chain(strings(&hops[4]["changed"]))
                .any(|candidate| candidate == path)
        );
    }
    let aws_hop = &hops[5];
    assert_eq!(aws_hop["from"], "0.158.0");
    assert_eq!(aws_hop["to"], "0.159.0");
    let changed_permission_files = strings(&aws_hop["changed"])
        .into_iter()
        .filter(|path| path.starts_with("codex-rs/protocol/src/permissions"))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        changed_permission_files,
        BTreeSet::from([
            "codex-rs/protocol/src/permissions.rs",
            "codex-rs/protocol/src/permissions/target.rs",
        ])
    );
}

#[test]
fn accepted_policy_changes_and_exact_claim_milestones_are_frozen() {
    let analysis = json(ANALYSIS);
    assert_exact_keys(
        &analysis,
        &[
            "accepted_policy_paths",
            "artifact_identity",
            "candidate",
            "inventory",
            "limitations",
            "official_channel",
            "qualification",
            "residual_gates",
            "route_boundary",
        ],
    );
    assert_eq!(analysis["candidate"]["version"], "0.161.0");
    assert_eq!(analysis["official_channel"]["npm_latest"], "0.161.0");
    assert_eq!(
        analysis["official_channel"]["github_latest_non_prerelease"],
        "rust-v0.161.0"
    );
    let final_reprobe = &analysis["official_channel"]["final_reprobe"];
    assert_exact_keys(
        final_reprobe,
        &[
            "channels_agree",
            "github_latest_non_prerelease",
            "npm_alpha_excluded",
            "npm_latest",
            "observed_at",
        ],
    );
    assert_eq!(final_reprobe["observed_at"], "2026-10-08");
    assert_eq!(final_reprobe["npm_latest"], "0.161.0");
    assert_eq!(final_reprobe["npm_alpha_excluded"], "0.162.0-alpha.20");
    assert_eq!(
        final_reprobe["github_latest_non_prerelease"],
        "rust-v0.161.0"
    );
    assert_eq!(final_reprobe["channels_agree"], true);
    assert_eq!(analysis["official_channel"]["agreement"], true);
    assert_eq!(analysis["inventory"]["tag_count"], 13);
    assert_eq!(analysis["inventory"]["hop_count"], 12);
    assert_eq!(analysis["artifact_identity"]["package"], "@openai/codex");
    assert_eq!(
        analysis["artifact_identity"]["runtime_package"],
        "@openai/codex-darwin-arm64@0.161.0"
    );
    assert_eq!(
        analysis["artifact_identity"]["runtime_sha256"],
        "12ac11d2c7eee27cfae34393986d7b7c9ed0dea537cb749831cdd7033893e6de"
    );
    assert_eq!(analysis["artifact_identity"]["runtime_executed"], false);

    let qualification = &analysis["qualification"];
    assert_exact_keys(
        qualification,
        &[
            "baseline",
            "codex_exec_ceiling_unchanged",
            "claim_id",
            "compatible_points_after_ceiling",
            "qualified_ceiling",
            "segments",
            "unpublished_gaps",
        ],
    );
    assert_eq!(qualification["claim_id"], "codex.app-server.cli-window-2");
    assert_eq!(qualification["baseline"], "0.155.1");
    assert_eq!(qualification["qualified_ceiling"], "0.161.0");
    assert_eq!(qualification["codex_exec_ceiling_unchanged"], "0.155.1");
    let expected_segments = [
        (
            "0.80.0..=0.81.0",
            "codex.app-server.v2.legacy-default-stdio",
            "deprecated",
        ),
        (
            "0.84.0..=0.99.0",
            "codex.app-server.v2.legacy-default-stdio",
            "deprecated",
        ),
        (
            "0.100.0..=0.107.0",
            "codex.app-server.v2.legacy-explicit-stdio",
            "deprecated",
        ),
        (
            "0.110.0..=0.130.0",
            "codex.app-server.v2.base",
            "deprecated",
        ),
        (
            "0.131.0..=0.155.1",
            "codex.app-server.v2.workspace-roots",
            "maintained",
        ),
        (
            "0.156.0..=0.156.1",
            "codex.app-server.v2.managed-policy-workspace-roots",
            "deprecated",
        ),
        (
            "0.157.0..=0.157.1",
            "codex.app-server.v2.managed-network-workspace-roots",
            "deprecated",
        ),
        (
            "0.158.0",
            "codex.app-server.v2.path-alias-workspace-roots",
            "deprecated",
        ),
        (
            "0.159.0..=0.161.0",
            "codex.app-server.v2.protected-aws-workspace-roots",
            "maintained",
        ),
    ];
    let segments = qualification["segments"]
        .as_array()
        .expect("claim segments");
    assert_eq!(segments.len(), expected_segments.len());
    for (segment, (range, behavior, support)) in segments.iter().zip(expected_segments) {
        assert_exact_keys(segment, &["behavior_revision", "range", "support"]);
        assert_eq!(segment["range"], range);
        assert_eq!(segment["behavior_revision"], behavior);
        assert_eq!(segment["support"], support);
    }
    assert_eq!(
        strings(&qualification["unpublished_gaps"]),
        vec![
            "0.82.0..=0.83.0",
            "0.108.0..=0.109.0",
            "0.149.2",
            "0.150.2",
            "0.151.1",
            "0.152.2",
            "0.154.1",
            "0.155.2"
        ]
    );

    let paths = analysis["accepted_policy_paths"]
        .as_array()
        .expect("accepted policy paths");
    assert_eq!(paths.len(), 5);
    let expected_paths = [
        ("0.156.0", "managed model-provider revalidation"),
        ("0.156.0", "fail-closed permission materialization"),
        (
            "0.156.0",
            "safe-to-replay bootstrap GET system-proxy fallback",
        ),
        (
            "0.156.0 and 0.161.0",
            "explicit Windows MxC under managed policy",
        ),
        (
            "0.157.0",
            "application.network policy for provider/API HTTP",
        ),
    ];
    for (path, (version, name)) in paths.iter().zip(expected_paths) {
        assert_exact_keys(
            path,
            &[
                "adapter_tests",
                "boundary",
                "path",
                "source_files",
                "upstream_tests",
                "version",
            ],
        );
        assert_eq!(path["version"], version);
        assert_eq!(path["path"], name);
        assert!(!strings(&path["source_files"]).is_empty());
        assert!(!strings(&path["upstream_tests"]).is_empty());
        assert!(!strings(&path["adapter_tests"]).is_empty());
        assert!(!path["boundary"].as_str().unwrap().is_empty());
    }
    assert_eq!(
        strings(&paths[0]["upstream_tests"]),
        vec![
            "provider_requirements_ignore_system_defaults_but_reject_requirement_changes",
            "provider_requirement_load_errors_reject_input",
            "provider_requirement_changes_reject_inputs_to_existing_threads",
        ]
    );
    assert_eq!(
        strings(&paths[1]["upstream_tests"]),
        vec![
            "permission_rules_require_absolute_paths_and_descendant_subpaths",
            "interior_dot_workspace_glob_fails_closed_for_every_path_convention",
        ]
    );
    assert_eq!(
        strings(&paths[2]["upstream_tests"]),
        vec![
            "bootstrap_gets_resolve_each_fallback_destination_and_preserve_headers",
            "bootstrap_gets_keep_default_responses_without_proxy_retry",
            "bootstrap_gets_honor_disabled_fallback",
            "bootstrap_get_recovers_from_stalled_body_before_cloud_startup_timeout",
            "system_proxy_fallback_config_matches_bootstrap",
            "system_proxy_fallback_honors_feature_requirements",
        ]
    );
    assert!(
        paths[3]["boundary"]
            .as_str()
            .unwrap()
            .contains("cannot establish runtime isolation")
    );
    assert!(
        paths[4]["boundary"]
            .as_str()
            .unwrap()
            .contains("provider/API HTTP separately from turn/start")
    );

    let boundary = &analysis["route_boundary"];
    assert_exact_keys(
        boundary,
        &[
            "failure_projection",
            "no_artifact_execution",
            "no_credentials",
            "no_host_update",
            "no_live_session",
            "no_new_operations",
            "no_provider_prompt",
            "request_overlay",
            "thread_start",
            "thread_start_fields",
            "turn_sandbox_policy_keys",
            "turn_start",
        ],
    );
    assert_eq!(
        strings(&boundary["thread_start_fields"]),
        vec![
            "approvalPolicy=never",
            "sandbox=workspace-write",
            "cwd=<preflight-approved working-resource root>",
            "runtimeWorkspaceRoots=[same root]"
        ]
    );
    assert_eq!(
        strings(&boundary["turn_sandbox_policy_keys"]),
        vec![
            "excludeSlashTmp",
            "excludeTmpdirEnvVar",
            "networkAccess",
            "type",
            "writableRoots"
        ]
    );
    assert!(
        boundary["request_overlay"]
            .as_str()
            .unwrap()
            .contains("windows.allow_mxc")
    );
    assert_eq!(boundary["no_new_operations"], true);
    assert_eq!(boundary["no_provider_prompt"], true);
    assert_eq!(boundary["no_live_session"], true);
    assert_eq!(boundary["no_credentials"], true);
    assert_eq!(boundary["no_artifact_execution"], true);
    assert_eq!(boundary["no_host_update"], true);
    assert_eq!(analysis["residual_gates"].as_array().unwrap().len(), 1);
    assert!(
        analysis["limitations"][0]
            .as_str()
            .unwrap()
            .contains("Contract 036")
    );
    assert!(
        analysis["limitations"][1]
            .as_str()
            .unwrap()
            .contains("runtime isolation")
    );

    let claim = codex_app_server_claim();
    let InterfaceCompatibilityAssessment::Qualified(old) = claim.assess(&version("0.155.1")) else {
        panic!("the previous exact point remains qualified");
    };
    assert_eq!(old.support_status(), InterfaceSupportStatus::Maintained);
    assert!(!claim.assess(&version("0.155.2")).is_permitted());
    for (point, behavior, status) in [
        (
            "0.156.0",
            "codex.app-server.v2.managed-policy-workspace-roots",
            InterfaceSupportStatus::Deprecated,
        ),
        (
            "0.157.0",
            "codex.app-server.v2.managed-network-workspace-roots",
            InterfaceSupportStatus::Deprecated,
        ),
        (
            "0.158.0",
            "codex.app-server.v2.path-alias-workspace-roots",
            InterfaceSupportStatus::Deprecated,
        ),
        (
            "0.159.0",
            "codex.app-server.v2.protected-aws-workspace-roots",
            InterfaceSupportStatus::Maintained,
        ),
        (
            "0.161.0",
            "codex.app-server.v2.protected-aws-workspace-roots",
            InterfaceSupportStatus::Maintained,
        ),
    ] {
        let InterfaceCompatibilityAssessment::Qualified(qualified) = claim.assess(&version(point))
        else {
            panic!("{point} must be qualified");
        };
        assert_eq!(qualified.behavior_revision().as_str(), behavior);
        assert_eq!(qualified.support_status(), status);
    }

    let lifecycle = codex_app_server_lifecycle_claim();
    let InterfaceCompatibilityAssessment::Qualified(lifecycle_qualified) =
        lifecycle.assess(&version("0.161.0"))
    else {
        panic!("app-server lifecycle behavior is qualified at the current point");
    };
    assert_eq!(
        lifecycle_qualified.support_status(),
        InterfaceSupportStatus::Maintained
    );
    assert!(!lifecycle.assess(&version("0.155.2")).is_permitted());

    let exec = codex_exec_claim();
    let InterfaceCompatibilityAssessment::Qualified(exec_qualified) =
        exec.assess(&version("0.161.0"))
    else {
        panic!("exec is qualified separately through current stable");
    };
    assert_eq!(
        exec_qualified.support_status(),
        InterfaceSupportStatus::Maintained
    );
    assert_eq!(
        exec_qualified.behavior_revision().as_str(),
        "codex.exec.jsonl-v1"
    );
}
