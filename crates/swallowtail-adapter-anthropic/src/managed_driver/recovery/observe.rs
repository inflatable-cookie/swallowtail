#[allow(clippy::too_many_arguments)]
async fn observe_recovered_run(
    transport: &ManagedCurlTransport,
    scope: &ScopeId,
    endpoint: &str,
    credential: &[u8],
    plan: &PreflightPlan,
    resources: &crate::managed_recovery::ManagedRecoveryResources,
    services: &HostServices,
    cancellation: &Arc<ImmediateCancellation>,
    deadline: Option<Deadline>,
) -> Result<ProviderRunReconciliationObservation, RuntimeFailure> {
    let (snapshot, events) = retrieve_recovery_state(
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
    .await?;
    let (state, output) = classify_recovered_run(&snapshot, &events)?;
    ProviderRunReconciliationObservation::new(
        state,
        RunRef::new(&resources.session_id).map_err(|_| recovery_invalid())?,
        output,
        state.is_terminal().then_some(snapshot.usage).flatten(),
    )
}

#[allow(clippy::too_many_arguments)]
async fn retrieve_recovery_state(
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
        crate::managed::ManagedSessionSnapshot,
        Vec<ManagedEvent>,
    ),
    RuntimeFailure,
> {
    let response = recovery_request(
        transport,
        scope,
        endpoint,
        credential,
        Request::session(&resources.session_id),
        services,
        cancellation,
        deadline,
    )
    .await?;
    require_success(&response, "recovered session retrieval")?;
    let agent = plan.provider_agent().ok_or_else(recovery_invalid)?;
    let version = agent
        .version()
        .as_str()
        .parse::<u64>()
        .map_err(|_| recovery_invalid())?;
    let snapshot = crate::managed::parse_session_snapshot(
        &response.body,
        &resources.environment_id,
        agent.id().as_str(),
        version,
        plan.model_id().ok_or_else(recovery_invalid)?.as_str(),
    )?;
    if snapshot.id != resources.session_id {
        return Err(recovery_invalid());
    }
    let events = retrieve_recovery_history(
        transport,
        scope,
        endpoint,
        credential,
        &resources.session_id,
        services,
        cancellation,
        deadline,
    )
    .await?;
    Ok((snapshot, events))
}

#[allow(clippy::too_many_arguments)]
async fn retrieve_recovery_history(
    transport: &ManagedCurlTransport,
    scope: &ScopeId,
    endpoint: &str,
    credential: &[u8],
    session_id: &str,
    services: &HostServices,
    cancellation: &Arc<ImmediateCancellation>,
    deadline: Option<Deadline>,
) -> Result<Vec<ManagedEvent>, RuntimeFailure> {
    let mut cursor = None;
    let mut seen_cursors = BTreeSet::new();
    let mut events = Vec::new();
    for _ in 0..MAXIMUM_RECOVERY_PAGES {
        let response = recovery_request(
            transport,
            scope,
            endpoint,
            credential,
            Request::history_page(session_id, cursor.as_deref()),
            services,
            cancellation,
            deadline,
        )
        .await?;
        require_success(&response, "recovered event retrieval")?;
        let page = crate::managed::parse_history_page(&response.body)?;
        if events.len().saturating_add(page.events.len()) > MAXIMUM_RECOVERY_EVENTS {
            return Err(failure(
                "swallowtail.anthropic.managed.recovery_event_bound",
                "Anthropic Managed Agents recovery exceeded its event bound",
            ));
        }
        events.extend(page.events);
        match page.next_page {
            None => return crate::managed::reconcile(events, []),
            Some(next) if seen_cursors.insert(next.clone()) => cursor = Some(next),
            Some(_) => return Err(recovery_invalid()),
        }
    }
    Err(failure(
        "swallowtail.anthropic.managed.recovery_page_bound",
        "Anthropic Managed Agents recovery exceeded its page bound",
    ))
}

fn classify_recovered_run(
    snapshot: &crate::managed::ManagedSessionSnapshot,
    events: &[ManagedEvent],
) -> Result<(InterruptedRunState, Option<OperationContent>), RuntimeFailure> {
    let mut state = match snapshot.status {
        crate::managed::ManagedSessionStatus::Running => InterruptedRunState::Active,
        crate::managed::ManagedSessionStatus::Idle => InterruptedRunState::InactiveUnresolved,
        crate::managed::ManagedSessionStatus::Terminated => InterruptedRunState::Unknown,
    };
    let mut submitted = false;
    let mut interrupted = false;
    let mut output = String::new();
    for event in events {
        match event.kind() {
            ManagedEventKind::UserMessage(_) => submitted = true,
            ManagedEventKind::UserInterrupt => interrupted = true,
            ManagedEventKind::Message(content) => output.push_str(content.as_str()),
            ManagedEventKind::Running | ManagedEventKind::Rescheduled => {
                state = InterruptedRunState::Active;
            }
            ManagedEventKind::Idle(IdleReason::RequiresAction(_)) => {
                state = InterruptedRunState::WaitingForProviderInput;
            }
            ManagedEventKind::Idle(IdleReason::RetriesExhausted)
            | ManagedEventKind::ProviderError => state = InterruptedRunState::Failed,
            ManagedEventKind::Idle(IdleReason::EndTurn) if interrupted => {
                state = InterruptedRunState::Cancelled;
            }
            ManagedEventKind::Idle(IdleReason::EndTurn) if submitted => {
                state = InterruptedRunState::Completed;
            }
            ManagedEventKind::Idle(IdleReason::EndTurn) | ManagedEventKind::Terminated => {
                state = InterruptedRunState::Unknown;
            }
            ManagedEventKind::Thinking
            | ManagedEventKind::ProviderToolUse { .. }
            | ManagedEventKind::ProviderToolResult { .. }
            | ManagedEventKind::CustomToolUse { .. }
            | ManagedEventKind::Observed
            | ManagedEventKind::Unknown(_) => {}
        }
    }
    let output = if state == InterruptedRunState::Completed && !output.is_empty() {
        Some(OperationContent::new(output).map_err(|_| recovery_invalid())?)
    } else {
        None
    };
    Ok((state, output))
}
