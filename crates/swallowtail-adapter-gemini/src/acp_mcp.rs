//! Consumer-declared HTTP MCP seam for `gemini-cli.acp`.
//!
//! Contract 063 admits one consumer-supplied streamable-HTTP placement.
//! Production `session/new` serializes it as the ACP `http` form, which the
//! provider maps onto its `httpUrl` transport with headers on
//! `requestInit.headers` (Research 351, `0.59.0`).

use serde_json::{Value, json};
use std::fmt;
use swallowtail_runtime::RuntimeFailure;
use url::Url;

use crate::failure::failure;

/// Reserved ACP `mcpServers` name owned by this route.
pub const GEMINI_ACP_MCP_SERVER_NAME: &str = "swallowtail-gemini-acp";
/// Contract 061 presence token for the admitted HTTP placement.
pub const GEMINI_ACP_HTTP_MCP_PLACEMENT: &str = "consumer-supplied-http";

/// Provider-representable consumer-supplied streamable-HTTP MCP placement.
///
/// URL and header values are consumer secrets: Debug, failures, and
/// projections never carry them.
#[derive(Clone, Eq, PartialEq)]
pub struct GeminiAcpHttpMcpPlacement {
    name: String,
    url: String,
    headers: Vec<(String, String)>,
}

impl GeminiAcpHttpMcpPlacement {
    /// Models the admitted `http` entry the consumer supplies.
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
        if self.name.is_empty() {
            return Err(http_invalid());
        }
        if self.name != GEMINI_ACP_MCP_SERVER_NAME {
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

impl fmt::Debug for GeminiAcpHttpMcpPlacement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GeminiAcpHttpMcpPlacement")
            .field("name", &self.name)
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
    http: Option<&GeminiAcpHttpMcpPlacement>,
) -> Result<Value, RuntimeFailure> {
    match http {
        None => Ok(json!([])),
        Some(remote) => Ok(json!([remote.to_acp_http_value()?])),
    }
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
        "Gemini ACP HTTP MCP declarations require the route-owned non-empty name, an absolute http or https URL, and well-formed header names",
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

    const CANARY_URL: &str = "http://127.0.0.1:9/mcp/g06-035-redaction-canary";
    const CANARY_HEADER: &str = "Bearer g06-035-redaction-canary";

    fn admitted_http() -> GeminiAcpHttpMcpPlacement {
        GeminiAcpHttpMcpPlacement::new(
            GEMINI_ACP_MCP_SERVER_NAME,
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
            !rendered.contains("g06-035-redaction-canary"),
            "debug leaked the canary: {rendered}"
        );
    }

    #[test]
    fn omission_keeps_the_wire_empty_and_byte_identical() {
        let value = production_mcp_servers(None).expect("omission is admitted");
        assert_eq!(
            serde_json::to_string(&value).expect("omission serializes"),
            "[]"
        );
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
        assert_eq!(value[0]["name"], GEMINI_ACP_MCP_SERVER_NAME);
        assert_eq!(value[0]["url"], CANARY_URL);
        assert_eq!(value[0]["headers"][0]["name"], "Authorization");
        assert_eq!(value[0]["headers"][0]["value"], CANARY_HEADER);
    }

    #[test]
    fn http_encoder_refuses_invalid_url_bad_header_name_and_wrong_name() {
        let invalid_url = GeminiAcpHttpMcpPlacement::new(
            GEMINI_ACP_MCP_SERVER_NAME,
            "not-a-url",
            Vec::<(String, String)>::new(),
        );
        let error = invalid_url
            .to_production_mcp_servers()
            .expect_err("relative URL is refused");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.gemini.acp.mcp_http_invalid"
        );
        assert!(!error.diagnostic().message().contains("not-a-url"));

        let bad_header = GeminiAcpHttpMcpPlacement::new(
            GEMINI_ACP_MCP_SERVER_NAME,
            CANARY_URL,
            vec![("Bad Header:".to_owned(), CANARY_HEADER.to_owned())],
        );
        let error = bad_header
            .to_production_mcp_servers()
            .expect_err("header names stay tokens");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.gemini.acp.mcp_http_invalid"
        );
        assert_secret_redacted(&error);

        let collision = GeminiAcpHttpMcpPlacement::new(
            "other-server",
            CANARY_URL,
            vec![("Authorization".to_owned(), CANARY_HEADER.to_owned())],
        );
        let error = collision
            .to_production_mcp_servers()
            .expect_err("name is reserved");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.gemini.acp.mcp_name_collision"
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

        let encoded = remote
            .to_production_mcp_servers()
            .expect("http is admitted");
        assert_secret_redacted(&encoded);
        assert!(format!("{encoded:?}").contains("<redacted>"));
        assert!(!format!("{encoded:?}").contains(CANARY_HEADER));
    }
}
