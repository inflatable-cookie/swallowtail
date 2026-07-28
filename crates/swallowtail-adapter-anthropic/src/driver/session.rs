use super::{AccessLeases, AnthropicDirectDriver, operation_scope, require_services};
use crate::failure::{failure, unsupported};
use crate::protocol::{ContentBlock, Event, Request, ToolSpec, parse_event, provider_failure};
use crate::transport::{StreamItem, Subscription};
use futures_channel::{mpsc, oneshot};
use std::collections::{BTreeMap, BTreeSet};
use std::future::poll_fn;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::Poll;
use swallowtail_core::{CancellationScope, PreflightPlan, ProviderRequestRef, SessionRef, TurnRef};
use swallowtail_runtime::{
    BoxDirectToolCallStream, BoxEventStream, BoxFuture, CancellationAcknowledgement,
    CancellationControl, CleanupOutcome, DirectAttemptFinishObservation,
    DirectAttemptUsageObservation, DirectContinuationState, DirectContinuationTurnRequest,
    DirectInferenceAttempt, DirectToolArguments, DirectToolCall, DirectToolCallId,
    DirectToolExchange, DirectToolResult, DirectToolResultSubmitter, HostServices,
    InteractiveSessionDriver, InteractiveSessionHandle, JoinedTask,
    OpenDirectContinuationSessionRequest, OpenSessionRequest, OperationContent,
    ProviderFinishReason, ProviderObservation, RequestId, ResumeSessionRequest, RuntimeEvent,
    RuntimeEventKind, RuntimeEventSender, RuntimeFailure, RuntimeSessionId, RuntimeTurnId,
    SessionResumeBinding, TerminalOutcome, TerminalStatus, TokenUsage, TurnHandle, TurnRequest,
    runtime_event_channel, terminal_outcome_channel, validate_direct_continuation_plan,
};

const EVENT_CAPACITY: usize = 32;

struct SessionHandle {
    request_id: RequestId,
    runtime_id: RuntimeSessionId,
    plan: PreflightPlan,
    scope: swallowtail_runtime::ScopeId,
    services: HostServices,
    transport: crate::transport::CurlTransport,
    endpoint: String,
    access: Option<AccessLeases>,
    tools: Arc<Vec<ToolSpec>>,
    state: Arc<Mutex<DirectContinuationState>>,
    history: Arc<Mutex<History>>,
    usable: Arc<AtomicBool>,
    active: ActiveSlot,
    cancellation: Arc<SessionCancellation>,
}

struct ActiveTurn {
    turn_id: RuntimeTurnId,
    task: Option<Box<dyn JoinedTask>>,
    cancellation: Arc<TurnCancellation>,
    terminal: Arc<AtomicBool>,
}

type ActiveSlot = Arc<Mutex<Option<ActiveTurn>>>;

impl InteractiveSessionDriver for AnthropicDirectDriver {
    fn open_session(
        &self,
        _plan: PreflightPlan,
        _request: OpenSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async { Err(unsupported("the generic session-open request")) })
    }

    fn resume_session(
        &self,
        _plan: PreflightPlan,
        _request: ResumeSessionRequest,
        _services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        Box::pin(async { Err(unsupported("session resume")) })
    }

    fn open_direct_continuation_session(
        &self,
        plan: PreflightPlan,
        request: OpenDirectContinuationSessionRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn InteractiveSessionHandle>, RuntimeFailure>> {
        let transport = self.transport.clone();
        Box::pin(async move {
            Self::validate_plan(&plan)?;
            services.require_execution_host(plan.execution_host_id())?;
            validate_direct_continuation_plan(&plan, &request)?;
            require_services(&services, true, false)?;
            if request.options().developer_instructions().is_some()
                || request.options().reasoning_mode().is_some()
                || request.options().tools().len() == 0
            {
                return Err(unsupported("the requested direct-session options"));
            }
            let tools = tool_specs(&request)?;
            let scope = operation_scope("session", request.request_id().as_str())?;
            let runtime_id = RuntimeSessionId::new(format!(
                "anthropic-direct:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| {
                failure(
                    "swallowtail.anthropic.session_id_invalid",
                    "Anthropic runtime session identity was invalid",
                )
            })?;
            let access = AccessLeases::acquire(&plan, scope.clone(), &services).await?;
            let endpoint = access.endpoint.clone();
            let config = request.config().clone();
            let active = Arc::new(Mutex::new(None));
            let usable = Arc::new(AtomicBool::new(true));
            let cancellation = Arc::new(SessionCancellation {
                active: Arc::clone(&active),
                usable: Arc::clone(&usable),
                requested: AtomicBool::new(false),
            });
            Ok(Box::new(SessionHandle {
                request_id: request.request_id().clone(),
                runtime_id,
                plan,
                scope,
                services,
                transport,
                endpoint,
                access: Some(access),
                tools: Arc::new(tools),
                state: Arc::new(Mutex::new(DirectContinuationState::new(config.clone()))),
                history: Arc::new(Mutex::new(History::new(
                    config.maximum_private_history_bytes().get(),
                ))),
                usable,
                active,
                cancellation,
            }) as Box<dyn InteractiveSessionHandle>)
        })
    }
}

impl InteractiveSessionHandle for SessionHandle {
    fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    fn session_id(&self) -> &RuntimeSessionId {
        &self.runtime_id
    }

    fn provider_session_ref(&self) -> Option<&SessionRef> {
        None
    }

    fn resume_binding(&self) -> Option<&SessionResumeBinding> {
        None
    }

    fn start_turn<'a>(
        &'a mut self,
        _request: TurnRequest,
        _services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn TurnHandle>, RuntimeFailure>> {
        Box::pin(async { Err(unsupported("the generic turn request")) })
    }

