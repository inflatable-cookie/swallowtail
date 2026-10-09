use serde_json::{Value, json};
use swallowtail_adapter_claude_agent::{
    CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, claude_code_headless_claim,
};
use swallowtail_core::{InterfaceCompatibilityAssessment, InterfaceVersion};

const IDENTITY: &str = include_str!("fixtures/claude-code-2.1.294/identity.json");
const INVENTORY: &str = include_str!("fixtures/claude-code-2.1.294/dist-inventory.json");
const PROTOCOL: &str = include_str!("fixtures/claude-code-2.1.294/protocol.json");
const HOOK_SEMANTICS: &str =
    include_str!("fixtures/claude-code-2.1.294/hook-safety-semantics.json");
const PRIOR_IDENTITY: &str = include_str!("fixtures/claude-code-2.1.293/identity.json");
const PRIOR_INVENTORY: &str = include_str!("fixtures/claude-code-2.1.293/dist-inventory.json");
const PRIOR_PROTOCOL: &str = include_str!("fixtures/claude-code-2.1.293/protocol.json");

fn fixture(value: &str) -> Value {
    serde_json::from_str(value).expect("frozen JSON is valid")
}

fn object_keys(value: &Value) -> Vec<&str> {
    value
        .as_object()
        .expect("fixture object")
        .keys()
        .map(String::as_str)
        .collect()
}

fn versions() -> Vec<String> {
    (281..=294).map(|patch| format!("2.1.{patch}")).collect()
}

fn hops() -> Vec<String> {
    versions()
        .windows(2)
        .map(|pair| format!("{}_to_{}", pair[0], pair[1]))
        .collect()
}

#[test]
fn historical_stop_is_preserved_and_current_claim_is_adapted() {
    let identity = fixture(IDENTITY);
    assert_eq!(identity["axis"], "claude-code.headless-stream-json");
    assert_eq!(identity["previous_ceiling"], "2.1.281");
    assert_eq!(identity["official_latest_at_observation"], "2.1.294");
    assert_eq!(
        identity["npm"]["dist_tags"],
        json!({
            "latest": "2.1.294",
            "next": "2.1.295",
            "stable": "2.1.286"
        })
    );
    assert_eq!(
        identity["npm"]["other_channels"]["next"]["version"],
        "2.1.295"
    );
    assert_eq!(
        identity["npm"]["other_channels"]["next"]["qualified_as_stable"],
        false
    );
    assert_eq!(identity["github"]["latest_non_prerelease"], "v2.1.294");
    assert_eq!(
        identity["github"]["tag_commits"]["2.1.294"],
        "71cdddec623889d38af14b7a489670a03186f659"
    );
    assert_eq!(
        identity["published_hops"],
        json!(versions().into_iter().skip(1).collect::<Vec<_>>())
    );
    assert_eq!(
        object_keys(&identity["github"]["tag_commits"]),
        versions().iter().map(String::as_str).collect::<Vec<_>>()
    );
    assert_eq!(identity["first_unpublished_after_latest"], "2.1.296");
    assert_eq!(identity["npm_2.1.296_http_status"], 404);
    assert_eq!(identity["github_tag_v2.1.296_present"], false);
    assert_eq!(identity["identity_decision"], "stop");
    assert_eq!(identity["segment_shape"], "stop");
    let stops = identity["stops"].as_array().expect("frozen stop list");
    assert_eq!(
        stops
            .iter()
            .map(|stop| stop["version"].as_str().unwrap())
            .collect::<Vec<_>>(),
        ["2.1.287", "2.1.290", "2.1.294"]
    );

    let prior_identity = fixture(PRIOR_IDENTITY);
    assert_eq!(
        identity["claim_at_observation"],
        prior_identity["claim_at_observation"]
    );
    let claim = claude_code_headless_claim();
    assert_eq!(CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION, "2.1.294");
    assert_eq!(claim.latest_qualified().as_str(), "2.1.294");
    assert!(matches!(
        claim.assess(&InterfaceVersion::new("2.1.294").unwrap()),
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.behavior_revision().as_str() == "claude-code.headless.stream-json.v2"
    ));
    assert!(matches!(
        claim.assess(&InterfaceVersion::new("2.1.295").unwrap()),
        InterfaceCompatibilityAssessment::UnverifiedNewer(newer)
            if newer.behavior_revision().as_str() == "claude-code.headless.stream-json.v2"
    ));
}

