//! OpenHands Agent Server harness driver.
//!
//! `openhands.agent-server` binds a host-approved Python interpreter to
//! `python -m openhands.agent_server --host 127.0.0.1` for one owned
//! loopback conversation through `prepare_openhands_agent_server`. V0
//! Socket.IO, Contract 035 remote ACP, the Python SDK, Docker/hosted
//! sandbox, and `NeverConfirm` stay out. Live HTTP/WebSocket conversation
//! remains unwired.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod access;
mod command;
mod discovery;
mod driver;
mod failure;
mod prepared;
mod protocol;
mod selection;
mod transport;

pub use access::{OPENHANDS_LOCAL_ACCOUNT_AUDIENCE, openhands_local_config_access_profile};
pub use driver::{OpenHandsAgentServerDriver, openhands_agent_server_descriptor};
pub use prepared::{
    OpenHandsAgentServerPreparationInput, OpenHandsAgentServerPreparationProbe,
    OpenHandsAgentServerPreparedIntegration, OpenHandsAgentServerPreparedRun,
    OpenHandsAgentServerRunProfileInput, prepare_openhands_agent_server,
};
pub use selection::{
    OPENHANDS_PACKAGE_AXIS, OPENHANDS_PACKAGE_VERSION, openhands_agent_server_claim,
    openhands_package_binding,
};

#[cfg(test)]
#[path = "../tests/driver_suite.rs"]
mod driver_suite;
#[cfg(test)]
#[path = "../tests/prepared_facade.rs"]
mod prepared_facade;
