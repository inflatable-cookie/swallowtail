async fn complete_before_deadline<T, F>(
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
    let time = services.time().ok_or_else(|| missing("time"))?;
    if time.now() >= deadline.instant() {
        return Err(failure(
            "swallowtail.ollama.deadline_elapsed",
            "Ollama deadline elapsed before provider work",
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
        Err(failure(
            "swallowtail.ollama.timed_out",
            "Ollama operation timed out",
        ))
    } else {
        result
    }
}

enum RunSignal {
    Item(Result<crate::protocol::NativeEvent, RuntimeFailure>),
    Closed,
    Deadline,
}

async fn next_run_signal(
    subscription: &mut Subscription,
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
