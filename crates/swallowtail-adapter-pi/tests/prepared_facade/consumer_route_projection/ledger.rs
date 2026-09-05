pub(super) const RPC_ROUTE: &str = "pi.rpc";
pub(super) const SIDECAR_ROUTE: &str = "pi.sdk-sidecar";

pub(super) const RPC_CATALOGUE: &str = "PiPreparedCatalogue";
pub(super) const RPC_RUN_MINIMAL: &str = "PiPreparedRun[minimal]";
pub(super) const RPC_RUN_ATTACHMENTS: &str = "PiPreparedRun[with_attachments]";
pub(super) const RPC_SESSION_MINIMAL: &str = "PiPreparedSession[minimal]";
pub(super) const RPC_SESSION_ATTACHMENTS: &str = "PiPreparedSession[with_image_attachments]";
pub(super) const RPC_PROFILES: [&str; 5] = [
    RPC_CATALOGUE,
    RPC_RUN_MINIMAL,
    RPC_RUN_ATTACHMENTS,
    RPC_SESSION_MINIMAL,
    RPC_SESSION_ATTACHMENTS,
];

pub(super) const SIDECAR_MINIMAL: &str = "PiSdkSidecarPreparedSession[minimal]";
pub(super) const SIDECAR_REASONING: &str = "PiSdkSidecarPreparedSession[with_reasoning]";
pub(super) const SIDECAR_ATTACHMENTS: &str = "PiSdkSidecarPreparedSession[with_image_attachments]";
pub(super) const SIDECAR_REASONING_ATTACHMENTS: &str =
    "PiSdkSidecarPreparedSession[with_reasoning_and_attachments]";
pub(super) const SIDECAR_PROFILES: [&str; 4] = [
    SIDECAR_MINIMAL,
    SIDECAR_REASONING,
    SIDECAR_ATTACHMENTS,
    SIDECAR_REASONING_ATTACHMENTS,
];

pub(super) const SIDECAR_NO_CATALOGUE_ROLE: &str = "sidecar/prepared/build.rs::prepare retains DriverRole::InteractiveSession; the session facade has no catalogue role or route";
pub(super) const SIDECAR_NO_USAGE_REQUIREMENT: &str =
    "sidecar/prepared/build.rs::prepare retains no Capability::UsageReporting requirement";
pub(super) const SIDECAR_NO_ACTIVITY_PROFILE: &str =
    "sidecar/prepared/build.rs::prepare retains no ObservableActivityProfile-derived requirement";

pub(super) struct LedgerEntry {
    pub(super) route_id: &'static str,
    pub(super) operation_shape: &'static str,
    pub(super) semantic_id: &'static str,
    pub(super) emitted_by: &'static [&'static str],
    pub(super) withheld_because: &'static str,
}

const RPC_RUNS: &[&str] = &[RPC_RUN_MINIMAL, RPC_RUN_ATTACHMENTS];
const RPC_SESSIONS: &[&str] = &[RPC_SESSION_MINIMAL, RPC_SESSION_ATTACHMENTS];
const RPC_RUN_SESSION: &[&str] = &[
    RPC_RUN_MINIMAL,
    RPC_RUN_ATTACHMENTS,
    RPC_SESSION_MINIMAL,
    RPC_SESSION_ATTACHMENTS,
];
const RPC_ATTACHMENT_RUN: &[&str] = &[RPC_RUN_ATTACHMENTS];
const RPC_ATTACHMENT_SESSION: &[&str] = &[RPC_SESSION_ATTACHMENTS];
const RPC_PREPARED: &[&str] = &[
    RPC_CATALOGUE,
    RPC_RUN_MINIMAL,
    RPC_RUN_ATTACHMENTS,
    RPC_SESSION_MINIMAL,
    RPC_SESSION_ATTACHMENTS,
];

