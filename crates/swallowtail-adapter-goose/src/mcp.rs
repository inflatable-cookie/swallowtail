//! Consumer-declared MCP seam for `goose.acp`.
//!
//! Contract 063 admits one consumer-supplied streamable-HTTP placement.
//! Production `session/new` serializes that shape, bound to one route-owned
//! name, as ACP `McpServer::Http`. The provider rejects SSE
//! (`"SSE is unsupported, migrate to streamable_http"`), so SSE is not offered.

use crate::failure::failure;
use serde_json::{Value, json};
use std::fmt;
use swallowtail_runtime::RuntimeFailure;
use url::Url;

/// Reserved ACP `mcpServers` name owned by this route.
pub const GOOSE_ACP_MCP_SERVER_NAME: &str = "swallowtail-goose-acp";
/// Contract 061 presence token for the admitted HTTP placement.
pub const GOOSE_ACP_HTTP_MCP_PLACEMENT: &str = "consumer-supplied-http";

/// ACP URL-plus-header transport the provider can represent.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GooseAcpRemoteMcpTransport {
    /// Streamable-HTTP form admitted on production `session/new`.
    Http,
    /// SSE form kept only so encoding can refuse it with the provider gate.
    Sse,
}

/// Provider-representable URL-plus-header MCP placement.
///
/// URL and header values are consumer secrets: Debug, failures, and
/// projections never carry them.
#[derive(Clone, Eq, PartialEq)]
pub struct GooseAcpRemoteMcpPlacement {
    name: String,
    url: String,
    headers: Vec<(String, String)>,
    transport: GooseAcpRemoteMcpTransport,
}

impl GooseAcpRemoteMcpPlacement {
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
            transport: GooseAcpRemoteMcpTransport::Http,
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
            transport: GooseAcpRemoteMcpTransport::Sse,
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
    pub const fn transport(&self) -> GooseAcpRemoteMcpTransport {
        self.transport
    }

    /// Encodes this placement onto production `mcpServers` when it is the admitted `http` form.
    ///
    /// The returned payload's `Debug` form redacts URL and header values. The
    /// wire JSON stays crate-private.
    pub fn to_production_mcp_servers(&self) -> Result<GooseAcpEncodedMcpServers, RuntimeFailure> {
        Ok(GooseAcpEncodedMcpServers::from_entry(
            self.to_acp_http_value()?,
        ))
    }

    pub(crate) fn to_acp_http_value(&self) -> Result<Value, RuntimeFailure> {
        if self.transport != GooseAcpRemoteMcpTransport::Http {
            return Err(failure(
                "swallowtail.goose.acp.mcp_sse_unsupported",
                "Goose ACP does not offer sse MCP entries; the provider rejects SSE",
            ));
        }
        if self.name.is_empty() {
            return Err(http_invalid());
        }
        if self.name != GOOSE_ACP_MCP_SERVER_NAME {
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

impl fmt::Debug for GooseAcpRemoteMcpPlacement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GooseAcpRemoteMcpPlacement")
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
pub struct GooseAcpEncodedMcpServers {
    value: Value,
}

impl GooseAcpEncodedMcpServers {
    fn from_entry(entry: Value) -> Self {
        Self {
            value: json!([entry]),
        }
    }
}

impl fmt::Debug for GooseAcpEncodedMcpServers {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GooseAcpEncodedMcpServers")
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
    http: Option<&GooseAcpRemoteMcpPlacement>,
) -> Result<Value, RuntimeFailure> {
    match http {
        None => Ok(json!([])),
        Some(remote) => Ok(json!([remote.to_acp_http_value()?])),
    }
}

fn name_collision() -> RuntimeFailure {
    failure(
        "swallowtail.goose.acp.mcp_name_collision",
        "Goose ACP binds consumer MCP declarations to one route-owned server name",
    )
}

fn http_invalid() -> RuntimeFailure {
    failure(
        "swallowtail.goose.acp.mcp_http_invalid",
        "Goose ACP HTTP MCP declarations require a non-empty name, an absolute http or https URL, and well-formed header names",
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

    const CANARY_URL: &str = "http://127.0.0.1:9/mcp/g06-036-redaction-canary";
    const CANARY_HEADER: &str = "Bearer g06-036-redaction-canary";

    fn admitted_http() -> GooseAcpRemoteMcpPlacement {
        GooseAcpRemoteMcpPlacement::new(
            GOOSE_ACP_MCP_SERVER_NAME,
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
            !rendered.contains("g06-036-redaction-canary"),
            "debug leaked the canary: {rendered}"
        );
    }

    #[test]
    fn omission_keeps_the_empty_mcp_servers_list() {
        assert_eq!(production_mcp_servers(None).expect("omission"), json!([]));
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
        assert_eq!(value[0]["name"], GOOSE_ACP_MCP_SERVER_NAME);
        assert_eq!(value[0]["url"], CANARY_URL);
        assert_eq!(value[0]["headers"][0]["name"], "Authorization");
        assert_eq!(value[0]["headers"][0]["value"], CANARY_HEADER);
    }

    #[test]
    fn http_encoder_refuses_invalid_url_bad_header_name_and_wrong_name() {
        let invalid_url = GooseAcpRemoteMcpPlacement::new(
            GOOSE_ACP_MCP_SERVER_NAME,
            "not-a-url",
            Vec::<(String, String)>::new(),
        );
        let error = invalid_url
            .to_production_mcp_servers()
            .expect_err("relative URL is refused");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.goose.acp.mcp_http_invalid"
        );
        assert_secret_redacted(&error);
        assert!(!error.diagnostic().message().contains("not-a-url"));

        let bad_header = GooseAcpRemoteMcpPlacement::new(
            GOOSE_ACP_MCP_SERVER_NAME,
            CANARY_URL,
            vec![("Bad Header:".to_owned(), CANARY_HEADER.to_owned())],
        );
        let error = bad_header
            .to_production_mcp_servers()
            .expect_err("header names stay tokens");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.goose.acp.mcp_http_invalid"
        );
        assert_secret_redacted(&error);

        let collision = GooseAcpRemoteMcpPlacement::new(
            "other-server",
            CANARY_URL,
            vec![("Authorization".to_owned(), CANARY_HEADER.to_owned())],
        );
        let error = collision
            .to_production_mcp_servers()
            .expect_err("name is reserved");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.goose.acp.mcp_name_collision"
        );
        assert_secret_redacted(&error);

        let empty_name =
            GooseAcpRemoteMcpPlacement::new("", CANARY_URL, Vec::<(String, String)>::new());
        let error = empty_name
            .to_production_mcp_servers()
            .expect_err("empty name is refused");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.goose.acp.mcp_http_invalid"
        );
        assert_secret_redacted(&error);
    }

    #[test]
    fn sse_is_not_offered_and_encoding_surfaces_the_provider_gate() {
        let remote = GooseAcpRemoteMcpPlacement::sse(
            GOOSE_ACP_MCP_SERVER_NAME,
            CANARY_URL,
            vec![("Authorization".to_owned(), CANARY_HEADER.to_owned())],
        );
        assert_eq!(remote.transport(), GooseAcpRemoteMcpTransport::Sse);
        let error = remote
            .to_production_mcp_servers()
            .expect_err("sse is not emitted");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.goose.acp.mcp_sse_unsupported"
        );
        assert_secret_redacted(&error);
        assert_secret_redacted(&remote);
        assert!(!error.diagnostic().message().contains(CANARY_URL));
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
