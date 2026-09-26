//! Bash cwd containment and disabled-tool proofs for Research 353.

use super::support::{PROTOCOL, json, text};

#[test]
fn uncontained_resolve_holds_byte_identical_through_2_1_1() {
    let cwd = &json(PROTOCOL)["bash_cwd_authority"];
    assert_eq!(cwd["first_published"], "0.40.0");
    assert_eq!(cwd["holds_through"], "2.1.1");
    assert_eq!(cwd["bash_tool_name"], "Bash");
    assert_eq!(
        cwd["bash_tool_call"],
        "view.resolve(args.cwd ?? view.workDir)"
    );
    assert_eq!(
        cwd["runtime_workspace_view_ts"]["0.40.0..=2.1.1"],
        "01db1bbf8f51cc1b16178d3d4e07817d4b63d372"
    );
    assert_eq!(
        cwd["bash_tool_ts"]["0.39.0..=2.1.1"],
        "41090010ce96a7c66586fda8dc17b3a19d62ceaf"
    );
    assert_eq!(cwd["after"]["assertAllowed_called_from_resolve"], false);
    assert_eq!(cwd["after"]["assertAllowed_exists_separately"], true);
    assert_eq!(cwd["after"]["pty_create_still_assertAllowed"], true);
}

#[test]
fn disabled_tools_can_hide_bash_and_does_not_contain_the_process() {
    let control = &json(PROTOCOL)["disabled_tools_control"];
    assert_eq!(control["route_already_maps_from"], "0.29.0");
    assert_eq!(control["rest_field"], "disabled_tools");
    assert_eq!(control["exact_tool_name"], "Bash");
    assert_eq!(control["wrong_case_is_unknown_tool"], "bash");
    assert_eq!(control["setModel_binds_default_profile"], "agent");
    assert_eq!(control["explicit_consumer_profile_required"], false);
    assert_eq!(
        control["evaluator_blob_unchanged_0_43_0_through_2_1_1"],
        "ae563141578d235a823456a40c81d2348791ca31"
    );
    assert_eq!(
        control["executor_guard_rejects_with"],
        "Tool \"Bash\" is disabled by the active tool policy"
    );
    assert_eq!(control["hides_uncontained_tool"], true);
    assert_eq!(control["contains_ambient_host_process"], false);
    assert_eq!(control["narrows_consumer_visible_bash_tool"], true);
    assert_eq!(
        control["structured_runs_share_interactive_prompt_path"],
        true
    );
}

#[test]
fn the_containment_trace_names_the_denylist_and_the_ambient_contracts() {
    let trace = &json(PROTOCOL)["containment_trace"];
    assert_eq!(trace["harness_isolation_declared"], "AmbientHost");
    assert_eq!(trace["adapter_control_found"], true);
    assert_eq!(trace["adapter_control_restores_assertAllowed"], false);
    assert_eq!(trace["adapter_control_contains_process"], false);
    assert_eq!(trace["runtime_control_found"], false);
    assert_eq!(trace["provider_boundary_remaining_for_bash_cwd"], false);
    assert_eq!(trace["process_request_sets_cwd"], false);
    assert_eq!(trace["restored_containment_at_any_later_point"], false);
    assert!(
        text(trace, &["contract_023_tool_clause"]).contains("do not contain the harness process")
    );
    assert!(text(trace, &["conclusion"]).contains("narrows a consumer-visible guarantee"));
}
