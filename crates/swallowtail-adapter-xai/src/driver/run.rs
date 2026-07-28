use super::access::AccessLeases;
use super::lifecycle::{TurnCancellation, cleanup_from_result, merge_cleanup};
use super::turn::pump::{PendingTurn, pump_turn};
use super::{PROVIDER_ID, XaiWebSocketDriver};
use crate::failure::{failure, unsupported};
use crate::transport::Connection;
use futures_channel::mpsc;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use swallowtail_core::{
    Capability, ExternalNetworkPolicy, ExternalSearchPolicy, PreflightPlan, RunRef, SafeDiagnostic,
};
use swallowtail_runtime::{
    BoxEventStream, BoxFuture, CancellationControl, CleanupOutcome, HostServices, JoinedTask,
    OperationPolicy, ProviderExecutionPolicy, ProviderRecoveryPolicy, ProviderRetentionPolicy,
    RequestId, RunHandle, RuntimeEvent, RuntimeEventKind, RuntimeFailure, RuntimeRunId,
    RuntimeTurnId, ScopeId, StreamReattachmentPolicy, StructuredRunDriver, StructuredRunRequest,
    TerminalOutcome, runtime_event_channel, terminal_outcome_channel,
};

const EVENT_CAPACITY: usize = 32;
const INGRESS_CAPACITY: usize = 32;

