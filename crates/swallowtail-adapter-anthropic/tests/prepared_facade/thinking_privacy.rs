use super::fixtures::PreparedFixture;
use crate::support::StreamFixture;
use futures_executor::block_on;
use futures_util::StreamExt;
use swallowtail_adapter_anthropic::{
    AnthropicModelSelection, AnthropicSessionProfileInput, AnthropicThinkingMode,
};
use swallowtail_core::{ExecutionHostId, ModelId, ModelRouteId, ModelRouteRevision};
use swallowtail_runtime::{
    CleanupOutcome, DirectContinuationTurnRequest, DirectToolResult, DirectToolResultContent,
    OperationContent, RequestId, RuntimeTurnId, SchemaDocument, TerminalStatus, ToolDeclaration,
};

const QUALIFIED_MODEL: &str = "claude-opus-4-7";
const SIGNATURE: &str = "sig_omitted_fixture_private";

#[test]
fn non_utf8_tool_result_fails_closed_without_leaking_or_posting() {
    let fixture = PreparedFixture::with_stream(
        ExecutionHostId::new("anthropic.thinking.non-utf8").unwrap(),
        StreamFixture::ThinkingToolContinuation,
    );
    let prepared_session = fixture
        .prepared()
        .prepare_session(
            AnthropicSessionProfileInput::new(
                RequestId::new("thinking-non-utf8").unwrap(),
                AnthropicModelSelection::new(
                    ModelRouteId::new(format!("anthropic.{QUALIFIED_MODEL}")).unwrap(),
                    ModelRouteRevision::new("1").unwrap(),
                    ModelId::new(QUALIFIED_MODEL).unwrap(),
                ),
                [ToolDeclaration::new(
                    "lookup_customer",
                    SchemaDocument::inline(
                        br#"{"type":"object","properties":{"customer_id":{"type":"string"}},"required":["customer_id"],"additionalProperties":false}"#.to_vec(),
                        4096,
                    )
                    .unwrap(),
                    "application/schema+json",
                    "json-schema-2020-12",
                )
                .unwrap()],
            )
            .with_thinking_mode(AnthropicThinkingMode::adaptive()),
        )
        .expect("qualified session thinking prepares");
    let mut session =
        block_on(prepared_session.open_session(fixture.services())).expect("session opens");
    let mut turn = block_on(session.start_direct_continuation_turn(
        DirectContinuationTurnRequest::new(
            RuntimeTurnId::new("thinking-non-utf8-turn").unwrap(),
            OperationContent::new("Look up the approved fixture customer.").unwrap(),
            swallowtail_runtime::Deadline::at(swallowtail_runtime::MonotonicInstant::from_ticks(
                100_000,
            )),
        ),
        fixture.services(),
    ))
    .expect("first turn starts");
    let mut exchange = turn
        .take_direct_tool_exchange()
        .expect("tool exchange exists");
    let mut calls = exchange.take_calls().expect("tool calls exist");
    let mut events = turn.take_events().expect("events exist");
    let terminal = turn.take_terminal_outcome().expect("terminal exists");
    let submitter = exchange.submitter();
    let call = block_on(calls.next())
        .expect("tool call arrives")
        .expect("tool call succeeds");
    block_on(submitter.submit(vec![DirectToolResult::new(
        call.call_id().clone(),
        DirectToolResultContent::new(vec![0xff, 0xfe], 65_536).unwrap(),
    )]))
    .expect("opaque non-utf8 result is legal to submit");
    let mut collected = Vec::new();
    while let Some(event) = block_on(events.next()) {
        collected.push(event.expect("event stream stays readable"));
    }
    let outcome = block_on(terminal);
    let TerminalStatus::RuntimeFailed(diagnostic) = outcome.status() else {
        panic!("non-utf8 tool result must fail closed: {outcome:?}");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.anthropic.history_state_invalid"
    );
    assert!(!format!("{collected:?}{outcome:?}").contains(SIGNATURE));
    assert_eq!(fixture.server.requests().len(), 1);
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn continuation_upload_reads_redacted_body_onto_the_wire() {
    let fixture = PreparedFixture::with_stream(
        ExecutionHostId::new("anthropic.thinking.upload").unwrap(),
        StreamFixture::ThinkingToolContinuation,
    );
    let prepared_session = fixture
        .prepared()
        .prepare_session(
            AnthropicSessionProfileInput::new(
                RequestId::new("thinking-upload").unwrap(),
                AnthropicModelSelection::new(
                    ModelRouteId::new(format!("anthropic.{QUALIFIED_MODEL}")).unwrap(),
                    ModelRouteRevision::new("1").unwrap(),
                    ModelId::new(QUALIFIED_MODEL).unwrap(),
                ),
                [ToolDeclaration::new(
                    "lookup_customer",
                    SchemaDocument::inline(
                        br#"{"type":"object","properties":{"customer_id":{"type":"string"}},"required":["customer_id"],"additionalProperties":false}"#.to_vec(),
                        4096,
                    )
                    .unwrap(),
                    "application/schema+json",
                    "json-schema-2020-12",
                )
                .unwrap()],
            )
            .with_thinking_mode(AnthropicThinkingMode::adaptive()),
        )
        .expect("qualified session thinking prepares");
    let mut session =
        block_on(prepared_session.open_session(fixture.services())).expect("session opens");
    let mut turn = block_on(session.start_direct_continuation_turn(
        DirectContinuationTurnRequest::new(
            RuntimeTurnId::new("thinking-upload-turn").unwrap(),
            OperationContent::new("Look up the approved fixture customer.").unwrap(),
            swallowtail_runtime::Deadline::at(swallowtail_runtime::MonotonicInstant::from_ticks(
                100_000,
            )),
        ),
        fixture.services(),
    ))
    .expect("first turn starts");
    let mut exchange = turn
        .take_direct_tool_exchange()
        .expect("tool exchange exists");
    let mut calls = exchange.take_calls().expect("tool calls exist");
    let mut events = turn.take_events().expect("events exist");
    let terminal = turn.take_terminal_outcome().expect("terminal exists");
    let submitter = exchange.submitter();
    let call = block_on(calls.next())
        .expect("tool call arrives")
        .expect("tool call succeeds");
    block_on(submitter.submit(vec![DirectToolResult::new(
        call.call_id().clone(),
        DirectToolResultContent::new(b"approved fixture result".to_vec(), 65_536).unwrap(),
    )]))
    .expect("tool result continues");
    while let Some(event) = block_on(events.next()) {
        event.expect("event succeeds");
    }
    assert_eq!(block_on(terminal).status(), &TerminalStatus::Completed);
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    let continuation = &fixture.server.requests()[1].body;
    assert!(
        continuation
            .windows(SIGNATURE.len())
            .any(|window| window == SIGNATURE.as_bytes()),
        "read-callback upload must still deliver the redacted request body"
    );
}
