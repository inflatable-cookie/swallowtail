//! Closed Contract 060 HTTP/MCP protocol constants.

use std::time::Duration;

/// JSON-RPC version admitted by the bridge.
pub const WATCHER_BRIDGE_JSONRPC_VERSION: &str = "2.0";
/// Exact MCP protocol revision admitted by the bridge.
pub const WATCHER_BRIDGE_MCP_PROTOCOL_VERSION: &str = "2025-03-26";
/// HTTP path bound on the lease endpoint.
pub const WATCHER_BRIDGE_HTTP_PATH: &str = "/mcp";
/// JSON-RPC initialize method.
pub const WATCHER_BRIDGE_INITIALIZE_METHOD: &str = "initialize";
/// JSON-RPC initialized notification.
pub const WATCHER_BRIDGE_INITIALIZED_NOTIFICATION: &str = "notifications/initialized";
/// JSON-RPC tool-list method.
pub const WATCHER_BRIDGE_TOOLS_LIST_METHOD: &str = "tools/list";
/// JSON-RPC tool-call method.
pub const WATCHER_BRIDGE_TOOLS_CALL_METHOD: &str = "tools/call";
/// Reserved start tool.
pub const WATCHER_BRIDGE_TOOL_START: &str = "swallowtail_watcher_start";
/// Reserved inspect tool.
pub const WATCHER_BRIDGE_TOOL_INSPECT: &str = "swallowtail_watcher_inspect";
/// Reserved list tool.
pub const WATCHER_BRIDGE_TOOL_LIST: &str = "swallowtail_watcher_list";
/// Reserved wait tool.
pub const WATCHER_BRIDGE_TOOL_WAIT: &str = "swallowtail_watcher_wait";
/// Reserved stop tool.
pub const WATCHER_BRIDGE_TOOL_STOP: &str = "swallowtail_watcher_stop";
/// Reserved completion-gate query.
pub const WATCHER_BRIDGE_TOOL_COMPLETION_GATE: &str = "swallowtail_completion_gate";
/// Cryptographic bearer length in bytes before hex encoding.
pub const WATCHER_BRIDGE_BEARER_BYTE_LEN: usize = 32;
/// Maximum HTTP body bytes admitted before decode.
pub const WATCHER_BRIDGE_MAX_BODY_BYTES: usize = 16_384;
/// Maximum HTTP header-block bytes admitted before decode.
pub const WATCHER_BRIDGE_MAX_HEADER_BYTES: usize = 4_096;
/// Maximum HTTP header count admitted before decode.
pub const WATCHER_BRIDGE_MAX_HEADER_COUNT: usize = 32;
/// Maximum concurrent accepted connections for one lease.
pub const WATCHER_BRIDGE_MAX_CONCURRENT_CONNECTIONS: usize = 8;
/// Maximum in-flight decoded requests for one lease.
pub const WATCHER_BRIDGE_MAX_IN_FLIGHT_REQUESTS: usize = 8;
/// Maximum retained request-correlation identities for one generation.
pub const WATCHER_BRIDGE_MAX_CORRELATION_IDS: usize = 256;
/// Positive wait-duration bound for one reserved wait call.
pub const WATCHER_BRIDGE_MAX_WAIT: Duration = Duration::from_secs(30);

/// Exact reserved tool family admitted by `tools/call`.
pub const WATCHER_BRIDGE_RESERVED_TOOLS: [&str; 6] = [
    WATCHER_BRIDGE_TOOL_START,
    WATCHER_BRIDGE_TOOL_INSPECT,
    WATCHER_BRIDGE_TOOL_LIST,
    WATCHER_BRIDGE_TOOL_WAIT,
    WATCHER_BRIDGE_TOOL_STOP,
    WATCHER_BRIDGE_TOOL_COMPLETION_GATE,
];
