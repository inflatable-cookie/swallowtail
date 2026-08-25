use super::fixtures::PreparedFixture;
use crate::support::StreamFixture;
use futures_executor::block_on;
use futures_util::StreamExt;
use serde_json::Value;
use swallowtail_adapter_anthropic::{
    AnthropicDirectDriver, AnthropicThinkingMode, AnthropicWebSearchInput,
};
use swallowtail_core::{Capability, ExecutionHostId, ReasoningMode};
use swallowtail_runtime::{
    CleanupOutcome, OperationContent, OperationPolicy, RequestId, StructuredRunDriver,
    StructuredRunRequest, TerminalStatus,
};

const QUALIFIED_MODEL: &str = "claude-opus-4-7";
const SIGNATURE: &str = "sig_omitted_fixture_private";
const REDACTED: &str = "redacted_fixture_private_data";
const THINKING_WIRE: &str = r#"{"display":"omitted","type":"adaptive"}"#;
const EFFORT_VALUES: [&str; 5] = ["low", "medium", "high", "xhigh", "max"];

#[test]
fn prepared_adaptive_thinking_binds_evidence_and_exact_omitted_wire() {
    let fixture = PreparedFixture::with_stream(
        ExecutionHostId::new("anthropic.thinking.run").unwrap(),
        StreamFixture::ThinkingThenText,
    );
    let attempt = fixture
        .prepared()
        .prepare_inference_attempt(
            fixture
                .attempt_input_for_model("thinking-run", QUALIFIED_MODEL)
                .with_thinking_mode(AnthropicThinkingMode::adaptive()),
        )
        .expect("qualified thinking prepares");
    assert_eq!(
        attempt.evidence().thinking_mode(),
        Some(AnthropicThinkingMode::adaptive())
    );
    assert!(attempt.evidence().reasoning_mode().is_none());
    assert!(
        !attempt
            .plan()
            .requirements()
            .capabilities()
            .any(|requirement| requirement.capability() == Capability::ReasoningSelection)
    );

    let (events, outcome) = complete_run(&fixture, &attempt);
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome.output().expect("output exists").as_str(),
        "Hello world"
    );
    assert_private_absent(&events, &outcome);
    assert_eq!(
        request_body(&fixture)["thinking"],
        serde_json::from_str::<Value>(THINKING_WIRE).unwrap()
    );
}

#[test]
fn unsupported_thinking_model_fails_before_network() {
    let fixture = PreparedFixture::new(ExecutionHostId::new("anthropic.thinking.reject").unwrap());
    let error = fixture
        .prepared()
        .prepare_inference_attempt(
            fixture
                .attempt_input("thinking-reject")
                .with_thinking_mode(AnthropicThinkingMode::adaptive()),
        )
        .expect_err("fixture model must reject thinking");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.anthropic.preparation.thinking_unsupported"
    );
    assert!(fixture.server.requests().is_empty());
    assert_eq!(fixture.releases(), 0);
}

#[test]
fn thinking_delta_and_unsigned_blocks_fail_closed_without_disclosure() {
    for (index, fixture_kind) in [
        StreamFixture::ThinkingDelta,
        StreamFixture::ThinkingUnsigned,
    ]
    .into_iter()
    .enumerate()
    {
        let fixture = PreparedFixture::with_stream(
            ExecutionHostId::new(format!("anthropic.thinking.fail.{index}"))
                .expect("execution host is valid"),
            fixture_kind,
        );
        let attempt = fixture
            .prepared()
            .prepare_inference_attempt(
                fixture
                    .attempt_input_for_model("thinking-fail", QUALIFIED_MODEL)
                    .with_thinking_mode(AnthropicThinkingMode::adaptive()),
            )
            .expect("qualified thinking prepares");
        let (events, outcome) = complete_run(&fixture, &attempt);
        let TerminalStatus::ProviderFailed(diagnostic) = outcome.status() else {
            panic!("invalid thinking stream must fail: {outcome:?}");
        };
        assert_eq!(
            diagnostic.code(),
            match fixture_kind {
                StreamFixture::ThinkingDelta => "swallowtail.anthropic.protocol_invalid",
                StreamFixture::ThinkingUnsigned => "swallowtail.anthropic.stream_order_invalid",
                _ => unreachable!(),
            }
        );
        assert_private_absent(&events, &outcome);
        assert!(!format!("{outcome:?}").contains("secret thought must not leak"));
    }
}

#[test]
fn omitted_mode_rejects_thinking_blocks() {
    let fixture = PreparedFixture::with_stream(
        ExecutionHostId::new("anthropic.thinking.unexpected").unwrap(),
        StreamFixture::ThinkingThenText,
    );
    let attempt = fixture
        .prepared()
        .prepare_inference_attempt(fixture.attempt_input("thinking-unexpected"))
        .expect("omitted thinking prepares");
    let (_, outcome) = complete_run(&fixture, &attempt);
    let TerminalStatus::ProviderFailed(diagnostic) = outcome.status() else {
        panic!("unexpected thinking must fail: {outcome:?}");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.anthropic.stream_order_invalid"
    );
}

