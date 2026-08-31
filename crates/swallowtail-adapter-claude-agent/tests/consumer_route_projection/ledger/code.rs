use super::{CODE_ROUTE, CODE_RUN, LedgerEntry};

const EVERY: &[&str] = &[CODE_RUN];
const NO_CATALOGUE: &str = "the prepared headless run carries no model-catalogue observation";

const fn emitted(operation_shape: &'static str, semantic_id: &'static str) -> LedgerEntry {
    LedgerEntry {
        route_id: CODE_ROUTE,
        operation_shape,
        semantic_id,
        emitted_by: EVERY,
        withheld_because: "",
    }
}

pub(crate) const CODE_TRANCHE: [LedgerEntry; 12] = [
    LedgerEntry {
        route_id: CODE_ROUTE,
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
    emitted("route-capability", "feature.working-resource"),
    emitted("route-capability", "feature.prepared-facade"),
    emitted("route-observation", "feature.activity-observation"),
    emitted("structured-run", "control.model-selection"),
    emitted("structured-run", "control.reasoning-selection"),
    emitted("structured-run", "control.maximum-agentic-turns"),
];
