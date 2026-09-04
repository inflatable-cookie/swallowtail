//! Ledger of all 43 Cursor census rows (13 ACP, 13 catalogue, 17 headless).

pub const CURSOR_ACP_ROUTE: &str = "cursor-agent.acp";
pub const CURSOR_CATALOGUE_ROUTE: &str = "cursor-agent.catalogue";
pub const CURSOR_HEADLESS_ROUTE: &str = "cursor-agent.headless";

pub const PROFILE_ACP: &str = "CursorPreparedAcpSession";
pub const PROFILE_CATALOGUE: &str = "CursorPreparedCatalogue";
pub const PROFILE_HEADLESS_MAXIMAL: &str = "CursorPreparedHeadlessRun[maximal]";
pub const PROFILE_HEADLESS_MINIMAL: &str = "CursorPreparedHeadlessRun[minimal]";

pub type RowTuple = (&'static str, &'static str, &'static str);

pub struct LedgerEntry {
    pub route_id: &'static str,
    pub operation_shape: &'static str,
    pub semantic_id: &'static str,
    pub emitted_by: &'static [&'static str],
    pub withheld_because: &'static str,
}

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

pub const CURSOR_HEADLESS_CENSUS_TUPLES: [RowTuple; 17] = [
    (
        CURSOR_HEADLESS_ROUTE,
        "model-catalogue",
        "feature.model-catalogue",
    ),
    (
        CURSOR_HEADLESS_ROUTE,
        "structured-run",
        "feature.structured-run",
    ),
    (
        CURSOR_HEADLESS_ROUTE,
        "interactive-session",
        "feature.interactive-session",
    ),
    (
        CURSOR_HEADLESS_ROUTE,
        "route-observation",
        "feature.streaming-events",
    ),
    (
        CURSOR_HEADLESS_ROUTE,
        "route-observation",
        "feature.usage-evidence",
    ),
    (
        CURSOR_HEADLESS_ROUTE,
        "route-capability",
        "feature.reasoning-selection",
    ),
    (
        CURSOR_HEADLESS_ROUTE,
        "route-capability",
        "feature.cancellation-or-interruption",
    ),
    (
        CURSOR_HEADLESS_ROUTE,
        "route-capability",
        "feature.working-resource",
    ),
    (
        CURSOR_HEADLESS_ROUTE,
        "route-capability",
        "feature.bounded-workspace-text-write",
    ),
    (
        CURSOR_HEADLESS_ROUTE,
        "session-lifecycle",
        "feature.persistent-session-posture",
    ),
    (
        CURSOR_HEADLESS_ROUTE,
        "route-capability",
        "feature.prepared-facade",
    ),
    (
        CURSOR_HEADLESS_ROUTE,
        "route-observation",
        "feature.activity-observation",
    ),
    (
        CURSOR_HEADLESS_ROUTE,
        "structured-run",
        "control.model-selection",
    ),
    (CURSOR_HEADLESS_ROUTE, "structured-run", "control.fast"),
    (
        CURSOR_HEADLESS_ROUTE,
        "structured-run",
        "control.context-window",
    ),
    (
        CURSOR_HEADLESS_ROUTE,
        "structured-run",
        "control.reasoning-effort",
    ),
    (CURSOR_HEADLESS_ROUTE, "structured-run", "control.read-mode"),
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
        withheld_because: "matrix posture is documentation only; no capability in preflight plan",
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

pub const CURSOR_HEADLESS_TRANCHE: [LedgerEntry; 17] = [
    LedgerEntry {
        route_id: CURSOR_HEADLESS_ROUTE,
        operation_shape: "model-catalogue",
        semantic_id: "feature.model-catalogue",
        emitted_by: &[],
        withheld_because: "headless run carries no model catalogue capability",
    },
    LedgerEntry {
        route_id: CURSOR_HEADLESS_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "feature.structured-run",
        emitted_by: &[PROFILE_HEADLESS_MAXIMAL, PROFILE_HEADLESS_MINIMAL],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: CURSOR_HEADLESS_ROUTE,
        operation_shape: "interactive-session",
        semantic_id: "feature.interactive-session",
        emitted_by: &[],
        withheld_because: "operation shape is StructuredRun, not interactive session",
    },
    LedgerEntry {
        route_id: CURSOR_HEADLESS_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.streaming-events",
        emitted_by: &[PROFILE_HEADLESS_MAXIMAL, PROFILE_HEADLESS_MINIMAL],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: CURSOR_HEADLESS_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.usage-evidence",
        emitted_by: &[PROFILE_HEADLESS_MAXIMAL, PROFILE_HEADLESS_MINIMAL],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: CURSOR_HEADLESS_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.reasoning-selection",
        emitted_by: &[PROFILE_HEADLESS_MAXIMAL],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: CURSOR_HEADLESS_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.cancellation-or-interruption",
        emitted_by: &[PROFILE_HEADLESS_MAXIMAL, PROFILE_HEADLESS_MINIMAL],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: CURSOR_HEADLESS_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.working-resource",
        emitted_by: &[PROFILE_HEADLESS_MAXIMAL, PROFILE_HEADLESS_MINIMAL],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: CURSOR_HEADLESS_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.bounded-workspace-text-write",
        emitted_by: &[],
        withheld_because: "matrix posture is documentation only; no capability in preflight plan",
    },
    LedgerEntry {
        route_id: CURSOR_HEADLESS_ROUTE,
        operation_shape: "session-lifecycle",
        semantic_id: "feature.persistent-session-posture",
        emitted_by: &[PROFILE_HEADLESS_MAXIMAL, PROFILE_HEADLESS_MINIMAL],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: CURSOR_HEADLESS_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.prepared-facade",
        emitted_by: &[PROFILE_HEADLESS_MAXIMAL, PROFILE_HEADLESS_MINIMAL],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: CURSOR_HEADLESS_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.activity-observation",
        emitted_by: &[PROFILE_HEADLESS_MAXIMAL, PROFILE_HEADLESS_MINIMAL],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: CURSOR_HEADLESS_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.model-selection",
        emitted_by: &[PROFILE_HEADLESS_MAXIMAL, PROFILE_HEADLESS_MINIMAL],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: CURSOR_HEADLESS_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.fast",
        emitted_by: &[PROFILE_HEADLESS_MAXIMAL],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: CURSOR_HEADLESS_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.context-window",
        emitted_by: &[PROFILE_HEADLESS_MAXIMAL],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: CURSOR_HEADLESS_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.reasoning-effort",
        emitted_by: &[PROFILE_HEADLESS_MAXIMAL],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: CURSOR_HEADLESS_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.read-mode",
        emitted_by: &[PROFILE_HEADLESS_MAXIMAL],
        withheld_because: "",
    },
];
