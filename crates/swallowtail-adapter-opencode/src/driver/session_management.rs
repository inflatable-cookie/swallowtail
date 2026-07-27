use swallowtail_core::{
    ProviderSessionAffectedScope, ProviderSessionDeletionStrength, ProviderSessionManagementAction,
    ProviderSessionManagementEffect,
};
use swallowtail_runtime::{
    ArchiveProviderSessionRequest, DeleteProviderSessionRequest, ImmediateCancellation,
    ProviderSessionManagementAgreement, ProviderSessionManagementDriver,
    ProviderSessionManagementOutcome, ProviderSessionManagementPlan,
    RestoreProviderSessionRequest, validate_provider_session_management_request,
};

impl ProviderSessionManagementDriver for OpenCodeHttpDriver {
    fn archive_session(
        &self,
        _plan: ProviderSessionManagementPlan,
        _request: ArchiveProviderSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        Box::pin(async { Err(unsupported("provider-session archive")) })
    }

    fn restore_session(
        &self,
        _plan: ProviderSessionManagementPlan,
        _request: RestoreProviderSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        Box::pin(async { Err(unsupported("provider-session restore")) })
    }

    fn delete_session(
        &self,
        plan: ProviderSessionManagementPlan,
        request: DeleteProviderSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<ProviderSessionManagementOutcome, RuntimeFailure>> {
        Box::pin(async move {
            validate_provider_session_management_request(&plan, request.agreement(), &services)?;
            self.manage_delete(
                plan,
                request.agreement(),
                request.cancellation(),
                request.request_id(),
                services,
            )
            .await
        })
    }
}

impl OpenCodeHttpDriver {
    async fn manage_delete(
        &self,
        plan: ProviderSessionManagementPlan,
        agreement: &ProviderSessionManagementAgreement,
        cancellation: &ImmediateCancellation,
        request_id: &RequestId,
        services: HostServices,
    ) -> Result<ProviderSessionManagementOutcome, RuntimeFailure> {
        let action = ProviderSessionManagementAction::Delete(
            ProviderSessionDeletionStrength::ProviderDataDeleted,
        );
        if agreement.action() != action {
            return Err(failure(
                "swallowtail.opencode.lifecycle.deletion_strength_mismatch",
                "OpenCode management plan requests a different deletion strength",
            ));
        }
        if agreement.affected_scope() != ProviderSessionAffectedScope::ProviderDefinedDescendants {
            return Err(failure(
                "swallowtail.opencode.lifecycle.affected_scope_mismatch",
                "OpenCode management plan requests a different affected scope",
            ));
        }
        let version = Self::validate_plan(plan.preflight())?;
        if cancelled_or_expired(agreement, cancellation, &services)? {
            return Ok(outcome(
                agreement,
                ProviderSessionManagementEffect::failed_before_effect(action),
            ));
        }
        let working_resource = agreement
            .binding()
            .working_resource()
            .ok_or_else(|| {
                failure(
                    "swallowtail.opencode.lifecycle.resource_missing",
                    "OpenCode deletion requires its bound working resource",
                )
            })?
            .clone();
        let scope = scope("management", request_id.as_str())?;
        let policy = SessionAccessPolicy::ambient_harness(ResourceAccess::Read);
        let mut access = AccessLeases::acquire(
            plan.preflight(),
            scope.clone(),
            &services,
            Some((&working_resource, &policy)),
        )
        .await?;
        let directory = access
            .directory
            .clone()
            .expect("management resource was acquired");

        let result = async {
            if cancelled_or_expired(agreement, cancellation, &services)? {
                return Ok(outcome(
                    agreement,
                    ProviderSessionManagementEffect::failed_before_effect(action),
                ));
            }
            let health_cancelled = Arc::new(AtomicBool::new(false));
            let health = wait_before_dispatch(
                self.transport.request(
                    scope.clone(),
                    access.endpoint.clone(),
                    Request::get("/global/health"),
                    &services,
                    Arc::clone(&health_cancelled),
                ),
                agreement,
                cancellation,
                &services,
                health_cancelled,
            )
            .await?;
            let Some(health) = health else {
                return Ok(outcome(
                    agreement,
                    ProviderSessionManagementEffect::failed_before_effect(action),
                ));
            };
            require_health_matches(&health, &version)?;
            if cancelled_or_expired(agreement, cancellation, &services)? {
                return Ok(outcome(
                    agreement,
                    ProviderSessionManagementEffect::failed_before_effect(action),
                ));
            }

            let transport_cancelled = Arc::new(AtomicBool::new(false));
            let request = session_delete(
                agreement
                    .binding()
                    .provider_session_ref()
                    .as_provider_value(),
                &directory,
            )?;
            let (response, interrupted) = wait_after_dispatch(
                self.transport.request(
                    scope,
                    access.endpoint.clone(),
                    request,
                    &services,
                    Arc::clone(&transport_cancelled),
                ),
                agreement,
                cancellation,
                &services,
                transport_cancelled,
            )
            .await?;
            if interrupted {
                return Ok(outcome(
                    agreement,
                    ProviderSessionManagementEffect::unconfirmed_after_effect(action),
                ));
            }
            Ok(match response {
                Ok(response) => match classify_session_delete(&response) {
                    SessionDeleteResponse::Applied => outcome(
                        agreement,
                        ProviderSessionManagementEffect::applied(
                            action,
                            ProviderSessionAffectedScope::ProviderDefinedDescendants,
                        ),
                    ),
                    SessionDeleteResponse::Rejected => outcome(
                        agreement,
                        ProviderSessionManagementEffect::failed_before_effect(action),
                    )
                    .with_diagnostic(SafeDiagnostic::new(
                        "swallowtail.opencode.lifecycle.delete_rejected",
                        "OpenCode rejected session deletion before applying it",
                    )),
                    SessionDeleteResponse::Unconfirmed => outcome(
                        agreement,
                        ProviderSessionManagementEffect::unconfirmed_after_effect(action),
                    )
                    .with_diagnostic(SafeDiagnostic::new(
                        "swallowtail.opencode.lifecycle.delete_unconfirmed",
                        "OpenCode deletion response did not confirm the provider effect",
                    )),
                },
                Err(error) => outcome(
                    agreement,
                    ProviderSessionManagementEffect::unconfirmed_after_effect(action),
                )
                .with_diagnostic(error.diagnostic().clone()),
            })
        }
        .await;
        let cleanup = access.release(&services).await;
        match result {
            Ok(result) => Ok(with_cleanup(result, cleanup)),
            Err(error) => {
                let _ = cleanup;
                Err(error)
            }
        }
    }
}

fn cancelled_or_expired(
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
            "swallowtail.opencode.lifecycle.time_service_missing",
            "Deadline-bound OpenCode deletion requires a time service",
        )
    })?;
    Ok(time.now() >= deadline.instant())
}

