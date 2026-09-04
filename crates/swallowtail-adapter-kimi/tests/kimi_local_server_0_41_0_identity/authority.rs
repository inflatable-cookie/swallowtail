//! Local-server Bash cwd authority proofs.
//!
//! These fail if the stop quietly becomes a compatible extension, if a
//! containing control appears without evidence, or if the conclusion leaks
//! onto ACP or headless.

use super::support::{IDENTITY, PROTOCOL, json, text};

#[test]
fn the_decision_is_stop_because_cwd_containment_is_absent() {
    let decision = &json(IDENTITY)["identity_decision"];
    assert_eq!(decision["shape"], "stop");
    assert_eq!(decision["compatible_extension"], false);
    assert_eq!(decision["private_milestone"], false);
    assert_eq!(decision["latest_qualified_stays"], "0.38.0");
    assert_eq!(decision["admit_segment_for_card_063"], false);
    assert_eq!(decision["authority_first"], true);
    assert_eq!(decision["wire_shape_stability_alone_is_insufficient"], true);
    assert!(text(decision, &["reason"]).contains("uncontained process-authority"));
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
    assert!(
        text(trace, &["contract_017_ambient_clause"]).contains("location and callback scope only")
    );
    assert!(
        text(trace, &["contract_023_clause"])
            .contains("ambient authority of the selected execution host")
    );
    assert!(text(trace, &["conclusion"]).contains("containment is absent"));
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
fn bash_cwd_change_is_in_resolve_from_0_40_0_and_holds_at_0_41_0() {
    let cwd = &json(PROTOCOL)["bash_cwd_authority"];
    assert_eq!(cwd["first_published"], "0.40.0");
    assert_eq!(cwd["holds_through"], "0.41.0");
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
        cwd["runtime_workspace_view_ts"]["0.38.0"],
        cwd["runtime_workspace_view_ts"]["0.40.0..=0.41.0"]
    );
}
