use super::DeepSeekDirectDriver;
use super::access::AccessLeases;
use super::catalogue::{operation_scope, require_services};
use super::lifecycle::{cleanup_result, merge_cleanup};
use crate::failure::{failure, protocol, unsupported};
use crate::protocol::{
    FinalStreamParser, FinalStreamUpdate, HttpRequest, Usage, encode_structured,
};
use crate::selection::{DEEPSEEK_MODEL_ID, deepseek_v4_config};
use crate::transport::{StreamItem, Subscription};
use std::collections::BTreeMap;
use std::future::poll_fn;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::Poll;
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, ExternalNetworkPolicy,
    ExternalSearchPolicy, PreflightPlan, ProviderRequestRef, RunRef, SafeDiagnostic,
};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CancellationAcknowledgement, CancellationControl, CleanupOutcome,
    DeadlineObservation, HostServices, JoinedTask, OperationContent, ProviderExecutionPolicy,
    ProviderObservation, ProviderRecoveryPolicy, ProviderRetentionPolicy, RequestId, RunHandle,
    RuntimeEvent, RuntimeEventKind, RuntimeFailure, RuntimeRunId, StreamReattachmentPolicy,
    StructuredRunDriver, StructuredRunRequest, TerminalOutcome, TerminalStatus, TokenUsage,
    runtime_event_channel, terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 32;

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
            validate_run(&plan, &request, &services)?;
            let maximum = request
                .maximum_output_tokens()
                .expect("validated maximum")
                .get();
            let body = encode_structured(request.content().as_str(), maximum).map_err(protocol)?;
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
    let reasoning = request
        .policy()
        .reasoning_mode()
        .ok_or_else(|| unsupported("an omitted reasoning selection"))?;
    if reasoning.as_str() != "high"
        || !plan.requirements().capabilities().any(|requirement| {
            requirement.capability() == Capability::ReasoningSelection
                && requirement.constraints().any(|constraint| {
                    matches!(
                        constraint,
                        CapabilityConstraint::ReasoningMode(mode) if mode == reasoning
                    )
                })
        })
    {
        return Err(unsupported("a reasoning selection other than high"));
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

struct RunCancellation {
    cancelled: Arc<AtomicBool>,
}

impl CancellationControl for RunCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::StructuredRun
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let requested = !self.cancelled.swap(true, Ordering::SeqCst);
        Box::pin(async move {
            Ok(if requested {
                CancellationAcknowledgement::Requested
            } else {
                CancellationAcknowledgement::AlreadyRequested
            })
        })
    }
}

struct DeepSeekRunHandle {
    request_id: RequestId,
    run_id: RuntimeRunId,
    events: Option<BoxEventStream>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: Arc<RunCancellation>,
    task: Box<dyn JoinedTask>,
}

impl RunHandle for DeepSeekRunHandle {
    fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    fn run_id(&self) -> &RuntimeRunId {
        &self.run_id
    }

    fn provider_run_ref(&self) -> Option<&RunRef> {
        None
    }

    fn take_events(&mut self) -> Option<BoxEventStream> {
        self.events.take()
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        self.cancellation.as_ref()
    }

    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>> {
        self.terminal.take()
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            self.cancellation.cancelled.store(true, Ordering::SeqCst);
            self.task.join().await.map_or_else(
                |_| {
                    CleanupOutcome::Failed(SafeDiagnostic::new(
                        "swallowtail.deepseek.task_join_failed",
                        "DeepSeek run task could not be joined",
                    ))
                },
                |_| CleanupOutcome::Clean,
            )
        })
    }
}

