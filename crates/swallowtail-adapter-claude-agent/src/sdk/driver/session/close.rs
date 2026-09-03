//! Descendant-tree close for one Claude Agent SDK sidecar session.
//!
//! Close owns nothing itself. It hands the session's whole remaining set — the
//! connection, the sidecar process, the pump, any live turn's host-deadline
//! task, the working-resource lease, and the credential lease — to one
//! enclosing guardian task, started under the reap reservation this session
//! pre-admitted at open. That guardian runs the single ordered continuation:
//! interrupt a live turn, end sidecar input through the explicit close command,
//! await the sidecar's own bounded join of its retained native handle, make the
//! declared descendant termination attempt through host authority, observe the
//! root, join the pump, release the resource, then release the credential.
//!
//! The caller waits for that continuation inside its own cleanup deadline. On
//! expiry it transfers the guardian, not the pump, and reports unconfirmed
//! cleanup. No lease is released around work that is still live.
//!
//! The sidecar's join covers its direct native child only; owned-tree
//! completion is the host's evidence, and only `OwnedTreeEmpty` may support
//! `Clean`. On a root-only platform a confirmed root exit after the declared
//! termination attempt is the accepted route-qualified degraded outcome, and
//! an observed survivor or unconfirmed root exit is failure.

use crate::sdk::bounded::HostBound;
use crate::sdk::close::ClaudeAgentSdkCloseState;
use crate::sdk::guardian::{CleanupReport, Owned, SessionGuardian};
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{CleanupOutcome, Deadline};

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
        active.turn.fail_connection(SafeDiagnostic::new(
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
    let mut owned = Owned {
        connection: Some(self_.connection.clone()),
        process: Some(self_.connection.process()),
        pump: self_.pump_task.take(),
        scoped: Vec::new(),
        resource: self_.resource.take(),
        credential: self_.credential.take(),
    };
    if let Some(mut active) = active
        && let Some(task) = active.deadline_task.take()
    {
        owned.scoped.push(task);
    }
    let Some(reservation) = self_.close_reservation.take() else {
        // Unreachable after open: the session cannot exist without its
        // pre-admitted reservation. Reporting unconfirmed cleanup is the
        // fail-closed reading, and nothing is released around live work.
        return guardian_unavailable();
    };
    let guardian = SessionGuardian::arm(
        &self_.services,
        reservation,
        self_.close_scope.clone(),
        self_.request_id.as_str(),
        deadline,
        owned,
        turn_active,
    );
    let Ok(guardian) = guardian else {
        return guardian_unavailable();
    };
    match guardian.settle(&bounded, &self_.services).await {
        Some(report) => decide(&report),
        // The guardian was accepted for reaping and still owns the process,
        // the pump, and both leases. Ownership transfer is not cleanup.
        None => CleanupOutcome::Failed(SafeDiagnostic::new(
            "swallowtail.claude-agent.sdk.close_cleanup_unconfirmed",
            "Claude Agent SDK sidecar cleanup did not complete inside the caller cleanup deadline",
        )),
    }
}

/// Projects one finished ordered continuation onto the contract outcome.
///
/// Only an actual pump join makes the recorded root exit readable, so a
/// continuation that could not join its own scoped work still reports an
/// unconfirmed root.
fn decide(report: &CleanupReport) -> CleanupOutcome {
    let root_exit = if report.pump_joined {
        report.root_exit
    } else {
        None
    };
    let state = ClaudeAgentSdkCloseState::decide(report.native_join, root_exit);
    merge_cleanup(
        merge_cleanup(state.cleanup_outcome(), report.resource.clone()),
        report.credential.clone(),
    )
}

fn guardian_unavailable() -> CleanupOutcome {
    CleanupOutcome::Failed(SafeDiagnostic::new(
        "swallowtail.claude-agent.sdk.close_guardian_unavailable",
        "Claude Agent SDK sidecar could not start its enclosing cleanup guardian",
    ))
}

fn merge_cleanup(left: CleanupOutcome, right: CleanupOutcome) -> CleanupOutcome {
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
