use crate::connection::{AcpConnection, AcpConnectionHooks, StandardAcpHooks};
use crate::diagnostics::AcpDiagnostics;
use crate::transport::AcpTransport;
use crate::turn::{
    AcpTurnHandle, AcpTurnSink, ActiveSlot, ActiveTask, ActiveTurn, ActiveTurnConfig,
    TurnCancellation,
};
use serde_json::{Value, json};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};
use swallowtail_core::{CancellationScope, ExecutionHostId, SessionRef};
use swallowtail_runtime::{
    BoxFuture, CancellationAcknowledgement, CancellationControl, CleanupOutcome, HostServices,
    InteractiveSessionHandle, JoinedTask, NegotiatedSessionModelOptions, RequestId, ResourceLease,
    RuntimeFailure, RuntimeSessionId, ScopeId, SessionCleanupRequest, SessionResumeBinding,
    TurnHandle, TurnRequest, bound_session_cleanup,
};

/// Turn request validation policy for [`AcpSessionHandle`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TurnValidationPolicy {
    /// Reject turn deadlines, attachments, and structured output.
    #[default]
    RejectDeadlineAttachmentsStructured,
}

/// Configuration for [`AcpSessionHandle`] and [`open_acp_session`].
#[derive(Clone, Copy, Debug)]
pub struct AcpSessionConfig {
    /// Prefix for `RuntimeSessionId` and turn `ScopeId` values (e.g. `"kiro-acp"`).
    pub scope_prefix: &'static str,
    /// Turn configuration (diagnostics, stop reason policy, usage progress policy).
    pub turn_config: ActiveTurnConfig,
    /// Turn validation policy.
    pub turn_validation: TurnValidationPolicy,
}

impl AcpSessionConfig {
    /// Creates a standard session configuration for a pure-ACP adapter.
    #[must_use]
    pub const fn new(scope_prefix: &'static str, diagnostics: AcpDiagnostics) -> Self {
        Self {
            scope_prefix,
            turn_config: ActiveTurnConfig::new(diagnostics),
            turn_validation: TurnValidationPolicy::RejectDeadlineAttachmentsStructured,
        }
    }
}

/// Session-level cancellation controller backed by [`AcpConnection::cancel_session`].
pub struct SessionCancellation {
    connection: Arc<AcpConnection>,
    requested: AtomicBool,
}

impl SessionCancellation {
    /// Creates a new session cancellation controller.
    #[must_use]
    pub fn new(connection: Arc<AcpConnection>) -> Self {
        Self {
            connection,
            requested: AtomicBool::new(false),
        }
    }
}

impl CancellationControl for SessionCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::InteractiveSession
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let already = self.requested.swap(true, Ordering::SeqCst);
        Box::pin(async move {
            if already {
                Ok(CancellationAcknowledgement::AlreadyRequested)
            } else {
                self.connection.cancel_session().await?;
                Ok(CancellationAcknowledgement::Requested)
            }
        })
    }
}

/// Shared [`InteractiveSessionHandle`] implementation for ACP v1 adapters.
///
/// Prevents `start_turn` session-wedging by validating task service and scope
/// before installing the active turn, and clears the active turn on any error.
pub struct AcpSessionHandle {
    config: AcpSessionConfig,
    request_id: RequestId,
    runtime_id: RuntimeSessionId,
    provider_ref: SessionRef,
    provider_id: String,
    execution_host_id: ExecutionHostId,
    connection: Arc<AcpConnection>,
    cancellation: SessionCancellation,
    pump_task: Option<Box<dyn JoinedTask>>,
    services: HostServices,
    resource: Option<ResourceLease>,
    active: ActiveSlot,
    negotiated_model_options: Option<NegotiatedSessionModelOptions>,
}

impl AcpSessionHandle {
    /// Attaches negotiated model options observed during `session/new`.
    #[must_use]
    pub fn with_negotiated_model_options(
        mut self,
        options: Option<NegotiatedSessionModelOptions>,
    ) -> Self {
        self.negotiated_model_options = options;
        self
    }
}

/// Bound runtime context required to open an [`AcpSessionHandle`].
pub struct AcpSessionOpenContext {
    /// Operation scope owning the session pump task.
    pub scope: ScopeId,
    /// Originating open-session request id.
    pub request_id: RequestId,
    /// Execution host bound by preflight.
    pub execution_host_id: ExecutionHostId,
    /// Resolved working-resource lease owned by the session.
    pub resource: ResourceLease,
    /// Host services for task spawning and cleanup.
    pub services: HostServices,
    /// Adapter session and turn configuration.
    pub config: AcpSessionConfig,
}

