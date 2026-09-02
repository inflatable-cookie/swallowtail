use super::fixtures::PreparedFixture;
use crate::support::StreamFixture;
use futures_executor::block_on;
use futures_util::StreamExt;
use swallowtail_adapter_anthropic::{
    AnthropicModelSelection, AnthropicSessionProfileInput, AnthropicThinkingMode,
};
use swallowtail_core::{ExecutionHostId, ModelId, ModelRouteId, ModelRouteRevision};
use swallowtail_runtime::{
    CleanupOutcome, DirectContinuationTurnRequest, OperationContent, RequestId, RuntimeTurnId,
    SchemaDocument, TerminalStatus, ToolDeclaration,
};

const QUALIFIED_MODEL: &str = "claude-opus-4-7";
const SIGNATURE: &str = "sig_omitted_fixture_private";
const REDACTED: &str = "redacted_fixture_private_data";

#[test]
fn structured_thinking_after_text_fails_closed() {
    let fixture = PreparedFixture::with_stream(
        ExecutionHostId::new("anthropic.thinking.after-text").unwrap(),
        StreamFixture::ThinkingAfterText,
    );
    let attempt = fixture
        .prepared()
        .prepare_inference_attempt(
            fixture
                .attempt_input_for_model("thinking-after-text", QUALIFIED_MODEL)
                .with_thinking_mode(AnthropicThinkingMode::adaptive()),
        )
        .expect("qualified thinking prepares");
    let mut run = block_on(attempt.start_run(fixture.services())).expect("run starts");
    let mut events = run.take_events().expect("events exist");
    let mut collected = Vec::new();
    while let Some(event) = block_on(events.next()) {
        collected.push(event.expect("event succeeds"));
    }
    let outcome = block_on(run.take_terminal_outcome().expect("terminal exists"));
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    let TerminalStatus::ProviderFailed(diagnostic) = outcome.status() else {
        panic!("late thinking must fail: {outcome:?}");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.anthropic.stream_order_invalid"
    );
    assert_private_absent(&collected, &outcome);
}

#[test]
fn session_rejects_late_duplicate_and_overflow_private_blocks() {
    for (host, stream, code) in [
        (
            "anthropic.thinking.late",
            StreamFixture::LateThinkingAfterTool,
            "swallowtail.anthropic.stream_order_invalid",
        ),
        (
            "anthropic.thinking.late-redacted",
            StreamFixture::LateRedactedAfterTool,
            "swallowtail.anthropic.stream_order_invalid",
        ),
        (
            "anthropic.thinking.duplicate",
            StreamFixture::DuplicateThinkingSignature,
            "swallowtail.anthropic.stream_order_invalid",
        ),
        (
            "anthropic.thinking.overflow",
            StreamFixture::OversizedThinkingSignature,
            "swallowtail.anthropic.private_continuation_exceeded",
        ),
    ] {
        let fixture = PreparedFixture::with_stream(ExecutionHostId::new(host).unwrap(), stream);
        let prepared_session = fixture
            .prepared()
            .prepare_session(
                AnthropicSessionProfileInput::new(
                    RequestId::new("thinking-order").unwrap(),
                    model(QUALIFIED_MODEL),
                    [fixture_tool()],
                )
                .with_thinking_mode(AnthropicThinkingMode::adaptive()),
            )
            .expect("qualified session thinking prepares");
        let mut session =
            block_on(prepared_session.open_session(fixture.services())).expect("session opens");
        let mut turn = block_on(session.start_direct_continuation_turn(
            DirectContinuationTurnRequest::new(
                RuntimeTurnId::new("thinking-order-turn").unwrap(),
                OperationContent::new("Look up the approved fixture customer.").unwrap(),
                swallowtail_runtime::Deadline::at(
                    swallowtail_runtime::MonotonicInstant::from_ticks(100_000),
                ),
            ),
            fixture.services(),
        ))
        .expect("turn starts");
        let mut events = turn.take_events().expect("events exist");
        let terminal = turn.take_terminal_outcome().expect("terminal exists");
        let mut collected = Vec::new();
        while let Some(event) = block_on(events.next()) {
            collected.push(event.expect("event succeeds"));
        }
        let outcome = block_on(terminal);
        let TerminalStatus::ProviderFailed(diagnostic) = outcome.status() else {
            panic!("{host} must fail closed: {outcome:?}");
        };
        assert_eq!(diagnostic.code(), code);
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
        assert_eq!(
            block_on(session.close(fixture.cleanup_request(), fixture.services())),
            CleanupOutcome::Clean
        );
        assert_private_absent(&collected, &outcome);
        assert_eq!(fixture.server.requests().len(), 1);
    }
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

fn model(id: &str) -> AnthropicModelSelection {
    AnthropicModelSelection::new(
        ModelRouteId::new(format!("anthropic.{id}")).unwrap(),
        ModelRouteRevision::new("1").unwrap(),
        ModelId::new(id).unwrap(),
    )
}

fn fixture_tool() -> ToolDeclaration {
    ToolDeclaration::new(
        "lookup_customer",
        SchemaDocument::inline(
            br#"{"type":"object","properties":{"customer_id":{"type":"string"}},"required":["customer_id"],"additionalProperties":false}"#.to_vec(),
            4096,
        )
        .unwrap(),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .unwrap()
}
