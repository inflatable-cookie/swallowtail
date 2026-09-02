//! Descendant-tree close evidence and its two separate vocabularies.
//!
//! The upstream SDK offers no joined stop: its cleanup races a bounded timer
//! inside a swallowed `catch`, discards the outcome, and its own escalation is
//! unreferenced and reaches only the direct child. This route therefore never
//! reads SDK cleanup as evidence.
//!
//! Two things are deliberately not the same claim. The sidecar can observe
//! only its own direct native child, so [`SidecarNativeJoin`] is all it may
//! report. The route's close outcome is about the whole host-owned descendant
//! tree, so [`ClaudeAgentSdkCloseState`] is decided in Rust from host
//! evidence. Collapsing the two would let one observed child stand in for a
//! tree, which is exactly the Review Oracle counterexample.

use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::CleanupOutcome;

/// What the sidecar itself proved about its retained native child handle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SidecarNativeJoin {
    /// The native child's exit was observed inside the declared bound.
    Observed,
    /// The bound expired with no observed exit. Never evidence of exit.
    Unconfirmed,
}

impl SidecarNativeJoin {
    /// Parses the sidecar-reported join. The sidecar may only report a join it
    /// performed itself, so it can never report host escalation or a
    /// whole-tree result.
    pub(crate) fn from_sidecar(value: &str) -> Option<Self> {
        match value {
            "graceful" => Some(Self::Observed),
            "unconfirmed" => Some(Self::Unconfirmed),
            _ => None,
        }
    }
}

/// Why the host's termination authority was part of this close.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum EscalationCause {
    /// The sidecar observed its native child exit, but the execution host
    /// still terminated the tree it owns as part of cleanup, and does not
    /// attest that the tree was already empty beforehand.
    HostOwnedTreeCleanup,
    /// The sidecar could not prove its native child exited, so the host
    /// terminated the whole tree and the exit was observed only afterwards.
    HostTermination,
}

/// Explicit outcome of one descendant-tree close. These never collapse into a
/// single success value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ClaudeAgentSdkCloseState {
    /// Every process in the owned tree exited on its own, and the tree was
    /// attested empty without host termination.
    ///
    /// This requires whole-tree emptiness evidence. The current host process
    /// authority terminates the tree it owns during cleanup but does not
    /// report whether anything remained, so this route does not reach this
    /// state today and must not claim it from a single observed child. The
    /// state stays declared, and exercised in tests, so the vocabulary does
    /// not silently shrink to two values while the attestation seam is open.
    #[allow(dead_code)]
    Graceful,
    /// Exit was observed, and the host's termination authority was part of
    /// getting there.
    Escalated(EscalationCause),
    /// No exit was observed. A provider process may still be running; this is
    /// cleanup failure, never a slow success.
    Unconfirmed,
}

impl ClaudeAgentSdkCloseState {
    /// Returns the exact wire label for this close state.
    #[allow(dead_code)]
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Graceful => "graceful",
            Self::Escalated(_) => "escalated",
            Self::Unconfirmed => "unconfirmed",
        }
    }

    /// Projects the close state onto the contract cleanup outcome. Escalation
    /// is honest degradation; an unconfirmed exit is failure.
    pub(crate) fn cleanup_outcome(self) -> CleanupOutcome {
        match self {
            Self::Graceful => CleanupOutcome::Clean,
            Self::Escalated(EscalationCause::HostOwnedTreeCleanup) => {
                CleanupOutcome::Degraded(SafeDiagnostic::new(
                    "swallowtail.claude-agent.sdk.close_escalated_host_owned_tree_cleanup",
                    "Claude Agent SDK sidecar and native child exits were observed, but the host \
                     owned-tree termination that completed cleanup does not attest the tree was \
                     already empty",
                ))
            }
            Self::Escalated(EscalationCause::HostTermination) => {
                CleanupOutcome::Degraded(SafeDiagnostic::new(
                    "swallowtail.claude-agent.sdk.close_escalated_host_termination",
                    "Claude Agent SDK sidecar descendant tree exited only after host termination",
                ))
            }
            Self::Unconfirmed => CleanupOutcome::Failed(SafeDiagnostic::new(
                "swallowtail.claude-agent.sdk.close_unconfirmed",
                "Claude Agent SDK sidecar descendant-tree exit was never observed",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ClaudeAgentSdkCloseState, EscalationCause, SidecarNativeJoin};
    use swallowtail_runtime::CleanupOutcome;

    #[test]
    fn close_states_never_collapse_to_one_success() {
        assert_eq!(
            ClaudeAgentSdkCloseState::Graceful.cleanup_outcome(),
            CleanupOutcome::Clean
        );
        for cause in [
            EscalationCause::HostOwnedTreeCleanup,
            EscalationCause::HostTermination,
        ] {
            assert!(matches!(
                ClaudeAgentSdkCloseState::Escalated(cause).cleanup_outcome(),
                CleanupOutcome::Degraded(_)
            ));
        }
        assert!(matches!(
            ClaudeAgentSdkCloseState::Unconfirmed.cleanup_outcome(),
            CleanupOutcome::Failed(_)
        ));
    }

    #[test]
    fn each_escalation_cause_stays_separately_diagnosable() {
        let owned = ClaudeAgentSdkCloseState::Escalated(EscalationCause::HostOwnedTreeCleanup)
            .cleanup_outcome();
        let forced =
            ClaudeAgentSdkCloseState::Escalated(EscalationCause::HostTermination).cleanup_outcome();
        let (CleanupOutcome::Degraded(owned), CleanupOutcome::Degraded(forced)) = (owned, forced)
        else {
            panic!("escalated close is degraded");
        };
        assert_ne!(owned.code(), forced.code());
        assert_eq!(
            ClaudeAgentSdkCloseState::Escalated(EscalationCause::HostTermination).as_str(),
            "escalated"
        );
    }

    #[test]
    fn the_sidecar_reports_only_its_own_native_join() {
        assert_eq!(
            SidecarNativeJoin::from_sidecar("graceful"),
            Some(SidecarNativeJoin::Observed)
        );
        assert_eq!(
            SidecarNativeJoin::from_sidecar("unconfirmed"),
            Some(SidecarNativeJoin::Unconfirmed)
        );
        // A sidecar can neither escalate nor speak for the whole tree.
        for rejected in ["escalated", "", "clean", "timeout"] {
            assert!(SidecarNativeJoin::from_sidecar(rejected).is_none());
        }
    }
}
