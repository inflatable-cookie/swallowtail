#[allow(clippy::too_many_arguments)]
async fn cleanup_recovered_run(
    transport: &ManagedCurlTransport,
    scope: &ScopeId,
    endpoint: &str,
    credential: &[u8],
    plan: &PreflightPlan,
    resources: &crate::managed_recovery::ManagedRecoveryResources,
    services: &HostServices,
    cancellation: &Arc<ImmediateCancellation>,
    deadline: Option<Deadline>,
) -> Result<
    (
        ProviderRecoveredResourceCleanupEffect,
        Option<SafeDiagnostic>,
    ),
    RuntimeFailure,
> {
    if stopped(cancellation, deadline, services) {
        return Ok((
            ProviderRecoveredResourceCleanupEffect::FailedBeforeEffect,
            Some(cleanup_stopped()),
        ));
    }
    let observed = retrieve_recovery_state(
        transport,
        scope,
        endpoint,
        credential,
        plan,
        resources,
        services,
        cancellation,
        deadline,
    )
    .await;
    let (snapshot, events) = match observed {
        Ok(state) => state,
        Err(error)
            if matches!(
                error.diagnostic().code(),
                "swallowtail.anthropic.managed.reconciliation_cancelled"
                    | "swallowtail.anthropic.managed.reconciliation_timed_out"
            ) =>
        {
            return Ok((
                ProviderRecoveredResourceCleanupEffect::FailedBeforeEffect,
                Some(error.diagnostic().clone()),
            ));
        }
        Err(error) => return Err(error),
    };
    let (state, _) = classify_recovered_run(&snapshot, &events)?;
    if matches!(
        state,
        InterruptedRunState::Active
            | InterruptedRunState::WaitingForProviderInput
            | InterruptedRunState::Unknown
    ) {
        return Ok((
            ProviderRecoveredResourceCleanupEffect::RejectedActiveOrUnknown,
            Some(SafeDiagnostic::new(
                "swallowtail.anthropic.managed.recovered_cleanup_active",
                "Anthropic Managed Agents recovered resources remain active or ambiguous",
            )),
        ));
    }

    let session = delete_recovered_resource(
        transport,
        scope,
        endpoint,
        credential,
        services,
        cancellation,
        deadline,
        Request::delete_session(&resources.session_id),
        &resources.session_id,
        OwnedRemoteResourceKind::Session,
    )
    .await;
    if session != RecoveryDeletion::Confirmed {
        return Ok((
            session.effect_before_any_confirmation(),
            Some(session.diagnostic()),
        ));
    }
    if stopped(cancellation, deadline, services) {
        return Ok((
            ProviderRecoveredResourceCleanupEffect::PartiallyApplied,
            Some(cleanup_stopped()),
        ));
    }
    let environment = delete_recovered_resource(
        transport,
        scope,
        endpoint,
        credential,
        services,
        cancellation,
        deadline,
        Request::delete_environment(&resources.environment_id),
        &resources.environment_id,
        OwnedRemoteResourceKind::Environment,
    )
    .await;
    match environment {
        RecoveryDeletion::Confirmed => Ok((ProviderRecoveredResourceCleanupEffect::Applied, None)),
        RecoveryDeletion::StoppedBeforeDispatch | RecoveryDeletion::Unconfirmed => Ok((
            ProviderRecoveredResourceCleanupEffect::PartiallyApplied,
            Some(environment.diagnostic()),
        )),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RecoveryDeletion {
    Confirmed,
    StoppedBeforeDispatch,
    Unconfirmed,
}

impl RecoveryDeletion {
    const fn effect_before_any_confirmation(self) -> ProviderRecoveredResourceCleanupEffect {
        match self {
            Self::StoppedBeforeDispatch => {
                ProviderRecoveredResourceCleanupEffect::FailedBeforeEffect
            }
            Self::Confirmed => ProviderRecoveredResourceCleanupEffect::Applied,
            Self::Unconfirmed => ProviderRecoveredResourceCleanupEffect::UnconfirmedAfterEffect,
        }
    }

    fn diagnostic(self) -> SafeDiagnostic {
        match self {
            Self::StoppedBeforeDispatch => cleanup_stopped(),
            Self::Confirmed => SafeDiagnostic::new(
                "swallowtail.anthropic.managed.recovered_cleanup_complete",
                "Anthropic Managed Agents recovered resource cleanup completed",
            ),
            Self::Unconfirmed => SafeDiagnostic::new(
                "swallowtail.anthropic.managed.recovered_cleanup_unconfirmed",
                "Anthropic Managed Agents recovered resource deletion is unconfirmed",
            ),
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn delete_recovered_resource(
    transport: &ManagedCurlTransport,
    scope: &ScopeId,
    endpoint: &str,
    credential: &[u8],
    services: &HostServices,
    cancellation: &Arc<ImmediateCancellation>,
    deadline: Option<Deadline>,
    provider_request: Request,
    expected_id: &str,
    kind: OwnedRemoteResourceKind,
) -> RecoveryDeletion {
    if stopped(cancellation, deadline, services) {
        return RecoveryDeletion::StoppedBeforeDispatch;
    }
    match recovery_request(
        transport,
        scope,
        endpoint,
        credential,
        provider_request,
        services,
        cancellation,
        deadline,
    )
    .await
    {
        Ok(response)
            if require_success(&response, "recovered resource deletion").is_ok()
                && crate::managed::parse_deletion(&response.body, expected_id, kind)
                    == Ok(RemoteResourceDeletionOutcome::Confirmed) =>
        {
            RecoveryDeletion::Confirmed
        }
        _ => RecoveryDeletion::Unconfirmed,
    }
}
