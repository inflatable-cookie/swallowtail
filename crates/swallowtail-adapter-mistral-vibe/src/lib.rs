//! Mistral Vibe headless harness driver.
//!
//! `mistral-vibe.headless` binds host-approved `vibe --prompt --output streaming`
//! for one bounded print run through `prepare_mistral_vibe_headless`. `vibe-acp`,
//! TUI, `--continue`/`--resume`, teleport, and `--auto-approve`/`--yolo` stay out.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod access;
mod command;
mod discovery;
mod driver;
mod failure;
mod prepared;
mod selection;

pub use access::{MISTRAL_VIBE_LOCAL_ACCOUNT_AUDIENCE, mistral_vibe_local_config_access_profile};
pub use driver::{MistralVibeHeadlessDriver, mistral_vibe_headless_descriptor};
pub use prepared::{
    MistralVibeHeadlessPreparationInput, MistralVibeHeadlessPreparationProbe,
    MistralVibeHeadlessPreparedIntegration, MistralVibeHeadlessPreparedRun,
    MistralVibeHeadlessRunProfileInput, prepare_mistral_vibe_headless,
};
pub use selection::{
    MISTRAL_VIBE_EXECUTABLE_NAME, MISTRAL_VIBE_RELEASE_AXIS, MISTRAL_VIBE_RELEASE_VERSION,
    mistral_vibe_headless_claim, mistral_vibe_release_binding,
};
