use crate::failure::failure;
use std::future::{Future, poll_fn};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::Poll;
use swallowtail_runtime::{
    HostServices, ImmediateCancellation, ProviderSessionManagementAgreement, RuntimeFailure,
};

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
    let time = services.time().ok_or_else(time_failure)?;
    Ok(time.now() >= deadline.instant())
}

pub(super) async fn wait_before_dispatch<T, F>(
    work: F,
    agreement: &ProviderSessionManagementAgreement,
    cancellation: &ImmediateCancellation,
    services: &HostServices,
) -> Result<(Result<T, RuntimeFailure>, bool), RuntimeFailure>
where
    F: Future<Output = Result<T, RuntimeFailure>>,
{
    wait_controlled_joined(work, agreement, cancellation, services, None).await
}

pub(super) async fn wait_after_dispatch<T, F>(
    work: F,
    agreement: &ProviderSessionManagementAgreement,
    cancellation: &ImmediateCancellation,
    services: &HostServices,
    transport_cancelled: Arc<AtomicBool>,
) -> Result<(Result<T, RuntimeFailure>, bool), RuntimeFailure>
where
    F: Future<Output = Result<T, RuntimeFailure>>,
{
    wait_controlled_joined(
        work,
        agreement,
        cancellation,
        services,
        Some(transport_cancelled),
    )
    .await
}

async fn wait_controlled_joined<T, F>(
    work: F,
    agreement: &ProviderSessionManagementAgreement,
    cancellation: &ImmediateCancellation,
    services: &HostServices,
    transport_cancelled: Option<Arc<AtomicBool>>,
) -> Result<(Result<T, RuntimeFailure>, bool), RuntimeFailure>
where
    F: Future<Output = Result<T, RuntimeFailure>>,
{
    let mut work = Box::pin(work);
    let mut cancellation_wait = cancellation.wait_requested();
    let mut deadline = agreement
        .deadline()
        .map(|deadline| {
            services
                .time()
                .ok_or_else(time_failure)
                .map(|time| time.wait_until(deadline))
        })
        .transpose()?;
    let mut interrupted = false;
    let result = poll_fn(|context| {
        if let Poll::Ready(result) = work.as_mut().poll(context) {
            return Poll::Ready(result);
        }
        if !interrupted && cancellation_wait.as_mut().poll(context).is_ready() {
            interrupted = true;
            cancel_transport(transport_cancelled.as_ref());
            context.waker().wake_by_ref();
        }
        if !interrupted
            && deadline
                .as_mut()
                .is_some_and(|deadline| deadline.as_mut().poll(context).is_ready())
        {
            interrupted = true;
            cancel_transport(transport_cancelled.as_ref());
            context.waker().wake_by_ref();
        }
        Poll::Pending
    })
    .await;
    Ok((result, interrupted))
}

fn cancel_transport(cancelled: Option<&Arc<AtomicBool>>) {
    if let Some(cancelled) = cancelled {
        cancelled.store(true, Ordering::SeqCst);
    }
}

fn time_failure() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.time_service_missing",
        "Deadline-bound Kimi local-server work requires a time service",
    )
}
