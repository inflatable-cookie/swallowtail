use super::OllamaPreparationProbe;
use std::future::Future;
use std::future::poll_fn;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::Poll;
use swallowtail_runtime::{HostServices, RuntimeFailure};

pub(super) async fn complete_probe_work<T, F>(
    work: F,
    probe: &OllamaPreparationProbe,
    services: &HostServices,
    cancelled: Arc<AtomicBool>,
) -> Result<T, RuntimeFailure>
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
    match terminal {
        Some(ProbeTerminal::Cancelled) => Err(crate::failure::failure(
            "swallowtail.ollama.preparation.cancelled",
            "Ollama preparation was cancelled",
        )),
        Some(ProbeTerminal::TimedOut) => Err(crate::failure::failure(
            "swallowtail.ollama.preparation.timed_out",
            "Ollama preparation timed out",
        )),
        None => result,
    }
}

#[derive(Clone, Copy)]
enum ProbeTerminal {
    Cancelled,
    TimedOut,
}
