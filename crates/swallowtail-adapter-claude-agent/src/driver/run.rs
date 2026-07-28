use super::session::merge_cleanup;
use super::validation::{permission_handling, validate_plan, validate_run};
use super::{ClaudeAgentAcpDriver, failure, malformed};
use crate::connection::AcpConnection;
use crate::turn::ActiveTurn;
use serde_json::json;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::{
    CancellationScope, HarnessConfigurationPosture, PreflightPlan, ResourceAccess, RunRef,
    SafeDiagnostic, SessionProviderStatePolicy,
};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CallbackExchange, CancellationAcknowledgement, CancellationControl,
    CleanupOutcome, HostServices, InteractiveSessionHandle, JoinedTask, OpenSessionRequest,
    RequestId, RunHandle, RuntimeFailure, RuntimeRunId, RuntimeTurnId, SessionOptions,
    SessionPlanAgreement, StructuredRunDriver, StructuredRunRequest, TerminalOutcome, TurnHandle,
    TurnRequest, terminal_outcome_channel,
};

impl StructuredRunDriver for ClaudeAgentAcpDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        Box::pin(async move {
            let selected = validate_plan(&plan, self.credential.as_ref())?;
            validate_run(&plan, &request, &services)?;
            let permission_handling = permission_handling(&plan)?;
            let reasoning = request.policy().reasoning_mode().cloned();
            if reasoning.is_some() && !selected.behavior().supports_config_options() {
                return Err(super::unsupported(
                    "reasoning selection for this adapter version",
                ));
            }
            let working_resource = request
                .working_resource()
                .expect("validated working resource")
                .clone();
            let access_policy = match permission_handling {
                crate::ClaudeAgentPermissionHandling::RejectAndStop => {
                    swallowtail_core::SessionAccessPolicy::ambient_harness(
                        ResourceAccess::ReadWrite,
                    )
                }
                crate::ClaudeAgentPermissionHandling::ConsumerMediated => {
                    swallowtail_core::SessionAccessPolicy::
                        ambient_harness_with_consumer_mediated_requests(
                            ResourceAccess::ReadWrite,
                            [crate::claude_agent_permission_namespace()],
                        )
                }
            };
            let mut open_request = OpenSessionRequest::new(
                request.request_id().clone(),
                working_resource,
                None,
                SessionPlanAgreement::explicit(
                    access_policy,
                    Some(SessionProviderStatePolicy::Prohibited),
                    Some(HarnessConfigurationPosture::Ambient),
                ),
            );
            if let Some(reasoning) = reasoning.as_ref() {
                open_request = open_request
                    .with_options(SessionOptions::default().with_reasoning_mode(reasoning.clone()));
            }
            let mut session = self
                .start_session(&plan, &open_request, &services, selected, reasoning)
                .await?;
            let run_id = RuntimeRunId::new(format!(
                "claude-agent-acp:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| malformed())?;
            let turn_id = RuntimeTurnId::new(format!(
                "claude-agent-acp:run:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| malformed())?;
            let mut turn_request = TurnRequest::new(turn_id, request.content().clone());
            if let Some(deadline) = request.deadline() {
                turn_request = turn_request.with_deadline(deadline);
            }
            let mut turn = match session.start_turn(turn_request, services.clone()).await {
                Ok(turn) => turn,
                Err(error) => {
                    let _ = Box::new(session).close().await;
                    return Err(error);
                }
            };
            let events = match turn.take_events() {
                Some(events) => events,
                None => {
                    let _ = turn.close().await;
                    let _ = Box::new(session).close().await;
                    return Err(failure(
                        "swallowtail.claude_agent.acp.run_events_missing",
                        "Claude Agent structured run did not expose its event stream",
                    ));
                }
            };
            let callbacks = turn.take_callbacks();
            let turn_terminal = match turn.take_terminal_outcome() {
                Some(terminal) => terminal,
                None => {
                    let _ = turn.close().await;
                    let _ = Box::new(session).close().await;
                    return Err(failure(
                        "swallowtail.claude_agent.acp.run_terminal_missing",
                        "Claude Agent structured run did not expose its terminal outcome",
                    ));
                }
            };
            let active = {
                let slot = session
                    .active
                    .lock()
                    .expect("Claude Agent active run lock poisoned");
                slot.as_ref().map(|active| Arc::clone(&active.turn))
            };
            let active = match active {
                Some(active) => active,
                None => {
                    let _ = turn.close().await;
                    let _ = Box::new(session).close().await;
                    return Err(failure(
                        "swallowtail.claude_agent.acp.run_active_missing",
                        "Claude Agent structured run lost its active turn",
                    ));
                }
            };
            let cancellation = Arc::new(ClaudeAgentRunCancellation {
                connection: Arc::clone(&session.connection),
                session_id: session.provider_id.clone(),
                turn: active,
                requested: AtomicBool::new(false),
            });
            let pending = Arc::new(Mutex::new(Some(ClaudeAgentRunResources {
                turn,
                session,
                terminal: turn_terminal,
            })));
            let (terminal_sender, terminal) = terminal_outcome_channel();
            let task_scope = swallowtail_runtime::ScopeId::new(format!(
                "claude-agent-acp:run-cleanup:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| malformed())?;
            let task_pending = Arc::clone(&pending);
            let task = services.task().expect("validated task").spawn(
                task_scope,
                Box::pin(async move {
                    let resources = task_pending
                        .lock()
                        .expect("Claude Agent pending run lock poisoned")
                        .take()
                        .expect("Claude Agent pending run exists");
                    let outcome = resources.terminal.await;
                    let turn_cleanup = resources.turn.close().await;
                    let session_cleanup = Box::new(resources.session).close().await;
                    let cleanup = merge_cleanup(
                        outcome.cleanup().clone(),
                        merge_cleanup(turn_cleanup, session_cleanup),
                    );
                    let mut finished = TerminalOutcome::new(outcome.status().clone(), cleanup);
                    if let Some(output) = outcome.output().cloned() {
                        finished = finished.with_output(output);
                    }
                    let _ = terminal_sender.complete(finished);
                }),
            );
            let task = match task {
                Ok(task) => task,
                Err(error) => {
                    let _ = cancellation.request().await;
                    let resources = pending
                        .lock()
                        .expect("Claude Agent pending run lock poisoned")
                        .take();
                    if let Some(resources) = resources {
                        let _ = resources.turn.close().await;
                        let _ = Box::new(resources.session).close().await;
                    }
                    return Err(error);
                }
            };
            Ok(Box::new(ClaudeAgentRunHandle {
                request_id: request.request_id().clone(),
                run_id,
                events: Some(events),
                callbacks,
                terminal: Some(Box::pin(terminal)),
                cancellation,
                task,
            }) as Box<dyn RunHandle>)
        })
    }
}

struct ClaudeAgentRunResources {
    turn: Box<dyn TurnHandle>,
    session: super::session::ClaudeAgentSessionHandle,
    terminal: BoxFuture<'static, TerminalOutcome>,
}

struct ClaudeAgentRunCancellation {
    connection: Arc<AcpConnection>,
    session_id: String,
    turn: Arc<ActiveTurn>,
    requested: AtomicBool,
}

impl CancellationControl for ClaudeAgentRunCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::StructuredRun
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let already = self.requested.swap(true, Ordering::SeqCst) || self.turn.is_finished();
        Box::pin(async move {
            if already {
                return Ok(CancellationAcknowledgement::AlreadyRequested);
            }
            self.turn.mark_cancelled();
            self.connection
                .notify("session/cancel", json!({"sessionId": self.session_id}))
                .await?;
            Ok(CancellationAcknowledgement::Requested)
        })
    }
}

struct ClaudeAgentRunHandle {
    request_id: RequestId,
    run_id: RuntimeRunId,
    events: Option<BoxEventStream>,
    callbacks: Option<CallbackExchange>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: Arc<ClaudeAgentRunCancellation>,
    task: Box<dyn JoinedTask>,
}

impl RunHandle for ClaudeAgentRunHandle {
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

    fn take_callbacks(&mut self) -> Option<CallbackExchange> {
        self.callbacks.take()
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        self.cancellation.as_ref()
    }

    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>> {
        self.terminal.take()
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            let _ = self.cancellation.request().await;
            self.task.join().await.map_or_else(
                |_| {
                    CleanupOutcome::Failed(SafeDiagnostic::new(
                        "swallowtail.claude_agent.acp.run_join_failed",
                        "Claude Agent structured-run cleanup task did not join",
                    ))
                },
                |_| CleanupOutcome::Clean,
            )
        })
    }
}
