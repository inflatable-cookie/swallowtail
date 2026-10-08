//! Identity and selected-surface evidence for official Mistral Vibe `2.26.0`.
//!
//! The complete PyPI wheel trees and every changed selected-input path are
//! frozen in the fixtures. Downloaded upstream artifacts were inspected
//! statically and never executed.

use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

const IDENTITY: &str = include_str!("fixtures/mistral-vibe-headless-2.26.0/identity.json");
const LEDGER: &str = include_str!("fixtures/mistral-vibe-headless-2.26.0/surface-ledger.json");
const INVENTORY: &str = include_str!("fixtures/mistral-vibe-headless-2.26.0/dist-inventory.json");

const PUBLISHED_VERSIONS: [&str; 5] = ["2.25.4", "2.25.5", "2.25.7", "2.25.8", "2.26.0"];
const SELECTED_INPUT_PATHS: [&str; 78] = [
    "vibe/_experimental_harness.py",
    "vibe/app_server/_runtime.py",
    "vibe/app_server/events.py",
    "vibe/app_server/models.py",
    "vibe/app_server/session.py",
    "vibe/cli/_rust.py",
    "vibe/cli/cli.py",
    "vibe/cli/entrypoint.py",
    "vibe/cli/launcher.py",
    "vibe/cli/programmatic.py",
    "vibe/core/agent_loop/__init__.py",
    "vibe/core/agent_loop/_loop.py",
    "vibe/core/agent_loop_hooks.py",
    "vibe/core/agents/install.py",
    "vibe/core/agents/manager.py",
    "vibe/core/agents/models.py",
    "vibe/core/auth/mcp_oauth.py",
    "vibe/core/checkpoints/checkpointer.py",
    "vibe/core/config/__init__.py",
    "vibe/core/config/_defaults.py",
    "vibe/core/config/builder.py",
    "vibe/core/config/fingerprint.py",
    "vibe/core/config/layers/_base.py",
    "vibe/core/config/mcp_servers.py",
    "vibe/core/config/models.py",
    "vibe/core/config/patch.py",
    "vibe/core/config/vibe_schema.py",
    "vibe/core/experiments/_constants.py",
    "vibe/core/experiments/active.py",
    "vibe/core/experiments/cache.py",
    "vibe/core/experiments/session.py",
    "vibe/core/git/repo.py",
    "vibe/core/hooks/config.py",
    "vibe/core/llm/utility_completion.py",
    "vibe/core/middleware.py",
    "vibe/core/paths/__init__.py",
    "vibe/core/paths/_vibe_home.py",
    "vibe/core/prompts/project_context.md",
    "vibe/core/session/image_snapshot.py",
    "vibe/core/session/last_session_pointer.py",
    "vibe/core/session/resume_sessions.py",
    "vibe/core/session/saved_sessions.py",
    "vibe/core/session/session_index.py",
    "vibe/core/session/session_interop.py",
    "vibe/core/session/session_lease.py",
    "vibe/core/session/session_logger.py",
    "vibe/core/session/session_permissions.py",
    "vibe/core/session/title_model.py",
    "vibe/core/session/worktrees.py",
    "vibe/core/skills/builtins/skill_creator.py",
    "vibe/core/skills/builtins/vibe.py",
    "vibe/core/skills/manager.py",
    "vibe/core/skills/models.py",
    "vibe/core/skills/parser.py",
    "vibe/core/skills/registry/_store.py",
    "vibe/core/system_prompt.py",
    "vibe/core/telemetry/send.py",
    "vibe/core/telemetry/session.py",
    "vibe/core/telemetry/types.py",
    "vibe/core/tools/arity.py",
    "vibe/core/tools/base.py",
    "vibe/core/tools/builtins/_shell_command_policy.py",
    "vibe/core/tools/builtins/_shell_permission_analysis.py",
    "vibe/core/tools/builtins/bash.py",
    "vibe/core/tools/builtins/experimental_bash.py",
    "vibe/core/tools/builtins/prompts/todo.md",
    "vibe/core/tools/builtins/skill.py",
    "vibe/core/tools/builtins/todo.py",
    "vibe/core/tools/builtins/web_fetch.py",
    "vibe/core/tools/builtins/windows_shell.py",
    "vibe/core/tools/connectors/connector_registry.py",
    "vibe/core/tools/manager.py",
    "vibe/core/tools/mcp/registry.py",
    "vibe/core/tools/mcp/tools.py",
    "vibe/core/tools/permissions.py",
    "vibe/core/tools/utils.py",
    "vibe/core/trusted_folders.py",
    "vibe/core/types.py",
];

