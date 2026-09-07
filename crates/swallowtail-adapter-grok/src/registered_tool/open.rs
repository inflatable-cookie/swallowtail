//! Session-open binding and turn-scoped lifecycle of one registered courier.
//!
//! The courier process is Grok's child: Swallowtail declares it in the ACP
//! `mcpServers` list and the provider spawns it. This route owns the bridge
//! lease only; it never holds a Swallowtail-spawned courier process handle.
//!
//! The lease is bound to exactly one ACP turn attempt, which is the Contract
//! 063 first-tranche rule of one active provider turn per server lease. Grok
//! spawns the courier from `session/new`, before any turn exists, so the
//! consumer names that turn on the binding and the route refuses to start any
//! other turn while the lease is live. Cancellation, deadline, and turn
//! terminal freeze admission before settling; the close that follows carries
//! the exact cause and its cleanup truth is never discarded.

use super::binding::GrokRegisteredToolBinding;
use super::declaration::GrokAcpMcpServerDeclaration;
use crate::failure::failure;
use std::sync::Mutex;
use swallowtail_core::{PreflightPlan, SafeDiagnostic};
use swallowtail_host_local::RegisteredToolProxyLaunch;
use swallowtail_runtime::{
    CleanupOutcome, HostServices, RegisteredToolBridgeLease, RegisteredToolCleanupCause,
    RegisteredToolFailure, RegisteredToolReadiness, RequestId, RuntimeFailure, RuntimeTurnId,
    ScopeId,
};

/// Live registered-tool lease bound to one open ACP session and one turn.
pub(crate) struct GrokRegisteredToolSession {
    lease: Mutex<Option<RegisteredToolBridgeLease>>,
    turn: RuntimeTurnId,
    settled: Mutex<Option<CleanupOutcome>>,
}

impl GrokRegisteredToolSession {
    /// Returns the exact turn attempt this lease was opened for.
    pub(crate) fn turn(&self) -> &RuntimeTurnId {
        &self.turn
    }

    /// Reports whether the lease has already been settled and closed.
    pub(crate) fn is_settled(&self) -> bool {
        self.settled
            .lock()
            .expect("registered settle lock poisoned")
            .is_some()
    }

    /// Freezes admission without closing, ahead of settling or abandoning.
    ///
    /// Contract 063 requires cancellation, deadline, and terminal to freeze
    /// admission first. The completion gate is the only freeze point; it
    /// observes outstanding work and never silently waits.
    pub(crate) async fn freeze(&self, services: &HostServices) {
        let Some(bridge) = services.registered_tool_bridge() else {
            return;
        };
        let frozen = {
            let lease = self.lease.lock().expect("registered lease lock poisoned");
            lease.as_ref().map(|lease| bridge.completion_gate(lease))
        };
        if let Some(frozen) = frozen {
            let _ = frozen.await;
        }
    }

    /// Freezes, joins, and closes this lease with its exact cause.
    ///
    /// The cleanup truth is retained, not discarded: a failed close is
    /// reported by [`Self::cleanup_outcome`] and never becomes a clean session
    /// close. Settling twice is a no-op that keeps the first outcome.
    pub(crate) async fn settle(
        &self,
        services: &HostServices,
        cause: RegisteredToolCleanupCause,
    ) -> CleanupOutcome {
        let taken = self
            .lease
            .lock()
            .expect("registered lease lock poisoned")
            .take();
        let Some(lease) = taken else {
            return self.cleanup_outcome();
        };
        let outcome = match services.registered_tool_bridge() {
            Some(bridge) => {
                // Freeze first: the gate observes outstanding work and never
                // turns provider-terminal into success.
                let _ = bridge.completion_gate(&lease).await;
                match bridge.close(lease, cause).await {
                    Ok(outcome) => outcome,
                    Err(error) => CleanupOutcome::Failed(error.diagnostic().clone()),
                }
            }
            // The lease exists only because the bridge minted it. A bridge
            // that disappeared cannot be a clean close.
            None => CleanupOutcome::Failed(SafeDiagnostic::new(
                "swallowtail.grok.acp.registered_tool.bridge_missing",
                "Grok Build registered-tool bridge disappeared during cleanup",
            )),
        };
        *self
            .settled
            .lock()
            .expect("registered settle lock poisoned") = Some(outcome.clone());
        outcome
    }

