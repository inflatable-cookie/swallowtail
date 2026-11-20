//! One turn-scoped registered-tool lease bound to one Codex turn attempt.
//!
//! A dispatch holds the lease through the kernel call. Closing freezes admission,
//! joins every dispatch, then takes the lease back and closes the kernel scope.

use super::binding::CodexRegisteredToolBinding;
use super::result::provider_result;
use crate::rpc::{RpcConnection, failure};
use futures_util::lock::Mutex as AsyncMutex;
use serde_json::Value;
use std::sync::{Arc, Mutex, Weak};
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    CleanupOutcome, Deadline, JoinedTask, RegisteredToolBridgeHostService,
    RegisteredToolBridgeLease, RegisteredToolCallId, RegisteredToolCallRequest,
    RegisteredToolCleanupCause, RegisteredToolCompletionState, RegisteredToolId,
    RegisteredToolPayload, RegisteredToolSchemaMediaType, RuntimeFailure, ScopeId,
    ScopedTaskService,
};

const CODEX_ARGUMENT_MEDIA_TYPE: &str = "application/json";
/// Bounded registered dispatch count for one turn attempt.
const MAX_REGISTERED_DISPATCHES_PER_TURN: usize = 64;

struct Dispatches {
    accepting: bool,
    tasks: Vec<Box<dyn JoinedTask>>,
}

/// Registered-tool state owned by exactly one Codex turn attempt.
pub(crate) struct CodexRegisteredTurn {
    binding: CodexRegisteredToolBinding,
    port: Arc<dyn RegisteredToolBridgeHostService>,
    tasks: Arc<dyn ScopedTaskService>,
    connection: Weak<RpcConnection>,
    scope: ScopeId,
    lease: Arc<AsyncMutex<Option<RegisteredToolBridgeLease>>>,
    /// Admission and task registration share one lock with close.
    dispatches: Mutex<Option<Dispatches>>,
}

impl CodexRegisteredTurn {
    pub(crate) fn new(
        binding: CodexRegisteredToolBinding,
        port: Arc<dyn RegisteredToolBridgeHostService>,
        tasks: Arc<dyn ScopedTaskService>,
        connection: Weak<RpcConnection>,
        scope: ScopeId,
        lease: RegisteredToolBridgeLease,
    ) -> Self {
        Self {
            binding,
            port,
            tasks,
            connection,
            scope,
            lease: Arc::new(AsyncMutex::new(Some(lease))),
            dispatches: Mutex::new(Some(Dispatches {
                accepting: true,
                tasks: Vec::new(),
            })),
        }
    }

    /// Returns the namespaced identity one declared dynamic tool name binds.
    pub(crate) fn tool_for(&self, wire_name: &str) -> Option<RegisteredToolId> {
        self.binding.tool_for(wire_name).cloned()
    }

    /// Dispatches one provider tool call through the registered kernel.
    ///
    /// The provider call id is the call correlation, so a reused id is refused
    /// by the kernel's duplicate check instead of dispatching twice.
    pub(crate) fn dispatch(
        self: &Arc<Self>,
        provider_request_id: Value,
        provider_call_id: &str,
        tool: RegisteredToolId,
        arguments: &Value,
        deadline: Deadline,
    ) -> Result<(), RuntimeFailure> {
        let mut dispatches = self
            .dispatches
            .lock()
            .expect("registered dispatch lock poisoned");
        let dispatches = dispatches
            .as_mut()
            .filter(|state| state.accepting)
            .ok_or_else(|| {
                failure(
                    "swallowtail.codex.app_server.registered_admission_closed",
                    "Codex registered tool admission is closed",
                )
            })?;
        if dispatches.tasks.len() >= MAX_REGISTERED_DISPATCHES_PER_TURN {
            return Err(dispatch_bound_failure());
        }
        let lease = self
            .lease
            .try_lock_owned()
            .ok_or_else(outstanding_call_failure)?;
        let request = self.prepare_call(
            lease.as_ref().expect("an accepting turn owns a lease"),
            provider_call_id,
            tool,
            arguments,
            deadline,
        )?;
        let connection = self.connection.clone();
        let task = self.tasks.spawn(
            self.scope.clone(),
            Box::pin(async move {
                let settled = lease
                    .as_ref()
                    .expect("dispatch owns a lease")
                    .call(request)
                    .await;
                drop(lease);
                let (result, _executed) = provider_result(settled);
                if let Some(connection) = connection.upgrade() {
                    let _ = connection
                        .respond_server_request(provider_request_id, result)
                        .await;
                }
            }),
        )?;
        dispatches.tasks.push(task);
        Ok(())
    }

