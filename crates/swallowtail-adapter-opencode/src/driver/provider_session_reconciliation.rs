impl ProviderSessionReconciliationDriver for OpenCodeHttpDriver {
    fn reconcile_provider_session(
        &self,
        plan: ProviderSessionReconciliationPlan,
        request: ProviderSessionReconciliationRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionReconciliationOutcome, RuntimeFailure>> {
        Box::pin(async move {
            validate_provider_session_reconciliation_execution(&plan, &request, &services)?;
            let version = qualified_plan(plan.preflight()).map_err(|failure| {
                RuntimeFailure::new(failure.diagnostic().clone())
            })?;
            let scope = scope("session-reconciliation", request.request_id().as_str())?;
            let policy = SessionAccessPolicy::ambient_harness(ResourceAccess::Read);
            let agreement = plan.agreement();
            let working_resource = agreement.binding().working_resource().ok_or_else(|| {
                failure(
                    "swallowtail.opencode.session_reconciliation.resource_invalid",
                    "OpenCode session reconciliation requires a filesystem working resource",
                )
            })?;
            let mut access = AccessLeases::acquire(
                plan.preflight(),
                scope.clone(),
                &services,
                Some((working_resource, &policy)),
            )
            .await?;
            let result = async {
                let directory = access.directory.as_deref().ok_or_else(|| {
                    failure(
                        "swallowtail.opencode.session_reconciliation.resource_invalid",
                        "OpenCode session reconciliation requires a filesystem working resource",
                    )
                })?;
                let cancelled = Arc::new(AtomicBool::new(false));
                let health = reconciliation_request(
                    &self.transport,
                    scope.clone(),
                    access.endpoint.clone(),
                    Request::get("/global/health"),
                    ReconciliationControl::new(
                        &services,
                        request.cancellation(),
                        agreement.deadline(),
                        Arc::clone(&cancelled),
                    ),
                )
                .await?;
                require_health_matches(&health, &version)?;
                let lookup = reconciliation_request(
                    &self.transport,
                    scope.clone(),
                    access.endpoint.clone(),
                    session_get(
                        agreement.binding().provider_session_ref().as_provider_value(),
                        directory,
                    ),
                    ReconciliationControl::new(
                        &services,
                        request.cancellation(),
                        agreement.deadline(),
                        Arc::clone(&cancelled),
                    ),
                )
                .await?;
                let observed = parse_session_lookup(&lookup)?;
                if observed.id
                    != agreement
                        .binding()
                        .provider_session_ref()
                        .as_provider_value()
                    || observed.directory != directory
                    || observed.version != version.binding().version().as_str()
                {
                    return Err(failure(
                        "swallowtail.opencode.session_reconciliation.binding_mismatch",
                        "OpenCode session reconciliation observed a different provider session",
                    ));
                }
                let statuses = reconciliation_request(
                    &self.transport,
                    scope.clone(),
                    access.endpoint.clone(),
                    session_status(directory),
                    ReconciliationControl::new(
                        &services,
                        request.cancellation(),
                        agreement.deadline(),
                        Arc::clone(&cancelled),
                    ),
                )
                .await
                .and_then(|response| parse_session_statuses(&response))?;
                let state = match statuses.get(&observed.id) {
                    Some(OpenCodeSessionStatus::Active) => {
                        swallowtail_runtime::InterruptedTurnState::Active
                    }
                    Some(OpenCodeSessionStatus::Idle) => {
                        swallowtail_runtime::InterruptedTurnState::InactiveUnresolved
                    }
                    Some(OpenCodeSessionStatus::Unavailable) | None => {
                        swallowtail_runtime::InterruptedTurnState::Unknown
                    }
                };
                let replay = self
                    .load_replay(
                        scope,
                        ReplaySource::new(
                            &access.endpoint,
                            directory,
                            agreement.binding().provider_session_ref(),
                        ),
                        agreement.deadline(),
                        &services,
                        cancelled,
                        Some(request.cancellation()),
                    )
                    .await?;
                Ok((
                    state,
                    swallowtail_runtime::bound_provider_session_replay_tail(
                        replay,
                        agreement.bounds(),
                    ),
                ))
            }
            .await;
            let cleanup = access.release(&services).await;
            let (state, (replay, replay_complete)) = match result {
                Ok(value) if matches!(cleanup, CleanupOutcome::Clean | CleanupOutcome::NotApplicable) => value,
                Ok(_) => {
                    return Err(failure(
                        "swallowtail.opencode.session_reconciliation.cleanup_incomplete",
                        "OpenCode session reconciliation cleanup did not complete",
                    ));
                }
                Err(error) => return Err(error),
            };
            ProviderSessionReconciliationOutcome::new(
                &plan,
                &request,
                swallowtail_runtime::ProviderSessionReconciliationObservation::session_scoped(
                    state,
                    replay,
                    replay_complete,
                ),
                cleanup,
            )
        })
    }
}

async fn reconciliation_request(
    transport: &CurlTransport,
    scope: ScopeId,
    endpoint: String,
    request: Request,
    control: ReconciliationControl<'_>,
) -> Result<Response, RuntimeFailure> {
    let ReconciliationControl {
        services,
        cancellation,
        deadline,
        cancelled,
    } = control;
    if cancellation.is_requested() {
        return Err(failure(
            "swallowtail.opencode.session_reconciliation.cancelled",
            "OpenCode session reconciliation was cancelled",
        ));
    }
    let mut work = Box::pin(transport.request(
        scope,
        endpoint,
        request,
        services,
        Arc::clone(&cancelled),
    ));
    let mut cancellation_wait = cancellation.wait_requested();
    let mut deadline_wait = deadline.map(|deadline| {
        services
            .time()
            .expect("validated time service")
            .wait_until(deadline)
    });
    std::future::poll_fn(|context| {
        if let Poll::Ready(result) = work.as_mut().poll(context) {
            return Poll::Ready(result);
        }
        if cancellation_wait.as_mut().poll(context).is_ready() {
            cancelled.store(true, Ordering::SeqCst);
            return Poll::Ready(Err(failure(
                "swallowtail.opencode.session_reconciliation.cancelled",
                "OpenCode session reconciliation was cancelled",
            )));
        }
        if deadline_wait
            .as_mut()
            .is_some_and(|wait| wait.as_mut().poll(context).is_ready())
        {
            cancelled.store(true, Ordering::SeqCst);
            return Poll::Ready(Err(failure(
                "swallowtail.opencode.session_reconciliation.timed_out",
                "OpenCode session reconciliation timed out",
            )));
        }
        Poll::Pending
    })
    .await
}

struct ReconciliationControl<'a> {
    services: &'a HostServices,
    cancellation: &'a swallowtail_runtime::ImmediateCancellation,
    deadline: Option<Deadline>,
    cancelled: Arc<AtomicBool>,
}

impl<'a> ReconciliationControl<'a> {
    fn new(
        services: &'a HostServices,
        cancellation: &'a swallowtail_runtime::ImmediateCancellation,
        deadline: Option<Deadline>,
        cancelled: Arc<AtomicBool>,
    ) -> Self {
        Self {
            services,
            cancellation,
            deadline,
            cancelled,
        }
    }
}
