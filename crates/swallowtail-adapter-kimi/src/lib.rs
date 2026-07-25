//! Kimi Code ACP integration for Swallowtail.

#![forbid(unsafe_code)]

mod connection;
mod discovery;
mod driver;
mod failure;
mod prepared;
mod prepared_profile;
mod selection;
mod turn;

pub use driver::{KimiAcpDriver, kimi_acp_descriptor};
pub use prepared::{
    KimiPreparationInput, KimiPreparationProbe, KimiPreparedIntegration, prepare_kimi,
};
pub use prepared_profile::{
    KimiModelSelection, KimiPreparedEvidence, KimiPreparedSession, KimiPreparedSessionFuture,
    KimiPreparedSessionLoadFuture, KimiSessionProfileInput,
};
pub use selection::{
    KIMI_CODE_AXIS, KIMI_CODE_BASELINE_VERSION, KIMI_CODE_LATEST_QUALIFIED_VERSION, kimi_acp_claim,
    kimi_code_binding,
};

const MAXIMUM_REPLAY_ITEMS: usize = 512;
const MAXIMUM_REPLAY_BYTES: usize = 4 * 1024 * 1024;
const MAXIMUM_WRITE_BYTES: usize = 1024 * 1024;
