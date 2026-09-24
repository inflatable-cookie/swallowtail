//! Consumer-declared MCP seam for `opencode.acp`.
//!
//! Contract 063 already admits a stdio placement. Production `session/new`
//! serializes only that shape, bound to one route-owned name. The URL-plus-header
//! `http`/`sse` shape is modelled here and cannot be encoded onto the production
//! seam until a contract admits it.

use serde_json::{Value, json};
use swallowtail_runtime::RuntimeFailure;

use super::failure::failure;

/// Reserved ACP `mcpServers` name owned by this route.
pub const OPENCODE_ACP_MCP_SERVER_NAME: &str = "swallowtail-opencode-acp";

/// One admitted stdio MCP declaration for production `session/new`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenCodeAcpStdioMcpServer {
    name: String,
    command: String,
    arguments: Vec<String>,
    environment: Vec<(String, String)>,
}

impl OpenCodeAcpStdioMcpServer {
    /// Creates a stdio declaration. Production encoding requires the route-owned name.
    pub fn new(
        name: impl Into<String>,
        command: impl Into<String>,
        arguments: impl Into<Vec<String>>,
        environment: impl Into<Vec<(String, String)>>,
    ) -> Self {
        Self {
            name: name.into(),
            command: command.into(),
            arguments: arguments.into(),
            environment: environment.into(),
        }
    }

    /// Returns the declared ACP server name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Renders the ACP v1 stdio object admitted on this route.
    pub(crate) fn to_acp_value(&self) -> Result<Value, RuntimeFailure> {
        if self.name != OPENCODE_ACP_MCP_SERVER_NAME {
            return Err(failure(
                "swallowtail.opencode.acp.mcp_name_collision",
                "OpenCode ACP binds consumer MCP declarations to one route-owned server name",
            ));
        }
        if self.command.is_empty() {
            return Err(failure(
                "swallowtail.opencode.acp.mcp_stdio_invalid",
                "OpenCode ACP stdio MCP declarations require a command",
            ));
        }
        Ok(json!({
            "name": self.name,
            "command": self.command,
            "args": self.arguments,
            "env": self
                .environment
                .iter()
                .map(|(name, value)| json!({"name": name, "value": value}))
                .collect::<Vec<_>>(),
        }))
    }
}

/// Provider-representable URL-plus-header MCP placement. Not admitted by Contract 063.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenCodeAcpRemoteMcpPlacement {
    name: String,
    url: String,
    headers: Vec<(String, String)>,
}

impl OpenCodeAcpRemoteMcpPlacement {
    /// Models an `http`/`sse` entry the provider can represent.
    pub fn new(
        name: impl Into<String>,
        url: impl Into<String>,
        headers: impl Into<Vec<(String, String)>>,
    ) -> Self {
        Self {
            name: name.into(),
            url: url.into(),
            headers: headers.into(),
        }
    }

    /// Returns the modelled server name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the modelled URL.
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Returns the modelled headers.
    #[must_use]
    pub fn headers(&self) -> &[(String, String)] {
        &self.headers
    }

    /// Refuses production encoding until a contract admits URL-plus-header placement.
    pub fn to_production_mcp_servers(&self) -> Result<Value, RuntimeFailure> {
        let _ = (self.url.as_str(), self.headers.as_slice());
        Err(failure(
            "swallowtail.opencode.acp.mcp_remote_not_admitted",
            "OpenCode ACP does not wire a URL-plus-header MCP placement until a contract admits it",
        ))
    }
}

/// Production `mcpServers` list: empty, or exactly one admitted stdio entry.
pub(crate) fn production_mcp_servers(
    stdio: Option<&OpenCodeAcpStdioMcpServer>,
) -> Result<Value, RuntimeFailure> {
    match stdio {
        None => Ok(json!([])),
        Some(server) => Ok(json!([server.to_acp_value()?])),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_seam_accepts_the_route_owned_stdio_name_only() {
        let admitted = OpenCodeAcpStdioMcpServer::new(
            OPENCODE_ACP_MCP_SERVER_NAME,
            "/usr/bin/echo-mcp",
            vec!["--stdio".to_owned()],
            vec![("SWALLOWTAIL".to_owned(), "1".to_owned())],
        );
        let value = production_mcp_servers(Some(&admitted)).expect("stdio is admitted");
        assert_eq!(value[0]["name"], OPENCODE_ACP_MCP_SERVER_NAME);
        assert_eq!(value[0]["command"], "/usr/bin/echo-mcp");
        assert!(value[0].get("url").is_none());
        assert!(value[0].get("headers").is_none());

        let collision = OpenCodeAcpStdioMcpServer::new(
            "other-server",
            "/usr/bin/echo-mcp",
            Vec::<String>::new(),
            Vec::<(String, String)>::new(),
        );
        let error = production_mcp_servers(Some(&collision)).expect_err("name is reserved");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.opencode.acp.mcp_name_collision"
        );
        assert_eq!(production_mcp_servers(None).expect("omission"), json!([]));
    }

    #[test]
    fn url_plus_header_shape_exists_and_cannot_reach_production() {
        let remote = OpenCodeAcpRemoteMcpPlacement::new(
            OPENCODE_ACP_MCP_SERVER_NAME,
            "http://127.0.0.1:9/mcp",
            vec![("Authorization".to_owned(), "Bearer secret".to_owned())],
        );
        let error = remote
            .to_production_mcp_servers()
            .expect_err("remote placement stays gated");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.opencode.acp.mcp_remote_not_admitted"
        );
        assert_eq!(remote.url(), "http://127.0.0.1:9/mcp");
        assert!(!error.diagnostic().message().contains("Bearer secret"));
    }
}
