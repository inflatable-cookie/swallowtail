use super::{KimiLocalServerPreparationProbe, preparation_failure};
use std::future::{Future, poll_fn};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::Poll;
use swallowtail_runtime::{HostServices, PreparationFailure, PreparationStage, RuntimeFailure};

pub(super) async fn complete_probe_work<T, F>(
    work: F,
    probe: &KimiLocalServerPreparationProbe,
    services: &HostServices,
    cancelled: Option<Arc<AtomicBool>>,
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
            cancel_transport(cancelled.as_ref());
            context.waker().wake_by_ref();
        }
        if terminal.is_none() && deadline.as_mut().poll(context).is_ready() {
            terminal = Some(ProbeTerminal::TimedOut);
            cancel_transport(cancelled.as_ref());
            context.waker().wake_by_ref();
        }
        Poll::Pending
    })
    .await;
    (result, terminal)
}

fn cancel_transport(cancelled: Option<&Arc<AtomicBool>>) {
    if let Some(cancelled) = cancelled {
        cancelled.store(true, Ordering::SeqCst);
    }
}

#[derive(Clone, Copy)]
pub(super) enum ProbeTerminal {
    Cancelled,
    TimedOut,
}

pub(super) fn terminal_failure(terminal: ProbeTerminal) -> PreparationFailure {
    match terminal {
        ProbeTerminal::Cancelled => preparation_failure(
            PreparationStage::BoundedOutput,
            "swallowtail.kimi.local_server.preparation.cancelled",
            "Kimi local-server preparation was cancelled",
        ),
        ProbeTerminal::TimedOut => preparation_failure(
            PreparationStage::BoundedOutput,
            "swallowtail.kimi.local_server.preparation.timed_out",
            "Kimi local-server preparation timed out",
        ),
    }
}
