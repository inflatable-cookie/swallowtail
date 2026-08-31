pub(super) const FIXTURE_VERSION: &str = "0.149.1";

/// Exact census route and qualified behavior revision of the fixture runs.
pub(super) const EXEC_ROUTE: &str = "codex.exec";
pub(super) const EXEC_BEHAVIOR: &str = "codex.exec.jsonl-v1";

/// One prepared exec run carrying every optional exec input the route admits.
pub(super) const MAXIMAL: &str = "CodexPreparedExec[maximal]";
/// One prepared exec run carrying only the inputs exec preparation requires.
pub(super) const MINIMAL: &str = "CodexPreparedExec[minimal]";

pub(super) const EXEC_PROFILES: [&str; 2] = [MAXIMAL, MINIMAL];
pub(super) const BOTH: &[&str] = &[MAXIMAL, MINIMAL];
pub(super) const OPTIONAL: &[&str] = &[MAXIMAL];

const ONE_SHOT: &str = "codex.exec is one-shot; no prepared exec plan requires the capability";
const TOOLS_REJECTED: &str = "Codex exec preparation rejects declared tools";

/// One exact `codex.exec` census row and its adapter disposition.
pub(super) struct LedgerEntry {
    pub(super) semantic_id: &'static str,
    pub(super) emitted_by: &'static [&'static str],
    pub(super) withheld_because: &'static str,
}

/// Deterministic disposition of exactly the 35 `codex.exec` census rows.
///
/// The ledger claims nothing about `codex.app-server` or any other route.
pub(super) const CODEX_EXEC_TRANCHE: [LedgerEntry; 35] = [
    LedgerEntry {
        semantic_id: "feature.model-catalogue",
        emitted_by: &[],
        withheld_because: "no prepared exec plan requires the model-catalogue capability",
    },
    LedgerEntry {
        semantic_id: "feature.structured-run",
        emitted_by: BOTH,
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.interactive-session",
        emitted_by: &[],
        withheld_because: ONE_SHOT,
    },
    LedgerEntry {
        semantic_id: "feature.streaming-events",
        emitted_by: BOTH,
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.usage-evidence",
        emitted_by: &[],
        withheld_because: "no prepared exec plan requires the usage-reporting capability",
    },
    LedgerEntry {
        semantic_id: "feature.reasoning-selection",
        emitted_by: OPTIONAL,
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.structured-output",
        emitted_by: OPTIONAL,
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.attachments",
        emitted_by: OPTIONAL,
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.consumer-tool-exchange",
        emitted_by: &[],
        withheld_because: TOOLS_REJECTED,
    },
    LedgerEntry {
        semantic_id: "feature.question-exchange",
        emitted_by: &[],
        withheld_because: "no prepared exec plan carries a session access policy",
    },
    LedgerEntry {
        semantic_id: "feature.cancellation-or-interruption",
        emitted_by: &[],
        withheld_because: "no prepared exec plan requires the interruption capability",
    },
    LedgerEntry {
        semantic_id: "feature.load-session",
        emitted_by: &[],
        withheld_because: ONE_SHOT,
    },
    LedgerEntry {
        semantic_id: "feature.resume-session",
        emitted_by: &[],
        withheld_because: ONE_SHOT,
    },
    LedgerEntry {
        semantic_id: "feature.provider-session-catalogue",
        emitted_by: &[],
        withheld_because: ONE_SHOT,
    },
    LedgerEntry {
        semantic_id: "feature.provider-session-import",
        emitted_by: &[],
        withheld_because: ONE_SHOT,
    },
    LedgerEntry {
        semantic_id: "feature.working-resource",
        emitted_by: &[],
        withheld_because: "no prepared exec plan requires the working-resource capability",
    },
    LedgerEntry {
        semantic_id: "feature.bounded-workspace-text-write",
        emitted_by: &[],
        withheld_because: "no prepared exec plan requires the workspace text-write capability",
    },
    LedgerEntry {
        semantic_id: "feature.external-search",
        emitted_by: OPTIONAL,
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.provider-session-archive",
        emitted_by: &[],
        withheld_because: ONE_SHOT,
    },
    LedgerEntry {
        semantic_id: "feature.provider-session-restore",
        emitted_by: &[],
        withheld_because: ONE_SHOT,
    },
    LedgerEntry {
        semantic_id: "feature.provider-session-delete",
        emitted_by: &[],
        withheld_because: ONE_SHOT,
    },
    LedgerEntry {
        semantic_id: "feature.persistent-session-posture",
        emitted_by: &[],
        withheld_because: "the census marks durable persistence not applicable to codex.exec",
    },
    LedgerEntry {
        semantic_id: "feature.prepared-facade",
        emitted_by: BOTH,
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "feature.activity-observation",
        emitted_by: BOTH,
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.model-selection",
        emitted_by: BOTH,
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.reasoning-selection",
        emitted_by: OPTIONAL,
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.maximum-output-tokens",
        emitted_by: &[],
        withheld_because: "the prepared exec request carries no output-token maximum",
    },
    LedgerEntry {
        semantic_id: "control.structured-output",
        emitted_by: OPTIONAL,
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.attachments",
        emitted_by: OPTIONAL,
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.external-network-policy",
        emitted_by: BOTH,
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.external-search-policy",
        emitted_by: BOTH,
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.model-verbosity",
        emitted_by: OPTIONAL,
        withheld_because: "",
    },
    LedgerEntry {
        semantic_id: "control.tool-declarations",
        emitted_by: &[],
        withheld_because: TOOLS_REJECTED,
    },
    LedgerEntry {
        semantic_id: "control.load-session",
        emitted_by: &[],
        withheld_because: "no public prepared exec session-load operation exists",
    },
    LedgerEntry {
        semantic_id: "control.resume-session",
        emitted_by: &[],
        withheld_because: "no public prepared exec session-resume operation exists",
    },
];

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
