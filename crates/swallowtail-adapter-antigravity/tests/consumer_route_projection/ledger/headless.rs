use super::{
    ANTIGRAVITY_HEADLESS_ROUTE, LedgerEntry, PROFILE_CONTINUATION, PROFILE_HEADLESS_MAXIMAL,
    PROFILE_HEADLESS_MINIMAL, RowTuple,
};

pub const ANTIGRAVITY_HEADLESS_CENSUS_TUPLES: [RowTuple; 18] = [
    (
        ANTIGRAVITY_HEADLESS_ROUTE,
        "model-catalogue",
        "feature.model-catalogue",
    ),
    (
        ANTIGRAVITY_HEADLESS_ROUTE,
        "structured-run",
        "feature.structured-run",
    ),
    (
        ANTIGRAVITY_HEADLESS_ROUTE,
        "interactive-session",
        "feature.interactive-session",
    ),
    (
        ANTIGRAVITY_HEADLESS_ROUTE,
        "route-observation",
        "feature.streaming-events",
    ),
    (
        ANTIGRAVITY_HEADLESS_ROUTE,
        "route-observation",
        "feature.usage-evidence",
    ),
    (
        ANTIGRAVITY_HEADLESS_ROUTE,
        "route-capability",
        "feature.reasoning-selection",
    ),
    (
        ANTIGRAVITY_HEADLESS_ROUTE,
        "route-capability",
        "feature.structured-output",
    ),
    (
        ANTIGRAVITY_HEADLESS_ROUTE,
        "route-capability",
        "feature.cancellation-or-interruption",
    ),
    (
        ANTIGRAVITY_HEADLESS_ROUTE,
        "route-capability",
        "feature.working-resource",
    ),
    (
        ANTIGRAVITY_HEADLESS_ROUTE,
        "route-capability",
        "feature.bounded-workspace-text-write",
    ),
    (
        ANTIGRAVITY_HEADLESS_ROUTE,
        "session-lifecycle",
        "feature.persistent-session-posture",
    ),
    (
        ANTIGRAVITY_HEADLESS_ROUTE,
        "route-capability",
        "feature.prepared-facade",
    ),
    (
        ANTIGRAVITY_HEADLESS_ROUTE,
        "route-observation",
        "feature.activity-observation",
    ),
    (
        ANTIGRAVITY_HEADLESS_ROUTE,
        "structured-run",
        "control.model-selection",
    ),
    (
        ANTIGRAVITY_HEADLESS_ROUTE,
        "structured-run",
        "control.reasoning-selection",
    ),
    (
        ANTIGRAVITY_HEADLESS_ROUTE,
        "structured-run",
        "control.structured-output",
    ),
    (
        ANTIGRAVITY_HEADLESS_ROUTE,
        "structured-run",
        "control.resource-access",
    ),
    (
        ANTIGRAVITY_HEADLESS_ROUTE,
        "structured-run",
        "control.isolation",
    ),
];

pub const ANTIGRAVITY_HEADLESS_TRANCHE: [LedgerEntry; 18] = [
    LedgerEntry {
        route_id: ANTIGRAVITY_HEADLESS_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "feature.structured-run",
        emitted_by: &[PROFILE_HEADLESS_MAXIMAL, PROFILE_HEADLESS_MINIMAL],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_HEADLESS_ROUTE,
        operation_shape: "interactive-session",
        semantic_id: "feature.interactive-session",
        emitted_by: &[PROFILE_CONTINUATION],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_HEADLESS_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.prepared-facade",
        emitted_by: &[
            PROFILE_HEADLESS_MAXIMAL,
            PROFILE_HEADLESS_MINIMAL,
            PROFILE_CONTINUATION,
        ],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_HEADLESS_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.working-resource",
        emitted_by: &[
            PROFILE_HEADLESS_MAXIMAL,
            PROFILE_HEADLESS_MINIMAL,
            PROFILE_CONTINUATION,
        ],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_HEADLESS_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.cancellation-or-interruption",
        emitted_by: &[
            PROFILE_HEADLESS_MAXIMAL,
            PROFILE_HEADLESS_MINIMAL,
            PROFILE_CONTINUATION,
        ],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_HEADLESS_ROUTE,
        operation_shape: "session-lifecycle",
        semantic_id: "feature.persistent-session-posture",
        emitted_by: &[
            PROFILE_HEADLESS_MAXIMAL,
            PROFILE_HEADLESS_MINIMAL,
            PROFILE_CONTINUATION,
        ],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_HEADLESS_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.streaming-events",
        emitted_by: &[
            PROFILE_HEADLESS_MAXIMAL,
            PROFILE_HEADLESS_MINIMAL,
            PROFILE_CONTINUATION,
        ],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_HEADLESS_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.usage-evidence",
        emitted_by: &[
            PROFILE_HEADLESS_MAXIMAL,
            PROFILE_HEADLESS_MINIMAL,
            PROFILE_CONTINUATION,
        ],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_HEADLESS_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.activity-observation",
        emitted_by: &[
            PROFILE_HEADLESS_MAXIMAL,
            PROFILE_HEADLESS_MINIMAL,
            PROFILE_CONTINUATION,
        ],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_HEADLESS_ROUTE,
        operation_shape: "model-catalogue",
        semantic_id: "feature.model-catalogue",
        emitted_by: &[],
        withheld_because: "headless run carries no model catalogue",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_HEADLESS_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.model-selection",
        emitted_by: &[
            PROFILE_HEADLESS_MAXIMAL,
            PROFILE_HEADLESS_MINIMAL,
            PROFILE_CONTINUATION,
        ],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_HEADLESS_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.reasoning-selection",
        emitted_by: &[PROFILE_HEADLESS_MAXIMAL],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_HEADLESS_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.structured-output",
        emitted_by: &[PROFILE_HEADLESS_MAXIMAL],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_HEADLESS_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.reasoning-selection",
        emitted_by: &[PROFILE_HEADLESS_MAXIMAL],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_HEADLESS_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.structured-output",
        emitted_by: &[PROFILE_HEADLESS_MAXIMAL],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_HEADLESS_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.resource-access",
        emitted_by: &[PROFILE_HEADLESS_MAXIMAL, PROFILE_HEADLESS_MINIMAL],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_HEADLESS_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.isolation",
        emitted_by: &[PROFILE_HEADLESS_MAXIMAL, PROFILE_HEADLESS_MINIMAL],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ANTIGRAVITY_HEADLESS_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.bounded-workspace-text-write",
        emitted_by: &[],
        withheld_because: "matrix posture is documentation only; no capability in preflight plan",
    },
];
