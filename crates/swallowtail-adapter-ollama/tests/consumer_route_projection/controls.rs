use swallowtail_runtime::{
    ConsumerRouteLifecycle, ConsumerRouteOmissionSemantics, ConsumerRouteValueKind,
};

use super::fixtures::{
    Host, inference_contribution, inference_maximal_contribution, session_contribution,
};
use super::naming::{admitted, row, rows, source};

#[test]
fn optional_request_rows_appear_only_under_the_maximal_attempt() {
    let minimal = inference_contribution("ollama.controls.minimal");
    let maximal = inference_maximal_contribution("ollama.controls.maximal");
    let minimal_rows = rows(&minimal);
    let maximal_rows = rows(&maximal);
    assert!(!minimal_rows.contains("feature.reasoning-selection"));
    assert!(!minimal_rows.contains("feature.structured-output"));
    assert!(!minimal_rows.contains("control.reasoning-selection"));
    assert!(!minimal_rows.contains("control.structured-output"));
    assert!(!minimal_rows.contains("control.context-window"));
    assert!(maximal_rows.contains("feature.reasoning-selection"));
    assert!(maximal_rows.contains("feature.structured-output"));
    assert!(maximal_rows.contains("control.reasoning-selection"));
    assert!(maximal_rows.contains("control.structured-output"));
    assert!(maximal_rows.contains("control.context-window"));
    assert_eq!(
        admitted(&maximal, "control.reasoning-selection"),
        ["high".to_owned()]
    );
    assert_eq!(
        admitted(&maximal, "control.context-window"),
        ["4096".to_owned()]
    );
    assert_eq!(
        admitted(&maximal, "control.maximum-output-tokens"),
        ["8".to_owned()]
    );
    let reasoning = row(&maximal, "control.reasoning-selection")
        .control_value()
        .expect("reasoning value");
    assert_eq!(reasoning.kind(), ConsumerRouteValueKind::BoundedEnumeration);
    assert_eq!(
        reasoning.omission(),
        ConsumerRouteOmissionSemantics::PreservesRouteBehavior
    );
    let structured = row(&maximal, "control.structured-output")
        .control_value()
        .expect("structured value");
    assert_eq!(
        structured.kind(),
        ConsumerRouteValueKind::StructuredDeclarations
    );
    assert_eq!(
        structured.omission(),
        ConsumerRouteOmissionSemantics::SuppliesNothing
    );
}

#[test]
fn session_context_window_is_shape_local_and_reasoning_stays_withheld() {
    let published = session_contribution("ollama.controls.session");
    assert_eq!(
        admitted(&published, "control.context-window"),
        ["4096".to_owned()]
    );
    assert_eq!(
        row(&published, "control.context-window").lifecycle(),
        ConsumerRouteLifecycle::SessionStartOnly
    );
    assert!(!rows(&published).contains("control.reasoning-selection"));
    let omitted = Host::new()
        .session_without_context()
        .consumer_route_projection_contribution(source("ollama.controls.session-min"))
        .expect("session contributes");
    assert!(!rows(&omitted).contains("control.context-window"));
    assert!(rows(&omitted).contains("control.model-selection"));
}