impl StructuredRunDriver for XaiWebSocketDriver {
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
            let scope = ScopeId::new(format!(
                "xai-websocket:run:{}",
                request.request_id().as_str()
            ))
            .map_err(|_| failure("swallowtail.xai.scope_invalid", "xAI run scope was invalid"))?;
            let run_id =
                RuntimeRunId::new(format!("xai-websocket:{}", request.request_id().as_str()))
                    .map_err(|_| {
                        failure(
                            "swallowtail.xai.run_id_invalid",
                            "xAI runtime run identity was invalid",
                        )
                    })?;
            let billed_turn_id =
                RuntimeTurnId::new(format!("xai-structured:{}", request.request_id().as_str()))
                    .map_err(|_| {
                        failure(
                            "swallowtail.xai.turn_id_invalid",
                            "xAI internal response identity was invalid",
                        )
                    })?;
            let model = plan
                .model_id()
                .expect("validated model")
                .as_str()
                .to_owned();
            let model_route_id = plan.model_route_id().expect("validated route").clone();
            let access_profile_id = plan.access_profile_id().clone();
            let mut access = AccessLeases::acquire(&plan, scope.clone(), &services).await?;
            let connection = match access.connect(scope.clone(), &services).await {
                Ok(connection) => connection,
                Err(error) => {
                    let _ = access.release(&services).await;
                    return Err(error);
                }
            };
            let (events, stream) = runtime_event_channel(EVENT_CAPACITY)?;
            events.send(RuntimeEvent::new(0, RuntimeEventKind::Started))?;
            let (updates, receiver) = mpsc::channel(INGRESS_CAPACITY);
            let chain_valid = Arc::new(AtomicBool::new(true));
            let cancellation = Arc::new(TurnCancellation::structured(
                connection.closer(),
                chain_valid,
            ));
            let work_connection = connection.clone();
            let input = request.content().as_str().to_owned();
            let work = services
                .blocking_work()
                .cloned()
                .expect("validated blocking work")
                .run(
                    scope.clone(),
                    Box::new(move || work_connection.run_one_response(&model, &input, updates)),
                );
            let pending = Arc::new(Mutex::new(Some(RunResources {
                pending: PendingTurn {
                    updates: receiver,
                    work,
                },
                connection,
                access,
            })));
            let deadline = request.deadline().map(|deadline| {
                services
                    .time()
                    .expect("validated time")
                    .wait_until(deadline)
            });
            let (terminal_sender, terminal) = terminal_outcome_channel();
            let task_pending = Arc::clone(&pending);
            let task_scope = scope.clone();
            let task = services.task().expect("validated task").spawn(
                scope.clone(),
                Box::pin({
                    let cancellation = Arc::clone(&cancellation);
                    let run_services = services.clone();
                    async move {
                        let resources = task_pending
                            .lock()
                            .expect("xAI run pending work lock poisoned")
                            .take()
                            .expect("xAI run pending work exists");
                        let outcome = pump_turn(
                            resources.pending,
                            events.clone(),
                            cancellation,
                            deadline,
                            billed_turn_id,
                            model_route_id,
                            access_profile_id,
                        )
                        .await;
                        let outcome = finish_run(
                            outcome,
                            resources.connection,
                            resources.access,
                            run_services,
                            task_scope,
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
                    let _ = cancellation.request().await;
                    let resources = pending
                        .lock()
                        .expect("xAI run pending work lock poisoned")
                        .take();
                    if let Some(mut resources) = resources {
                        let _ = resources.pending.work.await;
                        let connection = resources.connection.clone();
                        if let Some(blocking) = services.blocking_work() {
                            let _ = blocking
                                .run(scope, Box::new(move || connection.close()))
                                .await;
                        }
                        let _ = resources.access.release(&services).await;
                    }
                    return Err(error);
                }
            };
            Ok(Box::new(XaiRunHandle {
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
    if services.task().is_none()
        || services.blocking_work().is_none()
        || services.time().is_none()
        || services.network().is_none()
        || services.credential().is_none()
    {
        return Err(failure(
            "swallowtail.xai.host_services_missing",
            "xAI WebSocket required host services are unavailable",
        ));
    }
    if plan.requirements().driver_role() != swallowtail_core::DriverRole::StructuredRun
        || plan
            .provider_id()
            .is_none_or(|id| id.as_str() != PROVIDER_ID)
        || plan.model_id().is_none()
        || plan.model_route_id().is_none()
    {
        return Err(failure(
            "swallowtail.xai.model_binding_rejected",
            "xAI WebSocket run requires one exact xAI model route",
        ));
    }
    for capability in [
        Capability::StructuredRun,
        Capability::StreamingEvents,
        Capability::Interruption,
        Capability::UsageReporting,
        Capability::BilledCostReporting,
    ] {
        if !plan
            .requirements()
            .capabilities()
            .any(|requirement| requirement.capability() == capability)
        {
            return Err(failure(
                "swallowtail.xai.capability_binding_rejected",
                "xAI WebSocket run capability requirements were incomplete",
            ));
        }
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
    validate_policy(request.policy())?;
    if let Some(deadline) = request.deadline()
        && services.time().expect("validated time").now() >= deadline.instant()
    {
        return Err(failure(
            "swallowtail.xai.deadline_elapsed",
            "xAI run deadline elapsed before provider work",
        ));
    }
    Ok(())
}

fn validate_policy(policy: &OperationPolicy) -> Result<(), RuntimeFailure> {
    if policy.reasoning_mode().is_some()
        || policy.external_network() != ExternalNetworkPolicy::Denied
        || policy.external_search() != ExternalSearchPolicy::Disabled
        || policy.provider_execution() != ProviderExecutionPolicy::Attached
        || policy.provider_retention() != ProviderRetentionPolicy::Prohibited
        || policy.provider_recovery() != ProviderRecoveryPolicy::Prohibited
        || policy.stream_reattachment() != StreamReattachmentPolicy::Disabled
    {
        Err(unsupported(
            "reasoning, network, background, retention, recovery, or reattachment policy",
        ))
    } else {
        Ok(())
    }
}

struct RunResources {
    pending: PendingTurn,
    connection: Connection,
    access: AccessLeases,
}

struct XaiRunHandle {
    request_id: RequestId,
    run_id: RuntimeRunId,
    events: Option<BoxEventStream>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: Arc<TurnCancellation>,
    task: Box<dyn JoinedTask>,
}

impl RunHandle for XaiRunHandle {
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
            let _ = self.cancellation.request().await;
            self.task.join().await.map_or_else(
                |_| {
                    CleanupOutcome::Failed(SafeDiagnostic::new(
                        "swallowtail.xai.task_join_failed",
                        "xAI run task could not be joined",
                    ))
                },
                |_| CleanupOutcome::Clean,
            )
        })
    }
}

async fn finish_run(
    outcome: TerminalOutcome,
    connection: Connection,
    mut access: AccessLeases,
    services: HostServices,
    scope: ScopeId,
) -> TerminalOutcome {
    let connection_cleanup = match services.blocking_work() {
        Some(blocking) => cleanup_from_result(
            blocking
                .run(scope, Box::new(move || connection.close()))
                .await,
        ),
        None => CleanupOutcome::Failed(SafeDiagnostic::new(
            "swallowtail.xai.blocking_service_missing",
            "xAI blocking-work service disappeared during cleanup",
        )),
    };
    let credential_cleanup = access.release(&services).await;
    let cleanup = merge_cleanup(
        outcome.cleanup().clone(),
        merge_cleanup(connection_cleanup, credential_cleanup),
    );
    let mut finished = TerminalOutcome::new(outcome.status().clone(), cleanup);
    if let Some(output) = outcome.output().cloned() {
        finished = finished.with_output(output);
    }
    finished
}
