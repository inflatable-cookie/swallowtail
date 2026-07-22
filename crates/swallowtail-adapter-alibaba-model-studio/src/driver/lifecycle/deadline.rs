use std::future::{Future, poll_fn};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::Poll;
use swallowtail_runtime::{Deadline, HostServices, RuntimeFailure};

pub(in crate::driver) async fn complete_before_deadline<T, F>(
    work: F,
    deadline: Option<Deadline>,
    services: &HostServices,
    cancelled: Arc<AtomicBool>,
) -> Result<T, RuntimeFailure>
where
    F: Future<Output = Result<T, RuntimeFailure>>,
{
    let Some(deadline) = deadline else {
        return work.await;
    };
    let time = services.time().expect("validated time");
    if time.now() >= deadline.instant() {
        return Err(crate::failure::failure(
            "swallowtail.alibaba_model_studio.deadline_elapsed",
            "Alibaba Model Studio deadline elapsed before provider work",
        ));
    }
    let mut work = Box::pin(work);
    let mut wait = time.wait_until(deadline);
    let mut timed_out = false;
    let result = poll_fn(|context| {
        if let Poll::Ready(result) = work.as_mut().poll(context) {
            return Poll::Ready(result);
        }
        if !timed_out && wait.as_mut().poll(context).is_ready() {
            timed_out = true;
            cancelled.store(true, Ordering::SeqCst);
            context.waker().wake_by_ref();
        }
        Poll::Pending
    })
    .await;
    if timed_out {
        Err(crate::failure::failure(
            "swallowtail.alibaba_model_studio.timed_out",
            "Alibaba Model Studio operation timed out",
        ))
    } else {
        result
    }
}
