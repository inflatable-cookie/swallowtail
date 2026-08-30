//! Optional Contract 060 watcher HTTP-bridge host port.
//!
//! Registration binds no listener and starts no work. Opening a lease binds
//! one exact host, operation, turn, and watcher service.

mod identity;
mod lease;
mod protocol;
mod service;

pub use identity::{
    WatcherBridgeAdmission, WatcherBridgeBearer, WatcherBridgeEndpoint, WatcherBridgeGeneration,
    WatcherBridgeToken,
};
pub use lease::{WatcherBridgeCompletionState, WatcherBridgeLease, WatcherBridgeOpenRequest};
pub use protocol::{
    WATCHER_BRIDGE_BEARER_BYTE_LEN, WATCHER_BRIDGE_HTTP_PATH, WATCHER_BRIDGE_INITIALIZE_METHOD,
    WATCHER_BRIDGE_INITIALIZED_NOTIFICATION, WATCHER_BRIDGE_JSONRPC_VERSION,
    WATCHER_BRIDGE_MAX_BODY_BYTES, WATCHER_BRIDGE_MAX_CONCURRENT_CONNECTIONS,
    WATCHER_BRIDGE_MAX_CORRELATION_IDS, WATCHER_BRIDGE_MAX_HEADER_BYTES,
    WATCHER_BRIDGE_MAX_HEADER_COUNT, WATCHER_BRIDGE_MAX_IN_FLIGHT_REQUESTS,
    WATCHER_BRIDGE_MCP_PROTOCOL_VERSION, WATCHER_BRIDGE_RESERVED_TOOLS,
    WATCHER_BRIDGE_TOOL_COMPLETION_GATE, WATCHER_BRIDGE_TOOL_INSPECT, WATCHER_BRIDGE_TOOL_LIST,
    WATCHER_BRIDGE_TOOL_START, WATCHER_BRIDGE_TOOL_STOP, WATCHER_BRIDGE_TOOL_WAIT,
    WATCHER_BRIDGE_TOOLS_CALL_METHOD, WATCHER_BRIDGE_TOOLS_LIST_METHOD,
};
pub use service::WatcherBridgeHostService;
