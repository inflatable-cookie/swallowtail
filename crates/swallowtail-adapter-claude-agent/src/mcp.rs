//! Consumer-declared MCP seam for `claude-agent.acp`.
//!
//! Contract 063 admits one consumer-supplied streamable-HTTP placement.
//! Production `session/new` serializes that shape, bound to one route-owned
//! name. The provider also maps `stdio` and `sse`; this route emits `http`
//! only.

use crate::failure::failure;
use serde_json::{Value, json};
use std::fmt;
use swallowtail_acp_client::{
    AcpMcpDiagnostics, AcpRemoteMcpPlacement, redacted_mcp_servers_debug,
};
use swallowtail_runtime::RuntimeFailure;

/// Reserved ACP `mcpServers` name owned by this route.
pub const CLAUDE_AGENT_ACP_MCP_SERVER_NAME: &str = "swallowtail-claude-agent-acp";
/// Contract 061 presence token for the admitted HTTP placement.
pub const CLAUDE_AGENT_ACP_HTTP_MCP_PLACEMENT: &str = "consumer-supplied-http";

const MCP_DIAGNOSTICS: AcpMcpDiagnostics = AcpMcpDiagnostics {
    reserved_name: CLAUDE_AGENT_ACP_MCP_SERVER_NAME,
    name_collision,
    http_invalid,
};

/// ACP URL-plus-header transport the provider can represent.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClaudeAgentAcpRemoteMcpTransport {
    /// Streamable-HTTP form admitted on production `session/new`.
    Http,
    /// SSE form kept modelled and unemitted.
    Sse,
}

/// Provider-representable URL-plus-header MCP placement.
///
/// URL and header values are consumer secrets: Debug, failures, and
/// projections never carry them.
#[derive(Clone, Eq, PartialEq)]
pub struct ClaudeAgentAcpRemoteMcpPlacement {
    inner: AcpRemoteMcpPlacement,
    transport: ClaudeAgentAcpRemoteMcpTransport,
}

impl ClaudeAgentAcpRemoteMcpPlacement {
    /// Models an admitted `http` entry the consumer supplies.
    pub fn new(
        name: impl Into<String>,
        url: impl Into<String>,
        headers: impl Into<Vec<(String, String)>>,
    ) -> Self {
        Self {
            inner: AcpRemoteMcpPlacement::new(name, url, headers),
            transport: ClaudeAgentAcpRemoteMcpTransport::Http,
        }
    }

    /// Models an `sse` entry. Production encoding refuses it.
    pub fn sse(
        name: impl Into<String>,
        url: impl Into<String>,
        headers: impl Into<Vec<(String, String)>>,
    ) -> Self {
        Self {
            inner: AcpRemoteMcpPlacement::new(name, url, headers),
            transport: ClaudeAgentAcpRemoteMcpTransport::Sse,
        }
    }

    /// Returns the modelled server name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.inner.name()
    }

    /// Returns the modelled URL. Callers that log or project this value leak a secret.
    #[must_use]
    pub fn url(&self) -> &str {
        self.inner.url()
    }

    /// Returns the modelled headers. Values are consumer secrets.
    #[must_use]
    pub fn headers(&self) -> &[(String, String)] {
        self.inner.headers()
    }

    /// Returns the modelled ACP transport discriminant.
    #[must_use]
    pub const fn transport(&self) -> ClaudeAgentAcpRemoteMcpTransport {
        self.transport
    }

    /// Encodes this placement onto production `mcpServers` when it is the admitted `http` form.
    ///
    /// The returned payload's `Debug` form redacts URL and header values. The
    /// wire JSON stays crate-private.
    pub fn to_production_mcp_servers(
        &self,
    ) -> Result<ClaudeAgentAcpEncodedMcpServers, RuntimeFailure> {
        Ok(ClaudeAgentAcpEncodedMcpServers::from_entry(
            self.to_acp_http_value()?,
        ))
    }

    pub(crate) fn to_acp_http_value(&self) -> Result<Value, RuntimeFailure> {
        if self.transport != ClaudeAgentAcpRemoteMcpTransport::Http {
            return Err(failure(
                "swallowtail.claude_agent.acp.mcp_sse_not_emitted",
                "Claude Agent ACP models sse MCP entries but emits the http form only",
            ));
        }
        self.inner.to_acp_http_value(&MCP_DIAGNOSTICS)
    }
}

impl fmt::Debug for ClaudeAgentAcpRemoteMcpPlacement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ClaudeAgentAcpRemoteMcpPlacement")
            .field("name", &self.inner.name())
            .field("transport", &self.transport)
            .field("url", &"<redacted>")
            .field(
                "headers",
                &self
                    .inner
                    .headers()
                    .iter()
                    .map(|(name, _)| (name.as_str(), "<redacted>"))
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

/// Production `mcpServers` payload. Debug redacts URL and header values.
pub struct ClaudeAgentAcpEncodedMcpServers {
    value: Value,
}

impl ClaudeAgentAcpEncodedMcpServers {
    fn from_entry(entry: Value) -> Self {
        Self {
            value: json!([entry]),
        }
    }
}

impl fmt::Debug for ClaudeAgentAcpEncodedMcpServers {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ClaudeAgentAcpEncodedMcpServers")
            .field("entries", &redacted_mcp_servers_debug(&self.value))
            .finish()
    }
}

/// Production `mcpServers` list: empty, or exactly one admitted HTTP entry.
pub(crate) fn production_mcp_servers(
    http: Option<&ClaudeAgentAcpRemoteMcpPlacement>,
) -> Result<Value, RuntimeFailure> {
    match http {
        None => Ok(json!([])),
        Some(remote) => Ok(json!([remote.to_acp_http_value()?])),
    }
}

fn name_collision() -> RuntimeFailure {
    failure(
        "swallowtail.claude_agent.acp.mcp_name_collision",
        "Claude Agent ACP binds consumer MCP declarations to one route-owned server name",
    )
}

fn http_invalid() -> RuntimeFailure {
    failure(
        "swallowtail.claude_agent.acp.mcp_http_invalid",
        "Claude Agent ACP HTTP MCP declarations require a non-empty name, an absolute http or https URL, and well-formed header names",
    )
}
