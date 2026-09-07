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
use crate::sdk::close::{CLOSE_JOIN_BOUND_MS, SidecarCloseEvidence, SidecarNativeJoin};
use crate::sdk::connection::SdkConnection;
use crate::sdk::failure::command_rejected;
use crate::sdk::wire::ClaudeAgentSdkCommand;
use serde_json::json;
use std::sync::Arc;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    CleanupOutcome, CredentialLease, HostServices, JoinedTask, ProcessExit, ProcessHandle,
    ProcessTreeCompletion, RegisteredToolBridgeLease, RegisteredToolCleanupCause, ResourceLease,
};

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
    pub(crate) registered: Option<RegisteredToolBridgeLease>,
    pub(crate) registered_cause: Option<RegisteredToolCleanupCause>,
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
            registered: self.registered.take(),
            registered_cause: self.registered_cause.take(),
        }
    }
}

/// What the ordered continuation actually observed, in its own vocabulary.
///
/// Absent fields are absent observations. Nothing here is upgraded by the
/// guardian having merely finished.
pub(crate) struct CleanupReport {
    pub(crate) close_evidence: Option<SidecarCloseEvidence>,
    pub(crate) native_join: Option<SidecarNativeJoin>,
    pub(crate) cooperative_failure: Option<SafeDiagnostic>,
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
    if let Some(lease) = owned.registered.take() {
        crate::sdk::driver::registered::close_registered_lease(
            Some(lease),
            services,
            owned
                .registered_cause
                .take()
                .unwrap_or(RegisteredToolCleanupCause::ProviderFailure),
        )
        .await;
    }
    let (close_evidence, cooperative_failure) = match (&connection, cooperative) {
        (Some(connection), Cooperative::Session { turn_active }) => {
            cooperative_close(connection, bounded, request_id, turn_active).await
        }
        _ => (None, None),
    };
    let native_join = close_evidence.as_ref().map(|evidence| evidence.native_join);
    // The declared descendant termination attempt. It is a request through host
    // authority, made whether or not any cooperative stage answered.
    if let Some(connection) = &connection {
        let _ = connection.escalate().await;
    } else if let Some(process) = &owned.process {
        let _ = process.force_stop().await;
    }
    // Root/process observation, before any join is claimed. Keep this result:
    // the pump also waits on the same host handle, and a host is allowed to
    // make that wait consumptive. Dropping this result would turn an observed
    // root exit into `close_root_unconfirmed` after the pump joined.
    let process_root_exit = if let Some(process) = owned.process.take() {
        process.wait().await.ok().map(ProcessExit::tree_completion)
    } else {
        None
    };
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
    let pump_root_exit = match (pump_joined, &connection) {
        (true, Some(connection)) => connection.observed_exit().map(ProcessExit::tree_completion),
        _ => None,
    };
    let root_exit = observed_root_exit(pump_joined, pump_root_exit, process_root_exit);
    let resource = release_resource(owned.resource.take(), services).await;
    let credential = release_credential(owned.credential.take(), services).await;
    CleanupReport {
        close_evidence,
        native_join,
        cooperative_failure,
        pump_joined,
        root_exit,
        resource,
        credential,
    }
}

fn observed_root_exit(
    pump_joined: bool,
    pump_root_exit: Option<ProcessTreeCompletion>,
    process_root_exit: Option<ProcessTreeCompletion>,
) -> Option<ProcessTreeCompletion> {
    pump_joined
        .then(|| pump_root_exit.or(process_root_exit))
        .flatten()
}

async fn cooperative_close(
    connection: &Arc<SdkConnection>,
    bounded: &HostBound,
    request_id: &str,
    turn_active: bool,
) -> (Option<SidecarCloseEvidence>, Option<SafeDiagnostic>) {
    let mut cooperative_failure = None;
    if turn_active {
        let id = format!("close-interrupt:{request_id}");
        if let Some(Ok(response)) = bounded
            .run(connection.command(id, ClaudeAgentSdkCommand::Interrupt, json!({})))
            .await
            && !response.success
        {
            cooperative_failure = Some(
                command_rejected(
                    "swallowtail.claude-agent.sdk.interrupt_rejected",
                    "Claude Agent SDK sidecar rejected the close interrupt",
                    response
                        .failure_code
                        .expect("a rejected response carries its fixed sidecar code"),
                )
                .diagnostic()
                .clone(),
            );
        }
    }
    // The sidecar's own bounded native join. Bounded here as well, so a silent
    // sidecar cannot consume the whole cleanup budget inside this stage.
    let id = format!("close:{request_id}");
    let close_result = bounded
        .run(connection.command(
            id,
            ClaudeAgentSdkCommand::Close,
            json!({"joinBoundMs": CLOSE_JOIN_BOUND_MS}),
        ))
        .await;
    let reported = match close_result {
        Some(Ok(response)) if response.success => {
            SidecarCloseEvidence::from_sidecar(response.data.as_ref())
        }
        Some(Ok(response)) => {
            if cooperative_failure.is_none() {
                cooperative_failure = Some(
                    command_rejected(
                        "swallowtail.claude-agent.sdk.close_rejected",
                        "Claude Agent SDK sidecar rejected close",
                        response
                            .failure_code
                            .expect("a rejected response carries its fixed sidecar code"),
                    )
                    .diagnostic()
                    .clone(),
                );
            }
            None
        }
        Some(Err(_)) | None => None,
    };
    let _ = bounded.run(connection.begin_close()).await;
    (reported, cooperative_failure)
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

#[cfg(test)]
mod tests {
    use super::observed_root_exit;
    use swallowtail_runtime::ProcessTreeCompletion;

    #[test]
    fn retained_process_wait_evidence_prevents_false_root_unconfirmed() {
        assert_eq!(
            observed_root_exit(true, None, Some(ProcessTreeCompletion::RootOnly)),
            Some(ProcessTreeCompletion::RootOnly)
        );
        assert_eq!(
            observed_root_exit(
                true,
                Some(ProcessTreeCompletion::OwnedTreeEmpty),
                Some(ProcessTreeCompletion::RootOnly),
            ),
            Some(ProcessTreeCompletion::OwnedTreeEmpty)
        );
        assert_eq!(
            observed_root_exit(false, None, Some(ProcessTreeCompletion::RootOnly)),
            None
        );
    }
}
