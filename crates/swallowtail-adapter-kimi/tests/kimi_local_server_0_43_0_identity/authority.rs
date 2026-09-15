//! Local-server Bash cwd containment proofs for Research 326.
//!
//! These fail if the safe prefix quietly narrows, if the uncontained gap is
//! re-admitted, if a containing control appears without evidence, or if the
//! conclusion leaks onto ACP or headless.

use super::support::{IDENTITY, PROTOCOL, json, text};

#[test]
fn the_decision_is_extend_and_fail_closed_on_authority_evidence() {
    let decision = &json(IDENTITY)["identity_decision"];
    assert_eq!(decision["shape"], "extend-and-fail-closed");
    assert_eq!(decision["safe_prefix_becomes"], "0.35.0..=0.39.1");
    assert_eq!(decision["latest_qualified_becomes"], "0.39.1");
    assert_eq!(decision["posture_becomes"], "qualified_only");
    assert_eq!(decision["rejected_gap"], "0.40.0..=0.43.0");
    assert_eq!(decision["compatible_extension"], true);
    assert_eq!(decision["compatible_extension_span"], "0.39.0..=0.39.1");
    assert_eq!(decision["private_milestone"], false);
    assert_eq!(decision["new_behavior_revision"], false);
    assert_eq!(decision["new_public_operation"], false);
    assert_eq!(decision["public_api_change"], false);
    assert_eq!(decision["authority_first"], true);
    assert_eq!(decision["wire_shape_stability_alone_is_insufficient"], true);
    assert!(text(decision, &["reason"]).contains("uncontained"));
}

#[test]
fn the_containment_trace_names_absent_controls_and_the_ambient_contracts() {
    let trace = &json(PROTOCOL)["containment_trace"];
    assert_eq!(trace["adapter_control_found"], false);
    assert_eq!(trace["runtime_control_found"], false);
    assert_eq!(trace["provider_boundary_remaining_for_bash_cwd"], false);
    assert_eq!(trace["process_request_sets_cwd"], false);
    assert_eq!(trace["harness_isolation_declared"], "AmbientHost");
    assert_eq!(
        trace["loopback_bind_contains_network_not_process_cwd"],
        true
    );
    assert_eq!(trace["pty_assertAllowed_is_not_the_bash_tool_path"], true);
    assert_eq!(trace["restored_containment_at_any_later_point"], false);
    assert!(
        text(trace, &["contract_017_ambient_clause"]).contains("location and callback scope only")
    );
    assert!(
        text(trace, &["contract_023_clause"])
            .contains("ambient authority of the selected execution host")
    );
    assert!(text(trace, &["conclusion"]).contains("fails closed"));
}

#[test]
fn the_a2_comparison_is_same_conclusion_with_a_named_mechanism_difference() {
    let trace = &json(PROTOCOL)["containment_trace"];
    assert_eq!(trace["same_risk_class_as_acp_a2"], true);
    assert_eq!(trace["same_conclusion_as_acp_a2"], true);
    assert!(
        text(trace, &["mechanism_differs_from_acp_a2"]).contains("provider-internal resolve()")
    );
}

#[test]
fn bash_cwd_change_is_in_resolve_from_0_40_0_and_holds_through_0_43_0() {
    let cwd = &json(PROTOCOL)["bash_cwd_authority"];
    assert_eq!(cwd["first_published"], "0.40.0");
    assert_eq!(cwd["holds_through"], "0.43.0");
    assert_eq!(
        cwd["bash_tool_call"],
        "view.resolve(args.cwd ?? view.workDir)"
    );
    assert_eq!(
        cwd["after"]["bash_cwd_outside_workspace"],
        "accepted; test covers cd '/outside/workspace' && pwd"
    );
    assert_eq!(cwd["after"]["pty_create_still_assertAllowed"], true);
    assert_ne!(
        cwd["runtime_workspace_view_ts"]["0.39.0..=0.39.1"],
        cwd["runtime_workspace_view_ts"]["0.40.0..=0.43.0"]
    );
    assert_eq!(cwd["before"]["bash_cwd_outside_workspace"], "throws");
}
