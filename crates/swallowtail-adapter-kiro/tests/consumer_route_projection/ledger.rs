/// Exact census route this tranche dispositions.
pub(super) const KIRO_ROUTE: &str = "kiro.acp";

/// The one read-only session shape `kiro.acp` prepares.
pub(super) const SESSION: &str = "KiroPreparedSession[read-only]";

pub(super) const KIRO_PROFILES: [&str; 1] = [SESSION];
pub(super) const EVERY: &[&str] = &[SESSION];

/// Exact census operation shapes the `kiro.acp` tranche spans.
pub(super) const KIRO_OPERATION_SHAPES: [&str; 6] = [
    "model-catalogue",
    "interactive-session",
    "route-observation",
    "route-capability",
    "session-lifecycle",
    "route-selection",
];

const NO_CATALOGUE: &str = "the prepared ACP session carries no catalogue observation; the matrix posture is \
     documentation, not runtime authority";
const NO_PERSISTENCE: &str = "no prepared capability proves a route persistence posture; the matrix posture is \
     documentation, not runtime authority";
const NEGATIVE_COVERAGE: &str = "negative coverage: the audit records the absence of a route-specific selectable control, \
     so publishing one would falsify it";

/// One exact `kiro.acp` census row and its adapter disposition.
pub(super) struct LedgerEntry {
    pub(super) route_id: &'static str,
    pub(super) operation_shape: &'static str,
    pub(super) semantic_id: &'static str,
    pub(super) emitted_by: &'static [&'static str],
    pub(super) withheld_because: &'static str,
}

/// Deterministic disposition of exactly the nine `kiro.acp` census rows.
///
/// The ledger claims nothing about `deepagents.acp`, `qoder.headless`,
/// `zcode.app-server`, or any other route.
pub(super) const KIRO_ACP_TRANCHE: [LedgerEntry; 9] = [
    LedgerEntry {
        route_id: KIRO_ROUTE,
        operation_shape: "model-catalogue",
        semantic_id: "feature.model-catalogue",
        emitted_by: &[],
        withheld_because: NO_CATALOGUE,
    },
    LedgerEntry {
        route_id: KIRO_ROUTE,
        operation_shape: "interactive-session",
        semantic_id: "feature.interactive-session",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: KIRO_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.streaming-events",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: KIRO_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.cancellation-or-interruption",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: KIRO_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.working-resource",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: KIRO_ROUTE,
        operation_shape: "session-lifecycle",
        semantic_id: "feature.persistent-session-posture",
        emitted_by: &[],
        withheld_because: NO_PERSISTENCE,
    },
    LedgerEntry {
        route_id: KIRO_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.prepared-facade",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: KIRO_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.activity-observation",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: KIRO_ROUTE,
        operation_shape: "route-selection",
        semantic_id: "audit.no-public-route-specific-selectable-control",
        emitted_by: &[],
        withheld_because: NEGATIVE_COVERAGE,
    },
];

/// Identities with no `kiro.acp` census row at all.
///
/// A shared ACP or headless shape is not evidence: no `deepagents.acp`,
/// `qoder.headless`, or `zcode.app-server` identity may be constructed here.
pub(super) const WITHHELD_OFF_ROUTE: [&str; 6] = [
    "feature.structured-run",
    "feature.usage-evidence",
    "feature.owned-runtime-lifecycle",
    "feature.load-session",
    "control.model-selection",
    "control.app-server-mode",
];
