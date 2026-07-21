#[allow(clippy::too_many_arguments)]
fn open_reattachment(
    transport: &CurlTransport,
    scope: &ScopeId,
    response_id: &str,
    cursor: u64,
    endpoint: &str,
    credential: &SecretMaterial,
    services: &HostServices,
    cancellation: &RunCancellation,
) -> Result<Subscription, RuntimeFailure> {
    let connection = Arc::new(AtomicBool::new(false));
    cancellation.install(Arc::clone(&connection));
    transport.subscribe(
        scope.clone(),
        endpoint.to_owned(),
        credential.0.clone(),
        Request::reattach(response_id, cursor)?,
        services,
        connection,
    )
}

#[allow(clippy::too_many_arguments)]
async fn retrieve_terminal(
    transport: &CurlTransport,
    scope: &ScopeId,
    response_id: &str,
    endpoint: &str,
    credential: &SecretMaterial,
    services: &HostServices,
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
) -> FinalState {
    match retrieve_snapshot(
        transport,
        scope,
        response_id,
        endpoint,
        credential,
        services,
        events,
        sequence,
    )
    .await
    {
        Ok(snapshot) if snapshot.status.is_terminal() => management_terminal(snapshot, None),
        _ => FinalState::new(provider_status(failure(
            "swallowtail.openai.remote_state_unconfirmed",
            "OpenAI remote state remained unconfirmed after bounded stream recovery",
        ))),
    }
}

#[derive(Clone, Copy)]
enum LocalStop {
    Cancelled,
    TimedOut,
}

#[allow(clippy::too_many_arguments)]
async fn stop_remote(
    transport: &CurlTransport,
    scope: &ScopeId,
    response_id: &str,
    endpoint: &str,
    credential: &SecretMaterial,
    services: &HostServices,
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    local: LocalStop,
) -> FinalState {
    let cancelled = Arc::new(AtomicBool::new(false));
    let result = async {
        let response = transport
            .request(
                scope.clone(),
                endpoint.to_owned(),
                credential.0.clone(),
                Request::cancel(response_id)?,
                services,
                cancelled,
            )
            .await?;
        emit_headers(events, sequence, &response.headers)?;
        require_success(&response)?;
        parse_snapshot(&response.body)
    }
    .await;
    let snapshot = match result {
        Ok(snapshot) => Some(snapshot),
        Err(_) => retrieve_snapshot(
            transport,
            scope,
            response_id,
            endpoint,
            credential,
            services,
            events,
            sequence,
        )
        .await
        .ok(),
    };
    match snapshot {
        Some(snapshot) => management_terminal(snapshot, Some(local)),
        None => unconfirmed_stop(local),
    }
}

#[allow(clippy::too_many_arguments)]
async fn retrieve_snapshot(
    transport: &CurlTransport,
    scope: &ScopeId,
    response_id: &str,
    endpoint: &str,
    credential: &SecretMaterial,
    services: &HostServices,
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
) -> Result<ResponseSnapshot, RuntimeFailure> {
    let response = transport
        .request(
            scope.clone(),
            endpoint.to_owned(),
            credential.0.clone(),
            Request::retrieve(response_id)?,
            services,
            Arc::new(AtomicBool::new(false)),
        )
        .await?;
    emit_headers(events, sequence, &response.headers)?;
    require_success(&response)?;
    let snapshot = parse_snapshot(&response.body)?;
    if snapshot.response_id != response_id {
        return Err(failure(
            "swallowtail.openai.response_correlation_failed",
            "OpenAI management response did not match the active response",
        ));
    }
    Ok(snapshot)
}

fn management_terminal(snapshot: ResponseSnapshot, local: Option<LocalStop>) -> FinalState {
    match snapshot.status {
        BackgroundStatus::Cancelled => {
            let mut state = FinalState::new(match local {
                Some(LocalStop::TimedOut) => TerminalStatus::TimedOut,
                _ => TerminalStatus::Cancelled,
            });
            state.cancellation = Some(ProviderCancellationOutcome::Confirmed);
            state.usage = snapshot.usage;
            state
        }
        BackgroundStatus::Completed => {
            let mut state = FinalState::new(match local {
                Some(LocalStop::TimedOut) => TerminalStatus::TimedOut,
                _ => TerminalStatus::Completed,
            });
            if local.is_some() {
                state.cancellation = Some(ProviderCancellationOutcome::RacedWithCompletion);
            }
            state.output = snapshot.output_text;
            state.usage = snapshot.usage;
            state
        }
        BackgroundStatus::Incomplete => FinalState::new(provider_status(failure(
            "swallowtail.openai.response_incomplete",
            "OpenAI background response was incomplete",
        ))),
        BackgroundStatus::Failed => FinalState::new(provider_status(failure(
            "swallowtail.openai.response_failed",
            "OpenAI background response failed",
        ))),
        BackgroundStatus::Queued | BackgroundStatus::InProgress => {
            local.map_or_else(
                || {
                    FinalState::new(provider_status(failure(
                        "swallowtail.openai.remote_state_unconfirmed",
                        "OpenAI remote work remained active after bounded observation",
                    )))
                },
                unconfirmed_stop,
            )
        }
    }
}

fn unconfirmed_stop(local: LocalStop) -> FinalState {
    let mut state = FinalState::new(match local {
        LocalStop::Cancelled => TerminalStatus::Cancelled,
        LocalStop::TimedOut => TerminalStatus::TimedOut,
    });
    state.cancellation = Some(ProviderCancellationOutcome::Unconfirmed);
    state
}

struct SecretMaterial(Vec<u8>);

impl Drop for SecretMaterial {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}
