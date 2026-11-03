use super::{
    CLAUDE_AGENT_SDK_NATIVE_AXIS, CLAUDE_AGENT_SDK_NODE_AXIS, CLAUDE_AGENT_SDK_PACKAGE_AXIS,
    CLAUDE_AGENT_SDK_SIDECAR_AXIS, CLAUDE_AGENT_SDK_WIRE_AXIS, claude_agent_sdk_native_binding,
    claude_agent_sdk_native_claim, claude_agent_sdk_node_binding, claude_agent_sdk_node_claim,
    claude_agent_sdk_package_binding, claude_agent_sdk_package_claim,
    claude_agent_sdk_sidecar_binding, claude_agent_sdk_sidecar_claim,
    claude_agent_sdk_wire_binding, claude_agent_sdk_wire_claim,
};
use crate::sdk::{
    CLAUDE_AGENT_SDK_BASELINE_VERSION, CLAUDE_AGENT_SDK_BEHAVIOR,
    CLAUDE_AGENT_SDK_NATIVE_BASELINE_VERSION, CLAUDE_AGENT_SDK_NATIVE_VERSION,
    CLAUDE_AGENT_SDK_NODE_RUNTIME, CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG, CLAUDE_AGENT_SDK_VERSION,
    CLAUDE_AGENT_SDK_WIRE, ClaudeAgentSdkPackageNativePair,
};
use swallowtail_core::InterfaceVersion;

fn version(value: &str) -> InterfaceVersion {
    InterfaceVersion::new(value).expect("fixture version is valid")
}

#[test]
fn package_claim_qualifies_each_published_sdk_hop_in_the_maintained_segment() {
    let claim = claude_agent_sdk_package_claim();
    assert_eq!(claim.axis().as_str(), CLAUDE_AGENT_SDK_PACKAGE_AXIS);
    assert_eq!(
        claim.baseline(),
        &version(CLAUDE_AGENT_SDK_BASELINE_VERSION)
    );
    assert_eq!(claim.latest_qualified(), &version(CLAUDE_AGENT_SDK_VERSION));
    assert_eq!(claim.milestones().len(), 1);
    for pair in ClaudeAgentSdkPackageNativePair::MAINTAINED {
        let patch = pair
            .package_version()
            .strip_prefix("0.3.")
            .expect("maintained package version");
        let point = version(pair.package_version());
        let assessment = claim.assess(&point);
        assert!(assessment.is_permitted(), "published package patch {patch}");
        assert_eq!(
            assessment.behavior_revision().unwrap().as_str(),
            CLAUDE_AGENT_SDK_BEHAVIOR
        );
    }
    // The window claim spans its segment arithmetically; the unpublished
    // gap points inside it (0.3.297, 0.3.298, 0.3.313) are kept out by the
    // finite maintained pair map, not by this claim.
    for rejected in ["0.3.283", "0.3.318", "0.3.259-rc.1"] {
        assert!(
            !claim.permits(&version(rejected)),
            "unqualified point {rejected} must be rejected"
        );
    }
    assert!(claude_agent_sdk_package_binding(CLAUDE_AGENT_SDK_VERSION).is_some());
    for value in ["", " 0.3.270", "latest", "0.3.270 "] {
        assert!(claude_agent_sdk_package_binding(value).is_none());
    }
}

#[test]
fn native_and_node_axes_stay_separate_from_the_wrapper_axis() {
    let native = claude_agent_sdk_native_claim();
    assert_eq!(native.axis().as_str(), CLAUDE_AGENT_SDK_NATIVE_AXIS);
    assert_eq!(
        native.baseline(),
        &version(CLAUDE_AGENT_SDK_NATIVE_BASELINE_VERSION)
    );
    assert_eq!(
        native.latest_qualified(),
        &version(CLAUDE_AGENT_SDK_NATIVE_VERSION)
    );
    assert_eq!(native.milestones().len(), 1);
    for patch in ClaudeAgentSdkPackageNativePair::MAINTAINED
        .iter()
        .map(|pair| {
            pair.native_version()
                .strip_prefix("2.1.")
                .expect("maintained native version")
                .to_owned()
        })
    {
        assert!(native.permits(&version(&format!("2.1.{patch}"))));
    }
    // The Claude Code routes sit on the same native version family. Their
    // qualification never transfers here and this one never transfers back.
    for rejected in ["2.1.283", "2.1.318", "0.3.293"] {
        assert!(!native.permits(&version(rejected)));
    }
    assert!(claude_agent_sdk_native_binding(CLAUDE_AGENT_SDK_NATIVE_VERSION).is_some());
    assert!(claude_agent_sdk_native_binding("2.1").is_none());

    let node = claude_agent_sdk_node_claim();
    assert_eq!(node.axis().as_str(), CLAUDE_AGENT_SDK_NODE_AXIS);
    assert_eq!(node.id().as_str(), "claude-agent.sdk.node-window-1");
    assert_eq!(
        node.newer_version_posture(),
        swallowtail_core::InterfaceNewerVersionPosture::QualifiedOnly
    );
    assert_eq!(node.milestones().len(), 1);
    assert_eq!(
        node.milestones()
            .next()
            .expect("Node claim has its qualified segment")
            .behavior_revision()
            .as_str(),
        CLAUDE_AGENT_SDK_BEHAVIOR
    );
    assert_eq!(node.baseline(), &version("22.23.2"));
    assert_eq!(
        node.latest_qualified(),
        &version(CLAUDE_AGENT_SDK_NODE_RUNTIME)
    );
    assert_eq!(node.exclusions().len(), 0);
    for qualified in ["22.23.2", "22.23.3"] {
        assert!(node.permits(&version(qualified)), "missing {qualified}");
    }
    for rejected in ["22.23.1", "22.23.4", "18.0.0", "23.0.0"] {
        assert!(!node.permits(&version(rejected)));
    }
    assert_eq!(CLAUDE_AGENT_SDK_NODE_RUNTIME, "22.23.3");
    assert!(claude_agent_sdk_node_binding(CLAUDE_AGENT_SDK_NODE_RUNTIME).is_some());
    assert!(claude_agent_sdk_node_binding("22.x").is_none());
}

