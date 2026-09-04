/// Exact census route this tranche dispositions.
pub(super) const OLLAMA_ROUTE: &str = "ollama.attached";

pub(super) const INVENTORY: &str = "OllamaPreparedInventory";
pub(super) const INFERENCE: &str = "OllamaPreparedInferenceAttempt";
pub(super) const INFERENCE_MAXIMAL: &str = "OllamaPreparedInferenceAttempt[maximal]";
pub(super) const SESSION: &str = "OllamaPreparedSession";

pub(super) const OLLAMA_PROFILES: [&str; 4] = [INVENTORY, INFERENCE, INFERENCE_MAXIMAL, SESSION];
pub(super) const INVENTORY_ONLY: &[&str] = &[INVENTORY];
pub(super) const INFERENCE_BOTH: &[&str] = &[INFERENCE, INFERENCE_MAXIMAL];
pub(super) const INFERENCE_MAX_ONLY: &[&str] = &[INFERENCE_MAXIMAL];
pub(super) const SESSION_ONLY: &[&str] = &[SESSION];
pub(super) const STREAMING: &[&str] = &[INFERENCE, INFERENCE_MAXIMAL, SESSION];
pub(super) const EVERY_FACADE: &[&str] = &[INVENTORY, INFERENCE, INFERENCE_MAXIMAL, SESSION];

pub(super) const OLLAMA_SHAPES: [&str; 5] = [
    "model-catalogue",
    "structured-run",
    "interactive-session",
    "route-observation",
    "route-capability",
];

const NO_SESSION_REASONING: &str = "OllamaSessionProfileInput carries no reasoning field and prepare_session accepts no reasoning value; the census retains the matrix descriptor only";

/// One exact `ollama.attached` census row and its adapter disposition.
pub(super) struct LedgerEntry {
    pub(super) route_id: &'static str,
    pub(super) operation_shape: &'static str,
    pub(super) semantic_id: &'static str,
    pub(super) emitted_by: &'static [&'static str],
    pub(super) withheld_because: &'static str,
}

/// Deterministic disposition of exactly the nineteen `ollama.attached` census rows.
///
/// Twin rows share a semantic id and are distinct by operation shape.
pub(super) const OLLAMA_ATTACHED_TRANCHE: [LedgerEntry; 19] = [
    LedgerEntry {
        route_id: OLLAMA_ROUTE,
        operation_shape: "model-catalogue",
        semantic_id: "feature.model-catalogue",
        emitted_by: INVENTORY_ONLY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: OLLAMA_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "feature.structured-run",
        emitted_by: INFERENCE_BOTH,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: OLLAMA_ROUTE,
        operation_shape: "interactive-session",
        semantic_id: "feature.interactive-session",
        emitted_by: SESSION_ONLY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: OLLAMA_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.streaming-events",
        emitted_by: STREAMING,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: OLLAMA_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.usage-evidence",
        emitted_by: STREAMING,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: OLLAMA_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.output-token-limit",
        emitted_by: STREAMING,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: OLLAMA_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.reasoning-selection",
        emitted_by: INFERENCE_MAX_ONLY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: OLLAMA_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.structured-output",
        emitted_by: INFERENCE_MAX_ONLY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: OLLAMA_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.cancellation-or-interruption",
        emitted_by: SESSION_ONLY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: OLLAMA_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.prepared-facade",
        emitted_by: EVERY_FACADE,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: OLLAMA_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.activity-observation",
        emitted_by: STREAMING,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: OLLAMA_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.model-selection",
        emitted_by: INFERENCE_BOTH,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: OLLAMA_ROUTE,
        operation_shape: "interactive-session",
        semantic_id: "control.model-selection",
        emitted_by: SESSION_ONLY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: OLLAMA_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.reasoning-selection",
        emitted_by: INFERENCE_MAX_ONLY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: OLLAMA_ROUTE,
        operation_shape: "interactive-session",
        semantic_id: "control.reasoning-selection",
        emitted_by: &[],
        withheld_because: NO_SESSION_REASONING,
    },
    LedgerEntry {
        route_id: OLLAMA_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.maximum-output-tokens",
        emitted_by: INFERENCE_BOTH,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: OLLAMA_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.structured-output",
        emitted_by: INFERENCE_MAX_ONLY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: OLLAMA_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.context-window",
        emitted_by: INFERENCE_MAX_ONLY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: OLLAMA_ROUTE,
        operation_shape: "interactive-session",
        semantic_id: "control.context-window",
        emitted_by: SESSION_ONLY,
        withheld_because: "",
    },
];

pub(super) const WITHHELD_OFF_ROUTE: [&str; 6] = [
    "feature.owned-runtime-lifecycle",
    "feature.working-resource",
    "control.serving-model-artifact",
    "control.serving-context-size",
    "control.serving-reasoning",
    "control.app-server-mode",
];
