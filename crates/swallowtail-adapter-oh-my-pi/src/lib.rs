//! Oh My Pi RPC harness integration for Swallowtail.
//!
//! The installed package is discovered and version-qualified before catalogue,
//! structured-run, or interactive-session authority is prepared. Sessions are
//! fresh processes; restart restoration explicitly loses prior private state.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod activity;
mod callback;
mod catalogue;
mod connection;
mod discovery;
mod driver;
mod failure;
mod prepared;
mod prepared_profile;
/// Bounded public decoder for qualified Oh My Pi RPC record shapes.
pub mod protocol;
mod selection;
mod turn;

pub use driver::{OhMyPiRpcDriver, oh_my_pi_rpc_descriptor};
pub use prepared::{
    OhMyPiPreparationInput, OhMyPiPreparationProbe, OhMyPiPreparedIntegration, prepare_oh_my_pi_rpc,
};
pub use prepared_profile::{
    OhMyPiCatalogueProfileInput, OhMyPiModelSelection, OhMyPiPreparedCatalogue,
    OhMyPiPreparedEvidence, OhMyPiPreparedRun, OhMyPiPreparedSession, OhMyPiRunProfileInput,
    OhMyPiSessionProfileInput,
};
pub use selection::{
    OH_MY_PI_PACKAGE_AXIS, OH_MY_PI_PACKAGE_BASELINE_VERSION,
    OH_MY_PI_PACKAGE_LATEST_QUALIFIED_VERSION, oh_my_pi_package_binding, oh_my_pi_rpc_claim,
};

const DRIVER_ID: &str = "swallowtail.oh-my-pi.rpc";
