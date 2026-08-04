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
    reasoning_mode: Option<swallowtail_core::ReasoningMode>,
    structured_output: Option<StructuredOutputDescriptor>,
    image_attachments: bool,
    provider_callbacks: bool,
    active_turn_detachment: bool,
    callback_run_id: Option<swallowtail_runtime::RuntimeRunId>,
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
            validate_turn(&request, &services, self.image_attachments)?;
            let turn_scope = scope("turn", request.turn_id().as_str())?;
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
            let (event_sender, event_stream) = runtime_event_channel(EVENT_CAPACITY)?;
            let (terminal_sender, terminal) = terminal_outcome_channel();
            let terminal_flag = Arc::new(AtomicBool::new(false));
            let (file, attachment) = input::prepare(
                request.attachments().next().cloned(),
                &services,
                turn_scope.clone(),
            )
            .await?;
            let prompt_request = match prompt(
                &self.provider_session_id,
                self.provider_id.as_str(),
                self.model_id.as_str(),
                &self.directory,
                PromptPayload {
                    content: request.content().as_str(),
                    reasoning: self.reasoning_mode.as_ref(),
                    structured_output: self.structured_output.as_ref(),
                    file: file.as_ref(),
                },
            ) {
                Ok(request) => request,
                Err(error) => {
                    let _ = attachment.release().await;
                    return Err(error);
                }
            };
            let stream_cancelled = Arc::new(AtomicBool::new(false));
            let subscription = match self
                .transport
                .subscribe(
                    turn_scope.clone(),
                    self.endpoint.clone(),
                    self.directory.clone(),
                    &services,
                    Arc::clone(&stream_cancelled),
                )
                .await
            {
                Ok(subscription) => subscription,
                Err(error) => {
                    let _ = attachment.release().await;
                    return Err(error);
                }
            };
            if let Err(error) =
                event_sender.send(RuntimeEvent::new(1, RuntimeEventKind::Started))
            {
                drop(subscription);
                let _ = attachment.release().await;
                return Err(error);
            }
            let (callback_hub, callback_exchange) = if self.provider_callbacks {
                let (hub, exchange) = callback::CallbackHub::new(
                    turn_scope.clone(),
                    self.directory.clone(),
                    self.endpoint.clone(),
                    services.clone(),
                    self.transport.clone(),
                );
                (Some(hub), Some(exchange))
            } else {
                (None, None)
            };
            let detachment_stream = Arc::clone(&stream_cancelled);
            let cancellation = Arc::new(TurnCancellation {
                scope: turn_scope.clone(),
                session_id: self.provider_session_id.clone(),
                directory: self.directory.clone(),
                endpoint: self.endpoint.clone(),
                services: services.clone(),
                transport: self.transport.clone(),
                stream_cancelled,
                requested: AtomicBool::new(false),
                callbacks: callback_hub.clone(),
            });
            let detachment = self.active_turn_detachment.then(|| {
                Arc::new(TurnDetachment {
                    stream_cancelled: detachment_stream,
                    terminal: Arc::clone(&terminal_flag),
                    cancellation: Arc::clone(&cancellation),
                    requested: AtomicBool::new(false),
                })
            });
            let task_service = services.task().cloned().expect("validated task service");
            let pump_cancellation = Arc::clone(&cancellation);
            let pump_terminal = Arc::clone(&terminal_flag);
            let deadline = request.deadline();
            let turn_id = request.turn_id().clone();
            let pump_turn_id = turn_id.clone();
            let callback_operation = self.callback_run_id.clone().map_or_else(
                || swallowtail_runtime::CallbackOperationId::Turn(turn_id.clone()),
                swallowtail_runtime::CallbackOperationId::Run,
            );
            let pump_services = services.clone();
            let pump_callback_hub = callback_hub.clone();
            let pump_detachment = detachment.clone();
            let task = match task_service.spawn(
                turn_scope.clone(),
                Box::pin(async move {
                    pump_turn(TurnPump {
                        turn_id: pump_turn_id,
                        subscription,
                        deadline,
                        services: pump_services,
                        cancellation: pump_cancellation,
                        detachment: pump_detachment,
                        events: event_sender,
                        terminal: terminal_sender,
                        terminal_flag: Arc::clone(&pump_terminal),
                        callback_hub: pump_callback_hub,
                        callback_operation,
                    })
                    .await;
                }),
            ) {
                Ok(task) => task,
                Err(error) => {
                    if let Some(callback_hub) = callback_hub {
                        callback_hub.abandon(swallowtail_runtime::CallbackAbandonment::Closed);
                    }
                    let _ = attachment.release().await;
                    return Err(error);
                }
            };
            let prompt_response = self
                .transport
                .request(
                    turn_scope.clone(),
                    self.endpoint.clone(),
                    prompt_request,
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
                let _ = attachment.release().await;
                return Err(error);
            }
            *self.active.lock().expect("active turn lock poisoned") = Some(ActiveTurn {
                task: Some(task),
                cancellation: Arc::clone(&cancellation),
                detachment: detachment.clone(),
                terminal: Arc::clone(&terminal_flag),
                attachment: attachment.clone(),
            });
            Ok(Box::new(OpenCodeTurnHandle {
                runtime_id: turn_id,
                events: Some(Box::pin(event_stream)),
                terminal: Some(Box::pin(terminal)),
                cancellation,
                detachment,
                terminal_flag,
                active: Arc::clone(&self.active),
                callbacks: callback_exchange,
                attachment,
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
