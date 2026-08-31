/// Exact census route the bounded background descriptors belong to.
pub(super) const BACKGROUND_ROUTE: &str = "openai.background";

/// One prepared run carrying only the inputs background preparation requires.
pub(super) const MINIMAL: &str = "OpenAiPreparedBackgroundRun[minimal]";
/// One prepared run adding reasoning, structured output, and service tier.
pub(super) const TIERED: &str = "OpenAiPreparedBackgroundRun[tiered]";
/// One prepared run adding active-run detachment, which excludes service tier.
pub(super) const DETACHED: &str = "OpenAiPreparedBackgroundRun[detached]";

pub(super) const BACKGROUND_PROFILES: [&str; 3] = [MINIMAL, TIERED, DETACHED];

/// Exact census operation shapes the `openai.background` tranche spans.
pub(super) const BACKGROUND_OPERATION_SHAPES: [&str; 5] = [
    "model-catalogue",
    "structured-run",
    "route-observation",
    "route-capability",
    "session-lifecycle",
];
pub(super) const EVERY: &[&str] = &[MINIMAL, TIERED, DETACHED];
pub(super) const TIERED_ONLY: &[&str] = &[TIERED];
pub(super) const DETACHED_ONLY: &[&str] = &[DETACHED];

const SEPARATE_FAMILY: &str =
    "a separate prepared family owns it; background structured-run evidence does not";

/// One exact `openai.background` census row and its adapter disposition.
pub(super) struct LedgerEntry {
    pub(super) route_id: &'static str,
    pub(super) operation_shape: &'static str,
    pub(super) semantic_id: &'static str,
    pub(super) emitted_by: &'static [&'static str],
    pub(super) withheld_because: &'static str,
}

/// Deterministic disposition of exactly the 24 `openai.background` census rows.
///
/// The ledger claims nothing about `openai.realtime` or any other route.
pub(super) const OPENAI_BACKGROUND_TRANCHE: [LedgerEntry; 24] = [
    LedgerEntry {
        route_id: BACKGROUND_ROUTE,
        operation_shape: "model-catalogue",
        semantic_id: "feature.model-catalogue",
        emitted_by: &[],
        withheld_because: SEPARATE_FAMILY,
    },
    LedgerEntry {
        route_id: BACKGROUND_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "feature.structured-run",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BACKGROUND_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.streaming-events",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BACKGROUND_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.usage-evidence",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BACKGROUND_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.output-token-limit",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BACKGROUND_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.reasoning-selection",
        emitted_by: TIERED_ONLY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BACKGROUND_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.structured-output",
        emitted_by: TIERED_ONLY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BACKGROUND_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.cancellation-or-interruption",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BACKGROUND_ROUTE,
        operation_shape: "session-lifecycle",
        semantic_id: "feature.retained-background-execution",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BACKGROUND_ROUTE,
        operation_shape: "session-lifecycle",
        semantic_id: "feature.stream-reattachment",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BACKGROUND_ROUTE,
        operation_shape: "session-lifecycle",
        semantic_id: "feature.provider-managed-recovery",
        emitted_by: &[],
        withheld_because: "no prepared background plan requires a provider-recovery capability",
    },
    LedgerEntry {
        route_id: BACKGROUND_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.owned-remote-resource-cleanup",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BACKGROUND_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.prepared-facade",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BACKGROUND_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.activity-observation",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BACKGROUND_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.model-selection",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BACKGROUND_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.reasoning-selection",
        emitted_by: TIERED_ONLY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BACKGROUND_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.maximum-output-tokens",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BACKGROUND_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.structured-output",
        emitted_by: TIERED_ONLY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BACKGROUND_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.provider-execution-policy",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BACKGROUND_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.provider-retention-policy",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BACKGROUND_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.stream-reattachment",
        emitted_by: EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BACKGROUND_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.provider-recovery-policy",
        emitted_by: &[],
        withheld_because: "the background profile input exposes no provider-recovery value",
    },
    LedgerEntry {
        route_id: BACKGROUND_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.service-tier",
        emitted_by: TIERED_ONLY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: BACKGROUND_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.active-run-detachment",
        emitted_by: DETACHED_ONLY,
        withheld_because: "",
    },
];

/// Identities with no `openai.background` census row at all.
///
/// Package ownership is not evidence: neither a Realtime row nor a prepared
/// capability without census identity may be constructed on this route.
pub(super) const WITHHELD_OFF_ROUTE: [&str; 6] = [
    "feature.realtime-media-session",
    "feature.active-session-reasoning-ack",
    "feature.persistent-session-posture",
    "feature.provider-temporary-retention",
    "feature.active-operation-detachment",
    "control.realtime-media-config",
];
