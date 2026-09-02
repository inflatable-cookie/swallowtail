#[path = "driver/failures.rs"]
mod failures;
#[path = "driver/fixture.rs"]
mod fixture;
mod support;

use fixture::Fixture;
use futures_executor::block_on;
use futures_util::StreamExt;
use std::sync::Arc;
use swallowtail_adapter_deepseek::{
    DEEPSEEK_MODEL_ID, DeepSeekDirectDriver, deepseek_direct_descriptor, deepseek_v4_config,
};
use swallowtail_core::{DriverRole, ReasoningMode};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, DirectContinuationTurnRequest, DirectToolResult,
    DirectToolResultContent, DriverRegistration, InteractiveSessionDriver, ModelCatalogDriver,
    ModelCatalogRequest, MonotonicInstant, OpenDirectContinuationSessionRequest, OperationContent,
    OperationPolicy, ProviderObservation, RequestId, RunHandle, RuntimeEvent, RuntimeEventKind,
    RuntimeTurnId, SchemaDocument, SessionOptions, StructuredRunDriver, StructuredRunRequest,
    TerminalOutcome, TerminalStatus, ToolDeclaration, TurnHandle,
};

const FIRST_PROMPT: &str = "What is the fixture weather in London?";
const SECOND_PROMPT: &str = "Summarise that in three words.";
const TOOL_RESULT: &[u8] = br#"{"temperature_c":18,"condition":"clear"}"#;

#[test]
fn descriptor_registers_the_exact_catalogue_session_and_structured_roles() {
    let descriptor = deepseek_direct_descriptor();
    assert_eq!(
        descriptor.identity().id().as_str(),
        "swallowtail.deepseek.direct"
    );
    assert_eq!(descriptor.integration_family().as_str(), "deepseek");
    assert_eq!(
        descriptor.transport_family().as_str(),
        "openai-chat-http-sse"
    );
    let driver = Arc::new(DeepSeekDirectDriver::new());
    let registration = DriverRegistration::new(descriptor)
        .with_model_catalog(Arc::clone(&driver) as Arc<dyn ModelCatalogDriver>)
        .expect("catalogue role registers")
        .with_interactive_session(Arc::clone(&driver) as Arc<dyn InteractiveSessionDriver>)
        .expect("session role registers")
        .with_structured_run(driver as Arc<dyn StructuredRunDriver>)
        .expect("session role registers");
    assert!(registration.model_catalog().is_some());
    assert!(registration.interactive_session().is_some());
    assert!(registration.structured_run().is_some());
}

#[test]
fn one_request_structured_run_has_no_tools_or_private_continuation() {
    let fixture = Fixture::with_scenario(support::ServerScenario::StructuredSuccess);
    let request = StructuredRunRequest::new(
        RequestId::new("structured-run").expect("request id"),
        OperationContent::new("Answer once").expect("content"),
        OperationPolicy::offline()
            .with_reasoning_mode(ReasoningMode::new("high").expect("reasoning")),
    )
    .with_maximum_output_tokens(std::num::NonZeroU64::new(512).expect("maximum"));
    let mut run = block_on(DeepSeekDirectDriver::new().start_run(
        fixture.plan(DriverRole::StructuredRun),
        request,
        fixture.services(),
    ))
    .expect("run starts");
    assert!(run.provider_run_ref().is_none());
    let (events, outcome) = complete_run(&mut run);
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome.output().expect("output").as_str(),
        "London is 18 C and clear."
    );
    assert!(events.iter().any(|event| matches!(
        event.kind(),
        RuntimeEventKind::ProviderObservation(ProviderObservation::RequestCorrelation(_))
    )));
    assert!(events.iter().any(|event| matches!(
        event.kind(),
        RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(_))
    )));
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert_eq!(fixture.server.attempts(), 1);
    assert_eq!(fixture.releases(), 1);
    assert_eq!(fixture.release_after_blocking(), [1]);

    let requests = fixture.server.requests();
    assert_eq!(requests.len(), 1);
    let body: serde_json::Value = serde_json::from_slice(&requests[0].body).expect("request JSON");
    assert_eq!(body["model"], DEEPSEEK_MODEL_ID);
    assert_eq!(body["stream"], true);
    assert_eq!(body["stream_options"]["include_usage"], true);
    assert_eq!(body["max_tokens"], 512);
    assert_eq!(body["messages"].as_array().expect("messages").len(), 1);
    assert!(
        body.get("tools")
            .is_none_or(|tools| tools == &serde_json::json!([]))
    );
}

