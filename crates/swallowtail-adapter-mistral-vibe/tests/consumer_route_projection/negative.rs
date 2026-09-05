use super::{prepare, run_input};
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{ConsumerRouteControlId, ConsumerRouteRowIdentity};

#[test]
fn omitted_optional_turn_bound_does_not_create_a_control_row() {
    let host = ExecutionHostId::new("mistral-vibe.omitted.host").expect("host");
    let run = prepare(host)
        .prepare_run(run_input("omitted"))
        .expect("run");
    let contribution = run
        .consumer_route_projection_contribution(
            swallowtail_runtime::ConsumerRouteProjectionSourceId::new("omitted").unwrap(),
        )
        .expect("contribution");
    assert!(!contribution
        .session_start_rows()
        .any(|row| matches!(row.identity(), ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::Namespaced(extension)) if extension.semantic_id() == "control.maximum-agentic-turns")));
    assert!(contribution
        .selection_rows()
        .all(|row| row.identity().namespaced_extension().is_none()));
}
