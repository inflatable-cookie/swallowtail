enum TurnSignal {
    Data(Vec<u8>),
    Failure(RuntimeFailure),
    Closed,
    Deadline,
}

async fn next_signal(
    subscription: &mut Subscription,
    deadline: &mut Option<BoxFuture<'static, DeadlineObservation>>,
) -> TurnSignal {
    poll_fn(|context| {
        if let Poll::Ready(item) = subscription.poll_next(context) {
            return Poll::Ready(match item {
                Some(Ok(data)) => TurnSignal::Data(data),
                Some(Err(error)) => TurnSignal::Failure(error),
                None => TurnSignal::Closed,
            });
        }
        if let Some(deadline) = deadline
            && deadline.as_mut().poll(context).is_ready()
        {
            return Poll::Ready(TurnSignal::Deadline);
        }
        Poll::Pending
    })
    .await
}

fn validate_open(
    plan: &PreflightPlan,
    request: &OpenSessionRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    require_services(services, true)?;
    validate_session_access_plan(plan, request.access_policy())?;
    if request.access_policy()
        != &SessionAccessPolicy::ambient_harness(swallowtail_core::ResourceAccess::Read)
    {
        return Err(unsupported("non-ambient read session access"));
    }
    if request.working_resource().is_none() {
        return Err(unsupported("a resource-free session"));
    }
    if !request.options().is_empty() {
        return Err(unsupported("non-default session options"));
    }
    validate_deadline(request.deadline(), services)
}

fn validate_turn(request: &TurnRequest, services: &HostServices) -> Result<(), RuntimeFailure> {
    if request.attachments().len() != 0 {
        return Err(unsupported("turn attachments"));
    }
    if request.structured_output().is_some() {
        return Err(unsupported("structured turn output"));
    }
    validate_deadline(request.deadline(), services)
}

fn validate_deadline(
    deadline: Option<Deadline>,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    if let Some(deadline) = deadline {
        let time = services.time().ok_or_else(|| {
            failure(
                "swallowtail.opencode.time_service_missing",
                "OpenCode deadline requires a time service",
            )
        })?;
        if time.now() >= deadline.instant() {
            return Err(failure(
                "swallowtail.opencode.deadline_elapsed",
                "OpenCode deadline elapsed before provider work",
            ));
        }
    }
    Ok(())
}

fn require_services(services: &HostServices, session: bool) -> Result<(), RuntimeFailure> {
    if services.blocking_work().is_none()
        || services.time().is_none()
        || services.network().is_none()
        || services.credential().is_none()
        || (session && (services.task().is_none() || services.working_resource().is_none()))
    {
        Err(failure(
            "swallowtail.opencode.host_services_missing",
            "OpenCode HTTP required host services are unavailable",
        ))
    } else {
        Ok(())
    }
}

async fn complete_before_deadline<T, F>(
    work: F,
    deadline: Option<Deadline>,
    services: &HostServices,
    cancelled: Arc<AtomicBool>,
    timeout_code: &'static str,
    timeout_message: &'static str,
) -> Result<T, RuntimeFailure>
where
    F: Future<Output = Result<T, RuntimeFailure>>,
{
    let Some(deadline) = deadline else {
        return work.await;
    };
    let time = services.time().ok_or_else(|| {
        failure(
            "swallowtail.opencode.time_service_missing",
            "OpenCode deadline requires a time service",
        )
    })?;
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
        Err(failure(timeout_code, timeout_message))
    } else {
        result
    }
}

async fn reap_finished(active: &ActiveSlot) -> Result<(), RuntimeFailure> {
    let task = {
        let mut state = active.lock().expect("active turn lock poisoned");
        if state
            .as_ref()
            .is_some_and(|turn| turn.terminal.load(Ordering::SeqCst))
        {
            state.as_mut().and_then(|turn| turn.task.take())
        } else {
            None
        }
    };
    if let Some(task) = task {
        task.join().await?;
        *active.lock().expect("active turn lock poisoned") = None;
    }
    Ok(())
}

async fn join_active(active: &ActiveSlot) -> CleanupOutcome {
    let task = active
        .lock()
        .expect("active turn lock poisoned")
        .as_mut()
        .and_then(|turn| turn.task.take());
    let cleanup = match task {
        Some(task) => cleanup_from_result(task.join().await),
        None => CleanupOutcome::NotApplicable,
    };
    *active.lock().expect("active turn lock poisoned") = None;
    cleanup
}

async fn close_active(active: &ActiveSlot) -> CleanupOutcome {
    let cancellation = active
        .lock()
        .expect("active turn lock poisoned")
        .as_ref()
        .filter(|turn| !turn.terminal.load(Ordering::SeqCst))
        .map(|turn| Arc::clone(&turn.cancellation));
    let cancel_cleanup = match cancellation {
        Some(cancellation) => cleanup_from_result(cancellation.request().await.map(|_| ())),
        None => CleanupOutcome::NotApplicable,
    };
    merge_cleanup(cancel_cleanup, join_active(active).await)
}

fn cleanup_from_result(result: Result<(), RuntimeFailure>) -> CleanupOutcome {
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

fn scope(kind: &str, id: &str) -> Result<ScopeId, RuntimeFailure> {
    ScopeId::new(format!("opencode-http:{kind}:{id}")).map_err(|_| {
        failure(
            "swallowtail.opencode.scope_invalid",
            "OpenCode operation scope was invalid",
        )
    })
}
