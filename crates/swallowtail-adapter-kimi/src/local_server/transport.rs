use crate::failure::failure;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use swallowtail_runtime::{HostServices, RuntimeFailure, ScopeId};
use swallowtail_transport_http::{
    HttpRequest, HttpTransport, Method as HttpMethod, SecretHeader, TransportError,
};
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
        request_url(&endpoint, &request)?;
        let method = match request.method {
            Method::Get => HttpMethod::Get,
            Method::Post => HttpMethod::Post,
        };
        let mut shared = HttpRequest::new(method, request.path.as_str())
            .with_response_limit(MAX_HTTP_BODY_BYTES);
        if request.query.is_some() {
            shared = shared.with_query("status", "pending");
        }
        if let Some(bearer) = bearer {
            shared = shared.with_secret_header(
                SecretHeader::bearer(bearer).map_err(|_| credential_failure())?,
            );
        }
        if let Some(body) = &request.body {
            shared = shared
                .with_header("content-type", "application/json")
                .with_body(body.clone());
        }
        let response = HttpTransport::new()
            .execute(scope, &endpoint, shared, services, cancelled)
            .await
            .map_err(transport_failure)?;
        let status = u16::try_from(response.status).map_err(|_| {
            failure(
                "swallowtail.kimi.local_server.status_invalid",
                "Kimi local-server returned an invalid HTTP status",
            )
        })?;
        Ok(Response {
            status,
            body: response.body,
        })
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

fn transport_failure(error: TransportError) -> RuntimeFailure {
    match error {
        TransportError::BlockingServiceMissing => failure(
            "swallowtail.kimi.local_server.blocking_service_missing",
            "Kimi local-server HTTP requires a blocking-work service",
        ),
        TransportError::BlockingResultMissing => failure(
            "swallowtail.kimi.local_server.blocking_result_missing",
            "Kimi local-server blocking HTTP work did not return a result",
        ),
        TransportError::EndpointInvalid => endpoint_failure(),
        TransportError::CredentialInvalid => credential_failure(),
        TransportError::ResponseLimit => failure(
            "swallowtail.kimi.local_server.response_limit",
            "Kimi local-server response exceeded the bounded input limit",
        ),
        TransportError::Cancelled => failure(
            "swallowtail.kimi.local_server.request_cancelled",
            "Kimi local-server HTTP request was cancelled",
        ),
        TransportError::Transport => failure(
            "swallowtail.kimi.local_server.transport_failed",
            "Kimi local-server HTTP transport failed",
        ),
    }
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
