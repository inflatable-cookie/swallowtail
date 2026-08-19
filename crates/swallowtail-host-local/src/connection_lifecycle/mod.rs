//! Optional in-memory and JSON-file adapters for the Contract 057 store port.

mod document;
mod json_file;
mod memory;
mod state;

pub use json_file::JsonFileConnectionLifecycleStore;
pub use memory::MemoryConnectionLifecycleStore;
