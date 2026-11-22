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
    // Abandon the in-flight request at the deadline. The abort flag stops the
    // blocking work, and completed work still wins a same-poll deadline.
    let race = race_controls(
        work,
        None,
        Some(time.wait_until(deadline)),
        &cancelled,
        OnInterrupt::Abandon,
    )
    .await;
    match race.value {
        Some(result) => result,
        None => Err(failure(timeout_code, timeout_message)),
    }
}

async fn reap_finished(active: &ActiveSlot) -> Result<(), RuntimeFailure> {
    let finished = {
        let mut state = active.lock().expect("active turn lock poisoned");
        if state
            .as_ref()
            .is_some_and(|turn| turn.terminal.load(Ordering::SeqCst))
        {
            state.take()
        } else {
            None
        }
    };
    if let Some(mut turn) = finished {
        if let Some(task) = turn.task.take() {
            task.join().await?;
        }
        if matches!(turn.attachment.release().await, CleanupOutcome::Failed(_)) {
            return Err(failure(
                "swallowtail.opencode.attachment_cleanup_failed",
                "OpenCode attachment cleanup failed",
            ));
        }
    }
    Ok(())
}

async fn join_active(active: &ActiveSlot) -> CleanupOutcome {
    let turn = active.lock().expect("active turn lock poisoned").take();
    match turn {
        Some(mut turn) => {
            let task = match turn.task.take() {
                Some(task) => cleanup_from_result(task.join().await),
                None => CleanupOutcome::NotApplicable,
            };
            CleanupOutcome::merge(task, turn.attachment.release().await)
        }
        None => CleanupOutcome::NotApplicable,
    }
}

async fn close_active(active: &ActiveSlot) -> CleanupOutcome {
    let cancellation = active
        .lock()
        .expect("active turn lock poisoned")
        .as_ref()
        .filter(|turn| !turn.terminal.load(Ordering::SeqCst))
        .filter(|turn| {
            !turn
                .detachment
                .as_ref()
                .is_some_and(|detachment| detachment.is_requested())
        })
        .map(|turn| Arc::clone(&turn.cancellation));
    let cancel_cleanup = match cancellation {
        Some(cancellation) => {
            cleanup_from_result(cancellation.request_bounded().await.map(|_| ()))
        }
        None => CleanupOutcome::NotApplicable,
    };
    CleanupOutcome::merge(cancel_cleanup, join_active(active).await)
}

fn cleanup_from_result(result: Result<(), RuntimeFailure>) -> CleanupOutcome {
    match result {
        Ok(()) => CleanupOutcome::Clean,
        Err(error) => CleanupOutcome::Failed(error.diagnostic().clone()),
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
