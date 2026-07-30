impl StructuredRunDriver for AlibabaModelStudioDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        Box::pin(async move {
            Self::validate_plan(&plan)?;
            services.require_execution_host(plan.execution_host_id())?;
            validate_run(&plan, &request, &services)?;
            let wire = WireRequest::structured_response(request.content()).map_err(protocol)?;
            let scope = ScopeId::new(format!(
                "alibaba-model-studio:run:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| {
                failure(
                    "swallowtail.alibaba_model_studio.scope_invalid",
                    "Alibaba Model Studio run scope was invalid",
                )
            })?;
            let run_id = RuntimeRunId::new(format!(
                "alibaba-model-studio:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| {
                failure(
                    "swallowtail.alibaba_model_studio.run_id_invalid",
                    "Alibaba Model Studio runtime run identity was invalid",
                )
            })?;
            let mut access = AccessLeases::acquire(&plan, scope.clone(), &services).await?;
            let cancelled = Arc::new(AtomicBool::new(false));
            let subscription = match self.transport.subscribe(
                scope.clone(),
                access.endpoint.clone(),
                access.secret()?,
                wire,
                &services,
                Arc::clone(&cancelled),
            ) {
                Ok(subscription) => subscription,
                Err(error) => {
                    let _ = access.release(&services).await;
                    return Err(error);
                }
            };
            let (events, stream) = runtime_event_channel(EVENT_CAPACITY)?;
            events.send(RuntimeEvent::new(0, RuntimeEventKind::Started))?;
            let cancellation = Arc::new(RunCancellation {
                cancelled: Arc::clone(&cancelled),
            });
            let deadline = request.deadline().map(|deadline| {
                services
                    .time()
                    .expect("validated time")
                    .wait_until(deadline)
            });
            let (terminal_sender, terminal) = terminal_outcome_channel();
            let pending = Arc::new(Mutex::new(Some((subscription, access))));
            let task_pending = Arc::clone(&pending);
            let activity_run_id = run_id.clone();
            let task = services.task().expect("validated task").spawn(
                scope,
                Box::pin({
                    let cancellation = Arc::clone(&cancellation);
                    let run_services = services.clone();
                    async move {
                        let (subscription, access) = task_pending
                            .lock()
                            .expect("Alibaba run pending work lock poisoned")
                            .take()
                            .expect("Alibaba run pending work exists");
                        let outcome = pump_run(
                            subscription,
                            access,
                            run_services,
                            events.clone(),
                            cancellation,
                            deadline,
                            activity_run_id,
                        )
                        .await;
                        events.mark_terminal();
                        let _ = terminal_sender.complete(outcome);
                    }
                }),
            );
            let task = match task {
                Ok(task) => task,
                Err(error) => {
                    cancelled.store(true, Ordering::SeqCst);
                    let resources = pending
                        .lock()
                        .expect("Alibaba run pending work lock poisoned")
                        .take();
                    if let Some((subscription, mut access)) = resources {
                        let _ = subscription.close().await;
                        let _ = access.release(&services).await;
                    }
                    return Err(error);
                }
            };
            Ok(Box::new(AlibabaRunHandle {
                request_id: request.request_id().clone(),
                run_id,
                events: Some(Box::pin(stream)),
                terminal: Some(Box::pin(terminal)),
                cancellation,
                task,
            }) as Box<dyn RunHandle>)
        })
    }
}

fn validate_run(
    plan: &PreflightPlan,
    request: &StructuredRunRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    if plan.requirements().driver_role() != swallowtail_core::DriverRole::StructuredRun
        || !plan
            .requirements()
            .capabilities()
            .any(|requirement| requirement.capability() == Capability::StructuredRun)
    {
        return Err(failure(
            "swallowtail.alibaba_model_studio.role_mismatch",
            "Alibaba Model Studio run requires a structured-run preflight plan",
        ));
    }
    if services.task().is_none()
        || services.blocking_work().is_none()
        || services.time().is_none()
        || services.network().is_none()
        || services.credential().is_none()
    {
        return Err(failure(
            "swallowtail.alibaba_model_studio.host_services_missing",
            "Alibaba Model Studio required host services are unavailable",
        ));
    }
    if request.working_resource().is_some() {
        return Err(unsupported("a working resource"));
    }
    if request.attachments().len() != 0 {
        return Err(unsupported("structured-run attachments"));
    }
    if request.tools().len() != 0 {
        return Err(unsupported("structured-run tools"));
    }
    if request.structured_output().is_some() {
        return Err(unsupported("structured output"));
    }
    if request.maximum_output_tokens().is_some() {
        return Err(unsupported("a maximum output-token override"));
    }
    let policy = request.policy();
    if policy.reasoning_mode().is_some()
        || policy.external_network() != ExternalNetworkPolicy::Denied
        || policy.external_search() != ExternalSearchPolicy::Disabled
        || policy.provider_execution() != ProviderExecutionPolicy::Attached
        || policy.provider_retention() != ProviderRetentionPolicy::Prohibited
        || policy.provider_recovery() != ProviderRecoveryPolicy::Prohibited
        || policy.stream_reattachment() != StreamReattachmentPolicy::Disabled
    {
        return Err(unsupported(
            "reasoning, network, background, retention, recovery, or reattachment policy",
        ));
    }
    if let Some(deadline) = request.deadline()
        && services.time().expect("validated time").now() >= deadline.instant()
    {
        return Err(failure(
            "swallowtail.alibaba_model_studio.deadline_elapsed",
            "Alibaba Model Studio run deadline elapsed before provider work",
        ));
    }
    Ok(())
}