#[test]
fn structured_run_dispatches_each_exact_reasoning_effort() {
    for mode in ["low", "high", "max"] {
        let fixture = Fixture::with_scenario(support::ServerScenario::StructuredSuccess);
        let request = StructuredRunRequest::new(
            RequestId::new(format!("structured-{mode}")).expect("request id"),
            OperationContent::new("Answer once").expect("content"),
            OperationPolicy::offline()
                .with_reasoning_mode(ReasoningMode::new(mode).expect("reasoning")),
        )
        .with_maximum_output_tokens(std::num::NonZeroU64::new(512).expect("maximum"));
        let mut run = block_on(DeepSeekDirectDriver::new().start_run(
            fixture.plan_with_reasoning(DriverRole::StructuredRun, mode),
            request,
            fixture.services(),
        ))
        .expect("run starts");
        let (_, outcome) = complete_run(&mut run);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);

        let body: serde_json::Value =
            serde_json::from_slice(&fixture.server.requests()[0].body).expect("request JSON");
        assert_eq!(body["reasoning_effort"], mode);
        assert_eq!(body["thinking"]["type"], "enabled");
    }
}

#[test]
fn provider_effort_aliases_fail_before_network_work() {
    for mode in ["medium", "xhigh", "provider-high"] {
        let fixture = Fixture::with_scenario(support::ServerScenario::StructuredSuccess);
        let request = StructuredRunRequest::new(
            RequestId::new(format!("reject-{mode}")).expect("request id"),
            OperationContent::new("must reject").expect("content"),
            OperationPolicy::offline()
                .with_reasoning_mode(ReasoningMode::new(mode).expect("reasoning")),
        )
        .with_maximum_output_tokens(std::num::NonZeroU64::new(512).expect("maximum"));
        let error = block_on(DeepSeekDirectDriver::new().start_run(
            fixture.plan(DriverRole::StructuredRun),
            request,
            fixture.services(),
        ))
        .err()
        .expect("unsupported effort rejects");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.deepseek.unsupported"
        );
        assert!(fixture.server.requests().is_empty());
        assert_eq!(fixture.releases(), 0);
    }
}

#[test]
fn continuation_effort_aliases_fail_before_network_work() {
    for mode in ["medium", "xhigh", "provider-high"] {
        let fixture = Fixture::new();
        let error = block_on(
            DeepSeekDirectDriver::new().open_direct_continuation_session(
                fixture.plan(DriverRole::InteractiveSession),
                OpenDirectContinuationSessionRequest::new(
                    RequestId::new(format!("reject-session-{mode}")).expect("request id"),
                    deepseek_v4_config(),
                )
                .with_options(
                    SessionOptions::default()
                        .with_reasoning_mode(ReasoningMode::new(mode).expect("reasoning mode"))
                        .with_tools([fixture_tool()]),
                ),
                fixture.services(),
            ),
        )
        .err()
        .expect("unsupported effort rejects");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.deepseek.request_plan_mismatch"
        );
        assert!(fixture.server.requests().is_empty());
        assert_eq!(fixture.releases(), 0);
    }
}