const EXPECTED_WHEELS: [(&str, &str, &str, usize); 17] = [
    (
        "mistral_vibe-2.25.4-py3-none-any.whl",
        "d43aa028e1931f3d05d735111384eadc937b4d47b7a05eb2ee04313fd860662f",
        "e26e851dc667818378b95af5d86878fc7e8481de10b9f86dcd8bbcac431567c7",
        619,
    ),
    (
        "mistral_vibe-2.25.5-py3-none-any.whl",
        "070a29850246e5e99158ce55ad2c71a547661410ab030bd47db01e74c2da4420",
        "5d4bf5966c16490d16b0e949f814a5153accbf54d915c681b24ee42b1f1af1a2",
        632,
    ),
    (
        "mistral_vibe-2.25.7-cp312-abi3-macosx_11_0_arm64.whl",
        "036aeaecf3c464e461bca8d81659956ccdbdc5c18476ff30963c2e955e92bbab",
        "7e7632bbca658a40df61124bba6879bee9eb3708e075c64c0d7432f4dd5e97bc",
        728,
    ),
    (
        "mistral_vibe-2.25.7-cp312-abi3-macosx_11_0_x86_64.whl",
        "ae3062dffd8688e1c22546eb7695575ddaf636fabe9ba4ba4c07c6c7784af6e3",
        "684b5b58094d75925a3caa260971501fe74ad2150e6d40680cdc46650a5ca3eb",
        728,
    ),
    (
        "mistral_vibe-2.25.7-cp312-abi3-manylinux_2_28_aarch64.whl",
        "fb3331f73c7cdfa70f8692de1ede71c4a0aee4f9448dc07bddf9356cb9bc8160",
        "9352b90f11858007b3fd4ba78a59842d67118c7304904eddfc61e87316a1e260",
        728,
    ),
    (
        "mistral_vibe-2.25.7-cp312-abi3-manylinux_2_28_x86_64.whl",
        "7483256254e690c89d635dcd28fdefdde4f0453263f4943d6350227143a60664",
        "57ca02fd72ade86e37b1ffe906bca18d0b7d7ba66abe111145467070bd453d53",
        728,
    ),
    (
        "mistral_vibe-2.25.7-cp312-abi3-win_amd64.whl",
        "b9f396192a3e6e72eb10a7335d57499e4033b4d8bb6f519a643b6220b77cc7c9",
        "075cf326b00815ebb38e4ef44ca1c8a4d0f8b829106e2302d3501848efe4e0f3",
        728,
    ),
    (
        "mistral_vibe-2.25.8-cp312-abi3-macosx_11_0_arm64.whl",
        "9b2a3f9c1078e8cf2bd0a2cd860336b3ad8039894ecf423994588be4545d73ea",
        "055d82ff78fd742f314e92bb241c441b3ba717cc7c512e357b255776a008db80",
        733,
    ),
    (
        "mistral_vibe-2.25.8-cp312-abi3-macosx_11_0_x86_64.whl",
        "22a3eef4015ae16cdd3654be41cd5d1bdeb23a18cc5bf845d0cdf291277ed824",
        "707f0a37e7882c6eea7c792357a9d2d267b9ee7f616b64e8c510ebd887f74170",
        733,
    ),
    (
        "mistral_vibe-2.25.8-cp312-abi3-manylinux_2_28_aarch64.whl",
        "a73727361c98440d20c376bff69edf92fd18d01c03f81e54e3d47135df2c9f20",
        "ce0da48a88cf1413fba347bf4f1141d3e17bb8f04e8a5ae9ffd873bddc4d09c3",
        733,
    ),
    (
        "mistral_vibe-2.25.8-cp312-abi3-manylinux_2_28_x86_64.whl",
        "d0d004cb834e8e7973378bb779411fe97bb4ed9d7e533c121d64c7fbd2ec3dea",
        "b8b40019aa63f4ca6e7c037f7c4e523bc328cb6a714e03b5fb84b5bf2f44a86a",
        733,
    ),
    (
        "mistral_vibe-2.25.8-cp312-abi3-win_amd64.whl",
        "6c21acc2bffdfd1a94c9da6d54db51ee4e2542a60165e46d5727ff06e0a57ae1",
        "a13453fb01f3f6f340e188f0860e608e495bb82a25ffeafc484169b2b51b7726",
        733,
    ),
    (
        "mistral_vibe-2.26.0-cp312-abi3-macosx_11_0_arm64.whl",
        "bc8d3ec765d844be7b51ec0247296042113784b8b6925aac5e445cec69409a27",
        "ae3e4d18819e637e560ea349d82c0e9997a29459f169ffbe08bfa8d4c8b7221f",
        755,
    ),
    (
        "mistral_vibe-2.26.0-cp312-abi3-macosx_11_0_x86_64.whl",
        "6c61e33781ddbd54e3cacf098589e49234df45c6f2498858c3e0d3bb8181c454",
        "eae589234c5cbb51225b84c2c1faa8bed0b0acc1d286087b95cdb819a62d9761",
        755,
    ),
    (
        "mistral_vibe-2.26.0-cp312-abi3-manylinux_2_28_aarch64.whl",
        "91087d3507924f44823be8567d8d051da7669b1ad2b60e549e75e91ede282a4e",
        "77677a87e3dfe47cbfb3f873815f01c0e89857391679cea6103a5a8c4d144f9e",
        755,
    ),
    (
        "mistral_vibe-2.26.0-cp312-abi3-manylinux_2_28_x86_64.whl",
        "d8230b8abd7c39a8e90d60884b1f5ee8c95ffb459c62c21f3ea0e3041117a114",
        "9976e9a749dbb8208c2a09b3d10226019ae3196afb4895136869c83fe130d26c",
        755,
    ),
    (
        "mistral_vibe-2.26.0-cp312-abi3-win_amd64.whl",
        "e3807c3670f0a74357f63e82aa77648076a12ab97abc10c579d002950e082955",
        "c16191a64e273937611162737679827e1d3620c165af2b05fb3b57fa2ff70a7e",
        755,
    ),
];

