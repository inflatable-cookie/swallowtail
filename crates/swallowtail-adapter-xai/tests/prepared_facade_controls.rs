mod support;

use std::num::NonZeroU64;
use swallowtail_adapter_xai::{
    XaiRunProfileInput, XaiSessionProfileInput, prepare_xai_responses_websocket,
};
use swallowtail_core::{Capability, CapabilityConstraint, ReasoningMode};
use swallowtail_runtime::{OperationContent, RequestId};

use support::{
    DriverFixture, ServerScenario, assert_generation_requirement, assert_output_edge,
    qualified_model,
};

#[test]
fn prepared_xai_generation_controls_are_independent_and_exact() {
    for (label, reasoning, maximum) in [
        ("reasoning", Some("high"), None),
        ("output", None, Some(512)),
        ("both", Some("xhigh"), Some(512)),
    ] {
        let fixture = DriverFixture::new(ServerScenario::OneResponse);
        let prepared =
            prepare_xai_responses_websocket(fixture.preparation_input(), &fixture.services())
                .expect("xAI integration prepares");
        let mut input = XaiRunProfileInput::new(
            RequestId::new(format!("controls-{label}")).expect("request id is valid"),
            qualified_model("grok-4.6"),
            OperationContent::new("controlled response").expect("content is valid"),
            None,
        );
        if let Some(reasoning) = reasoning {
            input = input.with_reasoning_mode(ReasoningMode::new(reasoning).expect("mode"));
        }
        if let Some(maximum) = maximum {
            input = input.with_maximum_output_tokens(
                NonZeroU64::new(maximum).expect("maximum output is positive"),
            );
        }
        let operation = prepared
            .prepare_responses_run(input)
            .expect("controlled run prepares");
        assert_eq!(
            operation
                .evidence()
                .reasoning_mode()
                .map(ReasoningMode::as_str),
            reasoning
        );
        assert_eq!(
            operation
                .evidence()
                .maximum_output_tokens()
                .map(NonZeroU64::get),
            maximum
        );
        assert_eq!(
            operation
                .request()
                .policy()
                .reasoning_mode()
                .map(ReasoningMode::as_str),
            reasoning
        );
        assert_eq!(
            operation
                .request()
                .maximum_output_tokens()
                .map(NonZeroU64::get),
            maximum
        );
        assert_generation_requirement(
            operation.plan(),
            Capability::ReasoningSelection,
            reasoning.map(|value| {
                CapabilityConstraint::ReasoningMode(ReasoningMode::new(value).expect("mode"))
            }),
        );
        assert_generation_requirement(
            operation.plan(),
            Capability::OutputTokenLimit,
            maximum.map(CapabilityConstraint::OutputTokenMaximum),
        );
    }
}

#[test]
fn prepared_xai_exact_reasoning_rows_prepare_on_both_profiles() {
    let rows: &[(&str, &[&str])] = &[
        ("grok-4.5", &["low", "medium", "high"]),
        ("grok-4.6", &["low", "medium", "high", "xhigh"]),
    ];
    for &(model_id, reasoning_values) in rows {
        for &reasoning_value in reasoning_values {
            let fixture = DriverFixture::new(ServerScenario::OneResponse);
            let prepared =
                prepare_xai_responses_websocket(fixture.preparation_input(), &fixture.services())
                    .expect("xAI integration prepares");
            let reasoning = ReasoningMode::new(reasoning_value).expect("reasoning is valid");
            let run = prepared
                .prepare_responses_run(
                    XaiRunProfileInput::new(
                        RequestId::new(format!("row-run-{model_id}-{reasoning_value}"))
                            .expect("request id is valid"),
                        qualified_model(model_id),
                        OperationContent::new("row coverage").expect("content is valid"),
                        None,
                    )
                    .with_reasoning_mode(reasoning.clone()),
                )
                .expect("exact reasoning row prepares for run");
            assert_eq!(
                run.evidence().reasoning_mode().map(ReasoningMode::as_str),
                Some(reasoning_value)
            );
            assert_generation_requirement(
                run.plan(),
                Capability::ReasoningSelection,
                Some(CapabilityConstraint::ReasoningMode(reasoning.clone())),
            );

            let session = prepared
                .prepare_responses_session(
                    XaiSessionProfileInput::new(
                        RequestId::new(format!("row-session-{model_id}-{reasoning_value}"))
                            .expect("request id is valid"),
                        qualified_model(model_id),
                        None,
                    )
                    .with_reasoning_mode(reasoning),
                )
                .expect("exact reasoning row prepares for session");
            assert_eq!(
                session
                    .evidence()
                    .reasoning_mode()
                    .map(ReasoningMode::as_str),
                Some(reasoning_value)
            );
            assert_generation_requirement(
                session.plan(),
                Capability::ReasoningSelection,
                Some(CapabilityConstraint::ReasoningMode(
                    ReasoningMode::new(reasoning_value).expect("reasoning is valid"),
                )),
            );
        }
    }
}

#[test]
fn prepared_xai_output_bound_edges_prepare_on_both_profiles() {
    for maximum in [1, i32::MAX as u64] {
        let maximum = NonZeroU64::new(maximum).expect("maximum is positive");
        let fixture = DriverFixture::new(ServerScenario::OneResponse);
        let prepared =
            prepare_xai_responses_websocket(fixture.preparation_input(), &fixture.services())
                .expect("xAI integration prepares");
        let run = prepared
            .prepare_responses_run(
                XaiRunProfileInput::new(
                    RequestId::new(format!("edge-run-{maximum}")).expect("request id is valid"),
                    qualified_model("grok-4.6"),
                    OperationContent::new("output edge").expect("content is valid"),
                    None,
                )
                .with_maximum_output_tokens(maximum),
            )
            .expect("output edge prepares for run");
        assert_output_edge(&run, maximum);

        let session = prepared
            .prepare_responses_session(
                XaiSessionProfileInput::new(
                    RequestId::new(format!("edge-session-{maximum}")).expect("request id is valid"),
                    qualified_model("grok-4.6"),
                    None,
                )
                .with_maximum_output_tokens(maximum),
            )
            .expect("output edge prepares for session");
        assert_eq!(
            session
                .evidence()
                .maximum_output_tokens()
                .map(NonZeroU64::get),
            Some(maximum.get())
        );
        assert_generation_requirement(
            session.plan(),
            Capability::OutputTokenLimit,
            Some(CapabilityConstraint::OutputTokenMaximum(maximum.get())),
        );
    }
}
