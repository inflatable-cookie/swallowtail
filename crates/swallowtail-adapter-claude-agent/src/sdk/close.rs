//! Three-valued descendant-tree close outcome.
//!
//! The upstream SDK offers no joined stop: its cleanup races a bounded timer
//! inside a swallowed `catch` and discards the outcome, and its own
//! SIGTERM/SIGKILL escalation is unref'd and reaches only the direct child.
//! This route therefore never reads SDK cleanup as evidence. The sidecar
//! joins its own retained native handle, and the host owns termination of the
//! whole descendant tree rooted at the sidecar.

use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::CleanupOutcome;

/// Explicit outcome of one descendant-tree close. These never collapse into
/// a single success value. Consumers observe the projection on
/// `CleanupOutcome`; the state itself stays adapter-internal so the public
/// surface adds no new shared vocabulary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ClaudeAgentSdkCloseState {
    /// Every provider process exited on its own within the declared bound,
    /// observed rather than assumed.
    Graceful,
    /// Exit was observed, but only after host descendant-tree termination.
    Escalated,
    /// No exit was observed. A provider process may still be running; this is
    /// cleanup failure, never a slow success.
    Unconfirmed,
}

impl ClaudeAgentSdkCloseState {
    /// Returns the exact wire label for this close state.
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Graceful => "graceful",
            Self::Escalated => "escalated",
            Self::Unconfirmed => "unconfirmed",
        }
    }

    /// Parses the sidecar-reported close state. The sidecar may only report a
    /// join it performed itself, so it never reports `escalated`: escalation
    /// is host authority and is decided in Rust.
    pub(crate) fn from_sidecar(value: &str) -> Option<Self> {
        match value {
            "graceful" => Some(Self::Graceful),
            "unconfirmed" => Some(Self::Unconfirmed),
            _ => None,
        }
    }

    /// Projects the close state onto the contract cleanup outcome. Escalation
    /// is honest degradation; an unconfirmed exit is failure.
    pub(crate) fn cleanup_outcome(self) -> CleanupOutcome {
        match self {
            Self::Graceful => CleanupOutcome::Clean,
            Self::Escalated => CleanupOutcome::Degraded(SafeDiagnostic::new(
                "swallowtail.claude-agent.sdk.close_escalated",
                format!(
                    "Claude Agent SDK sidecar descendant tree exited only after host escalation ({})",
                    self.as_str()
                ),
            )),
            Self::Unconfirmed => CleanupOutcome::Failed(SafeDiagnostic::new(
                "swallowtail.claude-agent.sdk.close_unconfirmed",
                format!(
                    "Claude Agent SDK sidecar descendant-tree exit was never observed ({})",
                    self.as_str()
                ),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ClaudeAgentSdkCloseState;
    use swallowtail_runtime::CleanupOutcome;

    #[test]
    fn close_states_never_collapse_to_one_success() {
        assert_eq!(
            ClaudeAgentSdkCloseState::Graceful.cleanup_outcome(),
            CleanupOutcome::Clean
        );
        assert!(matches!(
            ClaudeAgentSdkCloseState::Escalated.cleanup_outcome(),
            CleanupOutcome::Degraded(_)
        ));
        assert!(matches!(
            ClaudeAgentSdkCloseState::Unconfirmed.cleanup_outcome(),
            CleanupOutcome::Failed(_)
        ));
    }

    #[test]
    fn the_sidecar_can_never_report_host_escalation() {
        assert_eq!(
            ClaudeAgentSdkCloseState::from_sidecar("graceful"),
            Some(ClaudeAgentSdkCloseState::Graceful)
        );
        assert_eq!(
            ClaudeAgentSdkCloseState::from_sidecar("unconfirmed"),
            Some(ClaudeAgentSdkCloseState::Unconfirmed)
        );
        for rejected in ["escalated", "", "clean", "timeout"] {
            assert!(ClaudeAgentSdkCloseState::from_sidecar(rejected).is_none());
        }
        assert_eq!(ClaudeAgentSdkCloseState::Escalated.as_str(), "escalated");
    }
}