    fn start_direct_continuation_turn<'a>(
        &'a mut self,
        request: DirectContinuationTurnRequest,
        services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn TurnHandle>, RuntimeFailure>> {
        Box::pin(async move { self.start_direct_turn(request, services).await })
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        self.cancellation.as_ref()
    }

    fn close(mut self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            self.usable.store(false, Ordering::SeqCst);
            self.state
                .lock()
                .expect("continuation state lock poisoned")
                .invalidate();
            let active = close_active(&self.active).await;
            self.history.lock().expect("history lock poisoned").clear();
            let credential = match self.access.as_mut() {
                Some(access) => access.release(&self.services).await,
                None => CleanupOutcome::NotApplicable,
            };
            super::merge_cleanup(active, credential)
        })
    }
}

struct TurnHandleImpl {
    runtime_id: RuntimeTurnId,
    events: Option<BoxEventStream>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    exchange: Option<DirectToolExchange>,
    cancellation: Arc<TurnCancellation>,
    terminal_flag: Arc<AtomicBool>,
    active: ActiveSlot,
}

impl SessionHandle {
    async fn start_direct_turn(
        &mut self,
        request: DirectContinuationTurnRequest,
        services: HostServices,
    ) -> Result<Box<dyn TurnHandle>, RuntimeFailure> {
        services.require_execution_host(self.services.execution_host_id())?;
        reap_finished(&self.active).await?;
        if self
            .active
            .lock()
            .expect("active turn lock poisoned")
            .is_some()
        {
            return Err(failure(
                "swallowtail.anthropic.turn_active",
                "Anthropic session already has an active turn",
            ));
        }
        if !self.usable.load(Ordering::SeqCst)
            || self.services.time().expect("validated time").now() >= request.deadline().instant()
        {
            return Err(failure(
                "swallowtail.anthropic.session_unavailable",
                "Anthropic session cannot start the requested turn",
            ));
        }
        let attempt = self
            .state
            .lock()
            .expect("continuation state lock poisoned")
            .authorize_user_turn(&request)?;
        let messages = build_user_messages(&self.history, &request, &attempt)?;
        let wire = Request::direct_message(
            self.plan.model_id().expect("validated model").as_str(),
            messages,
            &self.tools,
            self.plan
                .requirements()
                .direct_continuation()
                .expect("validated continuation")
                .config()
                .maximum_output_tokens_per_attempt()
                .get(),
        )?;
        let (events, stream) = runtime_event_channel(EVENT_CAPACITY)?;
        events.send(RuntimeEvent::new(0, RuntimeEventKind::Started))?;
        let cancelled = Arc::new(AtomicBool::new(false));
        let (control, cancel_receiver) =
            TurnCancellation::new(Arc::clone(&cancelled), Arc::clone(&self.usable));
        let control = Arc::new(control);
        let (call_sender, exchange, submitter, result_receiver) = if attempt.ordinal().get() == 1 {
            let (sender, receiver) = mpsc::channel(1);
            let (submitter, results) = ResultSubmitter::new();
            let submitter = Arc::new(submitter);
            (
                Some(sender),
                Some(DirectToolExchange::new(
                    Box::pin(receiver) as BoxDirectToolCallStream,
                    Arc::clone(&submitter) as Arc<dyn DirectToolResultSubmitter>,
                )),
                Some(submitter),
                Some(results),
            )
        } else {
            (None, None, None, None)
        };
        let credential = SecretBytes(
            self.access
                .as_ref()
                .expect("session access exists")
                .secret()?
                .to_vec(),
        );
        let context = TurnContext {
            plan: self.plan.clone(),
            scope: self.scope.clone(),
            services: self.services.clone(),
            transport: self.transport.clone(),
            endpoint: self.endpoint.clone(),
            tools: Arc::clone(&self.tools),
            state: Arc::clone(&self.state),
            history: Arc::clone(&self.history),
            usable: Arc::clone(&self.usable),
            cancelled,
            cancel_receiver,
        };
        let work = TurnWork {
            request: request.clone(),
            attempt,
            wire,
            credential,
            call_sender,
            submitter,
            result_receiver,
        };
        let (terminal_sender, terminal) = terminal_outcome_channel();
        let terminal_flag = Arc::new(AtomicBool::new(false));
        let task_terminal = Arc::clone(&terminal_flag);
        let task_control = Arc::clone(&control);
        let task = self.services.task().expect("validated task").spawn(
            self.scope.clone(),
            Box::pin(async move {
                let outcome = run_turn(work, context, events.clone(), task_control).await;
                events.mark_terminal();
                task_terminal.store(true, Ordering::SeqCst);
                let _ = terminal_sender.complete(outcome);
            }),
        )?;
        *self.active.lock().expect("active turn lock poisoned") = Some(ActiveTurn {
            turn_id: request.turn_id().clone(),
            task: Some(task),
            cancellation: Arc::clone(&control),
            terminal: Arc::clone(&terminal_flag),
        });
        Ok(Box::new(TurnHandleImpl {
            runtime_id: request.turn_id().clone(),
            events: Some(Box::pin(stream)),
            terminal: Some(Box::pin(terminal)),
            exchange,
            cancellation: control,
            terminal_flag,
            active: Arc::clone(&self.active),
        }))
    }
}

impl TurnHandle for TurnHandleImpl {
    fn turn_id(&self) -> &RuntimeTurnId {
        &self.runtime_id
    }

    fn provider_turn_ref(&self) -> Option<&TurnRef> {
        None
    }

    fn take_events(&mut self) -> Option<BoxEventStream> {
        self.events.take()
    }

