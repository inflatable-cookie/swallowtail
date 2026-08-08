#![deny(missing_docs)]

use crate::{RequestId, RuntimeRunId, RuntimeSessionId, RuntimeTurnId, ScopeId};
use std::fmt;

/// Maximum Unicode scalar values retained in one debug detail body.
pub const MAX_DEBUG_DETAIL_CHARS: usize = 4096;

/// Marker appended when debug detail is truncated to the runtime bound.
pub const DEBUG_DETAIL_TRUNCATED_SUFFIX: &str = " [debug detail truncated]";

const MAX_DEBUG_LABEL_CHARS: usize = 128;

/// Kind of restricted debug observation emitted to a host observer.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DebugObservationKind {
    /// Preparation or ordinary lifecycle stage boundary.
    Lifecycle,
    /// Interface-version or qualification evidence.
    InterfaceVersion,
    /// Host process start, exit, or supervision boundary.
    HostProcess,
    /// Bounded inbound wire context.
    WireInbound,
    /// Bounded outbound wire context.
    WireOutbound,
    /// Protocol parse or map failure context.
    ProtocolParse,
    /// Evidence used for portable failure classification.
    ClassificationEvidence,
    /// Bounded stderr-ring snapshot.
    StderrRing,
    /// Cleanup boundary context.
    Cleanup,
}

/// Structured restricted debug observation for an opt-in host sink.
#[derive(Clone, Eq, PartialEq)]
pub struct DebugObservation {
    request_id: Option<RequestId>,
    scope_id: Option<ScopeId>,
    run_id: Option<RuntimeRunId>,
    turn_id: Option<RuntimeTurnId>,
    session_id: Option<RuntimeSessionId>,
    route: Option<String>,
    kind: DebugObservationKind,
    stage: Option<String>,
    correlated_code: Option<&'static str>,
    detail: String,
    detail_truncated: bool,
}

impl DebugObservation {
    /// Creates one observation with a bounded restricted detail body.
    #[must_use]
    pub fn new(kind: DebugObservationKind, detail: impl Into<String>) -> Self {
        let (detail, detail_truncated) = bound_detail(detail.into());
        Self {
            request_id: None,
            scope_id: None,
            run_id: None,
            turn_id: None,
            session_id: None,
            route: None,
            kind,
            stage: None,
            correlated_code: None,
            detail,
            detail_truncated,
        }
    }

    /// Correlates the observation to one request id.
    #[must_use]
    pub fn with_request_id(mut self, request_id: RequestId) -> Self {
        self.request_id = Some(request_id);
        self
    }

    /// Correlates the observation to one scope id.
    #[must_use]
    pub fn with_scope_id(mut self, scope_id: ScopeId) -> Self {
        self.scope_id = Some(scope_id);
        self
    }

    /// Correlates the observation to one runtime run id.
    #[must_use]
    pub fn with_run_id(mut self, run_id: RuntimeRunId) -> Self {
        self.run_id = Some(run_id);
        self
    }

    /// Correlates the observation to one runtime turn id.
    #[must_use]
    pub fn with_turn_id(mut self, turn_id: RuntimeTurnId) -> Self {
        self.turn_id = Some(turn_id);
        self
    }

    /// Correlates the observation to one runtime session id.
    #[must_use]
    pub fn with_session_id(mut self, session_id: RuntimeSessionId) -> Self {
        self.session_id = Some(session_id);
        self
    }

    /// Labels the emitting route or adapter surface.
    #[must_use]
    pub fn with_route(mut self, route: impl Into<String>) -> Self {
        self.route = bound_label(route.into());
        self
    }

    /// Labels the stage or boundary that produced the observation.
    #[must_use]
    pub fn with_stage(mut self, stage: impl Into<String>) -> Self {
        self.stage = bound_label(stage.into());
        self
    }

    /// Correlates the observation to an exact safe diagnostic code.
    #[must_use]
    pub fn with_correlated_code(mut self, code: &'static str) -> Self {
        self.correlated_code = Some(code);
        self
    }

    #[must_use]
    /// Returns the optional request correlation id.
    pub const fn request_id(&self) -> Option<&RequestId> {
        self.request_id.as_ref()
    }

    #[must_use]
    /// Returns the optional scope correlation id.
    pub const fn scope_id(&self) -> Option<&ScopeId> {
        self.scope_id.as_ref()
    }

    #[must_use]
    /// Returns the optional run correlation id.
    pub const fn run_id(&self) -> Option<&RuntimeRunId> {
        self.run_id.as_ref()
    }

    #[must_use]
    /// Returns the optional turn correlation id.
    pub const fn turn_id(&self) -> Option<&RuntimeTurnId> {
        self.turn_id.as_ref()
    }

    #[must_use]
    /// Returns the optional session correlation id.
    pub const fn session_id(&self) -> Option<&RuntimeSessionId> {
        self.session_id.as_ref()
    }

    #[must_use]
    /// Returns the optional route or adapter label.
    pub fn route(&self) -> Option<&str> {
        self.route.as_deref()
    }

