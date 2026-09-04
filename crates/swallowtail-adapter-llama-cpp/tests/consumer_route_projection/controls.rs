use swallowtail_runtime::{
    ConsumerRouteLifecycle, ConsumerRouteOmissionSemantics, ConsumerRouteValueKind,
};

use super::fixtures::{inference_contribution, serving_contribution, serving_omitted};
use super::naming::{admitted, row, source};

#[test]
fn attached_model_and_output_token_controls_come_from_the_prepared_attempt() {
    let published = inference_contribution("llama-cpp.attached.controls");
    assert_eq!(
        admitted(&published, "control.model-selection"),
        ["swallowtail-fixture-stories260k".to_owned()]
    );
    assert_eq!(
        admitted(&published, "control.maximum-output-tokens"),
        ["8".to_owned()]
    );
    let model = row(&published, "control.model-selection");
    assert_eq!(model.lifecycle(), ConsumerRouteLifecycle::SelectionSummary);
    assert_eq!(
        model.control_value().expect("model value").kind(),
        ConsumerRouteValueKind::ExactModelRoute
    );
    assert_eq!(
        model.control_value().expect("model value").omission(),
        ConsumerRouteOmissionSemantics::Required
    );
    let maximum = row(&published, "control.maximum-output-tokens");
    assert_eq!(
        maximum.lifecycle(),
        ConsumerRouteLifecycle::SessionStartOnly
    );
    assert_eq!(
        maximum.control_value().expect("token value").kind(),
        ConsumerRouteValueKind::BoundedInteger
    );
    assert_eq!(
        maximum.control_value().expect("token value").omission(),
        ConsumerRouteOmissionSemantics::Required
    );
}

#[test]
fn owned_serving_controls_restate_the_maximal_prepared_binding() {
    let published = serving_contribution("llama-cpp.owned.controls");
    let artifact = row(&published, "control.serving-model-artifact");
    assert_eq!(
        artifact.lifecycle(),
        ConsumerRouteLifecycle::SessionStartOnly
    );
    assert_eq!(
        artifact.control_value().expect("artifact value").kind(),
        ConsumerRouteValueKind::StructuredDeclarations
    );
    assert_eq!(
        artifact.control_value().expect("artifact value").omission(),
        ConsumerRouteOmissionSemantics::Required
    );
    assert_eq!(
        admitted(&published, "control.serving-context-size"),
        ["4096".to_owned()]
    );
    assert_eq!(
        admitted(&published, "control.serving-reasoning"),
        ["off".to_owned()]
    );
    let context = row(&published, "control.serving-context-size")
        .control_value()
        .expect("context value");
    assert_eq!(context.kind(), ConsumerRouteValueKind::BoundedInteger);
    assert_eq!(
        context.omission(),
        ConsumerRouteOmissionSemantics::PreservesRouteBehavior
    );
    let reasoning = row(&published, "control.serving-reasoning")
        .control_value()
        .expect("reasoning value");
    assert_eq!(reasoning.kind(), ConsumerRouteValueKind::BoundedEnumeration);
    assert_eq!(
        reasoning.omission(),
        ConsumerRouteOmissionSemantics::PreservesRouteBehavior
    );
}

#[test]
fn omitted_owned_serving_options_are_withheld_at_construction() {
    let published = serving_omitted()
        .consumer_route_projection_contribution(source("llama-cpp.owned.omitted"))
        .expect("omitted serving contributes");
    let semantics = super::naming::rows(&published);
    assert!(semantics.contains("control.serving-model-artifact"));
    assert!(!semantics.contains("control.serving-context-size"));
    assert!(!semantics.contains("control.serving-reasoning"));
    assert!(!semantics.contains("feature.activity-observation"));
    assert!(!semantics.contains("feature.streaming-events"));
    assert!(!semantics.contains("feature.cancellation-or-interruption"));
}
