fn open_attachment(
    transport: &ManagedCurlTransport,
    scope: &ScopeId,
    endpoint: &str,
    credential: &[u8],
    resources: &OwnedResources,
    services: &HostServices,
    cancellation: &ManagedRunCancellation,
) -> Result<ManagedSubscription, RuntimeFailure> {
    let connection = Arc::new(AtomicBool::new(false));
    cancellation.install(Arc::clone(&connection));
    transport.subscribe(
        scope.clone(),
        endpoint.to_owned(),
        credential.to_vec(),
        Request::stream(resources.session_id.as_deref().expect("session exists")),
        services,
        connection,
    )
}

#[allow(clippy::too_many_arguments)]
async fn reconcile_history(
    transport: &ManagedCurlTransport,
    scope: &ScopeId,
    endpoint: &str,
    credential: &[u8],
    resources: &OwnedResources,
    services: &HostServices,
    deadline: Deadline,
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    run_id: &RuntimeRunId,
    callbacks: &ManagedCallbackHub,
    live: &[ManagedEvent],
    processed: &mut BTreeSet<String>,
    pending_tools: &mut BTreeSet<String>,
    declared_tools: &BTreeSet<String>,
    output: &mut String,
    operation_deadline: Deadline,
) -> Result<Option<AttachmentExit>, RuntimeFailure> {
    let response = request_before_deadline(
        transport,
        scope,
        endpoint,
        credential,
        Request::history(resources.session_id.as_deref().expect("session exists")),
        deadline,
        services,
    )
    .await?;
    emit_headers(events, sequence, &response.headers)?;
    require_success(&response, "event history retrieval")?;
    let history = crate::managed::parse_history(&response.body)?;
    let reconciled = crate::managed::reconcile(history, live.iter().cloned())?;
    for event in reconciled {
        match apply_event(
            &event,
            events,
            sequence,
            run_id,
            callbacks,
            processed,
            pending_tools,
            declared_tools,
            output,
            operation_deadline,
        )? {
            EventAction::Continue => {}
            EventAction::Terminal(state) => return Ok(Some(AttachmentExit::Terminal(state))),
            EventAction::Callbacks(ids) => return Ok(Some(AttachmentExit::Callbacks(ids))),
        }
    }
    Ok(None)
}

enum CallbackCompletion {
    Continue,
    Cancelled,
    Deadline,
    Failed(RuntimeFailure),
}

#[allow(clippy::too_many_arguments)]
async fn complete_callbacks(
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
    timer: &mut Option<BoxFuture<'static, DeadlineObservation>>,
    pending_tools: &mut BTreeSet<String>,
    deadline: Deadline,
) -> CallbackCompletion {
    let responses = match await_callback_results(callbacks, ids, cancellation, timer).await {
        CallbackSignal::Responses(responses) => responses,
        CallbackSignal::Cancelled => return CallbackCompletion::Cancelled,
        CallbackSignal::Deadline => return CallbackCompletion::Deadline,
        CallbackSignal::Failed(error) => return CallbackCompletion::Failed(error),
    };
    for (provider_event_id, result) in responses {
        let content = match callback_content(result) {
            Ok(content) => content,
            Err(error) => return CallbackCompletion::Failed(error),
        };
        let response = request_before_deadline(
            transport,
            scope,
            endpoint,
            credential,
            Request::custom_tool_result(
                resources.session_id.as_deref().expect("session exists"),
                &provider_event_id,
                &content,
            ),
            deadline,
            services,
        )
        .await;
        let result = response.and_then(|response| {
            emit_headers(events, sequence, &response.headers)?;
            require_success(&response, "custom tool result")
        });
        if let Err(error) = result {
            return if is_deadline_error(&error) {
                CallbackCompletion::Deadline
            } else {
                CallbackCompletion::Failed(error)
            };
        }
        pending_tools.remove(&provider_event_id);
    }
    CallbackCompletion::Continue
}

#[allow(clippy::too_many_arguments)]
async fn retrieve_usage(
    transport: &ManagedCurlTransport,
    scope: &ScopeId,
    endpoint: &str,
    credential: &[u8],
    resources: &OwnedResources,
    services: &HostServices,
    deadline: Deadline,
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
) -> Result<(), RuntimeFailure> {
    let response = request_before_deadline(
        transport,
        scope,
        endpoint,
        credential,
        Request::session(resources.session_id.as_deref().expect("session exists")),
        deadline,
        services,
    )
    .await?;
    emit_headers(events, sequence, &response.headers)?;
    require_success(&response, "session usage retrieval")?;
    let usage = crate::managed::parse_session_usage(&response.body)?;
    emit(
        events,
        sequence,
        RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(usage)),
    )
}

#[allow(clippy::too_many_arguments)]
async fn interrupt_remote(
    transport: &ManagedCurlTransport,
    scope: &ScopeId,
    endpoint: &str,
    credential: &[u8],
    resources: &OwnedResources,
    services: &HostServices,
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
) {
    let Some(session_id) = resources.session_id.as_deref() else {
        return;
    };
    let result = transport
        .request(
            scope.clone(),
            endpoint.to_owned(),
            credential.to_vec(),
            Request::interrupt(session_id),
            services,
            Arc::new(AtomicBool::new(false)),
        )
        .await;
    if let Ok(response) = result {
        let _ = emit_headers(events, sequence, &response.headers);
        let _ = require_success(&response, "interruption");
    }
}
