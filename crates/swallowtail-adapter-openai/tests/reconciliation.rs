use crate::{fixture::Fixture, server::ServerMode, services::TimeMode};
use futures_executor::block_on;
use futures_util::StreamExt;
use std::num::NonZeroU64;
use swallowtail_adapter_openai::{
    OPENAI_BACKGROUND_MODEL_ID, OPENAI_BACKGROUND_MODEL_ROUTE_ID, OpenAiBackgroundModelSelection,
    OpenAiBackgroundReconciliationInput, OpenAiBackgroundRunProfileInput,
    prepare_openai_background,
};
use swallowtail_core::{ModelId, ModelRouteId, ModelRouteRevision};
use swallowtail_runtime::{
    CancellationControl, CleanupOutcome, Deadline, InterruptedRunState, MonotonicInstant,
    OperationContent, OperationDetachmentAcknowledgement, PersistedProviderRunCheckpoint,
    RequestId, TerminalStatus,
};

#[test]
fn detached_response_reconciles_exact_completed_run_without_cancel_or_delete() {
    let fixture = Fixture::new(ServerMode::CancelRace, "host.local", TimeMode::Pending);
    let prepared = prepare_openai_background(fixture.preparation_input(), &fixture.services())
        .expect("OpenAI background integration prepares");
    let run = prepared
        .prepare_background_run(profile("detached-complete").with_active_run_detachment())
        .expect("detachable run prepares");
    let original_plan = run.plan().clone();
    let mut handle = block_on(run.start_run(fixture.services())).expect("run starts");
    let mut events = handle.take_events().expect("events exist");
    let terminal = handle.take_terminal_outcome().expect("terminal exists");
    let persisted = block_on(async {
        loop {
            let event = events
                .next()
                .await
                .expect("checkpoint event exists")
                .expect("event succeeds");
            if let Some(checkpoint) = event.run_reconciliation_checkpoint() {
                break checkpoint
                    .export_persisted(&original_plan)
                    .expect("checkpoint persists");
            }
        }
    });
    let detachment = handle.detachment().expect("detachment is exposed");
    assert_eq!(
        block_on(detachment.request()).expect("detachment succeeds"),
        OperationDetachmentAcknowledgement::Requested
    );
    assert_eq!(
        block_on(detachment.request()).expect("detachment is idempotent"),
        OperationDetachmentAcknowledgement::AlreadyRequested
    );
    let outcome = block_on(async {
        while let Some(event) = events.next().await {
            event.expect("event succeeds");
        }
        terminal.await
    });
    assert_eq!(outcome.status(), &TerminalStatus::Detached);
    assert!(outcome.remote_resource_deletions().next().is_none());
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);

    let reconciliation = prepared
        .prepare_run_reconciliation(reconciliation_input(
            "reconcile-complete",
            persisted.clone(),
        ))
        .expect("reconciliation prepares");
    let outcome =
        block_on(reconciliation.reconcile(fixture.services())).expect("reconciliation succeeds");
    assert_eq!(
        outcome.observation().state(),
        InterruptedRunState::Completed
    );
    assert_eq!(
        outcome
            .observation()
            .output()
            .expect("output exists")
            .as_str(),
        "Hello"
    );
    assert!(outcome.observation().usage().is_some());
    assert_eq!(outcome.cleanup(), &CleanupOutcome::Clean);
    let oversized = prepared
        .prepare_run_reconciliation(OpenAiBackgroundReconciliationInput::new(
            RequestId::new("reconcile-oversized").expect("request id is valid"),
            selection(),
            persisted,
            NonZeroU64::new(1).expect("bound is non-zero"),
            Some(Deadline::at(MonotonicInstant::from_ticks(100_000))),
        ))
        .expect("bounded reconciliation prepares");
    let error = block_on(oversized.reconcile(fixture.services()))
        .expect_err("oversized recovered output rejects");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.provider_run_reconciliation.output_oversized"
    );
    let requests = fixture.server.requests();
    assert_eq!(
        requests
            .iter()
            .filter(|request| request.method == "POST")
            .count(),
        1
    );
    assert_eq!(
        requests
            .iter()
            .filter(|request| request.method == "DELETE")
            .count(),
        0
    );
    assert_eq!(
        requests
            .iter()
            .filter(|request| request.method == "GET")
            .count(),
        2
    );
    assert_eq!(fixture.server.inference_attempts(), 1);
    assert_eq!(fixture.releases(), 3);
}

