use super::{LedgerEntry, RESPONSE_ROUTE, RESPONSE_RUN};

const EVERY: &[&str] = &[RESPONSE_RUN];
const NO_CATALOGUE: &str = "the prepared response-only run carries no model-catalogue observation";
const NO_WORKING_RESOURCE: &str =
    "the response-only request carries no prepared working-resource reference";

const fn emitted(operation_shape: &'static str, semantic_id: &'static str) -> LedgerEntry {
    LedgerEntry {
        route_id: RESPONSE_ROUTE,
        operation_shape,
        semantic_id,
        emitted_by: EVERY,
        withheld_because: "",
    }
}

pub(crate) const RESPONSE_TRANCHE: [LedgerEntry; 11] = [
    LedgerEntry {
        route_id: RESPONSE_ROUTE,
        operation_shape: "model-catalogue",
        semantic_id: "feature.model-catalogue",
        emitted_by: &[],
        withheld_because: NO_CATALOGUE,
    },
    emitted("structured-run", "feature.structured-run"),
    emitted("route-observation", "feature.streaming-events"),
    emitted("route-observation", "feature.usage-evidence"),
    emitted("route-capability", "feature.reasoning-selection"),
    emitted("route-capability", "feature.cancellation-or-interruption"),
    LedgerEntry {
        route_id: RESPONSE_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.working-resource",
        emitted_by: &[],
        withheld_because: NO_WORKING_RESOURCE,
    },
    emitted("route-capability", "feature.prepared-facade"),
    emitted("route-observation", "feature.activity-observation"),
    emitted("structured-run", "control.model-selection"),
    emitted("structured-run", "control.reasoning-selection"),
];
