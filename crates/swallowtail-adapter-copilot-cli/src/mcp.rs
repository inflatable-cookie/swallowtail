//! Consumer-declared MCP seam for `copilot-cli.acp`.
//!
//! Contract 063 admits one consumer-supplied streamable-HTTP placement.
//! Production `session/new` serializes that shape, bound to one route-owned
//! name. Stdio client entries are not offered: Research 351 shows the
//! provider rejects them. The provider's `sse` form stays modelled and is
//! not emitted.

use crate::failure::failure;
use serde_json::{Value, json};
use std::fmt;
use swallowtail_acp_client::{
    AcpMcpDiagnostics, AcpRemoteMcpPlacement, redacted_mcp_servers_debug,
};
use swallowtail_runtime::RuntimeFailure;

/// Reserved ACP `mcpServers` name owned by this route.
pub const COPILOT_CLI_ACP_MCP_SERVER_NAME: &str = "swallowtail-copilot-cli-acp";
/// Contract 061 presence token for the admitted HTTP placement.
pub const COPILOT_CLI_ACP_HTTP_MCP_PLACEMENT: &str = "consumer-supplied-http";

const MCP_DIAGNOSTICS: AcpMcpDiagnostics = AcpMcpDiagnostics {
    reserved_name: COPILOT_CLI_ACP_MCP_SERVER_NAME,
    name_collision,
    http_invalid,
};

/// ACP URL-plus-header transport the provider can represent.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CopilotCliAcpRemoteMcpTransport {
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
pub struct CopilotCliAcpRemoteMcpPlacement {
    inner: AcpRemoteMcpPlacement,
    transport: CopilotCliAcpRemoteMcpTransport,
}

impl CopilotCliAcpRemoteMcpPlacement {
    /// Models an admitted `http` entry the consumer supplies.
    pub fn new(
        name: impl Into<String>,
        url: impl Into<String>,
        headers: impl Into<Vec<(String, String)>>,
    ) -> Self {
        Self {
            inner: AcpRemoteMcpPlacement::new(name, url, headers),
            transport: CopilotCliAcpRemoteMcpTransport::Http,
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
            transport: CopilotCliAcpRemoteMcpTransport::Sse,
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
    pub const fn transport(&self) -> CopilotCliAcpRemoteMcpTransport {
        self.transport
    }

    /// Encodes this placement onto production `mcpServers` when it is the admitted `http` form.
    ///
    /// The returned payload's `Debug` form redacts URL and header values. The
    /// wire JSON stays crate-private.
    pub fn to_production_mcp_servers(
        &self,
    ) -> Result<CopilotCliAcpEncodedMcpServers, RuntimeFailure> {
        Ok(CopilotCliAcpEncodedMcpServers::from_entry(
            self.to_acp_http_value()?,
        ))
    }

    pub(crate) fn to_acp_http_value(&self) -> Result<Value, RuntimeFailure> {
        if self.transport != CopilotCliAcpRemoteMcpTransport::Http {
            return Err(failure(
                "swallowtail.copilot-cli.acp.mcp_sse_unsupported",
                "Copilot CLI ACP does not emit sse MCP entries on this route",
            ));
        }
        self.inner.to_acp_http_value(&MCP_DIAGNOSTICS)
    }
}

impl fmt::Debug for CopilotCliAcpRemoteMcpPlacement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CopilotCliAcpRemoteMcpPlacement")
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
pub struct CopilotCliAcpEncodedMcpServers {
    value: Value,
}

impl CopilotCliAcpEncodedMcpServers {
    fn from_entry(entry: Value) -> Self {
        Self {
            value: json!([entry]),
        }
    }
}

impl fmt::Debug for CopilotCliAcpEncodedMcpServers {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CopilotCliAcpEncodedMcpServers")
            .field("entries", &redacted_mcp_servers_debug(&self.value))
            .finish()
    }
}

/// Production `mcpServers` list: empty, or exactly one admitted HTTP entry.
pub(crate) fn production_mcp_servers(
    http: Option<&CopilotCliAcpRemoteMcpPlacement>,
) -> Result<Value, RuntimeFailure> {
    match http {
        None => Ok(json!([])),
        Some(remote) => Ok(json!([remote.to_acp_http_value()?])),
    }
}

fn name_collision() -> RuntimeFailure {
    failure(
        "swallowtail.copilot-cli.acp.mcp_name_collision",
        "Copilot CLI ACP binds consumer MCP declarations to one route-owned server name",
    )
}

fn http_invalid() -> RuntimeFailure {
    failure(
        "swallowtail.copilot-cli.acp.mcp_http_invalid",
        "Copilot CLI ACP HTTP MCP declarations require a non-empty name, an absolute http or https URL, and well-formed header names",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const CANARY_URL: &str = "http://127.0.0.1:9/mcp/g06-038-redaction-canary";
    const CANARY_HEADER: &str = "Bearer g06-038-redaction-canary";

    #[test]
    fn omission_stays_an_empty_list() {
        assert_eq!(production_mcp_servers(None).expect("omission"), json!([]));
    }

    #[test]
    fn http_encoder_and_sse_refusal() {
        let remote = CopilotCliAcpRemoteMcpPlacement::new(
            COPILOT_CLI_ACP_MCP_SERVER_NAME,
            CANARY_URL,
            vec![("Authorization".to_owned(), CANARY_HEADER.to_owned())],
        );
        let encoded = remote.to_production_mcp_servers().expect("http admitted");
        assert!(!format!("{encoded:?}").contains(CANARY_URL));
        assert!(!format!("{encoded:?}").contains(CANARY_HEADER));

        let sse = CopilotCliAcpRemoteMcpPlacement::sse(
            COPILOT_CLI_ACP_MCP_SERVER_NAME,
            CANARY_URL,
            Vec::<(String, String)>::new(),
        );
        let error = sse.to_production_mcp_servers().expect_err("sse refused");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.copilot-cli.acp.mcp_sse_unsupported"
        );
    }
}
