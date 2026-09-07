//! The exact ACP `mcpServers` entry this route declares for the courier.
//!
//! ACP v1 session setup carries a client-supplied `mcpServers` list, and the
//! exact Grok route was observed admitting one, spawning the described stdio
//! server, connecting to it, and listing its tools. This type is that one
//! entry and nothing else: a reserved name, the host-approved courier path,
//! the fixed wire tag with a non-authoritative rendezvous path, and the
//! allowlisted environment. No endpoint, bearer, or generation is on this
//! wire; those stay in the private rendezvous file.

use serde_json::{Value, json};

/// One client-supplied ACP stdio MCP server declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrokAcpMcpServerDeclaration {
    name: &'static str,
    command: String,
    arguments: Vec<String>,
    environment: Vec<(String, String)>,
}

impl GrokAcpMcpServerDeclaration {
    pub(crate) fn courier(
        name: &'static str,
        command: String,
        arguments: Vec<String>,
        environment: Vec<(String, String)>,
    ) -> Self {
        Self {
            name,
            command,
            arguments,
            environment,
        }
    }

    /// Returns the reserved ACP server name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// Returns the host-approved absolute courier command.
    #[must_use]
    pub fn command(&self) -> &str {
        &self.command
    }

    /// Returns the exact declared arguments.
    #[must_use]
    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }

    /// Returns the allowlisted environment bindings, in declaration order.
    #[must_use]
    pub fn environment(&self) -> &[(String, String)] {
        &self.environment
    }

    /// Renders the exact ACP v1 stdio server object.
    ///
    /// ACP carries `env` as a list of `{name, value}` objects, which is the
    /// shape the exact route was observed admitting; it is not a map.
    pub(crate) fn to_acp_value(&self) -> Value {
        json!({
            "name": self.name,
            "command": self.command,
            "args": self.arguments,
            "env": self
                .environment
                .iter()
                .map(|(name, value)| json!({"name": name, "value": value}))
                .collect::<Vec<_>>(),
        })
    }
}
