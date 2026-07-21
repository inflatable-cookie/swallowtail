#[path = "direct_support/fixture.rs"]
mod fixture;
#[path = "support/server.rs"]
mod server;
#[path = "support/services.rs"]
mod services;

use fixture::Fixture;
use futures_executor::block_on;
use futures_util::StreamExt;
use server::ServerMode;
use services::TimeMode;
use std::num::{NonZeroU32, NonZeroU64};
use swallowtail_adapter_openai::{OpenAiBackgroundDriver, openai_background_descriptor};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, MonotonicInstant, OperationContent, OperationPolicy,
    ProviderCancellationOutcome, ProviderExecutionPolicy, ProviderObservation,
    ProviderRetentionPolicy, RequestId, StreamReattachmentPolicy, StructuredRunDriver,
    StructuredRunRequest, TerminalStatus,
};

#[test]
fn descriptor_and_policy_expose_only_the_bounded_background_route() {
    let descriptor = openai_background_descriptor();
    assert_eq!(
        descriptor.identity().id().as_str(),
        "swallowtail.openai.background"
    );
    assert_eq!(descriptor.integration_family().as_str(), "openai");
    assert_eq!(
        descriptor.transport_family().as_str(),
        "http-sse-background"
    );
    assert!(descriptor.supports_role(swallowtail_core::DriverRole::StructuredRun));
    assert!(!descriptor.supports_role(swallowtail_core::DriverRole::InteractiveSession));
    assert!(
        !descriptor
            .required_host_services(swallowtail_core::DriverRole::StructuredRun)
            .any(|service| service == swallowtail_core::HostServiceKind::Process)
    );

    let ordinary = Fixture::new(ServerMode::Success, "host.local", TimeMode::Pending);
    let error = block_on(OpenAiBackgroundDriver::new().start_run(
        ordinary.plan(),
        base_request("ordinary", OperationPolicy::offline()),
        ordinary.services(),
    ))
    .err()
    .expect("ordinary policy is rejected");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.openai.unsupported_input"
    );
    assert!(ordinary.server.requests().is_empty());
    assert_eq!(ordinary.releases(), 0);
}

#[test]
fn one_create_and_one_reattachment_complete_in_both_host_topologies() {
    for host in ["host.local", "host.remote-authoritative"] {
        let fixture = Fixture::new(ServerMode::Success, host, TimeMode::Pending);
        let (run, events, outcome) = complete(&fixture, "success");
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(outcome.output().expect("output exists").as_str(), "Hello");
        assert!(run.provider_run_ref().is_some());
        assert!(events.iter().any(|event| matches!(
            event.kind(),
            swallowtail_runtime::RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(
                _
            ))
        )));
        assert!(events.iter().any(|event| matches!(
            event.kind(),
            swallowtail_runtime::RuntimeEventKind::ProviderObservation(
                ProviderObservation::RateLimit(_)
            )
        )));
        let targets: Vec<_> = fixture
            .server
            .requests()
            .into_iter()
            .map(|request| request.target)
            .collect();
        assert_eq!(targets[0], "/v1/responses");
        assert_eq!(
            targets[1],
            "/v1/responses/resp_fixture_123?stream=true&starting_after=3"
        );
        assert_eq!(fixture.server.inference_attempts(), 1);
        assert_eq!(fixture.releases(), 1);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    }
}

#[test]
fn provider_failure_after_reattachment_is_terminal_and_redacted() {
    let fixture = Fixture::new(ServerMode::ProviderFailed, "host.local", TimeMode::Pending);
    let (run, _events, outcome) = complete(&fixture, "provider-failure");
    assert!(matches!(
        outcome.status(),
        TerminalStatus::ProviderFailed(_)
    ));
    assert!(!format!("{:?}", outcome.status()).contains("synthetic-private-message"));
    assert_eq!(fixture.server.inference_attempts(), 1);
    assert_eq!(fixture.releases(), 1);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
}