    fn prepare_call(
        &self,
        lease: &RegisteredToolBridgeLease,
        provider_call_id: &str,
        tool: RegisteredToolId,
        arguments: &Value,
        deadline: Deadline,
    ) -> Result<RegisteredToolCallRequest, RuntimeFailure> {
        let call_id = RegisteredToolCallId::new(provider_call_id).map_err(|error| {
            let _ = error;
            failure(
                "swallowtail.codex.app_server.registered_call_identity_rejected",
                "Codex app-server supplied an unusable registered tool call id",
            )
        })?;
        let bytes = serde_json::to_vec(arguments).map_err(|_| {
            failure(
                "swallowtail.codex.app_server.registered_arguments_invalid",
                "Codex registered tool arguments are not valid JSON",
            )
        })?;
        let media_type = RegisteredToolSchemaMediaType::new(CODEX_ARGUMENT_MEDIA_TYPE)
            .expect("static registered argument media type is valid");
        let payload = RegisteredToolPayload::new(
            media_type,
            bytes,
            lease.selection().effective_bounds().max_argument_bytes(),
        )
        .map_err(|_| {
            failure(
                "swallowtail.codex.app_server.registered_arguments_too_large",
                "Codex registered tool arguments exceeded the selected bound",
            )
        })?;
        Ok(RegisteredToolCallRequest::new(
            call_id, tool, payload, deadline,
        ))
    }

    /// Freezes admission and gates the kernel immediately if its lease is free.
    pub(crate) async fn freeze_when_clear(&self) {
        {
            let mut dispatches = self
                .dispatches
                .lock()
                .expect("registered dispatch lock poisoned");
            let Some(dispatches) = dispatches.as_mut() else {
                return;
            };
            dispatches.accepting = false;
        }
        if let Some(lease) = self.lease.try_lock()
            && let Some(lease) = lease.as_ref()
        {
            let _ = self.port.completion_gate(lease).await;
        }
    }

    /// Freezes admission, joins every dispatch, and closes the returned lease.
    pub(crate) async fn close(&self, cause: RegisteredToolCleanupCause) -> CleanupOutcome {
        let dispatches = self
            .dispatches
            .lock()
            .expect("registered dispatch lock poisoned")
            .take();
        let Some(dispatches) = dispatches else {
            return CleanupOutcome::NotApplicable;
        };
        let mut joined = Ok(());
        for task in dispatches.tasks {
            if let Err(error) = task.join().await {
                joined = Err(error);
            }
        }
        let lease = self
            .lease
            .lock()
            .await
            .take()
            .expect("first close owns the lease");
        // Observe before close: a lease that still holds outstanding work must
        // not be reported as a clean completion barrier.
        let completion = self.port.completion_gate(&lease).await;
        let outcome = self.port.close(lease, cause).await;
        cleanup_outcome(joined, completion, outcome)
    }
}

fn cleanup_outcome(
    joined: Result<(), RuntimeFailure>,
    completion: Result<RegisteredToolCompletionState, RuntimeFailure>,
    closed: Result<CleanupOutcome, RuntimeFailure>,
) -> CleanupOutcome {
    if joined.is_err() {
        return CleanupOutcome::Failed(SafeDiagnostic::new(
            "swallowtail.codex.app_server.registered_dispatch_join_failed",
            "Codex registered tool dispatch did not join",
        ));
    }
    match completion {
        Ok(state) if !state.allows_successful_completion() => {
            return CleanupOutcome::Failed(SafeDiagnostic::new(
                "swallowtail.codex.app_server.registered_completion_unclear",
                "Codex registered tool lease did not reach a clear completion barrier",
            ));
        }
        Ok(_) => {}
        Err(error) => return CleanupOutcome::Failed(error.diagnostic().clone()),
    }
    match closed {
        Ok(outcome) => outcome,
        Err(error) => CleanupOutcome::Failed(error.diagnostic().clone()),
    }
}

fn dispatch_bound_failure() -> RuntimeFailure {
    failure(
        "swallowtail.codex.app_server.registered_dispatch_bound_exceeded",
        "Codex turn exceeded its bounded registered tool dispatch count",
    )
}

fn outstanding_call_failure() -> RuntimeFailure {
    failure(
        "swallowtail.codex.app_server.registered_call_outstanding",
        "Codex registered tool lease already has an outstanding call",
    )
}