    fn take_direct_tool_exchange(&mut self) -> Option<DirectToolExchange> {
        self.exchange.take()
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        self.cancellation.as_ref()
    }

    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>> {
        self.terminal.take()
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            if !self.terminal_flag.load(Ordering::SeqCst) {
                let _ = self.cancellation.request().await;
            }
            join_turn(&self.active, &self.runtime_id).await
        })
    }
}

struct TurnWork {
    request: DirectContinuationTurnRequest,
    attempt: DirectInferenceAttempt,
    wire: Request,
    credential: SecretBytes,
    call_sender: Option<mpsc::Sender<Result<DirectToolCall, RuntimeFailure>>>,
    submitter: Option<Arc<ResultSubmitter>>,
    result_receiver: Option<oneshot::Receiver<Vec<DirectToolResult>>>,
}

struct TurnContext {
    plan: PreflightPlan,
    scope: swallowtail_runtime::ScopeId,
    services: HostServices,
    transport: crate::transport::CurlTransport,
    endpoint: String,
    tools: Arc<Vec<ToolSpec>>,
    state: Arc<Mutex<DirectContinuationState>>,
    history: Arc<Mutex<History>>,
    usable: Arc<AtomicBool>,
    cancelled: Arc<AtomicBool>,
    cancel_receiver: oneshot::Receiver<()>,
}

async fn run_turn(
    mut work: TurnWork,
    mut context: TurnContext,
    events: RuntimeEventSender,
    cancellation: Arc<TurnCancellation>,
) -> TerminalOutcome {
    let mut sequence = 1;
    let mut deadline = context
        .services
        .time()
        .expect("validated time")
        .wait_until(work.request.deadline());
    let result = run_attempt(
        &work.attempt,
        work.wire.clone(),
        &work,
        &mut context,
        &events,
        &mut sequence,
        &mut deadline,
        &cancellation,
    )
    .await;
    let result = match result {
        Ok(AttemptOutcome::Tool(call)) if work.attempt.ordinal().get() == 1 => {
            if !context
                .tools
                .iter()
                .any(|tool| tool.name == call.tool_name())
            {
                return provider_terminal(
                    failure(
                        "swallowtail.anthropic.tool_undeclared",
                        "Anthropic returned an undeclared consumer tool",
                    ),
                    &context,
                );
            }
            let pause = context
                .state
                .lock()
                .expect("continuation state lock poisoned")
                .pause_for_tool_calls(&work.attempt, std::slice::from_ref(&call));
            if let Err(error) = pause {
                Err(TurnFailure::Runtime(error, CleanupOutcome::Clean))
            } else {
                async {
                context
                    .history
                    .lock()
                    .expect("history lock poisoned")
                    .record_tool(&work.request, &call)
                    .and_then(|_| {
                        work.submitter
                            .as_ref()
                            .expect("first attempt has submitter")
                            .open(call.call_id().clone())
                    })
                    .map_err(|error| TurnFailure::Runtime(error, CleanupOutcome::Clean))
                    .and_then(|_| {
                        work.call_sender
                            .as_mut()
                            .expect("first attempt has call sender")
                            .try_send(Ok(call.clone()))
                            .map_err(|_| {
                                TurnFailure::Runtime(
                                    failure(
                                        "swallowtail.anthropic.tool_channel_failed",
                                        "Anthropic tool-call channel could not deliver the pending call",
                                    ),
                                    CleanupOutcome::Clean,
                                )
                            })
                    })
                    .map(|_| ())
                    .and_then(|_| {
                        emit(
                            &events,
                            &mut sequence,
                            RuntimeEventKind::DirectToolCallAvailable(call.call_id().clone()),
                        )
                        .map_err(|error| TurnFailure::Runtime(error, CleanupOutcome::Clean))
                    })?;
                work.call_sender.take();
                match wait_results(
                    work.result_receiver
                        .as_mut()
                        .expect("first attempt has result receiver"),
                    &mut context.cancel_receiver,
                    &mut deadline,
                    &cancellation,
                )
                .await
                {
                    Err(stop) => Err(TurnFailure::Stopped(stop, CleanupOutcome::Clean)),
                    Ok(results) => {
                        let maximum_result = context
                            .plan
                            .requirements()
                            .direct_continuation()
                            .expect("validated continuation")
                            .config()
                            .maximum_tool_result_bytes()
                            .get() as usize;
                        if results
                            .iter()
                            .any(|result| result.content().byte_len() > maximum_result)
                        {
                            return Err(TurnFailure::Runtime(
                                failure(
                                    "swallowtail.anthropic.tool_result_exceeded",
                                    "Anthropic tool result exceeded the selected bound",
                                ),
                                CleanupOutcome::Clean,
                            ));
                        }
                        let next = context
                            .state
                            .lock()
                            .expect("continuation state lock poisoned")
                            .authorize_tool_results(&results)
                            .map_err(|error| {
                                TurnFailure::Runtime(error, CleanupOutcome::Clean)
                            })?;
                        context
                            .history
                            .lock()
                            .expect("history lock poisoned")
                            .record_result(results.first().expect("exact result exists"))
                            .map_err(|error| {
                                TurnFailure::Runtime(error, CleanupOutcome::Clean)
                            })?;
                        let messages = context
                            .history
                            .lock()
                            .expect("history lock poisoned")
                            .continuation_messages()
                            .map_err(|error| {
                                TurnFailure::Runtime(error, CleanupOutcome::Clean)
                            })?;
                        let wire = Request::direct_message(
                            context.plan.model_id().expect("validated model").as_str(),
                            messages,
                            &context.tools,
                            context
                                .plan
                                .requirements()
                                .direct_continuation()
                                .expect("validated continuation")
                                .config()
                                .maximum_output_tokens_per_attempt()
                                .get(),
                        )
                        .map_err(|error| TurnFailure::Runtime(error, CleanupOutcome::Clean))?;
                        context.cancelled.store(false, Ordering::SeqCst);
                        run_attempt(
                            &next,
                            wire,
                            &work,
                            &mut context,
                            &events,
                            &mut sequence,
                            &mut deadline,
                            &cancellation,
                        )
                        .await
                    }
                }
                }
                .await
            }
        }
        Ok(AttemptOutcome::Tool(_)) => Err(TurnFailure::Provider(
            failure(
                "swallowtail.anthropic.tool_call_unexpected",
                "Anthropic returned a tool call outside the qualified exchange",
            ),
            CleanupOutcome::Clean,
        )),
        other => other,
    };
    if result.is_err()
        && let Some(submitter) = work.submitter.as_ref()
    {
        submitter.abandon();
    }
    match result {
        Ok(AttemptOutcome::Final(output)) => {
            let completed = {
                let mut history = context.history.lock().expect("history lock poisoned");
                if work.attempt.ordinal().get() == 1 {
                    history.record_answer(&output)
                } else {
                    Ok(())
                }
            }
            .and_then(|_| {
                context
                    .state
                    .lock()
                    .expect("continuation state lock poisoned")
                    .complete_turn()
            });
            if let Err(error) = completed {
                context.usable.store(false, Ordering::SeqCst);
                return TerminalOutcome::new(
                    TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                    CleanupOutcome::Clean,
                );
            }
            TerminalOutcome::new(TerminalStatus::Completed, CleanupOutcome::Clean)
                .with_output(OperationContent::new(output).expect("final output is nonempty"))
        }
        Ok(AttemptOutcome::Tool(_)) => unreachable!(),
        Err(TurnFailure::Stopped(stop, cleanup)) => {
            invalidate(&context);
            TerminalOutcome::new(
                if stop == StopSignal::TimedOut {
                    TerminalStatus::TimedOut
                } else {
                    TerminalStatus::Cancelled
                },
                cleanup,
            )
        }
        Err(TurnFailure::Provider(error, cleanup)) => {
            invalidate(&context);
            TerminalOutcome::new(
                TerminalStatus::ProviderFailed(error.diagnostic().clone()),
                cleanup,
            )
        }
        Err(TurnFailure::Runtime(error, cleanup)) => {
            invalidate(&context);
            TerminalOutcome::new(
                TerminalStatus::RuntimeFailed(error.diagnostic().clone()),
                cleanup,
            )
        }
    }
}

