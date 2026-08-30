//! Contract 060 portable watcher-bridge assertions.

use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    RuntimeTurnId, ScopeId, WATCHER_BRIDGE_HTTP_PATH, WATCHER_BRIDGE_MCP_PROTOCOL_VERSION,
    WATCHER_BRIDGE_RESERVED_TOOLS, WATCHER_BRIDGE_TOOL_COMPLETION_GATE,
    WATCHER_BRIDGE_TOOL_INSPECT, WATCHER_BRIDGE_TOOL_LIST, WATCHER_BRIDGE_TOOL_START,
    WATCHER_BRIDGE_TOOL_STOP, WATCHER_BRIDGE_TOOL_WAIT, WatcherBridgeBearer, WatcherBridgeEndpoint,
    WatcherBridgeGeneration, WatcherBridgeLease,
};

const BRIDGE_RULE: &str = "Contract 060 watcher HTTP bridge";

/// Proves driver-only endpoint and bearer values stay out of default formatting.
pub fn assert_watcher_bridge_secret_redaction() {
    let endpoint = WatcherBridgeEndpoint::new("http://127.0.0.1:54321/mcp").expect("endpoint");
    let bearer = WatcherBridgeBearer::new("bridge-secret-token").expect("bearer");
    let lease = WatcherBridgeLease::new(
        ExecutionHostId::new("host.local").expect("host"),
        ScopeId::new("scope-1").expect("scope"),
        RuntimeTurnId::new("turn-1").expect("turn"),
        WatcherBridgeGeneration::initial(),
        endpoint,
        bearer,
    );
    let debug = format!("{lease:?}");
    assert!(
        !debug.contains("127.0.0.1"),
        "{BRIDGE_RULE}: lease debug exposed the endpoint host"
    );
    assert!(
        !debug.contains("54321"),
        "{BRIDGE_RULE}: lease debug exposed the endpoint port"
    );
    assert!(
        !debug.contains("bridge-secret-token"),
        "{BRIDGE_RULE}: lease debug exposed the bearer"
    );
    assert!(
        !debug.contains(WATCHER_BRIDGE_HTTP_PATH),
        "{BRIDGE_RULE}: lease debug exposed the HTTP path"
    );
    assert_eq!(
        format!("{}", lease.endpoint()),
        "<redacted watcher bridge endpoint>"
    );
    assert_eq!(
        format!("{}", lease.bearer()),
        "<redacted watcher bridge bearer>"
    );
    assert_eq!(lease.endpoint().expose(), "http://127.0.0.1:54321/mcp");
    assert!(lease.bearer().matches("bridge-secret-token"));
    assert!(!lease.bearer().matches("other-secret-token"));
}

/// Proves the closed reserved tool family and protocol revision are exact.
pub fn assert_watcher_bridge_closed_protocol_surface() {
    assert_eq!(WATCHER_BRIDGE_MCP_PROTOCOL_VERSION, "2025-03-26");
    assert_eq!(WATCHER_BRIDGE_HTTP_PATH, "/mcp");
    assert_eq!(
        WATCHER_BRIDGE_RESERVED_TOOLS,
        [
            WATCHER_BRIDGE_TOOL_START,
            WATCHER_BRIDGE_TOOL_INSPECT,
            WATCHER_BRIDGE_TOOL_LIST,
            WATCHER_BRIDGE_TOOL_WAIT,
            WATCHER_BRIDGE_TOOL_STOP,
            WATCHER_BRIDGE_TOOL_COMPLETION_GATE,
        ]
    );
}

/// Runs the Contract 060 portable watcher-bridge assertion pack.
pub fn assert_portable_watcher_bridge_contract() {
    assert_watcher_bridge_secret_redaction();
    assert_watcher_bridge_closed_protocol_surface();
}

#[cfg(test)]
mod tests {
    use super::assert_portable_watcher_bridge_contract;

    #[test]
    fn portable_watcher_bridge_contract_holds() {
        assert_portable_watcher_bridge_contract();
    }
}