#[test]
fn native_cancel_preserves_confirmed_raced_and_unconfirmed_truth() {
    for (mode, status, cancellation) in [
        (
            ServerMode::HoldForCancel,
            TerminalStatus::Cancelled,
            ProviderCancellationOutcome::Confirmed,
        ),
        (
            ServerMode::CancelRace,
            TerminalStatus::Completed,
            ProviderCancellationOutcome::RacedWithCompletion,
        ),
        (
            ServerMode::CancelUnconfirmed,
            TerminalStatus::Cancelled,
            ProviderCancellationOutcome::Unconfirmed,
        ),
    ] {
        let fixture = Fixture::new(mode, "host.local", TimeMode::Pending);
        let mut run = block_on(OpenAiBackgroundDriver::new().start_run(
            fixture.plan(),
            request("cancel"),
            fixture.services(),
        ))
        .expect("run starts");
        let terminal = run.take_terminal_outcome().expect("terminal exists");
        block_on(run.cancellation().request()).expect("cancel is accepted");
        let outcome = block_on(terminal);
        assert_eq!(outcome.status(), &status);
        assert_eq!(outcome.provider_cancellation(), Some(cancellation));
        assert_eq!(fixture.server.inference_attempts(), 1);
        assert_eq!(fixture.releases(), 1);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    }
}

#[test]
fn deadline_and_pre_identity_disconnect_remain_safe_and_distinct() {
    let deadline = Fixture::new(ServerMode::HoldForCancel, "host.local", TimeMode::Delayed);
    let (run, _events, outcome) = complete(&deadline, "deadline");
    assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
    assert_eq!(
        outcome.provider_cancellation(),
        Some(ProviderCancellationOutcome::Confirmed)
    );
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);

    let disconnected = Fixture::new(
        ServerMode::DisconnectBeforeIdentity,
        "host.local",
        TimeMode::Pending,
    );
    let error = block_on(OpenAiBackgroundDriver::new().start_run(
        disconnected.plan(),
        request("before-identity"),
        disconnected.services(),
    ))
    .err()
    .expect("missing identity fails start");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.openai.remote_state_unconfirmed"
    );
    assert!(!format!("{error:?}").contains("fixture-secret"));
    assert_eq!(disconnected.releases(), 1);
}

fn request(id: &str) -> StructuredRunRequest {
    base_request(id, background_policy())
}

fn base_request(id: &str, policy: OperationPolicy) -> StructuredRunRequest {
    StructuredRunRequest::new(
        RequestId::new(id).expect("request id is valid"),
        OperationContent::new("Say hello").expect("content is valid"),
        policy,
    )
    .with_maximum_output_tokens(NonZeroU64::new(64).expect("limit is non-zero"))
    .with_deadline(Deadline::at(MonotonicInstant::from_ticks(100_000)))
}

fn background_policy() -> OperationPolicy {
    OperationPolicy::offline()
        .with_provider_execution(ProviderExecutionPolicy::Background)
        .with_provider_retention(ProviderRetentionPolicy::TemporaryAllowed)
        .with_stream_reattachment(StreamReattachmentPolicy::Bounded(
            NonZeroU32::new(1).expect("one is non-zero"),
        ))
}

fn complete(
    fixture: &Fixture,
    id: &str,
) -> (
    Box<dyn swallowtail_runtime::RunHandle>,
    Vec<swallowtail_runtime::RuntimeEvent>,
    swallowtail_runtime::TerminalOutcome,
) {
    let mut run = block_on(OpenAiBackgroundDriver::new().start_run(
        fixture.plan(),
        request(id),
        fixture.services(),
    ))
    .expect("run starts");
    let mut stream = run.take_events().expect("events exist");
    let terminal = run.take_terminal_outcome().expect("terminal exists");
    let (events, outcome) = block_on(async {
        let mut events = Vec::new();
        while let Some(event) = stream.next().await {
            events.push(event.expect("event succeeds"));
        }
        (events, terminal.await)
    });
    (run, events, outcome)
}
