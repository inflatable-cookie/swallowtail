use super::{CURSOR_CATALOGUE_ROUTE, LedgerEntry, PROFILE_CATALOGUE, RowTuple};

pub const CURSOR_CATALOGUE_CENSUS_TUPLES: [RowTuple; 13] = [
    (
        CURSOR_CATALOGUE_ROUTE,
        "model-catalogue",
        "feature.model-catalogue",
    ),
    (
        CURSOR_CATALOGUE_ROUTE,
        "structured-run",
        "feature.structured-run",
    ),
    (
        CURSOR_CATALOGUE_ROUTE,
        "interactive-session",
        "feature.interactive-session",
    ),
    (
        CURSOR_CATALOGUE_ROUTE,
        "route-observation",
        "feature.streaming-events",
    ),
    (
        CURSOR_CATALOGUE_ROUTE,
        "route-observation",
        "feature.usage-evidence",
    ),
    (
        CURSOR_CATALOGUE_ROUTE,
        "route-capability",
        "feature.reasoning-selection",
    ),
    (
        CURSOR_CATALOGUE_ROUTE,
        "route-capability",
        "feature.cancellation-or-interruption",
    ),
    (
        CURSOR_CATALOGUE_ROUTE,
        "route-capability",
        "feature.working-resource",
    ),
    (
        CURSOR_CATALOGUE_ROUTE,
        "route-capability",
        "feature.bounded-workspace-text-write",
    ),
    (
        CURSOR_CATALOGUE_ROUTE,
        "session-lifecycle",
        "feature.persistent-session-posture",
    ),
    (
        CURSOR_CATALOGUE_ROUTE,
        "route-capability",
        "feature.prepared-facade",
    ),
    (
        CURSOR_CATALOGUE_ROUTE,
        "route-observation",
        "feature.activity-observation",
    ),
    (
        CURSOR_CATALOGUE_ROUTE,
        "route-selection",
        "audit.no-public-route-specific-selectable-control",
    ),
];

pub const CURSOR_CATALOGUE_TRANCHE: [LedgerEntry; 13] = [
    LedgerEntry {
        route_id: CURSOR_CATALOGUE_ROUTE,
        operation_shape: "model-catalogue",
        semantic_id: "feature.model-catalogue",
        emitted_by: &[PROFILE_CATALOGUE],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: CURSOR_CATALOGUE_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "feature.structured-run",
        emitted_by: &[],
        withheld_because: "operation role is ModelCatalog, not structured run",
    },
    LedgerEntry {
        route_id: CURSOR_CATALOGUE_ROUTE,
        operation_shape: "interactive-session",
        semantic_id: "feature.interactive-session",
        emitted_by: &[],
        withheld_because: "operation shape is ModelCatalog, not interactive session",
    },
    LedgerEntry {
        route_id: CURSOR_CATALOGUE_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.streaming-events",
        emitted_by: &[],
        withheld_because: "catalogue carries no streaming events capability",
    },
    LedgerEntry {
        route_id: CURSOR_CATALOGUE_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.usage-evidence",
        emitted_by: &[],
        withheld_because: "catalogue carries no usage reporting capability",
    },
    LedgerEntry {
        route_id: CURSOR_CATALOGUE_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.reasoning-selection",
        emitted_by: &[],
        withheld_because: "catalogue carries no reasoning capability",
    },
    LedgerEntry {
        route_id: CURSOR_CATALOGUE_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.cancellation-or-interruption",
        emitted_by: &[],
        withheld_because: "catalogue carries no cancellation capability",
    },
    LedgerEntry {
        route_id: CURSOR_CATALOGUE_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.working-resource",
        emitted_by: &[],
        withheld_because: "catalogue carries no working resource capability",
    },
    LedgerEntry {
        route_id: CURSOR_CATALOGUE_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.bounded-workspace-text-write",
        emitted_by: &[],
        withheld_because: "catalogue carries no workspace write capability",
    },
    LedgerEntry {
        route_id: CURSOR_CATALOGUE_ROUTE,
        operation_shape: "session-lifecycle",
        semantic_id: "feature.persistent-session-posture",
        emitted_by: &[],
        withheld_because: "catalogue carries no durable retention capability",
    },
    LedgerEntry {
        route_id: CURSOR_CATALOGUE_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.prepared-facade",
        emitted_by: &[PROFILE_CATALOGUE],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: CURSOR_CATALOGUE_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.activity-observation",
        emitted_by: &[],
        withheld_because: "catalogue carries no observable activity profile",
    },
    LedgerEntry {
        route_id: CURSOR_CATALOGUE_ROUTE,
        operation_shape: "route-selection",
        semantic_id: "audit.no-public-route-specific-selectable-control",
        emitted_by: &[],
        withheld_because: "negative coverage: no public selectable control",
    },
];
