/// Exact census route for the attached llama.cpp tranche.
pub(super) const ATTACHED_ROUTE: &str = "llama-cpp.attached";
/// Exact census route for the owned llama.cpp tranche.
pub(super) const OWNED_ROUTE: &str = "llama-cpp.owned";

pub(super) const CATALOGUE: &str = "LlamaCppPreparedCatalogue";
pub(super) const INFERENCE: &str = "LlamaCppPreparedInferenceAttempt";
pub(super) const SERVING: &str = "LlamaCppPreparedServingStart";

pub(super) const ATTACHED_PROFILES: [&str; 2] = [CATALOGUE, INFERENCE];
pub(super) const OWNED_PROFILES: [&str; 1] = [SERVING];
pub(super) const ATTACHED_EVERY: &[&str] = &[CATALOGUE, INFERENCE];
pub(super) const INFERENCE_ONLY: &[&str] = &[INFERENCE];
pub(super) const CATALOGUE_ONLY: &[&str] = &[CATALOGUE];
pub(super) const SERVING_EVERY: &[&str] = &[SERVING];

pub(super) const ATTACHED_SHAPES: [&str; 4] = [
    "model-catalogue",
    "structured-run",
    "route-observation",
    "route-capability",
];
pub(super) const OWNED_SHAPES: [&str; 3] =
    ["route-capability", "route-observation", "session-lifecycle"];

const NO_ATTACHED_INTERRUPTION: &str = "no attached prepared plan requires Interruption; driver-level cancellation is post-dispatch and not prepared evidence";
const NO_OWNED_ACTIVITY: &str = "owned prepared evidence retains no activity profile; the crate's only ObservableActivityProfile binds the attached runtime";

/// One exact census row and its adapter disposition.
pub(super) struct LedgerEntry {
    pub(super) route_id: &'static str,
    pub(super) operation_shape: &'static str,
    pub(super) semantic_id: &'static str,
    pub(super) emitted_by: &'static [&'static str],
    pub(super) withheld_because: &'static str,
}

/// Deterministic disposition of exactly the ten `llama-cpp.attached` census rows.
pub(super) const LLAMA_CPP_ATTACHED_TRANCHE: [LedgerEntry; 10] = [
    LedgerEntry {
        route_id: ATTACHED_ROUTE,
        operation_shape: "model-catalogue",
        semantic_id: "feature.model-catalogue",
        emitted_by: CATALOGUE_ONLY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ATTACHED_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "feature.structured-run",
        emitted_by: INFERENCE_ONLY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ATTACHED_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.streaming-events",
        emitted_by: INFERENCE_ONLY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ATTACHED_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.usage-evidence",
        emitted_by: INFERENCE_ONLY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ATTACHED_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.output-token-limit",
        emitted_by: INFERENCE_ONLY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ATTACHED_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.cancellation-or-interruption",
        emitted_by: &[],
        withheld_because: NO_ATTACHED_INTERRUPTION,
    },
    LedgerEntry {
        route_id: ATTACHED_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.prepared-facade",
        emitted_by: ATTACHED_EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ATTACHED_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.activity-observation",
        emitted_by: INFERENCE_ONLY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ATTACHED_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.model-selection",
        emitted_by: INFERENCE_ONLY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: ATTACHED_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.maximum-output-tokens",
        emitted_by: INFERENCE_ONLY,
        withheld_because: "",
    },
];

/// Deterministic disposition of exactly the six `llama-cpp.owned` census rows.
pub(super) const LLAMA_CPP_OWNED_TRANCHE: [LedgerEntry; 6] = [
    LedgerEntry {
        route_id: OWNED_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.owned-runtime-lifecycle",
        emitted_by: SERVING_EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: OWNED_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.prepared-facade",
        emitted_by: SERVING_EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: OWNED_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.activity-observation",
        emitted_by: &[],
        withheld_because: NO_OWNED_ACTIVITY,
    },
    LedgerEntry {
        route_id: OWNED_ROUTE,
        operation_shape: "session-lifecycle",
        semantic_id: "control.serving-model-artifact",
        emitted_by: SERVING_EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: OWNED_ROUTE,
        operation_shape: "session-lifecycle",
        semantic_id: "control.serving-context-size",
        emitted_by: SERVING_EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: OWNED_ROUTE,
        operation_shape: "session-lifecycle",
        semantic_id: "control.serving-reasoning",
        emitted_by: SERVING_EVERY,
        withheld_because: "",
    },
];

pub(super) const ATTACHED_OFF_ROUTE: [&str; 8] = [
    "feature.interactive-session",
    "feature.owned-runtime-lifecycle",
    "feature.reasoning-selection",
    "feature.structured-output",
    "control.serving-model-artifact",
    "control.serving-context-size",
    "control.serving-reasoning",
    "control.reasoning-selection",
];

pub(super) const OWNED_OFF_ROUTE: [&str; 8] = [
    "feature.model-catalogue",
    "feature.structured-run",
    "feature.streaming-events",
    "feature.usage-evidence",
    "feature.output-token-limit",
    "feature.cancellation-or-interruption",
    "feature.interactive-session",
    "control.model-selection",
];
