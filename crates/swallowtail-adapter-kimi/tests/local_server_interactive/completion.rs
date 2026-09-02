use super::fixture::{attached_input, id, prepare, probe, session_profile, turn};
use crate::interactive_support::{InteractiveFixtureServer, InteractiveScenario};
use crate::lifecycle_support::{FixtureHost, close_session};
use futures_executor::block_on;
use futures_util::StreamExt;
use std::num::{NonZeroU32, NonZeroU64};
use swallowtail_adapter_kimi::{
    KimiLocalServerOwnedInput, KimiLocalServerPermissionMode, KimiLocalServerReconciliationInput,
    KimiLocalServerSessionManagementInput, KimiModelSelection, start_kimi_local_server_owned,
};
use swallowtail_core::{
    ExecutionHostId, InstanceTargetRef, ModelId, ModelRouteId, ModelRouteRevision,
    ObservableActivityAvailability, ProviderSessionBindingOrigin, ProviderSessionEffectTruth,
};
use swallowtail_runtime::{
    CleanupOutcome, OperationDetachmentAcknowledgement, ProviderSessionReconciliationBounds,
    RequestId, SettledSessionAttachment, SettledSessionAttachmentKind,
    SettledSessionRestorationOutcome, TerminalStatus, WorkingStateRestorationMethod,
};

include!("completion/events.rs");
include!("completion/session.rs");
include!("completion/reconciliation.rs");
include!("completion/owned.rs");
