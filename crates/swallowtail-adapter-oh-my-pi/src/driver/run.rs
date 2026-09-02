use super::session::OhMyPiSessionHandle;
use super::session::cleanup::merge_cleanup;
use super::validation::validate_run;
use super::{OhMyPiRpcDriver, failure};
use crate::connection::OhMyPiConnection;
use crate::turn::ActiveTurn;
use serde_json::json;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::{
    CancellationScope, HarnessConfigurationPosture, PreflightPlan, ResourceAccess, RunRef,
    SafeDiagnostic, SessionAccessPolicy, SessionProviderStatePolicy,
};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CallbackExchange, CancellationAcknowledgement, CancellationControl,
    CleanupOutcome, HostServices, InteractiveSessionHandle, JoinedTask, OpenSessionRequest,
    RequestId, RunHandle, RuntimeFailure, RuntimeRunId, RuntimeTurnId, SessionCleanupRequest,
    SessionPlanAgreement, StructuredRunDriver, StructuredRunRequest, TerminalOutcome, TurnHandle,
    TurnRequest, terminal_outcome_channel,
};

impl StructuredRunDriver for OhMyPiRpcDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        Box::pin(async move {
            validate_run(&plan, &request, &services)?;
            let cleanup = SessionCleanupRequest::new(
                request.deadline().expect("validated OhMyPi run deadline"),
            );
            let run_id = RuntimeRunId::new(format!(
                "oh-my-pi-rpc:run:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| malformed_run_id())?;
            let turn_id = RuntimeTurnId::new(format!(
                "oh-my-pi-rpc:run:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| malformed_run_id())?;
            let task_scope = swallowtail_runtime::ScopeId::new(format!(
                "oh-my-pi-rpc:run-cleanup:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| malformed_run_id())?;
            let mut open_request = OpenSessionRequest::new(
                request.request_id().clone(),
                request
                    .working_resource()
                    .expect("validated OhMyPi working resource")
                    .clone(),
                None,
                SessionPlanAgreement::explicit(
                    SessionAccessPolicy::ambient_harness(ResourceAccess::Read),
                    Some(SessionProviderStatePolicy::Prohibited),
                    Some(HarnessConfigurationPosture::ProviderSuppressed),
                ),
            );
            if let Some(reasoning) = request.policy().reasoning_mode() {
                open_request = open_request.with_options(
                    swallowtail_runtime::SessionOptions::default()
                        .with_reasoning_mode(reasoning.clone()),
                );
            }
            let mut session = self
                .start_session(plan, open_request, services.clone())
                .await?;
            let turn_request = TurnRequest::new(turn_id, request.content().clone())
                .with_deadline(request.deadline().expect("validated OhMyPi run deadline"))
                .with_attachments(request.attachments().cloned());
            let mut turn = match session.start_turn(turn_request, services.clone()).await {
                Ok(turn) => turn,
                Err(error) => {
                    let _ = Box::new(session).close(cleanup, services).await;
                    return Err(error);
                }
            };
            let events = match turn.take_events() {
                Some(events) => events,
                None => {
                    drop(turn);
                    let _ = Box::new(session).close(cleanup, services).await;
                    return Err(failure(
                        "swallowtail.oh_my_pi.rpc.run_events_missing",
                        "OhMyPi RPC structured run did not expose its event stream",
                    ));
                }
            };
            let callbacks = turn.take_callbacks();
            let turn_terminal = match turn.take_terminal_outcome() {
                Some(terminal) => terminal,
                None => {
                    drop(turn);
                    let _ = Box::new(session).close(cleanup, services).await;
                    return Err(failure(
                        "swallowtail.oh_my_pi.rpc.run_terminal_missing",
                        "OhMyPi RPC structured run did not expose its terminal outcome",
                    ));
                }
            };
            let active = {
                let slot = session
                    .active
                    .lock()
                    .expect("OhMyPi active-task lock poisoned");
                slot.as_ref().map(|active| Arc::clone(&active.turn))
            };
            let active = match active {
                Some(active) => active,
                None => {
                    drop(turn);
                    let _ = Box::new(session).close(cleanup, services).await;
                    return Err(failure(
                        "swallowtail.oh_my_pi.rpc.run_active_missing",
                        "OhMyPi RPC structured run lost its active prompt",
                    ));
                }
            };
            let cancellation = Arc::new(OhMyPiRunCancellation {
                connection: Arc::clone(&session.connection),
                turn: active,
                requested: AtomicBool::new(false),
            });
            let pending = Arc::new(Mutex::new(Some(OhMyPiRunResources {
                turn,
                session,
                terminal: turn_terminal,
            })));
            let (terminal_sender, terminal) = terminal_outcome_channel();
            let task_pending = Arc::clone(&pending);
            let task_services = services.clone();
            let task = services
                .task()
                .expect("validated OhMyPi task service")
                .spawn(
                    task_scope,
                    Box::pin(async move {
                        let resources = task_pending
                            .lock()
                            .expect("OhMyPi pending run lock poisoned")
                            .take()
                            .expect("OhMyPi pending run exists");
                        let outcome = resources.terminal.await;
                        drop(resources.turn);
                        let session_cleanup = Box::new(resources.session)
                            .close(cleanup, task_services)
                            .await;
                        let cleanup = merge_cleanup(outcome.cleanup().clone(), session_cleanup);
                        let finished = copy_outcome_with_cleanup(outcome, cleanup);
                        let _ = terminal_sender.complete(finished);
                    }),
                );
            let task = match task {
                Ok(task) => task,
                Err(error) => {
                    let resources = pending
                        .lock()
                        .expect("OhMyPi pending run lock poisoned")
                        .take();
                    if let Some(resources) = resources {
                        drop(resources.turn);
                        let _ = Box::new(resources.session).close(cleanup, services).await;
                    }
                    return Err(error);
                }
            };
            Ok(Box::new(OhMyPiRunHandle {
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

struct OhMyPiRunResources {
    turn: Box<dyn TurnHandle>,
    session: OhMyPiSessionHandle,
    terminal: BoxFuture<'static, TerminalOutcome>,
}

struct OhMyPiRunCancellation {
    connection: Arc<OhMyPiConnection>,
    turn: Arc<ActiveTurn>,
    requested: AtomicBool,
}

impl CancellationControl for OhMyPiRunCancellation {
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
            let id = format!("abort:{}", self.turn.runtime_id().as_str());
            let response = self
                .connection
                .command(id.clone(), "abort", json!({"id": id, "type": "abort"}))
                .await?;
            if response.success {
                Ok(CancellationAcknowledgement::Requested)
            } else {
                Err(failure(
                    "swallowtail.oh_my_pi.rpc.abort_rejected",
                    "OhMyPi RPC rejected native abort",
                ))
            }
        })
    }
}

struct OhMyPiRunHandle {
    request_id: RequestId,
    run_id: RuntimeRunId,
    events: Option<BoxEventStream>,
    callbacks: Option<CallbackExchange>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: Arc<OhMyPiRunCancellation>,
    task: Box<dyn JoinedTask>,
}

impl RunHandle for OhMyPiRunHandle {
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
                        "swallowtail.oh_my_pi.rpc.run_join_failed",
                        "OhMyPi RPC structured-run cleanup task did not join",
                    ))
                },
                |_| CleanupOutcome::Clean,
            )
        })
    }
}

fn copy_outcome_with_cleanup(outcome: TerminalOutcome, cleanup: CleanupOutcome) -> TerminalOutcome {
    let mut finished = TerminalOutcome::new(outcome.status().clone(), cleanup);
    if let Some(output) = outcome.output().cloned() {
        finished = finished.with_output(output);
    }
    if let Some(cancellation) = outcome.provider_cancellation() {
        finished = finished.with_provider_cancellation(cancellation);
    }
    for (resource, deletion) in outcome.remote_resource_deletions() {
        finished = finished.with_remote_resource_deletion(resource, deletion);
    }
    finished
}

fn malformed_run_id() -> RuntimeFailure {
    failure(
        "swallowtail.oh_my_pi.rpc.run_id_invalid",
        "OhMyPi RPC structured-run identity was invalid",
    )
}
