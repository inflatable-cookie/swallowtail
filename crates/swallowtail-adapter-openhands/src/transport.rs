use crate::failure::failure;
use crate::protocol::require_loopback_endpoint;
use curl::easy::{Easy, List, WriteError};
use futures_channel::oneshot;
use serde_json::Value;
use std::cell::Cell;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use swallowtail_runtime::{BoxFuture, HostServices, RuntimeFailure, ScopeId};
use url::Url;

const MAX_BODY: usize = 1024 * 1024;

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub(crate) enum Method {
    Get,
    Post,
}

#[allow(dead_code)]
pub(crate) fn http_json<'a>(
    endpoint: &'a str,
    method: Method,
    path: &'a str,
    body: Option<Value>,
    services: &'a HostServices,
    scope: ScopeId,
    cancelled: Arc<AtomicBool>,
) -> BoxFuture<'a, Result<Value, RuntimeFailure>> {
    Box::pin(async move {
        let blocking = services.blocking_work().cloned().ok_or_else(|| {
            failure(
                "swallowtail.openhands.agent_server.host_service_missing",
                "OpenHands Agent Server HTTP requires a blocking-work service",
            )
        })?;
        let url = request_url(endpoint, path)?;
        let payload = match body {
            Some(value) => Some(serde_json::to_vec(&value).map_err(|_| {
                failure(
                    "swallowtail.openhands.agent_server.request_limit",
                    "OpenHands Agent Server request could not be encoded",
                )
            })?),
            None => None,
        };
        let (sender, receiver) = oneshot::channel();
        blocking
            .run(
                scope,
                Box::new(move || {
                    let result = perform(url, method, payload, cancelled);
                    let _ = sender.send(result);
                    Ok(())
                }),
            )
            .await?;
        receiver.await.map_err(|_| {
            failure(
                "swallowtail.openhands.agent_server.transport_failed",
                "OpenHands Agent Server HTTP did not return a bounded result",
            )
        })?
    })
}

fn request_url(endpoint: &str, path: &str) -> Result<Url, RuntimeFailure> {
    let mut url = require_loopback_endpoint(endpoint)?;
    if !is_allowed_path(path) {
        return Err(failure(
            "swallowtail.openhands.agent_server.route_invalid",
            "OpenHands Agent Server request route is invalid",
        ));
    }
    url.set_path(path);
    Ok(url)
}

pub(crate) fn is_allowed_path(path: &str) -> bool {
    matches!(
        path,
        "/health" | "/alive" | "/ready" | "/server_info" | "/api/conversations"
    ) || (path.starts_with("/api/conversations/")
        && !path.contains(['?', '#'])
        && path
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'/' | b'-' | b'_')))
}

fn perform(
    url: Url,
    method: Method,
    payload: Option<Vec<u8>>,
    cancelled: Arc<AtomicBool>,
) -> Result<Value, RuntimeFailure> {
    if cancelled.load(Ordering::SeqCst) {
        return Err(failure(
            "swallowtail.openhands.agent_server.transport_cancelled",
            "OpenHands Agent Server HTTP was cancelled",
        ));
    }
    let mut easy = Easy::new();
    easy.url(url.as_str()).map_err(curl_failure)?;
    easy.follow_location(false).map_err(curl_failure)?;
    easy.proxy("").map_err(curl_failure)?;
    if matches!(method, Method::Post) {
        let mut headers = List::new();
        headers
            .append("content-type: application/json")
            .map_err(curl_failure)?;
        easy.http_headers(headers).map_err(curl_failure)?;
        easy.post(true).map_err(curl_failure)?;
        easy.post_fields_copy(payload.as_deref().unwrap_or(b"{}"))
            .map_err(curl_failure)?;
    }
    let mut body = Vec::new();
    let overflow = Cell::new(false);
    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|chunk| {
                if body.len().saturating_add(chunk.len()) > MAX_BODY {
                    overflow.set(true);
                    return Err(WriteError::Pause);
                }
                body.extend_from_slice(chunk);
                Ok(chunk.len())
            })
            .map_err(curl_failure)?;
        transfer.perform().map_err(curl_failure)?;
    }
    if overflow.get() {
        return Err(failure(
            "swallowtail.openhands.agent_server.response_limit",
            "OpenHands Agent Server HTTP response exceeded its bound",
        ));
    }
    serde_json::from_slice(&body).map_err(|_| {
        failure(
            "swallowtail.openhands.agent_server.malformed_stream",
            "OpenHands Agent Server HTTP returned malformed JSON",
        )
    })
}

fn curl_failure(_: curl::Error) -> RuntimeFailure {
    failure(
        "swallowtail.openhands.agent_server.transport_failed",
        "OpenHands Agent Server HTTP transport failed",
    )
}

#[cfg(test)]
mod tests {
    use super::is_allowed_path;

    #[test]
    fn only_health_and_conversation_routes_are_allowed() {
        assert!(is_allowed_path("/health"));
        assert!(is_allowed_path("/api/conversations"));
        assert!(is_allowed_path(
            "/api/conversations/00000000-0000-4000-8000-000000000001/run"
        ));
        assert!(!is_allowed_path("/sockets/bash-events"));
        assert!(!is_allowed_path("/conversations/x/events/socket"));
        assert!(!is_allowed_path("/api/conversations/x?foo=1"));
    }
}
