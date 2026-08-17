//! DeepSeek Harness Web `/api` route.
//!
//! This module is deliberately separate from the JSON-RPC stdio route. It
//! owns one loopback `dsh web` process, speaks the host's HTTP/WebSocket API,
//! and admits only the method and frame subset frozen by card 222.

pub(crate) const WEB_DRIVER_ID: &str = "swallowtail.deepseek-harness.local-server";

mod protocol;
mod selection;
mod transport;

pub(crate) use transport::require_loopback_endpoint;

pub(crate) use protocol::{
    MuxFrame, WebMethod, decode_mux_frame, method_allowlist, parse_archive, parse_cancel,
    parse_fork, parse_history, parse_host_description, parse_models, parse_prompt, parse_search,
    parse_session_create, parse_session_list, parse_workspace_list, request_body,
};
pub(crate) use selection::{target_is_exact, validate_plan, web_claim};

pub use selection::{
    DEEPSEEK_HARNESS_WEB_EXECUTABLE_BASENAME, DEEPSEEK_HARNESS_WEB_RELEASE_AXIS,
    DEEPSEEK_HARNESS_WEB_RELEASE_VERSION, deepseek_harness_web_claim,
};

mod driver;
pub use driver::{
    DeepSeekHarnessWebDriver, DeepSeekHarnessWebModel, deepseek_harness_web_descriptor,
};
