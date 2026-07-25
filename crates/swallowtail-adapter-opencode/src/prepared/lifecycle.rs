use super::OpenCodePreparationProbe;
use std::future::Future;
use std::future::poll_fn;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::Poll;
use swallowtail_runtime::{HostServices, RuntimeFailure};

pub(super) async fn complete_probe_work<T, F>(
    work: F,
    probe: &OpenCodePreparationProbe,
    services: &HostServices,
    cancelled: Arc<AtomicBool>,
) -> Result<T, RuntimeFailure>
where
    F: Future<Output = Result<T, RuntimeFailure>>,
{
    let (result, terminal) = complete_probe_work_outcome(work, probe, services, cancelled).await;
    match terminal {
        Some(terminal) => Err(terminal_failure(terminal)),
        None => result,
    }
}

pub(super) async fn complete_probe_work_outcome<T, F>(
    work: F,
    probe: &OpenCodePreparationProbe,
    services: &HostServices,
    cancelled: Arc<AtomicBool>,
) -> (Result<T, RuntimeFailure>, Option<ProbeTerminal>)
where
    F: Future<Output = Result<T, RuntimeFailure>>,
{
    let mut work = Box::pin(work);
    let mut deadline = services
        .time()
        .expect("validated time service")
        .wait_until(probe.deadline);
    let mut cancellation = probe.cancellation.wait_requested();
    let mut terminal = None;
    let result = poll_fn(|context| {
        if let Poll::Ready(result) = work.as_mut().poll(context) {
            return Poll::Ready(result);
        }
        if terminal.is_none() && cancellation.as_mut().poll(context).is_ready() {
            terminal = Some(ProbeTerminal::Cancelled);
            cancelled.store(true, Ordering::SeqCst);
            context.waker().wake_by_ref();
        }
        if terminal.is_none() && deadline.as_mut().poll(context).is_ready() {
            terminal = Some(ProbeTerminal::TimedOut);
            cancelled.store(true, Ordering::SeqCst);
            context.waker().wake_by_ref();
        }
        Poll::Pending
    })
    .await;
    (result, terminal)
}

pub(super) fn terminal_failure(terminal: ProbeTerminal) -> RuntimeFailure {
    match terminal {
        ProbeTerminal::Cancelled => crate::failure::failure(
            "swallowtail.opencode.preparation.cancelled",
            "OpenCode preparation was cancelled",
        ),
        ProbeTerminal::TimedOut => crate::failure::failure(
            "swallowtail.opencode.preparation.timed_out",
            "OpenCode preparation timed out",
        ),
    }
}

#[derive(Clone, Copy)]
pub(super) enum ProbeTerminal {
    Cancelled,
    TimedOut,
}
