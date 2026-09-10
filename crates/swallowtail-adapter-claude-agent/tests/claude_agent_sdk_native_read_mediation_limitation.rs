//! Provider-free binding of Desktop g02.049's native-read mediation limit.
//!
//! Research 303 scores the merged successor capsule: native `Read` completed
//! under SDK `default` with zero recorded `canUseTool` callbacks or decisions
//! and an unchanged fixture. This file never contacts a provider.

use serde_json::Value;
use swallowtail_adapter_claude_agent::sdk::{
    CLAUDE_AGENT_SDK_NATIVE_VERSION, CLAUDE_AGENT_SDK_NODE_RUNTIME,
    CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG, CLAUDE_AGENT_SDK_VERSION, CLAUDE_AGENT_SDK_WIRE,
};

const EVIDENCE: &str =
    include_str!("fixtures/claude-agent-sdk-0.3.259/native-read-mediation-limitation.json");

fn json(text: &str) -> Value {
    serde_json::from_str(text).expect("frozen evidence is valid JSON")
}

#[test]
fn the_limitation_binds_the_merged_desktop_identities_and_three_capsule_digests() {
    let evidence = json(EVIDENCE);
    assert_eq!(
        evidence["schema"],
        "swallowtail.claude-agent-sdk.native-read-mediation-limitation.v1"
    );
    assert_eq!(evidence["research"], "303");
    assert_eq!(evidence["task"], "g05.045");
    assert_eq!(
        evidence["desktop"]["task"],
        "bc4acd98-91a3-4e14-86e0-38376b037da7"
    );
    assert_eq!(evidence["desktop"]["pr"], "205");
    assert_eq!(
        evidence["desktop"]["pr_head"],
        "8002833f45a66e63df1071f5bb99053d6bf85922"
    );
    assert_eq!(evidence["desktop"]["review"], "5610373055");
    assert_eq!(
        evidence["desktop"]["merge"],
        "e0217483ed8584ddca1bc4f4fdfedce53bcbaa76"
    );
    assert_eq!(
        evidence["desktop"]["closeout"],
        "941dfcdb5361d673dfd5c7adb54950cacbbbe478"
    );
    assert_eq!(
        evidence["source"]["sha"],
        "49c9e3b291609c9ebf5b35a284c08302f3b8d5e3"
    );
    assert_eq!(evidence["source"]["tag"], "v0.4.4");

    let capsules = evidence["capsules"].as_array().expect("three capsules");
    assert_eq!(capsules.len(), 3);
    assert_eq!(
        capsules[0]["sha256"],
        "10b77ede7a85b68831967731e6d5636c0d44b72a56b1774b29938ed4fd519cd6"
    );
    assert_eq!(
        capsules[1]["sha256"],
        "8921fa5edc0ba869a8d565e2834fd9e138178dd87bd0bbec8e1561205ffc1386"
    );
    assert_eq!(
        capsules[2]["sha256"],
        "4f23e55c548469ae61666052266497ae70b864566e0e3cc7de1c3a208a777dba"
    );
    assert_eq!(capsules[0]["proves_native_read_completion"], false);
    assert_eq!(capsules[1]["proves_native_read_completion"], false);
    assert_eq!(capsules[2]["proves_native_read_completion"], true);
    for capsule in capsules {
        assert_eq!(capsule["sha256"].as_str().expect("sha256").len(), 64);
        assert_eq!(capsule["opens"], 1);
        assert_eq!(capsule["retries"], 0);
    }
}

