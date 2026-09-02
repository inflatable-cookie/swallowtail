//! Descendant-tree close for one Claude Agent SDK sidecar session.
//!
//! Order: interrupt a live turn, end sidecar input through the explicit close
//! command, await the sidecar's own bounded join of its retained native
//! handle, escalate through the host's descendant-tree termination authority
//! on expiry, re-join, then report exactly one of `graceful`, `escalated`, or
//! `unconfirmed`. A discarded or unobserved wait is never evidence of exit.

use super::{CLOSE_JOIN_BOUND_MS, close_state};
use crate::sdk::close::ClaudeAgentSdkCloseState;
use crate::sdk::connection::SdkConnection;
use crate::sdk::wire::ClaudeAgentSdkCommand;
use serde_json::json;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    CleanupOutcome, CredentialLease, HostServices, JoinedTask, RequestId, ResourceLease,
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
    // The declared bound is stated to, and honoured by, the sidecar, which
    // always answers and then exits. A sidecar that neither answers nor exits
    // would hold this await open; bounding it in Rust needs a caller-supplied
    // deadline, because monotonic tick units are host-defined and a driver
    // must not invent one. Recorded as an exact residual rather than guessed.
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
        .and_then(|response| close_state(response.data.as_ref()));
    connection.begin_close().await;
    let escalated = if reported == Some(ClaudeAgentSdkCloseState::Graceful) {
        false
    } else {
        // The sidecar could not prove its native descendant exited, so host
        // authority terminates the whole tree rooted at the sidecar.
        connection.escalate().await.is_ok()
    };
    let joined = match pump_task {
        Some(task) => task.join().await.is_ok(),
        None => false,
    };
    // Re-join: only an observed sidecar exit closes the tree.
    let exit_observed = joined && connection.exit_observed() == Some(true);
    match (reported, escalated, exit_observed) {
        (Some(ClaudeAgentSdkCloseState::Graceful), _, true) => ClaudeAgentSdkCloseState::Graceful,
        (_, true, true) => ClaudeAgentSdkCloseState::Escalated,
        _ => ClaudeAgentSdkCloseState::Unconfirmed,
    }
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