#[test]
fn structured_run_rejects_tools_before_network_access() {
    let fixture = Fixture::with_scenario(support::ServerScenario::StructuredSuccess);
    let request = StructuredRunRequest::new(
        RequestId::new("tool-run").expect("request id"),
        OperationContent::new("must reject").expect("content"),
        OperationPolicy::offline()
            .with_reasoning_mode(ReasoningMode::new("high").expect("reasoning")),
    )
    .with_maximum_output_tokens(std::num::NonZeroU64::new(512).expect("maximum"))
    .with_tools([fixture_tool()]);
    let error = block_on(DeepSeekDirectDriver::new().start_run(
        fixture.plan(DriverRole::StructuredRun),
        request,
        fixture.services(),
    ))
    .err()
    .expect("tools reject");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.deepseek.unsupported"
    );
    assert!(fixture.server.requests().is_empty());
    assert_eq!(fixture.releases(), 0);
}

#[test]
fn structured_run_cancellation_is_terminal_and_joins_before_release() {
    let fixture = Fixture::with_scenario(support::ServerScenario::StructuredWait);
    let request = StructuredRunRequest::new(
        RequestId::new("cancel-run").expect("request id"),
        OperationContent::new("wait until cancelled").expect("content"),
        OperationPolicy::offline()
            .with_reasoning_mode(ReasoningMode::new("high").expect("reasoning")),
    )
    .with_maximum_output_tokens(std::num::NonZeroU64::new(512).expect("maximum"));
    let mut run = block_on(DeepSeekDirectDriver::new().start_run(
        fixture.plan(DriverRole::StructuredRun),
        request,
        fixture.services(),
    ))
    .expect("run starts");
    block_on(run.cancellation().request()).expect("cancellation requests");
    let (_, outcome) = complete_run(&mut run);
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    assert_eq!(fixture.releases(), 1);
    assert_eq!(fixture.release_after_blocking(), [1]);
}

#[test]
fn exact_catalogue_tool_exchange_and_private_replay_complete_three_attempts() {
    let fixture = Fixture::new();
    let driver = DeepSeekDirectDriver::new();
    let models = block_on(driver.list_models(
        fixture.plan(DriverRole::ModelCatalog),
        ModelCatalogRequest::new(RequestId::new("models").expect("request id")),
        fixture.services(),
    ))
    .expect("catalogue succeeds");
    assert_eq!(models.len(), 2);
    assert!(
        models
            .iter()
            .any(|model| model.id().as_str() == "deepseek-v4-pro")
    );
    assert!(
        models
            .iter()
            .all(|model| { model.provider_id().expect("provider").as_str() == "deepseek" })
    );

    let mut session = open(&fixture, "successful-session");
    assert!(session.provider_session_ref().is_none());
    assert!(session.resume_binding().is_none());
    let mut first = block_on(session.start_direct_continuation_turn(
        turn_request("turn-one", FIRST_PROMPT, 5_000),
        fixture.services(),
    ))
    .expect("first turn starts");
    let mut exchange = first
        .take_direct_tool_exchange()
        .expect("first turn exposes direct tool exchange");
    let mut calls = exchange.take_calls().expect("tool-call stream");
    let call = block_on(calls.next())
        .expect("one tool call")
        .expect("valid tool call");
    assert_eq!(call.tool_name(), "lookup_weather");
    assert_eq!(call.arguments().as_bytes(), br#"{"city":"London"}"#);
    assert!(block_on(calls.next()).is_none());
    assert_eq!(fixture.server.attempts(), 1);
    block_on(exchange.submitter().submit(vec![DirectToolResult::new(
        call.call_id().clone(),
        DirectToolResultContent::new(TOOL_RESULT.to_vec(), 65_536).expect("bounded result"),
    )]))
    .expect("correlated result authorizes continuation");
    let (first_events, first_outcome) = complete(&mut first);
    assert_eq!(first_outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        first_outcome.output().expect("first output").as_str(),
        "London is 18 C and clear."
    );
    assert_attempt_evidence(&first_events, 2);
    assert!(!format!("{first_events:?}").contains("fixture-private-reasoning"));
    assert_eq!(block_on(first.close()), CleanupOutcome::Clean);

    let mut second = block_on(session.start_direct_continuation_turn(
        turn_request("turn-two", SECOND_PROMPT, 5_000),
        fixture.services(),
    ))
    .expect("second turn starts from private replay");
    assert!(second.take_direct_tool_exchange().is_none());
    let (second_events, second_outcome) = complete(&mut second);
    assert_eq!(second_outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        second_outcome.output().expect("second output").as_str(),
        "Clear, mild, London."
    );
    assert_attempt_evidence(&second_events, 1);
    assert_eq!(block_on(second.close()), CleanupOutcome::Clean);
    let error = block_on(session.start_direct_continuation_turn(
        turn_request("turn-three", "Must reject before effects", 5_000),
        fixture.services(),
    ))
    .err()
    .expect("third user turn exceeds the immutable bound");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.direct_continuation.limit_reached"
    );
    assert_eq!(
        block_on(fixture.close_session(session)),
        CleanupOutcome::Clean
    );

    assert_eq!(fixture.server.attempts(), 3);
    assert_eq!(fixture.releases(), 2);
    assert_eq!(fixture.release_after_blocking(), [1, 4]);
    let requests = fixture.server.requests();
    assert_eq!(requests.len(), 4);
    assert_eq!(requests[0].target, "/models");
    for (request, expected) in requests[1..].iter().zip([
        fixture_json("attempt-1-request.json"),
        fixture_json("attempt-2-request.json"),
        fixture_json("attempt-3-request.json"),
    ]) {
        assert_eq!(request.target, "/chat/completions");
        assert_eq!(request.method, "POST");
        assert_eq!(
            request.headers.get("authorization").map(String::as_str),
            Some("Bearer fixture-secret")
        );
        let actual: serde_json::Value =
            serde_json::from_slice(&request.body).expect("request JSON");
        assert_eq!(actual, expected);
    }
}

