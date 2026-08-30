//! Provider-neutral operation-scoped watcher lifecycle core.
//!
//! This module owns identity correlation, pure registry transitions, distinct
//! model and operator control roles, optional host-service registration, and
//! turn activity projection. It does not start processes or select a route.

mod activity;
mod bridge;
mod control;
mod host_service;
mod registry;

pub use activity::{
    WatcherActivityProjection, WatcherActivityProjectionFailure, project_watcher_activity,
};
pub use bridge::{
    WATCHER_BRIDGE_BEARER_BYTE_LEN, WATCHER_BRIDGE_HTTP_PATH, WATCHER_BRIDGE_INITIALIZE_METHOD,
    WATCHER_BRIDGE_INITIALIZED_NOTIFICATION, WATCHER_BRIDGE_JSONRPC_VERSION,
    WATCHER_BRIDGE_MAX_BODY_BYTES, WATCHER_BRIDGE_MAX_CONCURRENT_CONNECTIONS,
    WATCHER_BRIDGE_MAX_CORRELATION_IDS, WATCHER_BRIDGE_MAX_HEADER_BYTES,
    WATCHER_BRIDGE_MAX_HEADER_COUNT, WATCHER_BRIDGE_MAX_IN_FLIGHT_REQUESTS,
    WATCHER_BRIDGE_MCP_PROTOCOL_VERSION, WATCHER_BRIDGE_RESERVED_TOOLS,
    WATCHER_BRIDGE_TOOL_COMPLETION_GATE, WATCHER_BRIDGE_TOOL_INSPECT, WATCHER_BRIDGE_TOOL_LIST,
    WATCHER_BRIDGE_TOOL_START, WATCHER_BRIDGE_TOOL_STOP, WATCHER_BRIDGE_TOOL_WAIT,
    WATCHER_BRIDGE_TOOLS_CALL_METHOD, WATCHER_BRIDGE_TOOLS_LIST_METHOD, WatcherBridgeAdmission,
    WatcherBridgeBearer, WatcherBridgeCompletionState, WatcherBridgeEndpoint,
    WatcherBridgeGeneration, WatcherBridgeHostService, WatcherBridgeLease,
    WatcherBridgeOpenRequest, WatcherBridgeToken,
};
pub use control::{
    ModelWatcherControl, ModelWatcherRole, OperatorWatcherControl, OperatorWatcherRole,
    SharedWatcherRegistry, WatcherControlSurface,
};
pub use host_service::{WatcherHostService, WatcherWaitOptions};
pub use registry::{
    WatcherFailure, WatcherFailureKind, WatcherRegistry, WatcherSnapshot,
    WatcherStopAcknowledgement, WatcherWaitRepresentation,
};
