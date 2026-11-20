use std::fmt;
use swallowtail_core::{ExtensionNamespace, SafeDiagnostic};
use swallowtail_protocol_acp::RpcError;
use swallowtail_runtime::RuntimeFailure;

/// How [`crate::AcpActivityProjection`] handles `AcpSessionUpdate::Plan` updates.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PlanProjection {
    /// Ignore plan updates in activity projection.
    #[default]
    Ignore,
    /// Project plan updates into a structured task-list activity snapshot.
    TaskList,
}

/// How [`crate::AcpActivityProjection`] classifies message chunks.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum MessageClassification {
    /// Standard ACP classification (agent -> final answer, thought -> reasoning summary).
    #[default]
    Standard,
    /// Also classify `[MODE_UPDATE]` agent text and `Warning:` thought text as lifecycle-only.
    ModeAndWarning,
}

/// How [`crate::ActiveTurn`] maps ACP `stopReason` values to terminal statuses.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum StopReasonPolicy {
    /// Recognize `end_turn`, `cancelled`, and `max_tokens` (provider failure).
    #[default]
    Standard,
    /// Also treat `max_turn_requests` and `refusal` as provider-stopped failures.
    ExtendedProviderStops,
}

/// How [`crate::ActiveTurn`] handles `AcpSessionUpdate::Usage` updates.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum UsageProgressPolicy {
    /// Do not emit a `Progress` runtime event for usage updates.
    #[default]
    Silent,
    /// Emit a `Progress` runtime event for usage updates.
    EmitProgress,
}

/// Configuration for [`crate::AcpActivityProjection`].
#[derive(Clone, Copy)]
pub struct AcpActivityConfig {
    /// Prefix for minted `ActivityId` values (e.g. `"kiro-acp"`).
    pub id_prefix: &'static str,
    /// Prefix for unknown `ActivityNamespace` values (e.g. `"kiro.acp"`).
    pub namespace_prefix: &'static str,
    /// Whether `Plan` updates emit task-list activity snapshots.
    pub plan_projection: PlanProjection,
    /// Message chunk classification policy.
    pub message_classification: MessageClassification,
    /// Constructor for malformed-payload runtime failures.
    pub malformed: fn() -> RuntimeFailure,
}

impl fmt::Debug for AcpActivityConfig {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AcpActivityConfig")
            .field("id_prefix", &self.id_prefix)
            .field("namespace_prefix", &self.namespace_prefix)
            .field("plan_projection", &self.plan_projection)
            .field("message_classification", &self.message_classification)
            .finish()
    }
}

/// Provider-scoped diagnostic factory table for an ACP adapter.
///
/// Every code and message stays owned by the adapter so provider identities and
/// consumer-visible diagnostics remain exact across migrations.
#[derive(Clone, Copy)]
pub struct AcpDiagnostics {
    /// Route label emitted in debug observations (e.g. `"kiro.acp"`).
    pub route_name: &'static str,
    /// Activity projection configuration.
    pub activity: AcpActivityConfig,
    /// Malformed payload failure.
    pub malformed: fn() -> RuntimeFailure,
    /// Framing or JSON-RPC codec failure.
    pub protocol_failure: fn() -> RuntimeFailure,
    /// Negotiated protocol version mismatch failure.
    pub version_mismatch: fn() -> RuntimeFailure,
    /// Write or request attempted on a closed connection.
    pub connection_closed: fn() -> RuntimeFailure,
    /// Pending request correlation limit exceeded.
    pub correlation_limit: fn() -> RuntimeFailure,
    /// Provider returned more than one session identity.
    pub session_duplicate: fn() -> RuntimeFailure,
    /// Turn started while another turn is still active.
    pub turn_active: fn() -> RuntimeFailure,
    /// Response arrived for an unknown correlation id.
    pub unknown_response: fn() -> RuntimeFailure,
    /// `session/update` arrived with no active turn (and was not session-scoped metadata).
    pub update_without_turn: fn() -> RuntimeFailure,
    /// Provider sent an unsupported `session/cancel` notification.
    pub agent_cancel_unsupported: fn() -> RuntimeFailure,
    /// Provider sent an unsupported notification.
    pub notification_unsupported: fn() -> RuntimeFailure,
    /// Provider sent an unsupported client callback request.
    pub callback_unsupported: fn() -> RuntimeFailure,
    /// Provider requested permission without an active turn.
    pub permission_without_turn: fn() -> RuntimeFailure,
    /// Callback `sessionId` does not match the active session.
    pub session_mismatch_callback: fn() -> RuntimeFailure,
    /// Update `sessionId` does not match the active session.
    pub session_mismatch_update: fn() -> RuntimeFailure,
    /// Process or transport cleanup failed.
    pub process_cleanup_failed: fn() -> SafeDiagnostic,
    /// Cleanup outcome was not recorded before close completed.
    pub cleanup_missing: fn() -> SafeDiagnostic,
    /// Connection ended unexpectedly while requests or a turn were active.
    pub connection_ended: fn() -> RuntimeFailure,
    /// Accumulated turn text output exceeded the adapter byte limit.
    pub output_limit_exceeded: fn() -> RuntimeFailure,
    /// Provider stopped before completing the turn (e.g. `max_tokens`).
    pub prompt_stopped: fn() -> SafeDiagnostic,
    /// Provider returned an unsupported `stopReason`.
    pub stop_reason_unsupported: fn() -> RuntimeFailure,
    /// Scoped task service was missing when starting a turn.
    pub task_service_missing: fn() -> RuntimeFailure,
    /// Protocol pump task did not join cleanly.
    pub task_join_failed: fn() -> SafeDiagnostic,
    /// Prompt task did not join cleanly during turn close.
    pub turn_join_failed: fn() -> SafeDiagnostic,
    /// Working-resource service disappeared during cleanup.
    pub resource_release_failed: fn() -> SafeDiagnostic,
    /// Classifies a JSON-RPC error response into an adapter failure.
    pub classify_rpc_error: fn(&RpcError) -> RuntimeFailure,
    /// Extension namespace for observed permission requests.
    pub permission_namespace: fn() -> ExtensionNamespace,
    /// Optional unsupported-feature failure helper used by shared turn validation.
    pub unsupported: fn(&'static str) -> RuntimeFailure,
}

impl fmt::Debug for AcpDiagnostics {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AcpDiagnostics")
            .field("route_name", &self.route_name)
            .field("activity", &self.activity)
            .finish()
    }
}

/// Default ACP permission observation namespace (`acp/session/request-permission`).
#[must_use]
pub fn default_permission_namespace() -> ExtensionNamespace {
    ExtensionNamespace::new("acp/session/request-permission").expect("static namespace is valid")
}
