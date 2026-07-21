#[allow(clippy::too_many_arguments)]
async fn continue_after_callbacks(
    transport: &ManagedCurlTransport,
    scope: &ScopeId,
    endpoint: &str,
    credential: &[u8],
    resources: &OwnedResources,
    services: &HostServices,
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    callbacks: &ManagedCallbackHub,
    ids: &[String],
    cancellation: &ManagedRunCancellation,
    timer: &mut Option<Pin<Box<dyn Future<Output = DeadlineObservation> + Send>>>,
    pending_tools: &mut BTreeSet<String>,
    deadline: Deadline,
) -> Result<ManagedSubscription, ManagedFinal> {
    match complete_callbacks(
        transport,
        scope,
        endpoint,
        credential,
        resources,
        services,
        events,
        sequence,
        callbacks,
        ids,
        cancellation,
        timer,
        pending_tools,
        deadline,
    )
    .await
    {
        CallbackCompletion::Continue => open_attachment(
            transport,
            scope,
            endpoint,
            credential,
            resources,
            services,
            cancellation,
        )
        .map_err(|error| ManagedFinal::status(provider_status(error))),
        CallbackCompletion::Cancelled => {
            callbacks.abandon(CallbackAbandonment::TurnCancelled);
            interrupt_remote(
                transport,
                scope,
                endpoint,
                credential,
                resources,
                services,
                events,
                sequence,
            )
            .await;
            Err(unconfirmed_stop(TerminalStatus::Cancelled))
        }
        CallbackCompletion::Deadline => {
            callbacks.abandon(CallbackAbandonment::TimedOut);
            interrupt_remote(
                transport,
                scope,
                endpoint,
                credential,
                resources,
                services,
                events,
                sequence,
            )
            .await;
            Err(unconfirmed_stop(TerminalStatus::TimedOut))
        }
        CallbackCompletion::Failed(error) => Err(ManagedFinal::status(provider_status(error))),
    }
}

fn unconfirmed_stop(status: TerminalStatus) -> ManagedFinal {
    let mut state = ManagedFinal::status(status);
    state.cancellation = Some(ProviderCancellationOutcome::Unconfirmed);
    state
}
