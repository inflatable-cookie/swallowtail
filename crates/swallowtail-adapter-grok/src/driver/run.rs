use swallowtail_core::{RunRef, SafeDiagnostic};
use swallowtail_runtime::{
    CallbackExchange, RunHandle, RuntimeRunId, RuntimeTurnId, SessionPlanAgreement,
    StructuredRunDriver, StructuredRunRequest, terminal_outcome_channel,
};

impl StructuredRunDriver for GrokAcpDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        Box::pin(async move {
            self.validate_plan(&plan)?;
            validate_run(&plan, &request, &services)?;
            let resource = request
                .working_resource()
                .expect("validated working resource")
                .clone();
            let open = OpenSessionRequest::new(
                request.request_id().clone(),
                resource,
                None,
                SessionPlanAgreement::explicit(
                    SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite),
                    Some(SessionProviderStatePolicy::DurableProviderSessionPreserved),
                    Some(HarnessConfigurationPosture::Ambient),
                ),
            );
            let mut session = self.start_session(&plan, &open, &services).await?;
            let run_id = RuntimeRunId::new(format!(
                "grok-acp:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| malformed())?;
            let turn_id = RuntimeTurnId::new(format!(
                "grok-acp:run:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| malformed())?;
            let mut turn_request = TurnRequest::new(turn_id, request.content().clone());
            if let Some(deadline) = request.deadline() {
                turn_request = turn_request.with_deadline(deadline);
            }
            let mut turn = match session.start_turn(turn_request, services.clone()).await {
                Ok(turn) => turn,
                Err(error) => {
                    let _ = Box::new(session).close().await;
                    return Err(error);
                }
            };
            let events = match turn.take_events() {
                Some(events) => events,
                None => {
                    let _ = turn.close().await;
                    let _ = Box::new(session).close().await;
                    return Err(failure(
                        "swallowtail.grok.acp.run_events_missing",
                        "Grok structured run did not expose its event stream",
                    ));
                }
            };
            let turn_terminal = match turn.take_terminal_outcome() {
                Some(terminal) => terminal,
                None => {
                    let _ = turn.close().await;
                    let _ = Box::new(session).close().await;
                    return Err(failure(
                        "swallowtail.grok.acp.run_terminal_missing",
                        "Grok structured run did not expose its terminal outcome",
                    ));
                }
            };
            let active = session
                .active
                .lock()
                .expect("ACP active-task lock poisoned")
                .as_ref()
                .map(|active| Arc::clone(&active.turn))
                .ok_or_else(|| {
                    failure(
                        "swallowtail.grok.acp.run_active_missing",
                        "Grok structured run lost its active turn",
                    )
                })?;
            let cancellation = Arc::new(GrokRunCancellation {
                connection: Arc::clone(&session.connection),
                session_id: session.provider_id.clone(),
                turn: active,
                requested: AtomicBool::new(false),
            });
            let pending = Arc::new(Mutex::new(Some((turn, session, turn_terminal))));
            let (sender, terminal) = terminal_outcome_channel();
            let task_pending = Arc::clone(&pending);
            let scope = ScopeId::new(format!(
                "grok-acp:run-cleanup:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| malformed())?;
            let task = services.task().expect("validated task").spawn(
                scope,
                Box::pin(async move {
                    let (turn, session, terminal) = task_pending
                        .lock()
                        .expect("Grok pending run lock poisoned")
                        .take()
                        .expect("Grok pending run exists");
                    let outcome = terminal.await;
                    let turn_cleanup = turn.close().await;
                    let session_cleanup = Box::new(session).close().await;
                    let cleanup = merge_cleanup(
                        outcome.cleanup().clone(),
                        merge_cleanup(turn_cleanup, session_cleanup),
                    );
                    let mut finished =
                        TerminalOutcome::new(outcome.status().clone(), cleanup);
                    if let Some(output) = outcome.output().cloned() {
                        finished = finished.with_output(output);
                    }
                    let _ = sender.complete(finished);
                }),
            );
            let task = match task {
                Ok(task) => task,
                Err(error) => {
                    let _ = cancellation.request().await;
                    let resources = {
                        pending
                            .lock()
                            .expect("Grok pending run lock poisoned")
                            .take()
                    };
                    if let Some((turn, session, _)) = resources {
                        let _ = turn.close().await;
                        let _ = Box::new(session).close().await;
                    }
                    return Err(error);
                }
            };
            Ok(Box::new(GrokRunHandle {
                request_id: request.request_id().clone(),
                run_id,
                events: Some(events),
                terminal: Some(Box::pin(terminal)),
                cancellation,
                task,
            }) as Box<dyn RunHandle>)
        })
    }
}

struct GrokRunCancellation {
    connection: Arc<AcpConnection>,
    session_id: String,
    turn: Arc<ActiveTurn>,
    requested: AtomicBool,
}

impl CancellationControl for GrokRunCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::StructuredRun
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let already = self.requested.swap(true, Ordering::SeqCst) || self.turn.is_finished();
        Box::pin(async move {
            if already {
                return Ok(CancellationAcknowledgement::AlreadyRequested);
            }
            self.turn.mark_cancelled();
            self.connection
                .notify("session/cancel", json!({"sessionId": self.session_id}))
                .await?;
            Ok(CancellationAcknowledgement::Requested)
        })
    }
}

struct GrokRunHandle {
    request_id: RequestId,
    run_id: RuntimeRunId,
    events: Option<BoxEventStream>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: Arc<GrokRunCancellation>,
    task: Box<dyn JoinedTask>,
}

impl RunHandle for GrokRunHandle {
    fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    fn run_id(&self) -> &RuntimeRunId {
        &self.run_id
    }

    fn provider_run_ref(&self) -> Option<&RunRef> {
        None
    }

    fn take_events(&mut self) -> Option<BoxEventStream> {
        self.events.take()
    }

    fn take_callbacks(&mut self) -> Option<CallbackExchange> {
        None
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        self.cancellation.as_ref()
    }

    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>> {
        self.terminal.take()
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            let _ = self.cancellation.request().await;
            self.task.join().await.map_or_else(
                |_| {
                    CleanupOutcome::Failed(SafeDiagnostic::new(
                        "swallowtail.grok.acp.run_join_failed",
                        "Grok structured-run cleanup task did not join",
                    ))
                },
                |_| CleanupOutcome::Clean,
            )
        })
    }
}
