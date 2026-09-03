//! The one ordered cleanup continuation an enclosing guardian owns.
//!
//! Contract 019 fixes the order, and the guardian runs all of it inside a
//! single host task so no stage can be skipped by a caller that returned:
//! interrupt the live turn, ask the sidecar to close and join its own native
//! child, request host termination, observe the root process, join the pump,
//! release the working resource, then release the credential.
//!
//! Every lease release therefore happens after the scoped work that used it has
//! stopped. A caller whose deadline expires first transfers this whole guardian
//! — the process, the pump, and both leases together — through its pre-admitted
//! reap reservation. Nothing is released around still-live work, and nothing is
//! detached.
//!
//! The guardian's cooperative stages are bounded by the caller's own deadline,
//! so a sidecar that accepts input and never answers still reaches the host
//! termination request. Termination, root observation, the pump join, and the
//! two releases are then unconditional: they belong to the guardian, not to the
//! caller's future.

use crate::sdk::bounded::HostBound;
use crate::sdk::close::SidecarNativeJoin;
use crate::sdk::connection::SdkConnection;
use crate::sdk::wire::ClaudeAgentSdkCommand;
use serde_json::{Value, json};
use std::sync::Arc;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    CleanupOutcome, CredentialLease, HostServices, JoinedTask, ProcessExit, ProcessHandle,
    ProcessTreeCompletion, ResourceLease,
};

/// Bound the sidecar states, and honours, when joining its own retained
/// native child handle before the host escalates.
pub(crate) const CLOSE_JOIN_BOUND_MS: u64 = 2_000;

/// Everything one guardian owns for the whole ordered continuation.
#[derive(Default)]
pub(crate) struct Owned {
    pub(crate) connection: Option<Arc<SdkConnection>>,
    pub(crate) process: Option<Arc<dyn ProcessHandle>>,
    /// The protocol pump. It is only ever joined here, inside the guardian that
    /// owns it; it is never transferred on its own.
    pub(crate) pump: Option<Box<dyn JoinedTask>>,
    /// Other scoped work the guardian inherits, such as a turn's host-deadline
    /// task. Joined before the leases are released, for the same reason.
    pub(crate) scoped: Vec<Box<dyn JoinedTask>>,
    pub(crate) resource: Option<ResourceLease>,
    pub(crate) credential: Option<CredentialLease>,
}

impl Owned {
    pub(crate) fn take(&mut self) -> Self {
        Self {
            connection: self.connection.take(),
            process: self.process.take(),
            pump: self.pump.take(),
            scoped: std::mem::take(&mut self.scoped),
            resource: self.resource.take(),
            credential: self.credential.take(),
        }
    }
}

/// What the ordered continuation actually observed, in its own vocabulary.
///
/// Absent fields are absent observations. Nothing here is upgraded by the
/// guardian having merely finished.
pub(crate) struct CleanupReport {
    pub(crate) native_join: Option<SidecarNativeJoin>,
    pub(crate) pump_joined: bool,
    pub(crate) root_exit: Option<ProcessTreeCompletion>,
    pub(crate) resource: CleanupOutcome,
    pub(crate) credential: CleanupOutcome,
}

/// Whether the guardian first tries the sidecar's own cooperative close.
#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum Cooperative {
    /// A session that reached readiness: interrupt, then the close command.
    Session { turn_active: bool },
    /// An open attempt that never reached readiness. There is no live turn and
    /// no agreed protocol state, so the guardian goes straight to termination.
    None,
}