/// Opens a standard ACP session over `transport`, ensuring no process or pump leak on failure.
///
/// Validates `runtime_id` before spawning the pump task, and closes the connection and joins
/// the pump task if handshake or session creation fails.
pub async fn open_acp_session<F, Fut>(
    transport: AcpTransport,
    context: AcpSessionOpenContext,
    handshake: F,
) -> Result<AcpSessionHandle, Box<(RuntimeFailure, ResourceLease)>>
where
    F: FnOnce(Arc<AcpConnection>) -> Fut,
    Fut: std::future::Future<Output = Result<String, RuntimeFailure>>,
{
    open_acp_session_with_hooks(transport, context, Arc::new(StandardAcpHooks), handshake).await
}

/// Opens an ACP session with custom [`AcpConnectionHooks`], ensuring no transport/pump leak on failure.
pub async fn open_acp_session_with_hooks<F, Fut>(
    transport: AcpTransport,
    context: AcpSessionOpenContext,
    hooks: Arc<dyn AcpConnectionHooks>,
    handshake: F,
) -> Result<AcpSessionHandle, Box<(RuntimeFailure, ResourceLease)>>
where
    F: FnOnce(Arc<AcpConnection>) -> Fut,
    Fut: std::future::Future<Output = Result<String, RuntimeFailure>>,
{
    let AcpSessionOpenContext {
        scope,
        request_id,
        execution_host_id,
        resource,
        services,
        config,
    } = context;
    let diagnostics = config.turn_config.diagnostics;
    let runtime_id =
        match RuntimeSessionId::new(format!("{}:{}", config.scope_prefix, request_id.as_str())) {
            Ok(id) => id,
            Err(_) => {
                let connection = AcpConnection::new(transport, services, diagnostics, hooks);
                connection.begin_close().await;
                return Err(Box::new(((diagnostics.malformed)(), resource)));
            }
        };
    let task_service = match services.task().cloned() {
        Some(task_service) => task_service,
        None => {
            let connection = AcpConnection::new(transport, services, diagnostics, hooks);
            connection.begin_close().await;
            return Err(Box::new(((diagnostics.task_service_missing)(), resource)));
        }
    };
    let connection = AcpConnection::new(transport, services.clone(), diagnostics, hooks);
    let pump_connection = Arc::clone(&connection);
    let pump_task =
        match task_service.spawn(scope, Box::pin(async move { pump_connection.pump().await })) {
            Ok(task) => task,
            Err(error) => {
                let _ = connection.cancel_session().await;
                connection.pump().await;
                return Err(Box::new((error, resource)));
            }
        };
    let provider_id = match handshake(Arc::clone(&connection)).await {
        Ok(provider_id) => provider_id,
        Err(error) => {
            connection.begin_close().await;
            let _ = pump_task.join().await;
            return Err(Box::new((error, resource)));
        }
    };
    if let Err(error) = connection.set_session_id(provider_id.clone()) {
        connection.begin_close().await;
        let _ = pump_task.join().await;
        return Err(Box::new((error, resource)));
    }
    let provider_ref = match SessionRef::new(&provider_id) {
        Ok(provider_ref) => provider_ref,
        Err(_) => {
            connection.begin_close().await;
            let _ = pump_task.join().await;
            return Err(Box::new(((diagnostics.malformed)(), resource)));
        }
    };
    Ok(AcpSessionHandle {
        config,
        request_id,
        runtime_id,
        provider_ref,
        provider_id,
        execution_host_id,
        connection: Arc::clone(&connection),
        cancellation: SessionCancellation::new(connection),
        pump_task: Some(pump_task),
        services,
        resource: Some(resource),
        active: Arc::new(Mutex::new(None)),
        negotiated_model_options: None,
    })
}

impl InteractiveSessionHandle for AcpSessionHandle {
    fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    fn session_id(&self) -> &RuntimeSessionId {
        &self.runtime_id
    }

    fn provider_session_ref(&self) -> Option<&SessionRef> {
        Some(&self.provider_ref)
    }

    fn resume_binding(&self) -> Option<&SessionResumeBinding> {
        None
    }

    fn negotiated_model_options(&self) -> Option<&NegotiatedSessionModelOptions> {
        self.negotiated_model_options.as_ref()
    }

