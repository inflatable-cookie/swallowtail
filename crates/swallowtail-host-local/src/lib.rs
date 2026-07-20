//! Host-approved local process services for Swallowtail.
//!
//! This crate owns concrete operating-system process behavior. Drivers receive
//! only opaque executable, environment, and working-resource references.

#![forbid(unsafe_code)]

mod attachment;
mod child;
mod deadline;
mod host;
mod limits;
mod materialization;
mod output;
mod process_exit;
mod process_reader;
mod schema;
mod working_resource;

pub use host::{LocalProcessHost, LocalProcessHostBuilder};
pub use limits::{LocalMaterializationLimits, LocalProcessLimits};
