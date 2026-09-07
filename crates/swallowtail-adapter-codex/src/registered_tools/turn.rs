//! One turn-scoped registered-tool lease bound to one Codex turn attempt.
//!
//! The lease is checked out for the exact duration of one dispatch and checked
//! back in before its result reaches the provider. Teardown joins that dispatch
//! before it takes the lease back, so a call is never abandoned silently and a
//! failed join never reports a clean close: the lease stays owned here and the
//! turn reports the failure.

use super::binding::CodexRegisteredToolBinding;
use super::result::provider_result;
use crate::rpc::{RpcConnection, failure};
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

/// Registered-tool state owned by exactly one Codex turn attempt.
pub(crate) struct CodexRegisteredTurn {
    binding: CodexRegisteredToolBinding,
    port: Arc<dyn RegisteredToolBridgeHostService>,
    tasks: Arc<dyn ScopedTaskService>,
    connection: Weak<RpcConnection>,
    scope: ScopeId,
    /// The lease, present whenever no dispatch holds it.
    lease: Mutex<Option<RegisteredToolBridgeLease>>,
    /// Every dispatch this turn started, all joined before teardown closes.
    dispatches: Mutex<Vec<Box<dyn JoinedTask>>>,
    closed: Mutex<bool>,
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
            lease: Mutex::new(Some(lease)),
            dispatches: Mutex::new(Vec::new()),
            closed: Mutex::new(false),
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
        if self
            .dispatches
            .lock()
            .expect("registered dispatch lock poisoned")
            .len()
            >= MAX_REGISTERED_DISPATCHES_PER_TURN
        {
            return Err(dispatch_bound_failure());
        }
        let lease = self
            .lease
            .lock()
            .expect("registered lease lock poisoned")
            .take()
            .ok_or_else(outstanding_call_failure)?;
        let prepared = self.prepare_call(&lease, provider_call_id, tool, arguments, deadline);
        let request = match prepared {
            Ok(request) => request,
            Err(error) => {
                self.check_in(lease);
                return Err(error);
            }
        };
        let owner = Arc::clone(self);
        let task = self.tasks.spawn(
            self.scope.clone(),
            Box::pin(async move {
                let settled = lease.call(request).await;
                let (result, _executed) = provider_result(settled);
                // The lease returns before the provider is answered, so joined
                // teardown never waits on the response write.
                owner.check_in(lease);
                if let Some(connection) = owner.connection.upgrade() {
                    let _ = connection
                        .respond_server_request(provider_request_id, result)
                        .await;
                }
            }),
        );
        match task {
            Ok(task) => {
                self.dispatches
                    .lock()
                    .expect("registered dispatch lock poisoned")
                    .push(task);
                Ok(())
            }
            Err(error) => Err(error),
        }
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

    fn check_in(&self, lease: RegisteredToolBridgeLease) {
        *self.lease.lock().expect("registered lease lock poisoned") = Some(lease);
    }

    /// Freezes new registered admission when no dispatch is outstanding.
    ///
    /// Cancellation cannot take a lease a live dispatch holds, so an
    /// outstanding call keeps settling under its own deadline and is joined by
    /// teardown. This never reports a close and never abandons the call.
    pub(crate) async fn freeze_when_clear(&self) {
        let held = self
            .lease
            .lock()
            .expect("registered lease lock poisoned")
            .take();
        let Some(lease) = held else {
            return;
        };
        let _ = self.port.completion_gate(&lease).await;
        self.check_in(lease);
    }

    /// Takes the lease back, joins its dispatches, and closes.
    ///
    /// A lease a live dispatch still holds is never taken, never abandoned, and
    /// never reported clean. Ownership stays here, the defensive release runs
    /// when the dispatch returns it, and the turn reports a failed cleanup.
    pub(crate) async fn close(&self, cause: RegisteredToolCleanupCause) -> CleanupOutcome {
        if std::mem::replace(
            &mut *self.closed.lock().expect("registered close lock poisoned"),
            true,
        ) {
            return CleanupOutcome::NotApplicable;
        }
        let lease = self
            .lease
            .lock()
            .expect("registered lease lock poisoned")
            .take();
        let Some(lease) = lease else {
            return CleanupOutcome::Failed(SafeDiagnostic::new(
                "swallowtail.codex.app_server.registered_lease_retained",
                "Codex registered tool lease is still held by an unsettled dispatch",
            ));
        };
        // Every dispatch returned the lease before its response, so these joins
        // are bounded by the responses already in flight.
        let dispatches = std::mem::take(
            &mut *self
                .dispatches
                .lock()
                .expect("registered dispatch lock poisoned"),
        );
        let mut joined = Ok(());
        for task in dispatches {
            if let Err(error) = task.join().await {
                joined = Err(error);
            }
        }
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