#[test]
fn each_reasoning_effort_stays_fixed_across_continuation_attempts() {
    for mode in ["low", "high", "max"] {
        let fixture = Fixture::new();
        let mut session = open_with_reasoning(&fixture, &format!("{mode}-session"), mode);

        let mut first = block_on(session.start_direct_continuation_turn(
            turn_request(&format!("{mode}-turn-one"), FIRST_PROMPT, 5_000),
            fixture.services(),
        ))
        .expect("first turn starts");
        submit_fixture_result(&mut first);
        let (_, first_outcome) = complete(&mut first);
        assert_eq!(first_outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(first.close()), CleanupOutcome::Clean);

        let mut second = block_on(session.start_direct_continuation_turn(
            turn_request(&format!("{mode}-turn-two"), SECOND_PROMPT, 5_000),
            fixture.services(),
        ))
        .expect("second turn starts from private replay");
        let (_, second_outcome) = complete(&mut second);
        assert_eq!(second_outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(second.close()), CleanupOutcome::Clean);
        assert_eq!(
            block_on(fixture.close_session(session)),
            CleanupOutcome::Clean
        );

        let requests = fixture
            .server
            .requests()
            .into_iter()
            .filter(|request| request.target == "/chat/completions")
            .collect::<Vec<_>>();
        assert_eq!(requests.len(), 3);
        for request in requests {
            let body: serde_json::Value =
                serde_json::from_slice(&request.body).expect("request JSON");
            assert_eq!(body["reasoning_effort"], mode);
            assert_eq!(body["thinking"]["type"], "enabled");
        }
    }
}

fn open(
    fixture: &Fixture,
    request_id: &str,
) -> Box<dyn swallowtail_runtime::InteractiveSessionHandle> {
    open_with_reasoning(fixture, request_id, "high")
}

fn open_with_reasoning(
    fixture: &Fixture,
    request_id: &str,
    reasoning: &str,
) -> Box<dyn swallowtail_runtime::InteractiveSessionHandle> {
    block_on(
        DeepSeekDirectDriver::new().open_direct_continuation_session(
            fixture.plan_with_reasoning(DriverRole::InteractiveSession, reasoning),
            OpenDirectContinuationSessionRequest::new(
                RequestId::new(request_id).expect("request id"),
                deepseek_v4_config(),
            )
            .with_options(
                SessionOptions::default()
                    .with_reasoning_mode(ReasoningMode::new(reasoning).expect("reasoning mode"))
                    .with_tools([fixture_tool()]),
            ),
            fixture.services(),
        ),
    )
    .expect("session opens")
}

