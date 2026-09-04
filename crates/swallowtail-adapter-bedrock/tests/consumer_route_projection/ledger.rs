//! Ledger of all 19 Amazon Bedrock census rows (9 catalogue, 10 runtime).

pub const BEDROCK_CATALOGUE_ROUTE: &str = "bedrock.catalogue";
pub const BEDROCK_RUNTIME_ROUTE: &str = "bedrock.runtime";

pub const PROFILE_CATALOGUE: &str = "BedrockPreparedCatalogue";
pub const PROFILE_RUNTIME: &str = "BedrockPreparedInferenceAttempt";

pub type RowTuple = (&'static str, &'static str, &'static str);

pub struct LedgerEntry {
    pub route_id: &'static str,
    pub operation_shape: &'static str,
    pub semantic_id: &'static str,
    pub emitted_by: &'static [&'static str],
    pub withheld_because: &'static str,
}

pub const BEDROCK_CATALOGUE_CENSUS_TUPLES: [RowTuple; 9] = [
    (
        BEDROCK_CATALOGUE_ROUTE,
        "model-catalogue",
        "feature.model-catalogue",
    ),
    (
        BEDROCK_CATALOGUE_ROUTE,
        "structured-run",
        "feature.structured-run",
    ),
    (
        BEDROCK_CATALOGUE_ROUTE,
        "route-observation",
        "feature.streaming-events",
    ),
    (
        BEDROCK_CATALOGUE_ROUTE,
        "route-observation",
        "feature.usage-evidence",
    ),
    (
        BEDROCK_CATALOGUE_ROUTE,
        "route-capability",
        "feature.output-token-limit",
    ),
    (
        BEDROCK_CATALOGUE_ROUTE,
        "route-capability",
        "feature.cancellation-or-interruption",
    ),
    (
        BEDROCK_CATALOGUE_ROUTE,
        "route-capability",
        "feature.prepared-facade",
    ),
    (
        BEDROCK_CATALOGUE_ROUTE,
        "route-observation",
        "feature.activity-observation",
    ),
    (
        BEDROCK_CATALOGUE_ROUTE,
        "route-selection",
        "audit.no-public-route-specific-selectable-control",
    ),
];

pub const BEDROCK_RUNTIME_CENSUS_TUPLES: [RowTuple; 10] = [
    (
        BEDROCK_RUNTIME_ROUTE,
        "model-catalogue",
        "feature.model-catalogue",
    ),
    (
        BEDROCK_RUNTIME_ROUTE,
        "structured-run",
        "feature.structured-run",
    ),
    (
        BEDROCK_RUNTIME_ROUTE,
        "route-observation",
        "feature.streaming-events",
    ),
    (
        BEDROCK_RUNTIME_ROUTE,
        "route-observation",
        "feature.usage-evidence",
    ),
    (
        BEDROCK_RUNTIME_ROUTE,
        "route-capability",
        "feature.output-token-limit",
    ),
    (
        BEDROCK_RUNTIME_ROUTE,
        "route-capability",
        "feature.cancellation-or-interruption",
    ),
    (
        BEDROCK_RUNTIME_ROUTE,
        "route-capability",
        "feature.prepared-facade",
    ),
    (
        BEDROCK_RUNTIME_ROUTE,
        "route-observation",
        "feature.activity-observation",
    ),
    (
        BEDROCK_RUNTIME_ROUTE,
        "structured-run",
        "control.model-selection",
    ),
    (
        BEDROCK_RUNTIME_ROUTE,
        "structured-run",
        "control.maximum-output-tokens",
    ),
];

pub const BEDROCK_CATALOGUE_TRANCHE: [LedgerEntry; 9] = [
    LedgerEntry {
        route_id: BEDROCK_CATALOGUE_ROUTE,
        operation_shape: "model-catalogue",
        semantic_id: "feature.model-catalogue",
        emitted_by: &[PROFILE_CATALOGUE],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BEDROCK_CATALOGUE_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "feature.structured-run",
        emitted_by: &[],
        withheld_because: "operation role is ModelCatalog, not structured run",
    },
    LedgerEntry {
        route_id: BEDROCK_CATALOGUE_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.streaming-events",
        emitted_by: &[],
        withheld_because: "catalogue carries no streaming events capability",
    },
    LedgerEntry {
        route_id: BEDROCK_CATALOGUE_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.usage-evidence",
        emitted_by: &[],
        withheld_because: "catalogue carries no usage reporting capability",
    },
    LedgerEntry {
        route_id: BEDROCK_CATALOGUE_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.output-token-limit",
        emitted_by: &[],
        withheld_because: "catalogue carries no output token limit capability",
    },
    LedgerEntry {
        route_id: BEDROCK_CATALOGUE_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.cancellation-or-interruption",
        emitted_by: &[],
        withheld_because: "catalogue carries no cancellation capability",
    },
    LedgerEntry {
        route_id: BEDROCK_CATALOGUE_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.prepared-facade",
        emitted_by: &[PROFILE_CATALOGUE],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BEDROCK_CATALOGUE_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.activity-observation",
        emitted_by: &[],
        withheld_because: "catalogue carries no observable activity profile",
    },
    LedgerEntry {
        route_id: BEDROCK_CATALOGUE_ROUTE,
        operation_shape: "route-selection",
        semantic_id: "audit.no-public-route-specific-selectable-control",
        emitted_by: &[],
        withheld_because: "negative coverage: no public selectable control",
    },
];

pub const BEDROCK_RUNTIME_TRANCHE: [LedgerEntry; 10] = [
    LedgerEntry {
        route_id: BEDROCK_RUNTIME_ROUTE,
        operation_shape: "model-catalogue",
        semantic_id: "feature.model-catalogue",
        emitted_by: &[],
        withheld_because: "runtime inference carries no ModelCatalog capability",
    },
    LedgerEntry {
        route_id: BEDROCK_RUNTIME_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "feature.structured-run",
        emitted_by: &[PROFILE_RUNTIME],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BEDROCK_RUNTIME_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.streaming-events",
        emitted_by: &[PROFILE_RUNTIME],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BEDROCK_RUNTIME_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.usage-evidence",
        emitted_by: &[PROFILE_RUNTIME],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BEDROCK_RUNTIME_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.output-token-limit",
        emitted_by: &[PROFILE_RUNTIME],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BEDROCK_RUNTIME_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.cancellation-or-interruption",
        emitted_by: &[],
        withheld_because: "matrix posture is partial; preflight plan carries no Interruption",
    },
    LedgerEntry {
        route_id: BEDROCK_RUNTIME_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.prepared-facade",
        emitted_by: &[PROFILE_RUNTIME],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BEDROCK_RUNTIME_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.activity-observation",
        emitted_by: &[PROFILE_RUNTIME],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BEDROCK_RUNTIME_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.model-selection",
        emitted_by: &[PROFILE_RUNTIME],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BEDROCK_RUNTIME_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.maximum-output-tokens",
        emitted_by: &[PROFILE_RUNTIME],
        withheld_because: "",
    },
];