#[test]
fn thinking_composes_with_effort_omission_and_every_admitted_value() {
    for (index, value) in EFFORT_VALUES.into_iter().enumerate() {
        let fixture = PreparedFixture::with_stream(
            ExecutionHostId::new(format!("anthropic.thinking.effort.{index}"))
                .expect("execution host is valid"),
            StreamFixture::ThinkingThenText,
        );
        let mode = ReasoningMode::new(value).unwrap();
        let attempt = fixture
            .prepared()
            .prepare_inference_attempt(
                fixture
                    .attempt_input_for_model("thinking-effort", QUALIFIED_MODEL)
                    .with_reasoning_mode(mode.clone())
                    .with_thinking_mode(AnthropicThinkingMode::adaptive()),
            )
            .expect("composed controls prepare");
        assert_eq!(attempt.evidence().reasoning_mode(), Some(&mode));
        assert_eq!(
            attempt.evidence().thinking_mode(),
            Some(AnthropicThinkingMode::adaptive())
        );
        let (_, outcome) = complete_run(&fixture, &attempt);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        let body = request_body(&fixture);
        assert_eq!(body["output_config"]["effort"], value);
        assert_eq!(
            body["thinking"],
            serde_json::from_str::<Value>(THINKING_WIRE).unwrap()
        );
    }
}

#[test]
fn thinking_composes_with_qualified_web_search_without_changing_search_wire() {
    let fixture = PreparedFixture::with_stream(
        ExecutionHostId::new("anthropic.thinking.search").unwrap(),
        StreamFixture::ThinkingThenText,
    );
    let attempt = fixture
        .prepared()
        .prepare_inference_attempt(
            fixture
                .attempt_input_for_model("thinking-search", QUALIFIED_MODEL)
                .with_web_search(AnthropicWebSearchInput::new(["example.com"]))
                .with_thinking_mode(AnthropicThinkingMode::adaptive()),
        )
        .expect("thinking plus search prepares");
    let (_, outcome) = complete_run(&fixture, &attempt);
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    let body = request_body(&fixture);
    assert_eq!(body["tools"][0]["type"], "web_search_20250305");
    assert_eq!(
        body["thinking"],
        serde_json::from_str::<Value>(THINKING_WIRE).unwrap()
    );
}

fn complete_run(
    fixture: &PreparedFixture,
    attempt: &swallowtail_adapter_anthropic::AnthropicPreparedInferenceAttempt,
) -> (
    Vec<swallowtail_runtime::RuntimeEvent>,
    swallowtail_runtime::TerminalOutcome,
) {
    let mut run = block_on(attempt.start_run(fixture.services())).expect("run starts");
    let mut events = run.take_events().expect("events exist");
    let mut collected = Vec::new();
    while let Some(event) = block_on(events.next()) {
        collected.push(event.expect("event succeeds"));
    }
    let outcome = block_on(run.take_terminal_outcome().expect("terminal exists"));
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    (collected, outcome)
}

fn request_body(fixture: &PreparedFixture) -> Value {
    serde_json::from_slice(&fixture.server.requests()[0].body).unwrap()
}

fn assert_private_absent(
    events: &[swallowtail_runtime::RuntimeEvent],
    outcome: &swallowtail_runtime::TerminalOutcome,
) {
    let dump = format!("{events:?}{outcome:?}");
    assert!(!dump.contains(SIGNATURE));
    assert!(!dump.contains(REDACTED));
    assert!(!dump.contains("secret thought must not leak"));
}

#[test]
fn structured_plan_request_effort_mismatch_still_fails_with_thinking_selected() {
    let fixture =
        PreparedFixture::new(ExecutionHostId::new("anthropic.thinking.mismatch").unwrap());
    let attempt = fixture
        .prepared()
        .prepare_inference_attempt(
            fixture
                .attempt_input_for_model("thinking-mismatch", QUALIFIED_MODEL)
                .with_reasoning_mode(ReasoningMode::new("xhigh").unwrap())
                .with_thinking_mode(AnthropicThinkingMode::adaptive()),
        )
        .expect("composed controls prepare");
    let request = StructuredRunRequest::new(
        RequestId::new("thinking-mismatch-request").unwrap(),
        OperationContent::new("prepared fixture prompt").unwrap(),
        OperationPolicy::offline().with_reasoning_mode(ReasoningMode::new("low").unwrap()),
    )
    .with_maximum_output_tokens(std::num::NonZeroU64::new(64).unwrap());
    let error = block_on(
        AnthropicDirectDriver::new()
            .with_thinking_mode(AnthropicThinkingMode::adaptive())
            .start_run(attempt.plan().clone(), request, fixture.services()),
    )
    .err()
    .expect("mismatched effort must reject");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.anthropic.generation_control_mismatch"
    );
    assert!(fixture.server.requests().is_empty());
}
