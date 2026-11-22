use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use swallowtail_runtime::{
    BoxFuture, Deadline, DeadlineObservation, MonotonicInstant, ProcessExit, ProcessHandle,
    ProcessInputChunk, ProcessOutputChunk, RuntimeFailure, TimeService,
};

/// A process that accepts the abort write and never sends an acknowledgement.
#[derive(Default)]
pub(crate) struct SilentAbort {
    pub(crate) writes: AtomicUsize,
    pub(crate) force_stopped: AtomicBool,
}

impl ProcessHandle for SilentAbort {
    fn write_stdin(&self, _: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.writes.fetch_add(1, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(std::future::pending())
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.force_stopped.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
    }
}

pub(crate) struct AbortGraceExpires;

impl TimeService for AbortGraceExpires {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(10)
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        assert_eq!(
            deadline.instant().ticks(),
            510,
            "close gets a fresh 500-tick grace"
        );
        Box::pin(async move { DeadlineObservation::new(deadline, deadline.instant()) })
    }
}
