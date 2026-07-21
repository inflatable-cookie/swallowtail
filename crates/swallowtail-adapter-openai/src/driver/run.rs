use crate::protocol::{BackgroundStream, ProviderEvent, Request};
use crate::transport::{StreamItem, Subscription};
use swallowtail_runtime::{
    StructuredRunDriver, runtime_event_channel, terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 64;

impl StructuredRunDriver for OpenAiBackgroundDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> swallowtail_runtime::BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        Box::pin(async move {
            Self::validate_plan(&plan)?;
            services.require_execution_host(plan.execution_host_id())?;
            require_services(&services)?;
            validate_run(&plan, &request, &services)?;

            let model = plan.model_id().expect("validated model").as_str().to_owned();
            let maximum = request
                .maximum_output_tokens()
                .expect("validated output bound")
                .get();
            let create = Request::create(&model, request.content(), maximum)?;
            let scope = operation_scope(request.request_id().as_str())?;
            let mut access = AccessLeases::acquire(&plan, scope.clone(), &services).await?;
            let connection = Arc::new(AtomicBool::new(false));
            let cancellation = Arc::new(RunCancellation::new(Arc::clone(&connection)));
            let mut subscription = match self.transport.subscribe(
                scope.clone(),
                access.endpoint.clone(),
                access.secret()?.to_vec(),
                create,
                &services,
                connection,
            ) {
                Ok(subscription) => subscription,
                Err(error) => {
                    let _ = access.release(&services).await;
                    return Err(error);
                }
            };
            let (event_sender, event_stream) = runtime_event_channel(EVENT_CAPACITY)?;
            event_sender.send(RuntimeEvent::new(0, RuntimeEventKind::Started))?;
            let mut sequence = 1;
            let mut stream = BackgroundStream::initial();
            let mut deadline = Some(
                services
                    .time()
                    .expect("validated time")
                    .wait_until(request.deadline().expect("validated deadline")),
            );
            let response_id = match await_response_identity(
                &mut subscription,
                &mut stream,
                &event_sender,
                &mut sequence,
                &mut deadline,
            )
            .await
            {
                Ok(response_id) => response_id,
                Err(error) => {
                    let _ = subscription.close().await;
                    let _ = access.release(&services).await;
                    return Err(error);
                }
            };
            let provider_run_ref = RunRef::new(response_id.clone())
                .expect("validated OpenAI response identity is non-blank");
            let run_id = RuntimeRunId::new(format!(
                "openai-background:{}",
                request.request_id().as_str()
            ))
            .expect("validated request identity makes a valid runtime run identity");
            let (terminal_sender, terminal_future) = terminal_outcome_channel();
            let pending = Arc::new(Mutex::new(Some((subscription, access))));
            let task_service = services.task().expect("validated task").clone();
            let task_pending = Arc::clone(&pending);
            let task_cancellation = Arc::clone(&cancellation);
            let run_services = services.clone();
            let transport = self.transport.clone();
            let task = task_service.spawn(
                scope.clone(),
                Box::pin(async move {
                    let (subscription, access) = task_pending
                        .lock()
                        .expect("OpenAI pending work lock poisoned")
                        .take()
                        .expect("OpenAI pending work is available");
                    let outcome = pump_run(
                        transport,
                        scope,
                        response_id,
                        subscription,
                        stream,
                        access,
                        run_services,
                        event_sender.clone(),
                        sequence,
                        task_cancellation,
                        deadline,
                    )
                    .await;
                    let _ = terminal_sender.complete(outcome);
                    event_sender.mark_terminal();
                }),
            );
            let task = match task {
                Ok(task) => task,
                Err(error) => {
                    cancellation.requested.store(true, Ordering::SeqCst);
                    cancellation.stop_active();
                    let resources = pending
                        .lock()
                        .expect("OpenAI pending work lock poisoned")
                        .take();
                    if let Some((subscription, mut access)) = resources {
                        let _ = subscription.close().await;
                        let _ = access.release(&services).await;
                    }
                    return Err(error);
                }
            };
            Ok(Box::new(OpenAiRunHandle {
                request_id: request.request_id().clone(),
                run_id,
                provider_run_ref,
                events: Some(Box::pin(event_stream)),
                terminal: Some(Box::pin(terminal_future)),
                cancellation,
                task,
            }) as Box<dyn RunHandle>)
        })
    }
}

async fn await_response_identity(
    subscription: &mut Subscription,
    stream: &mut BackgroundStream,
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    deadline: &mut Option<swallowtail_runtime::BoxFuture<'static, swallowtail_runtime::DeadlineObservation>>,
) -> Result<String, RuntimeFailure> {
    loop {
        match next_run_signal(subscription, deadline).await {
            RunSignal::Deadline => {
                return Err(failure(
                    "swallowtail.openai.identity_deadline",
                    "OpenAI response identity was not observed before the deadline",
                ));
            }
            RunSignal::Closed | RunSignal::Item(Err(_)) => {
                return Err(failure(
                    "swallowtail.openai.remote_state_unconfirmed",
                    "OpenAI remote state is unconfirmed because no response identity was observed",
                ));
            }
            RunSignal::Item(Ok(StreamItem::Headers(headers))) => {
                emit_headers(events, sequence, &headers)?;
            }
            RunSignal::Item(Ok(StreamItem::Frame(frame))) => match stream.apply(&frame)? {
                ProviderEvent::Created(snapshot) => {
                    emit(events, sequence, RuntimeEventKind::Progress)?;
                    return Ok(snapshot.response_id);
                }
                _ => {
                    return Err(failure(
                        "swallowtail.openai.identity_event_invalid",
                        "OpenAI emitted work before the response identity",
                    ));
                }
            },
        }
    }
}