pub(super) const PI_RPC_LEDGER: [LedgerEntry; 15] = [
    LedgerEntry {
        route_id: RPC_ROUTE,
        operation_shape: "model-catalogue",
        semantic_id: "feature.model-catalogue",
        emitted_by: &[RPC_CATALOGUE],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: RPC_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "feature.structured-run",
        emitted_by: RPC_RUNS,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: RPC_ROUTE,
        operation_shape: "interactive-session",
        semantic_id: "feature.interactive-session",
        emitted_by: RPC_SESSIONS,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: RPC_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.streaming-events",
        emitted_by: RPC_RUN_SESSION,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: RPC_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.usage-evidence",
        emitted_by: RPC_RUN_SESSION,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: RPC_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.attachments",
        emitted_by: &[RPC_RUN_ATTACHMENTS, RPC_SESSION_ATTACHMENTS],
        withheld_because: "",
    },
    LedgerEntry {
        route_id: RPC_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.question-exchange",
        emitted_by: RPC_SESSIONS,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: RPC_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.cancellation-or-interruption",
        emitted_by: RPC_RUN_SESSION,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: RPC_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.working-resource",
        emitted_by: RPC_RUN_SESSION,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: RPC_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.prepared-facade",
        emitted_by: RPC_PREPARED,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: RPC_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.activity-observation",
        emitted_by: RPC_RUN_SESSION,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: RPC_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.model-selection",
        emitted_by: RPC_RUNS,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: RPC_ROUTE,
        operation_shape: "interactive-session",
        semantic_id: "control.model-selection",
        emitted_by: RPC_SESSIONS,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: RPC_ROUTE,
        operation_shape: "structured-run",
        semantic_id: "control.attachments",
        emitted_by: RPC_ATTACHMENT_RUN,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: RPC_ROUTE,
        operation_shape: "interactive-session",
        semantic_id: "control.attachments",
        emitted_by: RPC_ATTACHMENT_SESSION,
        withheld_because: "",
    },
];

const SIDECAR_EVERY: &[&str] = &[
    SIDECAR_MINIMAL,
    SIDECAR_REASONING,
    SIDECAR_ATTACHMENTS,
    SIDECAR_REASONING_ATTACHMENTS,
];
const SIDECAR_REASONING_PROFILES: &[&str] = &[SIDECAR_REASONING, SIDECAR_REASONING_ATTACHMENTS];
const SIDECAR_ATTACHMENT_PROFILES: &[&str] = &[SIDECAR_ATTACHMENTS, SIDECAR_REASONING_ATTACHMENTS];

pub(super) const PI_SIDECAR_LEDGER: [LedgerEntry; 19] = [
    LedgerEntry {
        route_id: SIDECAR_ROUTE,
        operation_shape: "model-catalogue",
        semantic_id: "feature.model-catalogue",
        emitted_by: &[],
        withheld_because: SIDECAR_NO_CATALOGUE_ROLE,
    },
    LedgerEntry {
        route_id: SIDECAR_ROUTE,
        operation_shape: "interactive-session",
        semantic_id: "feature.interactive-session",
        emitted_by: SIDECAR_EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: SIDECAR_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.streaming-events",
        emitted_by: SIDECAR_EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: SIDECAR_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.usage-evidence",
        emitted_by: &[],
        withheld_because: SIDECAR_NO_USAGE_REQUIREMENT,
    },
    LedgerEntry {
        route_id: SIDECAR_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.reasoning-selection",
        emitted_by: SIDECAR_REASONING_PROFILES,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: SIDECAR_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.attachments",
        emitted_by: SIDECAR_ATTACHMENT_PROFILES,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: SIDECAR_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.cancellation-or-interruption",
        emitted_by: SIDECAR_EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: SIDECAR_ROUTE,
        operation_shape: "session-lifecycle",
        semantic_id: "feature.load-session",
        emitted_by: SIDECAR_EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: SIDECAR_ROUTE,
        operation_shape: "session-lifecycle",
        semantic_id: "feature.resume-session",
        emitted_by: SIDECAR_EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: SIDECAR_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.working-resource",
        emitted_by: SIDECAR_EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: SIDECAR_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.persistent-session-posture",
        emitted_by: SIDECAR_EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: SIDECAR_ROUTE,
        operation_shape: "route-capability",
        semantic_id: "feature.prepared-facade",
        emitted_by: SIDECAR_EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: SIDECAR_ROUTE,
        operation_shape: "route-observation",
        semantic_id: "feature.activity-observation",
        emitted_by: &[],
        withheld_because: SIDECAR_NO_ACTIVITY_PROFILE,
    },
    LedgerEntry {
        route_id: SIDECAR_ROUTE,
        operation_shape: "interactive-session",
        semantic_id: "control.model-selection",
        emitted_by: SIDECAR_EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: SIDECAR_ROUTE,
        operation_shape: "interactive-session",
        semantic_id: "control.reasoning-selection",
        emitted_by: SIDECAR_REASONING_PROFILES,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: SIDECAR_ROUTE,
        operation_shape: "interactive-session",
        semantic_id: "control.attachments",
        emitted_by: SIDECAR_ATTACHMENT_PROFILES,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: SIDECAR_ROUTE,
        operation_shape: "interactive-session",
        semantic_id: "control.session-options",
        emitted_by: SIDECAR_EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: SIDECAR_ROUTE,
        operation_shape: "session-management",
        semantic_id: "control.load-session",
        emitted_by: SIDECAR_EVERY,
        withheld_because: "",
    },
    LedgerEntry {
        route_id: SIDECAR_ROUTE,
        operation_shape: "session-management",
        semantic_id: "control.resume-session",
        emitted_by: SIDECAR_EVERY,
        withheld_because: "",
    },
];
