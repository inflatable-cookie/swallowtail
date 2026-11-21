use serde_json::{Value, json};
use std::fmt;
use swallowtail_runtime::RuntimeFailure;
use url::Url;
use zeroize::{Zeroize, Zeroizing};

/// Provider-scoped failure factories for HTTP MCP placement validation.
#[derive(Clone, Copy)]
pub struct AcpMcpDiagnostics {
    /// Reserved route-owned ACP `mcpServers` server name.
    pub reserved_name: &'static str,
    /// Failure returned when the declared name does not match `reserved_name`.
    pub name_collision: fn() -> RuntimeFailure,
    /// Failure returned when the name is empty, the URL is not absolute HTTP(S), or a header name is malformed.
    pub http_invalid: fn() -> RuntimeFailure,
}

impl fmt::Debug for AcpMcpDiagnostics {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AcpMcpDiagnostics")
            .field("reserved_name", &self.reserved_name)
            .finish()
    }
}

/// Provider `initialize` advertisement of `agentCapabilities.mcpCapabilities.http`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpHttpAdvertisement {
    /// `agentCapabilities.mcpCapabilities.http` is literally `true`.
    Advertised,
    /// `agentCapabilities.mcpCapabilities.http` is literally `false`.
    NotAdvertised,
    /// The field is absent or not a boolean.
    Unproven,
}

impl AcpHttpAdvertisement {
    /// Classifies the provider's `initialize` result.
    #[must_use]
    pub fn from_initialize(initialize: &Value) -> Self {
        match initialize
            .get("agentCapabilities")
            .and_then(|capabilities| capabilities.get("mcpCapabilities"))
            .and_then(|capabilities| capabilities.get("http"))
            .and_then(Value::as_bool)
        {
            Some(true) => Self::Advertised,
            Some(false) => Self::NotAdvertised,
            None => Self::Unproven,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct ZeroizingHeaders(Vec<(String, String)>);

impl ZeroizingHeaders {
    fn zeroize_values(&mut self) {
        for (_, value) in &mut self.0 {
            value.zeroize();
        }
    }
}

impl Drop for ZeroizingHeaders {
    fn drop(&mut self) {
        self.zeroize_values();
    }
}

/// One admitted consumer-supplied streamable-HTTP MCP declaration.
///
/// URL and header values are consumer secrets: `Debug` and failure diagnostics
/// never carry them, and both are zeroized on drop.
#[derive(Clone, Eq, PartialEq)]
pub struct AcpRemoteMcpPlacement {
    name: String,
    url: Zeroizing<String>,
    headers: ZeroizingHeaders,
}

impl AcpRemoteMcpPlacement {
    /// Creates an HTTP MCP placement entry.
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        url: impl Into<String>,
        headers: impl Into<Vec<(String, String)>>,
    ) -> Self {
        Self {
            name: name.into(),
            url: Zeroizing::new(url.into()),
            headers: ZeroizingHeaders(headers.into()),
        }
    }

    /// Returns the declared ACP server name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the declared URL. Callers must not log or project this secret.
    #[must_use]
    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    /// Returns the declared headers. Values are consumer secrets.
    #[must_use]
    pub fn headers(&self) -> &[(String, String)] {
        &self.headers.0
    }

    #[cfg(test)]
    pub(crate) fn wipe_secrets_for_test(&mut self) {
        self.url.zeroize();
        self.headers.zeroize_values();
    }

    /// Validates and encodes this placement as one ACP `type: "http"` entry.
    pub fn to_acp_http_value(
        &self,
        diagnostics: &AcpMcpDiagnostics,
    ) -> Result<Value, RuntimeFailure> {
        if self.name.is_empty() {
            return Err((diagnostics.http_invalid)());
        }
        if self.name != diagnostics.reserved_name {
            return Err((diagnostics.name_collision)());
        }
        if !absolute_http_url(&self.url) {
            return Err((diagnostics.http_invalid)());
        }
        if self.headers.0.iter().any(|(name, value)| {
            !header_name_is_well_formed(name) || value.bytes().any(is_line_break)
        }) {
            return Err((diagnostics.http_invalid)());
        }
        Ok(json!({
            "type": "http",
            "name": self.name,
            "url": self.url.as_str(),
            "headers": self
                .headers
                .0
                .iter()
                .map(|(name, value)| json!({"name": name, "value": value}))
                .collect::<Vec<_>>(),
        }))
    }
}

impl fmt::Debug for AcpRemoteMcpPlacement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AcpRemoteMcpPlacement")
            .field("name", &self.name)
            .field("url", &"<redacted>")
            .field(
                "headers",
                &self
                    .headers
                    .0
                    .iter()
                    .map(|(name, _)| (name.as_str(), "<redacted>"))
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

/// Encodes an optional HTTP MCP placement into an ACP `mcpServers` JSON array.
pub fn production_mcp_servers(
    http: Option<&AcpRemoteMcpPlacement>,
    diagnostics: &AcpMcpDiagnostics,
) -> Result<Value, RuntimeFailure> {
    match http {
        None => Ok(json!([])),
        Some(remote) => Ok(json!([remote.to_acp_http_value(diagnostics)?])),
    }
}

/// Returns a redacted JSON representation of an ACP `mcpServers` array suitable for `Debug`.
#[must_use]
pub fn redacted_mcp_servers_debug(value: &Value) -> Value {
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

fn absolute_http_url(raw: &str) -> bool {
    Url::parse(raw)
        .is_ok_and(|parsed| matches!(parsed.scheme(), "http" | "https") && parsed.has_host())
}

/// A carriage return or line feed would split the outgoing HTTP header block,
/// so header values reject both, as `SecretHeader` does in the HTTP transport.
const fn is_line_break(byte: u8) -> bool {
    matches!(byte, b'\r' | b'\n')
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
