use super::*;
use crate::support::app_server::ThreadCatalogueMode;
use std::num::{NonZeroU32, NonZeroU64};
use swallowtail_adapter_codex::{
    CodexSessionCatalogueInput, CodexSessionHistoryInput, CodexSessionReconciliationInput,
};
use swallowtail_core::{
    ProviderSessionActivityState, ProviderSessionBindingOrigin, ProviderSessionCatalogueBounds,
    ProviderSessionImportAvailability, ProviderSessionImportUnavailableReason,
};
use swallowtail_runtime::ProviderSessionCatalogueId;
use swallowtail_runtime::{
    ProviderSessionHistoryBounds, ProviderSessionHistoryId, ProviderSessionReconciliationBounds,
    RuntimeTurnId, SessionResumeBinding, SettledSessionAttachmentKind,
    SettledSessionRestorationOutcome, WorkingStateRestorationMethod,
    WorkingStateRestorationOutcome,
};
use swallowtail_testkit::RecordedHostCall;

mod acceptance;

const PRIVATE_TITLE: &str = "Imported thread";
const PRIVATE_PREVIEW: &str = "Bounded provider preview";

include!("provider_session_import/reconciliation.rs");
include!("provider_session_import/history.rs");
include!("provider_session_import/catalogue.rs");
include!("provider_session_import/import.rs");
include!("provider_session_import/support.rs");
