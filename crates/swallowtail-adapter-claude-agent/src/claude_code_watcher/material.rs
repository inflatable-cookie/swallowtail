use swallowtail_runtime::{WATCHER_BRIDGE_TOOL_COMPLETION_GATE, WatcherBridgeBearer};

pub(super) const MCP_SERVER_NAME: &str = "swallowtail-watchers";
pub(super) const MCP_CONFIG_LOCATOR: &str = "mcp.json";
pub(super) const SETTINGS_LOCATOR: &str = "settings.json";
pub(super) const SKILL_LOCATOR: &str = ".claude/skills/swallowtail-watchers/SKILL.md";

pub(super) fn mcp_config(endpoint: &str, bearer: &WatcherBridgeBearer) -> String {
    serde_json::json!({
        "mcpServers": {
            MCP_SERVER_NAME: {
                "type": "http",
                "url": endpoint,
                "headers": {
                    "Authorization": format!("Bearer {}", bearer.expose()),
                }
            }
        }
    })
    .to_string()
}

pub(super) fn settings() -> String {
    serde_json::json!({
        "hooks": {
            "Stop": [{
                "hooks": [{
                    "type": "mcp_tool",
                    "server": MCP_SERVER_NAME,
                    "tool": WATCHER_BRIDGE_TOOL_COMPLETION_GATE,
                }]
            }]
        }
    })
    .to_string()
}

pub(super) fn skill_markdown() -> &'static str {
    "---\n\
     name: swallowtail-watchers\n\
     description: Start, inspect, wait for, and stop host-owned turn-scoped watchers.\n\
     ---\n\
     \n\
     Use the Swallowtail watcher MCP tools to start, inspect, list, wait for,\n\
     and stop host-owned background work for this turn. Watcher ids are\n\
     host-issued and never PIDs or provider task ids. Do not finish the turn\n\
     while any watcher is active or unjoined: wait or stop first. The Stop hook\n\
     returns remaining watcher state to this same turn.\n"
}
