use super::{CURSOR_ACP_ROUTE, LedgerEntry, PROFILE_ACP, RowTuple};

pub const CURSOR_ACP_CENSUS_TUPLES: [RowTuple; 13] = [
    (
        CURSOR_ACP_ROUTE,
        "model-catalogue",
        "feature.model-catalogue",
    ),
    (CURSOR_ACP_ROUTE, "structured-run", "feature.structured-run"),
    (
        CURSOR_ACP_ROUTE,
        "interactive-session",
        "feature.interactive-session",
    ),
    (
        CURSOR_ACP_ROUTE,
        "route-observation",
        "feature.streaming-events",
    ),
    (
        CURSOR_ACP_ROUTE,
        "route-observation",
        "feature.usage-evidence",
    ),
    (
        CURSOR_ACP_ROUTE,
        "route-capability",
        "feature.reasoning-selection",
    ),
    (
        CURSOR_ACP_ROUTE,
        "route-capability",
        "feature.cancellation-or-interruption",
    ),
    (
        CURSOR_ACP_ROUTE,
        "route-capability",
        "feature.working-resource",
    ),
    (
        CURSOR_ACP_ROUTE,
        "route-capability",
        "feature.bounded-workspace-text-write",
    ),
    (
        CURSOR_ACP_ROUTE,
        "session-lifecycle",
        "feature.persistent-session-posture",
    ),
    (
        CURSOR_ACP_ROUTE,
        "route-capability",
        "feature.prepared-facade",
    ),
    (
        CURSOR_ACP_ROUTE,
        "route-observation",
        "feature.activity-observation",
    ),
    (
        CURSOR_ACP_ROUTE,
        "route-selection",
        "audit.no-public-route-specific-selectable-control",
    ),
];

pub const CURSOR_ACP_TRANCHE: [LedgerEntry; 13] = [
    LedgerEntry {
        route_id: CURSOR_ACP_ROUTE,
        operation_shape: "model-catalogue",
        semantic_id: "feature.model-catalogue",
        emitted_by: &[],
        withheld_because: "ACP carries no model catalogue capability",
    },
    LedgerEntry {
        route_id: CURSOR_ACP_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "feature.structured-run",
        emitted_by: &[],
        withheld_because: "operation shape is InteractiveSession, not structured run",
    },
    LedgerEntry {
        route_id: CURSOR_ACP_ROUTE,
        operation_shape: "interactive-session",
        semantic_id: "feature.interactive-session",
        emitted_by: &[PROFILE_ACP],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: CURSOR_ACP_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.streaming-events",
        emitted_by: &[PROFILE_ACP],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: CURSOR_ACP_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.usage-evidence",
        emitted_by: &[],
        withheld_because: "matrix posture is documentation only; no capability in preflight plan",
    },
    LedgerEntry {
        route_id: CURSOR_ACP_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.reasoning-selection",
        emitted_by: &[],
        withheld_because: "ACP carries no reasoning selection capability",
    },
    LedgerEntry {
        route_id: CURSOR_ACP_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.cancellation-or-interruption",
        emitted_by: &[PROFILE_ACP],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: CURSOR_ACP_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.working-resource",
        emitted_by: &[PROFILE_ACP],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: CURSOR_ACP_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.bounded-workspace-text-write",
        emitted_by: &[],
        withheld_because: "matrix posture is documentation only; no capability in preflight plan",
    },
    LedgerEntry {
        route_id: CURSOR_ACP_ROUTE,
        operation_shape: "session-lifecycle",
        semantic_id: "feature.persistent-session-posture",
        emitted_by: &[PROFILE_ACP],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: CURSOR_ACP_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.prepared-facade",
        emitted_by: &[PROFILE_ACP],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: CURSOR_ACP_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.activity-observation",
        emitted_by: &[PROFILE_ACP],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: CURSOR_ACP_ROUTE,
        operation_shape: "route-selection",
        semantic_id: "audit.no-public-route-specific-selectable-control",
        emitted_by: &[],
        withheld_because: "negative coverage: no public selectable control",
    },
];
