use serde_json::Value;
use sha2::{Digest, Sha256};

const IDENTITY: &str = include_str!("fixtures/cursor-agent-2026.10.22/identity.json");
const PROTOCOL: &str = include_str!("fixtures/cursor-agent-2026.10.22/protocol.json");
const DIST_INVENTORY: &str = include_str!("fixtures/cursor-agent-2026.10.22/dist-inventory.json");

const LINUX_MODELS_THROUGH_OCTOBER_19: &str =
    "229d9f124165a8239cef87e9c442bb32477d517205d0a9aed213fa9219e9b38b";
const LAUNCHER_SHA256: &str = "2ccc9a8e167797641448b5e5c936f006ba137a2555f117f38c5eb76a5238a233";

fn json(source: &str) -> Value {
    serde_json::from_str(source).expect("Cursor Agent 2026.10.22 fixture is valid JSON")
}

fn sha256_hex(source: &str) -> String {
    Sha256::digest(source.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<Vec<_>>()
        .join("")
}

#[test]
fn identity_freezes_official_hops_and_the_catalogue_only_claim() {
    assert_eq!(
        sha256_hex(IDENTITY),
        "a00e22cdf0896030381df94c0e1c3367fde6f6ab8e240a113dca9e94d34bf470"
    );
    let identity = json(IDENTITY);
    assert_eq!(identity["axis"], "cursor-agent.release-date");
    assert_eq!(identity["qualified_route"], "cursor-agent.catalogue");
    assert_eq!(
        identity["published_stables_after_ceiling"],
        serde_json::json!([
            "2026.10.14-0f9edde",
            "2026.10.14-941492e",
            "2026.10.16-e3cb207",
            "2026.10.18-ea085cb",
            "2026.10.19-6bbf2c4",
            "2026.10.22-27dcf5f"
        ])
    );
    assert_eq!(identity["registry"]["version"], "2026.10.22");
    assert_eq!(identity["registry"]["latest_stable"], "2026.10.22-27dcf5f");
    assert_eq!(
        identity["registry"]["observed_repository_main"],
        "ccea3c3011d3db7fca9aa294f4a539ce0377bc19"
    );
    assert_eq!(
        identity["previous_ceiling"]["version"],
        "2026.10.01-14929f9"
    );
    assert_eq!(
        identity["previous_ceiling"]["darwin_arm64_archive_sha256"],
        "778d04e542adc5c8b6760fda3ebe0757f903b1764f2792c232ef9a35e6e2151b"
    );
    assert_eq!(
        identity["previous_ceiling"]["linux_x64_archive_sha256"],
        "ba9a855f8f813c91b9f2707127572d2dc9ae5a62818e1c36719625d0fb8bd452"
    );
    assert_eq!(identity["host"]["present"], false);

    let hops = identity["hops"].as_array().expect("hops are an array");
    assert_eq!(hops.len(), 6);
    for (hop, (version, role)) in hops.iter().zip([
        ("2026.10.14-0f9edde", "superseded-same-date"),
        ("2026.10.14-941492e", "qualified-milestone"),
        ("2026.10.16-e3cb207", "qualified-milestone"),
        ("2026.10.18-ea085cb", "qualified-milestone"),
        ("2026.10.19-6bbf2c4", "qualified-milestone"),
        ("2026.10.22-27dcf5f", "qualified-milestone"),
    ]) {
        assert_eq!(hop["version"], version);
        assert_eq!(hop["role"], role);
        assert_eq!(hop["launcher_sha256"], LAUNCHER_SHA256);
        for field in [
            "darwin_arm64_archive_sha256",
            "linux_x64_archive_sha256",
            "darwin_arm64_models_module_sha256",
            "linux_x64_models_module_sha256",
            "darwin_arm64_node_runtime_sha256",
            "linux_x64_node_runtime_sha256",
            "launcher_sha256",
        ] {
            let digest = hop[field].as_str().expect("artifact digest is text");
            assert_eq!(digest.len(), 64, "{version} {field}");
            assert!(digest.bytes().all(|byte| byte.is_ascii_hexdigit()));
        }
    }

    for hop in hops.iter().take(5) {
        assert_eq!(
            hop["linux_x64_models_module_sha256"],
            LINUX_MODELS_THROUGH_OCTOBER_19
        );
    }
    assert_eq!(
        hops[5]["linux_x64_models_module_sha256"],
        "07734c6735548e2995d44b7bea7a0b2190316f9569db9ae30c08f983a56e452c"
    );
    assert_ne!(
        hops[5]["linux_x64_node_runtime_sha256"],
        hops[4]["linux_x64_node_runtime_sha256"]
    );

    let decision = &identity["identity_decision"];
    assert_eq!(decision["shape"], "compatible-extension");
    assert_eq!(
        decision["claim_id"],
        "cursor-agent.catalogue.release-window-3"
    );
    assert_eq!(
        decision["reuse_behavior"],
        "cursor-agent.catalogue.calendar-release-v1"
    );
    assert_eq!(decision["keep_baseline"], "2026.07.01-41b2de7");
    assert_eq!(decision["raise_latest_qualified_to"], "2026.10.22-27dcf5f");
    assert_eq!(
        decision["add_exact_milestones"],
        serde_json::json!([
            "2026.10.14-941492e",
            "2026.10.16-e3cb207",
            "2026.10.18-ea085cb",
            "2026.10.19-6bbf2c4",
            "2026.10.22-27dcf5f"
        ])
    );
    assert_eq!(
        decision["superseded_same_date_build_rejected"],
        "2026.10.14-0f9edde"
    );
    assert_eq!(decision["unpublished_later_stable"], "2026-10-23");
    assert_eq!(decision["preserve_acp_claim_at"], "2026.10.01-14929f9");
    assert_eq!(decision["preserve_headless_claim_at"], "2026.10.01-14929f9");
    assert_eq!(decision["infer_calendar_gap"], false);
    assert_eq!(decision["new_behavior_revision"], false);
    assert_eq!(decision["new_public_operation"], false);
    assert_eq!(decision["consumer_visible_narrowing"], false);
    assert_eq!(decision["mix_npm_cursor_agent_axis"], false);
    assert_eq!(decision["downloaded_artifacts_executed"], false);
    assert_eq!(decision["authenticated_catalogue_called"], false);
    assert_eq!(decision["provider_prompt_sent"], false);
}

#[test]
fn selected_catalogue_path_keeps_its_existing_adapter_boundary() {
    assert_eq!(
        sha256_hex(PROTOCOL),
        "6a3f4330965e1f6be4c5ab3cd062a66805752b8a58a2ea45871225f5f81bd44e"
    );
    let protocol = json(PROTOCOL);
    assert_eq!(protocol["selected_route"], "cursor-agent.catalogue");
    assert_eq!(protocol["selected_cli_command"], "models");
    assert_eq!(protocol["selected_cli_command_present_at_every_hop"], true);
    assert_eq!(
        protocol["selected_catalogue_fields"]["model_id"],
        "displayModelId || modelId"
    );
    assert_eq!(
        protocol["selected_catalogue_fields"]["display_name"],
        "displayName || displayNameShort"
    );
    assert_eq!(
        protocol["selected_request_path"]["account_options"],
        serde_json::json!(["endpoint", "apiKey", "authToken", "insecure"])
    );
    assert_eq!(
        protocol["compatible_surface_review"]["mapped_fields_unchanged"],
        true
    );
    assert_eq!(
        protocol["compatible_surface_review"]["linux_models_module_byte_identical_through"],
        "2026.10.19-6bbf2c4"
    );
    assert_eq!(
        protocol["compatible_surface_review"]["no_new_operation_or_authority"],
        true
    );
    assert_eq!(protocol["sibling_axes"]["acp_claim_changed"], false);
    assert_eq!(
        protocol["sibling_axes"]["acp_qualified_ceiling"],
        "2026.10.01-14929f9"
    );
    assert_eq!(protocol["sibling_axes"]["headless_claim_changed"], false);
    assert_eq!(
        protocol["sibling_axes"]["headless_qualified_ceiling"],
        "2026.10.01-14929f9"
    );
    assert_eq!(protocol["sibling_axes"]["evidence_transferred"], false);
    assert_eq!(protocol["safety"]["archive_executed"], false);
    assert_eq!(protocol["safety"]["credentials_used"], false);
    assert_eq!(protocol["safety"]["provider_prompt_sent"], false);
    assert_eq!(
        protocol["adapter_failure_and_lifecycle"]["argv"],
        serde_json::json!(["models"])
    );
}

#[test]
fn inventory_freezes_every_file_and_each_hop_classification() {
    assert_eq!(
        sha256_hex(DIST_INVENTORY),
        "1e6a0fea8978f891d96050749eaa541b983c5106d10a6839b8c9fae8e395f411"
    );
    let inventory = json(DIST_INVENTORY);
    assert_eq!(
        inventory["schema"],
        "cursor-agent-catalogue-package-inventory.v1"
    );
    assert_eq!(
        inventory["versions"],
        serde_json::json!([
            "2026.10.01-14929f9",
            "2026.10.14-0f9edde",
            "2026.10.14-941492e",
            "2026.10.16-e3cb207",
            "2026.10.18-ea085cb",
            "2026.10.19-6bbf2c4",
            "2026.10.22-27dcf5f"
        ])
    );
    assert_eq!(
        inventory["platforms"],
        serde_json::json!(["darwin-arm64", "linux-x64"])
    );

    for (platform, counts) in [
        ("darwin-arm64", [447, 450, 450, 452, 451, 452, 437]),
        ("linux-x64", [455, 458, 458, 460, 459, 460, 445]),
    ] {
        for (version, count) in inventory["versions"]
            .as_array()
            .expect("versions are an array")
            .iter()
            .zip(counts)
        {
            let version = version.as_str().expect("version is text");
            let tree = &inventory["archive_trees"][version][platform];
            assert_eq!(tree["file_count"], count, "{platform} {version}");
            assert_eq!(
                tree["files"].as_array().expect("files are an array").len(),
                count
            );
        }
    }

    let expected = [
        (
            "darwin-arm64",
            "2026.10.01-14929f9_to_2026.10.14-0f9edde",
            [65, 62, 85, 300],
        ),
        (
            "darwin-arm64",
            "2026.10.14-0f9edde_to_2026.10.14-941492e",
            [63, 63, 79, 308],
        ),
        (
            "darwin-arm64",
            "2026.10.14-941492e_to_2026.10.16-e3cb207",
            [66, 64, 82, 304],
        ),
        (
            "darwin-arm64",
            "2026.10.16-e3cb207_to_2026.10.18-ea085cb",
            [65, 66, 79, 307],
        ),
        (
            "darwin-arm64",
            "2026.10.18-ea085cb_to_2026.10.19-6bbf2c4",
            [65, 64, 79, 308],
        ),
        (
            "darwin-arm64",
            "2026.10.19-6bbf2c4_to_2026.10.22-27dcf5f",
            [69, 84, 82, 286],
        ),
        (
            "linux-x64",
            "2026.10.01-14929f9_to_2026.10.14-0f9edde",
            [6, 3, 69, 383],
        ),
        (
            "linux-x64",
            "2026.10.14-0f9edde_to_2026.10.14-941492e",
            [0, 0, 17, 441],
        ),
        (
            "linux-x64",
            "2026.10.14-941492e_to_2026.10.16-e3cb207",
            [4, 2, 47, 409],
        ),
        (
            "linux-x64",
            "2026.10.16-e3cb207_to_2026.10.18-ea085cb",
            [1, 2, 42, 416],
        ),
        (
            "linux-x64",
            "2026.10.18-ea085cb_to_2026.10.19-6bbf2c4",
            [4, 3, 19, 437],
        ),
        (
            "linux-x64",
            "2026.10.19-6bbf2c4_to_2026.10.22-27dcf5f",
            [6, 21, 98, 341],
        ),
    ];
    for (platform, hop, counts) in expected {
        for (category, count) in ["added", "removed", "changed", "identical"]
            .into_iter()
            .zip(counts)
        {
            let paths = inventory["hop_deltas"][platform][hop][category]["paths"]
                .as_array()
                .expect("classified paths are an array");
            assert_eq!(paths.len(), count, "{platform} {hop} {category}");
            assert_eq!(
                inventory["hop_deltas"][platform][hop][category]["count"],
                count
            );
        }
    }

    for version in inventory["versions"]
        .as_array()
        .expect("versions are an array")
        .iter()
        .take(6)
    {
        let version = version.as_str().expect("version is text");
        assert_eq!(
            inventory["selected_modules"][version]["linux-x64"]["models_module_sha256"],
            LINUX_MODELS_THROUGH_OCTOBER_19,
            "{version}"
        );
        assert_eq!(
            inventory["selected_modules"][version]["linux-x64"]["launcher"]["sha256"],
            LAUNCHER_SHA256
        );
    }
    assert_eq!(
        inventory["selected_modules"]["2026.10.22-27dcf5f"]["linux-x64"]["models_module_sha256"],
        "07734c6735548e2995d44b7bea7a0b2190316f9569db9ae30c08f983a56e452c"
    );
}
