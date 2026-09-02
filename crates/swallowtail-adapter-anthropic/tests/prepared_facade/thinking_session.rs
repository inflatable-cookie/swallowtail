use super::fixtures::PreparedFixture;
use crate::support::StreamFixture;
use futures_executor::block_on;
use futures_util::StreamExt;
use serde_json::{Value, json};
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
const REDACTED: &str = "redacted_fixture_private_data";
const THINKING_WIRE: &str = r#"{"display":"omitted","type":"adaptive"}"#;

#[test]
fn session_thinking_replays_private_blocks_before_tool_use_and_later_turns() {
    prove_session_replay(
        "anthropic.thinking.session",
        StreamFixture::ThinkingToolContinuation,
        json!([
            {"signature": SIGNATURE, "thinking": "", "type": "thinking"},
            {
                "id": "toolu_fixture_1",
                "input": {"customer_id": "customer-fixture"},
                "name": "lookup_customer",
                "type": "tool_use"
            }
        ]),
    );
}

#[test]
fn redacted_and_consecutive_private_blocks_replay_in_order() {
    prove_session_replay(
        "anthropic.thinking.redacted",
        StreamFixture::RedactedToolContinuation,
        json!([
            {"data": REDACTED, "type": "redacted_thinking"},
            {
                "id": "toolu_fixture_1",
                "input": {"customer_id": "customer-fixture"},
                "name": "lookup_customer",
                "type": "tool_use"
            }
        ]),
    );
    prove_session_replay(
        "anthropic.thinking.consecutive",
        StreamFixture::ConsecutiveThinkingToolContinuation,
        json!([
            {"signature": SIGNATURE, "thinking": "", "type": "thinking"},
            {"data": REDACTED, "type": "redacted_thinking"},
            {
                "id": "toolu_fixture_1",
                "input": {"customer_id": "customer-fixture"},
                "name": "lookup_customer",
                "type": "tool_use"
            }
        ]),
    );
}

#[test]
fn adaptive_skip_without_thinking_blocks_stays_valid() {
    prove_session_replay(
        "anthropic.thinking.skip",
        StreamFixture::ToolContinuation,
        json!([{
            "id": "toolu_fixture_1",
            "input": {"customer_id": "customer-fixture"},
            "name": "lookup_customer",
            "type": "tool_use"
        }]),
    );
}

fn prove_session_replay(host: &str, stream: StreamFixture, expected_assistant: Value) {
    let fixture = PreparedFixture::with_stream(ExecutionHostId::new(host).unwrap(), stream);
    let prepared_session = fixture
        .prepared()
        .prepare_session(
            AnthropicSessionProfileInput::new(
                RequestId::new("thinking-session").unwrap(),
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
            RuntimeTurnId::new("thinking-turn-1").unwrap(),
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
    let mut collected = Vec::new();
    while let Some(event) = block_on(events.next()) {
        collected.push(event.expect("event succeeds"));
    }
    let outcome = block_on(terminal);
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    assert_private_absent(&collected, &outcome);

    let mut later = block_on(session.start_direct_continuation_turn(
        DirectContinuationTurnRequest::new(
            RuntimeTurnId::new("thinking-turn-2").unwrap(),
            OperationContent::new("Summarize the approved fixture result.").unwrap(),
            swallowtail_runtime::Deadline::at(swallowtail_runtime::MonotonicInstant::from_ticks(
                100_000,
            )),
        ),
        fixture.services(),
    ))
    .expect("later turn starts");
    assert!(later.take_direct_tool_exchange().is_none());
    let mut events = later.take_events().expect("events exist");
    let terminal = later.take_terminal_outcome().expect("terminal exists");
    while let Some(event) = block_on(events.next()) {
        event.expect("event succeeds");
    }
    assert_eq!(block_on(terminal).status(), &TerminalStatus::Completed);
    assert_eq!(block_on(later.close()), CleanupOutcome::Clean);
    assert_eq!(
        block_on(session.close(fixture.cleanup_request(), fixture.services())),
        CleanupOutcome::Clean
    );

    let requests = fixture.server.requests();
    assert_eq!(requests.len(), 3);
    for request in &requests {
        let body: Value = serde_json::from_slice(&request.body).unwrap();
        assert_eq!(
            body["thinking"],
            serde_json::from_str::<Value>(THINKING_WIRE).unwrap()
        );
        assert!(body.get("output_config").is_none());
    }
    let continuation: Value = serde_json::from_slice(&requests[1].body).unwrap();
    assert_eq!(continuation["messages"][1]["content"], expected_assistant);
    let later_body: Value = serde_json::from_slice(&requests[2].body).unwrap();
    assert_eq!(later_body["messages"][1]["content"], expected_assistant);
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
