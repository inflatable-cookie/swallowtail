//! Pi RPC harness integration for Swallowtail.

#![forbid(unsafe_code)]

mod activity;
mod callback;
mod catalogue;
mod connection;
mod discovery;
mod driver;
mod failure;
mod prepared;
mod prepared_profile;
pub mod protocol;
mod selection;
mod turn;

pub use driver::{PiRpcDriver, pi_rpc_descriptor};
pub use prepared::{PiPreparationInput, PiPreparationProbe, PiPreparedIntegration, prepare_pi_rpc};
pub use prepared_profile::{
    PiCatalogueProfileInput, PiModelSelection, PiPreparedCatalogue, PiPreparedEvidence,
    PiPreparedRun, PiPreparedSession, PiRunProfileInput, PiSessionProfileInput,
};
pub use selection::{
    PI_PACKAGE_AXIS, PI_PACKAGE_BASELINE_VERSION, PI_PACKAGE_LATEST_QUALIFIED_VERSION,
    pi_package_binding, pi_rpc_claim,
};

pub const PINNED_PI_VERSION: &str = "0.80.10";

const DRIVER_ID: &str = "swallowtail.pi.rpc";
