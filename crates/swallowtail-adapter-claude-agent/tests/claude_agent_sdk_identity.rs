//! Exact identity, credential non-custody, and route-distinctness proofs for
//! the `claude-agent.sdk` route.
//!
//! Nothing here executes the downloaded official artifact, opens a session, or
//! contacts a provider: every assertion reads the shipped adapter asset and
//! the adapter's own declarations.

use std::collections::BTreeSet;
use swallowtail_adapter_claude_agent::sdk::{
    CLAUDE_AGENT_SDK_ADDABLE_ROUTE_ID, CLAUDE_AGENT_SDK_BEHAVIOR, CLAUDE_AGENT_SDK_NATIVE_VERSION,
    CLAUDE_AGENT_SDK_NODE_RUNTIME, CLAUDE_AGENT_SDK_PACKAGE, CLAUDE_AGENT_SDK_SIDECAR_ENTRY_FILE,
    CLAUDE_AGENT_SDK_SIDECAR_SOURCE, CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG, CLAUDE_AGENT_SDK_VERSION,
    CLAUDE_AGENT_SDK_WIRE, claude_agent_sdk_addable_route_descriptor, claude_agent_sdk_descriptor,
    claude_agent_sdk_native_claim, claude_agent_sdk_node_claim, claude_agent_sdk_package_claim,
    claude_agent_sdk_sidecar_claim, claude_agent_sdk_tool_admission_namespace,
    claude_agent_sdk_wire_claim,
};
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::HostServices;

const PROTOCOL: &str = include_str!("fixtures/claude-agent-sdk-v1/protocol.json");
const CURRENT_IDENTITY: &str = include_str!("fixtures/claude-agent-sdk-0.3.293/identity.json");
const CURRENT_PROTOCOL: &str = include_str!("fixtures/claude-agent-sdk-0.3.293/protocol.json");
const CURRENT_INVENTORY: &str =
    include_str!("fixtures/claude-agent-sdk-0.3.293/dist-inventory.json");

/// Reports whether one line names an option key, ignoring a longer key that
/// merely contains it: `disallowedTools` is not `allowedTools`.
fn names_option(line: &str, key: &str) -> bool {
    line.match_indices(key).any(|(index, _)| {
        line[..index]
            .chars()
            .next_back()
            .is_none_or(|character| !character.is_alphanumeric())
    })
}

fn protocol() -> serde_json::Value {
    serde_json::from_str(PROTOCOL).expect("frozen corpus identity is valid JSON")
}

fn string_set(value: &serde_json::Value) -> BTreeSet<String> {
    value
        .as_array()
        .expect("evidence field is a list")
        .iter()
        .map(|entry| entry.as_str().expect("evidence item is text").to_owned())
        .collect()
}

fn object_key_set(value: &serde_json::Value) -> BTreeSet<String> {
    value
        .as_object()
        .expect("evidence field is an object")
        .keys()
        .cloned()
        .collect()
}

struct HopPathClassification {
    hop: &'static str,
    added: &'static [&'static str],
    removed: &'static [&'static str],
    changed: &'static [&'static str],
}

impl HopPathClassification {
    fn new(
        hop: &'static str,
        added: &'static [&'static str],
        removed: &'static [&'static str],
        changed: &'static [&'static str],
    ) -> Self {
        Self {
            hop,
            added,
            removed,
            changed,
        }
    }
}

