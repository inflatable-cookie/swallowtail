use super::*;
use crate::failure::failure;
use std::future::Future;
use std::task::Poll;
use swallowtail_runtime::{ImmediateCancellation, ProviderSessionManagementAgreement};

pub(super) fn cancelled_or_expired(
    agreement: &ProviderSessionManagementAgreement,
    cancellation: &ImmediateCancellation,
    services: &HostServices,
) -> Result<bool, RuntimeFailure> {
    if cancellation.is_requested() {
        return Ok(true);
    }
    let Some(deadline) = agreement.deadline() else {
        return Ok(false);
    };
    let time = services.time().ok_or_else(|| {
        failure(
            "swallowtail.claude_agent.lifecycle.time_service_missing",
            "Deadline-bound Claude Agent deletion requires a time service",
        )
    })?;
    Ok(time.now() >= deadline.instant())
}

pub(super) fn deadline_wait(
    agreement: &ProviderSessionManagementAgreement,
    services: &HostServices,
) -> Result<Option<BoxFuture<'static, swallowtail_runtime::DeadlineObservation>>, RuntimeFailure> {
    agreement
        .deadline()
        .map(|deadline| {
            services
                .time()
                .ok_or_else(|| {
                    failure(
                        "swallowtail.claude_agent.lifecycle.time_service_missing",
                        "Deadline-bound Claude Agent deletion requires a time service",
                    )
                })
                .map(|time| time.wait_until(deadline))
        })
        .transpose()
}

pub(super) enum Controlled<T> {
    Completed(T),
    Cancelled,
    Deadline,
}

pub(super) async fn wait_controlled<F, T>(
    operation: F,
    cancellation: &ImmediateCancellation,
    deadline: Option<BoxFuture<'static, swallowtail_runtime::DeadlineObservation>>,
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