fn provider_terminal(error: RuntimeFailure, context: &TurnContext) -> TerminalOutcome {
    invalidate(context);
    TerminalOutcome::new(
        TerminalStatus::ProviderFailed(error.diagnostic().clone()),
        CleanupOutcome::Clean,
    )
}

#[allow(clippy::too_many_arguments)]
async fn run_attempt(
    attempt: &DirectInferenceAttempt,
    wire: Request,
    work: &TurnWork,
    context: &mut TurnContext,
    events: &RuntimeEventSender,
    sequence: &mut u64,
    deadline: &mut BoxFuture<'static, swallowtail_runtime::DeadlineObservation>,
    cancellation: &TurnCancellation,
) -> Result<AttemptOutcome, TurnFailure> {
    let mut subscription = context
        .transport
        .subscribe(
            context.scope.clone(),
            context.endpoint.clone(),
            work.credential.copy(),
            wire,
            &context.services,
            Arc::clone(&context.cancelled),
        )
        .map_err(|error| TurnFailure::Provider(error, CleanupOutcome::NotApplicable))?;
    let mut parser = AttemptParser::new(
        attempt.attempt_id().clone(),
        context
            .plan
            .requirements()
            .direct_continuation()
            .expect("validated continuation")
            .config()
            .maximum_tool_argument_bytes()
            .get() as usize,
    );
    loop {
        match next_signal(
            &mut subscription,
            &mut context.cancel_receiver,
            deadline,
            cancellation,
        )
        .await
        {
            StreamSignal::Item(Ok(StreamItem::Headers(headers))) => {
                emit_request(events, sequence, &headers)
                    .map_err(|error| TurnFailure::Runtime(error, CleanupOutcome::Clean))?;
            }
            StreamSignal::Item(Ok(StreamItem::Frame(frame))) => {
                let event = parse_event(&frame)
                    .map_err(|error| TurnFailure::Provider(error, CleanupOutcome::Clean))?;
                parser
                    .apply(event, events, sequence)
                    .map_err(|error| TurnFailure::Provider(error, CleanupOutcome::Clean))?;
            }
            StreamSignal::Item(Err(error)) => {
                let cleanup = cleanup_result(subscription.close().await);
                return Err(if cancellation.is_requested() {
                    TurnFailure::Stopped(cancellation.stop_reason(), cleanup)
                } else {
                    TurnFailure::Provider(error, cleanup)
                });
            }
            StreamSignal::Closed => {
                let cleanup = cleanup_result(subscription.close().await);
                return parser
                    .finish()
                    .map_err(|error| TurnFailure::Provider(error, cleanup));
            }
            StreamSignal::Stopped(stop) => {
                let cleanup = cleanup_result(subscription.close().await);
                return Err(TurnFailure::Stopped(stop, cleanup));
            }
        }
    }
}

enum AttemptOutcome {
    Tool(DirectToolCall),
    Final(String),
}

struct AttemptParser {
    attempt_id: swallowtail_runtime::DirectInferenceAttemptId,
    maximum_arguments: usize,
    started: bool,
    active: Option<ContentBlock>,
    tool_id: Option<String>,
    tool_name: Option<String>,
    arguments: String,
    output: String,
    stop_reason: Option<String>,
    stopped: bool,
}

