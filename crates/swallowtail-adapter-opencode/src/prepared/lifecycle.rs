use super::OpenCodePreparationProbe;
use std::future::Future;
use std::future::poll_fn;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::Poll;
use swallowtail_runtime::{HostServices, RuntimeFailure};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ProbeInterrupt {
    /// Return immediately on deadline or cancellation, dropping the stalled future.
    Abandon,
    /// Set the cancellation flag and keep polling the work to completion.
    Drain,
}

pub(super) async fn complete_probe_work<T, F>(
    work: F,
    probe: &OpenCodePreparationProbe,
    services: &HostServices,
    cancelled: Arc<AtomicBool>,
    on_interrupt: ProbeInterrupt,
) -> Result<T, RuntimeFailure>
where
    F: Future<Output = Result<T, RuntimeFailure>>,
{
    let (result, terminal) =
        complete_probe_work_outcome(work, probe, services, cancelled, on_interrupt).await;
    match terminal {
        Some(terminal) => Err(terminal_failure(terminal)),
        None => result.expect("uninterrupted probe work yields a result"),
    }
}

pub(super) async fn complete_probe_work_outcome<T, F>(
    work: F,
    probe: &OpenCodePreparationProbe,
    services: &HostServices,
    cancelled: Arc<AtomicBool>,
    on_interrupt: ProbeInterrupt,
) -> (Option<Result<T, RuntimeFailure>>, Option<ProbeTerminal>)
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
            return Poll::Ready(Some(result));
        }
        if terminal.is_none() {
            let fired = if cancellation.as_mut().poll(context).is_ready() {
                Some(ProbeTerminal::Cancelled)
            } else if deadline.as_mut().poll(context).is_ready() {
                Some(ProbeTerminal::TimedOut)
            } else {
                None
            };
            if let Some(kind) = fired {
                terminal = Some(kind);
                cancelled.store(true, Ordering::SeqCst);
                match on_interrupt {
                    ProbeInterrupt::Abandon => return Poll::Ready(None),
                    ProbeInterrupt::Drain => context.waker().wake_by_ref(),
                }
            }
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
