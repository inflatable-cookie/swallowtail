//! Consumer-declared MCP seam for `kiro.acp`.
//!
//! Contract 063 admits one consumer-supplied streamable-HTTP placement. Production
//! `session/new` serializes that one shape under a route-owned name. The provider's
//! `stdio`, `sse`, and `acp` forms stay out: Research 351 freezes `type: "http"` with
//! headers as the accepted form this route needs.
//!
//! Emission is not honouring. Research 351 names three gates that can still drop,
//! override, or leave the entry unproven, so the encoder returns a typed outcome that
//! names them and never reports provider acceptance. Only a separately authorized live
//! gate may claim honouring.

use crate::failure::failure;
use serde_json::{Value, json};
use std::fmt;
use swallowtail_runtime::RuntimeFailure;
use url::Url;

/// Reserved ACP `mcpServers` name owned by this route.
pub const KIRO_ACP_MCP_SERVER_NAME: &str = "swallowtail-kiro-acp";
/// Contract 061 presence token for the admitted HTTP placement.
pub const KIRO_ACP_HTTP_MCP_PLACEMENT: &str = "consumer-supplied-http";

/// Research 351 provider gate that keeps a session-injected Kiro MCP entry from being honoured.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KiroAcpMcpGate {
    /// `initialize` does not prove `mcpCapabilities.http`. The bundled schema default is
    /// `false` and Research 351 found no cited literal `true`, so emission may not infer
    /// that Kiro accepts the entry.
    InitializeAdvertisementUnproven,
    /// Kiro drops session-injected MCP servers when MCP is disabled by governance.
    GovernanceDisabled,
    /// ACP-supplied MCP servers override servers already in agent configuration.
    AgentConfigurationOverride,
}

/// Every gate Research 351 leaves open between emission and honouring.
static HTTP_MCP_GATES: [KiroAcpMcpGate; 3] = [
    KiroAcpMcpGate::InitializeAdvertisementUnproven,
    KiroAcpMcpGate::GovernanceDisabled,
    KiroAcpMcpGate::AgentConfigurationOverride,
];

/// Provider `initialize` MCP advertisement classified against the Research 351 gate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KiroAcpHttpAdvertisement {
    /// `agentCapabilities.mcpCapabilities.http` is literally `true`.
    Advertised,
    /// The field is present and `false`; Kiro would drop the emitted entry.
    NotAdvertised,
    /// The field is absent or not a boolean. Research 351's unproven default.
    Unproven,
}

impl KiroAcpHttpAdvertisement {
    /// Classifies the provider's `initialize` result.
    ///
    /// Absence is [`KiroAcpHttpAdvertisement::Unproven`], never an acceptance claim:
    /// Research 351 found the bundled schema default `false` but no cited literal `true`
    /// from the shipped `kiro-cli-chat`.
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

/// One admitted consumer-supplied streamable-HTTP MCP declaration.
///
/// URL and header values are consumer secrets: `Debug`, failures, and projections never
/// carry them.
#[derive(Clone, Eq, PartialEq)]
pub struct KiroAcpRemoteMcpPlacement {
    name: String,
    url: String,
    headers: Vec<(String, String)>,
}

impl KiroAcpRemoteMcpPlacement {
    /// Creates the admitted `http` entry.
    ///
    /// Production encoding requires the route-owned name
    /// [`KIRO_ACP_MCP_SERVER_NAME`].
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

    /// Returns the declared ACP server name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the declared URL. Callers that log or project this value leak a secret.
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Returns the declared headers. Values are consumer secrets.
    #[must_use]
    pub fn headers(&self) -> &[(String, String)] {
        &self.headers
    }

    /// Encodes this placement onto production `mcpServers`.
    ///
    /// The returned outcome's `Debug` form redacts URL and header values, and its
    /// [`KiroAcpEncodedMcpServers::is_honoured`] is always `false`. The wire JSON stays
    /// crate-private.
    pub fn to_production_mcp_servers(&self) -> Result<KiroAcpEncodedMcpServers, RuntimeFailure> {
        Ok(KiroAcpEncodedMcpServers::from_entry(
            self.to_acp_http_value()?,
        ))
    }

