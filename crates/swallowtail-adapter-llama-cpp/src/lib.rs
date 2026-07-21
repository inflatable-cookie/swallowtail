//! llama.cpp serving-runtime drivers for Swallowtail.
//!
#![forbid(unsafe_code)]

mod driver;
mod failure;
mod protocol;
mod transport;

pub use driver::{
    LlamaCppAttachedDriver, LlamaCppOwnedDriver, llama_cpp_attached_descriptor,
    llama_cpp_owned_descriptor,
};