impl AttemptParser {
    fn new(
        attempt_id: swallowtail_runtime::DirectInferenceAttemptId,
        maximum_arguments: usize,
    ) -> Self {
        Self {
            attempt_id,
            maximum_arguments,
            started: false,
            active: None,
            tool_id: None,
            tool_name: None,
            arguments: String::new(),
            output: String::new(),
            stop_reason: None,
            stopped: false,
        }
    }

    fn apply(
        &mut self,
        event: Event,
        events: &RuntimeEventSender,
        sequence: &mut u64,
    ) -> Result<(), RuntimeFailure> {
        match event {
            Event::Unknown => Ok(()),
            Event::Ping => emit(events, sequence, RuntimeEventKind::Keepalive),
            Event::MessageStart(usage) if !self.started => {
                self.started = true;
                emit_attempt_usage(events, sequence, &self.attempt_id, usage)
            }
            Event::ContentStart(block) if self.started && self.active.is_none() => {
                if let ContentBlock::ToolUse { id, name } = &block {
                    self.tool_id = Some(id.clone());
                    self.tool_name = Some(name.clone());
                }
                if matches!(block, ContentBlock::SearchUse | ContentBlock::SearchResult) {
                    return Err(failure(
                        "swallowtail.anthropic.provider_tool_unexpected",
                        "Anthropic provider-owned search appeared in a consumer-tool session",
                    ));
                }
                self.active = Some(block);
                Ok(())
            }
            Event::OutputDelta(delta) if self.active == Some(ContentBlock::Text) => {
                self.output.push_str(&delta);
                emit_content(events, sequence, RuntimeEventKind::OutputDelta, delta)
            }
            Event::InputJsonDelta(delta)
                if matches!(self.active, Some(ContentBlock::ToolUse { .. })) =>
            {
                if self.arguments.len().saturating_add(delta.len()) > self.maximum_arguments {
                    return Err(failure(
                        "swallowtail.anthropic.tool_arguments_exceeded",
                        "Anthropic tool arguments exceeded the selected bound",
                    ));
                }
                self.arguments.push_str(&delta);
                Ok(())
            }
            Event::ContentStop if self.active.take().is_some() => Ok(()),
            Event::Usage(usage, reason) if self.active.is_none() => {
                self.stop_reason = Some(reason.clone());
                emit_attempt_usage(events, sequence, &self.attempt_id, usage)?;
                let finish = match reason.as_str() {
                    "tool_use" => return Ok(()),
                    "end_turn" | "stop_sequence" => ProviderFinishReason::Stop,
                    "max_tokens" => ProviderFinishReason::Length,
                    _ => {
                        return Err(failure(
                            "swallowtail.anthropic.finish_reason_invalid",
                            "Anthropic finish reason was not qualified",
                        ));
                    }
                };
                emit(
                    events,
                    sequence,
                    RuntimeEventKind::ProviderObservation(
                        ProviderObservation::DirectAttemptFinish(
                            DirectAttemptFinishObservation::new(self.attempt_id.clone(), finish),
                        ),
                    ),
                )
            }
            Event::MessageStop if self.stop_reason.is_some() => {
                self.stopped = true;
                Ok(())
            }
            Event::ProviderFailed(kind) => Err(provider_failure(kind, "message stream")),
            _ => Err(failure(
                "swallowtail.anthropic.stream_order_invalid",
                "Anthropic direct-continuation stream order was invalid",
            )),
        }
    }

    fn finish(self) -> Result<AttemptOutcome, RuntimeFailure> {
        if !self.stopped {
            return Err(failure(
                "swallowtail.anthropic.stream_disconnected",
                "Anthropic stream closed before message completion",
            ));
        }
        match self.stop_reason.as_deref() {
            Some("tool_use") if self.output.is_empty() => {
                let arguments =
                    DirectToolArguments::new(self.arguments.into_bytes(), self.maximum_arguments)
                        .map_err(|_| {
                        failure(
                            "swallowtail.anthropic.tool_arguments_exceeded",
                            "Anthropic tool arguments exceeded the selected bound",
                        )
                    })?;
                serde_json::from_slice::<serde_json::Value>(arguments.as_bytes()).map_err(
                    |_| {
                        failure(
                            "swallowtail.anthropic.tool_arguments_invalid",
                            "Anthropic tool arguments were not valid JSON",
                        )
                    },
                )?;
                Ok(AttemptOutcome::Tool(
                    DirectToolCall::new(
                        DirectToolCallId::new(self.tool_id.ok_or_else(|| {
                            failure(
                                "swallowtail.anthropic.tool_call_invalid",
                                "Anthropic tool call identity was missing",
                            )
                        })?)
                        .map_err(|_| {
                            failure(
                                "swallowtail.anthropic.tool_call_invalid",
                                "Anthropic tool call identity was invalid",
                            )
                        })?,
                        self.attempt_id,
                        self.tool_name.ok_or_else(|| {
                            failure(
                                "swallowtail.anthropic.tool_call_invalid",
                                "Anthropic tool call name was missing",
                            )
                        })?,
                        arguments,
                    )
                    .map_err(|_| {
                        failure(
                            "swallowtail.anthropic.tool_call_invalid",
                            "Anthropic tool call was invalid",
                        )
                    })?,
                ))
            }
            Some("end_turn" | "stop_sequence" | "max_tokens")
                if self.tool_id.is_none() && !self.output.is_empty() =>
            {
                Ok(AttemptOutcome::Final(self.output))
            }
            _ => Err(failure(
                "swallowtail.anthropic.attempt_semantics_invalid",
                "Anthropic attempt completion did not match the selected operation",
            )),
        }
    }
}

