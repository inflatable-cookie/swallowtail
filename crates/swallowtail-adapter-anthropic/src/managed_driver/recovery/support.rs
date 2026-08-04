#[allow(clippy::too_many_arguments)]
async fn recovery_request(
    transport: &ManagedCurlTransport,
    scope: &ScopeId,
    endpoint: &str,
    credential: &[u8],
    provider_request: Request,
    services: &HostServices,
    cancellation: &Arc<ImmediateCancellation>,
    deadline: Option<Deadline>,
) -> Result<ManagedResponse, RuntimeFailure> {
    if cancellation.is_requested() {
        return Err(recovery_cancelled());
    }
    if deadline.is_some_and(|deadline| {
        services.time().expect("validated time").now() >= deadline.instant()
    }) {
        return Err(recovery_timed_out());
    }
    let stopped = Arc::new(AtomicBool::new(false));
    let mut operation = Box::pin(transport.request(
        scope.clone(),
        endpoint.to_owned(),
        credential.to_vec(),
        provider_request,
        services,
        Arc::clone(&stopped),
    ));
    let mut cancellation_wait = cancellation.wait_requested();
    let mut deadline_wait = deadline.map(|deadline| {
        services
            .time()
            .expect("validated time")
            .wait_until(deadline)
    });
    let signal = poll_fn(|context| {
        if let Poll::Ready(result) = operation.as_mut().poll(context) {
            return Poll::Ready(RecoverySignal::Response(result));
        }
        if cancellation_wait.as_mut().poll(context).is_ready() {
            return Poll::Ready(RecoverySignal::Cancelled);
        }
        if let Some(deadline) = deadline_wait.as_mut()
            && deadline.as_mut().poll(context).is_ready()
        {
            return Poll::Ready(RecoverySignal::Deadline);
        }
        Poll::Pending
    })
    .await;
    match signal {
        RecoverySignal::Response(result) => result,
        RecoverySignal::Cancelled => {
            stopped.store(true, Ordering::SeqCst);
            let _ = operation.await;
            Err(recovery_cancelled())
        }
        RecoverySignal::Deadline => {
            stopped.store(true, Ordering::SeqCst);
            let _ = operation.await;
            Err(recovery_timed_out())
        }
    }
}

enum RecoverySignal {
    Response(Result<ManagedResponse, RuntimeFailure>),
    Cancelled,
    Deadline,
}

fn stopped(
    cancellation: &Arc<ImmediateCancellation>,
    deadline: Option<Deadline>,
    services: &HostServices,
) -> bool {
    cancellation.is_requested()
        || deadline.is_some_and(|deadline| {
            services.time().expect("validated time").now() >= deadline.instant()
        })
}

fn cleanup_stopped() -> SafeDiagnostic {
    SafeDiagnostic::new(
        "swallowtail.anthropic.managed.recovered_cleanup_stopped",
        "Anthropic Managed Agents recovered cleanup stopped before its next effect",
    )
}

fn cleanup_diagnostic(cleanup: CleanupOutcome) -> Option<SafeDiagnostic> {
    match cleanup {
        CleanupOutcome::Clean | CleanupOutcome::NotApplicable => None,
        CleanupOutcome::Degraded(diagnostic) | CleanupOutcome::Failed(diagnostic) => {
            Some(diagnostic)
        }
    }
}

fn require_recovery_services(services: &HostServices) -> Result<(), RuntimeFailure> {
    if services.blocking_work().is_none()
        || services.time().is_none()
        || services.network().is_none()
        || services.credential().is_none()
    {
        Err(failure(
            "swallowtail.anthropic.managed.recovery_host_service_missing",
            "Anthropic Managed Agents recovery requires blocking-work, time, network, and credential services",
        ))
    } else {
        Ok(())
    }
}

fn recovery_cancelled() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.managed.reconciliation_cancelled",
        "Anthropic Managed Agents recovery was cancelled",
    )
}

fn recovery_timed_out() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.managed.reconciliation_timed_out",
        "Anthropic Managed Agents recovery timed out",
    )
}

fn recovery_invalid() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.managed.recovery_state_invalid",
        "Anthropic Managed Agents recovered state did not match its exact binding",
    )
}
