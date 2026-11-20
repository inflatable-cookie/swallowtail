//! Consumer-declared HTTP MCP seam for `gemini-cli.acp`.
//!
//! Contract 063 admits one consumer-supplied streamable-HTTP placement.
//! Production `session/new` serializes it as the ACP `http` form, which the
//! provider maps onto its `httpUrl` transport with headers on
//! `requestInit.headers` (Research 351, `0.59.0`).

use crate::failure::failure;
use serde_json::{Value, json};
use std::fmt;
use swallowtail_acp_client::{
    AcpMcpDiagnostics, AcpRemoteMcpPlacement, redacted_mcp_servers_debug,
};
use swallowtail_runtime::RuntimeFailure;

/// Reserved ACP `mcpServers` name owned by this route.
pub const GEMINI_ACP_MCP_SERVER_NAME: &str = "swallowtail-gemini-acp";
/// Contract 061 presence token for the admitted HTTP placement.
pub const GEMINI_ACP_HTTP_MCP_PLACEMENT: &str = "consumer-supplied-http";

const MCP_DIAGNOSTICS: AcpMcpDiagnostics = AcpMcpDiagnostics {
    reserved_name: GEMINI_ACP_MCP_SERVER_NAME,
    name_collision,
    http_invalid,
};

/// Provider-representable consumer-supplied streamable-HTTP MCP placement.
///
/// URL and header values are consumer secrets: Debug, failures, and
/// projections never carry them.
#[derive(Clone, Eq, PartialEq)]
pub struct GeminiAcpHttpMcpPlacement {
    inner: AcpRemoteMcpPlacement,
}

impl GeminiAcpHttpMcpPlacement {
    /// Models the admitted `http` entry the consumer supplies.
    pub fn new(
        name: impl Into<String>,
        url: impl Into<String>,
        headers: impl Into<Vec<(String, String)>>,
    ) -> Self {
        Self {
            inner: AcpRemoteMcpPlacement::new(name, url, headers),
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

    /// Encodes this placement onto production `mcpServers`.
    ///
    /// The returned payload's `Debug` form redacts URL and header values. The
    /// wire JSON stays crate-private.
    pub fn to_production_mcp_servers(&self) -> Result<GeminiAcpEncodedMcpServers, RuntimeFailure> {
        Ok(GeminiAcpEncodedMcpServers::from_entry(
            self.to_acp_http_value()?,
        ))
    }

    pub(crate) fn to_acp_http_value(&self) -> Result<Value, RuntimeFailure> {
        self.inner.to_acp_http_value(&MCP_DIAGNOSTICS)
    }
}

impl fmt::Debug for GeminiAcpHttpMcpPlacement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GeminiAcpHttpMcpPlacement")
            .field("name", &self.inner.name())
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
pub struct GeminiAcpEncodedMcpServers {
    value: Value,
}

impl GeminiAcpEncodedMcpServers {
    fn from_entry(entry: Value) -> Self {
        Self {
            value: json!([entry]),
        }
    }
}

impl fmt::Debug for GeminiAcpEncodedMcpServers {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GeminiAcpEncodedMcpServers")
            .field("entries", &redacted_mcp_servers_debug(&self.value))
            .finish()
    }
}

/// Production `mcpServers` list: empty, or exactly one admitted HTTP entry.
pub(crate) fn production_mcp_servers(
    http: Option<&GeminiAcpHttpMcpPlacement>,
) -> Result<Value, RuntimeFailure> {
    swallowtail_acp_client::production_mcp_servers(
        http.map(|remote| &remote.inner),
        &MCP_DIAGNOSTICS,
    )
}

fn name_collision() -> RuntimeFailure {
    failure(
        "swallowtail.gemini.acp.mcp_name_collision",
        "Gemini ACP binds consumer MCP declarations to one route-owned server name",
    )
}

fn http_invalid() -> RuntimeFailure {
    failure(
        "swallowtail.gemini.acp.mcp_http_invalid",
        "Gemini ACP HTTP MCP declarations require a non-empty name, an absolute http or https URL, and well-formed header names",
    )
}
