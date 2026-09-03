use super::session::PiSessionHandle;
use super::session::cleanup::merge_cleanup;
use super::validation::validate_run;
use super::{PiRpcDriver, failure};
use crate::connection::PiConnection;
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

impl StructuredRunDriver for PiRpcDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        Box::pin(async move {
            validate_run(&plan, &request, &services, &self.credential)?;
            let cleanup =
                SessionCleanupRequest::new(request.deadline().expect("validated Pi run deadline"));
            let run_id = RuntimeRunId::new(format!("pi-rpc:run:{}", request.request_id().as_str()))
                .map_err(|_| malformed_run_id())?;
            let turn_id =
                RuntimeTurnId::new(format!("pi-rpc:run:{}", request.request_id().as_str()))
                    .map_err(|_| malformed_run_id())?;
            let task_scope = swallowtail_runtime::ScopeId::new(format!(
                "pi-rpc:run-cleanup:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| malformed_run_id())?;
            let open_request = OpenSessionRequest::new(
                request.request_id().clone(),
                request
                    .working_resource()
                    .expect("validated Pi working resource")
                    .clone(),
                None,
                SessionPlanAgreement::explicit(
                    SessionAccessPolicy::ambient_harness(ResourceAccess::Read),
                    Some(SessionProviderStatePolicy::Prohibited),
                    Some(HarnessConfigurationPosture::ProviderSuppressed),
                ),
            );
            let mut session = self
                .start_session(plan, open_request, services.clone())
                .await?;
            let turn_request = TurnRequest::new(turn_id, request.content().clone())
                .with_deadline(request.deadline().expect("validated Pi run deadline"))
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
                        "swallowtail.pi.rpc.run_events_missing",
                        "Pi RPC structured run did not expose its event stream",
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
                        "swallowtail.pi.rpc.run_terminal_missing",
                        "Pi RPC structured run did not expose its terminal outcome",
                    ));
                }
            };
            let active = {
                let slot = session.active.lock().expect("Pi active-task lock poisoned");
                slot.as_ref().map(|active| Arc::clone(&active.turn))
            };
            let active = match active {
                Some(active) => active,
                None => {
                    drop(turn);
                    let _ = Box::new(session).close(cleanup, services).await;
                    return Err(failure(
                        "swallowtail.pi.rpc.run_active_missing",
                        "Pi RPC structured run lost its active prompt",
                    ));
                }
            };
            let cancellation = Arc::new(PiRunCancellation {
                connection: Arc::clone(&session.connection),
                turn: active,
                requested: AtomicBool::new(false),
            });
            let pending = Arc::new(Mutex::new(Some(PiRunResources {
                turn,
                session,
                terminal: turn_terminal,
            })));
            let (terminal_sender, terminal) = terminal_outcome_channel();
            let task_pending = Arc::clone(&pending);
            let task_services = services.clone();
            let task = services.task().expect("validated Pi task service").spawn(
                task_scope,
                Box::pin(async move {
                    let resources = task_pending
                        .lock()
                        .expect("Pi pending run lock poisoned")
                        .take()
                        .expect("Pi pending run exists");
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
                    let resources = pending.lock().expect("Pi pending run lock poisoned").take();
                    if let Some(resources) = resources {
                        drop(resources.turn);
                        let _ = Box::new(resources.session).close(cleanup, services).await;
                    }
                    return Err(error);
                }
            };
            Ok(Box::new(PiRunHandle {
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

struct PiRunResources {
    turn: Box<dyn TurnHandle>,
    session: PiSessionHandle,
    terminal: BoxFuture<'static, TerminalOutcome>,
}

struct PiRunCancellation {
    connection: Arc<PiConnection>,
    turn: Arc<ActiveTurn>,
    requested: AtomicBool,
}

impl CancellationControl for PiRunCancellation {
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
                    "swallowtail.pi.rpc.abort_rejected",
                    "Pi RPC rejected native abort",
                ))
            }
        })
    }
}

struct PiRunHandle {
    request_id: RequestId,
    run_id: RuntimeRunId,
    events: Option<BoxEventStream>,
    callbacks: Option<CallbackExchange>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: Arc<PiRunCancellation>,
    task: Box<dyn JoinedTask>,
}

impl RunHandle for PiRunHandle {
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
                        "swallowtail.pi.rpc.run_join_failed",
                        "Pi RPC structured-run cleanup task did not join",
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
        "swallowtail.pi.rpc.run_id_invalid",
        "Pi RPC structured-run identity was invalid",
    )
}
