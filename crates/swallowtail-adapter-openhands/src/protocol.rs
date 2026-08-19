#![allow(dead_code)]

use crate::command::MAXIMUM_ITERATIONS;
use crate::failure::failure;
use serde_json::{Value, json};
use swallowtail_runtime::RuntimeFailure;
use url::{Host, Url};

const BANNER_PREFIX: &str = "Starting OpenHands Agent Server on ";
const STARTUP_LIMIT: usize = 16 * 1024;

#[must_use]
pub(crate) fn start_conversation_body(working_dir: &str, prompt: &str) -> Value {
    json!({
        "workspace": {
            "kind": "LocalWorkspace",
            "working_dir": working_dir
        },
        "max_iterations": MAXIMUM_ITERATIONS,
        "confirmation_policy": { "kind": "AlwaysConfirm" },
        "initial_message": {
            "role": "user",
            "run": true,
            "content": [{ "type": "text", "text": prompt }]
        }
    })
}

pub(crate) fn require_loopback_endpoint(endpoint: &str) -> Result<Url, RuntimeFailure> {
    let url = Url::parse(endpoint).map_err(|_| endpoint_failure())?;
    if url.scheme() != "http"
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || !matches!(url.host(), Some(Host::Ipv4(address)) if address.is_loopback() && address == std::net::Ipv4Addr::LOCALHOST)
        || url.port().is_none_or(|port| port == 0)
        || !matches!(url.path(), "" | "/")
    {
        return Err(endpoint_failure());
    }
    Ok(url)
}

pub(crate) fn parse_startup_endpoint(output: &[u8]) -> Result<Option<String>, RuntimeFailure> {
    if output.len() > STARTUP_LIMIT {
        return Err(failure(
            "swallowtail.openhands.agent_server.startup_output_limit",
            "OpenHands Agent Server startup output exceeded its bound",
        ));
    }
    let text = std::str::from_utf8(output).map_err(|_| endpoint_failure())?;
    let matches = text.match_indices(BANNER_PREFIX).collect::<Vec<_>>();
    if matches.len() > 1 {
        return Err(failure(
            "swallowtail.openhands.agent_server.startup_endpoint_duplicate",
            "OpenHands Agent Server reported more than one bind address",
        ));
    }
    let Some((offset, _)) = matches.first().copied() else {
        return Ok(None);
    };
    let record = &text[offset + BANNER_PREFIX.len()..];
    let Some(record_end) = record.find('\n') else {
        return Ok(None);
    };
    let bind = record[..record_end].trim();
    let (host, port) = bind.split_once(':').ok_or_else(endpoint_failure)?;
    if host != "127.0.0.1" {
        return Err(endpoint_failure());
    }
    let endpoint = format!("http://{host}:{port}");
    require_loopback_endpoint(&endpoint)?;
    Ok(Some(endpoint))
}

pub(crate) fn health_ok(body: &Value) -> bool {
    body.get("status").and_then(Value::as_str) == Some("ok")
}

pub(crate) fn ready_ok(body: &Value) -> bool {
    body.get("status").and_then(Value::as_str) == Some("ready")
}

pub(crate) fn server_info_matches(body: &Value, version: &str) -> bool {
    body.get("version").and_then(Value::as_str) == Some(version)
}

fn endpoint_failure() -> RuntimeFailure {
    failure(
        "swallowtail.openhands.agent_server.endpoint_invalid",
        "OpenHands Agent Server requires one explicit 127.0.0.1 HTTP endpoint",
    )
}

#[cfg(test)]
mod tests {
    use super::{parse_startup_endpoint, require_loopback_endpoint, start_conversation_body};

    #[test]
    fn conversation_body_is_always_confirm_and_bounded() {
        let body = start_conversation_body("opaque-working-resource", "opaque fixture prompt");
        assert_eq!(body["workspace"]["kind"], "LocalWorkspace");
        assert_eq!(body["confirmation_policy"]["kind"], "AlwaysConfirm");
        assert_eq!(body["max_iterations"], 8);
        assert!(body.get("agent").is_none());
        assert!(body.pointer("/agent/llm/api_key").is_none());
        assert_ne!(body["confirmation_policy"]["kind"], "NeverConfirm");
    }

    #[test]
    fn banner_parses_only_one_loopback_bind() {
        assert_eq!(
            parse_startup_endpoint(b"Starting OpenHands Agent Server on 127.0.0.1:54999\n")
                .expect("banner parses")
                .as_deref(),
            Some("http://127.0.0.1:54999")
        );
        assert_eq!(
            parse_startup_endpoint(b"Starting OpenHands Agent Server on 127.0.0.1:54999")
                .expect("partial remains pending"),
            None
        );
        assert!(
            parse_startup_endpoint(b"Starting OpenHands Agent Server on 0.0.0.0:54999\n").is_err()
        );
        assert!(require_loopback_endpoint("http://127.0.0.1:54999").is_ok());
        assert!(require_loopback_endpoint("http://0.0.0.0:54999").is_err());
        assert!(require_loopback_endpoint("http://127.0.0.1:0").is_err());
    }
}
