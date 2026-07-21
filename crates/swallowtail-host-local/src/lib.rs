//! Host-approved local execution services for Swallowtail.
//!
//! This crate owns concrete operating-system process, endpoint, credential,
//! materialization, and time behavior. Drivers receive only host-approved
//! values through opaque references and scoped grants.

#![forbid(unsafe_code)]

mod attachment;
mod child;
mod credential;
mod deadline;
mod host;
mod hosted;
mod limits;
mod materialization;
mod model_artifact;
mod network;
mod output;
mod process_exit;
mod process_reader;
mod schema;
mod serving_endpoint;
mod working_resource;
mod working_resource_io;

pub use host::{LocalProcessHost, LocalProcessHostBuilder};
pub use limits::{LocalMaterializationLimits, LocalProcessLimits};
