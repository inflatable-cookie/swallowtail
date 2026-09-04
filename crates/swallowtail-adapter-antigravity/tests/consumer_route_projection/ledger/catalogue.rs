use super::{ANTIGRAVITY_CATALOGUE_ROUTE, LedgerEntry, PROFILE_CATALOGUE, RowTuple};

pub const ANTIGRAVITY_CATALOGUE_CENSUS_TUPLES: [RowTuple; 14] = [
    (
        ANTIGRAVITY_CATALOGUE_ROUTE,
        "model-catalogue",
        "feature.model-catalogue",
    ),
    (
        ANTIGRAVITY_CATALOGUE_ROUTE,
        "structured-run",
        "feature.structured-run",
    ),
    (
        ANTIGRAVITY_CATALOGUE_ROUTE,
        "interactive-session",
        "feature.interactive-session",
    ),
    (
        ANTIGRAVITY_CATALOGUE_ROUTE,
        "route-observation",
        "feature.streaming-events",
    ),
    (
        ANTIGRAVITY_CATALOGUE_ROUTE,
        "route-observation",
        "feature.usage-evidence",
    ),
    (
        ANTIGRAVITY_CATALOGUE_ROUTE,
        "route-capability",
        "feature.reasoning-selection",
    ),
    (
        ANTIGRAVITY_CATALOGUE_ROUTE,
        "route-capability",
        "feature.structured-output",
    ),
    (
        ANTIGRAVITY_CATALOGUE_ROUTE,
        "route-capability",
        "feature.cancellation-or-interruption",
    ),
    (
        ANTIGRAVITY_CATALOGUE_ROUTE,
        "route-capability",
        "feature.working-resource",
    ),
    (
        ANTIGRAVITY_CATALOGUE_ROUTE,
        "route-capability",
        "feature.bounded-workspace-text-write",
    ),
    (
        ANTIGRAVITY_CATALOGUE_ROUTE,
        "session-lifecycle",
        "feature.persistent-session-posture",
    ),
    (
        ANTIGRAVITY_CATALOGUE_ROUTE,
        "route-capability",
        "feature.prepared-facade",
    ),
    (
        ANTIGRAVITY_CATALOGUE_ROUTE,
        "route-observation",
        "feature.activity-observation",
    ),
    (
        ANTIGRAVITY_CATALOGUE_ROUTE,
        "route-selection",
        "audit.no-public-route-specific-selectable-control",
    ),
];

pub const ANTIGRAVITY_CATALOGUE_TRANCHE: [LedgerEntry; 14] = [
    LedgerEntry {
        route_id: ANTIGRAVITY_CATALOGUE_ROUTE,
        operation_shape: "model-catalogue",
        semantic_id: "feature.model-catalogue",
        emitted_by: &[PROFILE_CATALOGUE],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_CATALOGUE_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "feature.structured-run",
        emitted_by: &[],
        withheld_because: "operation role is DriverRole::ModelCatalog, not consumer structured run execution",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_CATALOGUE_ROUTE,
        operation_shape: "interactive-session",
        semantic_id: "feature.interactive-session",
        emitted_by: &[],
        withheld_because: "operation shape is StructuredRun/ModelCatalog, not an interactive session",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_CATALOGUE_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.streaming-events",
        emitted_by: &[],
        withheld_because: "matrix posture is documentation only; catalogue preflight plan carries no feature.streaming-events capability",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_CATALOGUE_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.usage-evidence",
        emitted_by: &[],
        withheld_because: "matrix posture is documentation only; catalogue preflight plan carries no feature.usage-evidence capability",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_CATALOGUE_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.reasoning-selection",
        emitted_by: &[],
        withheld_because: "matrix posture is documentation only; catalogue preflight plan carries no feature.reasoning-selection capability",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_CATALOGUE_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.structured-output",
        emitted_by: &[],
        withheld_because: "matrix posture is documentation only; catalogue preflight plan carries no feature.structured-output capability",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_CATALOGUE_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.cancellation-or-interruption",
        emitted_by: &[],
        withheld_because: "matrix posture is documentation only; catalogue preflight plan carries no feature.cancellation-or-interruption capability",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_CATALOGUE_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.working-resource",
        emitted_by: &[],
        withheld_because: "matrix posture is documentation only; catalogue preflight plan carries no feature.working-resource capability",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_CATALOGUE_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.bounded-workspace-text-write",
        emitted_by: &[],
        withheld_because: "matrix posture is documentation only; catalogue preflight plan carries no feature.bounded-workspace-text-write capability",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_CATALOGUE_ROUTE,
        operation_shape: "session-lifecycle",
        semantic_id: "feature.persistent-session-posture",
        emitted_by: &[],
        withheld_because: "matrix posture is documentation only; catalogue preflight plan carries no feature.persistent-session-posture capability",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_CATALOGUE_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.prepared-facade",
        emitted_by: &[PROFILE_CATALOGUE],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_CATALOGUE_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.activity-observation",
        emitted_by: &[],
        withheld_because: "catalogue preflight plan carries no ObservableActivityProfile",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_CATALOGUE_ROUTE,
        operation_shape: "route-selection",
        semantic_id: "audit.no-public-route-specific-selectable-control",
        emitted_by: &[],
        withheld_because: "negative coverage: audit records absence of route-specific selectable controls; withheld from public control descriptors",
    },
];
