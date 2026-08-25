use super::fixtures::PreparedFixture;
use crate::support::StreamFixture;
use futures_executor::block_on;
use futures_util::StreamExt;
use serde_json::Value;
use swallowtail_adapter_anthropic::{
    AnthropicDirectDriver, AnthropicModelSelection, AnthropicSessionProfileInput,
    AnthropicThinkingMode,
};
use swallowtail_core::{ExecutionHostId, ModelId, ModelRouteId, ModelRouteRevision, ReasoningMode};
use swallowtail_runtime::{
    CleanupOutcome, DirectContinuationTurnRequest, DirectToolResult, DirectToolResultContent,
    InteractiveSessionDriver, OpenDirectContinuationSessionRequest, OperationContent, RequestId,
    RuntimeTurnId, SchemaDocument, SessionOptions, TerminalStatus, ToolDeclaration,
    WorkingStateRestorationOutcome,
};

const QUALIFIED_MODEL: &str = "claude-opus-4-7";
const THINKING_WIRE: &str = r#"{"display":"omitted","type":"adaptive"}"#;

#[test]
fn restoration_repeats_thinking_selection_without_private_recovery() {
    let fixture = PreparedFixture::with_stream(
        ExecutionHostId::new("anthropic.thinking.restore").unwrap(),
        StreamFixture::ThinkingToolContinuation,
    );
    let prepared_session = fixture
        .prepared()
        .prepare_session(
            AnthropicSessionProfileInput::new(
                RequestId::new("thinking-restore").unwrap(),
                model(QUALIFIED_MODEL),
                [fixture_tool()],
            )
            .with_thinking_mode(AnthropicThinkingMode::adaptive()),
        )
        .expect("qualified session thinking prepares");
    assert_eq!(
        prepared_session.evidence().thinking_mode(),
        Some(AnthropicThinkingMode::adaptive())
    );
    let restoration = prepared_session
        .prepare_working_state_restoration(RuntimeTurnId::new("thinking-interrupted").unwrap());
    let restored = block_on(restoration.restore(fixture.services())).expect("restores");
    let WorkingStateRestorationOutcome::SessionReplaced(replacement) = restored else {
        panic!("fresh session replacement expected");
    };
    let (_, mut restored_session) = replacement.into_parts();
    let mut turn = block_on(restored_session.start_direct_continuation_turn(
        DirectContinuationTurnRequest::new(
            RuntimeTurnId::new("thinking-restored-turn").unwrap(),
            OperationContent::new("Look up the approved fixture customer.").unwrap(),
            swallowtail_runtime::Deadline::at(swallowtail_runtime::MonotonicInstant::from_ticks(
                100_000,
            )),
        ),
        fixture.services(),
    ))
    .expect("restored first turn starts");
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
    assert_eq!(block_on(restored_session.close()), CleanupOutcome::Clean);

    let requests = fixture.server.requests();
    assert_eq!(requests.len(), 2);
    let first: Value = serde_json::from_slice(&requests[0].body).unwrap();
    assert_eq!(
        first["thinking"],
        serde_json::from_str::<Value>(THINKING_WIRE).unwrap()
    );
    assert_eq!(first["messages"].as_array().unwrap().len(), 1);
}

#[test]
fn session_plan_request_without_thinking_still_rejects_effort_mismatch() {
    let fixture = PreparedFixture::with_stream(
        ExecutionHostId::new("anthropic.thinking.session-mismatch").unwrap(),
        StreamFixture::ThinkingToolContinuation,
    );
    let prepared_session = fixture
        .prepared()
        .prepare_session(
            AnthropicSessionProfileInput::new(
                RequestId::new("thinking-session-mismatch").unwrap(),
                model(QUALIFIED_MODEL),
                [fixture_tool()],
            )
            .with_thinking_mode(AnthropicThinkingMode::adaptive())
            .with_reasoning_mode(ReasoningMode::new("xhigh").unwrap()),
        )
        .expect("qualified composed session prepares");
    let request = OpenDirectContinuationSessionRequest::new(
        RequestId::new("thinking-session-mismatch-request").unwrap(),
        prepared_session.request().config().clone(),
    )
    .with_options(
        SessionOptions::default()
            .with_tools([fixture_tool()])
            .with_reasoning_mode(ReasoningMode::new("low").unwrap()),
    );
    let error = block_on(
        AnthropicDirectDriver::new()
            .with_thinking_mode(AnthropicThinkingMode::adaptive())
            .open_direct_continuation_session(
                prepared_session.plan().clone(),
                request,
                fixture.services(),
            ),
    )
    .err()
    .expect("mismatched session effort must reject");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.anthropic.generation_control_mismatch"
    );
    assert!(fixture.server.requests().is_empty());
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
