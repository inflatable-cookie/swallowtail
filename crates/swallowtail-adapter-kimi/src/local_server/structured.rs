#[path = "structured/prepared.rs"]
mod prepared;
#[path = "structured/validation.rs"]
mod validation;

pub use prepared::{KimiLocalServerPreparedRun, KimiLocalServerRunInput};

use crate::failure::failure;
use crate::local_server::interactive::{KimiInteractiveSession, TurnCancellation};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::{
    CancellationScope, HarnessConfigurationPosture, PreflightPlan, RunRef, SafeDiagnostic,
    SessionProviderStatePolicy,
};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CallbackExchange, CancellationAcknowledgement, CancellationControl,
    CleanupOutcome, HostServices, InteractiveSessionHandle, JoinedTask, OpenSessionRequest,
    RequestId, RunHandle, RuntimeFailure, RuntimeRunId, RuntimeTurnId, SessionCleanupRequest,
    SessionOptions, SessionPlanAgreement, StructuredRunDriver, StructuredRunRequest,
    TerminalOutcome, TurnHandle, TurnRequest, terminal_outcome_channel,
};

impl StructuredRunDriver for super::KimiLocalServerDriver {
    fn start_run(
        &self,
        plan: PreflightPlan,
        request: StructuredRunRequest,
        services: HostServices,
    ) -> BoxFuture<'_, Result<Box<dyn RunHandle>, RuntimeFailure>> {
        Box::pin(async move {
            validation::validate(self, &plan, &request, &services)?;
            let cleanup =
                SessionCleanupRequest::new(request.deadline().expect("validated caller deadline"));
            let configuration = self
                .configuration()
                .expect("validated session configuration")
                .clone();
            let access_policy = super::interactive::access_policy(configuration.permission_mode());
            let agreement = SessionPlanAgreement::explicit(
                access_policy,
                Some(SessionProviderStatePolicy::DurableProviderSessionPreserved),
                Some(HarnessConfigurationPosture::Ambient),
            );
            let mut options = SessionOptions::default();
            if let Some(reasoning) = request.policy().reasoning_mode() {
                options = options.with_reasoning_mode(reasoning.clone());
            }
            let open = OpenSessionRequest::new(
                request.request_id().clone(),
                request
                    .working_resource()
                    .expect("validated working resource")
                    .clone(),
                request.deadline(),
                agreement,
            )
            .with_options(options);
            let mut session = self
                .open_structured_inner(plan, open, services.clone())
                .await?;
            let turn_id =
                RuntimeTurnId::new(format!("kimi-local:run:{}", request.request_id().as_str()))
                    .map_err(|_| invalid_identity())?;
            let mut turn_request = TurnRequest::new(turn_id, request.content().clone());
            if let Some(deadline) = request.deadline() {
                turn_request = turn_request.with_deadline(deadline);
            }
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
                        "swallowtail.kimi.local_server.run_events_missing",
                        "Kimi local-server structured run did not expose its event stream",
                    ));
                }
            };
            let callbacks = turn.take_callbacks();
            let terminal = match turn.take_terminal_outcome() {
                Some(terminal) => terminal,
                None => {
                    drop(turn);
                    let _ = Box::new(session).close(cleanup, services).await;
                    return Err(failure(
                        "swallowtail.kimi.local_server.run_terminal_missing",
                        "Kimi local-server structured run did not expose its terminal outcome",
                    ));
                }
            };
            let active = {
                let active = session.active.lock().expect("active turn lock poisoned");
                active.as_ref().map(|active| {
                    (
                        Arc::clone(&active.cancellation),
                        Arc::clone(&active.terminal),
                    )
                })
            };
            let Some((active_cancellation, terminal_flag)) = active else {
                drop(turn);
                let _ = Box::new(session).close(cleanup, services).await;
                return Err(failure(
                    "swallowtail.kimi.local_server.run_active_missing",
                    "Kimi local-server structured run lost its active prompt",
                ));
            };
            let cancellation = Arc::new(KimiLocalRunCancellation {
                inner: active_cancellation,
                terminal: terminal_flag,
                requested: AtomicBool::new(false),
            });
            let pending = Arc::new(Mutex::new(Some(KimiLocalRunResources {
                turn,
                session,
                terminal,
            })));
            let (terminal_sender, terminal) = terminal_outcome_channel();
            let task_scope = swallowtail_runtime::ScopeId::new(format!(
                "kimi-local:run-cleanup:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| invalid_identity())?;
            let task_pending = Arc::clone(&pending);
            let task_services = services.clone();
            let task = services.task().expect("validated task service").spawn(
                task_scope,
                Box::pin(async move {
                    let resources = task_pending
                        .lock()
                        .expect("pending run lock poisoned")
                        .take()
                        .expect("pending run exists");
                    let outcome = resources.terminal.await;
                    drop(resources.turn);
                    let session_cleanup = Box::new(resources.session)
                        .close(cleanup, task_services)
                        .await;
                    let cleanup = merge_cleanup(outcome.cleanup().clone(), session_cleanup);
                    let finished = copy_terminal_outcome(outcome, cleanup);
                    let _ = terminal_sender.complete(finished);
                }),
            );
            let task = match task {
                Ok(task) => task,
                Err(error) => {
                    let resources = pending.lock().expect("pending run lock poisoned").take();
                    if let Some(resources) = resources {
                        drop(resources.turn);
                        let _ = Box::new(resources.session).close(cleanup, services).await;
                    }
                    return Err(error);
                }
            };
            let run_id =
                RuntimeRunId::new(format!("kimi-local:run:{}", request.request_id().as_str()))
                    .map_err(|_| invalid_identity())?;
            Ok(Box::new(KimiLocalRunHandle {
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

struct KimiLocalRunResources {
    turn: Box<dyn TurnHandle>,
    session: KimiInteractiveSession,
    terminal: BoxFuture<'static, TerminalOutcome>,
}

struct KimiLocalRunCancellation {
    inner: Arc<TurnCancellation>,
    terminal: Arc<AtomicBool>,
    requested: AtomicBool,
}

impl CancellationControl for KimiLocalRunCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::StructuredRun
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let already =
            self.requested.swap(true, Ordering::SeqCst) || self.terminal.load(Ordering::SeqCst);
        Box::pin(async move {
            if already {
                Ok(CancellationAcknowledgement::AlreadyRequested)
            } else {
                self.inner.request().await
            }
        })
    }
}

struct KimiLocalRunHandle {
    request_id: RequestId,
    run_id: RuntimeRunId,
    events: Option<BoxEventStream>,
    callbacks: Option<CallbackExchange>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: Arc<KimiLocalRunCancellation>,
    task: Box<dyn JoinedTask>,
}

impl RunHandle for KimiLocalRunHandle {
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
                        "swallowtail.kimi.local_server.run_join_failed",
                        "Kimi local-server structured-run cleanup task did not join",
                    ))
                },
                |_| CleanupOutcome::Clean,
            )
        })
    }
}

fn merge_cleanup(current: CleanupOutcome, next: CleanupOutcome) -> CleanupOutcome {
    match (&current, &next) {
        (CleanupOutcome::Failed(_), _) => current,
        (_, CleanupOutcome::Failed(_)) => next,
        (CleanupOutcome::Degraded(_), _) => current,
        (_, CleanupOutcome::Degraded(_)) => next,
        (CleanupOutcome::Clean, _) => current,
        (CleanupOutcome::NotApplicable, CleanupOutcome::Clean) => next,
        _ => current,
    }
}

fn copy_terminal_outcome(outcome: TerminalOutcome, cleanup: CleanupOutcome) -> TerminalOutcome {
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

fn invalid_identity() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.run_identity_invalid",
        "Kimi local-server structured-run identity was invalid",
    )
}
