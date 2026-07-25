//! llama.cpp serving-runtime drivers for Swallowtail.
//!
#![forbid(unsafe_code)]

mod driver;
mod failure;
mod prepared;
mod protocol;
mod selection;
mod transport;

pub use driver::{
    LlamaCppAttachedDriver, LlamaCppOwnedDriver, llama_cpp_attached_descriptor,
    llama_cpp_owned_descriptor,
};
pub use prepared::{
    LlamaCppAttachedPreparationInput, LlamaCppAttachedPreparedEvidence,
    LlamaCppAttachedPreparedIntegration, LlamaCppCatalogueProfileInput,
    LlamaCppInferenceProfileInput, LlamaCppModelSelection, LlamaCppOwnedPreparationInput,
    LlamaCppOwnedPreparedEvidence, LlamaCppOwnedPreparedIntegration, LlamaCppOwnedServingSelection,
    LlamaCppPreparedCatalogue, LlamaCppPreparedInferenceAttempt, LlamaCppPreparedServingStart,
    prepare_llama_cpp_attached, prepare_llama_cpp_owned,
};
pub use selection::{
    LLAMA_CPP_ATTACHED_ACCESS_PROFILE_ID, LLAMA_CPP_ATTACHED_BUILD, LLAMA_CPP_ATTACHED_COMMIT,
    LLAMA_CPP_ATTACHED_ENDPOINT_AUDIENCE, LLAMA_CPP_ATTACHED_RUNTIME_REVISION,
    LLAMA_CPP_OWNED_ACCESS_PROFILE_ID, LLAMA_CPP_OWNED_BUILD, LLAMA_CPP_OWNED_COMMIT,
    LLAMA_CPP_OWNED_ENDPOINT_AUDIENCE, LLAMA_CPP_OWNED_RUNTIME_REVISION,
    llama_cpp_attached_access_profile, llama_cpp_attached_runtime_binding,
    llama_cpp_attached_runtime_claim, llama_cpp_owned_access_profile,
    llama_cpp_owned_runtime_binding, llama_cpp_owned_runtime_claim,
};
