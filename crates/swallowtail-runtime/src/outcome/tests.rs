use super::{
    CleanupOutcome, ProviderCancellationOutcome, ProviderRequestObservation,
    RemoteResourceDeletionOutcome, TerminalOutcome, TerminalStatus, terminal_outcome_channel,
};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

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
fn terminal_failure_dimensions_remain_distinct() {
    let diagnostic = swallowtail_core::SafeDiagnostic::new("fixture.failure", "Failed");
    let statuses = [
        TerminalStatus::Completed,
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
