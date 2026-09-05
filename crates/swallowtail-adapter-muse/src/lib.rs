//! Exact installed Meta Muse Code headless integration for Swallowtail.
//!
//! The initial surface binds one signed versioned payload and decodes its
//! bounded JSONL run protocol. It does not invoke the mutable update launcher,
//! own login, retain sessions, or expose the separate Meta Model API.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod access;
mod activity;
mod command;
mod consumer_route_projection;
mod discovery;
mod driver;
mod events;
mod failure;
mod handle;
mod prepared;
mod pump;
mod selection;
mod validation;

pub use access::{MUSE_LOCAL_META_ACCOUNT_AUDIENCE, muse_local_meta_account_access_profile};
pub use driver::{MuseHeadlessDriver, muse_headless_descriptor};
pub use prepared::{
    MUSE_META_PROVIDER_ID, MuseHeadlessModelSelection, MusePreparationInput, MusePreparationProbe,
    MusePreparedIntegration, MusePreparedRun, MuseRunProfileInput, prepare_muse_headless,
};
pub use selection::{
    MUSE_CODE_PAYLOAD_BASENAME, MUSE_CODE_RELEASE_AXIS, MUSE_CODE_RELEASE_REVISION,
    MUSE_SPARK_MODEL_ID, muse_code_release_binding, muse_headless_claim,
};

pub(crate) const DRIVER_ID: &str = "swallowtail.muse-code.headless";