fn json(body: &str) -> Value {
    serde_json::from_str(body).expect("fixture JSON")
}

fn sha256(text: &str) -> String {
    format!("{:x}", Sha256::digest(text.as_bytes()))
}

fn selected_input_digest(rows: &[Value]) -> String {
    let mut rows = rows.to_vec();
    rows.sort_by(|left, right| left["path"].as_str().cmp(&right["path"].as_str()));
    let body = rows
        .iter()
        .map(|row| {
            format!(
                "{}\0{}\0{}\n",
                row["path"].as_str().expect("path"),
                row["sha256"].as_str().unwrap_or_default(),
                row["size"]
                    .as_u64()
                    .map_or_else(String::new, |size| size.to_string())
            )
        })
        .collect::<String>();
    sha256(&body)
}

fn change_ledger_digest(rows: &[Value]) -> String {
    let body = rows
        .iter()
        .map(|row| {
            format!(
                "{}\0{}\0{}\0{}\0{}\n",
                row["path"].as_str().expect("path"),
                row["from_sha256"].as_str().unwrap_or_default(),
                row["to_sha256"].as_str().unwrap_or_default(),
                row["change"].as_str().expect("change kind"),
                row["classification"].as_str().expect("classification")
            )
        })
        .collect::<String>();
    sha256(&body)
}

fn wheel_version(filename: &str) -> &str {
    filename
        .strip_prefix("mistral_vibe-")
        .expect("wheel prefix")
        .split('-')
        .next()
        .expect("wheel version")
}

