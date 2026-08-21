//! Pi RPC harness integration for Swallowtail.
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
/// Bounded public decoder for qualified Pi RPC record shapes.
pub mod protocol;
mod selection;
/// Pi SDK sidecar asset, identity constants, and private wire decoder.
pub mod sidecar;
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
pub use sidecar::{
    PI_SDK_SIDECAR_ADDABLE_ROUTE_ID, PI_SDK_SIDECAR_CREDENTIAL_FIELD_ID,
    PI_SDK_SIDECAR_ENVIRONMENT_FIELD_ID, PI_SDK_SIDECAR_LAUNCH_RECIPE_FIELD_ID,
    PI_SDK_SIDECAR_NODE_AXIS, PI_SDK_SIDECAR_PACKAGE_AXIS, PI_SDK_SIDECAR_SIDECAR_AXIS,
    PI_SDK_SIDECAR_WIRE_AXIS, PiSdkSidecarDriver, PiSdkSidecarPreparedSession,
    PiSdkSidecarSessionPreparation, pi_sdk_sidecar_addable_route_descriptor,
    pi_sdk_sidecar_descriptor, pi_sdk_sidecar_node_binding, pi_sdk_sidecar_node_claim,
    pi_sdk_sidecar_package_binding, pi_sdk_sidecar_package_claim, pi_sdk_sidecar_sidecar_binding,
    pi_sdk_sidecar_sidecar_claim, pi_sdk_sidecar_wire_binding, pi_sdk_sidecar_wire_claim,
    prepare_pi_sdk_sidecar_session,
};

const DRIVER_ID: &str = "swallowtail.pi.rpc";
