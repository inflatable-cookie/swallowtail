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
use crate::sdk::bounded::HostBound;
use crate::sdk::close::ClaudeAgentSdkCloseState;
use crate::sdk::connection::SdkConnection;
use crate::sdk::guardian::{EscalationWatchdog, bounded_join};
use crate::sdk::wire::ClaudeAgentSdkCommand;
use serde_json::json;
use std::sync::Arc;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    CleanupOutcome, CredentialLease, Deadline, HostServices, JoinedTask, ProcessExit, RequestId,
    ResourceLease,
};

pub(super) async fn close_tree(
    connection: &Arc<SdkConnection>,
    services: &HostServices,
    request_id: &RequestId,
    turn_active: bool,
    pump_task: Option<Box<dyn JoinedTask>>,
    bounded: &HostBound,
    deadline: Deadline,
) -> ClaudeAgentSdkCloseState {
    // Armed before any cooperative stage. From here the declared descendant
    // termination request happens on the caller's deadline even if the sidecar
    // accepts input and never answers, because the watchdog is a host task, not
    // part of this future.
    let watchdog = EscalationWatchdog::arm(
        services,
        Arc::clone(connection),
        request_id.as_str(),
        deadline,
    )
    .ok();

    if turn_active {
        let id = format!("close-interrupt:{}", request_id.as_str());
        let _ = bounded
            .run(connection.command(id, ClaudeAgentSdkCommand::Interrupt, json!({})))
            .await;
    }
    // The sidecar's own bounded native join. Raced here as well, so a silent
    // sidecar cannot consume the caller's deadline inside this stage.
    let id = format!("close:{}", request_id.as_str());
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

    // Ask for the termination now rather than at expiry, then join the guard.
    // Either way the request is made: the guard owns it.
    match watchdog {
        Some(watchdog) => {
            let _ = watchdog.terminate(bounded).await;
        }
        None => {
            let _ = bounded.run(connection.escalate()).await;
        }
    }

    let joined = match pump_task {
        Some(task) => bounded_join(bounded, task).await,
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
    bounded: &HostBound,
) -> CleanupOutcome {
    match (lease, services.working_resource()) {
        (Some(lease), Some(service)) => {
            bounded
                .run(service.release(lease))
                .await
                .unwrap_or_else(|| {
                    cleanup_failure(
                        "swallowtail.claude-agent.sdk.resource_release_unconfirmed",
                        "Claude Agent SDK sidecar working-resource release did not complete inside \
                     the caller cleanup deadline",
                    )
                })
        }
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
    bounded: &HostBound,
) -> CleanupOutcome {
    match (lease, services.credential()) {
        (Some(lease), Some(service)) => {
            bounded
                .run(service.release(lease))
                .await
                .unwrap_or_else(|| {
                    cleanup_failure(
                        "swallowtail.claude-agent.sdk.credential_release_unconfirmed",
                        "Claude Agent SDK sidecar credential release did not complete inside the \
                     caller cleanup deadline",
                    )
                })
        }
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