fn tool_specs(
    request: &OpenDirectContinuationSessionRequest,
) -> Result<Vec<ToolSpec>, RuntimeFailure> {
    request
        .options()
        .tools()
        .map(|tool| {
            if tool.schema_media_type() != "application/schema+json"
                || tool.schema_dialect() != "json-schema-2020-12"
            {
                return Err(unsupported("non-JSON-Schema tool declarations"));
            }
            let bytes = tool
                .input_schema()
                .inline_bytes()
                .ok_or_else(|| unsupported("referenced tool schemas"))?;
            let input_schema = serde_json::from_slice(bytes).map_err(|_| {
                failure(
                    "swallowtail.anthropic.tool_schema_invalid",
                    "Anthropic tool schema was not valid JSON",
                )
            })?;
            Ok(ToolSpec {
                name: tool.name().to_owned(),
                description: tool
                    .description()
                    .map_or_else(String::new, |value| value.as_str().to_owned()),
                input_schema,
            })
        })
        .collect()
}

fn build_user_messages(
    history: &Arc<Mutex<History>>,
    request: &DirectContinuationTurnRequest,
    attempt: &DirectInferenceAttempt,
) -> Result<serde_json::Value, RuntimeFailure> {
    match attempt.ordinal().get() {
        1 => Ok(serde_json::json!([{
            "role": "user",
            "content": request.content().as_str()
        }])),
        3 => history
            .lock()
            .expect("history lock poisoned")
            .later_messages(request.content().as_str()),
        _ => Err(failure(
            "swallowtail.anthropic.attempt_sequence_invalid",
            "Anthropic user turn authorized an invalid attempt ordinal",
        )),
    }
}

struct History {
    maximum_bytes: u64,
    first: Option<FirstHistory>,
}

struct FirstHistory {
    user: SecretText,
    call_id: String,
    tool_name: String,
    arguments: SecretText,
    result: Option<SecretText>,
    answer: Option<SecretText>,
}

impl History {
    fn new(maximum_bytes: u64) -> Self {
        Self {
            maximum_bytes,
            first: None,
        }
    }

    fn clear(&mut self) {
        self.first = None;
    }

    fn record_tool(
        &mut self,
        request: &DirectContinuationTurnRequest,
        call: &DirectToolCall,
    ) -> Result<(), RuntimeFailure> {
        if self.first.is_some() {
            return Err(history_failure());
        }
        self.first = Some(FirstHistory {
            user: SecretText::new(request.content().as_str()),
            call_id: call.call_id().as_str().to_owned(),
            tool_name: call.tool_name().to_owned(),
            arguments: SecretText(call.arguments().as_bytes().to_vec()),
            result: None,
            answer: None,
        });
        self.require_bound()
    }

    fn record_result(&mut self, result: &DirectToolResult) -> Result<(), RuntimeFailure> {
        let first = self.first.as_mut().ok_or_else(history_failure)?;
        if first.result.is_some() || result.call_id().as_str() != first.call_id {
            return Err(history_failure());
        }
        first.result = Some(SecretText(result.content().as_bytes().to_vec()));
        self.require_bound()
    }

    fn record_answer(&mut self, answer: &str) -> Result<(), RuntimeFailure> {
        let first = self.first.as_mut().ok_or_else(history_failure)?;
        if first.result.is_none() || first.answer.is_some() {
            return Err(history_failure());
        }
        first.answer = Some(SecretText::new(answer));
        self.require_bound()
    }

    fn continuation_messages(&self) -> Result<serde_json::Value, RuntimeFailure> {
        let first = self.first.as_ref().ok_or_else(history_failure)?;
        let arguments: serde_json::Value =
            serde_json::from_slice(&first.arguments.0).map_err(|_| history_failure())?;
        Ok(serde_json::json!([
            {"role":"user", "content":first.user.as_str()?},
            {"role":"assistant", "content":[{
                "type":"tool_use",
                "id":first.call_id,
                "name":first.tool_name,
                "input":arguments
            }]},
            {"role":"user", "content":[{
                "type":"tool_result",
                "tool_use_id":first.call_id,
                "content":first.result.as_ref().ok_or_else(history_failure)?.as_str()?
            }]}
        ]))
    }

    fn later_messages(&self, user: &str) -> Result<serde_json::Value, RuntimeFailure> {
        let mut messages = self
            .continuation_messages()?
            .as_array()
            .expect("continuation messages are an array")
            .clone();
        let answer = self
            .first
            .as_ref()
            .and_then(|first| first.answer.as_ref())
            .ok_or_else(history_failure)?
            .as_str()?;
        messages.push(serde_json::json!({"role":"assistant", "content":answer}));
        messages.push(serde_json::json!({"role":"user", "content":user}));
        Ok(serde_json::Value::Array(messages))
    }

    fn require_bound(&self) -> Result<(), RuntimeFailure> {
        let bytes = self.first.as_ref().map_or(0, |first| {
            first.user.0.len()
                + first.call_id.len()
                + first.tool_name.len()
                + first.arguments.0.len()
                + first.result.as_ref().map_or(0, |value| value.0.len())
                + first.answer.as_ref().map_or(0, |value| value.0.len())
        });
        if bytes as u64 > self.maximum_bytes {
            Err(failure(
                "swallowtail.anthropic.history_bound_exceeded",
                "Anthropic private session history exceeded its selected bound",
            ))
        } else {
            Ok(())
        }
    }
}

struct SecretText(Vec<u8>);

impl SecretText {
    fn new(value: &str) -> Self {
        Self(value.as_bytes().to_vec())
    }

    fn as_str(&self) -> Result<&str, RuntimeFailure> {
        std::str::from_utf8(&self.0).map_err(|_| history_failure())
    }
}

impl Drop for SecretText {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}

struct SecretBytes(Vec<u8>);

