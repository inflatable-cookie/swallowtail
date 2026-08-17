//! DeepSeek Harness Web `/api` route.
//!
//! This module is deliberately separate from the JSON-RPC stdio route. It
//! owns one loopback `dsh web` process, speaks the host's HTTP/WebSocket API,
//! and admits only the method and frame subset frozen by card 222.

pub(crate) const WEB_DRIVER_ID: &str = "swallowtail.deepseek-harness.local-server";

mod protocol;
mod selection;
mod transport;

pub(crate) use protocol::{
    MuxFrame, WebMethod, decode_mux_frame, parse_archive, parse_cancel, parse_fork, parse_history,
    parse_prompt, parse_search, parse_session_create, parse_session_list, request_body,
};
pub(crate) use selection::{target_is_exact, validate_plan, web_claim};

pub use selection::{
    DEEPSEEK_HARNESS_WEB_RELEASE_AXIS, DEEPSEEK_HARNESS_WEB_RELEASE_VERSION,
    deepseek_harness_web_claim,
};

mod driver;
pub use driver::{DeepSeekHarnessWebDriver, deepseek_harness_web_descriptor};