async fn wait_before_dispatch<F>(
    work: F,
    agreement: &ProviderSessionManagementAgreement,
    cancellation: &ImmediateCancellation,
    services: &HostServices,
    transport_cancelled: Arc<AtomicBool>,
) -> Result<Option<Response>, RuntimeFailure>
where
    F: Future<Output = Result<Response, RuntimeFailure>>,
{
    let (result, interrupted) = wait_controlled_joined(
        work,
        agreement,
        cancellation,
        services,
        transport_cancelled,
    )
    .await?;
    if interrupted {
        Ok(None)
    } else {
        result.map(Some)
    }
}

async fn wait_after_dispatch<F>(
    work: F,
    agreement: &ProviderSessionManagementAgreement,
    cancellation: &ImmediateCancellation,
    services: &HostServices,
    transport_cancelled: Arc<AtomicBool>,
) -> Result<(Result<Response, RuntimeFailure>, bool), RuntimeFailure>
where
    F: Future<Output = Result<Response, RuntimeFailure>>,
{
    wait_controlled_joined(
        work,
        agreement,
        cancellation,
        services,
        transport_cancelled,
    )
    .await
}

async fn wait_controlled_joined<F>(
    work: F,
    agreement: &ProviderSessionManagementAgreement,
    cancellation: &ImmediateCancellation,
    services: &HostServices,
    transport_cancelled: Arc<AtomicBool>,
) -> Result<(Result<Response, RuntimeFailure>, bool), RuntimeFailure>
where
    F: Future<Output = Result<Response, RuntimeFailure>>,
{
    let mut work = Box::pin(work);
    let mut cancellation_wait = cancellation.wait_requested();
    let mut deadline = agreement
        .deadline()
        .map(|deadline| {
            services
                .time()
                .ok_or_else(|| {
                    failure(
                        "swallowtail.opencode.lifecycle.time_service_missing",
                        "Deadline-bound OpenCode deletion requires a time service",
                    )
                })
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
            transport_cancelled.store(true, Ordering::SeqCst);
            context.waker().wake_by_ref();
        }
        if !interrupted
            && deadline
                .as_mut()
                .is_some_and(|deadline| deadline.as_mut().poll(context).is_ready())
        {
            interrupted = true;
            transport_cancelled.store(true, Ordering::SeqCst);
            context.waker().wake_by_ref();
        }
        Poll::Pending
    })
    .await;
    Ok((result, interrupted))
}

fn outcome(
    agreement: &ProviderSessionManagementAgreement,
    effect: ProviderSessionManagementEffect,
) -> ProviderSessionManagementOutcome {
    ProviderSessionManagementOutcome::new(agreement.binding().clone(), effect)
}

fn with_cleanup(
    outcome: ProviderSessionManagementOutcome,
    cleanup: CleanupOutcome,
) -> ProviderSessionManagementOutcome {
    match cleanup {
        CleanupOutcome::Clean | CleanupOutcome::NotApplicable => outcome,
        CleanupOutcome::Degraded(diagnostic) | CleanupOutcome::Failed(diagnostic) => {
            outcome.with_diagnostic(diagnostic)
        }
    }
}
