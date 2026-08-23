impl StructuredRunDriver for DeepSeekDirectDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        Box::pin(async move {
            Self::validate_plan(&plan)?;
            services.require_execution_host(plan.execution_host_id())?;
            require_services(&services, true)?;
            let thinking_mode = self.thinking_mode;
            validate_run(&plan, &request, &services, thinking_mode)?;
            let maximum = request
                .maximum_output_tokens()
                .expect("validated maximum")
                .get();
            let body = encode_structured(
                request.content().as_str(),
                maximum,
                request.policy().reasoning_mode(),
                thinking_mode,
            )
            .map_err(protocol)?;
            let wire = HttpRequest::completion(body, true);
            let scope = operation_scope("run", request.request_id().as_str())?;
            let run_id =
                RuntimeRunId::new(format!("deepseek-direct:{}", request.request_id().as_str()))
                    .map_err(|_| {
                        failure(
                            "swallowtail.deepseek.run_id_invalid",
                            "DeepSeek runtime run identity was invalid",
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
                            .expect("DeepSeek run pending work lock poisoned")
                            .take()
                            .expect("DeepSeek run pending work exists");
                        let outcome = pump_run(
                            subscription,
                            access,
                            run_services,
                            events.clone(),
                            cancellation,
                            deadline,
                            thinking_mode,
                            swallowtail_runtime::ActivityOperationId::Run(activity_run_id),
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
                        .expect("DeepSeek run pending work lock poisoned")
                        .take();
                    if let Some((subscription, mut access)) = resources {
                        let _ = subscription.close().await;
                        let _ = access.release(&services).await;
                    }
                    return Err(error);
                }
            };
            Ok(Box::new(DeepSeekRunHandle {
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
    thinking_mode: Option<DeepSeekThinkingMode>,
) -> Result<(), RuntimeFailure> {
    if plan.requirements().driver_role() != swallowtail_core::DriverRole::StructuredRun
        || plan
            .model_id()
            .is_none_or(|model| model.as_str() != DEEPSEEK_MODEL_ID)
        || !plan
            .requirements()
            .capabilities()
            .any(|requirement| requirement.capability() == Capability::StructuredRun)
        || !plan.requirements().capabilities().any(|requirement| {
            requirement.capability() == Capability::ProviderManagedInferenceCache
        })
    {
        return Err(failure(
            "swallowtail.deepseek.role_mismatch",
            "DeepSeek run requires its exact structured-run preflight plan",
        ));
    }
    let maximum = request.maximum_output_tokens().ok_or_else(|| {
        failure(
            "swallowtail.deepseek.output_limit_missing",
            "DeepSeek run requires a preflight-bound maximum output-token input",
        )
    })?;
    if maximum.get() > u64::from(u32::MAX)
        || !plan
            .requirements()
            .capabilities()
            .any(|requirement| requirement.capability() == Capability::OutputTokenLimit)
    {
        return Err(failure(
            "swallowtail.deepseek.output_limit_invalid",
            "DeepSeek maximum output tokens exceed the selected request range",
        ));
    }
    match (thinking_mode, request.policy().reasoning_mode()) {
        (None, Some(reasoning))
            if deepseek_reasoning_mode_is_supported(reasoning)
                && deepseek_plan_supports_reasoning(plan, reasoning) => {}
        (Some(_), None)
            if !plan
                .requirements()
                .capabilities()
                .any(|requirement| requirement.capability() == Capability::ReasoningSelection) => {}
        _ => {
            return Err(unsupported(
                "an unsupported thinking or reasoning selection",
            ));
        }
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
    let policy = request.policy();
    if policy.external_network() != ExternalNetworkPolicy::Denied
        || policy.external_search() != ExternalSearchPolicy::Disabled
        || policy.provider_execution() != ProviderExecutionPolicy::Attached
        || policy.provider_retention() != ProviderRetentionPolicy::Prohibited
        || policy.provider_recovery() != ProviderRecoveryPolicy::Prohibited
        || policy.stream_reattachment() != StreamReattachmentPolicy::Disabled
    {
        return Err(unsupported(
            "network, background, retention, recovery, or reattachment policy",
        ));
    }
    if let Some(deadline) = request.deadline()
        && services.time().expect("validated time").now() >= deadline.instant()
    {
        return Err(failure(
            "swallowtail.deepseek.deadline_elapsed",
            "DeepSeek run deadline elapsed before provider work",
        ));
    }
    Ok(())
}
