use swallowtail_runtime::{
    InterruptedRunState, ProviderRunReconciliationDriver,
    ProviderRunReconciliationObservation, ProviderRunReconciliationOutcome,
    ProviderRunReconciliationPlan, ProviderRunReconciliationRequest,
    validate_provider_run_reconciliation_execution,
};

impl ProviderRunReconciliationDriver for OpenAiBackgroundDriver {
    fn reconcile_provider_run(
        &self,
        plan: ProviderRunReconciliationPlan,
        request: ProviderRunReconciliationRequest,
        services: HostServices,
    ) -> swallowtail_runtime::BoxFuture<
        '_,
        Result<ProviderRunReconciliationOutcome, RuntimeFailure>,
    > {
        let transport = self.transport.clone();
        Box::pin(async move {
            Self::validate_plan(plan.preflight())?;
            require_reconciliation_services(&services)?;
            validate_provider_run_reconciliation_execution(&plan, &request, &services)?;
            if request.cancellation().is_requested() {
                return Err(reconciliation_cancelled());
            }
            if plan.agreement().deadline().is_some_and(|deadline| {
                services.time().expect("validated time").now() >= deadline.instant()
            }) {
                return Err(reconciliation_timed_out());
            }
            crate::checkpoint::decode_cursor(plan.agreement().checkpoint())?;
            let scope = operation_scope(request.request_id().as_str())?;
            let provider_run = plan
                .agreement()
                .provider_run_ref()
                .as_provider_value()
                .to_owned();
            let retrieve = Request::retrieve(&provider_run)?;
            let mut access =
                AccessLeases::acquire(plan.preflight(), scope.clone(), &services).await?;
            let endpoint = access.endpoint.clone();
            let credential = access.secret()?.to_vec();
            let result = await_reconciliation_response(
                &transport,
                scope,
                endpoint,
                credential,
                retrieve,
                &services,
                &request,
                plan.agreement().deadline(),
            )
                .await
                .and_then(|response| {
                if request.cancellation().is_requested() {
                    return Err(reconciliation_cancelled());
                }
                require_success(&response)?;
                let snapshot = parse_snapshot(&response.body)?;
                if snapshot.response_id != provider_run {
                    return Err(failure(
                        "swallowtail.openai.reconciliation_response_mismatch",
                        "OpenAI reconciliation observed a different response",
                    ));
                }
                let (state, output, usage) = match snapshot.status {
                    BackgroundStatus::Queued | BackgroundStatus::InProgress => {
                        (InterruptedRunState::Active, None, None)
                    }
                    BackgroundStatus::Completed => {
                        let output = snapshot
                            .output_text
                            .map(OperationContent::new)
                            .transpose()
                            .map_err(|_| {
                                failure(
                                    "swallowtail.openai.reconciliation_output_invalid",
                                    "OpenAI reconciliation output is invalid",
                                )
                            })?;
                        (InterruptedRunState::Completed, output, snapshot.usage)
                    }
                    BackgroundStatus::Incomplete | BackgroundStatus::Failed => {
                        (InterruptedRunState::Failed, None, snapshot.usage)
                    }
                    BackgroundStatus::Cancelled => {
                        (InterruptedRunState::Cancelled, None, snapshot.usage)
                    }
                };
                ProviderRunReconciliationObservation::new(
                    state,
                    plan.agreement().provider_run_ref().clone(),
                    output,
                    usage,
                )
            });
            let cleanup = access.release(&services).await;
            let observation = result?;
            ProviderRunReconciliationOutcome::new(&plan, &request, observation, cleanup)
        })
    }
}

enum ReconciliationSignal {
    Response(Result<crate::protocol::Response, RuntimeFailure>),
    Cancelled,
    Deadline,
}

#[allow(clippy::too_many_arguments)]
async fn await_reconciliation_response(
    transport: &CurlTransport,
    scope: ScopeId,
    endpoint: String,
    credential: Vec<u8>,
    retrieve: Request,
    services: &HostServices,
    request: &ProviderRunReconciliationRequest,
    deadline: Option<swallowtail_runtime::Deadline>,
) -> Result<crate::protocol::Response, RuntimeFailure> {
    let stopped = Arc::new(AtomicBool::new(false));
    let mut response = Box::pin(transport.request(
        scope,
        endpoint,
        credential,
        retrieve,
        services,
        Arc::clone(&stopped),
    ));
    let mut cancellation = request.cancellation().wait_requested();
    let mut deadline = deadline.map(|deadline| {
        services
            .time()
            .expect("validated time")
            .wait_until(deadline)
    });
    let signal = poll_fn(|context| {
        if let Poll::Ready(response) = response.as_mut().poll(context) {
            return Poll::Ready(ReconciliationSignal::Response(response));
        }
        if cancellation.as_mut().poll(context).is_ready() {
            return Poll::Ready(ReconciliationSignal::Cancelled);
        }
        if let Some(deadline) = deadline.as_mut()
            && deadline.as_mut().poll(context).is_ready()
        {
            return Poll::Ready(ReconciliationSignal::Deadline);
        }
        Poll::Pending
    })
    .await;
    match signal {
        ReconciliationSignal::Response(response) => response,
        ReconciliationSignal::Cancelled => {
            stopped.store(true, Ordering::SeqCst);
            let _ = response.await;
            Err(reconciliation_cancelled())
        }
        ReconciliationSignal::Deadline => {
            stopped.store(true, Ordering::SeqCst);
            let _ = response.await;
            Err(reconciliation_timed_out())
        }
    }
}

fn reconciliation_cancelled() -> RuntimeFailure {
    failure(
        "swallowtail.openai.reconciliation_cancelled",
        "OpenAI background reconciliation was cancelled",
    )
}

fn reconciliation_timed_out() -> RuntimeFailure {
    failure(
        "swallowtail.openai.reconciliation_timed_out",
        "OpenAI background reconciliation timed out",
    )
}

fn require_reconciliation_services(services: &HostServices) -> Result<(), RuntimeFailure> {
    if services.blocking_work().is_none()
        || services.time().is_none()
        || services.network().is_none()
        || services.credential().is_none()
    {
        Err(failure(
            "swallowtail.openai.reconciliation_host_service_missing",
            "OpenAI background reconciliation requires blocking-work, time, network, and credential services",
        ))
    } else {
        Ok(())
    }
}