#[test]
fn published_package_inventories_retain_the_full_prefix_and_freeze_294() {
    let inventory = fixture(INVENTORY);
    let prior = fixture(PRIOR_INVENTORY);
    let versions = versions();
    assert_eq!(inventory["compared"], json!(versions));
    assert_eq!(
        inventory["compared"].as_array().unwrap()[..13],
        prior["compared"].as_array().unwrap()[..]
    );
    assert_eq!(
        object_keys(&inventory["packages"]),
        [
            "claude-code",
            "claude-code-darwin-arm64",
            "claude-code-linux-x64"
        ]
    );

    let wrapper_files = [
        "LICENSE.md",
        "README.md",
        "bin/claude.exe",
        "cli-wrapper.cjs",
        "install.cjs",
        "package.json",
        "sdk-tools.d.ts",
    ];
    let platform_files = ["LICENSE.md", "README.md", "claude", "package.json"];
    for package in [
        "claude-code",
        "claude-code-darwin-arm64",
        "claude-code-linux-x64",
    ] {
        assert_eq!(
            object_keys(&inventory["packages"][package]),
            versions.iter().map(String::as_str).collect::<Vec<_>>()
        );
        assert_eq!(
            object_keys(&inventory["file_delta"][package]),
            hops().iter().map(String::as_str).collect::<Vec<_>>()
        );
        for version in &versions[..13] {
            assert_eq!(
                inventory["packages"][package][version],
                prior["packages"][package][version]
            );
        }
        for (index, hop) in hops()[..12].iter().enumerate() {
            assert_eq!(
                inventory["file_delta"][package][hop], prior["file_delta"][package][hop],
                "{package} {index}"
            );
        }
        let record = &inventory["packages"][package]["2.1.294"];
        let expected_files: &[&str] = if package == "claude-code" {
            &wrapper_files
        } else {
            &platform_files
        };
        assert_eq!(
            inventory["package_file_counts"][package]["2.1.294"],
            expected_files.len()
        );
        let mut files = object_keys(&record["files"]);
        files.sort_unstable();
        let mut sizes = object_keys(&record["file_sizes_bytes"]);
        sizes.sort_unstable();
        assert_eq!(files.as_slice(), expected_files);
        assert_eq!(sizes.as_slice(), expected_files);
        let expected = match package {
            "claude-code" => json!({
                "published_at": "2026-10-08T03:42:57.084Z",
                "tarball_url": "https://registry.npmjs.org/@anthropic-ai/claude-code/-/claude-code-2.1.294.tgz",
                "tarball_integrity": "sha512-n5JvkRu2cd8zc4bINO5myqVaEKid3ttD8BVh+LT4W6Jmwww5TyKBLH8RzLhk/hS/cpEtwF8onh8XunjCZYerUg==",
                "tarball_sha256": "a6292c46ed942bfa6b151ad4da5938484ce826aee16423edf36b93f48f6ec099",
                "unpacked_size_bytes": 188525,
                "files": {
                    "LICENSE.md": "8ce94b9478bb9868f9641f818e06cd722fbe55d4c22e2d2ed11971b20146173a",
                    "README.md": "da7cf15ce4e35bad6a107acd6a36b4fe052083068bb9e564b1aa3f2078b5d0ce",
                    "bin/claude.exe": "6d7abae055d3b598281300a6c835086dec81bf3048f8a2294c5d3e50c8830d7b",
                    "cli-wrapper.cjs": "61ad63033d9c8155d5e60a29f45dc4665afa07631c0b108e62cc83bf45ba490e",
                    "install.cjs": "32a7b2429d7447c5aa043e383bcf9a72e0b233dff66336ea4b5b32531da981ad",
                    "package.json": "e1607276a979d6d08901f1a48eb4b0f487f1920c7ae48cc0a7689b51d86d9f22",
                    "sdk-tools.d.ts": "d850d83ecd9e5f92be6e1d98b54b228e68ce0cc32767391f6e20573a4870e024"
                },
                "file_sizes_bytes": {
                    "LICENSE.md": 147,
                    "README.md": 2037,
                    "bin/claude.exe": 500,
                    "cli-wrapper.cjs": 4997,
                    "install.cjs": 8375,
                    "package.json": 1476,
                    "sdk-tools.d.ts": 170993
                }
            }),
            "claude-code-darwin-arm64" => json!({
                "published_at": "2026-10-08T03:42:02.637Z",
                "tarball_url": "https://registry.npmjs.org/@anthropic-ai/claude-code-darwin-arm64/-/claude-code-darwin-arm64-2.1.294.tgz",
                "tarball_integrity": "sha512-frtT/oUDIYgiFRRNIDcPQuteofIs+zQgV5ibO2F9pJiHiiIQ2b9iERj7Fyw7NpEJh2TjkmBfhOGNvFjTkNYm1g==",
                "tarball_sha256": "846fa18ec1924e0c0fc0948dc5d4c31c667986f7f5c84b30eeca1f1056c90ae2",
                "unpacked_size_bytes": 236331185,
                "files": {
                    "LICENSE.md": "8ce94b9478bb9868f9641f818e06cd722fbe55d4c22e2d2ed11971b20146173a",
                    "README.md": "26170f86550171a90850a39d867b0487d0d218e100ab5552e51ba4f1159b9a0c",
                    "claude": "def0d15e64dd7d89621f88d28214f885b1c38b0ddd69762fb8593e34915d6d53",
                    "package.json": "5b697db0af4cceebd60a33cfc0e7417ad654dce526b6b98ac1239f75b6d69027"
                },
                "file_sizes_bytes": {
                    "LICENSE.md": 147,
                    "README.md": 153,
                    "claude": 236330608,
                    "package.json": 277
                }
            }),
            "claude-code-linux-x64" => json!({
                "published_at": "2026-10-08T03:47:18.560Z",
                "tarball_url": "https://registry.npmjs.org/@anthropic-ai/claude-code-linux-x64/-/claude-code-linux-x64-2.1.294.tgz",
                "tarball_integrity": "sha512-PGRIYkEyXxDMcosvkidWA815Y3dnEmZUrTdydabU7OPsh0Cbxnj+ej87sHFmaWOLzlpQTmi2AMUsdEHfsh3Adg==",
                "tarball_sha256": "0955a56fc80a587dd2709ab7caa25b10a9b453def5fda6cf92754de0c6d9046d",
                "unpacked_size_bytes": 252755714,
                "files": {
                    "LICENSE.md": "8ce94b9478bb9868f9641f818e06cd722fbe55d4c22e2d2ed11971b20146173a",
                    "README.md": "ab4b9d72c3aa1ac7357e8eeb8b77c62e308e9af01ac0ecdb181f3bf1e57e25d5",
                    "claude": "27122ca7b624f537546fbef35b80c66370d974ff258f3d9b10ac50bb8771f262",
                    "package.json": "2a102dfa34beb3f43a7022da0da9f76fd7e8a55f9d93be641ece4c6e28d0a0c9"
                },
                "file_sizes_bytes": {
                    "LICENSE.md": 147,
                    "README.md": 150,
                    "claude": 252755128,
                    "package.json": 289
                }
            }),
            _ => unreachable!("known package name"),
        };
        for field in [
            "published_at",
            "tarball_url",
            "tarball_integrity",
            "tarball_sha256",
            "unpacked_size_bytes",
            "files",
            "file_sizes_bytes",
        ] {
            assert_eq!(record[field], expected[field], "{package} {field}");
        }
    }

    assert_eq!(
        inventory["file_delta"]["claude-code"]["2.1.293_to_2.1.294"],
        json!({
            "added": [],
            "removed": [],
            "changed": ["package.json"],
            "identical": ["LICENSE.md", "README.md", "bin/claude.exe", "cli-wrapper.cjs", "install.cjs", "sdk-tools.d.ts"]
        })
    );
    for package in ["claude-code-darwin-arm64", "claude-code-linux-x64"] {
        assert_eq!(
            inventory["file_delta"][package]["2.1.293_to_2.1.294"],
            json!({
                "added": [],
                "removed": [],
                "changed": ["claude", "package.json"],
                "identical": ["LICENSE.md", "README.md"]
            }),
            "{package}"
        );
    }
}

