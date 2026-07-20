use crate::output::failure;
use std::future::Future;
use std::pin::Pin;
use std::process::{Child, ExitStatus};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::Duration;
use swallowtail_runtime::{ProcessExit, RuntimeFailure};

pub(crate) enum ChildCommand {
    ForceStop,
}

#[derive(Default)]
struct ExitInner {
    result: Option<Result<ProcessExit, RuntimeFailure>>,
    waiters: Vec<Waker>,
}

#[derive(Default)]
pub(crate) struct ExitState {
    inner: Mutex<ExitInner>,
}

impl ExitState {
    pub(crate) fn complete(&self, result: Result<ProcessExit, RuntimeFailure>) {
        let mut inner = self.inner.lock().expect("local process exit lock poisoned");
        if inner.result.is_none() {
            inner.result = Some(result);
        }
        for waiter in inner.waiters.drain(..) {
            waiter.wake();
        }
    }

    pub(crate) fn future(self: &Arc<Self>) -> ExitFuture {
        ExitFuture {
            state: Arc::clone(self),
        }
    }

    pub(crate) fn is_complete(&self) -> bool {
        self.inner
            .lock()
            .expect("local process exit lock poisoned")
            .result
            .is_some()
    }
}

pub(crate) struct ExitFuture {
    state: Arc<ExitState>,
}

impl Future for ExitFuture {
    type Output = Result<ProcessExit, RuntimeFailure>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let mut inner = self
            .state
            .inner
            .lock()
            .expect("local process exit lock poisoned");
        if let Some(result) = &inner.result {
            Poll::Ready(result.clone())
        } else {
            if !inner
                .waiters
                .iter()
                .any(|waiter| waiter.will_wake(context.waker()))
            {
                inner.waiters.push(context.waker().clone());
            }
            Poll::Pending
        }
    }
}

pub(crate) fn supervise_child(
    child: &mut Child,
    commands: Receiver<ChildCommand>,
    stdout_reader: thread::JoinHandle<()>,
    stderr_reader: thread::JoinHandle<()>,
    exit: &ExitState,
) {
    let status = loop {
        if matches!(
            commands.recv_timeout(Duration::from_millis(10)),
            Ok(ChildCommand::ForceStop)
        ) && child.kill().is_err()
        {
            break Err(failure(
                "swallowtail.local_process.force_stop_failed",
                "Local process could not be force-stopped",
            ));
        }
        match child.try_wait() {
            Ok(Some(status)) => break Ok(status),
            Ok(None) => {}
            Err(_) => {
                break Err(failure(
                    "swallowtail.local_process.wait_failed",
                    "Local process exit could not be observed",
                ));
            }
        }
    };

    let reader_result = stdout_reader.join().and_then(|()| stderr_reader.join());
    if reader_result.is_err() {
        exit.complete(Err(failure(
            "swallowtail.local_process.reader_panicked",
            "Local process output supervision failed",
        )));
        return;
    }
    exit.complete(status.map(exit_record));
}

fn exit_record(status: ExitStatus) -> ProcessExit {
    ProcessExit::new(status.success(), status.code())
}