    pub(crate) fn to_acp_http_value(&self) -> Result<Value, RuntimeFailure> {
        if self.name.is_empty() {
            return Err(http_invalid());
        }
        if self.name != KIRO_ACP_MCP_SERVER_NAME {
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

impl fmt::Debug for KiroAcpRemoteMcpPlacement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("KiroAcpRemoteMcpPlacement")
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

/// Typed outcome of serializing one consumer HTTP entry onto `session/new`.
///
/// Emission is not honouring: this payload names every Research 351 gate that can still
/// drop or override the entry and exposes no accepted state. `Debug` redacts URL and
/// header values.
pub struct KiroAcpEncodedMcpServers {
    value: Value,
}

impl KiroAcpEncodedMcpServers {
    fn from_entry(entry: Value) -> Self {
        Self {
            value: json!([entry]),
        }
    }

    /// Returns the provider gates that still stand between emission and honouring.
    #[must_use]
    pub fn gates(&self) -> &'static [KiroAcpMcpGate] {
        &HTTP_MCP_GATES
    }

    /// Reports whether the provider has been observed to honour the entry.
    ///
    /// Always `false`: emission proves serialization only. A live gate owns any
    /// acceptance claim.
    #[must_use]
    pub const fn is_honoured(&self) -> bool {
        false
    }
}

impl fmt::Debug for KiroAcpEncodedMcpServers {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("KiroAcpEncodedMcpServers")
            .field("entries", &redacted_mcp_servers_debug(&self.value))
            .field("gates", &self.gates())
            .field("honoured", &self.is_honoured())
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
///
/// Omission stays byte-identical to the pre-wiring route.
pub(crate) fn production_mcp_servers(
    http: Option<&KiroAcpRemoteMcpPlacement>,
) -> Result<Value, RuntimeFailure> {
    match http {
        None => Ok(json!([])),
        Some(remote) => Ok(json!([remote.to_acp_http_value()?])),
    }
}

pub(crate) fn name_collision() -> RuntimeFailure {
    failure(
        "swallowtail.kiro.acp.mcp_name_collision",
        "Kiro ACP binds consumer MCP declarations to one route-owned server name",
    )
}

fn http_invalid() -> RuntimeFailure {
    failure(
        "swallowtail.kiro.acp.mcp_http_invalid",
        "Kiro ACP HTTP MCP declarations require a non-empty name, an absolute http or https URL, and well-formed header names",
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

    const CANARY_URL: &str = "http://127.0.0.1:9/mcp/g06-037-redaction-canary";
    const CANARY_HEADER: &str = "Bearer g06-037-redaction-canary";

    fn admitted_http() -> KiroAcpRemoteMcpPlacement {
        KiroAcpRemoteMcpPlacement::new(
            KIRO_ACP_MCP_SERVER_NAME,
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
            !rendered.contains("g06-037-redaction-canary"),
            "debug leaked the canary: {rendered}"
        );
    }

    #[test]
    fn omission_stays_an_empty_list() {
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
        assert_eq!(value[0]["name"], KIRO_ACP_MCP_SERVER_NAME);
        assert_eq!(value[0]["url"], CANARY_URL);
        assert_eq!(value[0]["headers"][0]["name"], "Authorization");
        assert_eq!(value[0]["headers"][0]["value"], CANARY_HEADER);
    }

    #[test]
    fn http_encoder_refuses_invalid_url_bad_header_name_and_wrong_name() {
        let invalid_url = KiroAcpRemoteMcpPlacement::new(
            KIRO_ACP_MCP_SERVER_NAME,
            "not-a-url",
            Vec::<(String, String)>::new(),
        );
        let error = invalid_url
            .to_production_mcp_servers()
            .expect_err("relative URL is refused");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.kiro.acp.mcp_http_invalid"
        );
        assert_secret_redacted(&error);
        assert!(!error.diagnostic().message().contains("not-a-url"));

        let bad_header = KiroAcpRemoteMcpPlacement::new(
            KIRO_ACP_MCP_SERVER_NAME,
            CANARY_URL,
            vec![("Bad Header:".to_owned(), CANARY_HEADER.to_owned())],
        );
        let error = bad_header
            .to_production_mcp_servers()
            .expect_err("header names stay tokens");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.kiro.acp.mcp_http_invalid"
        );
        assert_secret_redacted(&error);

        let collision = KiroAcpRemoteMcpPlacement::new(
            "other-server",
            CANARY_URL,
            vec![("Authorization".to_owned(), CANARY_HEADER.to_owned())],
        );
        let error = collision
            .to_production_mcp_servers()
            .expect_err("name is reserved");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.kiro.acp.mcp_name_collision"
        );
        assert_secret_redacted(&error);
    }

    #[test]
    fn emission_names_the_initialize_advertisement_gate() {
        let encoded = admitted_http()
            .to_production_mcp_servers()
            .expect("http is admitted");
        assert!(
            encoded
                .gates()
                .contains(&KiroAcpMcpGate::InitializeAdvertisementUnproven)
        );
        assert!(!encoded.is_honoured());
    }

    #[test]
    fn emission_names_the_governance_drop_gate() {
        let encoded = admitted_http()
            .to_production_mcp_servers()
            .expect("http is admitted");
        assert!(
            encoded
                .gates()
                .contains(&KiroAcpMcpGate::GovernanceDisabled)
        );
        assert!(!encoded.is_honoured());
    }

    #[test]
    fn emission_names_the_agent_configuration_override_gate() {
        let encoded = admitted_http()
            .to_production_mcp_servers()
            .expect("http is admitted");
        assert!(
            encoded
                .gates()
                .contains(&KiroAcpMcpGate::AgentConfigurationOverride)
        );
        assert!(!encoded.is_honoured());
    }

    #[test]
    fn initialize_advertisement_is_unproven_without_a_cited_true() {
        assert_eq!(
            KiroAcpHttpAdvertisement::from_initialize(&json!({
                "agentCapabilities": {"loadSession": true}
            })),
            KiroAcpHttpAdvertisement::Unproven
        );
        assert_eq!(
            KiroAcpHttpAdvertisement::from_initialize(&json!({
                "agentCapabilities": {"mcpCapabilities": {"http": false, "sse": false}}
            })),
            KiroAcpHttpAdvertisement::NotAdvertised
        );
        assert_eq!(
            KiroAcpHttpAdvertisement::from_initialize(&json!({
                "agentCapabilities": {"mcpCapabilities": {"http": true}}
            })),
            KiroAcpHttpAdvertisement::Advertised
        );
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