async fn pump_run(
    mut subscription: Subscription,
    mut access: AccessLeases,
    services: HostServices,
    events: swallowtail_runtime::RuntimeEventSender,
    cancellation: Arc<RunCancellation>,
    mut deadline: Option<BoxFuture<'static, DeadlineObservation>>,
) -> TerminalOutcome {
    let mut parser = FinalStreamParser::new(&deepseek_v4_config());
    let mut sequence = 1;
    let mut output = None;
    let status = 'pump: loop {
        match next_signal(&mut subscription, &mut deadline).await {
            Signal::Deadline => {
                cancellation.cancelled.store(true, Ordering::SeqCst);
                break TerminalStatus::TimedOut;
            }
            Signal::Closed if cancellation.cancelled.load(Ordering::SeqCst) => {
                break TerminalStatus::Cancelled;
            }
            Signal::Closed => match parser.finish().map_err(protocol) {
                Ok(final_attempt) => {
                    let content = match OperationContent::new(final_attempt.output) {
                        Ok(content) => content,
                        Err(_) => {
                            break TerminalStatus::RuntimeFailed(SafeDiagnostic::new(
                                "swallowtail.deepseek.output_invalid",
                                "DeepSeek emitted invalid output content",
                            ));
                        }
                    };
                    if let Err(error) = events.send(RuntimeEvent::with_content(
                        sequence,
                        RuntimeEventKind::OutputAvailable,
                        content.clone(),
                    )) {
                        break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                    }
                    output = Some(content);
                    break TerminalStatus::Completed;
                }
                Err(error) => break TerminalStatus::ProviderFailed(error.diagnostic().clone()),
            },
            Signal::Item(Err(_)) if cancellation.cancelled.load(Ordering::SeqCst) => {
                break TerminalStatus::Cancelled;
            }
            Signal::Item(Err(error)) => {
                break TerminalStatus::ProviderFailed(error.diagnostic().clone());
            }
            Signal::Item(Ok(StreamItem::Metadata(headers))) => {
                if let Err(error) = emit_request(&events, &mut sequence, &headers) {
                    break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                }
            }
            Signal::Item(Ok(StreamItem::Data(bytes))) => {
                match parser.push(&bytes).map_err(protocol) {
                    Err(error) => break TerminalStatus::ProviderFailed(error.diagnostic().clone()),
                    Ok(updates) => {
                        for update in updates {
                            if let Err(error) = emit_update(&events, &mut sequence, update) {
                                break 'pump TerminalStatus::RuntimeFailed(
                                    error.diagnostic().clone(),
                                );
                            }
                        }
                    }
                }
            }
        }
    };
    let stream_cleanup = cleanup_result(subscription.close().await);
    let credential_cleanup = access.release(&services).await;
    let cleanup = merge_cleanup(stream_cleanup, credential_cleanup);
    let mut outcome = TerminalOutcome::new(status, cleanup);
    if matches!(outcome.status(), TerminalStatus::Completed)
        && let Some(output) = output
    {
        outcome = outcome.with_output(output);
    }
    outcome
}

fn emit_request(
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    headers: &BTreeMap<String, String>,
) -> Result<(), RuntimeFailure> {
    let Some(value) = headers.get("x-request-id") else {
        return Ok(());
    };
    let reference = ProviderRequestRef::new(value.clone()).map_err(|_| {
        failure(
            "swallowtail.deepseek.request_id_invalid",
            "DeepSeek request correlation was invalid",
        )
    })?;
    emit(
        events,
        sequence,
        RuntimeEventKind::ProviderObservation(ProviderObservation::RequestCorrelation(reference)),
    )
}

fn emit_update(
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    update: FinalStreamUpdate,
) -> Result<(), RuntimeFailure> {
    match update {
        FinalStreamUpdate::Output(delta) => {
            let content = OperationContent::new(delta).map_err(|_| {
                failure(
                    "swallowtail.deepseek.output_invalid",
                    "DeepSeek emitted invalid output content",
                )
            })?;
            events.send(RuntimeEvent::with_content(
                *sequence,
                RuntimeEventKind::OutputDelta,
                content,
            ))?;
            *sequence += 1;
            Ok(())
        }
        FinalStreamUpdate::Usage(usage) => emit_usage(events, sequence, usage),
        FinalStreamUpdate::Finished(_) => emit(events, sequence, RuntimeEventKind::Progress),
    }
}

fn emit_usage(
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    usage: Usage,
) -> Result<(), RuntimeFailure> {
    if usage.prompt_tokens.saturating_add(usage.completion_tokens) != usage.total_tokens {
        return Err(failure(
            "swallowtail.deepseek.usage_invalid",
            "DeepSeek usage totals were inconsistent",
        ));
    }
    let usage = TokenUsage::new(Some(usage.prompt_tokens), Some(usage.completion_tokens))
        .with_cache_tokens(Some(usage.cache_hit_tokens), None)
        .with_cache_miss_input_tokens(Some(usage.cache_miss_tokens));
    emit(
        events,
        sequence,
        RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(usage)),
    )
}

fn emit(
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    kind: RuntimeEventKind,
) -> Result<(), RuntimeFailure> {
    events.send(RuntimeEvent::new(*sequence, kind))?;
    *sequence += 1;
    Ok(())
}

enum Signal {
    Item(Result<StreamItem, RuntimeFailure>),
    Closed,
    Deadline,
}

async fn next_signal(
    subscription: &mut Subscription,
    deadline: &mut Option<BoxFuture<'static, DeadlineObservation>>,
) -> Signal {
    poll_fn(|context| {
        if let Poll::Ready(item) = subscription.poll_next(context) {
            return Poll::Ready(item.map_or(Signal::Closed, Signal::Item));
        }
        if let Some(deadline) = deadline
            && deadline.as_mut().poll(context).is_ready()
        {
            return Poll::Ready(Signal::Deadline);
        }
        Poll::Pending
    })
    .await
}
