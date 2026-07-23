#[allow(dead_code)]
#[path = "driver/fixture.rs"]
mod fixture;
#[allow(dead_code)]
mod support;

use fixture::Fixture;
use futures_executor::block_on;
use futures_util::StreamExt;
use swallowtail_adapter_deepseek::{DEEPSEEK_ENDPOINT, DeepSeekDirectDriver, deepseek_v4_config};
use swallowtail_core::{DriverRole, ReasoningMode};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, DirectContinuationTurnRequest, DirectToolResult,
    DirectToolResultContent, InteractiveSessionDriver, MonotonicInstant,
    OpenDirectContinuationSessionRequest, OperationContent, RequestId, RuntimeTurnId,
    SchemaDocument, SessionOptions, TerminalStatus, ToolDeclaration,
};
use swallowtail_testkit::{
    ConformanceAssertion, ExecutionTopologyFixture, SyntheticProfile,
    run_locally_continued_direct_session_profile,
};

#[test]
fn provider_neutral_continuation_profile_covers_deepseek_boundaries() {
    let report = run_locally_continued_direct_session_profile();
    assert_eq!(
        report.profile(),
        SyntheticProfile::LocallyContinuedDirectSession
    );
    for assertion in [
        ConformanceAssertion::PreflightBeforeSideEffects,
        ConformanceAssertion::BoundSelection,
        ConformanceAssertion::OrderedEvents,
        ConformanceAssertion::SingleTerminalOutcome,
        ConformanceAssertion::CancellationAndTimeoutDistinct,
        ConformanceAssertion::CleanupRemainsVisible,
        ConformanceAssertion::Redaction,
        ConformanceAssertion::NoImplicitFallback,
        ConformanceAssertion::SessionLifecycle,
        ConformanceAssertion::HostTopologyPreserved,
        ConformanceAssertion::HostedEndpointCredentialBinding,
        ConformanceAssertion::DirectSessionNoResource,
        ConformanceAssertion::NoImplicitSessionRecovery,
        ConformanceAssertion::ExplicitAttemptAuthorization,
        ConformanceAssertion::ConsumerToolExchange,
        ConformanceAssertion::PrivateContinuationBounded,
        ConformanceAssertion::ProviderCachePosture,
        ConformanceAssertion::RequestScopedLeaseLifecycle,
    ] {
        assert!(report.covers(assertion), "missing {assertion:?}");
    }
}

#[test]
fn local_and_remote_host_authority_preserve_the_exact_three_attempt_route() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        assert_topology(&topology);
    }
}

fn assert_topology(topology: &ExecutionTopologyFixture) {
    let fixture = Fixture::for_topology(topology);
    let plan = fixture.plan(DriverRole::InteractiveSession);
    assert_eq!(plan.execution_host_id(), topology.execution_host_id());
    assert_eq!(plan.instance_id(), topology.configured_instance_id());
    assert_eq!(
        plan.instance_target_ref().as_host_value(),
        DEEPSEEK_ENDPOINT
    );
    let mut session = block_on(
        DeepSeekDirectDriver::new().open_direct_continuation_session(
            plan,
            OpenDirectContinuationSessionRequest::new(
                RequestId::new("topology-session").expect("request id"),
                deepseek_v4_config(),
            )
            .with_options(
                SessionOptions::default()
                    .with_reasoning_mode(ReasoningMode::new("high").expect("reasoning mode"))
                    .with_tools([tool()]),
            ),
            fixture.services(),
        ),
    )
    .expect("session opens");
    let mut turn = block_on(session.start_direct_continuation_turn(
        DirectContinuationTurnRequest::new(
            RuntimeTurnId::new("topology-turn").expect("turn id"),
            OperationContent::new("What is the fixture weather in London?").expect("content"),
            Deadline::at(MonotonicInstant::from_ticks(5_000)),
        ),
        fixture.services(),
    ))
    .expect("turn starts");
    let mut exchange = turn.take_direct_tool_exchange().expect("tool exchange");
    let mut calls = exchange.take_calls().expect("tool calls");
    let call = block_on(calls.next())
        .expect("one call")
        .expect("valid call");
    block_on(exchange.submitter().submit(vec![DirectToolResult::new(
        call.call_id().clone(),
        DirectToolResultContent::new(
            br#"{"temperature_c":18,"condition":"clear"}"#.to_vec(),
            65_536,
        )
        .expect("bounded result"),
    )]))
    .expect("result accepted");
    let mut events = turn.take_events().expect("events");
    let terminal = turn.take_terminal_outcome().expect("terminal");
    let outcome = block_on(async {
        while let Some(event) = events.next().await {
            event.expect("valid event");
        }
        terminal.await
    });
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(fixture.server.attempts(), 2);
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert_eq!(fixture.releases(), 1);
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
