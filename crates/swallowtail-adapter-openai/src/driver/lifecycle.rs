use std::future::poll_fn;
use std::task::Poll;
use swallowtail_runtime::DeadlineObservation;

enum RunSignal {
    Item(Result<crate::transport::StreamItem, RuntimeFailure>),
    Closed,
    Deadline,
}

async fn next_run_signal(
    subscription: &mut crate::transport::Subscription,
    deadline: &mut Option<BoxFuture<'static, DeadlineObservation>>,
) -> RunSignal {
    poll_fn(|context| {
        if let Poll::Ready(item) = subscription.poll_next(context) {
            return Poll::Ready(match item {
                Some(item) => RunSignal::Item(item),
                None => RunSignal::Closed,
            });
        }
        if let Some(deadline) = deadline
            && deadline.as_mut().poll(context).is_ready()
        {
            return Poll::Ready(RunSignal::Deadline);
        }
        Poll::Pending
    })
    .await
}

fn cleanup_result(result: Result<(), RuntimeFailure>) -> CleanupOutcome {
    match result {
        Ok(()) => CleanupOutcome::Clean,
        Err(error) => CleanupOutcome::Failed(error.diagnostic().clone()),
    }
}

fn merge_cleanup(current: CleanupOutcome, next: CleanupOutcome) -> CleanupOutcome {
    match (&current, &next) {
        (CleanupOutcome::Failed(_), _) => current,
        (_, CleanupOutcome::Failed(_)) => next,
        (CleanupOutcome::Degraded(_), _) => current,
        (_, CleanupOutcome::Degraded(_)) => next,
        (CleanupOutcome::Clean, _) => current,
        (CleanupOutcome::NotApplicable, CleanupOutcome::Clean) => next,
        _ => current,
    }
}

fn provider_status(error: RuntimeFailure) -> swallowtail_runtime::TerminalStatus {
    swallowtail_runtime::TerminalStatus::ProviderFailed(error.diagnostic().clone())
}