    /// Returns the retained cleanup truth of this lease.
    pub(crate) fn cleanup_outcome(&self) -> CleanupOutcome {
        self.settled
            .lock()
            .expect("registered settle lock poisoned")
            .clone()
            .unwrap_or(CleanupOutcome::NotApplicable)
    }
}

/// Partial registered-tool open that never holds a courier process handle.
pub(crate) struct PendingRegisteredOpen {
    lease: Option<RegisteredToolBridgeLease>,
    launch: Option<RegisteredToolProxyLaunch>,
    declaration: GrokAcpMcpServerDeclaration,
    turn: RuntimeTurnId,
    claimed: bool,
}

impl PendingRegisteredOpen {
    pub(crate) fn declaration(&self) -> &GrokAcpMcpServerDeclaration {
        &self.declaration
    }

    pub(crate) fn wait_until_ready(&mut self) -> Result<(), RuntimeFailure> {
        self.launch
            .as_mut()
            .expect("registered launch is present until claimed")
            .wait_until_ready()
    }

    pub(crate) fn claim(mut self) -> GrokRegisteredToolSession {
        self.claimed = true;
        drop(self.launch.take());
        GrokRegisteredToolSession {
            lease: Mutex::new(Some(
                self.lease.take().expect("registered lease is present"),
            )),
            turn: self.turn.clone(),
            settled: Mutex::new(None),
        }
    }

    /// Closes a lease that never reached an open session, with its truth.
    pub(crate) async fn abandon(
        mut self,
        services: &HostServices,
        cause: RegisteredToolCleanupCause,
    ) -> CleanupOutcome {
        self.claimed = true;
        drop(self.launch.take());
        close_registered_lease(self.lease.take(), services, cause).await
    }
}

impl Drop for PendingRegisteredOpen {
    fn drop(&mut self) {
        if self.claimed {
            return;
        }
        drop(self.launch.take());
        drop(self.lease.take());
    }
}

/// Closes one minted lease and reports its exact cleanup truth.
pub(crate) async fn close_registered_lease(
    lease: Option<RegisteredToolBridgeLease>,
    services: &HostServices,
    cause: RegisteredToolCleanupCause,
) -> CleanupOutcome {
    let Some(lease) = lease else {
        return CleanupOutcome::NotApplicable;
    };
    match services.registered_tool_bridge() {
        Some(bridge) => {
            let _ = bridge.completion_gate(&lease).await;
            match bridge.close(lease, cause).await {
                Ok(outcome) => outcome,
                Err(error) => CleanupOutcome::Failed(error.diagnostic().clone()),
            }
        }
        None => CleanupOutcome::Failed(SafeDiagnostic::new(
            "swallowtail.grok.acp.registered_tool.bridge_missing",
            "Grok Build registered-tool bridge disappeared during cleanup",
        )),
    }
}