#[test]
fn current_release_identity_and_channel_holes_are_frozen() {
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "mistral-vibe.release");
    assert_eq!(identity["route"], "mistral-vibe.headless");
    assert_eq!(identity["claim_at_observation"]["point"], "2.25.4");
    assert_eq!(identity["official"]["version"], "2.26.0");
    assert_eq!(
        identity["official"]["github_tag_commit"],
        "376f6a33413a3eec9b3795b0c0e004066c47b5c"
    );
    assert_eq!(
        identity["official"]["pypi_sdist_sha256"],
        "86ee13da13f9ca6b2f023cc5ca254a81bd99eeb5d2caa7b9e791421531bb2f5c"
    );
    assert_eq!(
        identity["official"]["pypi_attestation_source_commit"],
        "7cb91894c40bb25173abcfa36e5ea2b4b81eb28c"
    );
    assert_eq!(identity["host"]["present"], false);
    assert_eq!(
        identity["identity_decision"]["downloaded_artifact_executed"],
        false
    );
    assert_eq!(identity["identity_decision"]["provider_prompt_sent"], false);
    assert_eq!(
        identity["publication_channels"]["pypi_latest_stable"],
        "2.26.0"
    );
    assert_eq!(
        identity["publication_channels"]["github_latest_stable_release"],
        "v2.26.0"
    );
    assert_eq!(
        identity["publication_channels"]["pypi_hops_after_2_25_4"],
        serde_json::json!(["2.25.5", "2.25.7", "2.25.8", "2.26.0"])
    );
    assert_eq!(
        identity["publication_channels"]["absent_from_pypi_and_github_refs"],
        serde_json::json!(["2.25.6", "2.25.9"])
    );
    let ledger = json(LEDGER);
    let tags = ledger["github_tags"]
        .as_object()
        .expect("stable GitHub tags");
    assert_eq!(tags.len(), PUBLISHED_VERSIONS.len());
    for (tag, commit) in [
        ("v2.25.4", "19b5b74faa78d0816b8d4d4c7d7543fc3520678c"),
        ("v2.25.5", "c069ffa1e12fb5f2487b489217c40ab97721d553"),
        ("v2.25.7", "4a96003186b166d55b9f06895c45bb136eef61cd"),
        ("v2.25.8", "7c19608af06f6c61d63f8f7a5c3430da73fba2ab"),
        ("v2.26.0", "376f6a33413a3eec9b3795b0c0e004066c47b5c"),
    ] {
        assert_eq!(tags[tag], commit, "GitHub tag {tag}");
    }
    let distributions = ledger["pypi_distributions"]
        .as_object()
        .expect("PyPI stable distributions");
    assert_eq!(distributions.len(), PUBLISHED_VERSIONS.len());
    for (version, expected_count) in [
        ("2.25.4", 2),
        ("2.25.5", 2),
        ("2.25.7", 5),
        ("2.25.8", 5),
        ("2.26.0", 6),
    ] {
        assert_eq!(
            distributions[version]["files"]
                .as_array()
                .expect("files")
                .len(),
            expected_count,
            "PyPI artifact set {version}"
        );
    }
    assert_eq!(
        ledger["pypi_sdist_to_github_tag"]
            .as_object()
            .unwrap()
            .len(),
        3
    );
    assert!(
        ledger["pypi_sdist_to_github_tag"]["2.25.4"]
            .as_str()
            .expect("sdist comparison")
            .contains("sentry.py")
    );
    assert!(
        ledger["pypi_sdist_to_github_tag"]["2.25.5"]
            .as_str()
            .expect("sdist comparison")
            .contains("sentry.py")
    );
    assert!(
        ledger["pypi_sdist_to_github_tag"]["2.26.0"]
            .as_str()
            .expect("sdist comparison")
            .contains("byte-for-byte")
    );
    assert_eq!(
        identity["qualified_claim"]["claim_id"],
        "mistral-vibe.headless.release-window-1"
    );
    assert_eq!(
        identity["qualified_claim"]["behavior_revision"],
        "mistral-vibe.headless.stdio-streaming-v1"
    );
    assert_eq!(identity["qualified_claim"]["baseline"], "2.25.4");
    assert_eq!(identity["qualified_claim"]["latest_qualified"], "2.26.0");
    assert_eq!(
        identity["qualified_claim"]["newer_version_posture"],
        "AllowUnverified"
    );
    assert_eq!(
        identity["qualified_claim"]["excluded_unpublished_stables"],
        serde_json::json!(["2.25.6", "2.25.9"])
    );

    for (version, expected_filename, expected_digest) in [
        (
            "2.25.4",
            "mistral_vibe-2.25.4.tar.gz",
            "9e3ecadfa8d9be4a2693b19d5c2cf71ec914bc623649463b93b478be423f7fb8",
        ),
        (
            "2.25.5",
            "mistral_vibe-2.25.5.tar.gz",
            "acc73923c0636540f904d6e1327c72c4861674147c5df8b0b6f5ecc21a730c84",
        ),
        (
            "2.26.0",
            "mistral_vibe-2.26.0.tar.gz",
            "86ee13da13f9ca6b2f023cc5ca254a81bd99eeb5d2caa7b9e791421531bb2f5c",
        ),
    ] {
        let files = ledger["pypi_distributions"][version]["files"]
            .as_array()
            .expect("PyPI release files");
        assert!(files.iter().any(|file| {
            file["filename"] == expected_filename
                && file["sha256"] == expected_digest
                && file["packagetype"] == "sdist"
                && file["yanked"] == false
        }));
    }
    for version in ["2.25.7", "2.25.8"] {
        assert!(
            ledger["pypi_distributions"][version]["files"]
                .as_array()
                .expect("PyPI release files")
                .iter()
                .all(|file| file["packagetype"] != "sdist")
        );
    }
}

