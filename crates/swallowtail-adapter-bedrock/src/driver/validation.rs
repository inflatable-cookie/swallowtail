fn run_sdk(
    executor: Arc<dyn SdkExecutor>,
    invocation: SdkInvocation,
    updates: mpsc::Sender<Result<StreamUpdate, RuntimeFailure>>,
    cancelled: watch::Receiver<bool>,
) -> Result<(), RuntimeFailure> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|_| failure("swallowtail.bedrock.executor_start_failed", "Bedrock Runtime private executor could not start"))?;
    runtime.block_on(executor.execute(invocation, updates, cancelled))
}

fn validate_run(
    plan: &PreflightPlan,
    request: &StructuredRunRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    if plan.model_route_id().is_none() || plan.model_id().is_none() {
        return Err(failure("swallowtail.bedrock.model_binding_rejected", "Bedrock Runtime requires an exact model route"));
    }
    if request.maximum_output_tokens().is_none()
        || !plan.requirements().capabilities().any(|item| item.capability() == Capability::OutputTokenLimit)
    {
        return Err(failure("swallowtail.bedrock.output_limit_missing", "Bedrock Runtime requires a preflight-bound output-token limit"));
    }
    if request.working_resource().is_some() {
        return Err(unsupported("a working resource"));
    }
    if request.attachments().len() != 0 {
        return Err(unsupported("attachments"));
    }
    if request.tools().len() != 0 {
        return Err(unsupported("structured-run tools"));
    }
    if request.structured_output().is_some() {
        return Err(unsupported("structured output"));
    }
    if request.policy().reasoning_mode().is_some()
        || request.policy().external_network() != ExternalNetworkPolicy::Denied
        || request.policy().external_search() != ExternalSearchPolicy::Disabled
        || request.policy().provider_execution()
            != swallowtail_runtime::ProviderExecutionPolicy::Attached
        || request.policy().provider_retention()
            != swallowtail_runtime::ProviderRetentionPolicy::Prohibited
        || request.policy().provider_recovery()
            != swallowtail_runtime::ProviderRecoveryPolicy::Prohibited
        || request.policy().stream_reattachment()
            != swallowtail_runtime::StreamReattachmentPolicy::Disabled
    {
        return Err(unsupported("provider tools, reasoning, network, background, retention, or reattachment policy"));
    }
    if let Some(deadline) = request.deadline()
        && services.time().expect("validated").now() >= deadline.instant()
    {
        return Err(failure("swallowtail.bedrock.deadline_elapsed", "Bedrock Runtime deadline elapsed before provider work"));
    }
    Ok(())
}

fn require_services(services: &HostServices) -> Result<(), RuntimeFailure> {
    if services.task().is_none() || services.blocking_work().is_none()
        || services.time().is_none() || services.network().is_none()
        || services.credential().is_none()
    {
        Err(failure("swallowtail.bedrock.host_service_missing", "Bedrock Runtime requires task, blocking-work, time, network, and credential services"))
    } else {
        Ok(())
    }
}

fn operation_scope(id: &str) -> Result<ScopeId, RuntimeFailure> {
    ScopeId::new(format!("bedrock-direct:run:{id}")).map_err(|_| failure("swallowtail.bedrock.scope_invalid", "Bedrock Runtime operation scope was invalid"))
}