#[test]
fn the_successor_records_a_native_read_result_with_zero_decisions_and_an_unchanged_fixture() {
    let evidence = json(EVIDENCE);
    let successor = &evidence["successor"];
    // Recorded zeros, not a missing optional: allow/deny are present and 0.
    assert_eq!(successor["allow_count"], 0);
    assert_eq!(successor["deny_count"], 0);
    assert_eq!(successor["can_use_tool_callbacks"], 0);
    assert_eq!(successor["can_use_tool_decisions"], 0);
    assert_eq!(successor["can_use_tool_field_present"], false);
    assert_eq!(successor["write_successes"], 0);
    assert_eq!(successor["turns_started"], 1);
    assert_eq!(successor["turns_completed"], 0);
    assert_eq!(successor["opens"], 1);
    assert_eq!(successor["retries"], 0);
    assert_eq!(successor["fallbacks"], 0);
    assert_eq!(successor["reconnects"], 0);
    assert_eq!(
        successor["failure_code"],
        "ordering.tool_result_before_decision"
    );

    let chain = successor["chain"].as_array().expect("chain");
    assert_eq!(chain.len(), 1);
    assert_eq!(chain[0]["kind"], "tool_proposal");
    assert_eq!(chain[0]["tool"], "Read");
    assert_eq!(chain[0]["phase"], "started");
    assert_eq!(chain[0]["preceding_decision"], false);
    assert!(
        !chain.iter().any(|row| row["kind"] == "canUseTool"
            || row["kind"] == "decision"
            || row["kind"] == "callback"),
        "the scored chain records no canUseTool callback or decision"
    );

    assert_eq!(
        successor["frames"],
        serde_json::json!([
            "sidecar:started",
            "sidecar:progress",
            "sidecar:progress",
            "sidecar:activity",
            "sidecar:activity"
        ])
    );

    let digest = "sha256:b6a98d9ce9a2d9149288fa3df42d377c3e42737afdcdaf714e33c0a100b51060";
    assert_eq!(successor["initial_digest"], digest);
    assert_eq!(successor["before_digest"], digest);
    assert_eq!(successor["last_digest"], digest);
    assert_eq!(successor["fixture_unchanged"], true);
    assert_eq!(successor["cleanup"], "degraded");
    assert_eq!(successor["cleanup_accepted"], true);
    assert_eq!(successor["operation_bridge_listeners"], 0);
    assert_eq!(successor["registered_tool_leases"], 0);
    assert_eq!(successor["task_reapers_joined"], true);
}

#[test]
fn the_limitation_does_not_flip_emitted_callback_or_registered_tool_truths() {
    let evidence = json(EVIDENCE);
    let tuple = &evidence["tuple"];
    assert_eq!(tuple["sdk"], CLAUDE_AGENT_SDK_VERSION);
    assert_eq!(tuple["native"], CLAUDE_AGENT_SDK_NATIVE_VERSION);
    assert_eq!(tuple["node"], CLAUDE_AGENT_SDK_NODE_RUNTIME);
    // The sidecar source tag moves with every coordinated release. The
    // capsules ran the exact source build frozen in this fixture, so the
    // tuple binds to that source's coordinated version and is never
    // silently rebased onto a newer tag; the live source tag shares only
    // the asset lineage prefix.
    let frozen_source_tag = evidence["source"]["tag"]
        .as_str()
        .expect("frozen source tag");
    let frozen_version = frozen_source_tag
        .strip_prefix('v')
        .expect("versioned frozen source tag");
    assert_eq!(
        tuple["sidecar_source_tag"],
        format!("swallowtail-claude-agent-sdk-sidecar@{frozen_version}")
    );
    assert!(
        CLAUDE_AGENT_SDK_SIDECAR_SOURCE_TAG.starts_with("swallowtail-claude-agent-sdk-sidecar@")
    );
    assert_eq!(tuple["sidecar_wire"], CLAUDE_AGENT_SDK_WIRE);
    assert_eq!(tuple["permission_mode"], "default");
    assert_eq!(tuple["model"], "claude-sonnet-5");
    assert_eq!(tuple["allowed_tools"], "omitted");

    let retained = &evidence["retained"];
    assert_eq!(retained["permission_exchange"], "Yes");
    assert_eq!(
        retained["permission_exchange_scope"],
        "SDK-emitted canUseTool callbacks only"
    );
    assert_eq!(retained["registered_tools"], "Yes");
    assert_eq!(retained["bounded_workspace_text_write"], "No");
    assert_eq!(
        evidence["limitation"],
        "universal native-tool canUseTool mediation is unavailable on the qualified tuple"
    );
    let non_claims = evidence["non_claims"].as_array().expect("non-claims");
    assert!(
        non_claims
            .iter()
            .any(|claim| claim == "universal native-tool canUseTool mediation")
    );
}
