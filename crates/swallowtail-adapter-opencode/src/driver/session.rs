struct OpenCodeSessionHandle {
    request_id: RequestId,
    runtime_id: RuntimeSessionId,
    resume_binding: SessionResumeBinding,
    provider_id: ProviderId,
    model_id: swallowtail_core::ModelId,
    provider_session_id: String,
    directory: String,
    endpoint: String,
    services: HostServices,
    transport: CurlTransport,
    access: Option<AccessLeases>,
    active: ActiveSlot,
    cancellation: SessionCancellation,
}

impl InteractiveSessionHandle for OpenCodeSessionHandle {
    fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    fn session_id(&self) -> &RuntimeSessionId {
        &self.runtime_id
    }

    fn provider_session_ref(&self) -> Option<&SessionRef> {
        Some(self.resume_binding.provider_session_ref())
    }

    fn resume_binding(&self) -> Option<&SessionResumeBinding> {
        Some(&self.resume_binding)
    }

    fn start_turn<'a>(
        &'a mut self,
        request: TurnRequest,
        services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn TurnHandle>, RuntimeFailure>> {
        Box::pin(async move {
            services.require_execution_host(self.resume_binding.execution_host_id())?;
            validate_turn(&request, &services)?;
            if self.cancellation.requested.load(Ordering::SeqCst) {
                return Err(failure(
                    "swallowtail.opencode.session_cancelled",
                    "OpenCode session was already cancelled",
                ));
            }
            reap_finished(&self.active).await?;
            if self
                .active
                .lock()
                .expect("active turn lock poisoned")
                .is_some()
            {
                return Err(failure(
                    "swallowtail.opencode.turn_active",
                    "OpenCode session already has an active turn",
                ));
            }
            let stream_cancelled = Arc::new(AtomicBool::new(false));
            let turn_scope = scope("turn", request.turn_id().as_str())?;
            let subscription = self
                .transport
                .subscribe(
                    turn_scope.clone(),
                    self.endpoint.clone(),
                    self.directory.clone(),
                    &services,
                    Arc::clone(&stream_cancelled),
                )
                .await?;
            let (event_sender, event_stream) = runtime_event_channel(EVENT_CAPACITY)?;
            event_sender.send(RuntimeEvent::new(1, RuntimeEventKind::Started))?;
            let (terminal_sender, terminal) = terminal_outcome_channel();
            let terminal_flag = Arc::new(AtomicBool::new(false));
            let cancellation = Arc::new(TurnCancellation {
                scope: turn_scope.clone(),
                session_id: self.provider_session_id.clone(),
                directory: self.directory.clone(),
                endpoint: self.endpoint.clone(),
                services: services.clone(),
                transport: self.transport.clone(),
                stream_cancelled,
                requested: AtomicBool::new(false),
            });
            let task_service = services.task().cloned().expect("validated task service");
            let pump_cancellation = Arc::clone(&cancellation);
            let pump_terminal = Arc::clone(&terminal_flag);
            let deadline = request.deadline();
            let pump_services = services.clone();
            let task = task_service.spawn(
                turn_scope.clone(),
                Box::pin(async move {
                    pump_turn(
                        subscription,
                        deadline,
                        pump_services,
                        pump_cancellation,
                        event_sender,
                        terminal_sender,
                        Arc::clone(&pump_terminal),
                    )
                    .await;
                }),
            )?;
            let prompt_response = self
                .transport
                .request(
                    turn_scope.clone(),
                    self.endpoint.clone(),
                    prompt(
                        &self.provider_session_id,
                        self.provider_id.as_str(),
                        self.model_id.as_str(),
                        &self.directory,
                        request.content().as_str(),
                    ),
                    &services,
                    Arc::new(AtomicBool::new(false)),
                )
                .await;
            let prompt_result = match prompt_response {
                Ok(response) => require_no_content(&response),
                Err(error) => Err(error),
            };
            if let Err(error) = prompt_result {
                let _ = cancellation.request().await;
                let _ = task.join().await;
                return Err(error);
            }
            *self.active.lock().expect("active turn lock poisoned") = Some(ActiveTurn {
                task: Some(task),
                cancellation: Arc::clone(&cancellation),
                terminal: Arc::clone(&terminal_flag),
            });
            Ok(Box::new(OpenCodeTurnHandle {
                runtime_id: request.turn_id().clone(),
                events: Some(Box::pin(event_stream)),
                terminal: Some(Box::pin(terminal)),
                cancellation,
                terminal_flag,
                active: Arc::clone(&self.active),
            }) as Box<dyn TurnHandle>)
        })
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        &self.cancellation
    }

    fn close(mut self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            let active_cleanup = close_active(&self.active).await;
            let lease_cleanup = match self.access.as_mut() {
                Some(access) => access.release(&self.services).await,
                None => CleanupOutcome::NotApplicable,
            };
            merge_cleanup(active_cleanup, lease_cleanup)
        })
    }
}


