//! Descendant-tree close for one Claude Agent SDK sidecar session.
//!
//! Order, all inside the caller's single cleanup deadline: interrupt a live
//! turn, end sidecar input through the explicit close command, await the
//! sidecar's own bounded join of its retained native handle, make the declared
//! descendant termination attempt through host authority, re-join the root,
//! then decide the outcome from host evidence.
//!
//! The sidecar's join covers its direct native child only; owned-tree
//! completion is the host's evidence, and only `OwnedTreeEmpty` may support
//! `Clean`. On a root-only platform a confirmed root exit after the declared
//! termination attempt is the accepted route-qualified degraded outcome, and
//! an observed survivor or unconfirmed root exit is failure.

use super::{CLOSE_JOIN_BOUND_MS, native_join};
use crate::sdk::close::ClaudeAgentSdkCloseState;
use crate::sdk::connection::SdkConnection;
use crate::sdk::wire::ClaudeAgentSdkCommand;
use serde_json::json;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    CleanupOutcome, CredentialLease, HostServices, JoinedTask, ProcessExit, RequestId,
    ResourceLease,
};

pub(super) async fn close_tree(
    connection: &SdkConnection,
    request_id: &RequestId,
    turn_active: bool,
    pump_task: Option<Box<dyn JoinedTask>>,
) -> ClaudeAgentSdkCloseState {
    if turn_active {
        let id = format!("close-interrupt:{}", request_id.as_str());
        let _ = connection
            .command(id, ClaudeAgentSdkCommand::Interrupt, json!({}))
            .await;
    }
    // Every await below sits inside the caller's cleanup deadline, which the
    // public close seam applies once and no stage may restart.
    let id = format!("close:{}", request_id.as_str());
    let reported = connection
        .command(
            id,
            ClaudeAgentSdkCommand::Close,
            json!({"joinBoundMs": CLOSE_JOIN_BOUND_MS}),
        )
        .await
        .ok()
        .filter(|response| response.success)
        .and_then(|response| native_join(response.data.as_ref()));
    connection.begin_close().await;
    // The declared descendant termination attempt always runs, whatever the
    // sidecar reported. Degraded truth is only admissible after it.
    let _ = connection.escalate().await;
    let joined = match pump_task {
        Some(task) => task.join().await.is_ok(),
        None => false,
    };
    // Re-join, then let the host's own evidence decide. Root exit is not tree
    // completion, so `Clean` needs `OwnedTreeEmpty` from the host.
    let root_exit = if joined {
        connection.observed_exit().map(ProcessExit::tree_completion)
    } else {
        None
    };
    ClaudeAgentSdkCloseState::decide(reported, root_exit)
}

pub(super) async fn release_resource(
    lease: Option<ResourceLease>,
    services: &HostServices,
) -> CleanupOutcome {
    match (lease, services.working_resource()) {
        (Some(lease), Some(service)) => service.release(lease).await,
        (Some(_), None) => cleanup_failure(
            "swallowtail.claude-agent.sdk.resource_release_failed",
            "Claude Agent SDK sidecar working-resource service disappeared during cleanup",
        ),
        (None, _) => CleanupOutcome::NotApplicable,
    }
}

pub(super) async fn release_credential(
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

pub(crate) fn merge_cleanup(left: CleanupOutcome, right: CleanupOutcome) -> CleanupOutcome {
    match (left, right) {
        (CleanupOutcome::Failed(error), _) | (_, CleanupOutcome::Failed(error)) => {
            CleanupOutcome::Failed(error)
        }
        (CleanupOutcome::Degraded(error), _) | (_, CleanupOutcome::Degraded(error)) => {
            CleanupOutcome::Degraded(error)
        }
        (CleanupOutcome::Clean, CleanupOutcome::Clean | CleanupOutcome::NotApplicable)
        | (CleanupOutcome::NotApplicable, CleanupOutcome::Clean) => CleanupOutcome::Clean,
        (CleanupOutcome::NotApplicable, CleanupOutcome::NotApplicable) => {
            CleanupOutcome::NotApplicable
        }
    }
}

fn cleanup_failure(code: &'static str, message: &'static str) -> CleanupOutcome {
    CleanupOutcome::Failed(SafeDiagnostic::new(code, message))
}
