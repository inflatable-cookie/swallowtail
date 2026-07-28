use super::AlibabaModelStudioDriver;
use super::access::AccessLeases;
use super::lifecycle::{cleanup_result, merge_cleanup};
use crate::failure::{failure, protocol, unsupported};
use crate::protocol::{ProviderEvent, ResponseStream, WireRequest};
use crate::transport::{StreamItem, Subscription};
use std::future::poll_fn;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::Poll;
use swallowtail_core::{
    CancellationScope, Capability, ExternalNetworkPolicy, ExternalSearchPolicy, PreflightPlan,
    RunRef, SafeDiagnostic,
};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CancellationAcknowledgement, CancellationControl, CleanupOutcome,
    DeadlineObservation, HostServices, JoinedTask, ProviderExecutionPolicy, ProviderObservation,
    ProviderRecoveryPolicy, ProviderRetentionPolicy, RequestId, RunHandle, RuntimeEvent,
    RuntimeEventKind, RuntimeFailure, RuntimeRunId, ScopeId, StreamReattachmentPolicy,
    StructuredRunDriver, StructuredRunRequest, TerminalOutcome, TerminalStatus,
    runtime_event_channel, terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 32;

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

struct AlibabaRunHandle {
    request_id: RequestId,
    run_id: RuntimeRunId,
    events: Option<BoxEventStream>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: Arc<RunCancellation>,
    task: Box<dyn JoinedTask>,
}

impl RunHandle for AlibabaRunHandle {
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
                        "swallowtail.alibaba_model_studio.task_join_failed",
                        "Alibaba Model Studio run task could not be joined",
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
    let mut provider = ResponseStream::default();
    let mut sequence = 1;
    let mut output = None;
    let status = loop {
        match next_signal(&mut subscription, &mut deadline).await {
            Signal::Deadline => {
                cancellation.cancelled.store(true, Ordering::SeqCst);
                break TerminalStatus::TimedOut;
            }
            Signal::Closed if cancellation.cancelled.load(Ordering::SeqCst) => {
                break TerminalStatus::Cancelled;
            }
            Signal::Closed => {
                break if output.is_some() {
                    TerminalStatus::Completed
                } else {
                    TerminalStatus::ProviderFailed(SafeDiagnostic::new(
                        "swallowtail.alibaba_model_studio.stream_disconnected",
                        "Alibaba Model Studio stream closed before completion",
                    ))
                };
            }
            Signal::Item(Err(_)) if cancellation.cancelled.load(Ordering::SeqCst) => {
                break TerminalStatus::Cancelled;
            }
            Signal::Item(Err(error)) => {
                break TerminalStatus::ProviderFailed(error.diagnostic().clone());
            }
            Signal::Item(Ok(StreamItem::Correlation(reference))) => {
                if let Err(error) = emit(
                    &events,
                    &mut sequence,
                    RuntimeEventKind::ProviderObservation(ProviderObservation::RequestCorrelation(
                        reference,
                    )),
                ) {
                    break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                }
            }
            Signal::Item(Ok(StreamItem::Frame(frame))) => match provider.apply(&frame) {
                Err(error) => {
                    break TerminalStatus::ProviderFailed(error.diagnostic().clone());
                }
                Ok(
                    ProviderEvent::Created(_)
                    | ProviderEvent::Progress(_)
                    | ProviderEvent::Unknown(_),
                ) => {
                    if let Err(error) = emit(&events, &mut sequence, RuntimeEventKind::Progress) {
                        break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                    }
                }
                Ok(ProviderEvent::TextDelta(content)) => {
                    if let Err(error) = events.send(RuntimeEvent::with_content(
                        sequence,
                        RuntimeEventKind::OutputDelta,
                        content,
                    )) {
                        break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                    }
                    sequence += 1;
                }
                Ok(ProviderEvent::TextDone(_)) => {}
                Ok(ProviderEvent::Completed {
                    output: completed,
                    usage,
                    ..
                }) => {
                    if let Err(error) = events.send(RuntimeEvent::with_content(
                        sequence,
                        RuntimeEventKind::OutputAvailable,
                        completed.clone(),
                    )) {
                        break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                    }
                    sequence += 1;
                    if let Err(error) = emit(
                        &events,
                        &mut sequence,
                        RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(usage)),
                    ) {
                        break TerminalStatus::RuntimeFailed(error.diagnostic().clone());
                    }
                    output = Some(completed);
                }
            },
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

fn emit(
    events: &swallowtail_runtime::RuntimeEventSender,
    sequence: &mut u64,
    kind: RuntimeEventKind,
) -> Result<(), RuntimeFailure> {
    events.send(RuntimeEvent::new(*sequence, kind))?;
    *sequence += 1;
    Ok(())
}
