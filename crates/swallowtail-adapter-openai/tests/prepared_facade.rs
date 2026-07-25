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
use swallowtail_adapter_openai::{
    OPENAI_BACKGROUND_FACADE_REVISION, OPENAI_BACKGROUND_MODEL_ID,
    OPENAI_BACKGROUND_MODEL_ROUTE_ID, OpenAiBackgroundModelSelection,
    OpenAiBackgroundRunProfileInput, prepare_openai_background,
};
use swallowtail_core::{ModelId, ModelRouteId, ModelRouteRevision};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, MonotonicInstant, OperationContent, ProviderCancellationOutcome,
    ProviderExecutionPolicy, ProviderObservation, ProviderRetentionPolicy, RequestId,
    StreamReattachmentPolicy, TerminalStatus,
};

#[test]
fn prepared_background_run_preserves_one_attempt_and_one_reattachment_on_both_hosts() {
    for host in ["host.local", "host.remote-authoritative"] {
        let fixture = Fixture::new(ServerMode::Success, host, TimeMode::Pending);
        let prepared = prepare_openai_background(fixture.preparation_input(), &fixture.services())
            .expect("OpenAI background integration prepares");
        let run = prepared
            .prepare_background_run(profile("prepared-success"))
            .expect("background run prepares");

        assert_eq!(
            run.plan().protocol_facade_id().as_str(),
            OPENAI_BACKGROUND_FACADE_REVISION
        );
        assert_eq!(
            run.request().policy().provider_execution(),
            ProviderExecutionPolicy::Background
        );
        assert_eq!(
            run.request().policy().provider_retention(),
            ProviderRetentionPolicy::TemporaryAllowed
        );
        assert_eq!(
            run.request().policy().stream_reattachment(),
            StreamReattachmentPolicy::Bounded(NonZeroU32::new(1).expect("one is non-zero"))
        );
        let compatibility: Vec<_> = run
            .evidence()
            .operation()
            .interface_compatibility()
            .collect();
        assert_eq!(compatibility.len(), 1);
        assert!(matches!(
            compatibility[0].assessment(),
            swallowtail_core::InterfaceCompatibilityAssessment::Qualified(_)
        ));

        let (run, events, outcome) = complete(run.start_run(fixture.services()));
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(outcome.output().expect("output exists").as_str(), "Hello");
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
        assert_eq!(
            targets,
            [
                "/v1/responses",
                "/v1/responses/resp_fixture_123?stream=true&starting_after=3",
            ]
        );
        assert_eq!(fixture.server.inference_attempts(), 1);
        assert_eq!(fixture.releases(), 1);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    }
}

#[test]
fn prepared_cancel_and_deadline_preserve_remote_truth_and_cleanup() {
    let raced = Fixture::new(ServerMode::CancelRace, "host.local", TimeMode::Pending);
    let prepared = prepare_openai_background(raced.preparation_input(), &raced.services())
        .expect("OpenAI background integration prepares");
    let run = prepared
        .prepare_background_run(profile("prepared-cancel-race"))
        .expect("background run prepares");
    let mut handle = block_on(run.start_run(raced.services())).expect("run starts");
    assert_eq!(
        block_on(handle.cancellation().request()).expect("cancel request succeeds"),
        swallowtail_runtime::CancellationAcknowledgement::Requested
    );
    let outcome = consume(&mut handle);
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome.provider_cancellation(),
        Some(ProviderCancellationOutcome::RacedWithCompletion)
    );
    assert_eq!(raced.server.inference_attempts(), 1);
    assert_eq!(raced.releases(), 1);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);

    let deadline = Fixture::new(ServerMode::HoldForCancel, "host.remote", TimeMode::Delayed);
    let prepared = prepare_openai_background(deadline.preparation_input(), &deadline.services())
        .expect("OpenAI background integration prepares");
    let run = prepared
        .prepare_background_run(profile("prepared-deadline"))
        .expect("background run prepares");
    let (handle, _events, outcome) = complete(run.start_run(deadline.services()));
    assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
    assert_eq!(
        outcome.provider_cancellation(),
        Some(ProviderCancellationOutcome::Confirmed)
    );
    assert_eq!(deadline.server.inference_attempts(), 1);
    assert_eq!(deadline.releases(), 1);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
}