#[test]
fn package_and_native_points_keep_the_published_coupling() {
    for pair in ClaudeAgentSdkPackageNativePair::MAINTAINED {
        let package = pair.package_version();
        let native = pair.native_version();
        assert!(super::package_native_pair_matches(package, native));
        assert_eq!(
            package.strip_prefix("0.3.").expect("package prefix"),
            native.strip_prefix("2.1.").expect("native prefix")
        );
    }
    // The unpublished gap points are not maintained pairs even though the
    // window claim spans them arithmetically and their patch numbers would
    // satisfy the coupling predicate.
    for patch in [297, 298, 313] {
        let package = format!("0.3.{patch}");
        let native = format!("2.1.{patch}");
        assert!(super::package_native_pair_matches(&package, &native));
        assert!(
            !ClaudeAgentSdkPackageNativePair::MAINTAINED
                .iter()
                .any(|pair| pair.package_version() == package)
        );
        assert!(
            !ClaudeAgentSdkPackageNativePair::MAINTAINED
                .iter()
                .any(|pair| pair.native_version() == native)
        );
    }
    assert!(!super::package_native_pair_matches("0.3.292", "2.1.293"));
    assert!(!super::package_native_pair_matches("0.3.293", "2.1.292"));
    assert!(!super::package_native_pair_matches("0.3.316", "2.1.317"));
}

#[test]
fn default_pair_and_previous_ten_pairs_remain_unchanged() {
    assert_eq!(
        ClaudeAgentSdkPackageNativePair::DEFAULT.package_version(),
        "0.3.293"
    );
    assert_eq!(
        ClaudeAgentSdkPackageNativePair::DEFAULT.native_version(),
        "2.1.293"
    );
    for (patch, pair) in ClaudeAgentSdkPackageNativePair::MAINTAINED[..10]
        .iter()
        .enumerate()
    {
        assert_eq!(pair.package_version(), format!("0.3.{}", 284 + patch));
        assert_eq!(pair.native_version(), format!("2.1.{}", 284 + patch));
    }
}

#[test]
fn opaque_claims_qualify_only_the_exact_wire_and_sidecar_points() {
    let wire = claude_agent_sdk_wire_claim();
    assert_eq!(wire.axis().as_str(), CLAUDE_AGENT_SDK_WIRE_AXIS);
    assert!(wire.permits(&version(CLAUDE_AGENT_SDK_WIRE)));
    assert!(!wire.permits(&version("swallowtail-claude-agent-sdk-jsonl-v2")));
    // The ACP route's wire identity is a different axis entirely.
    assert!(claude_agent_sdk_wire_binding("acp-v1").is_none());
    assert!(claude_agent_sdk_wire_binding(CLAUDE_AGENT_SDK_WIRE).is_some());

    let sidecar = claude_agent_sdk_sidecar_claim();
    assert_eq!(sidecar.axis().as_str(), CLAUDE_AGENT_SDK_SIDECAR_AXIS);
    assert!(sidecar.permits(&version(CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG)));
    assert!(!sidecar.permits(&version("swallowtail-claude-agent-sdk-sidecar@0.0.0")));
    assert!(claude_agent_sdk_sidecar_binding(CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG).is_some());
    assert!(claude_agent_sdk_sidecar_binding("").is_none());
}

#[test]
fn every_axis_is_distinct() {
    let axes = [
        CLAUDE_AGENT_SDK_PACKAGE_AXIS,
        CLAUDE_AGENT_SDK_NATIVE_AXIS,
        CLAUDE_AGENT_SDK_NODE_AXIS,
        CLAUDE_AGENT_SDK_WIRE_AXIS,
        CLAUDE_AGENT_SDK_SIDECAR_AXIS,
    ];
    let unique: std::collections::BTreeSet<&str> = axes.into_iter().collect();
    assert_eq!(unique.len(), axes.len());
    for axis in axes {
        assert!(axis.starts_with("claude-agent.sdk."));
        assert_ne!(axis, crate::CLAUDE_AGENT_ACP_AXIS);
        assert_ne!(axis, crate::CLAUDE_CODE_HEADLESS_AXIS);
        assert_ne!(axis, crate::CLAUDE_CODE_RESPONSE_ONLY_AXIS);
    }
}
