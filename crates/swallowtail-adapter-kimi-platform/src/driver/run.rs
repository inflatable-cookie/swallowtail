use crate::protocol::{Event, MODEL_ID, MAXIMUM_OUTPUT_TOKENS, PROVIDER_ID, parse_events, provider_failure};
use crate::transport::{StreamItem, Subscription};
use std::sync::Mutex;
use swallowtail_core::{
    Capability, CapabilityConstraint, ExternalNetworkPolicy, ExternalSearchPolicy,
};
use swallowtail_runtime::{
    OperationContent, ProviderObservation, RunHandle, RuntimeEvent, RuntimeEventKind, RuntimeRunId,
    StructuredRunDriver, StructuredRunRequest, TerminalOutcome, TerminalStatus, TokenUsage,
    runtime_event_channel, terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 64;

impl StructuredRunDriver for KimiPlatformDirectDriver {
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
            validate_run(&plan, &request, &services)?;
            let reasoning = request.policy().reasoning_mode().expect("validated reasoning");
            let maximum = request.maximum_output_tokens().expect("validated maximum").get();
            let chat = Request::chat(MODEL_ID, request.content(), reasoning, maximum)?;
            let scope = operation_scope("run", request.request_id().as_str())?;
            let mut access = AccessLeases::acquire(&plan, scope.clone(), &services).await?;
            let cancelled = Arc::new(AtomicBool::new(false));
            let subscription = match self.transport.subscribe(
                scope.clone(),
                access.endpoint.clone(),
                access.secret()?.to_vec(),
                chat,
                &services,
                Arc::clone(&cancelled),
            ) {
                Ok(subscription) => subscription,
                Err(error) => {
                    let _ = access.release(&services).await;
                    return Err(error);
                }
            };
            let (event_sender, event_stream) = runtime_event_channel(EVENT_CAPACITY)?;
            event_sender.send(RuntimeEvent::new(0, RuntimeEventKind::Started))?;
            let (terminal_sender, terminal_future) = terminal_outcome_channel();
            let cancellation = Arc::new(RunCancellation::new(Arc::clone(&cancelled)));
            let deadline = request
                .deadline()
                .map(|deadline| services.time().expect("validated time").wait_until(deadline));
            let task_service = services.task().expect("validated task").clone();
            let pending = Arc::new(Mutex::new(Some((subscription, access))));
            let run_services = services.clone();
            let task = task_service.spawn(
                scope,
                Box::pin({
                    let cancellation = Arc::clone(&cancellation);
                    let pending = Arc::clone(&pending);
                    async move {
                        let (subscription, access) = pending
                            .lock()
                            .expect("Kimi Platform pending work lock poisoned")
                            .take()
                            .expect("Kimi Platform pending work is available");
                        let outcome = pump_run(
                            subscription,
                            access,
                            run_services,
                            event_sender.clone(),
                            cancellation,
                            deadline,
                        )
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
                    let resources = pending
                        .lock()
                        .expect("Kimi Platform pending work lock poisoned")
                        .take();
                    if let Some((subscription, mut access)) = resources {
                        let _ = subscription.close().await;
                        let _ = access.release(&services).await;
                    }
                    return Err(error);
                }
            };
            let run_id = RuntimeRunId::new(format!(
                "kimi-platform-direct:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| failure(
                "swallowtail.kimi_platform.run_id_invalid",
                "Kimi Platform runtime run id was invalid",
            ))?;
            Ok(Box::new(KimiPlatformRunHandle::new(
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

fn validate_run(
    plan: &PreflightPlan,
    request: &StructuredRunRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    if plan.requirements().driver_role() != DriverRole::StructuredRun
        || plan.requirements().operation_shape() != OperationShape::StructuredRun
        || plan.model_id().map(|id| id.as_str()) != Some(MODEL_ID)
        || plan.provider_id().map(|id| id.as_str()) != Some(PROVIDER_ID)
    {
        return Err(failure(
            "swallowtail.kimi_platform.model_binding_rejected",
            "Kimi Platform run requires the exact moonshot kimi-k3 route",
        ));
    }
    let maximum = request.maximum_output_tokens().map(std::num::NonZeroU64::get);
    if maximum.is_none_or(|maximum| maximum > MAXIMUM_OUTPUT_TOKENS)
        || !has_capability(plan, Capability::OutputTokenLimit)
    {
        return Err(failure(
            "swallowtail.kimi_platform.output_limit_missing",
            "Kimi Platform run requires a bounded preflight output-token input",
        ));
    }
    let reasoning = request.policy().reasoning_mode().ok_or_else(|| failure(
        "swallowtail.kimi_platform.reasoning_missing",
        "Kimi Platform K3 requires an explicit reasoning selection",
    ))?;
    if !matches!(reasoning.as_str(), "low" | "high" | "max")
        || !plan.requirements().capabilities().any(|requirement| {
            requirement.capability() == Capability::ReasoningSelection
                && requirement.constraints().any(|constraint| {
                    matches!(constraint, CapabilityConstraint::ReasoningMode(mode) if mode == reasoning)
                })
        })
    {
        return Err(failure(
            "swallowtail.kimi_platform.reasoning_binding_rejected",
            "Kimi Platform reasoning selection did not match preflight",
        ));
    }
    if request.working_resource().is_some() { return Err(unsupported("a working resource")); }
    if request.attachments().len() != 0 { return Err(unsupported("attachments")); }
    if request.tools().len() != 0 { return Err(unsupported("structured-run tools")); }
    if request.structured_output().is_some() { return Err(unsupported("structured output")); }
    if request.policy().external_network() != ExternalNetworkPolicy::Denied
        || request.policy().external_search() != ExternalSearchPolicy::Disabled
        || request.policy().provider_execution() != swallowtail_runtime::ProviderExecutionPolicy::Attached
        || request.policy().provider_retention() != swallowtail_runtime::ProviderRetentionPolicy::Prohibited
        || request.policy().provider_recovery() != swallowtail_runtime::ProviderRecoveryPolicy::Prohibited
        || request.policy().stream_reattachment() != swallowtail_runtime::StreamReattachmentPolicy::Disabled
    {
        return Err(unsupported("provider tools, network, background, retention, recovery, or reattachment policy"));
    }
    if let Some(deadline) = request.deadline()
        && services.time().expect("validated time").now() >= deadline.instant()
    {
        return Err(failure(
            "swallowtail.kimi_platform.deadline_elapsed",
            "Kimi Platform deadline elapsed before provider work",
        ));
    }
    Ok(())
}

fn has_capability(plan: &PreflightPlan, capability: Capability) -> bool {
    plan.requirements().capabilities().any(|item| item.capability() == capability)
}

include!("pump.rs");
