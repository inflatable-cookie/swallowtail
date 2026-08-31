pub(super) const FIXTURE_VERSION: &str = "0.146.0";

pub(super) const CATALOGUE: &str = "CodexPreparedCatalogue";
pub(super) const SESSION: &str = "CodexPreparedSession";
pub(super) const SESSION_CATALOGUE: &str = "CodexPreparedSessionCatalogue";
pub(super) const SESSION_HISTORY: &str = "CodexPreparedSessionHistory";
pub(super) const SESSION_IMPORT: &str = "CodexPreparedSessionImport";
pub(super) const SESSION_RECONCILIATION: &str = "CodexPreparedSessionReconciliation";
pub(super) const ARCHIVE: &str = "CodexPreparedArchive";
pub(super) const RESTORE: &str = "CodexPreparedRestore";
pub(super) const DELETE: &str = "CodexPreparedDelete";

pub(super) const CODEX_FACADES: [&str; 9] = [
    CATALOGUE,
    SESSION,
    SESSION_CATALOGUE,
    SESSION_HISTORY,
    SESSION_IMPORT,
    SESSION_RECONCILIATION,
    ARCHIVE,
    RESTORE,
    DELETE,
];

pub(super) const MATRIX_ONLY: &str =
    "matrix or route-wide posture only; no exact app-server prepared authority";
pub(super) const EXEC_ONLY: &str =
    "proved only by the codex.exec prepared route, not codex.app-server";

/// One exact `codex.app-server` census row and its adapter disposition.
pub(super) struct LedgerEntry {
    pub(super) semantic_id: &'static str,
    pub(super) emitted_by: &'static [&'static str],
    pub(super) withheld_because: &'static str,
}

/// Deterministic disposition of exactly the 36 `codex.app-server` census rows.
///
/// The ledger claims nothing about the remaining 716 census rows.
pub(super) const CODEX_FIRST_TRANCHE: [LedgerEntry; 36] = [
    LedgerEntry {
        semantic_id: "feature.model-catalogue",
        emitted_by: &[CATALOGUE],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.structured-run",
        emitted_by: &[],
        withheld_because: EXEC_ONLY,
    },
    LedgerEntry {
        semantic_id: "feature.interactive-session",
        emitted_by: &[SESSION, SESSION_IMPORT],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.streaming-events",
        emitted_by: &[SESSION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.usage-evidence",
        emitted_by: &[],
        withheld_because: MATRIX_ONLY,
    },
    LedgerEntry {
        semantic_id: "feature.reasoning-selection",
        emitted_by: &[SESSION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.structured-output",
        emitted_by: &[],
        withheld_because: EXEC_ONLY,
    },
    LedgerEntry {
        semantic_id: "feature.attachments",
        emitted_by: &[],
        withheld_because: EXEC_ONLY,
    },
    LedgerEntry {
        semantic_id: "feature.consumer-tool-exchange",
        emitted_by: &[SESSION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.question-exchange",
        emitted_by: &[SESSION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.cancellation-or-interruption",
        emitted_by: &[],
        withheld_because: MATRIX_ONLY,
    },
    LedgerEntry {
        semantic_id: "feature.load-session",
        emitted_by: &[SESSION, SESSION_IMPORT],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.resume-session",
        emitted_by: &[SESSION, SESSION_IMPORT],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.provider-session-catalogue",
        emitted_by: &[SESSION_CATALOGUE],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.provider-session-import",
        emitted_by: &[SESSION_IMPORT],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.working-resource",
        emitted_by: &[
            SESSION,
            SESSION_CATALOGUE,
            SESSION_HISTORY,
            SESSION_IMPORT,
            SESSION_RECONCILIATION,
        ],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.bounded-workspace-text-write",
        emitted_by: &[],
        withheld_because: "no prepared app-server plan requires the bounded workspace text-write capability",
    },
    LedgerEntry {
        semantic_id: "feature.external-search",
        emitted_by: &[],
        withheld_because: EXEC_ONLY,
    },
    LedgerEntry {
        semantic_id: "feature.provider-session-archive",
        emitted_by: &[ARCHIVE],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.provider-session-restore",
        emitted_by: &[RESTORE],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.provider-session-delete",
        emitted_by: &[DELETE],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.persistent-session-posture",
        emitted_by: &[SESSION_HISTORY, SESSION_IMPORT, SESSION_RECONCILIATION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.prepared-facade",
        emitted_by: &CODEX_FACADES,
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.activity-observation",
        emitted_by: &[SESSION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.model-selection",
        emitted_by: &[
            SESSION,
            SESSION_HISTORY,
            SESSION_IMPORT,
            SESSION_RECONCILIATION,
        ],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.reasoning-selection",
        emitted_by: &[SESSION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.session-options",
        emitted_by: &[SESSION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.tool-declarations",
        emitted_by: &[SESSION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.developer-instructions",
        emitted_by: &[SESSION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.idioms",
        emitted_by: &[SESSION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.user-input-exchange",
        emitted_by: &[SESSION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.load-session",
        emitted_by: &[SESSION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.resume-session",
        emitted_by: &[SESSION],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.session-catalogue-bounds",
        emitted_by: &[SESSION_CATALOGUE],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.session-history-bounds",
        emitted_by: &[SESSION_HISTORY],
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.session-reconciliation",
        emitted_by: &[SESSION_RECONCILIATION],
        withheld_because: "",
    },
];

/// Feature rows the app-server route proves that this tranche withholds.
///
/// Both are withheld at construction rather than emitted and then filtered, so
/// no facade may publish them and the ledger needs no exception list.
pub(super) const WITHHELD_OUT_OF_TRANCHE: [&str; 2] = [
    "feature.provider-session-history",
    "feature.provider-session-reconciliation",
];
