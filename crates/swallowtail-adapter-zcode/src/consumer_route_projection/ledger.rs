use super::ZCODE_APP_SERVER_ROUTE;

/// One prepared run bound to the exact read-only planning mode.
pub(super) const PLAN: &str = "ZcodePreparedRun[plan]";
/// One prepared run bound to the exact host-supplied build mode.
pub(super) const BUILD: &str = "ZcodePreparedRun[build]";

pub(super) const ZCODE_PROFILES: [&str; 2] = [PLAN, BUILD];
pub(super) const EVERY: &[&str] = &[PLAN, BUILD];

/// Exact census operation shapes the `zcode.app-server` tranche spans.
pub(super) const ZCODE_OPERATION_SHAPES: [&str; 5] = [
    "model-catalogue",
    "structured-run",
    "route-observation",
    "route-capability",
    "session-lifecycle",
];

const NO_CATALOGUE: &str = "the prepared app-server run carries no catalogue observation; the matrix posture is \
     documentation, not runtime authority";
const NO_PERSISTENCE: &str = "no prepared capability proves a route persistence posture; the matrix posture is \
     documentation, not runtime authority";

/// One exact `zcode.app-server` census row and its adapter disposition.
pub(super) struct LedgerEntry {
    pub(super) route_id: &'static str,
    pub(super) operation_shape: &'static str,
    pub(super) semantic_id: &'static str,
    pub(super) emitted_by: &'static [&'static str],
    pub(super) withheld_because: &'static str,
}

/// Deterministic disposition of exactly the 12 `zcode.app-server` census rows.
///
/// The ledger claims nothing about `deepagents.acp`, `kiro.acp`,
/// `qoder.headless`, or any other route.
pub(super) const ZCODE_APP_SERVER_TRANCHE: [LedgerEntry; 12] = [
    LedgerEntry {
        route_id: ZCODE_APP_SERVER_ROUTE,
        operation_shape: "model-catalogue",
        semantic_id: "feature.model-catalogue",
        emitted_by: &[],
        withheld_because: NO_CATALOGUE,
    },
    LedgerEntry {
        route_id: ZCODE_APP_SERVER_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "feature.structured-run",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ZCODE_APP_SERVER_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.streaming-events",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ZCODE_APP_SERVER_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.usage-evidence",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ZCODE_APP_SERVER_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.cancellation-or-interruption",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ZCODE_APP_SERVER_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.working-resource",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ZCODE_APP_SERVER_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.owned-runtime-lifecycle",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ZCODE_APP_SERVER_ROUTE,
        operation_shape: "session-lifecycle",
        semantic_id: "feature.persistent-session-posture",
        emitted_by: &[],
        withheld_because: NO_PERSISTENCE,
    },
    LedgerEntry {
        route_id: ZCODE_APP_SERVER_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.prepared-facade",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ZCODE_APP_SERVER_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.activity-observation",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ZCODE_APP_SERVER_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.model-selection",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ZCODE_APP_SERVER_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.app-server-mode",
        emitted_by: EVERY,
        withheld_because: "",
    },
];

/// Identities with no `zcode.app-server` census row at all.
///
/// A shared structured-run or ACP shape is not evidence: no `deepagents.acp`,
/// `kiro.acp`, or `qoder.headless` identity may be constructed here, and this
/// route carries no no-control audit to borrow either.
pub(super) const WITHHELD_OFF_ROUTE: [&str; 5] = [
    "feature.interactive-session",
    "feature.reasoning-selection",
    "feature.load-session",
    "control.reasoning-selection",
    "audit.no-public-route-specific-selectable-control",
];