#[test]
fn every_wheel_file_inventory_matches_its_frozen_digest_and_tree() {
    let ledger = json(LEDGER);
    let inventory = json(INVENTORY);
    assert_eq!(
        inventory["schema"],
        "swallowtail.mistral-vibe.complete-wheel-inventory.v1"
    );
    assert_eq!(inventory["wheel_count"], EXPECTED_WHEELS.len());
    let wheels = inventory["wheels"].as_array().expect("wheel inventories");
    assert_eq!(wheels.len(), EXPECTED_WHEELS.len());
    let source_trees = ledger["selected_source_python_tree_by_version"]
        .as_object()
        .expect("source tree identities");
    assert_eq!(source_trees.len(), PUBLISHED_VERSIONS.len());
    let sentry_injection = &ledger["sentry_build_injection"];
    assert_eq!(sentry_injection["path"], "vibe/observability/sentry.py");
    assert_eq!(
        sentry_injection["github_tag_sha256"],
        "34c44bec66485bea4523d1d7c217156e27ec8bdb52da9d99d8a9f8580fb7a10f"
    );
    assert_eq!(
        sentry_injection["pypi_wheel_sha256"],
        "c877497060e8d8b9f5c9c22e6d849a532b34d651a4924f49ae61f6628212d891"
    );
    let mut normalized_python_trees = BTreeMap::new();
    let mut python_entry_counts = BTreeMap::new();

    for (wheel, (filename, artifact_sha, tree_sha, entry_count)) in
        wheels.iter().zip(EXPECTED_WHEELS)
    {
        assert_eq!(wheel["filename"], filename);
        assert_eq!(wheel["sha256"], artifact_sha);
        assert_eq!(wheel["tree_manifest_sha256"], tree_sha);
        let entries = wheel["entries"].as_array().expect("file entries");
        assert_eq!(entries.len(), entry_count);
        assert_eq!(wheel["entry_count"], entry_count);
        let mut prior = None;
        let mut manifest = String::new();
        let mut python_manifest = String::new();
        let mut archive_paths = BTreeSet::new();
        let mut sentry_entry = None;
        for entry in entries {
            let path = entry["path"].as_str().expect("archive path");
            assert!(
                archive_paths.insert(path.to_owned()),
                "duplicate {filename}/{path}"
            );
            if let Some(previous) = prior {
                assert!(previous < path, "unsorted {filename}/{path}");
            }
            prior = Some(path);
            let digest = entry["sha256"].as_str().expect("file digest");
            let size = entry["size"].as_u64().expect("uncompressed size");
            manifest.push_str(&format!("{path}\0{digest}\0{size}\n"));
            if path.starts_with("vibe/") && path.ends_with(".py") {
                python_manifest.push_str(&format!("{path}\0{digest}\0{size}\n"));
            }
            if path == "vibe/observability/sentry.py" {
                sentry_entry = Some((digest, size));
            }
        }
        assert_eq!(sha256(&manifest), tree_sha, "file tree {filename}");
        assert_eq!(
            sha256(&python_manifest),
            wheel["vibe_python_manifest_sha256"],
            "Python tree {filename}"
        );
        let version = wheel_version(filename);
        let metadata_files = ledger["pypi_distributions"][version]["files"]
            .as_array()
            .expect("PyPI metadata");
        assert!(
            metadata_files
                .iter()
                .any(|file| { file["filename"] == filename && file["sha256"] == artifact_sha })
        );
        if ["2.25.7", "2.25.8", "2.26.0"].contains(&version) {
            assert!(
                archive_paths.contains("mistralai_vibe_local_harness/_native.abi3.so")
                    || archive_paths.contains("mistralai_vibe_local_harness/_native.pyd"),
                "native Unified Harness file absent from {filename}"
            );
        }
        let normalized_tree = wheel["vibe_python_lf_normalized_manifest_sha256"]
            .as_str()
            .expect("normalized Python tree digest");
        if let Some(previous) = normalized_python_trees.insert(version, normalized_tree) {
            assert_eq!(previous, normalized_tree, "platform Python tree {version}");
        }
        let python_entry_count = wheel["vibe_python_entry_count"]
            .as_u64()
            .expect("Python source entry count");
        if let Some(previous) = python_entry_counts.insert(version, python_entry_count) {
            assert_eq!(
                previous, python_entry_count,
                "platform Python file count {version}"
            );
        }
        assert_eq!(
            sentry_entry.expect("Sentry source present in wheel"),
            (
                sentry_injection["pypi_wheel_sha256"]
                    .as_str()
                    .expect("wheel source digest"),
                sentry_injection["pypi_wheel_size"]
                    .as_u64()
                    .expect("wheel source size")
            ),
            "wheel Sentry injection {filename}"
        );
    }

    for (version, source_tree) in source_trees {
        let expected_tag_manifest = match version.as_str() {
            "2.25.4" => "4473d4f95fa655dc9ea6957ae564a0b7456787648babbf6dfcefe9954f512328",
            "2.25.5" => "9bc26c74e0c03ae7d5be0e6171bcb6e41af569dc09c62e466edface12231172e",
            "2.25.7" => "a7c71535e20dca514407c6bcba19184401d56976a950450f60594c3cb272cb11",
            "2.25.8" => "b366b8fefdc07abc6ea6320c2238f7b9ef8449f792723ab61edbf92e5c134e65",
            "2.26.0" => "5d7cb2b07593c2cac7f6d28068f9c9c29c7eac08e717e0c9b54a7beb7019e6df",
            other => panic!("unexpected source version {other}"),
        };
        assert_eq!(
            source_tree["manifest_sha256"]
                .as_str()
                .expect("tag manifest digest"),
            expected_tag_manifest,
            "tag Python tree {version}"
        );
        let normalized = normalized_python_trees
            .get(version.as_str())
            .expect("wheel source tree version");
        let entry_count = python_entry_counts
            .get(version.as_str())
            .expect("wheel source tree file count");
        assert_eq!(
            *entry_count,
            source_tree["entry_count"]
                .as_u64()
                .expect("tag source count"),
            "tag and normalized wheel Python entry count {version}"
        );
        let expected_manifest = match version.as_str() {
            "2.25.4" => "9befd1077d5a7f4cb9c13e05f3dfa910920ab14b193994a7867ab2ab876ca1ae",
            "2.25.5" => "a2839ce531dc9cafa32099b02ce89bcba996ed45bcd5dea2bc465fc3f66e3245",
            "2.25.7" => "a0ab2ef3df9b8728d8a4c8b05ca72b576f98b28e2716c15460a488789e466698",
            "2.25.8" => "c5e6f0d79b0804c16b50961a9cfddc9960f691229913b9d7d1f51ad9d4e34b64",
            "2.26.0" => "e95c75ea7505321bc9c34252f9705e0428fb2adf795a179c21c482c7dae25bfe",
            other => panic!("unexpected source version {other}"),
        };
        assert_eq!(
            *normalized, expected_manifest,
            "normalized wheel Python tree {version}"
        );
    }
}