/// Runs the whole ordered cleanup. Called only from inside a guardian task.
pub(crate) async fn run(
    mut owned: Owned,
    services: &HostServices,
    bounded: &HostBound,
    request_id: &str,
    cooperative: Cooperative,
) -> CleanupReport {
    let connection = owned.connection.take();
    let native_join = match (&connection, cooperative) {
        (Some(connection), Cooperative::Session { turn_active }) => {
            cooperative_close(connection, bounded, request_id, turn_active).await
        }
        _ => None,
    };
    // The declared descendant termination attempt. It is a request through host
    // authority, made whether or not any cooperative stage answered.
    if let Some(connection) = &connection {
        let _ = connection.escalate().await;
    } else if let Some(process) = &owned.process {
        let _ = process.force_stop().await;
    }
    // Root/process observation, before any join is claimed.
    if let Some(process) = owned.process.take() {
        let _ = process.wait().await;
    }
    // Scoped work joined before either lease is released, so a release is
    // evidence that the work using it had already stopped.
    for task in std::mem::take(&mut owned.scoped) {
        let _ = task.join().await;
    }
    let pump_joined = match owned.pump.take() {
        Some(pump) => pump.join().await.is_ok(),
        None => false,
    };
    // Root exit is only readable once the pump that recorded it was joined.
    let root_exit = match (pump_joined, &connection) {
        (true, Some(connection)) => connection.observed_exit().map(ProcessExit::tree_completion),
        _ => None,
    };
    let resource = release_resource(owned.resource.take(), services).await;
    let credential = release_credential(owned.credential.take(), services).await;
    CleanupReport {
        native_join,
        pump_joined,
        root_exit,
        resource,
        credential,
    }
}

async fn cooperative_close(
    connection: &Arc<SdkConnection>,
    bounded: &HostBound,
    request_id: &str,
    turn_active: bool,
) -> Option<SidecarNativeJoin> {
    if turn_active {
        let id = format!("close-interrupt:{request_id}");
        let _ = bounded
            .run(connection.command(id, ClaudeAgentSdkCommand::Interrupt, json!({})))
            .await;
    }
    // The sidecar's own bounded native join. Bounded here as well, so a silent
    // sidecar cannot consume the whole cleanup budget inside this stage.
    let id = format!("close:{request_id}");
    let reported = bounded
        .run(connection.command(
            id,
            ClaudeAgentSdkCommand::Close,
            json!({"joinBoundMs": CLOSE_JOIN_BOUND_MS}),
        ))
        .await
        .and_then(Result::ok)
        .filter(|response| response.success)
        .and_then(|response| native_join(response.data.as_ref()));
    let _ = bounded.run(connection.begin_close()).await;
    reported
}

/// Reads the sidecar's report of its own direct native child.
///
/// A reported join must carry the observation that produced it, and must state
/// the exact bound this route declared.
pub(crate) fn native_join(data: Option<&Value>) -> Option<SidecarNativeJoin> {
    let data = data?;
    if data.get("joinBoundMs").and_then(Value::as_u64) != Some(CLOSE_JOIN_BOUND_MS) {
        return None;
    }
    let observed = data.get("nativeExitObserved").and_then(Value::as_bool)?;
    let join = SidecarNativeJoin::from_sidecar(data.get("nativeJoin")?.as_str()?)?;
    match (join, observed) {
        (SidecarNativeJoin::Exited, true) | (SidecarNativeJoin::Survivor, false) => Some(join),
        _ => None,
    }
}

async fn release_resource(lease: Option<ResourceLease>, services: &HostServices) -> CleanupOutcome {
    match (lease, services.working_resource()) {
        (Some(lease), Some(service)) => service.release(lease).await,
        (Some(_), None) => cleanup_failure(
            "swallowtail.claude-agent.sdk.resource_release_failed",
            "Claude Agent SDK sidecar working-resource service disappeared during cleanup",
        ),
        (None, _) => CleanupOutcome::NotApplicable,
    }
}

async fn release_credential(
    lease: Option<CredentialLease>,
    services: &HostServices,
) -> CleanupOutcome {
    match (lease, services.credential()) {
        (Some(lease), Some(service)) => service.release(lease).await,
        (Some(_), None) => cleanup_failure(
            "swallowtail.claude-agent.sdk.credential_release_failed",
            "Claude Agent SDK sidecar credential service disappeared during cleanup",
        ),
        (None, _) => CleanupOutcome::NotApplicable,
    }
}

fn cleanup_failure(code: &'static str, message: &'static str) -> CleanupOutcome {
    CleanupOutcome::Failed(SafeDiagnostic::new(code, message))
}