#[test]
fn historical_protocol_snapshot_preserves_the_original_hook_stop() {
    let protocol = fixture(PROTOCOL);
    let prior = fixture(PRIOR_PROTOCOL);
    let hops = protocol["per_hop_classification"]
        .as_array()
        .expect("hop ledger");
    let prior_hops = prior["per_hop_classification"]
        .as_array()
        .expect("prior hop ledger");
    assert_eq!(hops.len(), 13);
    assert_eq!(&hops[..12], prior_hops.as_slice());
    let latest = hops.last().unwrap();
    assert_eq!(latest["hop"], "2.1.293_to_2.1.294");
    assert_eq!(latest["selected_mapped_change"], true);
    assert_eq!(
        latest["decision"],
        "stop-before-claim; additional operator-approved adaptation required"
    );
    assert_eq!(
        latest["changed_files"]
            .as_array()
            .unwrap()
            .iter()
            .map(|file| file["path"].as_str().unwrap())
            .collect::<Vec<_>>(),
        [
            "claude-code/package.json",
            "claude-code-darwin-arm64/claude",
            "claude-code-darwin-arm64/package.json",
            "claude-code-linux-x64/claude",
            "claude-code-linux-x64/package.json"
        ]
    );
    assert_eq!(
        latest["release_note_review"]
            .as_array()
            .unwrap()
            .iter()
            .map(|review| review["surface"].as_str().unwrap())
            .collect::<Vec<_>>(),
        [
            "configured prompt and agent hooks",
            "prompt hooks on Stop and SubagentStop"
        ]
    );
    let review = latest["release_note_review"].as_array().unwrap();
    assert_eq!(
        review[0]["classification"],
        "selected safety behavior candidate outside approved adaptation"
    );
    assert_eq!(
        review[1]["classification"],
        "selected lifecycle behavior candidate outside approved adaptation"
    );
    assert_eq!(protocol["decision"], "stop");
    assert_eq!(
        protocol["static_string_inspection"]["versions"],
        json!(versions())
    );
    assert_eq!(protocol["static_string_inspection"]["binaries_checked"], 28);
    assert_eq!(protocol["static_string_inspection"]["missing"], json!([]));
    assert_eq!(protocol["downloaded_artifacts_executed"], false);
    assert_eq!(protocol["provider_prompt_sent"], false);
    assert_eq!(protocol["live_session"], false);
    assert_eq!(protocol["host_install_changed"], false);
}

