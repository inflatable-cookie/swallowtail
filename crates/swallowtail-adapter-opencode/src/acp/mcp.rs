//! Consumer-declared MCP seam for `opencode.acp`.
//!
//! Contract 063 admits one stdio placement and one consumer-supplied
//! streamable-HTTP placement. Production `session/new` serializes exactly one
//! of those shapes, bound to one route-owned name. The provider's `sse` form
//! stays modelled and is not emitted.

use serde_json::{Value, json};
use std::fmt;
use swallowtail_runtime::RuntimeFailure;
use url::Url;

use super::failure::failure;

/// Reserved ACP `mcpServers` name owned by this route.
pub const OPENCODE_ACP_MCP_SERVER_NAME: &str = "swallowtail-opencode-acp";
/// Contract 061 presence token for the admitted HTTP placement.
pub const OPENCODE_ACP_HTTP_MCP_PLACEMENT: &str = "consumer-supplied-http";

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
            return Err(name_collision());
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

/// ACP URL-plus-header transport the provider can represent.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpenCodeAcpRemoteMcpTransport {
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
pub struct OpenCodeAcpRemoteMcpPlacement {
    name: String,
    url: String,
    headers: Vec<(String, String)>,
    transport: OpenCodeAcpRemoteMcpTransport,
}

impl OpenCodeAcpRemoteMcpPlacement {
    /// Models an admitted `http` entry the consumer supplies.
    pub fn new(
        name: impl Into<String>,
        url: impl Into<String>,
        headers: impl Into<Vec<(String, String)>>,
    ) -> Self {
        Self {
            name: name.into(),
            url: url.into(),
            headers: headers.into(),
            transport: OpenCodeAcpRemoteMcpTransport::Http,
        }
    }

    /// Models an `sse` entry. Production encoding refuses it.
    pub fn sse(
        name: impl Into<String>,
        url: impl Into<String>,
        headers: impl Into<Vec<(String, String)>>,
    ) -> Self {
        Self {
            name: name.into(),
            url: url.into(),
            headers: headers.into(),
            transport: OpenCodeAcpRemoteMcpTransport::Sse,
        }
    }

    /// Returns the modelled server name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the modelled URL. Callers that log or project this value leak a secret.
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Returns the modelled headers. Values are consumer secrets.
    #[must_use]
    pub fn headers(&self) -> &[(String, String)] {
        &self.headers
    }

    /// Returns the modelled ACP transport discriminant.
    #[must_use]
    pub const fn transport(&self) -> OpenCodeAcpRemoteMcpTransport {
        self.transport
    }

    /// Encodes this placement onto production `mcpServers` when it is the admitted `http` form.
    pub fn to_production_mcp_servers(&self) -> Result<Value, RuntimeFailure> {
        Ok(json!([self.to_acp_http_value()?]))
    }

    pub(crate) fn to_acp_http_value(&self) -> Result<Value, RuntimeFailure> {
        if self.transport != OpenCodeAcpRemoteMcpTransport::Http {
            return Err(failure(
                "swallowtail.opencode.acp.mcp_sse_not_emitted",
                "OpenCode ACP models sse MCP entries but emits the http form only",
            ));
        }
        if self.name.is_empty() {
            return Err(http_invalid());
        }
        if self.name != OPENCODE_ACP_MCP_SERVER_NAME {
            return Err(name_collision());
        }
        if !absolute_http_url(&self.url) {
            return Err(http_invalid());
        }
        if self
            .headers
            .iter()
            .any(|(name, _)| !header_name_is_well_formed(name))
        {
            return Err(http_invalid());
        }
        Ok(json!({
            "type": "http",
            "name": self.name,
            "url": self.url,
            "headers": self
                .headers
                .iter()
                .map(|(name, value)| json!({"name": name, "value": value}))
                .collect::<Vec<_>>(),
        }))
    }
}

