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
use crate::sdk::guardian::{EscalationWatchdog, TaskOwner, bounded_join};
use crate::sdk::wire::ClaudeAgentSdkCommand;
use serde_json::json;
use std::sync::Arc;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    CleanupOutcome, CredentialLease, Deadline, HostServices, JoinedTask, ProcessExit, RequestId,
    ResourceLease,
};

/// The exact session identity close acts on: its transport, its host services,
/// and the host and scope that own its scoped work.
pub(super) struct CloseTarget<'a> {
    pub(super) connection: &'a Arc<SdkConnection>,
    pub(super) services: &'a HostServices,
    pub(super) execution_host_id: &'a swallowtail_core::ExecutionHostId,
    pub(super) request_id: &'a RequestId,
    pub(super) session_scope: &'a swallowtail_runtime::ScopeId,
}

/// The full ordered close for one session, inside the caller's one deadline.
pub(super) async fn close_session(
    session: &mut super::ClaudeAgentSdkSessionHandle,
    deadline: Deadline,
) -> CleanupOutcome {
    let self_ = session;
    let active = self_
        .active
        .lock()
        .expect("SDK sidecar active lock poisoned")
        .take();
    // Closing the session ends any live turn first. The turn must
    // reach its terminal outcome before its host-deadline task can be
    // joined: that task waits on completion or expiry, so joining it
    // while the turn is still open would wait for an event that close
    // itself just prevented.
    if self_.cancellation.was_requested()
        && let Some(active) = &active
    {
        // The consumer already asked the session to cancel; close
        // is where that request actually reaches the tree.
        active.turn.mark_cancelled();
    }
    if let Some(active) = &active
        && !active.turn.is_finished()
    {
        active.turn.mark_cancelled();
        active
            .turn
            .fail_connection(swallowtail_core::SafeDiagnostic::new(
                "swallowtail.claude-agent.sdk.session_closing",
                "Claude Agent SDK sidecar session closed while a turn was active",
            ));
    }
    let turn_active = active.is_some();
    let bounded = HostBound::new(
        self_
            .services
            .time()
            .cloned()
            .expect("validated sidecar time service"),
        deadline,
    );
    if let Some(mut active) = active
        && let Some(task) = active.deadline_task.take()
    {
        let owner = crate::sdk::guardian::TaskOwner::new(
            &self_.services,
            &self_.execution_host_id,
            &active.deadline_scope,
        );
        let _ = bounded_join(&bounded, &owner, task).await;
    }
    let target = CloseTarget {
        connection: &self_.connection,
        services: &self_.services,
        execution_host_id: &self_.execution_host_id,
        request_id: &self_.request_id,
        session_scope: &self_.session_scope,
    };
    let state = close_tree(
        &target,
        turn_active,
        self_.pump_task.take(),
        &bounded,
        deadline,
    )
    .await;
    let resource = release_resource(self_.resource.take(), &self_.services, &bounded).await;
    let credential = release_credential(self_.credential.take(), &self_.services, &bounded).await;
    merge_cleanup(merge_cleanup(state.cleanup_outcome(), resource), credential)
}

pub(super) async fn close_tree(
    target: &CloseTarget<'_>,
    turn_active: bool,
    pump_task: Option<Box<dyn JoinedTask>>,
    bounded: &HostBound,
    deadline: Deadline,
) -> ClaudeAgentSdkCloseState {
    let CloseTarget {
        connection,
        services,
        execution_host_id,
        request_id,
        session_scope,
    } = target;
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
            let _ = watchdog.terminate(bounded, services).await;
        }
        None => {
            let _ = bounded.run(connection.escalate()).await;
        }
    }

    // Only an actual join is join evidence. A pump handed to the host for
    // reaping at the deadline is ownership transfer, not completion, so it
    // still yields an unconfirmed root below.
    let joined = match pump_task {
        Some(task) => {
            let owner = TaskOwner::new(services, execution_host_id, session_scope);
            bounded_join(bounded, &owner, task).await.joined()
        }
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