    fn start_turn<'a>(
        &'a mut self,
        request: TurnRequest,
        services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn TurnHandle>, RuntimeFailure>> {
        Box::pin(async move {
            let diagnostics = self.config.turn_config.diagnostics;
            services.require_execution_host(&self.execution_host_id)?;
            validate_turn(&request, self.config.turn_validation, diagnostics)?;
            reap_finished(&self.active).await?;
            if self
                .active
                .lock()
                .expect("ACP active-task lock poisoned")
                .is_some()
            {
                return Err((diagnostics.turn_active)());
            }
            // Validate task service and scope BEFORE registering the active turn
            // so a pre-flight validation error can never wedge the session.
            let task_service = services
                .task()
                .cloned()
                .ok_or_else(diagnostics.task_service_missing)?;
            let scope = ScopeId::new(format!(
                "{}:turn:{}",
                self.config.scope_prefix,
                request.turn_id().as_str()
            ))
            .map_err(|_| (diagnostics.malformed)())?;

            let (turn, events, terminal) = ActiveTurn::new(
                request.turn_id().clone(),
                self.provider_id.clone(),
                self.config.turn_config,
            )?;
            let turn_sink: Arc<dyn AcpTurnSink> = turn;
            self.connection.set_active_turn(Arc::clone(&turn_sink))?;

            let connection = Arc::clone(&self.connection);
            let prompt_turn = Arc::clone(&turn_sink);
            let prompt = request.content().as_str().to_owned();
            let session_id = self.provider_id.clone();

            let response = match connection
                .begin_request(
                    "session/prompt",
                    json!({
                        "sessionId": session_id,
                        "prompt": [{"type": "text", "text": prompt}]
                    }),
                )
                .await
            {
                Ok(response) => response,
                Err(error) => {
                    self.connection.clear_active_turn(&turn_sink);
                    turn_sink.fail(&error);
                    return Err(error);
                }
            };
            let task = match task_service.spawn(
                scope,
                Box::pin(async move {
                    let response = response.await;
                    match response {
                        Ok(response) => match response.get("stopReason").and_then(Value::as_str) {
                            Some(reason) => prompt_turn.finish_prompt(reason),
                            None => prompt_turn.fail(&(diagnostics.malformed)()),
                        },
                        Err(error) => prompt_turn.fail(&error),
                    }
                    connection.clear_active_turn(&prompt_turn);
                }),
            ) {
                Ok(task) => task,
                Err(error) => {
                    self.connection.clear_active_turn(&turn_sink);
                    turn_sink.fail(&error);
                    let _ = self.connection.cancel_session().await;
                    return Err(error);
                }
            };
            *self.active.lock().expect("ACP active-task lock poisoned") = Some(ActiveTask {
                turn: Arc::clone(&turn_sink),
                task: Some(task),
            });
            Ok(Box::new(AcpTurnHandle::new(
                request.turn_id().clone(),
                events,
                terminal,
                TurnCancellation::new(
                    Arc::clone(&self.connection),
                    self.provider_id.clone(),
                    turn_sink,
                ),
                Arc::clone(&self.active),
                diagnostics,
            )) as Box<dyn TurnHandle>)
        })
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        &self.cancellation
    }

    fn close(
        mut self: Box<Self>,
        request: SessionCleanupRequest,
        services: HostServices,
    ) -> BoxFuture<'static, CleanupOutcome> {
        let execution_host_id = self.execution_host_id.clone();
        let diagnostics = self.config.turn_config.diagnostics;
        bound_session_cleanup(
            execution_host_id,
            request,
            services,
            Box::pin(async move {
                let active = self
                    .active
                    .lock()
                    .expect("ACP active-task lock poisoned")
                    .take();
                if let Some(mut active) = active {
                    if !active.turn.is_finished() {
                        active.turn.mark_cancelled();
                        let _ = self
                            .connection
                            .notify("session/cancel", json!({"sessionId": self.provider_id}))
                            .await;
                    }
                    self.connection.begin_close().await;
                    if let Some(task) = active.task.take() {
                        let _ = task.join().await;
                    }
                } else {
                    self.connection.begin_close().await;
                }
                let task_cleanup = match self.pump_task.take() {
                    Some(task) => match task.join().await {
                        Ok(()) => self.connection.cleanup_outcome(),
                        Err(_) => CleanupOutcome::Failed((diagnostics.task_join_failed)()),
                    },
                    None => CleanupOutcome::NotApplicable,
                };
                let resource_cleanup = match self.resource.take() {
                    Some(resource) => match self.services.working_resource() {
                        Some(service) => service.release(resource).await,
                        None => CleanupOutcome::Failed((diagnostics.resource_release_failed)()),
                    },
                    None => CleanupOutcome::NotApplicable,
                };
                task_cleanup.merge(resource_cleanup)
            }),
        )
    }
}

fn validate_turn(
    request: &TurnRequest,
    policy: TurnValidationPolicy,
    diagnostics: AcpDiagnostics,
) -> Result<(), RuntimeFailure> {
    match policy {
        TurnValidationPolicy::RejectDeadlineAttachmentsStructured => {
            if request.deadline().is_some() {
                return Err((diagnostics.unsupported)("turn deadline"));
            }
            if request.attachments().len() != 0 {
                return Err((diagnostics.unsupported)("turn attachments"));
            }
            if request.structured_output().is_some() {
                return Err((diagnostics.unsupported)("structured output"));
            }
            Ok(())
        }
    }
}

async fn reap_finished(active: &ActiveSlot) -> Result<(), RuntimeFailure> {
    let finished = {
        let mut active = active.lock().expect("ACP active-task lock poisoned");
        if active
            .as_ref()
            .is_some_and(|active| active.turn.is_finished())
        {
            active.take()
        } else {
            None
        }
    };
    if let Some(mut finished) = finished
        && let Some(task) = finished.task.take()
    {
        task.join().await?;
    }
    Ok(())
}
