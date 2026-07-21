impl StructuredRunDriver for LlamaCppAttachedDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        Box::pin(async move {
            self.validate_plan(&plan)?;
            services.require_execution_host(plan.execution_host_id())?;
            require_services(&services, true)?;
            validate_run(&plan, &request, &services)?;
            let model = plan
                .model_id()
                .expect("validated model")
                .as_str()
                .to_owned();
            let maximum = request
                .maximum_output_tokens()
                .expect("validated maximum")
                .get();
            let chat = Request::chat(&model, request.content(), maximum)?;
            let scope = operation_scope("run", request.request_id().as_str())?;
            let endpoint = authorize_endpoint(&plan, scope.clone(), &services).await?;
            let cancelled = Arc::new(AtomicBool::new(false));
            self.observe(
                scope.clone(),
                &endpoint,
                Some(&model),
                request.deadline(),
                &services,
                Arc::clone(&cancelled),
            )
            .await?;
            let subscription = self.transport.subscribe(
                scope.clone(),
                endpoint,
                chat,
                &services,
                Arc::clone(&cancelled),
            )?;
            let (event_sender, event_stream) = runtime_event_channel(EVENT_CAPACITY)?;
            event_sender.send(RuntimeEvent::new(0, RuntimeEventKind::Started))?;
            let (terminal_sender, terminal_future) = terminal_outcome_channel();
            let cancellation = Arc::new(RunCancellation::new(Arc::clone(&cancelled)));
            let deadline = request.deadline().map(|deadline| {
                services
                    .time()
                    .expect("validated time")
                    .wait_until(deadline)
            });
            let pending = Arc::new(Mutex::new(Some(subscription)));
            let task_service = services.task().expect("validated task").clone();
            let task = task_service.spawn(
                scope,
                Box::pin({
                    let cancellation = Arc::clone(&cancellation);
                    let pending = Arc::clone(&pending);
                    async move {
                        let subscription = pending
                            .lock()
                            .expect("llama.cpp pending work lock poisoned")
                            .take()
                            .expect("llama.cpp pending work is available");
                        let outcome =
                            pump_run(subscription, event_sender.clone(), cancellation, deadline)
                                .await;
                        let _ = terminal_sender.complete(outcome);
                        event_sender.mark_terminal();
                    }
                }),
            );
            let task = match task {
                Ok(task) => task,
                Err(error) => {
                    cancelled.store(true, Ordering::SeqCst);
                    let subscription = pending
                        .lock()
                        .expect("llama.cpp pending work lock poisoned")
                        .take();
                    if let Some(subscription) = subscription {
                        let _ = subscription.close().await;
                    }
                    return Err(error);
                }
            };
            let run_id = RuntimeRunId::new(format!(
                "{}:{}",
                self.run_id_prefix,
                request.request_id().as_str()
            ))
            .map_err(|_| {
                failure(
                    "swallowtail.llama_cpp.run_id_invalid",
                    "llama.cpp runtime run id was invalid",
                )
            })?;
            Ok(Box::new(LlamaCppRunHandle::new(
                request.request_id().clone(),
                run_id,
                Box::pin(event_stream),
                Box::pin(terminal_future),
                cancellation,
                task,
            )) as Box<dyn RunHandle>)
        })
    }
}