#[test]
fn the_route_binds_package_native_segments_and_exact_axes_with_node_window() {
    let descriptor = claude_agent_sdk_descriptor();
    assert_eq!(
        descriptor.identity().id().as_str(),
        "swallowtail.claude-agent.sdk"
    );
    assert_eq!(
        descriptor.transport_family().as_str(),
        CLAUDE_AGENT_SDK_WIRE
    );
    for (axis, point) in [
        ("claude-agent.sdk.package", CLAUDE_AGENT_SDK_VERSION),
        ("claude-agent.sdk.native", CLAUDE_AGENT_SDK_NATIVE_VERSION),
        ("claude-agent.sdk.wire", CLAUDE_AGENT_SDK_WIRE),
        (
            "claude-agent.sdk.sidecar",
            CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG,
        ),
    ] {
        let axis = swallowtail_core::InterfaceVersionAxis::new(axis).expect("valid axis");
        let claim = descriptor
            .interface_compatibility(&axis)
            .unwrap_or_else(|| panic!("axis {axis:?} is bound"));
        assert_eq!(
            claim.newer_version_posture(),
            swallowtail_core::InterfaceNewerVersionPosture::QualifiedOnly,
            "no axis may inherit an unverified-newer posture"
        );
        assert_eq!(claim.milestones().len(), 1, "each axis has one segment");
        let point = swallowtail_core::InterfaceVersion::new(point).expect("valid version");
        assert_eq!(claim.exclusions().len(), 0);
        if axis.as_str() == "claude-agent.sdk.package" {
            assert_eq!(
                claim.baseline(),
                &swallowtail_core::InterfaceVersion::new("0.3.284").unwrap()
            );
            assert_eq!(claim.latest_qualified(), &point);
        } else if axis.as_str() == "claude-agent.sdk.native" {
            assert_eq!(
                claim.baseline(),
                &swallowtail_core::InterfaceVersion::new("2.1.284").unwrap()
            );
            assert_eq!(claim.latest_qualified(), &point);
        } else {
            assert_eq!(claim.baseline(), &point);
            assert_eq!(claim.latest_qualified(), &point);
        }
    }
    let node_axis =
        swallowtail_core::InterfaceVersionAxis::new("claude-agent.sdk.node").expect("valid axis");
    let node_claim = descriptor
        .interface_compatibility(&node_axis)
        .expect("the Node axis is bound");
    assert_eq!(
        node_claim.newer_version_posture(),
        swallowtail_core::InterfaceNewerVersionPosture::QualifiedOnly
    );
    assert_eq!(node_claim.milestones().len(), 1);
    assert_eq!(
        node_claim.baseline().as_str(),
        "22.23.2",
        "the prior qualified Node point remains the baseline"
    );
    assert_eq!(
        node_claim.latest_qualified().as_str(),
        CLAUDE_AGENT_SDK_NODE_RUNTIME
    );
    assert_eq!(node_claim.exclusions().len(), 0);
    // The SDK wrapper and the native binary it delivers are coupled but never
    // equal, and neither is the Claude Code axis.
    assert_ne!(CLAUDE_AGENT_SDK_VERSION, CLAUDE_AGENT_SDK_NATIVE_VERSION);
    assert_eq!(
        swallowtail_adapter_claude_agent::CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION,
        "2.1.281",
        "the Claude Code window is observed separately and does not transfer"
    );
    assert_ne!(
        swallowtail_adapter_claude_agent::CLAUDE_CODE_HEADLESS_LATEST_QUALIFIED_VERSION,
        CLAUDE_AGENT_SDK_NATIVE_VERSION,
        "raising the Claude Code ceiling must not rebind the SDK native pin"
    );
}

