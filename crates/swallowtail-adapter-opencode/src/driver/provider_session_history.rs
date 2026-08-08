impl ProviderSessionHistoryDriver for OpenCodeHttpDriver {
    fn page_provider_session_history(
        &self,
        plan: ProviderSessionHistoryPlan,
        request: ProviderSessionHistoryRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionHistoryPage, RuntimeFailure>> {
        Box::pin(async move {
            validate_provider_session_history_execution(&plan, &request, &services)?;
            let version = qualified_plan(plan.preflight()).map_err(|failure| {
                RuntimeFailure::new(failure.diagnostic().clone())
            })?;
            let scope = scope("session-history", request.request_id().as_str())?;
            let policy = SessionAccessPolicy::ambient_harness(ResourceAccess::Read);
            let agreement = plan.agreement();
            let working_resource = agreement.binding().working_resource().ok_or_else(|| {
                failure(
                    "swallowtail.opencode.session_history.resource_invalid",
                    "OpenCode session history requires a filesystem working resource",
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
                        "swallowtail.opencode.session_history.resource_invalid",
                        "OpenCode session history requires a filesystem working resource",
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
                        "swallowtail.opencode.session_history.binding_mismatch",
                        "OpenCode session history observed a different provider session",
                    ));
                }
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
                let total = u32::try_from(replay.len()).map_err(|_| {
                    RuntimeFailure::new(SafeDiagnostic::new(
                        "swallowtail.opencode.session_history.total_invalid",
                        "OpenCode session history exceeds portable cardinality",
                    ))
                })?;
                page_provider_session_history_window(
                    &plan,
                    &request,
                    replay,
                    ProviderSessionHistoryTotal::Exact(total),
                )
            }
            .await;
            let cleanup = access.release(&services).await;
            let window = match result {
                Ok(window)
                    if matches!(cleanup, CleanupOutcome::Clean | CleanupOutcome::NotApplicable) =>
                {
                    window
                }
                Ok(_) => {
                    return Err(failure(
                        "swallowtail.opencode.session_history.cleanup_incomplete",
                        "OpenCode session history cleanup did not complete",
                    ));
                }
                Err(error) => return Err(error),
            };
            ProviderSessionHistoryPage::new(&plan, &request, window, cleanup)
        })
    }
}
