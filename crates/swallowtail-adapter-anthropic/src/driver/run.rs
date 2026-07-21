use crate::protocol::{Event, parse_event, provider_failure};
use crate::transport::{StreamItem, Subscription};
use std::collections::BTreeMap;
use std::sync::Mutex;
use swallowtail_core::{ExternalNetworkPolicy, ExternalSearchPolicy, ProviderRequestRef};
use swallowtail_runtime::{
    OperationContent, ProviderObservation, RateLimitKind, RateLimitObservation, RunHandle,
    RuntimeEvent, RuntimeEventKind, RuntimeRunId, StructuredRunDriver, StructuredRunRequest,
    TerminalOutcome, TerminalStatus, TokenUsage, runtime_event_channel, terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 64;

impl StructuredRunDriver for AnthropicDirectDriver {
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
            let model = plan.model_id().expect("validated model").as_str().to_owned();
            let maximum = request
                .maximum_output_tokens()
                .expect("validated maximum")
                .get();
            let message = Request::message(&model, request.content(), maximum)?;
            let scope = operation_scope("run", request.request_id().as_str())?;
            let mut access = AccessLeases::acquire(&plan, scope.clone(), &services).await?;
            let cancelled = Arc::new(AtomicBool::new(false));
            let subscription = match self.transport.subscribe(
                scope.clone(),
                access.endpoint.clone(),
                access.secret()?.to_vec(),
                message,
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
                            .expect("Anthropic pending work lock poisoned")
                            .take()
                            .expect("Anthropic pending work is available");
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
                    let resources = {
                        pending
                            .lock()
                            .expect("Anthropic pending work lock poisoned")
                            .take()
                    };
                    if let Some((subscription, mut access)) = resources {
                        let _ = subscription.close().await;
                        let _ = access.release(&services).await;
                    }
                    return Err(error);
                }
            };
            let run_id = RuntimeRunId::new(format!(
                "anthropic-direct:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| failure("swallowtail.anthropic.run_id_invalid", "Anthropic runtime run id was invalid"))?;
            Ok(Box::new(AnthropicRunHandle::new(
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
    if plan.model_id().is_none()
        || plan.provider_id().is_some_and(|id| id.as_str() != PROVIDER_ID)
    {
        return Err(failure(
            "swallowtail.anthropic.model_binding_rejected",
            "Anthropic run requires an exact Anthropic model route",
        ));
    }
    if request.maximum_output_tokens().is_none()
        || !plan
            .requirements()
            .capabilities()
            .any(|requirement| requirement.capability() == Capability::OutputTokenLimit)
    {
        return Err(failure(
            "swallowtail.anthropic.output_limit_missing",
            "Anthropic run requires a preflight-bound maximum output-token input",
        ));
    }
    if request.working_resource().is_some() {
        return Err(unsupported("a working resource"));
    }
    if request.attachments().len() != 0 {
        return Err(unsupported("attachments"));
    }
    if request.structured_output().is_some() {
        return Err(unsupported("structured output"));
    }
    if request.policy().reasoning_mode().is_some()
        || request.policy().external_network() != ExternalNetworkPolicy::Denied
        || request.policy().external_search() != ExternalSearchPolicy::Disabled
    {
        return Err(unsupported("provider tools, reasoning, or external-network policy"));
    }
    if let Some(deadline) = request.deadline()
        && services.time().expect("validated time").now() >= deadline.instant()
    {
        return Err(failure(
            "swallowtail.anthropic.deadline_elapsed",
            "Anthropic deadline elapsed before provider work",
        ));
    }
    Ok(())
}

include!("pump.rs");
include!("observations.rs");
