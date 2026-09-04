//! Antigravity CLI harness integration for Swallowtail.
//!
//! The initial production surface provides exact installed discovery and an
//! authenticated model catalogue. Headless execution remains a separate next
//! driver.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod access;
mod catalogue;
mod consumer_route_projection;
mod descriptor;
mod discovery;
mod failure;
mod headless;
mod headless_activity;
mod headless_command;
mod headless_events;
mod headless_handle;
mod headless_pump;
mod headless_validation;
mod prepared;
mod selection;
mod session;

pub use access::{
    ANTIGRAVITY_PERSONAL_GOOGLE_AUDIENCE, antigravity_personal_google_access_profile,
};
pub use catalogue::AntigravityCatalogueDriver;
pub use descriptor::{antigravity_catalogue_descriptor, antigravity_headless_descriptor};
pub use headless::AntigravityHeadlessDriver;
pub use prepared::{
    AntigravityCatalogueProfileInput, AntigravityContinuationProfileInput,
    AntigravityHeadlessModelSelection, AntigravityHeadlessRunProfileInput,
    AntigravityPreparationInput, AntigravityPreparationProbe, AntigravityPreparedCatalogue,
    AntigravityPreparedCatalogueIntegration, AntigravityPreparedContinuation,
    AntigravityPreparedContinuationIntegration, AntigravityPreparedDriver,
    AntigravityPreparedHeadlessIntegration, AntigravityPreparedHeadlessRun,
    AntigravityPreparedIntegration, prepare_antigravity,
};
pub use selection::{
    ANTIGRAVITY_AUTOMATIC_EXECUTABLE_NAME, ANTIGRAVITY_BASELINE_VERSION,
    ANTIGRAVITY_LATEST_QUALIFIED_VERSION, ANTIGRAVITY_RELEASE_AXIS, antigravity_catalogue_claim,
    antigravity_headless_claim, antigravity_release_binding,
};

const CATALOGUE_DRIVER_ID: &str = "swallowtail.antigravity.catalogue";
pub(crate) const HEADLESS_DRIVER_ID: &str = "swallowtail.antigravity.headless";