impl SecretBytes {
    fn copy(&self) -> Vec<u8> {
        self.0.clone()
    }
}

impl Drop for SecretBytes {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}

enum SubmitState {
    AwaitingCall,
    Waiting(BTreeSet<DirectToolCallId>),
    Submitted,
    Abandoned,
}

struct ResultSubmitter {
    state: Mutex<SubmitState>,
    sender: Mutex<Option<oneshot::Sender<Vec<DirectToolResult>>>>,
}

impl ResultSubmitter {
    fn new() -> (Self, oneshot::Receiver<Vec<DirectToolResult>>) {
        let (sender, receiver) = oneshot::channel();
        (
            Self {
                state: Mutex::new(SubmitState::AwaitingCall),
                sender: Mutex::new(Some(sender)),
            },
            receiver,
        )
    }

    fn open(&self, call_id: DirectToolCallId) -> Result<(), RuntimeFailure> {
        let mut state = self.state.lock().expect("tool result state lock poisoned");
        if !matches!(*state, SubmitState::AwaitingCall) {
            return Err(exchange_failure());
        }
        *state = SubmitState::Waiting(BTreeSet::from([call_id]));
        Ok(())
    }

    fn abandon(&self) {
        *self.state.lock().expect("tool result state lock poisoned") = SubmitState::Abandoned;
        self.sender
            .lock()
            .expect("tool result sender lock poisoned")
            .take();
    }
}

impl DirectToolResultSubmitter for ResultSubmitter {
    fn submit(&self, results: Vec<DirectToolResult>) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let result = {
            let mut state = self.state.lock().expect("tool result state lock poisoned");
            let SubmitState::Waiting(expected) = &*state else {
                return Box::pin(async { Err(exchange_failure()) });
            };
            let supplied: BTreeSet<_> = results
                .iter()
                .map(|result| result.call_id().clone())
                .collect();
            if supplied != *expected || supplied.len() != results.len() {
                return Box::pin(async { Err(exchange_failure()) });
            }
            let sender = self
                .sender
                .lock()
                .expect("tool result sender lock poisoned")
                .take();
            *state = SubmitState::Submitted;
            sender
        };
        Box::pin(async move {
            result
                .ok_or_else(exchange_failure)?
                .send(results)
                .map_err(|_| exchange_failure())
        })
    }
}

struct TurnCancellation {
    cancelled: Arc<AtomicBool>,
    session_usable: Arc<AtomicBool>,
    reason: std::sync::atomic::AtomicU8,
    signal: Mutex<Option<oneshot::Sender<()>>>,
}

impl TurnCancellation {
    fn new(
        cancelled: Arc<AtomicBool>,
        session_usable: Arc<AtomicBool>,
    ) -> (Self, oneshot::Receiver<()>) {
        let (sender, receiver) = oneshot::channel();
        (
            Self {
                cancelled,
                session_usable,
                reason: std::sync::atomic::AtomicU8::new(0),
                signal: Mutex::new(Some(sender)),
            },
            receiver,
        )
    }

    fn timeout(&self) {
        if self
            .reason
            .compare_exchange(0, 2, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            self.stop();
        }
    }

    fn stop(&self) {
        self.session_usable.store(false, Ordering::SeqCst);
        self.cancelled.store(true, Ordering::SeqCst);
        if let Some(signal) = self.signal.lock().expect("cancel lock poisoned").take() {
            let _ = signal.send(());
        }
    }

    fn is_requested(&self) -> bool {
        self.reason.load(Ordering::SeqCst) != 0
    }

    fn stop_reason(&self) -> StopSignal {
        if self.reason.load(Ordering::SeqCst) == 2 {
            StopSignal::TimedOut
        } else {
            StopSignal::Cancelled
        }
    }
}

impl CancellationControl for TurnCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::ActiveTurn
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let requested = self
            .reason
            .compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok();
        if requested {
            self.stop();
        }
        Box::pin(async move {
            Ok(if requested {
                CancellationAcknowledgement::Requested
            } else {
                CancellationAcknowledgement::AlreadyRequested
            })
        })
    }
}

struct SessionCancellation {
    active: ActiveSlot,
    usable: Arc<AtomicBool>,
    requested: AtomicBool,
}

impl CancellationControl for SessionCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::InteractiveSession
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let requested = !self.requested.swap(true, Ordering::SeqCst);
        self.usable.store(false, Ordering::SeqCst);
        let active = self
            .active
            .lock()
            .expect("active turn lock poisoned")
            .as_ref()
            .map(|turn| Arc::clone(&turn.cancellation));
        Box::pin(async move {
            if let Some(active) = active {
                let _ = active.request().await?;
            }
            Ok(if requested {
                CancellationAcknowledgement::Requested
            } else {
                CancellationAcknowledgement::AlreadyRequested
            })
        })
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum StopSignal {
    Cancelled,
    TimedOut,
}

enum StreamSignal {
    Item(Result<StreamItem, RuntimeFailure>),
    Closed,
    Stopped(StopSignal),
}

async fn next_signal(
    subscription: &mut Subscription,
    cancel: &mut oneshot::Receiver<()>,
    deadline: &mut BoxFuture<'static, swallowtail_runtime::DeadlineObservation>,
    cancellation: &TurnCancellation,
) -> StreamSignal {
    poll_fn(|context| {
        if let Poll::Ready(item) = subscription.poll_next(context) {
            return Poll::Ready(item.map_or(StreamSignal::Closed, StreamSignal::Item));
        }
        if Pin::new(&mut *cancel).poll(context).is_ready() {
            return Poll::Ready(StreamSignal::Stopped(cancellation.stop_reason()));
        }
        if deadline.as_mut().poll(context).is_ready() {
            cancellation.timeout();
            return Poll::Ready(StreamSignal::Stopped(StopSignal::TimedOut));
        }
        Poll::Pending
    })
    .await
}

