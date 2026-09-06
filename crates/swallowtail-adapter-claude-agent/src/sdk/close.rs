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

use serde_json::Value;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{CleanupOutcome, ProcessTreeCompletion};

const MAXIMUM_CLOSE_TIMELINE_LABELS: usize = 16;
const MAXIMUM_CLOSE_LABEL_BYTES: usize = 64;
pub(crate) const CLOSE_JOIN_BOUND_MS: u64 = 2_000;
const CLOSE_TIMELINE_LABELS: &[&str] = &[
    "close_requested",
    "interrupt_requested",
    "interrupt_completed",
    "interrupt_failed",
    "session_input_closed",
    "sdk_transport_close_ran",
    "sdk_transport_close_failed",
    "native_join_exited",
    "native_join_survivor",
];

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

/// Sanitized evidence from the sidecar's bounded close response.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SidecarCloseEvidence {
    pub(crate) native_join: SidecarNativeJoin,
    pub(crate) native_exit_observed: bool,
    pub(crate) native_exit_event: Option<String>,
    pub(crate) native_exit_code: Option<i64>,
    pub(crate) native_exit_signal: Option<String>,
    pub(crate) sdk_transport_close_ran: bool,
    pub(crate) close_timeline: Vec<String>,
}

impl SidecarCloseEvidence {
    pub(crate) fn from_sidecar(data: Option<&Value>) -> Option<Self> {
        let data = data?;
        if data.get("joinBoundMs").and_then(Value::as_u64) != Some(CLOSE_JOIN_BOUND_MS) {
            return None;
        }
        let native_exit_observed = data.get("nativeExitObserved").and_then(Value::as_bool)?;
        let native_join = SidecarNativeJoin::from_sidecar(data.get("nativeJoin")?.as_str()?)?;
        let native_exit_event = optional_label(data, "nativeExitEvent")?;
        if native_exit_event
            .as_deref()
            .is_some_and(|event| !matches!(event, "exit" | "error"))
        {
            return None;
        }
        let native_exit_code = optional_integer(data, "nativeExitCode")?;
        let native_exit_signal = optional_label(data, "nativeExitSignal")?;
        let sdk_transport_close_ran = data.get("sdkTransportCloseRan").and_then(Value::as_bool)?;
        let close_timeline = data
            .get("closeTimeline")
            .and_then(Value::as_array)
            .filter(|labels| labels.len() <= MAXIMUM_CLOSE_TIMELINE_LABELS)?
            .iter()
            .map(|label| {
                let label = label.as_str()?;
                CLOSE_TIMELINE_LABELS
                    .contains(&label)
                    .then(|| label.to_owned())
            })
            .collect::<Option<Vec<_>>>()?;
        match (native_join, native_exit_observed) {
            (SidecarNativeJoin::Exited, true) | (SidecarNativeJoin::Survivor, false) => {}
            _ => return None,
        }
        Some(Self {
            native_join,
            native_exit_observed,
            native_exit_event,
            native_exit_code,
            native_exit_signal,
            sdk_transport_close_ran,
            close_timeline,
        })
    }

    pub(crate) fn diagnostic_fragment(&self) -> String {
        let timeline = self.close_timeline.join(",");
        format!(
            "close_evidence: nativeExitObserved={}; nativeExitEvent={}; nativeExitCode={}; nativeExitSignal={}; sdkTransportCloseRan={}; closeTimeline=[{timeline}]",
            self.native_exit_observed,
            optional_text(self.native_exit_event.as_deref()),
            optional_integer_text(self.native_exit_code),
            optional_text(self.native_exit_signal.as_deref()),
            self.sdk_transport_close_ran,
        )
    }
}

fn optional_label(data: &Value, field: &str) -> Option<Option<String>> {
    let value = data.get(field)?;
    if value.is_null() {
        return Some(None);
    }
    let text = value.as_str()?;
    if text.is_empty()
        || text.len() > MAXIMUM_CLOSE_LABEL_BYTES
        || !text.chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '_' | '-' | '.')
        })
    {
        return None;
    }
    Some(Some(text.to_owned()))
}

fn optional_integer(data: &Value, field: &str) -> Option<Option<i64>> {
    let value = data.get(field)?;
    if value.is_null() {
        Some(None)
    } else {
        value.as_i64().map(Some)
    }
}

fn optional_text(value: Option<&str>) -> &str {
    value.unwrap_or("<null>")
}

fn optional_integer_text(value: Option<i64>) -> String {
    value.map_or_else(|| "<null>".to_owned(), |value| value.to_string())
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
    use super::{
        CLOSE_JOIN_BOUND_MS, ClaudeAgentSdkCloseState, SidecarCloseEvidence, SidecarNativeJoin,
    };
    use serde_json::json;
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

    #[test]
    fn close_evidence_is_bounded_and_projects_into_a_safe_fragment() {
        let evidence = SidecarCloseEvidence::from_sidecar(Some(&json!({
            "nativeJoin": "exited",
            "joinBoundMs": CLOSE_JOIN_BOUND_MS,
            "nativeExitObserved": true,
            "nativeExitEvent": "exit",
            "nativeExitCode": 1,
            "nativeExitSignal": null,
            "sdkTransportCloseRan": true,
            "closeTimeline": [
                "close_requested",
                "session_input_closed",
                "sdk_transport_close_ran",
                "native_join_exited"
            ]
        })))
        .expect("valid close evidence decodes");
        assert!(
            evidence
                .diagnostic_fragment()
                .contains("nativeExitObserved=true")
        );
        assert!(evidence.diagnostic_fragment().contains("nativeExitCode=1"));
        assert!(evidence
            .diagnostic_fragment()
            .contains("closeTimeline=[close_requested,session_input_closed,sdk_transport_close_ran,native_join_exited]"));

        for invalid in [
            json!({
                "nativeJoin": "exited", "joinBoundMs": CLOSE_JOIN_BOUND_MS,
                "nativeExitObserved": true, "nativeExitEvent": "provider error",
                "nativeExitCode": 1, "nativeExitSignal": null,
                "sdkTransportCloseRan": true, "closeTimeline": []
            }),
            json!({
                "nativeJoin": "exited", "joinBoundMs": CLOSE_JOIN_BOUND_MS,
                "nativeExitObserved": true, "nativeExitEvent": "exit",
                "nativeExitCode": 1, "nativeExitSignal": null,
                "sdkTransportCloseRan": true, "closeTimeline": ["provider_error"]
            }),
        ] {
            assert!(SidecarCloseEvidence::from_sidecar(Some(&invalid)).is_none());
        }
    }
}
