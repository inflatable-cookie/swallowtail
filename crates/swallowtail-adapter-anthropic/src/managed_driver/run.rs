#[derive(Default)]
struct OwnedResources {
    environment_id: Option<String>,
    session_id: Option<String>,
}

impl StructuredRunDriver for AnthropicManagedAgentDriver {
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
            let agent_version = validate_run(&plan, &request, &services)?;
            let agent = plan.provider_agent().expect("validated agent").clone();
            let model = plan.model_id().expect("validated model").as_str().to_owned();
            let deadline = request.deadline().expect("validated deadline");
            let run_id = RuntimeRunId::new(format!(
                "anthropic-managed:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| binding_failure("runtime run identity"))?;
            let scope = operation_scope(request.request_id().as_str())?;
            let mut access = ManagedAccessLeases::acquire(&plan, scope.clone(), &services).await?;
            let endpoint = access.endpoint.clone();
            let credential = ManagedSecret(access.secret()?.to_vec());
            let mut resources = OwnedResources::default();
            let mut setup_headers = Vec::new();
            let setup = provision(
                &self.transport,
                &scope,
                &agent,
                agent_version,
                &model,
                &request,
                deadline,
                &services,
                &endpoint,
                &credential.0,
                &mut resources,
                &mut setup_headers,
            )
            .await;
            let (subscription, connection) = match setup {
                Ok(value) => value,
                Err(error) => {
                    let _ = cleanup_resources(
                        &self.transport,
                        &scope,
                        &endpoint,
                        &credential.0,
                        &mut resources,
                        &services,
                    )
                    .await;
                    let _ = access.release(&services).await;
                    return Err(error);
                }
            };
            let (event_sender, event_stream) = runtime_event_channel(EVENT_CAPACITY)?;
            event_sender.send(RuntimeEvent::new(0, RuntimeEventKind::Started))?;
            let mut sequence = 1;
            for headers in setup_headers {
                emit_headers(&event_sender, &mut sequence, &headers)?;
            }
            let (callback_hub, callback_exchange) = ManagedCallbackHub::new();
            let declared_tools = request
                .tools()
                .map(|tool| tool.name().to_owned())
                .collect::<BTreeSet<_>>();
            let cancellation = Arc::new(ManagedRunCancellation::new(
                Arc::clone(&connection),
                callback_hub.clone(),
            ));
            let (terminal_sender, terminal_future) = terminal_outcome_channel();
            let pending = Arc::new(Mutex::new(Some((
                subscription,
                access,
                resources,
                endpoint,
                credential,
            ))));
            let task_pending = Arc::clone(&pending);
            let task_cancellation = Arc::clone(&cancellation);
            let task_services = services.clone();
            let task_transport = self.transport.clone();
            let task_scope = scope.clone();
            let task_run_id = run_id.clone();
            let task = services.task().expect("validated task").spawn(
                scope.clone(),
                Box::pin(async move {
                    let (subscription, access, resources, endpoint, credential) = task_pending
                        .lock()
                        .expect("managed pending work lock poisoned")
                        .take()
                        .expect("managed pending work is available");
                    let outcome = pump_managed_run(
                        task_transport,
                        task_scope,
                        task_run_id,
                        subscription,
                        access,
                        resources,
                        endpoint,
                        credential,
                        task_services,
                        event_sender.clone(),
                        sequence,
                        task_cancellation,
                        callback_hub,
                        declared_tools,
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
                    let resources = {
                        pending
                            .lock()
                            .expect("managed pending work lock poisoned")
                            .take()
                    };
                    if let Some((subscription, mut access, mut resources, endpoint, credential)) =
                        resources
                    {
                        let _ = subscription.close().await;
                        let _ = cleanup_resources(
                            &self.transport,
                            &scope,
                            &endpoint,
                            &credential.0,
                            &mut resources,
                            &services,
                        )
                        .await;
                        let _ = access.release(&services).await;
                    }
                    return Err(error);
                }
            };
            Ok(Box::new(ManagedRunHandle {
                request_id: request.request_id().clone(),
                run_id,
                events: Some(Box::pin(event_stream)),
                callbacks: Some(callback_exchange),
                terminal: Some(Box::pin(terminal_future)),
                cancellation,
                task,
            }) as Box<dyn RunHandle>)
        })
    }
}
