//! Host-approved local execution services for Swallowtail.
//!
//! This crate owns concrete operating-system process, endpoint, credential,
//! materialization, and time behavior. Drivers receive only host-approved
//! values through opaque references and scoped grants.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod attachment;
mod child;
mod connection_lifecycle;
mod credential;
mod deadline;
mod executable_launch;
mod host;
mod hosted;
mod installed_target;
mod limits;
mod materialization;
mod model_artifact;
mod network;
mod output;
mod process;
mod process_exit;
mod process_reader;
mod schema;
mod services;
mod serving_endpoint;
mod sign_in;
mod task;
mod watcher;
mod watcher_bridge;
mod working_resource;
mod working_resource_io;

pub use connection_lifecycle::{JsonFileConnectionLifecycleStore, MemoryConnectionLifecycleStore};
pub use executable_launch::LocalExecutableLaunch;
pub use host::{LocalProcessHost, LocalProcessHostBuilder};
pub use limits::{LocalMaterializationLimits, LocalProcessLimits};
pub use services::LocalHostServices;
pub use sign_in::{LocalSignInCall, LocalSignInPorts};
pub use task::LocalScopedTaskService;
