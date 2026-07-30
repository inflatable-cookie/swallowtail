use super::{PROTOCOL, RELEASES};
use crate::claude_agent_support::{compatibility_claim, version};
use crate::support::parse_json;
use swallowtail_core::InterfaceCompatibilityAssessment;
use swallowtail_protocol_acp::ACP_PROTOCOL_VERSION;

#[test]
fn release_corpus_freezes_every_candidate_point_and_gap() {
    let corpus = parse_json(RELEASES);
    let releases = corpus["releases"].as_array().expect("release list");

    assert_eq!(corpus["publication"]["candidate_first"], "0.52.0");
    assert_eq!(corpus["publication"]["qualified_first"], "0.53.0");
    assert_eq!(corpus["publication"]["last"], "0.61.0");
    assert_eq!(corpus["publication"]["published_count"], 11);
    assert_eq!(
        corpus["publication"]["missing"],
        serde_json::json!(["0.58.0"])
    );
    assert_eq!(releases.len(), 11);
    assert_eq!(
        releases
            .iter()
            .filter(|release| release["qualified"] == true)
            .count(),
        10
    );
    assert_eq!(releases[0]["version"], "0.52.0");
    assert_eq!(releases[0]["qualified"], false);
    assert_eq!(releases.last().expect("latest")["version"], "0.61.0");
}

#[test]
fn compatibility_claim_uses_four_milestones_and_allows_visible_newer_execution() {
    let claim = compatibility_claim();

    assert_eq!(claim.baseline().as_str(), "0.53.0");
    assert_eq!(claim.latest_qualified().as_str(), "0.61.0");
    assert_eq!(claim.milestones().len(), 4);
    assert_eq!(claim.exclusions().len(), 2);

    for (candidate, behavior) in [
        ("0.53.0", "claude-agent.acp.baseline-v1"),
        ("0.54.1", "claude-agent.acp.session-config-v2"),
        ("0.58.1", "claude-agent.acp.session-config-v2"),
        ("0.59.0", "claude-agent.acp.session-config-v2"),
        ("0.60.0", "claude-agent.acp.provider-capability-v3"),
        ("0.61.0", "claude-agent.acp.steering-metadata-v4"),
    ] {
        let InterfaceCompatibilityAssessment::Qualified(matched) =
            claim.assess(&version(candidate))
        else {
            panic!("{candidate} must be qualified");
        };
        assert_eq!(matched.behavior_revision().as_str(), behavior);
    }

    for rejected in ["0.52.0", "0.58.0", "0.61.0-rc.1", "malformed"] {
        assert_eq!(
            claim.assess(&version(rejected)),
            InterfaceCompatibilityAssessment::Incompatible
        );
    }

    let InterfaceCompatibilityAssessment::UnverifiedNewer(newer) = claim.assess(&version("0.62.0"))
    else {
        panic!("stable newer release must be permitted as unverified");
    };
    assert_eq!(newer.version().as_str(), "0.62.0");
    assert_eq!(newer.latest_qualified().as_str(), "0.61.0");
    assert_eq!(
        newer.behavior_revision().as_str(),
        "claude-agent.acp.steering-metadata-v4"
    );
}

#[test]
fn wrapper_sdk_native_binary_wire_and_model_axes_stay_separate() {
    let corpus = parse_json(RELEASES);
    let protocol = parse_json(PROTOCOL);
    let releases = corpus["releases"].as_array().expect("release list");

    for release in releases {
        let sdk = release["agent_sdk"].as_str().expect("Agent SDK version");
        let native = release["native_cli"].as_str().expect("native version");
        assert_ne!(release["version"], release["agent_sdk"]);
        assert_ne!(release["version"], release["native_cli"]);
        assert_eq!(
            sdk.strip_prefix("0.3.").expect("SDK patch"),
            native.strip_prefix("2.1.").expect("native patch")
        );
        assert_eq!(
            release["native_cli_sha256"]
                .as_str()
                .expect("native digest")
                .len(),
            64
        );
    }

    assert_eq!(protocol["identity"]["wire_version"], ACP_PROTOCOL_VERSION);
    assert_eq!(
        protocol["identity"]["artifact"],
        "@agentclientprotocol/claude-agent-acp"
    );
    assert_eq!(
        protocol["observation"]["wrapper_argv"],
        serde_json::json!(["--version"])
    );
    assert_eq!(
        protocol["observation"]["nested_binary_argv"],
        serde_json::json!(["--cli", "--version"])
    );
    assert_eq!(
        protocol["observation"]["candidate_rejection"]["version"],
        "0.52.0"
    );
    assert_eq!(
        protocol["observation"]["candidate_rejection"]["nested_probe_safe"],
        false
    );
}
