/// Exact census route this tranche dispositions.
pub(super) const QODER_ROUTE: &str = "qoder.headless";

/// The one bounded print-run shape `qoder.headless` prepares.
pub(super) const RUN: &str = "QoderHeadlessPreparedRun[print-run]";

pub(super) const QODER_PROFILES: [&str; 1] = [RUN];
pub(super) const EVERY: &[&str] = &[RUN];

/// Exact census operation shapes the `qoder.headless` tranche spans.
pub(super) const QODER_OPERATION_SHAPES: [&str; 5] = [
    "model-catalogue",
    "structured-run",
    "route-observation",
    "route-capability",
    "route-selection",
];

const NO_CATALOGUE: &str = "the prepared headless run carries no catalogue observation; the matrix posture is \
     documentation, not runtime authority";
const NEGATIVE_COVERAGE: &str = "negative coverage: the audit records the absence of a route-specific selectable control, \
     so publishing one would falsify it";

/// One exact `qoder.headless` census row and its adapter disposition.
pub(super) struct LedgerEntry {
    pub(super) route_id: &'static str,
    pub(super) operation_shape: &'static str,
    pub(super) semantic_id: &'static str,
    pub(super) emitted_by: &'static [&'static str],
    pub(super) withheld_because: &'static str,
}

/// Deterministic disposition of exactly the eight `qoder.headless` census rows.
///
/// The ledger claims nothing about `deepagents.acp`, `kiro.acp`,
/// `zcode.app-server`, or any other route.
pub(super) const QODER_HEADLESS_TRANCHE: [LedgerEntry; 8] = [
    LedgerEntry {
        route_id: QODER_ROUTE,
        operation_shape: "model-catalogue",
        semantic_id: "feature.model-catalogue",
        emitted_by: &[],
        withheld_because: NO_CATALOGUE,
    },
    LedgerEntry {
        route_id: QODER_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "feature.structured-run",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: QODER_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.streaming-events",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: QODER_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.cancellation-or-interruption",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: QODER_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.working-resource",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: QODER_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.prepared-facade",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: QODER_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.activity-observation",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: QODER_ROUTE,
        operation_shape: "route-selection",
        semantic_id: "audit.no-public-route-specific-selectable-control",
        emitted_by: &[],
        withheld_because: NEGATIVE_COVERAGE,
    },
];

/// Identities with no `qoder.headless` census row at all.
///
/// A shared ACP or headless shape is not evidence: no `deepagents.acp`,
/// `kiro.acp`, or `zcode.app-server` identity may be constructed here.
pub(super) const WITHHELD_OFF_ROUTE: [&str; 6] = [
    "feature.interactive-session",
    "feature.persistent-session-posture",
    "feature.usage-evidence",
    "feature.owned-runtime-lifecycle",
    "control.model-selection",
    "control.app-server-mode",
];