#[test]
fn package_native_segments_preserve_baselines_claim_ids_and_qualified_only() {
    // Research 416 qualifies only the coupled package/native surfaces while
    // preserving each original baseline, claim id and QualifiedOnly posture.
    let claims = [
        (
            claude_agent_sdk_package_claim(),
            "claude-agent.sdk.package-window-1",
            CLAUDE_AGENT_SDK_VERSION,
            "0.3.284",
        ),
        (
            claude_agent_sdk_native_claim(),
            "claude-agent.sdk.native-window-1",
            CLAUDE_AGENT_SDK_NATIVE_VERSION,
            "2.1.284",
        ),
        (
            claude_agent_sdk_wire_claim(),
            "claude-agent.sdk.wire-v1",
            CLAUDE_AGENT_SDK_WIRE,
            CLAUDE_AGENT_SDK_WIRE,
        ),
        (
            claude_agent_sdk_sidecar_claim(),
            "claude-agent.sdk.sidecar-v1",
            CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG,
            CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG,
        ),
    ];
    for (claim, id, point, baseline) in claims {
        assert_eq!(claim.id().as_str(), id);
        assert_eq!(
            claim.newer_version_posture(),
            swallowtail_core::InterfaceNewerVersionPosture::QualifiedOnly
        );
        assert_eq!(claim.milestones().len(), 1);
        let point = swallowtail_core::InterfaceVersion::new(point).expect("valid version");
        assert_eq!(
            claim.baseline(),
            &swallowtail_core::InterfaceVersion::new(baseline).unwrap()
        );
        assert_eq!(claim.latest_qualified(), &point);
        assert_eq!(claim.exclusions().len(), 0);
        assert!(claim.permits(&point));
    }

    let node = claude_agent_sdk_node_claim();
    assert_eq!(node.id().as_str(), "claude-agent.sdk.node-window-1");
    assert_eq!(
        node.newer_version_posture(),
        swallowtail_core::InterfaceNewerVersionPosture::QualifiedOnly
    );
    assert_eq!(node.milestones().len(), 1);
    assert_eq!(node.baseline().as_str(), "22.23.2");
    assert_eq!(
        node.latest_qualified().as_str(),
        CLAUDE_AGENT_SDK_NODE_RUNTIME
    );
    assert_eq!(node.exclusions().len(), 0);
    for qualified in ["22.23.2", "22.23.3"] {
        assert!(
            node.permits(&swallowtail_core::InterfaceVersion::new(qualified).expect("valid Node")),
            "missing Node {qualified}"
        );
    }

    let version =
        |value: &str| swallowtail_core::InterfaceVersion::new(value).expect("valid version");
    let package = claude_agent_sdk_package_claim();
    let native = claude_agent_sdk_native_claim();
    for patch in 284..=293 {
        assert!(package.permits(&version(&format!("0.3.{patch}"))));
        assert!(native.permits(&version(&format!("2.1.{patch}"))));
    }
    assert!(!package.permits(&version("0.3.283")));
    assert!(!package.permits(&version("0.3.294")));
    assert!(!native.permits(&version("2.1.283")));
    assert!(!native.permits(&version("2.1.294")));
    // Older native inventory points stay outside the selected segment.
    assert!(!package.permits(&version("0.3.260")));
    assert!(!native.permits(&version("2.1.258")));
    assert!(!claude_agent_sdk_node_claim().permits(&version("26.7.0")));
}