async fn wait_results(
    receiver: &mut oneshot::Receiver<Vec<DirectToolResult>>,
    cancel: &mut oneshot::Receiver<()>,
    deadline: &mut BoxFuture<'static, swallowtail_runtime::DeadlineObservation>,
    cancellation: &TurnCancellation,
) -> Result<Vec<DirectToolResult>, StopSignal> {
    poll_fn(|context| {
        if let Poll::Ready(result) = Pin::new(&mut *receiver).poll(context) {
            return Poll::Ready(result.map_err(|_| cancellation.stop_reason()));
        }
        if Pin::new(&mut *cancel).poll(context).is_ready() {
            return Poll::Ready(Err(cancellation.stop_reason()));
        }
        if deadline.as_mut().poll(context).is_ready() {
            cancellation.timeout();
            return Poll::Ready(Err(StopSignal::TimedOut));
        }
        Poll::Pending
    })
    .await
}

async fn reap_finished(active: &ActiveSlot) -> Result<(), RuntimeFailure> {
    let task = {
        let mut active = active.lock().expect("active turn lock poisoned");
        if active
            .as_ref()
            .is_some_and(|turn| turn.terminal.load(Ordering::SeqCst))
        {
            active.as_mut().and_then(|turn| turn.task.take())
        } else {
            None
        }
    };
    if let Some(task) = task {
        task.join().await?;
        *active.lock().expect("active turn lock poisoned") = None;
    }
    Ok(())
}

async fn close_active(active: &ActiveSlot) -> CleanupOutcome {
    let cancellation = active
        .lock()
        .expect("active turn lock poisoned")
        .as_ref()
        .filter(|turn| !turn.terminal.load(Ordering::SeqCst))
        .map(|turn| Arc::clone(&turn.cancellation));
    if let Some(cancellation) = cancellation {
        let _ = cancellation.request().await;
    }
    let task = active
        .lock()
        .expect("active turn lock poisoned")
        .as_mut()
        .and_then(|turn| turn.task.take());
    let cleanup = match task {
        Some(task) => cleanup_result(task.join().await),
        None => CleanupOutcome::NotApplicable,
    };
    *active.lock().expect("active turn lock poisoned") = None;
    cleanup
}

async fn join_turn(active: &ActiveSlot, turn_id: &RuntimeTurnId) -> CleanupOutcome {
    let task = {
        let mut active = active.lock().expect("active turn lock poisoned");
        match active.as_mut() {
            Some(turn) if &turn.turn_id == turn_id => turn.task.take(),
            _ => return CleanupOutcome::NotApplicable,
        }
    };
    let cleanup = match task {
        Some(task) => cleanup_result(task.join().await),
        None => CleanupOutcome::NotApplicable,
    };
    *active.lock().expect("active turn lock poisoned") = None;
    cleanup
}

fn emit_attempt_usage(
    events: &RuntimeEventSender,
    sequence: &mut u64,
    attempt: &swallowtail_runtime::DirectInferenceAttemptId,
    usage: TokenUsage,
) -> Result<(), RuntimeFailure> {
    emit(
        events,
        sequence,
        RuntimeEventKind::ProviderObservation(ProviderObservation::DirectAttemptUsage(
            DirectAttemptUsageObservation::new(attempt.clone(), usage),
        )),
    )
}

fn emit_request(
    events: &RuntimeEventSender,
    sequence: &mut u64,
    headers: &BTreeMap<String, String>,
) -> Result<(), RuntimeFailure> {
    let Some(value) = headers.get("request-id") else {
        return Ok(());
    };
    let request = ProviderRequestRef::new(value).map_err(|_| {
        failure(
            "swallowtail.anthropic.request_id_invalid",
            "Anthropic request correlation was invalid",
        )
    })?;
    emit(
        events,
        sequence,
        RuntimeEventKind::ProviderObservation(ProviderObservation::RequestCorrelation(request)),
    )
}

fn emit_content(
    events: &RuntimeEventSender,
    sequence: &mut u64,
    kind: RuntimeEventKind,
    value: String,
) -> Result<(), RuntimeFailure> {
    let content = OperationContent::new(value).map_err(|_| {
        failure(
            "swallowtail.anthropic.output_invalid",
            "Anthropic emitted empty output content",
        )
    })?;
    events.send(RuntimeEvent::with_content(*sequence, kind, content))?;
    *sequence += 1;
    Ok(())
}

fn emit(
    events: &RuntimeEventSender,
    sequence: &mut u64,
    kind: RuntimeEventKind,
) -> Result<(), RuntimeFailure> {
    events.send(RuntimeEvent::new(*sequence, kind))?;
    *sequence += 1;
    Ok(())
}

fn invalidate(context: &TurnContext) {
    context.usable.store(false, Ordering::SeqCst);
    context
        .state
        .lock()
        .expect("continuation state lock poisoned")
        .invalidate();
}

fn cleanup_result(result: Result<(), RuntimeFailure>) -> CleanupOutcome {
    result.map_or_else(
        |error| CleanupOutcome::Failed(error.diagnostic().clone()),
        |_| CleanupOutcome::Clean,
    )
}

fn history_failure() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.history_state_invalid",
        "Anthropic private continuation history was not in the required state",
    )
}

fn exchange_failure() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.tool_result_rejected",
        "Anthropic tool results did not match the active consumer wait",
    )
}

enum TurnFailure {
    Stopped(StopSignal, CleanupOutcome),
    Provider(RuntimeFailure, CleanupOutcome),
    Runtime(RuntimeFailure, CleanupOutcome),
}
