use super::settle_deadline_and_attachment;
use crate::sidecar::driver::input::SharedAttachmentMaterialization;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{BoxFuture, CleanupOutcome, JoinedTask, RuntimeFailure};

/// Deadline task whose join reports a failure.
struct FailingJoin;

impl JoinedTask for FailingJoin {
    fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async {
            Err(RuntimeFailure::new(SafeDiagnostic::new(
                "fixture.join_failed",
                "fixture deadline task failed to join",
            )))
        })
    }
}

#[test]
fn a_sidecar_deadline_task_that_fails_to_join_is_reported_at_session_close() {
    let cleanup = futures_executor::block_on(settle_deadline_and_attachment(
        Some(Box::new(FailingJoin)),
        SharedAttachmentMaterialization::default(),
    ));

    assert!(
        matches!(&cleanup, CleanupOutcome::Failed(diagnostic) if diagnostic.message().contains("deadline_join_failed")),
        "a deadline task that does not join is a cleanup failure, got {cleanup:?}"
    );
}

#[test]
fn a_sidecar_session_without_a_deadline_task_settles_clean() {
    assert_eq!(
        futures_executor::block_on(settle_deadline_and_attachment(
            None,
            SharedAttachmentMaterialization::default(),
        )),
        CleanupOutcome::NotApplicable
    );
}
