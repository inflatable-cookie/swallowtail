#[allow(dead_code)]
#[path = "driver/fixture.rs"]
mod fixture;
#[allow(dead_code)]
mod support;

use fixture::Fixture;
use futures_executor::block_on;
use futures_util::StreamExt;
use swallowtail_adapter_deepseek::{
    DEEPSEEK_MODEL_ID, DeepSeekModelSelection, DeepSeekSessionProfileInput, prepare_deepseek_direct,
};
use swallowtail_core::{
    ModelId, ModelRouteId, ModelRouteRevision, ProviderInferenceCachePolicy, ReasoningMode,
};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, DirectContinuationTurnRequest, MonotonicInstant, OperationContent,
    RequestId, RuntimeTurnId, SchemaDocument, ToolDeclaration, WorkingStateRestorationMethod,
    WorkingStateRestorationOutcome,
};

#[test]
fn direct_session_restoration_preserves_each_exact_reasoning_selection() {
    for mode in ["low", "high", "max"] {
        let fixture = Fixture::new();
        let prepared = prepare_deepseek_direct(fixture.preparation_input(), &fixture.services())
            .expect("integration prepares");
        let session = prepared
            .prepare_session(session_input(&format!("restoration-{mode}"), mode))
            .expect("session prepares");
        assert_eq!(
            session
                .evidence()
                .reasoning_mode()
                .map(|selected| selected.as_str()),
            Some(mode)
        );
        let interrupted =
            RuntimeTurnId::new(format!("deepseek-interrupted-{mode}")).expect("turn id");
        let restoration = session.prepare_working_state_restoration(interrupted.clone());
        assert_eq!(
            restoration.method(),
            WorkingStateRestorationMethod::FreshSessionReplacement
        );
        let restored =
            block_on(restoration.restore(fixture.services())).expect("replacement opens");
        let WorkingStateRestorationOutcome::SessionReplaced(replacement) = restored else {
            panic!("fresh session replacement expected");
        };
        assert_eq!(replacement.interrupted_turn_id(), &interrupted);
        let (_, mut replacement) = replacement.into_parts();
        assert!(replacement.provider_session_ref().is_none());
        let mut turn = block_on(replacement.start_direct_continuation_turn(
            DirectContinuationTurnRequest::new(
                RuntimeTurnId::new(format!("restored-turn-{mode}")).expect("turn id"),
                OperationContent::new("restored prompt").expect("content"),
                Deadline::at(MonotonicInstant::from_ticks(5_000)),
            ),
            fixture.services(),
        ))
        .expect("restored turn starts");
        let mut exchange = turn
            .take_direct_tool_exchange()
            .expect("restored tool exchange");
        let mut calls = exchange.take_calls().expect("restored tool calls");
        let call = block_on(calls.next())
            .expect("one restored tool call")
            .expect("valid restored tool call");
        assert_eq!(call.tool_name(), "lookup_weather");
        drop(calls);
        drop(exchange);
        let requests = fixture.server.requests();
        let request = requests
            .into_iter()
            .find(|request| request.target == "/chat/completions")
            .expect("restored request");
        let body: serde_json::Value = serde_json::from_slice(&request.body).expect("request JSON");
        assert_eq!(body["reasoning_effort"], mode);
        assert_eq!(body["thinking"]["type"], "enabled");
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
        assert_eq!(block_on(replacement.close()), CleanupOutcome::Clean);
        assert_eq!(fixture.server.attempts(), 1);
    }
}

fn session_input(id: &str, reasoning: &str) -> DeepSeekSessionProfileInput {
    DeepSeekSessionProfileInput::new(
        RequestId::new(id).expect("request id"),
        DeepSeekModelSelection::new(
            ModelRouteId::new("deepseek.prepared.v4-pro").expect("route id"),
            ModelRouteRevision::new("2026-07-22").expect("route revision"),
            ModelId::new(DEEPSEEK_MODEL_ID).expect("model id"),
        ),
        ReasoningMode::new(reasoning).expect("reasoning"),
        [ToolDeclaration::new(
            "lookup_weather",
            SchemaDocument::inline(
                br#"{"type":"object","properties":{"city":{"type":"string"}},"required":["city"],"additionalProperties":false}"#.to_vec(),
                1_024,
            )
            .expect("schema"),
            "application/schema+json",
            "json-schema-2020-12",
        )
        .expect("tool")],
        ProviderInferenceCachePolicy::AcceptedWithoutManagementAuthority,
    )
}