#[test]
fn selected_hook_semantics_are_bound_to_both_exact_runtime_hashes() {
    let semantics = fixture(HOOK_SEMANTICS);
    let inventory = fixture(INVENTORY);
    assert_eq!(semantics["version"], "2.1.294");
    assert_eq!(
        semantics["evidence"],
        "static inspection of the exact published Linux x64 and Darwin arm64 executables; no executable was run"
    );
    assert_eq!(
        semantics["artifacts"],
        json!({
            "linux_x64_sha256": "27122ca7b624f537546fbef35b80c66370d974ff258f3d9b10ac50bb8771f262",
            "darwin_arm64_sha256": "def0d15e64dd7d89621f88d28214f885b1c38b0ddd69762fb8593e34915d6d53"
        })
    );
    assert_eq!(
        semantics["artifacts"]["linux_x64_sha256"],
        inventory["packages"]["claude-code-linux-x64"]["2.1.294"]["files"]["claude"]
    );
    assert_eq!(
        semantics["artifacts"]["darwin_arm64_sha256"],
        inventory["packages"]["claude-code-darwin-arm64"]["2.1.294"]["files"]["claude"]
    );
    assert_eq!(
        semantics["prompt_and_agent_hooks"]["instruction_prompt_decision"],
        "requires ok and reason; ok=false selects a blocking outcome"
    );
    assert_eq!(
        semantics["prompt_and_agent_hooks"]["pre_tool_use_deny"],
        "permissionDecision=deny selects a blocking error before tool dispatch"
    );
    assert_eq!(
        semantics["prompt_and_agent_hooks"]["pre_tool_use_rewrite"],
        "a hook-updated input is passed through a subsequent safety and permission decision; a resulting deny blocks the call"
    );
    assert_eq!(
        semantics["stop_hooks"]["events"],
        json!(["Stop", "SubagentStop"])
    );
    assert_eq!(
        semantics["stop_hooks"]["active_reentry"],
        "stop_hook_active=true is treated as successful by the stop-condition evaluation"
    );
    assert_eq!(
        semantics["stop_hooks"]["repeated_blocks"],
        "uses CLAUDE_CODE_STOP_HOOK_BLOCK_CAP or defaults to 8; a positive cap overrides after the configured count"
    );
    assert_eq!(semantics["limits"]["source_to_runtime_claim"], false);
    assert_eq!(semantics["limits"]["release_note_only_inference"], false);
    assert_eq!(semantics["limits"]["vendor_artifact_execution"], false);
    assert_eq!(
        semantics["limits"]["provider_prompt_or_live_session"],
        false
    );
}