fn fixture_tool() -> ToolDeclaration {
    ToolDeclaration::new(
        "lookup_weather",
        SchemaDocument::inline(
            br#"{"type":"object","properties":{"city":{"type":"string"}},"required":["city"],"additionalProperties":false}"#.to_vec(),
            1_024,
        )
        .expect("bounded schema"),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .expect("tool declaration")
    .with_description(OperationContent::new("Return fixture weather").expect("description"))
}

fn turn_request(id: &str, prompt: &str, deadline_ticks: u64) -> DirectContinuationTurnRequest {
    DirectContinuationTurnRequest::new(
        RuntimeTurnId::new(id).expect("turn id"),
        OperationContent::new(prompt).expect("content"),
        Deadline::at(MonotonicInstant::from_ticks(deadline_ticks)),
    )
}

fn submit_fixture_result(turn: &mut Box<dyn TurnHandle>) {
    let mut exchange = turn
        .take_direct_tool_exchange()
        .expect("tool exchange exists");
    let mut calls = exchange.take_calls().expect("tool calls");
    let call = block_on(calls.next())
        .expect("tool call exists")
        .expect("tool call is valid");
    block_on(exchange.submitter().submit(vec![DirectToolResult::new(
        call.call_id().clone(),
        DirectToolResultContent::new(TOOL_RESULT.to_vec(), 65_536).expect("bounded result"),
    )]))
    .expect("tool result accepted");
}

fn complete(turn: &mut Box<dyn TurnHandle>) -> (Vec<RuntimeEvent>, TerminalOutcome) {
    let mut events = turn.take_events().expect("event stream");
    let terminal = turn.take_terminal_outcome().expect("terminal outcome");
    block_on(async {
        let mut collected = Vec::new();
        while let Some(event) = events.next().await {
            collected.push(event.expect("valid event"));
        }
        (collected, terminal.await)
    })
}

fn complete_run(run: &mut Box<dyn RunHandle>) -> (Vec<RuntimeEvent>, TerminalOutcome) {
    let mut events = run.take_events().expect("event stream");
    let terminal = run.take_terminal_outcome().expect("terminal outcome");
    block_on(async {
        let mut collected = Vec::new();
        while let Some(event) = events.next().await {
            collected.push(event.expect("valid event"));
        }
        (collected, terminal.await)
    })
}

fn assert_attempt_evidence(events: &[RuntimeEvent], count: usize) {
    let usage: Vec<_> = events
        .iter()
        .filter_map(|event| match event.kind() {
            RuntimeEventKind::ProviderObservation(ProviderObservation::DirectAttemptUsage(
                observation,
            )) => Some(observation),
            _ => None,
        })
        .collect();
    assert_eq!(usage.len(), count);
    assert!(usage.iter().all(|observation| {
        observation.usage().cache_read_input_tokens().is_some()
            && observation.usage().cache_miss_input_tokens().is_some()
    }));
    assert!(events.iter().any(|event| matches!(
        event.kind(),
        RuntimeEventKind::ProviderObservation(ProviderObservation::RequestCorrelation(_))
    )));
    assert!(events.iter().any(|event| matches!(
        event.kind(),
        RuntimeEventKind::ProviderObservation(ProviderObservation::DirectAttemptFinish(_))
    )));
}

fn fixture_json(name: &str) -> serde_json::Value {
    let value = match name {
        "attempt-1-request.json" => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/deepseek-openai-chat-2026-07-22/attempt-1-request.json"
        )),
        "attempt-2-request.json" => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/deepseek-openai-chat-2026-07-22/attempt-2-request.json"
        )),
        "attempt-3-request.json" => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/deepseek-openai-chat-2026-07-22/attempt-3-request.json"
        )),
        _ => unreachable!("known fixture"),
    };
    serde_json::from_str(value).expect("fixture JSON")
}
