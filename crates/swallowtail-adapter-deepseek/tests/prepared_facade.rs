#[allow(dead_code)]
#[path = "driver/fixture.rs"]
mod fixture;
#[allow(dead_code)]
mod support;

use fixture::Fixture;
use futures_executor::block_on;
use futures_util::StreamExt;
use swallowtail_adapter_deepseek::{
    DEEPSEEK_ENDPOINT_AUDIENCE, DEEPSEEK_MODEL_ID, DeepSeekCatalogueProfileInput,
    DeepSeekModelSelection, DeepSeekSessionProfileInput, prepare_deepseek_direct,
};
use swallowtail_core::{
    DriverRole, ModelId, ModelRouteId, ModelRouteRevision, ProviderInferenceCachePolicy,
    ReasoningMode,
};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, DirectContinuationTurnRequest, DirectToolResult,
    DirectToolResultContent, MonotonicInstant, OperationContent, RequestId, RuntimeTurnId,
    SchemaDocument, TerminalStatus, ToolDeclaration,
};
use swallowtail_testkit::{
    ExecutionTopologyFixture, assert_prepared_operation_evidence_matches_plan,
};

#[test]
fn catalogue_and_consumer_authorized_continuation_run_on_both_host_topologies() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let fixture = Fixture::for_topology(&topology);
        let prepared = prepare_deepseek_direct(fixture.preparation_input(), &fixture.services())
            .expect("DeepSeek integration prepares");
        assert_eq!(
            prepared.access_profile().endpoint_audience().as_str(),
            DEEPSEEK_ENDPOINT_AUDIENCE
        );

        let catalogue = prepared
            .prepare_catalogue(DeepSeekCatalogueProfileInput::new(
                RequestId::new("prepared-catalogue").expect("request id"),
            ))
            .expect("catalogue prepares");
        assert_eq!(
            catalogue.plan().requirements().driver_role(),
            DriverRole::ModelCatalog
        );
        assert_prepared_operation_evidence_matches_plan(
            catalogue.evidence().operation(),
            catalogue.plan(),
        );
        let models =
            block_on(catalogue.list_models(fixture.services())).expect("catalogue succeeds");
        assert!(
            models
                .iter()
                .any(|model| model.id().as_str() == DEEPSEEK_MODEL_ID)
        );
        assert_eq!(fixture.server.attempts(), 0);

        let prepared_session = prepared
            .prepare_session(session_input("prepared-session", DEEPSEEK_MODEL_ID))
            .expect("session prepares");
        assert_eq!(
            prepared_session.plan().requirements().driver_role(),
            DriverRole::InteractiveSession
        );
        assert_prepared_operation_evidence_matches_plan(
            prepared_session.evidence().operation(),
            prepared_session.plan(),
        );
        let mut session = block_on(prepared_session.open_session(fixture.services()))
            .expect("prepared session opens");
        let mut turn = block_on(session.start_direct_continuation_turn(
            DirectContinuationTurnRequest::new(
                RuntimeTurnId::new("prepared-turn").expect("turn id"),
                OperationContent::new("What is the fixture weather in London?").expect("content"),
                Deadline::at(MonotonicInstant::from_ticks(5_000)),
            ),
            fixture.services(),
        ))
        .expect("first consumer-authorized attempt starts");
        let mut exchange = turn
            .take_direct_tool_exchange()
            .expect("provider tool call pauses the turn");
        let mut calls = exchange.take_calls().expect("tool calls");
        let call = block_on(calls.next())
            .expect("one tool call")
            .expect("valid tool call");
        assert_eq!(fixture.server.attempts(), 1);
        block_on(exchange.submitter().submit(vec![DirectToolResult::new(
            call.call_id().clone(),
            DirectToolResultContent::new(
                br#"{"temperature_c":18,"condition":"clear"}"#.to_vec(),
                65_536,
            )
            .expect("bounded result"),
        )]))
        .expect("correlated result explicitly authorizes attempt two");
        let mut events = turn.take_events().expect("events");
        let terminal = turn.take_terminal_outcome().expect("terminal");
        let outcome = block_on(async {
            while let Some(event) = events.next().await {
                event.expect("event succeeds");
            }
            terminal.await
        });
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(fixture.server.attempts(), 2);
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        assert_eq!(fixture.releases(), 2);
        assert_eq!(fixture.release_after_blocking(), [1, 3]);
    }
}

#[test]
fn model_substitution_and_unaccepted_cache_posture_fail_before_effects() {
    let fixture = Fixture::new();
    let prepared = prepare_deepseek_direct(fixture.preparation_input(), &fixture.services())
        .expect("integration prepares");
    let failure = prepared
        .prepare_session(session_input("wrong-model", "deepseek-v4-flash"))
        .expect_err("alternate model rejects");
    assert_eq!(
        failure.stage(),
        swallowtail_runtime::PreparationStage::Preflight
    );

    let failure = prepared
        .prepare_session(DeepSeekSessionProfileInput::new(
            RequestId::new("cache-not-accepted").expect("request id"),
            model(DEEPSEEK_MODEL_ID),
            ReasoningMode::new("high").expect("reasoning"),
            [tool()],
            ProviderInferenceCachePolicy::Prohibited,
        ))
        .expect_err("unaccepted cache posture rejects");
    assert_eq!(
        failure.stage(),
        swallowtail_runtime::PreparationStage::Preflight
    );
    assert!(fixture.server.requests().is_empty());
    assert_eq!(fixture.releases(), 0);
}

fn session_input(id: &str, model_id: &str) -> DeepSeekSessionProfileInput {
    DeepSeekSessionProfileInput::new(
        RequestId::new(id).expect("request id"),
        model(model_id),
        ReasoningMode::new("high").expect("reasoning"),
        [tool()],
        ProviderInferenceCachePolicy::AcceptedWithoutManagementAuthority,
    )
}

fn model(model_id: &str) -> DeepSeekModelSelection {
    DeepSeekModelSelection::new(
        ModelRouteId::new("deepseek.prepared.v4-pro").expect("route id"),
        ModelRouteRevision::new("2026-07-22").expect("route revision"),
        ModelId::new(model_id).expect("model id"),
    )
}

fn tool() -> ToolDeclaration {
    ToolDeclaration::new(
        "lookup_weather",
        SchemaDocument::inline(
            br#"{"type":"object","properties":{"city":{"type":"string"}},"required":["city"],"additionalProperties":false}"#.to_vec(),
            1_024,
        )
        .expect("schema"),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .expect("tool")
    .with_description(OperationContent::new("Return fixture weather").expect("description"))
}
