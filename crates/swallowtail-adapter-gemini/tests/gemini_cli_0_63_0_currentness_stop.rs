//! Frozen identity and stop evidence for official Gemini CLI ACP 0.62.0–0.63.0.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use swallowtail_adapter_gemini::gemini_cli_acp_claim;
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const IDENTITY: &str = include_str!("fixtures/gemini-cli-acp-0.63.0/identity.json");
const SURFACE: &str = include_str!("fixtures/gemini-cli-acp-0.63.0/surface-ledger.json");
const NPM: &str = include_str!("fixtures/gemini-cli-acp-0.63.0/npm-package-inventory.json");
const PROTOCOL: &str = include_str!("fixtures/gemini-cli-acp-0.63.0/protocol.json");
const PRIOR: &str = include_str!("fixtures/gemini-cli-0.61.0/identity.json");
const VERSIONS: [&str; 3] = ["0.61.0", "0.62.0", "0.63.0"];

fn fixture(body: &str, name: &str) -> Value {
    serde_json::from_str(body).unwrap_or_else(|error| panic!("{name}: {error}"))
}

fn strings(value: &Value, name: &str) -> BTreeSet<String> {
    value
        .as_array()
        .unwrap_or_else(|| panic!("{name} is a list"))
        .iter()
        .map(|item| item.as_str().expect("entry is text").to_owned())
        .collect()
}

fn keys(value: &Value, name: &str) -> BTreeSet<String> {
    value
        .as_object()
        .unwrap_or_else(|| panic!("{name} is an object"))
        .keys()
        .cloned()
        .collect()
}

fn hashes(value: &Value, name: &str) -> BTreeMap<String, String> {
    value
        .as_object()
        .unwrap_or_else(|| panic!("{name} is a map"))
        .iter()
        .map(|(path, digest)| {
            let digest = digest.as_str().expect("digest is text");
            assert_eq!(digest.len(), 64, "{name}.{path} is SHA-256 hex");
            assert!(digest.bytes().all(|byte| byte.is_ascii_hexdigit()));
            (path.clone(), digest.to_owned())
        })
        .collect()
}

#[test]
fn frozen_research_fixture_files_match_their_exact_digests() {
    for (name, body, expected) in [
        (
            "identity",
            IDENTITY,
            "e9a3f26ebdf4e5936e966b3807a2e24416842dfc66f01b7effe02c96595d5fc1",
        ),
        (
            "surface ledger",
            SURFACE,
            "8085a4e0a34e3a2c4f204ed48c2a82d79766ebcb8857b3c34e719ed02cd95f5c",
        ),
        (
            "npm inventory",
            NPM,
            "e4558baecff14077a430c13bbebaf8f9d0ca8482b5210946753e2e2ee4a09297",
        ),
        (
            "protocol",
            PROTOCOL,
            "8888cbb260169470d4f170a8931592ed66d30c3dde677d7c3d98e67473bf13de",
        ),
        (
            "prior identity",
            PRIOR,
            "c5fae058ed6f05a9d0fa457256e03db68df2bac44cc178a05b136b7291ce3f3b",
        ),
    ] {
        assert_eq!(
            format!("{:x}", Sha256::digest(body.as_bytes())),
            expected,
            "{name}"
        );
    }
}

