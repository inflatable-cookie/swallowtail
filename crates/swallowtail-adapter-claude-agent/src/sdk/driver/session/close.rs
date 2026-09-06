//! Descendant-tree close for one Claude Agent SDK sidecar session.
//!
//! Close owns nothing itself and starts nothing fallible. The enclosing
//! guardian task was started before this session took any effect, so closing is
//! two infallible steps: resolve any live turn, then hand the guardian the
//! connection, the sidecar process, the pump, any remaining turn-deadline task,
//! and both leases at once.
//!
//! That handover happens **before** the public cleanup future exists. The
//! runtime may refuse an already-elapsed deadline, a missing time service, or
//! the wrong host without ever polling that future, and a caller may drop it
//! after one pending poll; in both cases the guardian already owns the whole
//! continuation and is transferred to its owning host by its own drop rather
//! than joined on the dropping thread.
//!
//! The guardian then runs the single ordered continuation: interrupt a live
//! turn, end sidecar input through the explicit close command, await the
//! sidecar's own bounded join of its retained native handle, make the declared
//! descendant termination attempt through host authority, observe the root,
//! join the pump, release the resource, then release the credential.
//!
//! The sidecar's join covers its direct native child only; owned-tree
//! completion is the host's evidence, and only `OwnedTreeEmpty` may support
//! `Clean`. On a root-only platform a confirmed root exit after the declared
//! termination attempt is the accepted route-qualified degraded outcome, and
//! an observed survivor or unconfirmed root exit is failure.

use crate::sdk::bounded::HostBound;
use crate::sdk::close::ClaudeAgentSdkCloseState;
use crate::sdk::guardian::{CleanupReport, Cooperative, Owned, SessionGuardian};
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{CleanupOutcome, Deadline, HostServices};

/// Takes everything the session still owns and hands it to the guardian.
///
/// Synchronous and infallible: the guardian task already exists, so this cannot
/// fail after the session holds live state. Returns the guardian so the caller
/// can wait for its ordered continuation; dropping the returned guardian is a
/// host handoff, not a join.
pub(super) fn activate(
    session: &mut super::ClaudeAgentSdkSessionHandle,
    deadline: Deadline,
    cooperative_close: bool,
) -> Option<SessionGuardian> {
    let guardian = session.close_guardian.take()?;
    let active = session
        .active
        .lock()
        .expect("SDK sidecar active lock poisoned")
        .take();
    // Closing the session ends any live turn first. The turn must
    // reach its terminal outcome before its host-deadline task can be
    // joined: that task waits on completion or expiry, so joining it
    // while the turn is still open would wait for an event that close
    // itself just prevented.
    if session.cancellation.was_requested()
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
    let mut owned = Owned {
        connection: Some(session.connection.clone()),
        process: Some(session.connection.process()),
        pump: session.pump_task.take(),
        scoped: Vec::new(),
        resource: session.resource.take(),
        credential: session.credential.take(),
    };
    if let Some(mut active) = active
        && let Some(task) = active.deadline_task.take()
    {
        owned.scoped.push(task);
    }
    let cooperative = if cooperative_close {
        Cooperative::Session { turn_active }
    } else {
        // A session dropped without close carries no caller deadline, so there
        // is no bound to spend on cooperative stages. The guardian goes
        // straight to the host termination request and the ordered release.
        Cooperative::None
    };
    guardian.activate(owned, cooperative, deadline);
    Some(guardian)
}

/// Waits for the activated guardian inside the caller's one cleanup deadline.
pub(super) async fn settle(
    guardian: Option<SessionGuardian>,
    services: &HostServices,
    deadline: Deadline,
) -> CleanupOutcome {
    let Some(guardian) = guardian else {
        // Unreachable after open: the session cannot exist without its
        // pre-started guardian, and close consumes it exactly once.
        return guardian_unavailable();
    };
    let bounded = HostBound::new(
        services
            .time()
            .cloned()
            .expect("validated sidecar time service"),
        deadline,
    );
    match guardian.settle(&bounded).await {
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
    let cooperative = report
        .cooperative_failure
        .clone()
        .map_or(CleanupOutcome::NotApplicable, CleanupOutcome::Failed);
    merge_cleanup(
        merge_cleanup(
            merge_cleanup(state.cleanup_outcome(), cooperative),
            report.resource.clone(),
        ),
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
