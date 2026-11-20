use crate::failure::failure;
use crate::managed::{API_VERSION, BETA_VERSION, Method, Request};
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::task::{Context, Poll};
use swallowtail_runtime::{HostServices, RuntimeFailure, ScopeId};
use swallowtail_transport_http::{
    BodyDecoder, HttpRequest, HttpTransport, Method as HttpMethod, SecretHeader, SseDecodeError,
    SseDecodeErrorKind, SseFramer, StreamError, StreamItem as SharedStreamItem,
    Subscription as SharedSubscription, TransportError,
};

const RESPONSE_LIMIT: usize = 4 * 1024 * 1024;
const MANAGEMENT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);

#[derive(Clone, Default)]
pub(crate) struct ManagedCurlTransport;

impl ManagedCurlTransport {
    pub(crate) async fn request(
        &self,
        scope: ScopeId,
        endpoint: String,
        credential: Vec<u8>,
        request: Request,
        services: &HostServices,
        cancelled: Arc<AtomicBool>,
    ) -> Result<ManagedResponse, RuntimeFailure> {
        let shared = shared_request(&request, credential, false)?;
        let response = HttpTransport::new()
            .execute(scope, &endpoint, shared, services, cancelled)
            .await
            .map_err(transport_failure)?;
        Ok(ManagedResponse {
            status: response.status,
            headers: response.headers,
            body: response.body,
        })
    }

    pub(crate) fn subscribe(
        &self,
        scope: ScopeId,
        endpoint: String,
        credential: Vec<u8>,
        request: Request,
        services: &HostServices,
        cancelled: Arc<AtomicBool>,
    ) -> Result<ManagedSubscription, RuntimeFailure> {
        let shared = shared_request(&request, credential, true)?;
        let subscription = HttpTransport::new()
            .subscribe(
                scope,
                &endpoint,
                shared,
                RawSseDecoder::default(),
                services,
                cancelled,
            )
            .map_err(transport_failure)?;
        Ok(ManagedSubscription(subscription))
    }
}

#[derive(Debug)]
pub(crate) struct ManagedResponse {
    pub status: u32,
    pub headers: BTreeMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Debug)]
pub(crate) enum ManagedStreamItem {
    Headers(BTreeMap<String, String>),
    Frame(String),
}

pub(crate) struct ManagedSubscription(SharedSubscription<String, RuntimeFailure>);

impl ManagedSubscription {
    pub(crate) async fn close(self) -> Result<(), RuntimeFailure> {
        self.0.close().await.map_err(stream_failure)
    }

