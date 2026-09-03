//! Descendant-completion evidence for close, and its two separate vocabularies.
//!
//! The upstream SDK offers no joined stop: its cleanup races a bounded timer
//! inside a swallowed `catch`, discards the outcome, and its own escalation is
//! unreferenced and reaches only the direct child. This route therefore never
//! reads SDK cleanup as evidence.
//!
//! Two things are deliberately not the same claim. The sidecar can observe only
//! its own direct native child, so [`SidecarNativeJoin`] is all it may report.
//! What the whole owned tree did is the execution host's evidence, carried on
//! `ProcessExit::tree_completion`, and only `OwnedTreeEmpty` may support
//! `Clean`. Collapsing the two would let one observed child stand in for a
//! tree, which is exactly the Review Oracle counterexample.

use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{CleanupOutcome, ProcessTreeCompletion};

/// What the sidecar itself observed about its retained native child handle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SidecarNativeJoin {
    /// The native child's exit was observed inside the declared bound.
    Exited,
    /// The bound expired with the retained handle still showing a live child.
    /// This is a positive observation of a survivor, not an absence of news.
    Survivor,
}

impl SidecarNativeJoin {
    /// Parses the sidecar-reported join. The sidecar may only report what it
    /// observed of its own child; it can never report host escalation or speak
    /// for the owned tree.
    pub(crate) fn from_sidecar(value: &str) -> Option<Self> {
        match value {
            "exited" => Some(Self::Exited),
            "survivor" => Some(Self::Survivor),
            _ => None,
        }
    }
}

/// Exact close outcome for one session, decided from host evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ClaudeAgentSdkCloseState {
    /// The execution host attested that no member of its owned tree remains.
    OwnedTreeEmpty,
    /// The host attests root completion only. The sidecar root exited after the
    /// declared descendant termination attempt, and descendants stay
    /// unconfirmed because this platform cannot observe owned-tree emptiness.
    RootCompleted,
    /// The sidecar observed its native child still running.
    ObservedSurvivor,
    /// No root exit was observed at all.
    RootUnconfirmed,
}

impl ClaudeAgentSdkCloseState {
    /// Decides the close outcome from the sidecar's own observation and the
    /// host's owned-tree completion evidence.
    ///
    /// A survivor outranks a confirmed root exit: seeing a descendant alive is
    /// stronger evidence than the root's own exit is reassurance.
    pub(crate) fn decide(
        native_join: Option<SidecarNativeJoin>,
        root_exit: Option<ProcessTreeCompletion>,
    ) -> Self {
        if native_join == Some(SidecarNativeJoin::Survivor) {
            return Self::ObservedSurvivor;
        }
        match root_exit {
            Some(ProcessTreeCompletion::OwnedTreeEmpty) => Self::OwnedTreeEmpty,
            Some(ProcessTreeCompletion::RootOnly) => Self::RootCompleted,
            None => Self::RootUnconfirmed,
        }
    }

    /// Projects the outcome onto the contract cleanup result.
    ///
    /// Only attested owned-tree emptiness may be `Clean`. Root-only completion
    /// is the accepted route-qualified degraded posture. Anything weaker is
    /// cleanup failure, never a slow success.
    pub(crate) fn cleanup_outcome(self) -> CleanupOutcome {
        match self {
            Self::OwnedTreeEmpty => CleanupOutcome::Clean,
            Self::RootCompleted => CleanupOutcome::Degraded(SafeDiagnostic::new(
                "swallowtail.claude-agent.sdk.close_root_only_degraded",
                "Claude Agent SDK sidecar root exited after the declared descendant termination \
                 attempt, and this execution host cannot attest that its owned tree is empty",
            )),
            Self::ObservedSurvivor => CleanupOutcome::Failed(SafeDiagnostic::new(
                "swallowtail.claude-agent.sdk.close_descendant_survived",
                "Claude Agent SDK sidecar observed its native child still running at close",
            )),
            Self::RootUnconfirmed => CleanupOutcome::Failed(SafeDiagnostic::new(
                "swallowtail.claude-agent.sdk.close_root_unconfirmed",
                "Claude Agent SDK sidecar root exit was never observed",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ClaudeAgentSdkCloseState, SidecarNativeJoin};
    use swallowtail_runtime::{CleanupOutcome, ProcessTreeCompletion};

    #[test]
    fn only_attested_owned_tree_emptiness_supports_clean() {
        assert_eq!(
            ClaudeAgentSdkCloseState::decide(
                Some(SidecarNativeJoin::Exited),
                Some(ProcessTreeCompletion::OwnedTreeEmpty)
            )
            .cleanup_outcome(),
            CleanupOutcome::Clean
        );
        // Root-only evidence is the accepted degraded posture, never Clean.
        let root_only = ClaudeAgentSdkCloseState::decide(
            Some(SidecarNativeJoin::Exited),
            Some(ProcessTreeCompletion::RootOnly),
        )
        .cleanup_outcome();
        let CleanupOutcome::Degraded(diagnostic) = &root_only else {
            panic!("root-only completion is degraded, got {root_only:?}");
        };
        assert_eq!(
            diagnostic.code(),
            "swallowtail.claude-agent.sdk.close_root_only_degraded"
        );
    }

    #[test]
    fn an_observed_survivor_or_unconfirmed_root_is_failure() {
        for (join, root, code) in [
            (
                Some(SidecarNativeJoin::Survivor),
                Some(ProcessTreeCompletion::RootOnly),
                "swallowtail.claude-agent.sdk.close_descendant_survived",
            ),
            (
                // A survivor outranks even attested emptiness: the two cannot
                // both be true, and the stronger negative wins.
                Some(SidecarNativeJoin::Survivor),
                Some(ProcessTreeCompletion::OwnedTreeEmpty),
                "swallowtail.claude-agent.sdk.close_descendant_survived",
            ),
            (
                Some(SidecarNativeJoin::Exited),
                None,
                "swallowtail.claude-agent.sdk.close_root_unconfirmed",
            ),
            (
                None,
                None,
                "swallowtail.claude-agent.sdk.close_root_unconfirmed",
            ),
        ] {
            let outcome = ClaudeAgentSdkCloseState::decide(join, root).cleanup_outcome();
            let CleanupOutcome::Failed(diagnostic) = &outcome else {
                panic!("{join:?} with {root:?} must fail, got {outcome:?}");
            };
            assert_eq!(diagnostic.code(), code);
        }
    }

    #[test]
    fn the_sidecar_reports_only_what_it_observed_of_its_own_child() {
        assert_eq!(
            SidecarNativeJoin::from_sidecar("exited"),
            Some(SidecarNativeJoin::Exited)
        );
        assert_eq!(
            SidecarNativeJoin::from_sidecar("survivor"),
            Some(SidecarNativeJoin::Survivor)
        );
        // No sidecar vocabulary for escalation, tree emptiness, or cleanliness.
        for rejected in ["graceful", "escalated", "clean", "unconfirmed", ""] {
            assert!(SidecarNativeJoin::from_sidecar(rejected).is_none());
        }
    }
}