#[test]
fn active_response_reconciliation_is_exact_and_cross_host_or_foreign_truth_fails_closed() {
    let fixture = Fixture::new(ServerMode::HoldForCancel, "host.local", TimeMode::Pending);
    let (prepared, persisted) = detach_with_checkpoint(&fixture, "active");
    let reconciliation = prepared
        .prepare_run_reconciliation(reconciliation_input("reconcile-active", persisted.clone()))
        .expect("active reconciliation prepares");
    let outcome = block_on(reconciliation.reconcile(fixture.services()))
        .expect("active reconciliation succeeds");
    assert_eq!(outcome.observation().state(), InterruptedRunState::Active);
    assert!(outcome.observation().output().is_none());

    let foreign_host = Fixture::new(ServerMode::HoldForCancel, "host.foreign", TimeMode::Pending);
    let foreign_prepared =
        prepare_openai_background(foreign_host.preparation_input(), &foreign_host.services())
            .expect("foreign host prepares");
    let error = foreign_prepared
        .prepare_run_reconciliation(reconciliation_input("foreign-host", persisted))
        .expect_err("cross-host checkpoint rejects");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.openai.preparation.reconciliation_checkpoint_rejected"
    );
    assert!(foreign_host.server.requests().is_empty());

    let mismatch = Fixture::new(
        ServerMode::ReconcileMismatch,
        "host.mismatch",
        TimeMode::Pending,
    );
    let (prepared, persisted) = detach_with_checkpoint(&mismatch, "mismatch");
    let reconciliation = prepared
        .prepare_run_reconciliation(reconciliation_input("foreign-response", persisted))
        .expect("mismatch reconciliation prepares");
    let error = block_on(reconciliation.reconcile(mismatch.services()))
        .expect_err("foreign provider response rejects");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.openai.reconciliation_response_mismatch"
    );
}

#[test]
fn default_terminal_and_cancellation_dispositions_remain_non_detachable() {
    let fixture = Fixture::new(ServerMode::Success, "host.local", TimeMode::Pending);
    let prepared = prepare_openai_background(fixture.preparation_input(), &fixture.services())
        .expect("OpenAI background integration prepares");
    let run = prepared
        .prepare_background_run(profile("ordinary"))
        .expect("ordinary run prepares");
    let handle = block_on(run.start_run(fixture.services())).expect("run starts");
    assert!(handle.detachment().is_none());
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);

    let terminal = Fixture::new(ServerMode::Success, "host.terminal", TimeMode::Pending);
    let prepared = prepare_openai_background(terminal.preparation_input(), &terminal.services())
        .expect("OpenAI background integration prepares");
    let run = prepared
        .prepare_background_run(profile("terminal").with_active_run_detachment())
        .expect("detachable run prepares");
    let mut handle = block_on(run.start_run(terminal.services())).expect("run starts");
    let mut events = handle.take_events().expect("events exist");
    let outcome = block_on(async {
        while let Some(event) = events.next().await {
            event.expect("event succeeds");
        }
        handle
            .take_terminal_outcome()
            .expect("terminal exists")
            .await
    });
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    let error = block_on(handle.detachment().expect("detachment exists").request())
        .expect_err("terminal run rejects detachment");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.openai.detachment_terminal"
    );
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);

    let cancelled = Fixture::new(ServerMode::HoldForCancel, "host.cancel", TimeMode::Pending);
    let prepared = prepare_openai_background(cancelled.preparation_input(), &cancelled.services())
        .expect("OpenAI background integration prepares");
    let run = prepared
        .prepare_background_run(profile("cancelled").with_active_run_detachment())
        .expect("detachable run prepares");
    let handle = block_on(run.start_run(cancelled.services())).expect("run starts");
    block_on(handle.cancellation().request()).expect("cancellation starts");
    let error = block_on(handle.detachment().expect("detachment exists").request())
        .expect_err("cancellation wins");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.openai.detachment_cancelled"
    );
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
}

