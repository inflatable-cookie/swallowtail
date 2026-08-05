#[path = "tests/support.rs"]
mod support;

use super::{
    SettledSessionAttachment, SettledSessionAttachmentKind, SettledSessionRestorationFailurePhase,
    SettledSessionRestorationOutcome,
};
use crate::{CleanupOutcome, InterruptedTurnState};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use support::{AttachmentBehavior, ReconciliationBehavior, fixture_sequence, resolve, services};

#[test]
fn only_settled_or_inactive_state_dispatches_attachment() {
    for (state, eligible) in [
        (InterruptedTurnState::Active, false),
        (InterruptedTurnState::WaitingForProviderInput, false),
        (InterruptedTurnState::Completed, true),
        (InterruptedTurnState::Failed, true),
        (InterruptedTurnState::Cancelled, true),
        (InterruptedTurnState::InactiveUnresolved, true),
        (InterruptedTurnState::Unknown, false),
    ] {
        let reconciliation_calls = Arc::new(AtomicUsize::new(0));
        let attachment_calls = Arc::new(AtomicUsize::new(0));
        let order = Arc::new(Mutex::new(Vec::new()));
        let prepared = fixture_sequence(
            ReconciliationBehavior::State(state),
            AttachmentBehavior::Fail,
            SettledSessionAttachmentKind::Load,
            Arc::clone(&reconciliation_calls),
            Arc::clone(&attachment_calls),
            Arc::clone(&order),
        );
        let result = resolve(prepared.restore(services()));

        assert_eq!(reconciliation_calls.load(Ordering::SeqCst), 1);
        assert_eq!(
            attachment_calls.load(Ordering::SeqCst),
            usize::from(eligible)
        );
        if eligible {
            let failure = match result {
                Ok(_) => panic!("eligible fixture attachment must fail"),
                Err(failure) => failure,
            };
            assert_eq!(
                failure.phase(),
                SettledSessionRestorationFailurePhase::Attachment
            );
            assert_eq!(
                failure.reconciliation().expect("observation").state(),
                state
            );
        } else {
            let SettledSessionRestorationOutcome::Observed(observation) =
                result.expect("ineligible state is an observed success")
            else {
                panic!("ineligible state cannot attach");
            };
            assert_eq!(observation.state(), state);
        }
    }
}

#[test]
fn reconciliation_failure_starts_no_attachment() {
    let reconciliation_calls = Arc::new(AtomicUsize::new(0));
    let attachment_calls = Arc::new(AtomicUsize::new(0));
    let order = Arc::new(Mutex::new(Vec::new()));
    let prepared = fixture_sequence(
        ReconciliationBehavior::Fail,
        AttachmentBehavior::Succeed(SettledSessionAttachmentKind::Load),
        SettledSessionAttachmentKind::Load,
        Arc::clone(&reconciliation_calls),
        Arc::clone(&attachment_calls),
        Arc::clone(&order),
    );

    let failure = match resolve(prepared.restore(services())) {
        Ok(_) => panic!("reconciliation must fail"),
        Err(failure) => failure,
    };
    assert_eq!(
        failure.phase(),
        SettledSessionRestorationFailurePhase::Reconciliation
    );
    assert!(failure.reconciliation().is_none());
    assert_eq!(reconciliation_calls.load(Ordering::SeqCst), 1);
    assert_eq!(attachment_calls.load(Ordering::SeqCst), 0);
    assert_eq!(*order.lock().expect("order lock poisoned"), ["reconcile"]);
}

#[test]
fn loaded_and_resumed_success_preserve_order_and_distinct_truth() {
    for kind in [
        SettledSessionAttachmentKind::Load,
        SettledSessionAttachmentKind::Resume,
    ] {
        let reconciliation_calls = Arc::new(AtomicUsize::new(0));
        let attachment_calls = Arc::new(AtomicUsize::new(0));
        let order = Arc::new(Mutex::new(Vec::new()));
        let prepared = fixture_sequence(
            ReconciliationBehavior::State(InterruptedTurnState::InactiveUnresolved),
            AttachmentBehavior::Succeed(kind),
            kind,
            reconciliation_calls,
            attachment_calls,
            Arc::clone(&order),
        );
        assert_eq!(prepared.attachment_kind(), kind);

        let SettledSessionRestorationOutcome::Attached(attached) =
            resolve(prepared.restore(services())).expect("attachment succeeds")
        else {
            panic!("eligible state attaches");
        };
        assert_eq!(
            attached.reconciliation().state(),
            InterruptedTurnState::InactiveUnresolved
        );
        assert_eq!(attached.attachment().kind(), kind);
        assert_eq!(
            *order.lock().expect("order lock poisoned"),
            ["reconcile", "attach"]
        );
        let (_, attachment) = attached.into_parts();
        let session = match attachment {
            SettledSessionAttachment::Loaded(loaded) => {
                assert_eq!(loaded.replay().len(), 0);
                loaded.into_parts().1
            }
            SettledSessionAttachment::Resumed(session) => session,
        };
        assert_eq!(resolve(session.close()), CleanupOutcome::Clean);
    }
}

#[test]
fn attachment_kind_mismatch_preserves_reconciliation_and_returns_no_handle() {
    let prepared = fixture_sequence(
        ReconciliationBehavior::State(InterruptedTurnState::InactiveUnresolved),
        AttachmentBehavior::Succeed(SettledSessionAttachmentKind::Resume),
        SettledSessionAttachmentKind::Load,
        Arc::new(AtomicUsize::new(0)),
        Arc::new(AtomicUsize::new(0)),
        Arc::new(Mutex::new(Vec::new())),
    );

    let failure = match resolve(prepared.restore(services())) {
        Ok(_) => panic!("kind mismatch must fail"),
        Err(failure) => failure,
    };
    assert_eq!(
        failure.phase(),
        SettledSessionRestorationFailurePhase::Attachment
    );
    assert_eq!(
        failure.reconciliation().expect("observation").state(),
        InterruptedTurnState::InactiveUnresolved
    );
    assert_eq!(
        failure.failure().diagnostic().code(),
        "swallowtail.settled_session_restoration.attachment_kind_mismatch"
    );
}
