use std::future::Future;
use std::task::Poll;
use swallowtail_runtime::{
    BoxFuture, Deadline, DeadlineObservation, HostServices, ProviderSessionOperationFailure,
    ProviderSessionOperationFailureStage,
};

use super::operation_failure;

pub(super) enum Controlled<T> {
    Completed(T),
    Cancelled,
    Deadline,
}

pub(super) async fn wait_controlled<F, T>(
    operation: F,
    cancellation: &swallowtail_runtime::ImmediateCancellation,
    deadline: Option<BoxFuture<'static, DeadlineObservation>>,
) -> Controlled<T>
where
    F: Future<Output = T>,
{
    let mut operation = Box::pin(operation);
    let mut cancelled = cancellation.wait_requested();
    let mut deadline = deadline;
    std::future::poll_fn(|context| {
        if let Poll::Ready(result) = operation.as_mut().poll(context) {
            return Poll::Ready(Controlled::Completed(result));
        }
        if cancelled.as_mut().poll(context).is_ready() {
            return Poll::Ready(Controlled::Cancelled);
        }
        if deadline
            .as_mut()
            .is_some_and(|deadline| deadline.as_mut().poll(context).is_ready())
        {
            return Poll::Ready(Controlled::Deadline);
        }
        Poll::Pending
    })
    .await
}

pub(super) fn deadline_wait(
    deadline: Option<Deadline>,
    services: &HostServices,
) -> Result<Option<BoxFuture<'static, DeadlineObservation>>, ProviderSessionOperationFailure> {
    deadline
        .map(|deadline| {
            services
                .time()
                .ok_or_else(|| {
                    operation_failure(
                        ProviderSessionOperationFailureStage::BeforeDispatch,
                        "swallowtail.kimi.provider_session.time_service_missing",
                        "Deadline-bound Kimi provider-session operation requires a time service",
                    )
                })
                .map(|time| time.wait_until(deadline))
        })
        .transpose()
}
