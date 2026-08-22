//! Ollama native attached-runtime integration for Swallowtail.
//!
//! Preparation binds an externally managed local runtime to an exact model
//! tag and manifest digest before inventory, structured-run, or interactive
//! session authority is produced.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod activity;
mod addable;
mod context_window;

pub(crate) use context_window::validate_context_window_agreement;
mod driver;
mod failure;
mod prepared;
mod prepared_profile;
/// Bounded native HTTP request, response, catalogue, and stream projection.
pub mod protocol;
mod selection;
mod transport;

pub use addable::{
    OLLAMA_ATTACHED_ADDABLE_ROUTE_ID, OLLAMA_ATTACHED_ENDPOINT_FIELD_ID,
    ollama_attached_addable_route_descriptor,
};
pub use context_window::{OllamaContextWindow, MAXIMUM, MINIMUM};
pub use driver::OllamaNativeAttachedDriver;
pub use prepared::{
    OllamaPreparationInput, OllamaPreparationProbe, OllamaPreparedIntegration,
    OllamaPreparedRuntimeObservation, prepare_ollama_attached,
};
pub use prepared_profile::{
    OllamaInferenceAttemptInput, OllamaInventoryProfileInput, OllamaInventorySnapshot,
    OllamaModelSelection, OllamaPreparedEvidence, OllamaPreparedInferenceAttempt,
    OllamaPreparedInventory, OllamaPreparedSession, OllamaSessionProfileInput,
};
pub use protocol::OllamaModelCapability;
pub use selection::{
    OLLAMA_BASELINE_VERSION, OLLAMA_LATEST_QUALIFIED_VERSION, OLLAMA_NATIVE_FACADE,
    ollama_native_descriptor, ollama_runtime_binding, ollama_runtime_claim,
};
