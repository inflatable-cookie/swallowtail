use super::{
    CleanupOutcome, ProviderCancellationOutcome, ProviderRequestObservation,
    RemoteResourceDeletionOutcome, TerminalFailureSource, TerminalOutcome, TerminalStatus,
    terminal_outcome_channel,
};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

#[test]
fn dropping_a_sender_recovers_a_poisoned_terminal_lock() {
    let (sender, mut future) = terminal_outcome_channel();
    let state = std::sync::Arc::clone(&future.state);
    let poisoned = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _guard = state.lock().expect("terminal lock");
        panic!("poison the terminal lock");
    }));
    assert!(poisoned.is_err(), "the fixture poisons the lock");

    drop(sender);

    let mut context = Context::from_waker(Waker::noop());
    let outcome = match Pin::new(&mut future).poll(&mut context) {
        Poll::Ready(outcome) => outcome,
        Poll::Pending => panic!("a poisoned lock must not stall the terminal future"),
    };
    assert!(matches!(outcome.status(), TerminalStatus::RuntimeFailed(_)));
}

#[test]
fn exactly_one_terminal_outcome_wins() {
    let (sender, _future) = terminal_outcome_channel();
    sender
        .complete(TerminalOutcome::new(
            TerminalStatus::Completed,
            CleanupOutcome::Clean,
        ))
        .expect("first terminal outcome wins");
    sender
        .complete(TerminalOutcome::new(
            TerminalStatus::Cancelled,
            CleanupOutcome::Clean,
        ))
        .expect_err("second terminal outcome must fail");
}

#[test]
fn provider_success_does_not_hide_cleanup_failure() {
    let diagnostic =
        swallowtail_core::SafeDiagnostic::new("fixture.cleanup_failed", "Cleanup failed");
    let outcome = TerminalOutcome::new(
        TerminalStatus::Completed,
        CleanupOutcome::Failed(diagnostic.clone()),
    );

    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(outcome.cleanup(), &CleanupOutcome::Failed(diagnostic));
}

#[test]
fn terminal_future_resolves_to_the_single_winner() {
    let (sender, mut future) = terminal_outcome_channel();
    let expected = TerminalOutcome::new(TerminalStatus::TimedOut, CleanupOutcome::Clean);
    sender
        .complete(expected.clone())
        .expect("terminal outcome completes once");
    let mut context = Context::from_waker(Waker::noop());

    assert_eq!(
        Pin::new(&mut future).poll(&mut context),
        Poll::Ready(expected)
    );
}

#[test]
fn dropping_the_last_sender_without_completion_resolves_with_a_failure() {
    let (sender, mut future) = terminal_outcome_channel();
    let second = sender.clone();
    drop(second);
    let mut context = Context::from_waker(Waker::noop());
    assert_eq!(
        Pin::new(&mut future).poll(&mut context),
        Poll::Pending,
        "a remaining sender keeps the future pending"
    );
    drop(sender);
    let outcome = match Pin::new(&mut future).poll(&mut context) {
        Poll::Ready(outcome) => outcome,
        Poll::Pending => panic!("the last sender dropping must resolve the future"),
    };
    assert_eq!(
        outcome.status(),
        &TerminalStatus::RuntimeFailed(swallowtail_core::SafeDiagnostic::new(
            "swallowtail.terminal_sender_dropped",
            "Operation terminal outcome was not published",
        ))
    );
}

#[test]
fn completing_before_drop_keeps_the_published_outcome() {
    let (sender, mut future) = terminal_outcome_channel();
    let expected = TerminalOutcome::new(TerminalStatus::Completed, CleanupOutcome::Clean);
    sender
        .complete(expected.clone())
        .expect("terminal outcome completes once");
    drop(sender);
    let mut context = Context::from_waker(Waker::noop());
    assert_eq!(
        Pin::new(&mut future).poll(&mut context),
        Poll::Ready(expected)
    );
}

#[test]
fn terminal_failure_dimensions_remain_distinct() {
    let diagnostic = swallowtail_core::SafeDiagnostic::new("fixture.failure", "Failed");
    let statuses = [
        TerminalStatus::Completed,
        TerminalStatus::Detached,
        TerminalStatus::Cancelled,
        TerminalStatus::TimedOut,
        TerminalStatus::ProviderRequestObserved(ProviderRequestObservation::new(
            crate::CallbackId::new("fixture-callback").expect("callback id is valid"),
            swallowtail_core::ExtensionNamespace::new("fixture/provider-request")
                .expect("namespace is valid"),
            swallowtail_core::ProviderRequestRef::new("provider-request-1")
                .expect("provider request ref is valid"),
        )),
        TerminalStatus::ProviderFailed(diagnostic.clone()),
        TerminalStatus::HostFailed(diagnostic.clone()),
        TerminalStatus::RuntimeFailed(diagnostic),
    ];

    for (index, status) in statuses.iter().enumerate() {
        assert!(!statuses[index + 1..].contains(status));
    }
}

