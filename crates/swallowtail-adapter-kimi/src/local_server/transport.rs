use crate::failure::failure;
use curl::easy::{Easy, List, WriteError};
use futures_channel::oneshot;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use swallowtail_runtime::{HostServices, RuntimeFailure, ScopeId};
use url::{Host, Url};

use super::protocol::MAX_HTTP_BODY_BYTES;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Method {
    Get,
    Post,
}

#[derive(Clone)]
pub(super) struct Request {
    method: Method,
    path: String,
    query: Option<&'static str>,
    body: Option<Vec<u8>>,
}

impl Request {
    pub(super) fn get(path: impl Into<String>) -> Self {
        Self {
            method: Method::Get,
            path: path.into(),
            query: None,
            body: None,
        }
    }

    pub(super) fn post(path: impl Into<String>) -> Self {
        Self {
            method: Method::Post,
            path: path.into(),
            query: None,
            body: None,
        }
    }

    pub(super) fn post_json(path: impl Into<String>, body: Vec<u8>) -> Self {
        Self {
            method: Method::Post,
            path: path.into(),
            query: None,
            body: Some(body),
        }
    }

    pub(super) fn get_pending(path: impl Into<String>) -> Self {
        Self {
            method: Method::Get,
            path: path.into(),
            query: Some("status=pending"),
            body: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Response {
    pub(super) status: u16,
    pub(super) body: Vec<u8>,
}

#[derive(Clone, Default)]
pub(super) struct CurlTransport;

impl CurlTransport {
    pub(super) async fn request(
        &self,
        scope: ScopeId,
        endpoint: String,
        request: Request,
        bearer: Option<Vec<u8>>,
        services: &HostServices,
        cancelled: Arc<AtomicBool>,
    ) -> Result<Response, RuntimeFailure> {
        let blocking = services.blocking_work().cloned().ok_or_else(|| {
            failure(
                "swallowtail.kimi.local_server.blocking_service_missing",
                "Kimi local-server HTTP requires a blocking-work service",
            )
        })?;
        let url = request_url(&endpoint, &request)?;
        let (sender, receiver) = oneshot::channel();
        blocking
            .run(
                scope,
                Box::new(move || {
                    let result = perform_request(url, request, bearer, cancelled);
                    let _ = sender.send(result);
                    Ok(())
                }),
            )
            .await?;
        receiver.await.map_err(|_| {
            failure(
                "swallowtail.kimi.local_server.blocking_result_missing",
                "Kimi local-server blocking HTTP work did not return a result",
            )
        })?
    }
}

pub(super) fn require_loopback_endpoint(endpoint: &str) -> Result<Url, RuntimeFailure> {
    let url = Url::parse(endpoint).map_err(|_| endpoint_failure())?;
    if url.scheme() != "http"
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || !matches!(url.host(), Some(Host::Ipv4(address)) if address.is_loopback())
        || url.port().is_none()
        || !matches!(url.path(), "" | "/")
    {
        return Err(endpoint_failure());
    }
    Ok(url)
}

pub(super) fn endpoint_port(endpoint: &str) -> Result<u16, RuntimeFailure> {
    require_loopback_endpoint(endpoint)?
        .port()
        .ok_or_else(endpoint_failure)
}

pub(super) fn session_path(provider_session_id: &str) -> Result<String, RuntimeFailure> {
    if provider_session_id.is_empty()
        || !provider_session_id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~'))
    {
        return Err(failure(
            "swallowtail.kimi.local_server.session_ref_invalid",
            "Kimi local-server session reference is not an eligible route segment",
        ));
    }
    Ok(format!("/api/v1/sessions/{provider_session_id}"))
}

fn request_url(endpoint: &str, request: &Request) -> Result<Url, RuntimeFailure> {
    let mut url = require_loopback_endpoint(endpoint)?;
    if !request.path.starts_with("/api/v1/")
        || request.path.contains('?')
        || request.path.contains('#')
        || request.query.is_some_and(|query| query != "status=pending")
    {
        return Err(failure(
            "swallowtail.kimi.local_server.route_invalid",
            "Kimi local-server request route is invalid",
        ));
    }
    url.set_path(&request.path);
    url.set_query(request.query);
    Ok(url)
}

fn perform_request(
    url: Url,
    request: Request,
    bearer: Option<Vec<u8>>,
    cancelled: Arc<AtomicBool>,
) -> Result<Response, RuntimeFailure> {
    if request
        .body
        .as_ref()
        .is_some_and(|body| body.is_empty() || body.len() > MAX_HTTP_BODY_BYTES)
    {
        return Err(failure(
            "swallowtail.kimi.local_server.request_limit",
            "Kimi local-server request body was empty or exceeded its bound",
        ));
    }
    let mut easy = Easy::new();
    easy.url(url.as_str()).map_err(curl_failure)?;
    easy.follow_location(false).map_err(curl_failure)?;
    easy.proxy("").map_err(curl_failure)?;
    easy.progress(true).map_err(curl_failure)?;
    let progress_cancelled = Arc::clone(&cancelled);
    easy.progress_function(move |_, _, _, _| !progress_cancelled.load(Ordering::SeqCst))
        .map_err(curl_failure)?;
    if request.method == Method::Post {
        easy.post(true).map_err(curl_failure)?;
        easy.post_fields_copy(request.body.as_deref().unwrap_or_default())
            .map_err(curl_failure)?;
    }
    if let Some(mut bearer) = bearer {
        let token = std::str::from_utf8(&bearer).map_err(|_| credential_failure())?;
        if token.is_empty() || token.bytes().any(|byte| matches!(byte, b'\r' | b'\n')) {
            bearer.fill(0);
            return Err(credential_failure());
        }
        let mut header = String::with_capacity("Authorization: Bearer ".len() + token.len());
        header.push_str("Authorization: Bearer ");
        header.push_str(token);
        let mut headers = List::new();
        let configured = headers.append(&header).map_err(curl_failure);
        unsafe_fill_string(&mut header);
        bearer.fill(0);
        configured?;
        if request.body.is_some() {
            headers
                .append("Content-Type: application/json")
                .map_err(curl_failure)?;
        }
        easy.http_headers(headers).map_err(curl_failure)?;
    }

    let mut body = Vec::new();
    let overflow = Arc::new(AtomicBool::new(false));
    {
        let callback_overflow = Arc::clone(&overflow);
        let mut transfer = easy.transfer();
        transfer
            .write_function(|chunk| {
                if body.len().saturating_add(chunk.len()) > MAX_HTTP_BODY_BYTES {
                    callback_overflow.store(true, Ordering::SeqCst);
                    return Err(WriteError::Pause);
                }
                body.extend_from_slice(chunk);
                Ok(chunk.len())
            })
            .map_err(curl_failure)?;
        let result = transfer.perform();
        if overflow.load(Ordering::SeqCst) {
            return Err(failure(
                "swallowtail.kimi.local_server.response_limit",
                "Kimi local-server response exceeded the bounded input limit",
            ));
        }
        if cancelled.load(Ordering::SeqCst) {
            return Err(failure(
                "swallowtail.kimi.local_server.request_cancelled",
                "Kimi local-server HTTP request was cancelled",
            ));
        }
        result.map_err(curl_failure)?;
    }
    let status = easy.response_code().map_err(curl_failure)?;
    let status = u16::try_from(status).map_err(|_| {
        failure(
            "swallowtail.kimi.local_server.status_invalid",
            "Kimi local-server returned an invalid HTTP status",
        )
    })?;
    Ok(Response { status, body })
}

fn unsafe_fill_string(value: &mut String) {
    // `String` has no stable safe zeroization API. Replacing it promptly limits
    // the adapter-owned lifetime; the host still owns lease destruction.
    value.clear();
}

fn endpoint_failure() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.endpoint_invalid",
        "Kimi local-server requires one explicit loopback HTTP endpoint",
    )
}

fn credential_failure() -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.credential_invalid",
        "Kimi local-server bearer credential is invalid",
    )
}

fn curl_failure(_: curl::Error) -> RuntimeFailure {
    failure(
        "swallowtail.kimi.local_server.transport_failed",
        "Kimi local-server HTTP transport failed",
    )
}

#[cfg(test)]
mod tests {
    use super::require_loopback_endpoint;

    #[test]
    fn endpoint_is_exact_loopback_http() {
        assert!(require_loopback_endpoint("http://127.0.0.1:54999").is_ok());
        for rejected in [
            "https://127.0.0.1:54999",
            "http://localhost:54999",
            "http://127.0.0.1",
            "http://127.0.0.1:54999/sibling",
            "http://127.0.0.1:54999?token=secret",
            "http://user@127.0.0.1:54999",
        ] {
            assert!(require_loopback_endpoint(rejected).is_err());
        }
    }
}
