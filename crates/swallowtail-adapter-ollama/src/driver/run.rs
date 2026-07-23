impl StructuredRunDriver for OllamaNativeAttachedDriver {
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
            let requirements = plan
                .requirements()
                .attached_runtime()
                .expect("validated attached requirements");
            let model = requirements.model_tag().as_str().to_owned();
            let maximum = request
                .maximum_output_tokens()
                .expect("validated maximum")
                .get();
            let chat = Request::chat(&model, request.content().as_str(), maximum)?;
            let scope = operation_scope("run", request.request_id().as_str())?;
            let endpoint = authorize_endpoint(&plan, scope.clone(), &services).await?;
            let cancelled = Arc::new(AtomicBool::new(false));
            self.observe_catalogue(
                scope.clone(),
                &endpoint,
                &plan,
                request.deadline(),
                &services,
                Arc::clone(&cancelled),
            )
            .await?;
            let subscription = self.transport.subscribe(
                scope.clone(),
                endpoint,
                chat,
                model,
                &services,
                Arc::clone(&cancelled),
            )?;
            let (event_sender, event_stream) = runtime_event_channel(EVENT_CAPACITY)?;
            event_sender.send(RuntimeEvent::new(0, RuntimeEventKind::Started))?;
            let (terminal_sender, terminal_future) = terminal_outcome_channel();
            let cancellation = Arc::new(RunCancellation::new(Arc::clone(&cancelled)));
            let deadline = request
                .deadline()
                .map(|deadline| services.time().expect("validated time").wait_until(deadline));
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
                            .expect("Ollama pending work lock poisoned")
                            .take()
                            .expect("Ollama pending work is available");
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
                        .expect("Ollama pending work lock poisoned")
                        .take();
                    if let Some(subscription) = subscription {
                        let _ = subscription.close().await;
                    }
                    return Err(error);
                }
            };
            let run_id =
                RuntimeRunId::new(format!("ollama-native:{}", request.request_id().as_str()))
                    .map_err(|_| {
                        failure(
                            "swallowtail.ollama.run_id_invalid",
                            "Ollama runtime run id was invalid",
                        )
                    })?;
            Ok(Box::new(OllamaRunHandle::new(
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
