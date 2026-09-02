use super::{AGENT_DELETE, AGENT_OBSERVED, AGENT_ROUTE, AGENT_RUN, AGENT_SESSION, LedgerEntry};

const RUN: &[&str] = &[AGENT_RUN];
const SESSION: &[&str] = &[AGENT_SESSION, AGENT_OBSERVED];
const RUN_SESSION: &[&str] = &[AGENT_RUN, AGENT_SESSION, AGENT_OBSERVED];
const DELETE: &[&str] = &[AGENT_DELETE];
const SESSION_DELETE: &[&str] = &[AGENT_SESSION, AGENT_OBSERVED, AGENT_DELETE];
const OBSERVED: &[&str] = &[AGENT_OBSERVED];
const NO_CATALOGUE: &str = "no prepared route carries a model-catalogue observation";

const fn emitted(
    operation_shape: &'static str,
    semantic_id: &'static str,
    emitted_by: &'static [&'static str],
) -> LedgerEntry {
    LedgerEntry {
        route_id: AGENT_ROUTE,
        operation_shape,
        semantic_id,
        emitted_by,
        withheld_because: "",
    }
}

pub(crate) const AGENT_TRANCHE: [LedgerEntry; 31] = [
    LedgerEntry {
        route_id: AGENT_ROUTE,
        operation_shape: "model-catalogue",
        semantic_id: "feature.model-catalogue",
        emitted_by: &[],
        withheld_because: NO_CATALOGUE,
    },
    emitted("structured-run", "feature.structured-run", RUN),
    emitted(
        "interactive-session",
        "feature.interactive-session",
        SESSION,
    ),
    emitted("route-observation", "feature.streaming-events", RUN_SESSION),
    emitted("route-observation", "feature.usage-evidence", RUN_SESSION),
    emitted(
        "route-capability",
        "feature.reasoning-selection",
        RUN_SESSION,
    ),
    emitted(
        "route-capability",
        "feature.permission-exchange",
        RUN_SESSION,
    ),
    emitted("route-capability", "feature.question-exchange", RUN_SESSION),
    emitted(
        "route-capability",
        "feature.cancellation-or-interruption",
        RUN_SESSION,
    ),
    emitted("session-lifecycle", "feature.load-session", SESSION),
    emitted("session-lifecycle", "feature.resume-session", SESSION),
    emitted("route-capability", "feature.working-resource", RUN_SESSION),
    emitted(
        "session-lifecycle",
        "feature.provider-session-delete",
        SESSION_DELETE,
    ),
    emitted(
        "session-lifecycle",
        "feature.native-session-close",
        RUN_SESSION,
    ),
    emitted(
        "route-capability",
        "feature.owned-remote-resource-cleanup",
        RUN,
    ),
    emitted(
        "session-lifecycle",
        "feature.persistent-session-posture",
        RUN,
    ),
    emitted(
        "route-capability",
        "feature.prepared-facade",
        &[AGENT_RUN, AGENT_SESSION, AGENT_DELETE, AGENT_OBSERVED],
    ),
    emitted(
        "route-observation",
        "feature.activity-observation",
        RUN_SESSION,
    ),
    emitted(
        "interactive-session",
        "feature.active-session-reasoning-ack",
        OBSERVED,
    ),
    emitted(
        "interactive-session",
        "feature.negotiated-model-options-observation",
        OBSERVED,
    ),
    emitted("structured-run", "control.model-selection", RUN),
    emitted("interactive-session", "control.model-selection", SESSION),
    emitted("structured-run", "control.reasoning-selection", RUN),
    emitted(
        "interactive-session",
        "control.reasoning-selection",
        SESSION,
    ),
    emitted("interactive-session", "control.session-options", SESSION),
    emitted("session-management", "control.load-session", SESSION),
    emitted("session-management", "control.resume-session", SESSION),
    emitted("structured-run", "control.permission-handling", RUN),
    emitted(
        "interactive-session",
        "control.permission-handling",
        SESSION,
    ),
    emitted("structured-run", "control.run-retention", RUN),
    emitted(
        "session-management",
        "control.provider-session-delete",
        DELETE,
    ),
];