#[test]
fn reconciliation_cancellation_and_elapsed_deadline_fail_before_provider_observation() {
    let fixture = Fixture::new(ServerMode::HoldForCancel, "host.local", TimeMode::Pending);
    let (prepared, persisted) = detach_with_checkpoint(&fixture, "reconciliation-stop");

    let cancelled = prepared
        .prepare_run_reconciliation(reconciliation_input(
            "reconciliation-cancelled",
            persisted.clone(),
        ))
        .expect("cancelled reconciliation prepares");
    block_on(cancelled.request().cancellation().request()).expect("cancellation is requested");
    let error = block_on(cancelled.reconcile(fixture.services()))
        .expect_err("pre-cancelled reconciliation rejects");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.openai.reconciliation_cancelled"
    );

    let elapsed = prepared
        .prepare_run_reconciliation(OpenAiBackgroundReconciliationInput::new(
            RequestId::new("reconciliation-elapsed").expect("request id is valid"),
            selection(),
            persisted,
            NonZeroU64::new(1024).expect("bound is non-zero"),
            Some(Deadline::at(MonotonicInstant::from_ticks(0))),
        ))
        .expect("deadline reconciliation prepares");
    let error = block_on(elapsed.reconcile(fixture.services()))
        .expect_err("elapsed reconciliation rejects");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.openai.reconciliation_timed_out"
    );

    assert_eq!(
        fixture
            .server
            .requests()
            .iter()
            .filter(|request| request.method == "GET")
            .count(),
        0
    );
}

fn detach_with_checkpoint(
    fixture: &Fixture,
    id: &str,
) -> (
    swallowtail_adapter_openai::OpenAiBackgroundPreparedIntegration,
    PersistedProviderRunCheckpoint,
) {
    let prepared = prepare_openai_background(fixture.preparation_input(), &fixture.services())
        .expect("OpenAI background integration prepares");
    let run = prepared
        .prepare_background_run(profile(id).with_active_run_detachment())
        .expect("detachable run prepares");
    let plan = run.plan().clone();
    let mut handle = block_on(run.start_run(fixture.services())).expect("run starts");
    let mut events = handle.take_events().expect("events exist");
    let terminal = handle.take_terminal_outcome().expect("terminal exists");
    let persisted = block_on(async {
        loop {
            let event = events
                .next()
                .await
                .expect("checkpoint event exists")
                .expect("event succeeds");
            if let Some(checkpoint) = event.run_reconciliation_checkpoint() {
                break checkpoint
                    .export_persisted(&plan)
                    .expect("checkpoint persists");
            }
        }
    });
    block_on(handle.detachment().expect("detachment exists").request())
        .expect("detachment succeeds");
    let outcome = block_on(async {
        while let Some(event) = events.next().await {
            event.expect("event succeeds");
        }
        terminal.await
    });
    assert_eq!(outcome.status(), &TerminalStatus::Detached);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    (prepared, persisted)
}

fn profile(id: &str) -> OpenAiBackgroundRunProfileInput {
    OpenAiBackgroundRunProfileInput::background_with_temporary_retention_and_one_reattachment(
        RequestId::new(id).expect("request id is valid"),
        selection(),
        OperationContent::new("Say hello").expect("content is valid"),
        NonZeroU64::new(64).expect("limit is non-zero"),
        Deadline::at(MonotonicInstant::from_ticks(100_000)),
    )
}

fn reconciliation_input(
    id: &str,
    checkpoint: PersistedProviderRunCheckpoint,
) -> OpenAiBackgroundReconciliationInput {
    OpenAiBackgroundReconciliationInput::new(
        RequestId::new(id).expect("request id is valid"),
        selection(),
        checkpoint,
        NonZeroU64::new(1024).expect("bound is non-zero"),
        Some(Deadline::at(MonotonicInstant::from_ticks(100_000))),
    )
}

fn selection() -> OpenAiBackgroundModelSelection {
    OpenAiBackgroundModelSelection::new(
        ModelRouteId::new(OPENAI_BACKGROUND_MODEL_ROUTE_ID).expect("route id is valid"),
        ModelRouteRevision::new("prepared-1").expect("route revision is valid"),
        ModelId::new(OPENAI_BACKGROUND_MODEL_ID).expect("model id is valid"),
    )
}
