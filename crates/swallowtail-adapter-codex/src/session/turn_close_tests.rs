use super::{CodexTurnHandle, TurnCancellation};
use crate::rpc::RpcConnection;
use crate::turn_state::ActiveTurn;
use std::collections::BTreeSet;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use swallowtail_core::{ExecutionHostId, ProviderRequestPolicy, SafeDiagnostic, TurnRef};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, HostServices, JoinedTask, ProcessExit, ProcessHandle,
    ProcessInputChunk, ProcessOutputChunk, RuntimeFailure, RuntimeTurnId, TurnHandle,
};

/// Child whose stdin refuses writes, so the `turn/interrupt` request cannot be sent.
struct RefusingStdin;

impl ProcessHandle for RefusingStdin {
    fn write_stdin(&self, _chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async {
            Err(RuntimeFailure::new(SafeDiagnostic::new(
                "test.stdin_refused",
                "stdin refused",
            )))
        })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(async { Ok(None) })
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
    }
}

/// Stands in for the turn deadline watcher: it finishes only when the turn
/// finishes, like the production `select(wait, finished)` task. It gives up
/// after a bounded number of polls so a regression fails instead of hanging.
struct WatchTurnFinish {
    turn: Arc<ActiveTurn>,
}

impl JoinedTask for WatchTurnFinish {
    fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async move {
            let mut finished = Box::pin(self.turn.finished_future());
            let mut polls = 0usize;
            std::future::poll_fn(move |context| {
                if std::future::Future::poll(finished.as_mut(), context).is_ready() {
                    return std::task::Poll::Ready(Ok(()));
                }
                polls += 1;
                if polls > 16 {
                    return std::task::Poll::Ready(Err(RuntimeFailure::new(SafeDiagnostic::new(
                        "test.deadline_watch_hung",
                        "deadline watcher waited past the interrupt failure",
                    ))));
                }
                context.waker().wake_by_ref();
                std::task::Poll::Pending
            })
            .await
        })
    }
}

#[test]
fn a_refused_interrupt_releases_the_deadline_watcher_and_reports_cleanup_failure() {
    let services =
        HostServices::new(ExecutionHostId::new("fixture.host.local").expect("host id is valid"));
    let connection = RpcConnection::new(Arc::new(RefusingStdin), services);
    let (turn, events, _callbacks, _terminal) = ActiveTurn::new(
        RuntimeTurnId::new("turn-close").expect("turn id is valid"),
        None,
        BTreeSet::new(),
        None,
        ProviderRequestPolicy::reject_all(),
        "thread-close".to_owned(),
        Arc::downgrade(&connection),
    )
    .expect("turn state is valid");
    let handle = CodexTurnHandle {
        runtime_id: RuntimeTurnId::new("turn-close").expect("turn id is valid"),
        provider_ref: TurnRef::new("turn-close-provider").expect("provider turn ref is valid"),
        events: Some(events),
        callbacks: None,
        terminal: None,
        cancellation: TurnCancellation {
            connection: Arc::clone(&connection),
            thread_id: "thread-close".to_owned(),
            turn_id: "turn-close-provider".to_owned(),
            turn: Arc::clone(&turn),
            requested: AtomicBool::new(false),
        },
        deadline_task: Some(Box::new(WatchTurnFinish {
            turn: Arc::clone(&turn),
        })),
        registered: None,
    };

    let handle: Box<dyn TurnHandle> = Box::new(handle);
    let cleanup = futures_executor::block_on(handle.close());

    assert!(
        matches!(cleanup, CleanupOutcome::Failed(ref diagnostic) if diagnostic.code() == "test.stdin_refused"),
        "a refused interrupt is reported as cleanup failure, got {cleanup:?}"
    );
    assert!(
        turn.is_finished(),
        "the turn is finished so the watcher exits"
    );
}
