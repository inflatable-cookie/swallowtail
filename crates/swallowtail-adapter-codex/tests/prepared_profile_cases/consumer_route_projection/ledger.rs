pub(super) const FIXTURE_VERSION: &str = "0.146.0";
pub(super) const APP_SERVER_ROUTE: &str = "codex.app-server";

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
    pub(super) route_id: &'static str,
    pub(super) operation_shape: &'static str,
    pub(super) semantic_id: &'static str,
    pub(super) emitted_by: &'static [&'static str],
    pub(super) withheld_because: &'static str,
}

const fn emitted(
    operation_shape: &'static str,
    semantic_id: &'static str,
    emitted_by: &'static [&'static str],
) -> LedgerEntry {
    LedgerEntry {
        route_id: APP_SERVER_ROUTE,
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
        route_id: APP_SERVER_ROUTE,
        operation_shape,
        semantic_id,
        emitted_by: &[],
        withheld_because,
    }
}

/// Deterministic disposition of exactly the 36 `codex.app-server` census rows.
///
/// The ledger claims nothing about the remaining 716 census rows.
pub(super) const CODEX_FIRST_TRANCHE: [LedgerEntry; 36] = [
    emitted("model-catalogue", "feature.model-catalogue", &[CATALOGUE]),
    withheld("structured-run", "feature.structured-run", EXEC_ONLY),
    emitted(
        "interactive-session",
        "feature.interactive-session",
        &[SESSION, SESSION_IMPORT],
    ),
    emitted("route-observation", "feature.streaming-events", &[SESSION]),
    withheld("route-observation", "feature.usage-evidence", MATRIX_ONLY),
    emitted(
        "route-capability",
        "feature.reasoning-selection",
        &[SESSION],
    ),
    withheld("route-capability", "feature.structured-output", EXEC_ONLY),
    withheld("route-capability", "feature.attachments", EXEC_ONLY),
    emitted(
        "route-capability",
        "feature.consumer-tool-exchange",
        &[SESSION],
    ),
    emitted("route-capability", "feature.question-exchange", &[SESSION]),
    withheld(
        "route-capability",
        "feature.cancellation-or-interruption",
        MATRIX_ONLY,
    ),
    emitted(
        "session-lifecycle",
        "feature.load-session",
        &[SESSION, SESSION_IMPORT],
    ),
    emitted(
        "session-lifecycle",
        "feature.resume-session",
        &[SESSION, SESSION_IMPORT],
    ),
    emitted(
        "session-lifecycle",
        "feature.provider-session-catalogue",
        &[SESSION_CATALOGUE],
    ),
    emitted(
        "session-lifecycle",
        "feature.provider-session-import",
        &[SESSION_IMPORT],
    ),
    emitted(
        "route-capability",
        "feature.working-resource",
        &[
            SESSION,
            SESSION_CATALOGUE,
            SESSION_HISTORY,
            SESSION_IMPORT,
            SESSION_RECONCILIATION,
        ],
    ),
    withheld(
        "route-capability",
        "feature.bounded-workspace-text-write",
        "no prepared app-server plan requires the bounded workspace text-write capability",
    ),
    withheld("route-capability", "feature.external-search", EXEC_ONLY),
    emitted(
        "session-lifecycle",
        "feature.provider-session-archive",
        &[ARCHIVE],
    ),
    emitted(
        "session-lifecycle",
        "feature.provider-session-restore",
        &[RESTORE],
    ),
    emitted(
        "session-lifecycle",
        "feature.provider-session-delete",
        &[DELETE],
    ),
    emitted(
        "session-lifecycle",
        "feature.persistent-session-posture",
        &[SESSION_HISTORY, SESSION_IMPORT, SESSION_RECONCILIATION],
    ),
    emitted(
        "route-capability",
        "feature.prepared-facade",
        &CODEX_FACADES,
    ),
    emitted(
        "route-observation",
        "feature.activity-observation",
        &[SESSION],
    ),
    emitted(
        "interactive-session",
        "control.model-selection",
        &[
            SESSION,
            SESSION_HISTORY,
            SESSION_IMPORT,
            SESSION_RECONCILIATION,
        ],
    ),
    emitted(
        "interactive-session",
        "control.reasoning-selection",
        &[SESSION],
    ),
    emitted("interactive-session", "control.session-options", &[SESSION]),
    emitted(
        "interactive-session",
        "control.tool-declarations",
        &[SESSION],
    ),
    emitted(
        "interactive-session",
        "control.developer-instructions",
        &[SESSION],
    ),
    emitted("interactive-session", "control.idioms", &[SESSION]),
    emitted(
        "interactive-session",
        "control.user-input-exchange",
        &[SESSION],
    ),
    emitted("session-management", "control.load-session", &[SESSION]),
    emitted("session-management", "control.resume-session", &[SESSION]),
    emitted(
        "session-management",
        "control.session-catalogue-bounds",
        &[SESSION_CATALOGUE],
    ),
    emitted(
        "session-management",
        "control.session-history-bounds",
        &[SESSION_HISTORY],
    ),
    emitted(
        "session-management",
        "control.session-reconciliation",
        &[SESSION_RECONCILIATION],
    ),
];

/// Feature rows the app-server route proves that this tranche withholds.
///
/// Both are withheld at construction rather than emitted and then filtered, so
/// no facade may publish them and the ledger needs no exception list.
pub(super) const WITHHELD_OUT_OF_TRANCHE: [&str; 2] = [
    "feature.provider-session-history",
    "feature.provider-session-reconciliation",
];