#[test]
fn frozen_current_identity_records_every_published_hop_and_selected_surface() {
    let identity: serde_json::Value = serde_json::from_str(CURRENT_IDENTITY).unwrap();
    let protocol: serde_json::Value = serde_json::from_str(CURRENT_PROTOCOL).unwrap();
    let inventory: serde_json::Value = serde_json::from_str(CURRENT_INVENTORY).unwrap();
    assert_eq!(identity["official_channel"]["selected"], "dist-tags.latest");
    assert_eq!(identity["official_channel"]["current_version"], "0.3.293");
    assert_eq!(
        identity["official_channel"]["published_gaps_inside_qualified_segment"],
        serde_json::json!([])
    );
    assert_eq!(
        identity["official"]["tarball_sha256"],
        "395bfbda4294c6334fa18be3706a46ff31851b4f4a6f74ba11ac96c9d97ced9f"
    );
    assert_eq!(
        identity["current_native_artifact_verification"]["manifest_match"],
        true
    );
    let native = &identity["native_identity_by_sdk_version"]["0.3.293"];
    assert_eq!(native["version"], "2.1.293");
    assert_eq!(native["commit"], "3abc54a9d60b4d12c627afad22d6e5f58a6199d2");
    assert_eq!(
        native["mods_commit"],
        "765f236fe1bfcc678e0ec59af9170fb7d6771a3a"
    );
    assert_eq!(native["build_date"], "2026-10-07T06:56:40Z");
    assert_eq!(native["harness_schema"], 1);
    assert_eq!(
        string_set(&serde_json::Value::Array(
            native["platforms"]
                .as_object()
                .unwrap()
                .keys()
                .map(|key| serde_json::Value::String(key.clone()))
                .collect()
        )),
        [
            "darwin-arm64",
            "darwin-x64",
            "linux-arm64",
            "linux-arm64-musl",
            "linux-x64",
            "linux-x64-musl",
            "win32-arm64",
            "win32-x64"
        ]
        .into_iter()
        .map(str::to_owned)
        .collect()
    );
    assert_eq!(
        native["platforms"]["darwin-arm64"]["checksum"],
        "4e21122a227857da1178aca3299700c1fd7f2b77c93f12e73c2c76db796a105e"
    );
    assert_eq!(native["platforms"]["darwin-arm64"]["size"], 236330608);
    let expected_native_platforms = [
        "darwin-arm64",
        "darwin-x64",
        "linux-arm64",
        "linux-arm64-musl",
        "linux-x64",
        "linux-x64-musl",
        "win32-arm64",
        "win32-x64",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<BTreeSet<_>>();
    let native_versions = &identity["native_identity_by_sdk_version"];
    assert_eq!(
        object_key_set(native_versions),
        (284..=293).map(|patch| format!("0.3.{patch}")).collect()
    );
    let mut preceding_platform_hashes: Option<&serde_json::Value> = None;
    for patch in 284..=293 {
        let sdk = format!("0.3.{patch}");
        let embedded = &native_versions[&sdk];
        assert_eq!(embedded["version"], format!("2.1.{patch}"));
        let platform_hashes = &embedded["platforms"];
        assert_eq!(object_key_set(platform_hashes), expected_native_platforms);
        if let Some(previous) = preceding_platform_hashes {
            for platform in &expected_native_platforms {
                assert_ne!(
                    previous[platform.as_str()]["checksum"],
                    platform_hashes[platform.as_str()]["checksum"],
                    "the embedded {platform} binary rotates at {sdk}"
                );
            }
        }
        preceding_platform_hashes = Some(platform_hashes);
    }
    assert_eq!(protocol["mapped_behavior_changes"], serde_json::json!([]));
    assert_eq!(protocol["hop_review"].as_array().unwrap().len(), 9);
    assert_eq!(
        inventory["compared"],
        serde_json::json!([
            "0.3.284", "0.3.285", "0.3.286", "0.3.287", "0.3.288", "0.3.289", "0.3.290", "0.3.291",
            "0.3.292", "0.3.293"
        ])
    );
    assert_eq!(
        inventory["package_file_counts"],
        serde_json::json!({
            "0.3.284":19,"0.3.285":19,"0.3.286":19,"0.3.287":19,"0.3.288":19,
            "0.3.289":19,"0.3.290":19,"0.3.291":19,"0.3.292":19,"0.3.293":19
        })
    );
    let trees = serde_json::json!({
        "0.3.284":"069235a5598d5eea0f3dfa0698e451677e5e56d1f5091c2a92e73d185da92372",
        "0.3.285":"40c755a76bb6cef40716069e45a4a605df732e3f37789af7d970fc4c1dbe923e",
        "0.3.286":"24fb1bee6c7274691bc7bd796bd7d552d0736a723c426b0dffa028648e110041",
        "0.3.287":"9e071a2e789003aeca3e7c3c420e14a75c34e32be56962c12f11010abd367d43",
        "0.3.288":"242975636b7a1a2a94cc1c89f3d0a78a93d3638b9decb79abb846414dfb57a99",
        "0.3.289":"82e44817e8a20a031bd53d12b959315d0905158574096faaae1d64f761b1ac2d",
        "0.3.290":"a11dcff5096b52d6ac3b01357adc8feba43d44c18638219a9e2b7320d210924a",
        "0.3.291":"d69a861b3f1473542cf5dfb13df1b68dca2cf8f2369b332782b52afb19fc16ff",
        "0.3.292":"4e7cdbac0fc10849f793526b81fdf599279d146a5ffcb35c1f9bed2dc4f8b125",
        "0.3.293":"b6339d8c766a4b1b67c1ef11c7cd49cd6ddd493a9de26dafb49984ba538f5f36"
    });
    assert_eq!(inventory["tree_sha256"], trees);
    let expected_file_paths = [
        "LICENSE.md",
        "README.md",
        "agentSdkTypes.d.ts",
        "bridge.d.ts",
        "bridge.mjs",
        "browser-sdk.d.ts",
        "browser-sdk.js",
        "core-29pt3grd.mjs",
        "core-3ctbshc7.mjs",
        "core-5c9xf7pv.mjs",
        "core-8p15jxca.mjs",
        "core-95ey60bt.mjs",
        "core-ae32wa3s.mjs",
        "core-czfq02x9.mjs",
        "core-d0szsqzn.mjs",
        "core-eg2e19h0.mjs",
        "core-ejtftkpy.mjs",
        "core-fgc83rkd.mjs",
        "core-ke40g9bw.mjs",
        "core-kgfyd2t2.mjs",
        "core-q38rzykt.mjs",
        "core-repw6452.mjs",
        "core-rf2bg9hj.mjs",
        "core-w2yya30y.mjs",
        "core-wn2j871p.mjs",
        "core-y9rc2t7d.mjs",
        "core-ygywjedj.mjs",
        "core.d.ts",
        "core.mjs",
        "extractFromBunfs.d.ts",
        "extractFromBunfs.js",
        "manifest.json",
        "manifest.zst.json",
        "package.json",
        "sdk-tools.d.ts",
        "sdk.d.ts",
        "sdk.mjs",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    assert_eq!(
        string_set(&serde_json::Value::Array(
            inventory["file_hashes"]
                .as_object()
                .unwrap()
                .keys()
                .map(|key| serde_json::Value::String(key.clone()))
                .collect()
        )),
        expected_file_paths
    );
    let hop_paths = [
        HopPathClassification::new(
            "from_0_3_284_to_0_3_285",
            &["core-rf2bg9hj.mjs", "core-w2yya30y.mjs"],
            &["core-3ctbshc7.mjs", "core-eg2e19h0.mjs"],
            &[
                "bridge.mjs",
                "browser-sdk.js",
                "core.mjs",
                "manifest.json",
                "manifest.zst.json",
                "package.json",
                "sdk-tools.d.ts",
                "sdk.d.ts",
                "sdk.mjs",
            ],
        ),
        HopPathClassification::new(
            "from_0_3_285_to_0_3_286",
            &["core-5c9xf7pv.mjs", "core-y9rc2t7d.mjs"],
            &["core-rf2bg9hj.mjs", "core-w2yya30y.mjs"],
            &[
                "bridge.mjs",
                "browser-sdk.js",
                "core.mjs",
                "manifest.json",
                "manifest.zst.json",
                "package.json",
                "sdk.d.ts",
                "sdk.mjs",
            ],
        ),
        HopPathClassification::new(
            "from_0_3_286_to_0_3_287",
            &["core-kgfyd2t2.mjs", "core-q38rzykt.mjs"],
            &["core-5c9xf7pv.mjs", "core-y9rc2t7d.mjs"],
            &[
                "bridge.mjs",
                "browser-sdk.js",
                "core.mjs",
                "manifest.json",
                "manifest.zst.json",
                "package.json",
                "sdk.d.ts",
                "sdk.mjs",
            ],
        ),
        HopPathClassification::new(
            "from_0_3_287_to_0_3_288",
            &["core-ejtftkpy.mjs", "core-wn2j871p.mjs"],
            &["core-kgfyd2t2.mjs", "core-q38rzykt.mjs"],
            &[
                "bridge.mjs",
                "browser-sdk.js",
                "core.mjs",
                "manifest.json",
                "manifest.zst.json",
                "package.json",
                "sdk.d.ts",
                "sdk.mjs",
            ],
        ),
        HopPathClassification::new(
            "from_0_3_288_to_0_3_289",
            &["core-8p15jxca.mjs", "core-repw6452.mjs"],
            &["core-ejtftkpy.mjs", "core-wn2j871p.mjs"],
            &[
                "bridge.mjs",
                "browser-sdk.js",
                "core.mjs",
                "manifest.json",
                "manifest.zst.json",
                "package.json",
                "sdk.mjs",
            ],
        ),
        HopPathClassification::new(
            "from_0_3_289_to_0_3_290",
            &["core-29pt3grd.mjs", "core-ygywjedj.mjs"],
            &["core-8p15jxca.mjs", "core-repw6452.mjs"],
            &[
                "bridge.d.ts",
                "bridge.mjs",
                "browser-sdk.js",
                "core.mjs",
                "manifest.json",
                "manifest.zst.json",
                "package.json",
                "sdk-tools.d.ts",
                "sdk.d.ts",
                "sdk.mjs",
            ],
        ),
        HopPathClassification::new(
            "from_0_3_290_to_0_3_291",
            &["core-fgc83rkd.mjs", "core-ke40g9bw.mjs"],
            &["core-29pt3grd.mjs", "core-ygywjedj.mjs"],
            &[
                "bridge.mjs",
                "browser-sdk.js",
                "core.mjs",
                "manifest.json",
                "manifest.zst.json",
                "package.json",
                "sdk.mjs",
            ],
        ),
        HopPathClassification::new(
            "from_0_3_291_to_0_3_292",
            &["core-95ey60bt.mjs", "core-czfq02x9.mjs"],
            &["core-fgc83rkd.mjs", "core-ke40g9bw.mjs"],
            &[
                "bridge.mjs",
                "browser-sdk.js",
                "core.mjs",
                "manifest.json",
                "manifest.zst.json",
                "package.json",
                "sdk-tools.d.ts",
                "sdk.d.ts",
                "sdk.mjs",
            ],
        ),
        HopPathClassification::new(
            "from_0_3_292_to_0_3_293",
            &["core-ae32wa3s.mjs", "core-d0szsqzn.mjs"],
            &["core-95ey60bt.mjs", "core-czfq02x9.mjs"],
            &[
                "bridge.mjs",
                "browser-sdk.js",
                "core.mjs",
                "manifest.json",
                "manifest.zst.json",
                "package.json",
                "sdk.d.ts",
                "sdk.mjs",
            ],
        ),
    ];
    for classification in hop_paths {
        let hop = classification.hop;
        let record = &inventory[hop];
        let set = |items: &[&str]| {
            items
                .iter()
                .map(|item| (*item).to_owned())
                .collect::<BTreeSet<_>>()
        };
        assert_eq!(
            string_set(&record["added"]),
            set(classification.added),
            "added paths in {hop}"
        );
        assert_eq!(
            string_set(&record["removed"]),
            set(classification.removed),
            "removed paths in {hop}"
        );
        assert_eq!(
            string_set(&record["changed"]),
            set(classification.changed),
            "changed paths in {hop}"
        );
        let classified: BTreeSet<String> = record["changed_file_classification"]
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect();
        assert_eq!(
            classified,
            set(classification.changed),
            "classification covers exactly each content-changed path in {hop}"
        );
    }
    let hops = [
        (
            "0.3.285",
            "5c68fdbf22cbe796d533dfb59e939aecdee7d3ce37d4853003b9d4b33863f40f",
        ),
        (
            "0.3.286",
            "c70947ab030c4b627a2f640fe1df91b02296026a59ade47155fa7ef806e8c07c",
        ),
        (
            "0.3.287",
            "eff703b8cb9d9fc51e2dc80bf86257a063f2556f6d4bb5aa9814020a6bf48b55",
        ),
        (
            "0.3.288",
            "eb97c0f5a7d96189bceaf1986c2751a024b7e13844ba843531cc55bb0b4fac54",
        ),
        (
            "0.3.289",
            "5ce0058cb62535a88705619ef0bd4293612d1b13ae95d620b2eb4dadbf83c898",
        ),
        (
            "0.3.290",
            "086d6923a4232db6d2d9480dbc27a67555ee9031a12b75eb5ae5336fb8dd3108",
        ),
        (
            "0.3.291",
            "9bd28f18be0f9651ae32ab7780099db93d9a2a4b785d722056a3be231f47571a",
        ),
        (
            "0.3.292",
            "967024d934865047062b5b5ed6e0d613d4454a846e9c71e1ac55ee3c6d57c168",
        ),
        (
            "0.3.293",
            "395bfbda4294c6334fa18be3706a46ff31851b4f4a6f74ba11ac96c9d97ced9f",
        ),
    ];
    for (version, sha256) in hops {
        let entry = identity["published_stables_after_previous_ceiling"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["version"] == version)
            .expect("every published hop retained");
        assert_eq!(entry["sha256"], sha256, "exact package artifact {version}");
        assert_eq!(
            identity["native_identity_by_sdk_version"][version]["version"],
            format!("2.1.{}", &version[4..])
        );
    }
}

#[test]
fn the_shipped_asset_matches_the_frozen_identity() {
    let protocol = protocol();
    assert_eq!(protocol["sdk_package"], CLAUDE_AGENT_SDK_PACKAGE);
    assert_eq!(protocol["sdk_version"], CLAUDE_AGENT_SDK_VERSION);
    assert_eq!(protocol["native_version"], CLAUDE_AGENT_SDK_NATIVE_VERSION);
    assert_eq!(protocol["node_runtime"], CLAUDE_AGENT_SDK_NODE_RUNTIME);
    assert_eq!(protocol["behavior_revision"], CLAUDE_AGENT_SDK_BEHAVIOR);
    assert_eq!(
        protocol["sidecar_entry_file"],
        CLAUDE_AGENT_SDK_SIDECAR_ENTRY_FILE
    );
    assert!(
        CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG.starts_with("swallowtail-claude-agent-sdk-sidecar@")
    );
    assert!(CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains(CLAUDE_AGENT_SDK_WIRE));
    assert!(CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains(CLAUDE_AGENT_SDK_VERSION));
    assert!(CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains(CLAUDE_AGENT_SDK_NATIVE_VERSION));
}

#[test]
fn the_sidecar_asset_can_never_reach_a_credential_bearing_surface() {
    // Mechanical falsifier from the frozen evidence: the `.` entry point only.
    // `/bridge` and `/browser` declare raw access tokens, minted worker
    // credentials, and OAuth credential messages.
    let forbidden = protocol();
    let forbidden = forbidden["forbidden_specifiers"]
        .as_array()
        .expect("the frozen corpus lists forbidden specifiers");
    assert!(!forbidden.is_empty());
    for specifier in forbidden {
        let specifier = specifier.as_str().expect("specifier is text");
        for line in CLAUDE_AGENT_SDK_SIDECAR_SOURCE.lines() {
            let code = line.trim_start();
            if code.starts_with("//") {
                continue;
            }
            assert!(
                !code.contains(specifier),
                "sidecar code must never reference {specifier}"
            );
        }
    }
    // Explicit environment on every launch: omission would inherit the parent
    // environment and could silently select API-key authentication.
    assert!(CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains("env: childEnvironment()"));
    assert!(CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains("settingSources: []"));
    assert!(CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains("skills: []"));
    assert!(CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains("persistSession: false"));
}

#[test]
fn the_sidecar_holds_an_independently_joinable_native_handle() {
    // The SDK supplies a discarded bounded wait, not a join, so the sidecar
    // must retain its own handle and report only what it observed of it.
    assert!(CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains("spawnClaudeCodeProcess"));
    assert!(CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains("class NativeChild"));
    assert!(CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains("detached: false"));
    for observation in ["exited", "survivor"] {
        assert!(CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains(observation));
    }
    // The sidecar owns no cleanup vocabulary: it cannot escalate, speak for
    // the owned tree, or call anything clean.
    for reserved in ["OwnedTreeEmpty", "CleanupOutcome", "escalated"] {
        assert!(
            !CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains(reserved),
            "the sidecar must not carry the host's {reserved} vocabulary"
        );
    }
    let protocol = protocol();
    assert_eq!(
        protocol["sidecar_native_join_observations"]
            .as_array()
            .expect("native join observations are listed")
            .len(),
        2
    );
    // Route cleanup outcomes stay Rust-side and evidence-keyed.
    assert_eq!(
        protocol["route_cleanup_outcomes"]
            .as_array()
            .expect("route cleanup outcomes are listed")
            .len(),
        3
    );
}

#[test]
fn the_addable_route_exposes_references_and_no_sign_in_action() {
    let host = ExecutionHostId::new("claude-agent-sdk.fixture.addable").expect("valid host");
    let services = HostServices::new(host);
    let unavailable = claude_agent_sdk_addable_route_descriptor(&services);
    assert_eq!(
        unavailable.availability(),
        swallowtail_core::AddableRouteAvailability::Unavailable(
            swallowtail_core::AddableRouteMissingRequirement::HostService
        )
    );
    assert_eq!(unavailable.id().as_str(), CLAUDE_AGENT_SDK_ADDABLE_ROUTE_ID);
    assert_eq!(
        unavailable.topology(),
        swallowtail_core::RouteTopology::Installed
    );
    let fields: Vec<&str> = unavailable
        .config_fields()
        .map(|field| field.id().as_str())
        .collect();
    assert_eq!(fields, ["environment", "launch_recipe"]);
    let credentials: Vec<&str> = unavailable
        .credential_fields()
        .map(|field| field.id().as_str())
        .collect();
    assert_eq!(credentials, ["delegated_subscription"]);
    assert_eq!(unavailable.sign_in_actions().count(), 0);
}

#[test]
fn the_asset_can_never_auto_allow_or_bypass_admission() {
    // `allowedTools` auto-allows without prompting, and every auto-approving
    // upstream permission mode skips `canUseTool` entirely. Neither may be
    // reachable from the shipped asset.
    for reserved in [
        "allowedTools",
        "permissionPrompts",
        "permissionPromptToolName",
    ] {
        assert!(
            !CLAUDE_AGENT_SDK_SIDECAR_SOURCE
                .lines()
                .filter(|line| !line.trim_start().starts_with("//"))
                .any(|line| names_option(line, reserved)),
            "the sidecar must never set {reserved}"
        );
    }
    let protocol = protocol();
    for mode in protocol["rejected_permission_modes"]
        .as_array()
        .expect("rejected permission modes are listed")
    {
        let mode = mode.as_str().expect("mode is text");
        assert!(
            CLAUDE_AGENT_SDK_SIDECAR_SOURCE.contains(mode),
            "{mode} must be refused by name, not merely omitted"
        );
    }
    assert_eq!(
        protocol["permission_modes"],
        serde_json::json!(["default", "plan", "acceptEdits"])
    );
    assert_eq!(
        protocol["commands"],
        serde_json::json!(["open", "query", "interrupt", "set_permission_mode", "close"])
    );
    assert_eq!(
        protocol["default_tools"],
        serde_json::json!(["Read", "Glob", "Grep"]),
        "the default admitted set is unchanged"
    );
}

#[test]
fn tool_admission_uses_a_route_local_namespace() {
    // Research 279 leaves shared vocabulary to orchestrator integration, so
    // this route does not reuse the ACP permission namespace.
    assert_eq!(
        claude_agent_sdk_tool_admission_namespace().as_str(),
        "claude-agent-sdk/can-use-tool"
    );
    assert_ne!(
        claude_agent_sdk_tool_admission_namespace(),
        swallowtail_adapter_claude_agent::claude_agent_permission_namespace()
    );
}
