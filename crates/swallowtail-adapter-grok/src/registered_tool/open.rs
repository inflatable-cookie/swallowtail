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
///
/// Settlement is serialized: exactly one caller closes the lease, and every
/// concurrent caller awaits that same completion instead of reading a
/// not-yet-recorded outcome. Cancellation, turn terminal, transport failure,
/// and session close all race for this, and a caller that saw
/// `NotApplicable` while a close was still joining could publish terminal or
/// release the credential beside work the host had not finished.
pub(crate) struct GrokRegisteredToolSession {
    state: Mutex<SettleState>,
    turn: RuntimeTurnId,
}

#[derive(Default)]
struct SettleState {
    lease: Option<RegisteredToolBridgeLease>,
    outcome: Option<CleanupOutcome>,
    settling: bool,
    waiters: Vec<futures_channel::oneshot::Sender<CleanupOutcome>>,
}

/// What one `settle` caller must do to obtain the exact cleanup truth.
///
/// The lease is boxed only to keep the variants comparable in size; exactly
/// one caller ever receives it.
enum SettleStep {
    Recorded(CleanupOutcome),
    Await(futures_channel::oneshot::Receiver<CleanupOutcome>),
    Close(Box<RegisteredToolBridgeLease>),
}

impl GrokRegisteredToolSession {
    fn new(lease: RegisteredToolBridgeLease, turn: RuntimeTurnId) -> Self {
        Self {
            state: Mutex::new(SettleState {
                lease: Some(lease),
                ..SettleState::default()
            }),
            turn,
        }
    }

    /// Returns the exact turn attempt this lease was opened for.
    pub(crate) fn turn(&self) -> &RuntimeTurnId {
        &self.turn
    }

    /// Reports whether the lease settled and released everything it held.
    ///
    /// A settlement still in flight is not clean, and neither is a failed one:
    /// in both cases the host may still hold the lease and work it could not
    /// join, so the operation is not over.
    pub(crate) fn settled_clean(&self) -> bool {
        let state = self.locked();
        !state.settling
            && matches!(
                state.outcome.as_ref(),
                Some(CleanupOutcome::Clean | CleanupOutcome::NotApplicable)
            )
    }

    /// Reports whether a joined cleanup attempt already failed.
    pub(crate) fn cleanup_failed(&self) -> bool {
        matches!(
            self.locked().outcome.as_ref(),
            Some(CleanupOutcome::Failed(_) | CleanupOutcome::Degraded(_))
        )
    }

    /// Freezes admission, joins issued work, and closes with its exact cause.
    ///
    /// The first caller performs the close; every other caller awaits its
    /// result and receives the same cleanup truth. A settled lease reports its
    /// retained outcome rather than closing twice.
    pub(crate) async fn settle(
        &self,
        services: &HostServices,
        cause: RegisteredToolCleanupCause,
    ) -> CleanupOutcome {
        let step = {
            let mut state = self.locked();
            if let Some(outcome) = state.outcome.clone() {
                SettleStep::Recorded(outcome)
            } else if state.settling {
                let (sender, receiver) = futures_channel::oneshot::channel();
                state.waiters.push(sender);
                SettleStep::Await(receiver)
            } else if let Some(lease) = state.lease.take() {
                state.settling = true;
                SettleStep::Close(Box::new(lease))
            } else {
                SettleStep::Recorded(CleanupOutcome::NotApplicable)
            }
        };
        match step {
            SettleStep::Recorded(outcome) => outcome,
            // A settlement whose result cannot be observed is never clean.
            SettleStep::Await(receiver) => receiver.await.unwrap_or_else(|_| {
                CleanupOutcome::Failed(SafeDiagnostic::new(
                    "swallowtail.grok.acp.registered_tool.cleanup_unobserved",
                    "Grok Build registered-tool cleanup result was not observed",
                ))
            }),
            SettleStep::Close(lease) => {
                let outcome = close_lease(*lease, services, cause).await;
                let waiters = {
                    let mut state = self.locked();
                    state.outcome = Some(outcome.clone());
                    state.settling = false;
                    std::mem::take(&mut state.waiters)
                };
                for waiter in waiters {
                    let _ = waiter.send(outcome.clone());
                }
                outcome
            }
        }
    }

    fn locked(&self) -> std::sync::MutexGuard<'_, SettleState> {
        self.state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
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
        GrokRegisteredToolSession::new(
            self.lease.take().expect("registered lease is present"),
            self.turn.clone(),
        )
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
    close_lease(lease, services, cause).await
}

async fn close_lease(
    lease: RegisteredToolBridgeLease,
    services: &HostServices,
    cause: RegisteredToolCleanupCause,
) -> CleanupOutcome {
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