#[test]
fn selected_input_file_sets_and_every_published_hop_are_frozen() {
    let ledger = json(LEDGER);
    let selected = ledger["selected_inputs_by_version"]
        .as_object()
        .expect("selected source files");
    assert_eq!(selected.len(), PUBLISHED_VERSIONS.len());
    for version in PUBLISHED_VERSIONS {
        let rows = selected[version].as_array().expect("source rows");
        let paths = rows
            .iter()
            .map(|row| row["path"].as_str().expect("source path"))
            .collect::<Vec<_>>();
        assert_eq!(paths, SELECTED_INPUT_PATHS, "source paths at {version}");
    }

    let expected_input_digests = [
        (
            "2.25.4",
            "62f70d8500175a80565db796f8299a1f3094203612f62e5f32426577dd7cdfc5",
        ),
        (
            "2.25.5",
            "f1144e76d9227a8b293958b343bec6ef7eb180424c1f5fb494076827c084d323",
        ),
        (
            "2.25.7",
            "e45cae44ccf7fc343f02fa7baaf225ee7b553cf192272b226dba09f3276c5292",
        ),
        (
            "2.25.8",
            "fcfde6f8a1f56cdf16d9706a97ab938b99d5013d2a23b3bfcf24a7ba41422a19",
        ),
        (
            "2.26.0",
            "ad17b56095c0dff56b777a8b44a591af0d729ce6d9750da2f000e62e24013fb8",
        ),
    ];
    for (version, expected_digest) in expected_input_digests {
        assert_eq!(
            selected_input_digest(selected[version].as_array().expect("source rows")),
            expected_digest,
            "selected input identity at {version}"
        );
    }

    let expected_hops: [(&str, usize, &str); 4] = [
        (
            "2.25.4..2.25.5",
            54,
            "ccd22aa28dabe4e6622a85badb3473e283ec035f4ee47d8c3adb91eaefcac7bd",
        ),
        (
            "2.25.5..2.25.7",
            20,
            "95e64b3939f65cba621875c51d3c6bec87951b47e1359e83f7941080a03ee5f3",
        ),
        (
            "2.25.7..2.25.8",
            17,
            "f744159e34a1004db483e8b154284c41992cb52fd1c609d68ebb89da274281c6",
        ),
        (
            "2.25.8..2.26.0",
            33,
            "1d869b0c7859d2f638e4ccf19a507c67b28ffa926445893ea4fd998bba432df8",
        ),
    ];
    let hops = ledger["per_hop_changed_selected_inputs"]
        .as_object()
        .expect("hop ledger");
    assert_eq!(hops.len(), expected_hops.len());
    for (name, expected_count, change_digest) in expected_hops {
        let hop = &hops[name];
        let (from_version, to_version) = name.split_once("..").expect("hop versions");
        let from_rows = selected[from_version].as_array().expect("source rows");
        let to_rows = selected[to_version].as_array().expect("source rows");
        assert_eq!(from_rows.len(), to_rows.len());
        let mut source_changed_paths = Vec::new();
        for (from, to) in from_rows.iter().zip(to_rows) {
            let path = from["path"].as_str().expect("source path");
            assert_eq!(path, to["path"].as_str().expect("source path"));
            if from["sha256"] != to["sha256"] {
                source_changed_paths.push(path);
            }
        }
        let paths = hop["changed_selected_input_paths"]
            .as_array()
            .expect("changed paths")
            .iter()
            .map(|path| path.as_str().expect("path"))
            .collect::<Vec<_>>();
        assert_eq!(
            source_changed_paths, paths,
            "complete changed paths at {name}"
        );
        assert_eq!(
            paths.len(),
            expected_count,
            "changed selected files at {name}"
        );
        let changes = hop["classifications"].as_array().expect("classifications");
        assert_eq!(changes.len(), expected_count);
        assert_eq!(
            changes
                .iter()
                .map(|entry| entry["path"].as_str().expect("classification path"))
                .collect::<Vec<_>>(),
            paths
        );
        let from_by_path = from_rows
            .iter()
            .map(|row| (row["path"].as_str().expect("source path"), row))
            .collect::<BTreeMap<_, _>>();
        let to_by_path = to_rows
            .iter()
            .map(|row| (row["path"].as_str().expect("source path"), row))
            .collect::<BTreeMap<_, _>>();
        for change in changes {
            let path = change["path"].as_str().expect("classification path");
            assert_eq!(change["from_sha256"], from_by_path[path]["sha256"]);
            assert_eq!(change["to_sha256"], to_by_path[path]["sha256"]);
            let from_digest = from_by_path[path]["sha256"].as_str();
            let to_digest = to_by_path[path]["sha256"].as_str();
            let kind = match (from_digest, to_digest) {
                (None, Some(_)) => "added",
                (Some(_), None) => "removed",
                (Some(from), Some(to)) if from != to => "modified",
                _ => panic!("unchanged source path classified as changed: {path}"),
            };
            assert_eq!(change["change"], kind, "change kind at {name}/{path}");
        }
        assert_eq!(
            change_ledger_digest(changes),
            change_digest,
            "classification content at {name}"
        );
    }
}