    pub(crate) fn poll_next(
        &mut self,
        context: &mut Context<'_>,
    ) -> Poll<Option<Result<ManagedStreamItem, RuntimeFailure>>> {
        match self.0.poll_next(context) {
            Poll::Ready(Some(Ok(SharedStreamItem::Headers(headers)))) => {
                Poll::Ready(Some(Ok(ManagedStreamItem::Headers(headers))))
            }
            Poll::Ready(Some(Ok(SharedStreamItem::Body(frame)))) => {
                Poll::Ready(Some(Ok(ManagedStreamItem::Frame(frame))))
            }
            Poll::Ready(Some(Err(error))) => Poll::Ready(Some(Err(stream_failure(error)))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Managed Agents consumes whole normalized event frames, not decoded names.
struct RawSseDecoder(SseFramer);

impl Default for RawSseDecoder {
    fn default() -> Self {
        Self(
            SseFramer::new()
                .without_event_limit()
                .with_stream_limit(RESPONSE_LIMIT),
        )
    }
}

impl BodyDecoder for RawSseDecoder {
    type Item = String;
    type Error = RuntimeFailure;

    fn push(&mut self, chunk: &[u8]) -> Result<Vec<Self::Item>, Self::Error> {
        self.0
            .push(chunk)
            .map_err(map_sse_failure)?
            .into_iter()
            .map(|frame| String::from_utf8(frame).map_err(|_| protocol_failure()))
            .collect()
    }

    fn finish(self) -> Result<(), Self::Error> {
        self.0.finish().map_err(map_sse_failure)
    }
}

fn map_sse_failure(error: SseDecodeError) -> RuntimeFailure {
    match error.kind() {
        SseDecodeErrorKind::LimitExceeded => response_limit(),
        SseDecodeErrorKind::InvalidUtf8 | SseDecodeErrorKind::IncompleteFrame => protocol_failure(),
    }
}

fn shared_request(
    request: &Request,
    credential: Vec<u8>,
    stream: bool,
) -> Result<HttpRequest, RuntimeFailure> {
    let method = match request.method {
        Method::Get => HttpMethod::Get,
        Method::Post => HttpMethod::Post,
        Method::Delete => HttpMethod::Delete,
    };
    let mut shared = HttpRequest::new(method, request.path.as_str())
        .with_response_limit(RESPONSE_LIMIT)
        .capture_headers(capture_metadata)
        .with_secret_header(
            SecretHeader::prefixed("x-api-key", "", credential).map_err(transport_failure)?,
        )
        .with_header("anthropic-version", API_VERSION)
        .with_header("anthropic-beta", BETA_VERSION)
        .with_header(
            "accept",
            if request.expects_stream() {
                "text/event-stream"
            } else {
                "application/json"
            },
        );
    for (name, value) in &request.query {
        shared = shared.with_query(name.as_str(), value.as_str());
    }
    if request.method == Method::Post {
        shared = shared.with_header("content-type", "application/json");
        if let Some(body) = &request.body {
            shared = shared.with_body(body.clone());
        }
    }
    if stream {
        shared = shared.stream_response_headers();
    } else {
        shared = shared.with_transfer_timeout(MANAGEMENT_TIMEOUT);
    }
    Ok(shared)
}

fn capture_metadata(name: &str) -> bool {
    name == "request-id" || name.starts_with("anthropic-ratelimit-")
}

fn transport_failure(error: TransportError) -> RuntimeFailure {
    match error {
        TransportError::BlockingServiceMissing => missing(),
        TransportError::BlockingResultMissing => failure(
            "swallowtail.anthropic.managed.blocking_result_missing",
            "Anthropic Managed Agents blocking HTTP work returned no result",
        ),
        TransportError::EndpointInvalid => endpoint_failure(),
        TransportError::CredentialInvalid => failure(
            "swallowtail.anthropic.managed.credential_invalid",
            "Anthropic Managed Agents API-key credential was invalid",
        ),
        TransportError::ResponseLimit => response_limit(),
        TransportError::Cancelled => cancelled_failure(),
        TransportError::Transport => curl_failure(),
    }
}

fn stream_failure(error: StreamError<RuntimeFailure>) -> RuntimeFailure {
    match error {
        StreamError::Transport(error) => transport_failure(error),
        StreamError::Decode(error) => error,
        StreamError::Status { .. } => provider_failure("event stream"),
    }
}

fn missing() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.managed.blocking_service_missing",
        "Anthropic Managed Agents HTTP requires a blocking-work service",
    )
}

fn endpoint_failure() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.managed.endpoint_invalid",
        "Host-approved Anthropic Managed Agents endpoint was not eligible",
    )
}

fn response_limit() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.managed.response_limit",
        "Anthropic Managed Agents response exceeded the bounded input limit",
    )
}

fn cancelled_failure() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.managed.request_cancelled",
        "Anthropic Managed Agents HTTP request was cancelled",
    )
}

fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.managed.protocol_invalid",
        "Anthropic Managed Agents event stream framing was invalid",
    )
}

fn provider_failure(operation: &str) -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.managed.provider_failed",
        format!("Anthropic Managed Agents {operation} failed"),
    )
}

fn curl_failure() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.managed.transport_failed",
        "Anthropic Managed Agents HTTP transport failed",
    )
}