#[test]
fn prepared_policy_and_route_drift_fail_before_endpoint_or_credential_effects() {
    let fixture = Fixture::new(ServerMode::Success, "host.local", TimeMode::Pending);
    let prepared = prepare_openai_background(fixture.preparation_input(), &fixture.services())
        .expect("OpenAI background integration prepares");

    for input in [
        explicit(
            "attached",
            selection(OPENAI_BACKGROUND_MODEL_ROUTE_ID, OPENAI_BACKGROUND_MODEL_ID),
            ProviderExecutionPolicy::Attached,
            ProviderRetentionPolicy::TemporaryAllowed,
            reattachment(),
        ),
        explicit(
            "retention",
            selection(OPENAI_BACKGROUND_MODEL_ROUTE_ID, OPENAI_BACKGROUND_MODEL_ID),
            ProviderExecutionPolicy::Background,
            ProviderRetentionPolicy::Prohibited,
            reattachment(),
        ),
        explicit(
            "reattach",
            selection(OPENAI_BACKGROUND_MODEL_ROUTE_ID, OPENAI_BACKGROUND_MODEL_ID),
            ProviderExecutionPolicy::Background,
            ProviderRetentionPolicy::TemporaryAllowed,
            StreamReattachmentPolicy::Disabled,
        ),
        explicit(
            "route",
            selection("openai.public.other", OPENAI_BACKGROUND_MODEL_ID),
            ProviderExecutionPolicy::Background,
            ProviderRetentionPolicy::TemporaryAllowed,
            reattachment(),
        ),
        explicit(
            "model",
            selection(OPENAI_BACKGROUND_MODEL_ROUTE_ID, "gpt-other"),
            ProviderExecutionPolicy::Background,
            ProviderRetentionPolicy::TemporaryAllowed,
            reattachment(),
        ),
    ] {
        assert!(prepared.prepare_background_run(input).is_err());
    }

    assert!(fixture.server.requests().is_empty());
    assert_eq!(fixture.releases(), 0);
}

fn profile(id: &str) -> OpenAiBackgroundRunProfileInput {
    OpenAiBackgroundRunProfileInput::background_with_temporary_retention_and_one_reattachment(
        RequestId::new(id).expect("request id is valid"),
        selection(OPENAI_BACKGROUND_MODEL_ROUTE_ID, OPENAI_BACKGROUND_MODEL_ID),
        OperationContent::new("Say hello").expect("content is valid"),
        NonZeroU64::new(64).expect("limit is non-zero"),
        Deadline::at(MonotonicInstant::from_ticks(100_000)),
    )
}

fn explicit(
    id: &str,
    model: OpenAiBackgroundModelSelection,
    execution: ProviderExecutionPolicy,
    retention: ProviderRetentionPolicy,
    reattachment: StreamReattachmentPolicy,
) -> OpenAiBackgroundRunProfileInput {
    OpenAiBackgroundRunProfileInput::new(
        RequestId::new(id).expect("request id is valid"),
        model,
        OperationContent::new("Say hello").expect("content is valid"),
        NonZeroU64::new(64).expect("limit is non-zero"),
        Deadline::at(MonotonicInstant::from_ticks(100_000)),
        execution,
        retention,
        reattachment,
    )
}

fn selection(route: &str, model: &str) -> OpenAiBackgroundModelSelection {
    OpenAiBackgroundModelSelection::new(
        ModelRouteId::new(route).expect("route id is valid"),
        ModelRouteRevision::new("prepared-1").expect("route revision is valid"),
        ModelId::new(model).expect("model id is valid"),
    )
}

fn reattachment() -> StreamReattachmentPolicy {
    StreamReattachmentPolicy::Bounded(NonZeroU32::new(1).expect("one is non-zero"))
}

fn complete(
    run: swallowtail_runtime::BoxFuture<
        'static,
        Result<Box<dyn swallowtail_runtime::RunHandle>, swallowtail_runtime::RuntimeFailure>,
    >,
) -> (
    Box<dyn swallowtail_runtime::RunHandle>,
    Vec<swallowtail_runtime::RuntimeEvent>,
    swallowtail_runtime::TerminalOutcome,
) {
    let mut run = block_on(run).expect("run starts");
    let mut events = run.take_events().expect("events exist");
    let terminal = run.take_terminal_outcome().expect("terminal exists");
    let (events, outcome) = block_on(async {
        let mut observed = Vec::new();
        while let Some(event) = events.next().await {
            observed.push(event.expect("event succeeds"));
        }
        (observed, terminal.await)
    });
    (run, events, outcome)
}

fn consume(
    run: &mut Box<dyn swallowtail_runtime::RunHandle>,
) -> swallowtail_runtime::TerminalOutcome {
    let mut events = run.take_events().expect("events exist");
    let terminal = run.take_terminal_outcome().expect("terminal exists");
    block_on(async {
        while let Some(event) = events.next().await {
            event.expect("event succeeds");
        }
        terminal.await
    })
}
