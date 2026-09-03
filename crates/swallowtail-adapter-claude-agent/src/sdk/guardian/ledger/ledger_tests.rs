//! Barrier tests for the two interleavings the guard has to survive.
//!
//! Both are driven directly against the state machine, so the ordering is the
//! test's choice rather than the scheduler's.

use super::{GuardLedger, RecordingLease};
use futures_executor::block_on;
use std::future::Future;
use std::pin::pin;
use std::sync::Arc;
use std::task::{Context, Waker};
use swallowtail_runtime::{BoxFuture, JoinedTask, RuntimeFailure};

struct StubTask;

impl JoinedTask for StubTask {
    fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }
}

fn cleanup_is_waiting(ledger: &Arc<GuardLedger>) -> bool {
    let mut take = pin!(ledger.take_for_cleanup());
    let waker = Waker::noop();
    take.as_mut()
        .poll(&mut Context::from_waker(waker))
        .is_pending()
}

#[test]
fn a_recording_that_lands_after_cleanup_started_is_still_released() {
    // Interleaving one: the deadline task wins the transition while the open
    // future is mid-acquisition, and an acquisition completes afterwards.
    let (ledger, lease): (Arc<GuardLedger>, RecordingLease) = GuardLedger::new();
    assert!(ledger.begin_cleanup(), "cleanup wins an unclaimed guard");

    // Cleanup cannot take the ledger while the open future can still record.
    assert!(
        cleanup_is_waiting(&ledger),
        "cleanup must wait for recording to end"
    );

    ledger.record_pump(Box::new(StubTask));
    drop(lease);

    let acquired = block_on(ledger.take_for_cleanup());
    assert!(
        acquired.pump.is_some(),
        "an acquisition recorded after cleanup started must not be orphaned"
    );
}

#[test]
fn open_cannot_report_success_once_cleanup_won() {
    // Interleaving two: readiness lands on the boundary and the bound returns
    // the operation, while the deadline task already owns cleanup.
    let (ledger, lease) = GuardLedger::new();
    ledger.record_pump(Box::new(StubTask));
    assert!(ledger.begin_cleanup(), "cleanup wins an unclaimed guard");
    drop(lease);

    assert!(
        ledger.claim().is_none(),
        "open must not take ownership of what cleanup is terminating"
    );
    let acquired = block_on(ledger.take_for_cleanup());
    assert!(
        acquired.pump.is_some(),
        "cleanup keeps what open was refused"
    );
}

#[test]
fn a_claim_that_wins_leaves_cleanup_with_nothing_to_do() {
    let (ledger, lease) = GuardLedger::new();
    ledger.record_pump(Box::new(StubTask));
    drop(lease);

    let acquired = ledger.claim().expect("an unclaimed guard yields ownership");
    assert!(acquired.pump.is_some());
    assert!(
        !ledger.begin_cleanup(),
        "cleanup must not run after open claimed"
    );
    assert!(
        block_on(ledger.take_for_cleanup()).pump.is_none(),
        "a claimed ledger holds nothing"
    );
}
