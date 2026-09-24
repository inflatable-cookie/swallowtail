//! Consumer-declared MCP seam for `copilot-cli.acp`.
//!
//! Contract 063 admits one consumer-supplied streamable-HTTP placement.
//! Production `session/new` serializes that shape, bound to one route-owned
//! name. Stdio client entries are not offered: Research 351 shows the
//! provider rejects them. The provider's `sse` form stays modelled and is
//! not emitted.

use serde_json::{Value, json};
use std::fmt;
use swallowtail_runtime::RuntimeFailure;
use url::Url;

use crate::failure::failure;

/// Reserved ACP `mcpServers` name owned by this route.
pub const COPILOT_CLI_ACP_MCP_SERVER_NAME: &str = "swallowtail-copilot-cli-acp";
/// Contract 061 presence token for the admitted HTTP placement.
pub const COPILOT_CLI_ACP_HTTP_MCP_PLACEMENT: &str = "consumer-supplied-http";

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
    name: String,
    url: String,
    headers: Vec<(String, String)>,
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
            name: name.into(),
            url: url.into(),
            headers: headers.into(),
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
            name: name.into(),
            url: url.into(),
            headers: headers.into(),
            transport: CopilotCliAcpRemoteMcpTransport::Sse,
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
                "swallowtail.copilot-cli.acp.mcp_sse_not_emitted",
                "Copilot CLI ACP models sse MCP entries but emits the http form only",
            ));
        }
        if self.name.is_empty() {
            return Err(http_invalid());
        }
        if self.name != COPILOT_CLI_ACP_MCP_SERVER_NAME {
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

impl fmt::Debug for CopilotCliAcpRemoteMcpPlacement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CopilotCliAcpRemoteMcpPlacement")
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

fn redacted_mcp_servers_debug(value: &Value) -> Value {
    let Some(entries) = value.as_array() else {
        return json!("<redacted>");
    };
    Value::Array(
        entries
            .iter()
            .map(|entry| {
                let mut object = serde_json::Map::new();
                if let Some(kind) = entry.get("type") {
                    object.insert("type".to_owned(), kind.clone());
                }
                if let Some(name) = entry.get("name") {
                    object.insert("name".to_owned(), name.clone());
                }
                if entry.get("url").is_some() {
                    object.insert("url".to_owned(), json!("<redacted>"));
                }
                if let Some(headers) = entry.get("headers").and_then(Value::as_array) {
                    object.insert(
                        "headers".to_owned(),
                        Value::Array(
                            headers
                                .iter()
                                .map(|header| {
                                    json!({
                                        "name": header.get("name").cloned().unwrap_or(Value::Null),
                                        "value": "<redacted>",
                                    })
                                })
                                .collect(),
                        ),
                    );
                }
                Value::Object(object)
            })
            .collect(),
    )
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

#[cfg(test)]
fn stdio_not_admitted() -> RuntimeFailure {
    failure(
        "swallowtail.copilot-cli.acp.mcp_stdio_not_admitted",
        "Copilot CLI ACP does not offer stdio client MCP entries",
    )
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

    const CANARY_URL: &str = "http://127.0.0.1:9/mcp/g06-034-redaction-canary";
    const CANARY_HEADER: &str = "Bearer g06-034-redaction-canary";

    fn admitted_http() -> CopilotCliAcpRemoteMcpPlacement {
        CopilotCliAcpRemoteMcpPlacement::new(
            COPILOT_CLI_ACP_MCP_SERVER_NAME,
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
            !rendered.contains("g06-034-redaction-canary"),
            "debug leaked the canary: {rendered}"
        );
    }

    #[test]
    fn omission_keeps_empty_mcp_servers() {
        assert_eq!(production_mcp_servers(None).expect("omission"), json!([]));
    }

    #[test]
    fn stdio_client_entries_are_a_typed_refusal() {
        let error = stdio_not_admitted();
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.copilot-cli.acp.mcp_stdio_not_admitted"
        );
        let value = production_mcp_servers(Some(&admitted_http())).expect("http list");
        assert!(value[0].get("command").is_none());
        assert!(value[0].get("args").is_none());
        assert!(value[0].get("env").is_none());
        assert_eq!(value[0]["type"], "http");
    }

    #[test]
    fn http_encoder_passes_url_and_headers_verbatim() {
        let remote = admitted_http();
        let encoded = remote
            .to_production_mcp_servers()
            .expect("http is admitted");
        assert_secret_redacted(&encoded);
        let value = production_mcp_servers(Some(&remote)).expect("http list");
        assert_eq!(value[0]["type"], "http");
        assert_eq!(value[0]["name"], COPILOT_CLI_ACP_MCP_SERVER_NAME);
        assert_eq!(value[0]["url"], CANARY_URL);
        assert_eq!(value[0]["headers"][0]["name"], "Authorization");
        assert_eq!(value[0]["headers"][0]["value"], CANARY_HEADER);
    }

    #[test]
    fn http_encoder_refuses_invalid_url_bad_header_name_and_wrong_name() {
        let invalid_url = CopilotCliAcpRemoteMcpPlacement::new(
            COPILOT_CLI_ACP_MCP_SERVER_NAME,
            "not-a-url",
            Vec::<(String, String)>::new(),
        );
        let error = invalid_url
            .to_production_mcp_servers()
            .expect_err("relative URL is refused");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.copilot-cli.acp.mcp_http_invalid"
        );
        assert_secret_redacted(&error);
        assert!(!error.diagnostic().message().contains("not-a-url"));

        let bad_header = CopilotCliAcpRemoteMcpPlacement::new(
            COPILOT_CLI_ACP_MCP_SERVER_NAME,
            CANARY_URL,
            vec![("Bad Header:".to_owned(), CANARY_HEADER.to_owned())],
        );
        let error = bad_header
            .to_production_mcp_servers()
            .expect_err("header names stay tokens");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.copilot-cli.acp.mcp_http_invalid"
        );
        assert_secret_redacted(&error);

        let collision = CopilotCliAcpRemoteMcpPlacement::new(
            "other-server",
            CANARY_URL,
            vec![("Authorization".to_owned(), CANARY_HEADER.to_owned())],
        );
        let error = collision
            .to_production_mcp_servers()
            .expect_err("name is reserved");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.copilot-cli.acp.mcp_name_collision"
        );
        assert_secret_redacted(&error);
    }

    #[test]
    fn sse_stays_modelled_and_unemitted() {
        let remote = CopilotCliAcpRemoteMcpPlacement::sse(
            COPILOT_CLI_ACP_MCP_SERVER_NAME,
            CANARY_URL,
            vec![("Authorization".to_owned(), CANARY_HEADER.to_owned())],
        );
        assert_eq!(remote.transport(), CopilotCliAcpRemoteMcpTransport::Sse);
        let error = remote
            .to_production_mcp_servers()
            .expect_err("sse is not emitted");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.copilot-cli.acp.mcp_sse_not_emitted"
        );
        assert_secret_redacted(&error);
        assert_secret_redacted(&remote);
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

        let encoded = remote
            .to_production_mcp_servers()
            .expect("http is admitted");
        assert_secret_redacted(&encoded);
        assert!(format!("{encoded:?}").contains("<redacted>"));
        assert!(!format!("{encoded:?}").contains(CANARY_HEADER));
    }
}
