pub(super) const ROUTE: &str = "opencode.http";

pub(super) const CATALOGUE: &str = "OpenCodePreparedCatalogue";
pub(super) const RUN_MAXIMAL: &str = "OpenCodePreparedRun[maximal]";
pub(super) const SESSION_MAXIMAL: &str = "OpenCodePreparedSession[maximal]";
pub(super) const SESSION_DETACHED: &str = "OpenCodePreparedSession[with_active_turn_detachment]";
pub(super) const SESSION_CATALOGUE: &str = "OpenCodePreparedSessionCatalogue";
pub(super) const SESSION_IMPORT: &str = "OpenCodePreparedSessionImport";
pub(super) const DELETE: &str = "OpenCodePreparedDelete";
pub(super) const HISTORY: &str = "OpenCodePreparedSessionHistory";
pub(super) const RECONCILIATION: &str = "OpenCodePreparedSessionReconciliation";

pub(super) const PROFILES: [&str; 9] = [
    CATALOGUE,
    RUN_MAXIMAL,
    SESSION_MAXIMAL,
    SESSION_DETACHED,
    SESSION_CATALOGUE,
    SESSION_IMPORT,
    DELETE,
    HISTORY,
    RECONCILIATION,
];

const RUN: &[&str] = &[RUN_MAXIMAL];
const SESSION: &[&str] = &[SESSION_MAXIMAL, SESSION_DETACHED];
const SESSION_IMPORTING: &[&str] = &[SESSION_IMPORT];
const RUN_SESSION_IMPORT: &[&str] = &[
    RUN_MAXIMAL,
    SESSION_MAXIMAL,
    SESSION_DETACHED,
    SESSION_IMPORT,
];
const PREPARED: &[&str] = &[
    CATALOGUE,
    RUN_MAXIMAL,
    SESSION_MAXIMAL,
    SESSION_DETACHED,
    SESSION_CATALOGUE,
    SESSION_IMPORT,
    DELETE,
];

pub(super) struct LedgerEntry {
    pub(super) shape: &'static str,
    pub(super) semantic_id: &'static str,
    pub(super) emitted_by: &'static [&'static str],
    pub(super) withheld_because: &'static str,
}

