#[path = "direct_driver/fixture.rs"]
mod fixture;
mod support;

use fixture::Fixture;
use futures_executor::block_on;
use futures_util::StreamExt;
use std::num::NonZeroU64;
use support::StreamFixture;
use swallowtail_adapter_kimi_platform::{
    KimiPlatformDirectDriver, kimi_platform_direct_descriptor,
};
use swallowtail_core::{DriverRole, EndpointAudience, ReasoningMode};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, ModelCatalogDriver, ModelCatalogRequest, MonotonicInstant,
    OperationContent, OperationPolicy, ProviderObservation, RequestId, RuntimeEventKind,
    StructuredRunDriver, StructuredRunRequest, TerminalStatus,
};

#[test]
fn descriptor_is_a_distinct_direct_kimi_platform_driver() {
    let descriptor = kimi_platform_direct_descriptor();
    assert_eq!(
        descriptor.identity().id().as_str(),
        "swallowtail.kimi-platform.direct-chat"
    );
    assert_eq!(descriptor.integration_family().as_str(), "kimi-platform");
    assert_eq!(descriptor.transport_family().as_str(), "http-sse");
    assert!(descriptor.supports_role(DriverRole::ModelCatalog));
    assert!(descriptor.supports_role(DriverRole::StructuredRun));
}

#[test]
fn catalogue_and_one_k3_attempt_use_host_approved_access_and_cleanup() {
    let fixture = Fixture::new();
    let driver = KimiPlatformDirectDriver::new();
    let models = block_on(driver.list_models(
        fixture.plan(DriverRole::ModelCatalog),
        ModelCatalogRequest::new(RequestId::new("catalogue").expect("request id")),
        fixture.services(),
    ))
    .expect("catalogue succeeds");
    assert_eq!(models.len(), 1);
    assert_eq!(models[0].id().as_str(), "kimi-k3");

    let (run, events, outcome) = complete_run(&fixture);
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(outcome.output().expect("output").as_str(), "Fixture answer");
    assert!(
        events
            .iter()
            .any(|event| event.kind() == &RuntimeEventKind::ReasoningProgress)
    );
    assert!(events.iter().any(|event| matches!(
        event.kind(),
        RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(_))
    )));
    assert_eq!(fixture.server.attempts(), 1);
    assert_eq!(fixture.releases(), 2);
    assert_eq!(fixture.release_after_blocking(), [1, 2]);
    let requests = fixture.server.requests();
    assert_eq!(requests[0].target, "/v1/models");
    assert_eq!(requests[1].target, "/v1/chat/completions");
    assert_eq!(
        requests[1].headers.get("authorization").map(String::as_str),
        Some("Bearer fixture-secret")
    );
    let body: serde_json::Value = serde_json::from_slice(&requests[1].body).expect("request JSON");
    assert_eq!(body["model"], "kimi-k3");
    assert_eq!(body["reasoning_effort"], "high");
    assert_eq!(body["max_completion_tokens"], 128);
    for omitted in [
        "temperature",
        "top_p",
        "n",
        "presence_penalty",
        "frequency_penalty",
    ] {
        assert!(body.get(omitted).is_none());
    }
    assert!(matches!(block_on(run.close()), CleanupOutcome::Clean));
}

#[test]
fn access_reasoning_output_and_elapsed_deadline_fail_before_effects() {
    let fixture = Fixture::new();
    let driver = KimiPlatformDirectDriver::new();
    let wrong_audience = EndpointAudience::new("api.moonshot.cn").expect("audience");
    let error = block_on(driver.start_run(
        fixture.plan_with_audience(DriverRole::StructuredRun, wrong_audience),
        request(OperationPolicy::offline().with_reasoning_mode(mode("high"))),
        fixture.services(),
    ))
    .err()
    .expect("regional audience fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.kimi_platform.access_binding_rejected"
    );

    let error = block_on(driver.start_run(
        fixture.plan(DriverRole::StructuredRun),
        request(OperationPolicy::offline()),
        fixture.services(),
    ))
    .err()
    .expect("reasoning is required");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.kimi_platform.reasoning_missing"
    );

    let unbounded = StructuredRunRequest::new(
        RequestId::new("unbounded").expect("request id"),
        OperationContent::new("fixture prompt").expect("content"),
        OperationPolicy::offline().with_reasoning_mode(mode("high")),
    );
    let error = block_on(driver.start_run(
        fixture.plan(DriverRole::StructuredRun),
        unbounded,
        fixture.services(),
    ))
    .err()
    .expect("output bound is required");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.kimi_platform.output_limit_missing"
    );

    let expired = request(OperationPolicy::offline().with_reasoning_mode(mode("high")))
        .with_deadline(Deadline::at(MonotonicInstant::from_ticks(0)));
    let error = block_on(driver.start_run(
        fixture.plan(DriverRole::StructuredRun),
        expired,
        fixture.services(),
    ))
    .err()
    .expect("elapsed deadline fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.kimi_platform.deadline_elapsed"
    );
    assert!(fixture.server.requests().is_empty());
    assert_eq!(fixture.releases(), 0);
}