#[test]
fn official_identity_and_complete_npm_inventory_are_frozen() {
    let identity = fixture(IDENTITY, "identity");
    let prior = fixture(PRIOR, "prior identity");
    assert_eq!(identity["family"], "gemini-cli");
    assert_eq!(identity["axis"], "gemini-cli.acp-agent");
    assert_eq!(identity["npm_package"], "@google/gemini-cli");
    assert_eq!(identity["official_channels"]["npm_latest"], "0.63.0");
    assert_eq!(
        identity["official_channels"]["github_latest_stable"],
        "0.63.0"
    );
    assert_eq!(identity["official_channels"]["agreement"], true);
    assert_eq!(identity["previous_claim"]["latest_qualified"], "0.61.0");
    assert_eq!(
        identity["previous_claim"]["published_excluded_interior_points"],
        json!(["0.56.1", "0.59.1"])
    );
    let points = identity["official_points"]
        .as_array()
        .expect("official points are a list");
    assert_eq!(points.len(), VERSIONS.len());
    let exact_identity = [
        (
            "0.61.0",
            "bb523741c7429a44d03e964bc124c7c92df59d5f",
            "c4226f654bb0b35109c4f5ce34535c5addecda47",
            "bc4efa5c925c4430105b552820ed3164bbeffa9dc227990fc922f954733bcd7d",
            "0b6e283ae88682b0e27e8ef85a608ab74807a1513dc0c053e5aa80d5b80b29ab",
            "917e0ac08eb3ef2048910ecc84d4da672ad0c58d82bb2eaa1d6f9e18af80241d",
            "2a01f6f000a7ac060ff0c8e945c650c94cd05ddae52ad19ee0ada05a9bdba3ea",
            "829738c00cab5a73b3ed01ce874c7ba79064fbe5869ad821517cba24f778dce2",
        ),
        (
            "0.62.0",
            "b460678f3db508407554afd604cc9d6635becb2a",
            "65ae43a5a0bf3670294be027ffe40eb5b0c5f18c",
            "2276032b1c33d2b828b1cf197e52f48e74b0a395326763ff01a80d97d0fbc0c3",
            "ef1d1bd9ee5aaae37ebcfb601b56659e3b16c5b258b21c138109c541e487ade7",
            "18d3955d07457723e5f9b24ff2d7622081b855ea8591d00a977b89a2089626f0",
            "68c199ca0c352ee3107e33d121faea24486f4416534224994357b567576e431c",
            "050d9e3a53e6fc9cc2c00cd7d6771d72077b8fa2c0d1ec925ce3c353ee94b55b",
        ),
        (
            "0.63.0",
            "573846625af9e93b3b968e0e0b86bb093a4c9b16",
            "a6a023123538ba75eb8412528b9942d0de8917e3",
            "97a6edfc10645463b517f0518d46a8c72efbdc12558a9a948607f726284a0420",
            "5aee9ecb65b0b821e36a12f6e9e837301636de1a849a5a8bcc9c2b2620bbd72e",
            "75903470f15719bc061df6c2792cc55274494f7c25efdb7a864c50f5913bf917",
            "c9d66e50a0fe098a32486360308e558c472809da276caf2aac2769d8f10571c2",
            "61babb895286b69a6112d7eab3f61aa0e2370ad6c1693a08badae8f64d9dd287",
        ),
    ];
    for (point, version) in points.iter().zip(VERSIONS) {
        assert_eq!(point["version"], version);
        assert_eq!(point["npm_latest_at_observation"], version == "0.63.0");
        assert_eq!(point["acp_sdk_pin"], "0.16.1");
        assert_eq!(point["downloaded_artifact_executed"], false);
        for field in [
            "npm_integrity",
            "npm_shasum",
            "npm_tarball_sha256",
            "npm_package_json_sha256",
            "npm_bin_entry_sha256",
            "github_source_archive_sha256",
            "gemini_cli_bundle_zip_sha256",
            "darwin_arm64_unsigned_zip_sha256",
        ] {
            assert!(point[field].as_str().is_some(), "{version}.{field}");
        }
    }
    for (point, expected) in points.iter().zip(exact_identity) {
        let (version, commit, tree, npm, runtime, source, bundle, darwin) = expected;
        assert_eq!(point["version"], version);
        assert_eq!(point["npm_tarball_sha256"], npm);
        assert_eq!(point["npm_bin_entry_sha256"], runtime);
        assert_eq!(point["github_commit"], commit);
        assert_eq!(point["github_tree"], tree);
        assert_eq!(point["github_source_archive_sha256"], source);
        assert_eq!(point["gemini_cli_bundle_zip_sha256"], bundle);
        assert_eq!(point["darwin_arm64_unsigned_zip_sha256"], darwin);
    }
    assert_eq!(
        points[0]["npm_tarball_sha256"],
        prior["official"]["npm_tarball_sha256"]
    );
    assert_eq!(
        points[0]["github_source_archive_sha256"],
        prior["official"]["github_source_archive_sha256"]
    );
    assert_eq!(
        points[0]["npm_bin_entry_sha256"],
        identity["host_observation"]["bundle_sha256"]
    );
    assert_eq!(identity["host_observation"]["version"], "0.61.0");
    assert_eq!(identity["host_observation"]["host_install_changed"], false);

    let npm = fixture(NPM, "npm inventory");
    assert_eq!(npm["compared"], json!(VERSIONS));
    let file_hashes = &npm["file_hashes"];
    let mut package_maps = BTreeMap::new();
    for version in VERSIONS {
        let map = hashes(&file_hashes[version], version);
        assert_eq!(map.len(), 449);
        package_maps.insert(version, map);
    }
    for (
        hop,
        from,
        to,
        expected_added,
        expected_removed,
        expected_changed,
        expected_same,
        changed_files,
    ) in [
        (
            "from_0.61.0_to_0.62.0",
            "0.61.0",
            "0.62.0",
            50,
            50,
            6,
            393,
            vec![
                "bundle/docs/changelogs/index.md",
                "bundle/docs/changelogs/latest.md",
                "bundle/docs/changelogs/preview.md",
                "bundle/docs/get-started/authentication.mdx",
                "bundle/gemini.js",
                "package.json",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>(),
        ),
        (
            "from_0.62.0_to_0.63.0",
            "0.62.0",
            "0.63.0",
            48,
            48,
            5,
            396,
            vec![
                "bundle/docs/changelogs/index.md",
                "bundle/docs/changelogs/latest.md",
                "bundle/docs/changelogs/preview.md",
                "bundle/gemini.js",
                "package.json",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>(),
        ),
    ] {
        let before = &package_maps[from];
        let after = &package_maps[to];
        let added: BTreeSet<_> = after
            .keys()
            .filter(|path| !before.contains_key(*path))
            .cloned()
            .collect();
        let removed: BTreeSet<_> = before
            .keys()
            .filter(|path| !after.contains_key(*path))
            .cloned()
            .collect();
        let common: BTreeSet<_> = before
            .keys()
            .filter(|path| after.contains_key(*path))
            .cloned()
            .collect();
        let changed: BTreeSet<_> = common
            .iter()
            .filter(|path| before[*path] != after[*path])
            .cloned()
            .collect();
        let identical: BTreeSet<_> = common.difference(&changed).cloned().collect();
        let record = &npm["hops"][hop];
        assert_eq!(
            record["added"].as_array().map(Vec::len),
            Some(expected_added)
        );
        assert_eq!(
            record["removed"].as_array().map(Vec::len),
            Some(expected_removed)
        );
        assert_eq!(
            record["changed"].as_array().map(Vec::len),
            Some(expected_changed)
        );
        assert_eq!(
            record["identical"].as_array().map(Vec::len),
            Some(expected_same)
        );
        assert_eq!(strings(&record["added"], hop), added);
        assert_eq!(strings(&record["removed"], hop), removed);
        assert_eq!(strings(&record["changed"], hop), changed);
        assert_eq!(strings(&record["identical"], hop), identical);
        assert_eq!(changed, changed_files.into_iter().collect());
    }
}

#[test]
fn complete_source_trees_and_changed_behavior_files_are_classified() {
    let surface = fixture(SURFACE, "surface ledger");
    assert_eq!(surface["compared_points"], json!(VERSIONS));
    assert_eq!(
        surface["source_file_counts"],
        json!({"0.61.0": 3004, "0.62.0": 3014, "0.63.0": 3018})
    );
    let trees = &surface["source_tree_sha256"];
    let mut source_maps = BTreeMap::new();
    for version in VERSIONS {
        let map = hashes(&trees[version], version);
        assert_eq!(map.len(), surface["source_file_counts"][version]);
        source_maps.insert(version, map);
        assert_eq!(
            surface["source_non_regular_paths"][version],
            json!(["docs/CONTRIBUTING.md"])
        );
    }

    let expected_hops = [
        ("0.61.0..0.62.0", "0.61.0", "0.62.0", 88, 10, 0, 78),
        ("0.62.0..0.63.0", "0.62.0", "0.63.0", 91, 4, 0, 87),
    ];
    for (hop, from, to, count, added_count, removed_count, modified_count) in expected_hops {
        let before = &source_maps[from];
        let after = &source_maps[to];
        let before_keys: BTreeSet<_> = before.keys().cloned().collect();
        let after_keys: BTreeSet<_> = after.keys().cloned().collect();
        let added: BTreeSet<_> = after_keys.difference(&before_keys).cloned().collect();
        let removed: BTreeSet<_> = before_keys.difference(&after_keys).cloned().collect();
        let common: BTreeSet<_> = before_keys.intersection(&after_keys).cloned().collect();
        let modified: BTreeSet<_> = common
            .iter()
            .filter(|path| before[*path] != after[*path])
            .cloned()
            .collect();
        let changed = added
            .union(&removed)
            .cloned()
            .chain(modified.iter().cloned())
            .collect::<BTreeSet<_>>();
        let record = &surface["changed_paths"][hop];
        assert_eq!(record["count"], count);
        assert_eq!(record["added"].as_array().map(Vec::len), Some(added_count));
        assert_eq!(
            record["removed"].as_array().map(Vec::len),
            Some(removed_count)
        );
        assert_eq!(
            record["modified"].as_array().map(Vec::len),
            Some(modified_count)
        );
        assert_eq!(strings(&record["added"], hop), added);
        assert_eq!(strings(&record["removed"], hop), removed);
        assert_eq!(strings(&record["modified"], hop), modified);
        assert_eq!(strings(&record["paths"], hop), changed);
    }

    let expected_classifications = [
        (
            "0.61.0..0.62.0",
            [
                "packages/cli/src/acp/acpSession.ts",
                "packages/core/src/tools/mcp-tool.ts",
                "packages/core/src/tools/tool-registry.ts",
                "packages/core/src/tools/shell.ts",
                "packages/core/src/services/shellExecutionService.ts",
                "packages/core/src/services/executionLifecycleService.ts",
                "packages/core/src/utils/terminalSerializer.ts",
                "packages/cli/src/patches/http-proxy-agent.ts",
                "packages/cli/src/patches/https-proxy-agent.ts",
                "packages/core/src/code_assist/oauth-credential-storage.ts",
                "packages/core/src/code_assist/oauth2.ts",
                "packages/core/src/code_assist/setup.ts",
                "packages/core/src/mcp/token-storage/keychain-token-storage.ts",
            ]
            .map(str::to_owned)
            .into_iter()
            .collect::<BTreeSet<_>>(),
        ),
        (
            "0.62.0..0.63.0",
            [
                "packages/cli/src/acp/acpSessionManager.ts",
                "packages/core/src/policy/policy-engine.ts",
                "packages/core/src/safety/built-in.ts",
                "packages/core/src/utils/paths.ts",
                "packages/core/src/tools/read-file.ts",
                "packages/core/src/tools/shell.ts",
                "packages/core/src/tools/list-mcp-resources.ts",
                "packages/core/src/tools/read-mcp-resource.ts",
                "packages/core/src/tools/web-search.ts",
                "packages/core/src/scheduler/tool-executor.ts",
                "packages/core/src/utils/fileUtils.ts",
                "packages/core/src/utils/tool-utils.ts",
                "packages/core/src/utils/constants.ts",
                "packages/core/src/agents/local-executor.ts",
                "packages/core/src/context/chatCompressionService.ts",
                "packages/core/src/core/client.ts",
                "packages/core/src/services/chatRecordingService.ts",
                "packages/core/src/utils/security.ts",
                "packages/core/src/utils/gitUtils.ts",
                "packages/cli/src/config/settings.ts",
                "packages/cli/src/config/mcp/mcpServerEnablement.ts",
                "packages/cli/src/utils/processUtils.ts",
                "packages/cli/src/utils/relaunch.ts",
                "packages/cli/index.ts",
                "packages/cli/src/utils/commentJson.ts",
                "packages/core/src/prompts/snippets.ts",
            ]
            .map(str::to_owned)
            .into_iter()
            .collect::<BTreeSet<_>>(),
        ),
    ];
    let classifications = &surface["relevant_changed_sources"];
    assert_eq!(
        keys(classifications, "classification hops"),
        expected_classifications
            .iter()
            .map(|(hop, _)| (*hop).to_owned())
            .collect()
    );
    for (hop, expected_files) in expected_classifications {
        let classified = &classifications[hop];
        assert_eq!(keys(classified, hop), expected_files);
        let changed = strings(&surface["changed_paths"][hop]["paths"], hop);
        for path in expected_files {
            assert!(changed.contains(&path), "{path} is in {hop}");
            assert!(
                classified[&path]
                    .as_str()
                    .is_some_and(|value| !value.is_empty())
            );
        }
    }

    let selected = &surface["selected_acp_sources"];
    assert_eq!(
        keys(selected, "selected ACP source set"),
        [
            "packages/cli/src/acp/acpCommandHandler.ts",
            "packages/cli/src/acp/acpRpcDispatcher.ts",
            "packages/cli/src/acp/acpSessionManager.ts",
            "packages/cli/src/acp/acpSession.ts",
            "packages/cli/src/acp/acpFileSystemService.ts",
            "packages/cli/src/acp/acpStdioTransport.ts",
            "packages/cli/src/acp/acpUtils.ts",
            "packages/core/src/tools/mcp-client.ts",
        ]
        .map(str::to_owned)
        .into_iter()
        .collect()
    );
    assert_eq!(
        surface["selected_acp_sdk_pin"],
        json!({"0.61.0":"0.16.1", "0.62.0":"0.16.1", "0.63.0":"0.16.1"})
    );
    assert_eq!(
        surface["selected_acp_sources"]["packages/cli/src/acp/acpSession.ts"]["0.61.0"],
        "f21c78e7edc1b17972b3d3a94ea507788fcc67fdd1bb0fe857cb97218be63e3b"
    );
    assert_eq!(
        surface["selected_acp_sources"]["packages/cli/src/acp/acpSession.ts"]["0.62.0"],
        "df677d8a3bab686c97870ac3c9e88aaaa7ff880abd59dfdfb2016b098b52d463"
    );
    assert_eq!(
        surface["selected_acp_sources"]["packages/cli/src/acp/acpSessionManager.ts"]["0.63.0"],
        "a78d6a5ba38dcbe1f6939507936d5de2ac3ac4501b8191818ea6db22646aad4f"
    );
    for (path, before, after) in [
        (
            "packages/cli/src/acp/acpSessionManager.ts",
            "63e38dfcfe035a317acc9e2943810b765e7403ecb6383597633394e8ff214f1e",
            "a78d6a5ba38dcbe1f6939507936d5de2ac3ac4501b8191818ea6db22646aad4f",
        ),
        (
            "packages/core/src/policy/policy-engine.ts",
            "3483535ed1d106e75cf1953da7255815f336003ad91a8e841db5df516592cfb1",
            "3b7118ceadaa36589f64def7669a503d64bd805fbff696077f5f7929361dfad2",
        ),
        (
            "packages/core/src/safety/built-in.ts",
            "e59fb30b90906aac439fe6e790dd0613c26b21ef6ec7e29af1c86f01815a206d",
            "a3d88b1b5a9289194edf7cdcb59964e4ba99c8eb4a4b100eca7bb64c6f61657e",
        ),
        (
            "packages/core/src/tools/read-file.ts",
            "51c2a2e2c78519e09af956485bdd2c14b3e261f9bcd2cdcbdc53d0db84ac17c8",
            "5377927463fa737d276852097ad63a20357b19b51f3458c4252028b6019033de",
        ),
        (
            "packages/core/src/utils/paths.ts",
            "d86215a13c63445b0186b82c346ceca3810907268394b9072b5d110f005cb987",
            "4676b4a937bf61a8b87c6f3f6f4123fbe23da20837c8f7d67b41f8847b333571",
        ),
        (
            "packages/core/src/tools/shell.ts",
            "d1aea4b1e3fb3724ad637985aebf0f8a2d2fcfeb13cfa11932b8681ed617e139",
            "981d96965fbc84fab420f30f1b265cee15c7b4b0486fdde42324021a95f22be4",
        ),
        (
            "packages/core/src/scheduler/tool-executor.ts",
            "a70086b7ecda8edb1701ce1918410d5471f78bd84d1cee5017af808652fec593",
            "5879f47abe2b64fc6ea1ca361b4502f39a7e597bdcfa0e20450b4226d6031f58",
        ),
        (
            "packages/core/src/utils/tool-utils.ts",
            "d28e32c72c3afd677795c81e5d1bfabc804eeba7872db5832871b5aa84b76403",
            "fb7fb4e70c7b05bed4fe65636cb9e36153aa3e90c5d24fa74d489179a8884c0b",
        ),
    ] {
        assert_eq!(surface["source_tree_sha256"]["0.62.0"][path], before);
        assert_eq!(surface["source_tree_sha256"]["0.63.0"][path], after);
    }
}

#[test]
fn historical_stop_is_resolved_only_for_acp_and_live_mcp_stays_exact() {
    let protocol = fixture(PROTOCOL, "protocol");
    assert_eq!(protocol["official_version"], "0.63.0");
    assert_eq!(protocol["acp_sdk"], "@agentclientprotocol/sdk@0.16.1");
    assert_eq!(protocol["wire_version"], 1);
    assert_eq!(
        protocol["permission_boundary"]["consumer_visible_narrowing"],
        true
    );
    assert_eq!(protocol["claim"]["latest_qualified"], "0.61.0");
    assert_eq!(protocol["claim"]["0.62.0"], "unverified_newer");
    assert_eq!(protocol["claim"]["0.63.0"], "unverified_newer");
    assert_eq!(protocol["claim"]["claim_changed"], false);
    assert_eq!(
        protocol["http_mcp"]["live_honouring_exact_version"],
        "0.61.0"
    );
    assert_eq!(protocol["http_mcp"]["live_honouring_transferred"], false);
    assert_eq!(protocol["downloaded_artifacts_executed"], false);

    let claim = gemini_cli_acp_claim();
    let baseline = InterfaceVersion::new("0.61.0").expect("qualified version");
    assert!(claim.supports(&baseline));
    for (value, revision) in [
        ("0.62.0", "gemini-cli.acp.v0.62.0-tool-updates"),
        ("0.63.0", "gemini-cli.acp.v0.63.0-restricted-files"),
    ] {
        let version = InterfaceVersion::new(value).expect("stable version");
        let InterfaceCompatibilityAssessment::Qualified(matched) = claim.assess(&version) else {
            panic!("{value} has a qualified ACP behavior segment");
        };
        assert_eq!(matched.behavior_revision().as_str(), revision);
    }
    for value in ["0.56.1", "0.59.1"] {
        let version = InterfaceVersion::new(value).expect("excluded version");
        assert_eq!(
            claim.assess(&version),
            InterfaceCompatibilityAssessment::Incompatible
        );
    }
}