pub(super) const LEDGER: [LedgerEntry; 35] = [
    LedgerEntry {
        shape: "model-catalogue",
        semantic_id: "feature.model-catalogue",
        emitted_by: &[CATALOGUE],
        withheld_because: "",
    },
    LedgerEntry {
        shape: "structured-run",
        semantic_id: "feature.structured-run",
        emitted_by: RUN,
        withheld_because: "",
    },
    LedgerEntry {
        shape: "interactive-session",
        semantic_id: "feature.interactive-session",
        emitted_by: &[SESSION_MAXIMAL, SESSION_DETACHED, SESSION_IMPORT],
        withheld_because: "",
    },
    LedgerEntry {
        shape: "route-observation",
        semantic_id: "feature.streaming-events",
        emitted_by: RUN_SESSION_IMPORT,
        withheld_because: "",
    },
    LedgerEntry {
        shape: "route-observation",
        semantic_id: "feature.usage-evidence",
        emitted_by: RUN_SESSION_IMPORT,
        withheld_because: "",
    },
    LedgerEntry {
        shape: "route-capability",
        semantic_id: "feature.reasoning-selection",
        emitted_by: RUN,
        withheld_because: "",
    },
    LedgerEntry {
        shape: "route-capability",
        semantic_id: "feature.structured-output",
        emitted_by: RUN,
        withheld_because: "",
    },
    LedgerEntry {
        shape: "route-capability",
        semantic_id: "feature.attachments",
        emitted_by: &[RUN_MAXIMAL, SESSION_MAXIMAL, SESSION_IMPORT],
        withheld_because: "",
    },
    LedgerEntry {
        shape: "route-capability",
        semantic_id: "feature.permission-exchange",
        emitted_by: &[RUN_MAXIMAL, SESSION_MAXIMAL],
        withheld_because: "",
    },
    LedgerEntry {
        shape: "route-capability",
        semantic_id: "feature.question-exchange",
        emitted_by: &[RUN_MAXIMAL, SESSION_MAXIMAL],
        withheld_because: "",
    },
    LedgerEntry {
        shape: "route-capability",
        semantic_id: "feature.cancellation-or-interruption",
        emitted_by: RUN_SESSION_IMPORT,
        withheld_because: "",
    },
    LedgerEntry {
        shape: "session-lifecycle",
        semantic_id: "feature.load-session",
        emitted_by: &[SESSION_MAXIMAL, SESSION_DETACHED, SESSION_IMPORT],
        withheld_because: "",
    },
    LedgerEntry {
        shape: "session-lifecycle",
        semantic_id: "feature.resume-session",
        emitted_by: &[SESSION_MAXIMAL, SESSION_DETACHED, SESSION_IMPORT],
        withheld_because: "",
    },
    LedgerEntry {
        shape: "session-lifecycle",
        semantic_id: "feature.provider-session-catalogue",
        emitted_by: &[SESSION_CATALOGUE],
        withheld_because: "",
    },
    LedgerEntry {
        shape: "session-lifecycle",
        semantic_id: "feature.provider-session-import",
        emitted_by: SESSION_IMPORTING,
        withheld_because: "",
    },
    LedgerEntry {
        shape: "route-capability",
        semantic_id: "feature.working-resource",
        emitted_by: &[
            RUN_MAXIMAL,
            SESSION_MAXIMAL,
            SESSION_DETACHED,
            SESSION_CATALOGUE,
            SESSION_IMPORT,
        ],
        withheld_because: "",
    },
    LedgerEntry {
        shape: "session-lifecycle",
        semantic_id: "feature.provider-session-delete",
        emitted_by: &[DELETE],
        withheld_because: "",
    },
    LedgerEntry {
        shape: "route-capability",
        semantic_id: "feature.owned-remote-resource-cleanup",
        emitted_by: RUN,
        withheld_because: "",
    },
    LedgerEntry {
        shape: "session-lifecycle",
        semantic_id: "feature.persistent-session-posture",
        emitted_by: &[SESSION_DETACHED, SESSION_IMPORT],
        withheld_because: "",
    },
    LedgerEntry {
        shape: "route-capability",
        semantic_id: "feature.prepared-facade",
        emitted_by: PREPARED,
        withheld_because: "",
    },
    LedgerEntry {
        shape: "route-observation",
        semantic_id: "feature.activity-observation",
        emitted_by: &[RUN_MAXIMAL, SESSION_MAXIMAL, SESSION_DETACHED],
        withheld_because: "",
    },
    LedgerEntry {
        shape: "structured-run",
        semantic_id: "control.model-selection",
        emitted_by: RUN,
        withheld_because: "",
    },
    LedgerEntry {
        shape: "interactive-session",
        semantic_id: "control.model-selection",
        emitted_by: SESSION,
        withheld_because: "",
    },
    LedgerEntry {
        shape: "structured-run",
        semantic_id: "control.reasoning-selection",
        emitted_by: RUN,
        withheld_because: "",
    },
    LedgerEntry {
        shape: "interactive-session",
        semantic_id: "control.reasoning-selection",
        emitted_by: &[],
        withheld_because: "matrix-descriptor-only; no retained interactive-session owner",
    },
    LedgerEntry {
        shape: "structured-run",
        semantic_id: "control.structured-output",
        emitted_by: RUN,
        withheld_because: "",
    },
    LedgerEntry {
        shape: "structured-run",
        semantic_id: "control.attachments",
        emitted_by: RUN,
        withheld_because: "",
    },
    LedgerEntry {
        shape: "interactive-session",
        semantic_id: "control.attachments",
        emitted_by: &[SESSION_MAXIMAL],
        withheld_because: "",
    },
    LedgerEntry {
        shape: "structured-run",
        semantic_id: "control.provider-callbacks",
        emitted_by: RUN,
        withheld_because: "",
    },
    LedgerEntry {
        shape: "interactive-session",
        semantic_id: "control.provider-callbacks",
        emitted_by: &[SESSION_MAXIMAL],
        withheld_because: "",
    },
    LedgerEntry {
        shape: "interactive-session",
        semantic_id: "control.active-turn-detachment",
        emitted_by: &[SESSION_DETACHED],
        withheld_because: "",
    },
    LedgerEntry {
        shape: "session-management",
        semantic_id: "control.load-session",
        emitted_by: SESSION,
        withheld_because: "",
    },
    LedgerEntry {
        shape: "session-management",
        semantic_id: "control.resume-session",
        emitted_by: SESSION,
        withheld_because: "",
    },
    LedgerEntry {
        shape: "session-management",
        semantic_id: "control.provider-session-catalogue",
        emitted_by: &[SESSION_CATALOGUE],
        withheld_because: "",
    },
    LedgerEntry {
        shape: "per-turn",
        semantic_id: "control.provider-turn-reference",
        emitted_by: &[],
        withheld_because: "matrix-descriptor-only; reconciliation rejects provider turn references",
    },
];