#[test]
fn terminal_failure_has_one_route_neutral_view() {
    let diagnostic = swallowtail_core::SafeDiagnostic::new("fixture.failure", "Failed")
        .with_failure_classification(swallowtail_core::FailureClassification::new(
            swallowtail_core::FailureOrigin::Protocol,
            swallowtail_core::FailureKind::MalformedData,
            swallowtail_core::FailureRecovery::HarnessUpdateRequired,
        ));
    let outcome = TerminalOutcome::new(
        TerminalStatus::RuntimeFailed(diagnostic.clone()),
        CleanupOutcome::Failed(swallowtail_core::SafeDiagnostic::new(
            "fixture.cleanup",
            "Cleanup failed",
        )),
    );

    let failure = outcome.failure().expect("runtime failure is exposed");
    assert_eq!(failure.source(), TerminalFailureSource::Runtime);
    assert_eq!(failure.diagnostic(), &diagnostic);
    assert_eq!(
        failure.diagnostic().failure_classification().origin(),
        swallowtail_core::FailureOrigin::Protocol
    );
    assert_eq!(
        outcome
            .cleanup()
            .diagnostic()
            .expect("cleanup diagnostic remains separate")
            .code(),
        "fixture.cleanup"
    );
}

#[test]
fn provider_cancellation_truth_does_not_replace_terminal_status() {
    let confirmed = TerminalOutcome::new(TerminalStatus::Cancelled, CleanupOutcome::Clean)
        .with_provider_cancellation(ProviderCancellationOutcome::Confirmed);
    let raced = TerminalOutcome::new(TerminalStatus::Completed, CleanupOutcome::Clean)
        .with_provider_cancellation(ProviderCancellationOutcome::RacedWithCompletion);
    let unconfirmed = TerminalOutcome::new(TerminalStatus::Cancelled, CleanupOutcome::Clean)
        .with_provider_cancellation(ProviderCancellationOutcome::Unconfirmed);

    assert_eq!(
        confirmed.provider_cancellation(),
        Some(ProviderCancellationOutcome::Confirmed)
    );
    assert_eq!(raced.status(), &TerminalStatus::Completed);
    assert_eq!(
        unconfirmed.provider_cancellation(),
        Some(ProviderCancellationOutcome::Unconfirmed)
    );
}

#[test]
fn owned_remote_resource_deletion_truth_remains_per_resource() {
    let outcome = TerminalOutcome::new(
        TerminalStatus::Completed,
        CleanupOutcome::Degraded(swallowtail_core::SafeDiagnostic::new(
            "fixture.remote_cleanup_unconfirmed",
            "Remote cleanup was not fully confirmed",
        )),
    )
    .with_remote_resource_deletion(
        swallowtail_core::OwnedRemoteResourceKind::Session,
        RemoteResourceDeletionOutcome::Confirmed,
    )
    .with_remote_resource_deletion(
        swallowtail_core::OwnedRemoteResourceKind::Environment,
        RemoteResourceDeletionOutcome::Unconfirmed,
    );

    assert_eq!(
        outcome.remote_resource_deletion(swallowtail_core::OwnedRemoteResourceKind::Session),
        Some(RemoteResourceDeletionOutcome::Confirmed)
    );
    assert_eq!(
        outcome.remote_resource_deletion(swallowtail_core::OwnedRemoteResourceKind::Environment),
        Some(RemoteResourceDeletionOutcome::Unconfirmed)
    );
    assert_eq!(outcome.remote_resource_deletions().count(), 2);
}

