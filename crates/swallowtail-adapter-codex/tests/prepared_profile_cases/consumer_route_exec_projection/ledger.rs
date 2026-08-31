pub(super) const FIXTURE_VERSION: &str = "0.149.1";

/// Exact census route and qualified behavior revision of the fixture runs.
pub(super) const EXEC_ROUTE: &str = "codex.exec";
pub(super) const EXEC_BEHAVIOR: &str = "codex.exec.jsonl-v1";

/// One prepared exec run carrying every optional exec input the route admits.
pub(super) const MAXIMAL: &str = "CodexPreparedExec[maximal]";
/// One prepared exec run carrying only the inputs exec preparation requires.
pub(super) const MINIMAL: &str = "CodexPreparedExec[minimal]";

pub(super) const EXEC_PROFILES: [&str; 2] = [MAXIMAL, MINIMAL];

/// Exact census operation shapes the `codex.exec` tranche spans.
pub(super) const EXEC_OPERATION_SHAPES: [&str; 7] = [
    "model-catalogue",
    "structured-run",
    "interactive-session",
    "route-observation",
    "route-capability",
    "session-lifecycle",
    "session-management",
];
pub(super) const BOTH: &[&str] = &[MAXIMAL, MINIMAL];
pub(super) const OPTIONAL: &[&str] = &[MAXIMAL];

pub(super) const ONE_SHOT: &str =
    "codex.exec is one-shot; no prepared exec plan requires the capability";
pub(super) const TOOLS_REJECTED: &str = "Codex exec preparation rejects declared tools";

/// One exact `codex.exec` census row and its adapter disposition.
pub(super) struct LedgerEntry {
    pub(super) route_id: &'static str,
    pub(super) operation_shape: &'static str,
    pub(super) semantic_id: &'static str,
    pub(super) emitted_by: &'static [&'static str],
    pub(super) withheld_because: &'static str,
}

/// App-server identities with no `codex.exec` census row at all.
///
/// Package ownership is not evidence: these must never be constructed by the
/// exec contribution, so the ledger needs no exception list for them.
pub(super) const WITHHELD_APP_SERVER_ONLY: [&str; 7] = [
    "feature.provider-session-history",
    "feature.provider-session-reconciliation",
    "control.session-options",
    "control.developer-instructions",
    "control.idioms",
    "control.user-input-exchange",
    "control.session-catalogue-bounds",
];
