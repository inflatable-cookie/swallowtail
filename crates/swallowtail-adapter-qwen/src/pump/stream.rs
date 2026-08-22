use crate::handle::QwenProcessCancellation;
use std::future::poll_fn;
use std::task::Poll;
use swallowtail_runtime::{
    BoxFuture, DeadlineObservation, ProcessHandle, ProcessOutputChunk, RuntimeEventSender,
    RuntimeFailure,
};

pub(super) enum NextOutput {
    Process(Result<Option<ProcessOutputChunk>, RuntimeFailure>),
    Deadline,
}

pub(super) async fn next_output(
    process: &dyn ProcessHandle,
    cancellation: &QwenProcessCancellation,
    deadline: &mut Option<BoxFuture<'static, DeadlineObservation>>,
) -> NextOutput {
    let mut read = process.read_output();
    poll_fn(|context| {
        if !cancellation.is_requested()
            && let Some(wait) = deadline.as_mut()
            && wait.as_mut().poll(context).is_ready()
        {
            return Poll::Ready(NextOutput::Deadline);
        }
        read.as_mut().poll(context).map(NextOutput::Process)
    })
    .await
}

pub(super) fn send_all(
    sender: &RuntimeEventSender,
    events: impl IntoIterator<Item = swallowtail_runtime::RuntimeEvent>,
) -> Result<(), RuntimeFailure> {
    for event in events {
        sender.send(event)?;
    }
    Ok(())
}
