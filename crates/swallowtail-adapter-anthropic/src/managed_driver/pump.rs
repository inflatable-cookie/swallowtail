struct ManagedFinal {
    status: TerminalStatus,
    output: Option<OperationContent>,
    cancellation: Option<ProviderCancellationOutcome>,
}

impl ManagedFinal {
    fn status(status: TerminalStatus) -> Self {
        Self {
            status,
            output: None,
            cancellation: None,
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn pump_managed_run(
    transport: ManagedCurlTransport,
    scope: ScopeId,
    run_id: RuntimeRunId,
    mut subscription: ManagedSubscription,
    mut access: ManagedAccessLeases,
    mut resources: OwnedResources,
    endpoint: String,
    credential: ManagedSecret,
    services: HostServices,
    events: swallowtail_runtime::RuntimeEventSender,
    mut sequence: u64,
    cancellation: Arc<ManagedRunCancellation>,
    callbacks: ManagedCallbackHub,
    declared_tools: BTreeSet<String>,
    deadline: Deadline,
) -> TerminalOutcome {
    let mut timer = Some(
        services
            .time()
            .expect("validated time")
            .wait_until(deadline),
    );
    let mut live = Vec::new();
    let mut processed = BTreeSet::new();
    let mut pending_tools = BTreeSet::new();
    let mut output = String::new();
    let mut recovered = false;
    let final_state = 'run: loop {
        let exit = pump_attachment(
            &mut subscription,
            &events,
            &mut sequence,
            &run_id,
            &callbacks,
            &mut live,
            &mut processed,
            &mut pending_tools,
            &declared_tools,
            &mut output,
            &cancellation,
            deadline,
            &mut timer,
        )
        .await;
        let _ = subscription.close().await;
        match exit {
            AttachmentExit::Terminal(state) => break state,
            AttachmentExit::Cancelled => {
                callbacks.abandon(CallbackAbandonment::TurnCancelled);
                interrupt_remote(
                    &transport,
                    &scope,
                    &endpoint,
                    &credential.0,
                    &resources,
                    &services,
                    &events,
                    &mut sequence,
                )
                .await;
                let mut state = ManagedFinal::status(TerminalStatus::Cancelled);
                state.cancellation = Some(ProviderCancellationOutcome::Unconfirmed);
                break state;
            }
            AttachmentExit::Deadline => {
                callbacks.abandon(CallbackAbandonment::TimedOut);
                interrupt_remote(
                    &transport,
                    &scope,
                    &endpoint,
                    &credential.0,
                    &resources,
                    &services,
                    &events,
                    &mut sequence,
                )
                .await;
                let mut state = ManagedFinal::status(TerminalStatus::TimedOut);
                state.cancellation = Some(ProviderCancellationOutcome::Unconfirmed);
                break state;
            }
            AttachmentExit::Callbacks(ids) => {
                match continue_after_callbacks(
                    &transport,
                    &scope,
                    &endpoint,
                    &credential.0,
                    &resources,
                    &services,
                    &events,
                    &mut sequence,
                    &callbacks,
                    &ids,
                    &cancellation,
                    &mut timer,
                    &mut pending_tools,
                    deadline,
                )
                .await
                {
                    Ok(next) => subscription = next,
                    Err(state) => break 'run state,
                }
            }
            AttachmentExit::Disconnected if !recovered => {
                recovered = true;
                match reconcile_history(
                    &transport,
                    &scope,
                    &endpoint,
                    &credential.0,
                    &resources,
                    &services,
                    deadline,
                    &events,
                    &mut sequence,
                    &run_id,
                    &callbacks,
                    &live,
                    &mut processed,
                    &mut pending_tools,
                    &declared_tools,
                    &mut output,
                    deadline,
                )
                .await
                {
                    Ok(Some(AttachmentExit::Terminal(state))) => break state,
                    Ok(Some(AttachmentExit::Callbacks(ids))) => {
                        match continue_after_callbacks(
                            &transport,
                            &scope,
                            &endpoint,
                            &credential.0,
                            &resources,
                            &services,
                            &events,
                            &mut sequence,
                            &callbacks,
                            &ids,
                            &cancellation,
                            &mut timer,
                            &mut pending_tools,
                            deadline,
                        )
                        .await
                        {
                            Ok(next) => subscription = next,
                            Err(state) => break 'run state,
                        }
                    }
                    Ok(Some(_)) => {
                        break ManagedFinal::status(provider_status(failure(
                            "swallowtail.anthropic.managed.recovery_state_invalid",
                            "Anthropic Managed Agents history recovery returned an invalid state",
                        )));
                    }
                    Ok(None) => match open_attachment(
                        &transport,
                        &scope,
                        &endpoint,
                        &credential.0,
                        &resources,
                        &services,
                        &cancellation,
                    ) {
                        Ok(next) => subscription = next,
                        Err(error) => break ManagedFinal::status(provider_status(error)),
                    },
                    Err(error) if is_deadline_error(&error) => {
                        callbacks.abandon(CallbackAbandonment::TimedOut);
                        interrupt_remote(
                            &transport,
                            &scope,
                            &endpoint,
                            &credential.0,
                            &resources,
                            &services,
                            &events,
                            &mut sequence,
                        )
                        .await;
                        let mut state = ManagedFinal::status(TerminalStatus::TimedOut);
                        state.cancellation = Some(ProviderCancellationOutcome::Unconfirmed);
                        break state;
                    }
                    Err(error) => break ManagedFinal::status(provider_status(error)),
                }
            }
            AttachmentExit::Disconnected => {
                break ManagedFinal::status(provider_status(failure(
                    "swallowtail.anthropic.managed.remote_state_unconfirmed",
                    "Anthropic Managed Agents state remained unconfirmed after bounded recovery",
                )));
            }
        }
    };

    finish_managed_run(
        &transport,
        &scope,
        &endpoint,
        &credential.0,
        &mut access,
        &mut resources,
        &services,
        &events,
        &mut sequence,
        &callbacks,
        deadline,
        final_state,
        output,
    )
    .await
}

fn provider_status(error: RuntimeFailure) -> TerminalStatus {
    TerminalStatus::ProviderFailed(error.diagnostic().clone())
}

fn is_deadline_error(error: &RuntimeFailure) -> bool {
    error.diagnostic().code() == "swallowtail.anthropic.managed.setup_deadline"
}

include!("pump/attachment.rs");
include!("pump/events.rs");
include!("pump/management.rs");
include!("pump/cleanup.rs");
include!("pump/transitions.rs");
include!("pump/finalize.rs");
