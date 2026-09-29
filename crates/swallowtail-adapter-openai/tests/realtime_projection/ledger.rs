pub(super) const PREPARED_SOURCE: &str = "openai.realtime.prepared";
pub(super) const OBSERVATION_SOURCE: &str = "openai.realtime.active-session";
pub(super) const REALTIME_ROUTE: &str = "openai.realtime";

pub(super) const PREPARED_FACADE: &str = "OpenAiPreparedRealtimeSession";
pub(super) const PROJECTION_OPEN: &str = "open_session_with_projection";

pub(super) const MATRIX_ONLY: &str =
    "matrix or route-wide posture only; no exact prepared realtime authority";

/// One exact `openai.realtime` census row and its adapter disposition.
pub(super) struct LedgerEntry {
    pub(super) route_id: &'static str,
    pub(super) operation_shape: &'static str,
    pub(super) semantic_id: &'static str,
    pub(super) emitted_by: &'static [&'static str],
    pub(super) withheld_because: &'static str,
}

const BOTH: &[&str] = &[PREPARED_FACADE, PROJECTION_OPEN];

const fn emitted(
    operation_shape: &'static str,
    semantic_id: &'static str,
    emitted_by: &'static [&'static str],
) -> LedgerEntry {
    LedgerEntry {
        route_id: REALTIME_ROUTE,
        operation_shape,
        semantic_id,
        emitted_by,
        withheld_because: "",
    }
}

const fn withheld(
    operation_shape: &'static str,
    semantic_id: &'static str,
    withheld_because: &'static str,
) -> LedgerEntry {
    LedgerEntry {
        route_id: REALTIME_ROUTE,
        operation_shape,
        semantic_id,
        emitted_by: &[],
        withheld_because,
    }
}

/// Deterministic disposition of exactly the 15 `openai.realtime` census rows.
///
/// The ledger claims nothing about the remaining 716 census rows.
pub(super) const REALTIME_FIRST_TRANCHE: [LedgerEntry; 15] = [
    withheld(
        "model-catalogue",
        "feature.model-catalogue",
        "no prepared realtime plan carries model-catalogue authority",
    ),
    emitted(
        "interactive-session",
        "feature.realtime-media-session",
        BOTH,
    ),
    emitted("route-observation", "feature.streaming-events", BOTH),
    emitted("route-observation", "feature.usage-evidence", BOTH),
    emitted("route-capability", "feature.output-token-limit", BOTH),
    emitted("route-capability", "feature.reasoning-selection", BOTH),
    emitted(
        "route-capability",
        "feature.cancellation-or-interruption",
        BOTH,
    ),
    withheld(
        "session-lifecycle",
        "feature.persistent-session-posture",
        MATRIX_ONLY,
    ),
    emitted("route-capability", "feature.prepared-facade", BOTH),
    withheld(
        "route-observation",
        "feature.activity-observation",
        "no prepared realtime plan requires the observable-activity capability",
    ),
    emitted(
        "interactive-session",
        "feature.active-session-reasoning-ack",
        &[PROJECTION_OPEN],
    ),
    emitted(
        "interactive-session",
        "control.reasoning-selection-session-start",
        BOTH,
    ),
    emitted("interactive-session", "control.maximum-output-tokens", BOTH),
    emitted("interactive-session", "control.realtime-media-config", BOTH),
    emitted(
        "interactive-session",
        "control.planned-connection-rollover",
        BOTH,
    ),
];
