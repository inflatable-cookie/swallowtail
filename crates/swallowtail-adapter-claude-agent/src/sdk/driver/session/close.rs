//! Descendant-tree close for one Claude Agent SDK sidecar session.
//!
//! Order: interrupt a live turn, end sidecar input through the explicit close
//! command, await the sidecar's own bounded join of its retained native
//! handle, escalate through the host's descendant-tree termination authority,
//! re-join, then report exactly one of `graceful`, `escalated`, or
//! `unconfirmed`. A discarded or unobserved wait is never evidence of exit.
//!
//! The sidecar's join covers its direct native child only. The host owns the
//! tree and terminates it during cleanup, but does not report whether the
//! tree was already empty, so an observed exit is reported as escalated
//! rather than graceful. Claiming graceful from one observed child is the
//! Review Oracle counterexample, and this route will not do it.

use super::{CLOSE_JOIN_BOUND_MS, native_join};
use crate::sdk::close::{ClaudeAgentSdkCloseState, EscalationCause, SidecarNativeJoin};
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
    // BLOCKED, reported rather than papered over. Two awaits below are not
    // bounded by anything this driver can observe: the correlated close
    // response, and the pump join that follows escalation. `close` carries no
    // caller deadline on the shared session seam, and monotonic tick units are
    // host-defined, so no fresh host-observed bound can be derived here.
    // The only stated bounds today are the sidecar's own declared join bound
    // and the execution host's internal termination bounds, neither of which
    // this driver can observe or report. Contract 019's bounded-join clause is
    // therefore not satisfied by this path, and card 055 is not complete on it.
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
    let escalated = if reported == Some(SidecarNativeJoin::Observed) {
        // Host cleanup still terminates the tree it owns; this close did not
        // have to force a surviving child.
        EscalationCause::HostOwnedTreeCleanup
    } else {
        // The sidecar could not prove its native descendant exited, so host
        // authority terminates the whole tree rooted at the sidecar.
        let _ = connection.escalate().await;
        EscalationCause::HostTermination
    };
    let joined = match pump_task {
        Some(task) => task.join().await.is_ok(),
        None => false,
    };
    // Re-join. An observed root exit is not proof that every descendant
    // exited: only a host process API that attests the owned tree is empty
    // could establish that, and this one does not expose it. So the observed
    // exit downgrades to escalated rather than upgrading to graceful.
    let exit_observed = joined && connection.exit_observed() == Some(true);
    if exit_observed {
        ClaudeAgentSdkCloseState::Escalated(escalated)
    } else {
        ClaudeAgentSdkCloseState::Unconfirmed
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
