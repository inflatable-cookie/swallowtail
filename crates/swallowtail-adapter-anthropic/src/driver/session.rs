use super::{AccessLeases, AnthropicDirectDriver, operation_scope, require_services};
use crate::failure::{failure, unsupported};
use crate::protocol::{
    ContentBlock, Event, RedactedBytes, Request, ToolSpec, parse_event, provider_failure,
};
use crate::reasoning::validate_runtime_binding;
use crate::transport::{StreamItem, Subscription};
use futures_channel::{mpsc, oneshot};
use std::collections::{BTreeMap, BTreeSet};
use std::future::poll_fn;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::Poll;
use swallowtail_core::{
    CancellationScope, PreflightPlan, ProviderRequestRef, ReasoningMode, SessionRef, TurnRef,
};
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
    reasoning: Option<ReasoningMode>,
    thinking: Option<crate::AnthropicThinkingMode>,
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
            validate_runtime_binding(&plan, request.options().reasoning_mode())?;
            crate::thinking::validate_runtime_binding(plan.model_id(), self.thinking_mode)?;
            if request.options().developer_instructions().is_some()
                || request.options().tools().len() == 0
            {
                return Err(unsupported("the requested direct-session options"));
            }
            let tools = tool_specs(&request)?;
            let reasoning = request.options().reasoning_mode().cloned();
            let thinking = self.thinking_mode;
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
                reasoning,
                thinking,
                state: Arc::new(Mutex::new(DirectContinuationState::new(config.clone()))),
                history: Arc::new(Mutex::new(History::new(
                    config.maximum_private_history_bytes().get(),
                    config.maximum_private_continuation_bytes().get(),
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

    fn close(
        mut self: Box<Self>,
        request: swallowtail_runtime::SessionCleanupRequest,
        services: HostServices,
    ) -> BoxFuture<'static, CleanupOutcome> {
        let execution_host_id = self.services.execution_host_id().clone();
        swallowtail_runtime::bound_session_cleanup(
            execution_host_id,
            request,
            services,
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
            }),
        )
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
            self.reasoning.as_ref(),
            self.thinking,
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
            reasoning: self.reasoning.clone(),
            thinking: self.thinking,
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

include!("session/turn.rs");
include!("session/attempt.rs");
include!("session/parser.rs");
include!("session/history.rs");
include!("session/encode.rs");
include!("session/control.rs");
include!("session/events.rs");