impl fmt::Debug for OpenCodeAcpRemoteMcpPlacement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpenCodeAcpRemoteMcpPlacement")
            .field("name", &self.name)
            .field("transport", &self.transport)
            .field("url", &"<redacted>")
            .field(
                "headers",
                &self
                    .headers
                    .iter()
                    .map(|(name, _)| (name.as_str(), "<redacted>"))
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

/// Production `mcpServers` list: empty, or exactly one admitted stdio or HTTP entry.
pub(crate) fn production_mcp_servers(
    stdio: Option<&OpenCodeAcpStdioMcpServer>,
    http: Option<&OpenCodeAcpRemoteMcpPlacement>,
) -> Result<Value, RuntimeFailure> {
    match (stdio, http) {
        (None, None) => Ok(json!([])),
        (Some(server), None) => Ok(json!([server.to_acp_value()?])),
        (None, Some(remote)) => Ok(json!([remote.to_acp_http_value()?])),
        (Some(_), Some(_)) => Err(entry_conflict()),
    }
}

pub(crate) fn entry_conflict() -> RuntimeFailure {
    failure(
        "swallowtail.opencode.acp.mcp_entry_conflict",
        "OpenCode ACP admits one consumer MCP entry per session, stdio or HTTP",
    )
}

fn name_collision() -> RuntimeFailure {
    failure(
        "swallowtail.opencode.acp.mcp_name_collision",
        "OpenCode ACP binds consumer MCP declarations to one route-owned server name",
    )
}

fn http_invalid() -> RuntimeFailure {
    failure(
        "swallowtail.opencode.acp.mcp_http_invalid",
        "OpenCode ACP HTTP MCP declarations require a non-empty name, an absolute http or https URL, and well-formed header names",
    )
}

fn absolute_http_url(raw: &str) -> bool {
    Url::parse(raw)
        .is_ok_and(|parsed| matches!(parsed.scheme(), "http" | "https") && parsed.has_host())
}

fn header_name_is_well_formed(name: &str) -> bool {
    !name.is_empty()
        && name.bytes().all(|byte| {
            matches!(
                byte,
                b'!'
                    | b'#'
                    | b'$'
                    | b'%'
                    | b'&'
                    | b'\''
                    | b'*'
                    | b'+'
                    | b'-'
                    | b'.'
                    | b'^'
                    | b'_'
                    | b'`'
                    | b'|'
                    | b'~'
                    | b'0'..=b'9'
                    | b'A'..=b'Z'
                    | b'a'..=b'z'
            )
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const CANARY_URL: &str = "http://127.0.0.1:9/mcp/g06-019-redaction-canary";
    const CANARY_HEADER: &str = "Bearer g06-019-redaction-canary";

    fn admitted_http() -> OpenCodeAcpRemoteMcpPlacement {
        OpenCodeAcpRemoteMcpPlacement::new(
            OPENCODE_ACP_MCP_SERVER_NAME,
            CANARY_URL,
            vec![("Authorization".to_owned(), CANARY_HEADER.to_owned())],
        )
    }

    fn assert_secret_redacted(value: &impl fmt::Debug) {
        let rendered = format!("{value:?}");
        assert!(
            !rendered.contains(CANARY_URL),
            "debug leaked the URL: {rendered}"
        );
        assert!(
            !rendered.contains(CANARY_HEADER),
            "debug leaked a header value: {rendered}"
        );
        assert!(
            !rendered.contains("g06-019-redaction-canary"),
            "debug leaked the canary: {rendered}"
        );
    }

    #[test]
    fn production_seam_accepts_the_route_owned_stdio_name_only() {
        let admitted = OpenCodeAcpStdioMcpServer::new(
            OPENCODE_ACP_MCP_SERVER_NAME,
            "/usr/bin/echo-mcp",
            vec!["--stdio".to_owned()],
            vec![("SWALLOWTAIL".to_owned(), "1".to_owned())],
        );
        let value = production_mcp_servers(Some(&admitted), None).expect("stdio is admitted");
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
        let error = production_mcp_servers(Some(&collision), None).expect_err("name is reserved");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.opencode.acp.mcp_name_collision"
        );
        assert_eq!(
            production_mcp_servers(None, None).expect("omission"),
            json!([])
        );
    }

    #[test]
    fn http_encoder_passes_url_and_headers_verbatim() {
        let remote = admitted_http();
        let value = remote
            .to_production_mcp_servers()
            .expect("http is admitted");
        assert_eq!(value[0]["type"], "http");
        assert_eq!(value[0]["name"], OPENCODE_ACP_MCP_SERVER_NAME);
        assert_eq!(value[0]["url"], CANARY_URL);
        assert_eq!(value[0]["headers"][0]["name"], "Authorization");
        assert_eq!(value[0]["headers"][0]["value"], CANARY_HEADER);
        assert_eq!(
            production_mcp_servers(None, Some(&remote)).expect("http list"),
            value
        );
    }

    #[test]
    fn http_encoder_refuses_invalid_url_bad_header_name_and_wrong_name() {
        let invalid_url = OpenCodeAcpRemoteMcpPlacement::new(
            OPENCODE_ACP_MCP_SERVER_NAME,
            "not-a-url",
            Vec::<(String, String)>::new(),
        );
        let error = invalid_url
            .to_production_mcp_servers()
            .expect_err("relative URL is refused");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.opencode.acp.mcp_http_invalid"
        );
        assert_secret_redacted(&error);
        assert!(!error.diagnostic().message().contains("not-a-url"));

        let bad_header = OpenCodeAcpRemoteMcpPlacement::new(
            OPENCODE_ACP_MCP_SERVER_NAME,
            CANARY_URL,
            vec![("Bad Header:".to_owned(), CANARY_HEADER.to_owned())],
        );
        let error = bad_header
            .to_production_mcp_servers()
            .expect_err("header names stay tokens");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.opencode.acp.mcp_http_invalid"
        );
        assert_secret_redacted(&error);

        let collision = OpenCodeAcpRemoteMcpPlacement::new(
            "other-server",
            CANARY_URL,
            vec![("Authorization".to_owned(), CANARY_HEADER.to_owned())],
        );
        let error = collision
            .to_production_mcp_servers()
            .expect_err("name is reserved");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.opencode.acp.mcp_name_collision"
        );
        assert_secret_redacted(&error);
    }

    #[test]
    fn sse_stays_modelled_and_unemitted() {
        let remote = OpenCodeAcpRemoteMcpPlacement::sse(
            OPENCODE_ACP_MCP_SERVER_NAME,
            CANARY_URL,
            vec![("Authorization".to_owned(), CANARY_HEADER.to_owned())],
        );
        assert_eq!(remote.transport(), OpenCodeAcpRemoteMcpTransport::Sse);
        let error = remote
            .to_production_mcp_servers()
            .expect_err("sse is not emitted");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.opencode.acp.mcp_sse_not_emitted"
        );
        assert_secret_redacted(&error);
        assert_secret_redacted(&remote);
    }

    #[test]
    fn stdio_and_http_together_are_a_typed_refusal() {
        let stdio = OpenCodeAcpStdioMcpServer::new(
            OPENCODE_ACP_MCP_SERVER_NAME,
            "/usr/bin/echo-mcp",
            Vec::<String>::new(),
            Vec::<(String, String)>::new(),
        );
        let http = admitted_http();
        let error =
            production_mcp_servers(Some(&stdio), Some(&http)).expect_err("one entry per session");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.opencode.acp.mcp_entry_conflict"
        );
        assert_secret_redacted(&error);
    }

    #[test]
    fn debug_and_failures_redact_url_and_header_values() {
        let remote = admitted_http();
        assert_secret_redacted(&remote);
        assert_eq!(remote.url(), CANARY_URL);
        assert_eq!(remote.headers()[0].1, CANARY_HEADER);
        let rendered = format!("{remote:?}");
        assert!(rendered.contains("Authorization"));
        assert!(rendered.contains("<redacted>"));
        assert!(!rendered.contains(CANARY_URL));
    }
}