pub(crate) async fn prepare_registered(
    binding: &GrokRegisteredToolBinding,
    plan: &PreflightPlan,
    request_id: &RequestId,
    services: &HostServices,
) -> Result<PendingRegisteredOpen, RuntimeFailure> {
    let host = binding.require_host()?;
    let deadline = binding.require_deadline()?;
    let turn = binding.require_turn()?.clone();
    let recipe = binding
        .selection()
        .proxy_recipe()
        .expect("qualify requires a proxy recipe");
    let command = host
        .approved_executable_path(recipe.executable())
        .and_then(|path| path.to_str().map(str::to_owned))
        .filter(|command| !command.is_empty())
        .ok_or_else(|| {
            failure(
                "swallowtail.grok.acp.registered_tool.command_unresolved",
                "Grok Build ACP registered-tool open requires a host-approved native courier path",
            )
        })?;
    if !std::path::Path::new(&command).is_file() {
        return Err(failure(
            "swallowtail.grok.acp.registered_tool.command_unspawnable",
            "Grok Build ACP registered-tool courier command is not a spawnable filesystem path",
        ));
    }
    let environment = declared_environment(
        host.approved_environment(recipe.environment())
            .ok_or_else(|| {
                failure(
                    "swallowtail.grok.acp.registered_tool.environment_unresolved",
                    "Grok Build ACP registered-tool open requires a host-approved courier environment",
                )
            })?,
    )?;
    RegisteredToolReadiness::evaluate(services, binding.selection())
        .require_ready()
        .map_err(RegisteredToolFailure::into_runtime_failure)?;
    let scope = ScopeId::new(format!("grok-acp:registered-tool:{}", request_id.as_str()))
        .map_err(|_| registered_identity_rejected())?;
    let prepared = binding.preparation().prepare(
        services,
        plan.instance_id().clone(),
        scope,
        turn.clone(),
        deadline,
    )?;
    let lease = prepared.open().await?;
    // Every failure after the lease is minted closes it explicitly and reports
    // its cleanup truth. Drop alone is defensive, never close evidence.
    let launch = match host.registered_tool_proxy_launch(&lease) {
        Ok(launch) => launch,
        Err(error) => {
            return Err(merge_open_failure(
                error,
                close_registered_lease(
                    Some(lease),
                    services,
                    RegisteredToolCleanupCause::ProviderFailure,
                )
                .await,
            ));
        }
    };
    let declaration = GrokAcpMcpServerDeclaration::courier(
        binding.carrier().server_name(),
        command,
        launch
            .process_request()
            .arguments()
            .map(str::to_owned)
            .collect(),
        environment,
    );
    Ok(PendingRegisteredOpen {
        lease: Some(lease),
        launch: Some(launch),
        declaration,
        turn,
        claimed: false,
    })
}

/// Reports the original open failure, or the cleanup failure that hid it.
///
/// A failed close is never invisible: when both happen the cleanup failure is
/// surfaced, because a retained lease is the more serious truth.
fn merge_open_failure(error: RuntimeFailure, cleanup: CleanupOutcome) -> RuntimeFailure {
    match cleanup {
        CleanupOutcome::Failed(diagnostic) | CleanupOutcome::Degraded(diagnostic) => {
            RuntimeFailure::new(diagnostic)
        }
        CleanupOutcome::Clean | CleanupOutcome::NotApplicable => error,
    }
}

/// Renders the host-approved recipe environment for the ACP declaration.
///
/// Swallowtail-private bindings are dropped: the rendezvous file carries the
/// endpoint, bearer, and generations, so nothing named `SWALLOWTAIL_` needs to
/// cross an ACP wire the provider can read.
fn declared_environment(
    bindings: &[(std::ffi::OsString, std::ffi::OsString)],
) -> Result<Vec<(String, String)>, RuntimeFailure> {
    let mut environment = Vec::new();
    for (key, value) in bindings {
        let (Some(key), Some(value)) = (key.to_str(), value.to_str()) else {
            return Err(failure(
                "swallowtail.grok.acp.registered_tool.environment_unresolved",
                "Grok Build ACP registered-tool environment must be UTF-8",
            ));
        };
        if key.starts_with("SWALLOWTAIL_") {
            continue;
        }
        environment.push((key.to_owned(), value.to_owned()));
    }
    Ok(environment)
}

fn registered_identity_rejected() -> RuntimeFailure {
    failure(
        "swallowtail.grok.acp.registered_tool.identity_rejected",
        "Grok Build ACP registered-tool open could not bind its scope and turn identity",
    )
}