#[test]
fn conversation_deletion_cannot_stand_in_for_item_deletion() {
    let outcome = TerminalOutcome::new(
        TerminalStatus::Completed,
        CleanupOutcome::Degraded(swallowtail_core::SafeDiagnostic::new(
            "fixture.conversation_items_unconfirmed",
            "Conversation item deletion was not confirmed",
        )),
    )
    .with_remote_resource_deletion(
        swallowtail_core::OwnedRemoteResourceKind::ConversationItems,
        RemoteResourceDeletionOutcome::Unconfirmed,
    )
    .with_remote_resource_deletion(
        swallowtail_core::OwnedRemoteResourceKind::Conversation,
        RemoteResourceDeletionOutcome::Confirmed,
    );

    assert_eq!(
        outcome
            .remote_resource_deletion(swallowtail_core::OwnedRemoteResourceKind::ConversationItems),
        Some(RemoteResourceDeletionOutcome::Unconfirmed)
    );
    assert_eq!(
        outcome.remote_resource_deletion(swallowtail_core::OwnedRemoteResourceKind::Conversation),
        Some(RemoteResourceDeletionOutcome::Confirmed)
    );
}

#[test]
fn terminal_already_set_implements_display_and_error() {
    use super::TerminalAlreadySet;
    use std::error::Error;

    let error = TerminalAlreadySet;
    assert_eq!(
        error.to_string(),
        "Operation already has a terminal outcome"
    );
    let as_error: &dyn Error = &error;
    assert!(as_error.source().is_none());
}

#[test]
fn merged_cleanup_reports_the_worse_of_two_releases() {
    let degraded = swallowtail_core::SafeDiagnostic::new("fixture.degraded", "Cleanup degraded");
    let failed = swallowtail_core::SafeDiagnostic::new("fixture.failed", "Cleanup failed");
    let cases = [
        (
            CleanupOutcome::Clean,
            CleanupOutcome::Clean,
            CleanupOutcome::Clean,
        ),
        (
            CleanupOutcome::Clean,
            CleanupOutcome::NotApplicable,
            CleanupOutcome::Clean,
        ),
        (
            CleanupOutcome::NotApplicable,
            CleanupOutcome::Clean,
            CleanupOutcome::Clean,
        ),
        (
            CleanupOutcome::NotApplicable,
            CleanupOutcome::NotApplicable,
            CleanupOutcome::NotApplicable,
        ),
        (
            CleanupOutcome::Clean,
            CleanupOutcome::Degraded(degraded.clone()),
            CleanupOutcome::Degraded(degraded.clone()),
        ),
        (
            CleanupOutcome::Degraded(degraded.clone()),
            CleanupOutcome::Clean,
            CleanupOutcome::Degraded(degraded.clone()),
        ),
        (
            CleanupOutcome::Degraded(degraded.clone()),
            CleanupOutcome::Failed(failed.clone()),
            CleanupOutcome::Failed(failed.clone()),
        ),
        (
            CleanupOutcome::Failed(failed.clone()),
            CleanupOutcome::Degraded(degraded.clone()),
            CleanupOutcome::Failed(failed.clone()),
        ),
        (
            CleanupOutcome::NotApplicable,
            CleanupOutcome::Failed(failed.clone()),
            CleanupOutcome::Failed(failed.clone()),
        ),
    ];
    for (left, right, expected) in cases {
        let merged = left.clone().merge(right.clone());
        assert_eq!(merged, expected, "merging {left:?} with {right:?}");
    }
}

#[test]
fn merging_two_failures_keeps_the_first_observed_reason() {
    let first = swallowtail_core::SafeDiagnostic::new("fixture.first", "First failure");
    let second = swallowtail_core::SafeDiagnostic::new("fixture.second", "Second failure");
    assert_eq!(
        CleanupOutcome::Failed(first.clone()).merge(CleanupOutcome::Failed(second)),
        CleanupOutcome::Failed(first)
    );
}

#[test]
fn merging_a_cleanup_chain_is_order_independent_in_severity() {
    let degraded = swallowtail_core::SafeDiagnostic::new("fixture.degraded", "Cleanup degraded");
    let failed = swallowtail_core::SafeDiagnostic::new("fixture.failed", "Cleanup failed");
    let forward = CleanupOutcome::Clean
        .merge(CleanupOutcome::Degraded(degraded.clone()))
        .merge(CleanupOutcome::Failed(failed.clone()))
        .merge(CleanupOutcome::NotApplicable);
    let reverse = CleanupOutcome::NotApplicable
        .merge(CleanupOutcome::Failed(failed.clone()))
        .merge(CleanupOutcome::Degraded(degraded))
        .merge(CleanupOutcome::Clean);
    assert_eq!(forward, CleanupOutcome::Failed(failed.clone()));
    assert_eq!(reverse, CleanupOutcome::Failed(failed));
}
