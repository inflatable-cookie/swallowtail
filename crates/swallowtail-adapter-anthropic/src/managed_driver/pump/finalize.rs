#[allow(clippy::too_many_arguments)]
async fn finish_managed_run(
    transport: &ManagedCurlTransport,
    scope: &ScopeId,
    endpoint: &str,
    credential: &[u8],
    access: &mut ManagedAccessLeases,
    resources: &mut OwnedResources,
    services: &HostServices,
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    callbacks: &ManagedCallbackHub,
    deadline: Deadline,
    mut final_state: ManagedFinal,
    output: String,
) -> TerminalOutcome {
    if matches!(final_state.status, TerminalStatus::Completed)
        && let Err(error) = retrieve_usage(
            transport,
            scope,
            endpoint,
            credential,
            resources,
            services,
            deadline,
            events,
            sequence,
        )
        .await
    {
        final_state.status = provider_status(error);
    }
    callbacks.abandon(CallbackAbandonment::TurnTerminated);
    let (session_deletion, environment_deletion, mut cleanup) =
        cleanup_resources(transport, scope, endpoint, credential, resources, services).await;
    cleanup = merge_cleanup(cleanup, access.release(services).await);
    let mut outcome = TerminalOutcome::new(final_state.status, cleanup)
        .with_remote_resource_deletion(OwnedRemoteResourceKind::Session, session_deletion)
        .with_remote_resource_deletion(
            OwnedRemoteResourceKind::Environment,
            environment_deletion,
        );
    if let Some(output) = final_state.output.or_else(|| OperationContent::new(output).ok())
        && !output.as_str().is_empty()
    {
        outcome = outcome.with_output(output);
    }
    if let Some(provider_cancellation) = final_state.cancellation {
        outcome = outcome.with_provider_cancellation(provider_cancellation);
    }
    outcome
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