#[test]
fn unknown_mismatch_provider_error_and_disconnect_remain_distinct() {
    for (stream, code) in [
        (
            StreamFixture::Unknown,
            "swallowtail.kimi_platform.content_semantics_unknown",
        ),
        (
            StreamFixture::Mismatch,
            "swallowtail.kimi_platform.returned_model_mismatch",
        ),
        (
            StreamFixture::ProviderError,
            "swallowtail.kimi_platform.rate_limited",
        ),
        (
            StreamFixture::Disconnect,
            "swallowtail.kimi_platform.sse_disconnected",
        ),
    ] {
        let fixture = Fixture::with_stream(stream);
        let (run, _events, outcome) = complete_run(&fixture);
        let TerminalStatus::ProviderFailed(diagnostic) = outcome.status() else {
            panic!("fixture should fail: {:?}", outcome.status());
        };
        assert_eq!(diagnostic.code(), code);
        assert!(!diagnostic.to_string().contains("raw private detail"));
        assert_eq!(fixture.server.attempts(), 1);
        assert_eq!(fixture.releases(), 1);
        assert!(matches!(block_on(run.close()), CleanupOutcome::Clean));
    }
}

#[test]
fn cancellation_stops_the_local_connection_and_joins_before_release() {
    let fixture = Fixture::with_stream(StreamFixture::WaitForCancel);
    let mut run = block_on(KimiPlatformDirectDriver::new().start_run(
        fixture.plan(DriverRole::StructuredRun),
        request(OperationPolicy::offline().with_reasoning_mode(mode("high"))),
        fixture.services(),
    ))
    .expect("run starts");
    let _events = run.take_events().expect("events");
    let terminal = run.take_terminal_outcome().expect("terminal");
    for _ in 0..100 {
        if fixture.server.attempts() == 1 {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    block_on(run.cancellation().request()).expect("cancellation accepted");
    let outcome = block_on(terminal);
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(fixture.server.attempts(), 1);
    assert_eq!(fixture.releases(), 1);
    assert!(matches!(block_on(run.close()), CleanupOutcome::Clean));
}

#[test]
fn in_flight_deadline_times_out_and_releases_after_connection_join() {
    let fixture = Fixture::with_stream(StreamFixture::WaitForCancel);
    let deadline = Deadline::at(MonotonicInstant::from_ticks(20));
    let timed = request(OperationPolicy::offline().with_reasoning_mode(mode("high")))
        .with_deadline(deadline);
    let mut run = block_on(KimiPlatformDirectDriver::new().start_run(
        fixture.plan(DriverRole::StructuredRun),
        timed,
        fixture.services(),
    ))
    .expect("run starts");
    let _events = run.take_events().expect("events");
    let outcome = block_on(run.take_terminal_outcome().expect("terminal"));
    assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
    assert_eq!(fixture.server.attempts(), 1);
    assert_eq!(fixture.releases(), 1);
    assert!(matches!(block_on(run.close()), CleanupOutcome::Clean));
}

fn complete_run(
    fixture: &Fixture,
) -> (
    Box<dyn swallowtail_runtime::RunHandle>,
    Vec<swallowtail_runtime::RuntimeEvent>,
    swallowtail_runtime::TerminalOutcome,
) {
    let mut run = block_on(KimiPlatformDirectDriver::new().start_run(
        fixture.plan(DriverRole::StructuredRun),
        request(OperationPolicy::offline().with_reasoning_mode(mode("high"))),
        fixture.services(),
    ))
    .expect("run starts");
    let mut stream = run.take_events().expect("events");
    let terminal = run.take_terminal_outcome().expect("terminal");
    let (events, outcome) = block_on(async {
        let mut events = Vec::new();
        while let Some(event) = stream.next().await {
            events.push(event.expect("event"));
        }
        (events, terminal.await)
    });
    (run, events, outcome)
}

fn request(policy: OperationPolicy) -> StructuredRunRequest {
    StructuredRunRequest::new(
        RequestId::new("fixture-run").expect("request id"),
        OperationContent::new("fixture prompt").expect("content"),
        policy,
    )
    .with_maximum_output_tokens(NonZeroU64::new(128).expect("positive bound"))
}

fn mode(value: &str) -> ReasoningMode {
    ReasoningMode::new(value).expect("reasoning mode")
}