    #[must_use]
    /// Returns the observation kind.
    pub const fn kind(&self) -> DebugObservationKind {
        self.kind
    }

    #[must_use]
    /// Returns the optional stage or boundary label.
    pub fn stage(&self) -> Option<&str> {
        self.stage.as_deref()
    }

    #[must_use]
    /// Returns the optional correlated exact safe diagnostic code.
    pub const fn correlated_code(&self) -> Option<&'static str> {
        self.correlated_code
    }

    #[must_use]
    /// Returns the bounded restricted detail body for an authorized host sink.
    pub fn detail(&self) -> &str {
        &self.detail
    }

    #[must_use]
    /// Returns whether the detail body was truncated to the runtime bound.
    pub const fn detail_truncated(&self) -> bool {
        self.detail_truncated
    }
}

impl fmt::Debug for DebugObservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DebugObservation")
            .field("request_id", &self.request_id)
            .field("scope_id", &self.scope_id)
            .field("run_id", &self.run_id)
            .field("turn_id", &self.turn_id)
            .field("session_id", &self.session_id)
            .field("route", &self.route.as_ref().map(|_| "<redacted>"))
            .field("kind", &self.kind)
            .field("stage", &self.stage)
            .field("correlated_code", &self.correlated_code)
            .field(
                "detail",
                &format_args!("<redacted:{} chars>", self.detail.chars().count()),
            )
            .field("detail_truncated", &self.detail_truncated)
            .finish()
    }
}

impl fmt::Display for DebugObservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:?}", self.kind)?;
        if let Some(stage) = self.stage.as_deref() {
            write!(formatter, " stage={stage}")?;
        }
        if let Some(code) = self.correlated_code {
            write!(formatter, " code={code}")?;
        }
        write!(
            formatter,
            " detail=<redacted:{} chars>",
            self.detail.chars().count()
        )
    }
}

/// Builds one failure-path observation with route, stage, and correlated code.
#[must_use]
pub fn failure_debug_observation(
    kind: DebugObservationKind,
    route: &'static str,
    stage: &'static str,
    code: &'static str,
    detail: impl Into<String>,
) -> DebugObservation {
    DebugObservation::new(kind, detail)
        .with_route(route)
        .with_stage(stage)
        .with_correlated_code(code)
}

fn bound_detail(detail: String) -> (String, bool) {
    let char_count = detail.chars().count();
    if char_count <= MAX_DEBUG_DETAIL_CHARS {
        return (detail, false);
    }
    let truncated: String = detail.chars().take(MAX_DEBUG_DETAIL_CHARS).collect();
    (format!("{truncated}{DEBUG_DETAIL_TRUNCATED_SUFFIX}"), true)
}

fn bound_label(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    if trimmed.chars().count() <= MAX_DEBUG_LABEL_CHARS {
        return Some(trimmed.to_owned());
    }
    Some(trimmed.chars().take(MAX_DEBUG_LABEL_CHARS).collect())
}

#[cfg(test)]
mod tests {
    use super::{
        DEBUG_DETAIL_TRUNCATED_SUFFIX, DebugObservation, DebugObservationKind, MAX_DEBUG_DETAIL_CHARS,
    };
    use crate::RequestId;

    #[test]
    fn detail_truncation_is_explicit() {
        let detail = "x".repeat(MAX_DEBUG_DETAIL_CHARS + 32);
        let observation = DebugObservation::new(DebugObservationKind::WireInbound, detail);

        assert!(observation.detail_truncated());
        assert!(observation.detail().ends_with(DEBUG_DETAIL_TRUNCATED_SUFFIX));
        assert_eq!(
            observation
                .detail()
                .trim_end_matches(DEBUG_DETAIL_TRUNCATED_SUFFIX)
                .chars()
                .count(),
            MAX_DEBUG_DETAIL_CHARS
        );
    }

    #[test]
    fn default_formatting_redacts_detail_body() {
        let observation = DebugObservation::new(
            DebugObservationKind::ProtocolParse,
            "token=secret-provider-detail method=item/plan/delta",
        )
        .with_correlated_code("swallowtail.fixture.malformed")
        .with_stage("rpc.pump")
        .with_route("codex.app_server")
        .with_request_id(RequestId::new("req-1").expect("request id"));

        let rendered = format!("{observation}");
        let debug = format!("{observation:?}");
        assert!(!rendered.contains("secret-provider-detail"));
        assert!(!debug.contains("secret-provider-detail"));
        assert!(rendered.contains("ProtocolParse"));
        assert!(rendered.contains("swallowtail.fixture.malformed"));
        assert!(observation.detail().contains("secret-provider-detail"));
        assert_eq!(observation.kind(), DebugObservationKind::ProtocolParse);
        assert_eq!(observation.stage(), Some("rpc.pump"));
        assert_eq!(observation.route(), Some("codex.app_server"));
        assert_eq!(
            observation.correlated_code(),
            Some("swallowtail.fixture.malformed")
        );
    }
}
