pub(super) const PREPARED_SOURCE: &str = "openai.realtime.prepared";
pub(super) const OBSERVATION_SOURCE: &str = "openai.realtime.active-session";

pub(super) const PREPARED_FACADE: &str = "OpenAiPreparedRealtimeSession";
pub(super) const PROJECTION_OPEN: &str = "open_session_with_projection";

pub(super) const MATRIX_ONLY: &str =
    "matrix or route-wide posture only; no exact prepared realtime authority";

/// One exact `openai.realtime` census row and its adapter disposition.
pub(super) struct LedgerEntry {
    pub(super) semantic_id: &'static str,
    pub(super) emitted_by: &'static [&'static str],
    pub(super) withheld_because: &'static str,
}

/// Deterministic disposition of exactly the 15 `openai.realtime` census rows.
///
/// The ledger claims nothing about the remaining 716 census rows.
pub(super) const REALTIME_FIRST_TRANCHE: [LedgerEntry; 15] = [
    LedgerEntry {
        semantic_id: "feature.model-catalogue",
        emitted_by: &[],
        withheld_because: "no prepared realtime plan carries model-catalogue authority",
    },
    LedgerEntry {
        semantic_id: "feature.realtime-media-session",
        emitted_by: &[PREPARED_FACADE, PROJECTION_OPEN],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.streaming-events",
        emitted_by: &[PREPARED_FACADE, PROJECTION_OPEN],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.usage-evidence",
        emitted_by: &[PREPARED_FACADE, PROJECTION_OPEN],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.output-token-limit",
        emitted_by: &[PREPARED_FACADE, PROJECTION_OPEN],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.reasoning-selection",
        emitted_by: &[PREPARED_FACADE, PROJECTION_OPEN],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.cancellation-or-interruption",
        emitted_by: &[PREPARED_FACADE, PROJECTION_OPEN],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.persistent-session-posture",
        emitted_by: &[],
        withheld_because: MATRIX_ONLY,
    },
    LedgerEntry {
        semantic_id: "feature.prepared-facade",
        emitted_by: &[PREPARED_FACADE, PROJECTION_OPEN],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.activity-observation",
        emitted_by: &[],
        withheld_because: "no prepared realtime plan requires the observable-activity capability",
    },
    LedgerEntry {
        semantic_id: "feature.active-session-reasoning-ack",
        emitted_by: &[PROJECTION_OPEN],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.reasoning-selection-session-start",
        emitted_by: &[PREPARED_FACADE, PROJECTION_OPEN],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.maximum-output-tokens",
        emitted_by: &[PREPARED_FACADE, PROJECTION_OPEN],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.realtime-media-config",
        emitted_by: &[PREPARED_FACADE, PROJECTION_OPEN],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.planned-connection-rollover",
        emitted_by: &[PREPARED_FACADE, PROJECTION_OPEN],
        withheld_because: "",
    },
];
