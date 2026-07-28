//! Ollama native attached-runtime integration for Swallowtail.

#![forbid(unsafe_code)]

mod driver;
mod failure;
mod prepared;
mod prepared_profile;
pub mod protocol;
mod selection;
mod transport;

pub use driver::OllamaNativeAttachedDriver;
pub use prepared::{
    OllamaPreparationInput, OllamaPreparationProbe, OllamaPreparedIntegration,
    OllamaPreparedRuntimeObservation, prepare_ollama_attached,
};
pub use prepared_profile::{
    OllamaInferenceAttemptInput, OllamaInventoryProfileInput, OllamaInventorySnapshot,
    OllamaModelSelection, OllamaPreparedEvidence, OllamaPreparedInferenceAttempt,
    OllamaPreparedInventory,
};
pub use protocol::OllamaModelCapability;
pub use selection::{
    OLLAMA_BASELINE_VERSION, OLLAMA_LATEST_QUALIFIED_VERSION, OLLAMA_NATIVE_FACADE,
    ollama_native_descriptor, ollama_runtime_binding, ollama_runtime_claim,
};
