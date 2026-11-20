use crate::failure::failure;
use crate::protocol::{
    API_VERSION, Method, Request, Response, SseDecoder, SseFrame, require_success,
};
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::task::{Context, Poll};
use swallowtail_runtime::{HostServices, RuntimeFailure, ScopeId};
use swallowtail_transport_http::{
    BodyDecoder, HttpRequest, HttpTransport, Method as HttpMethod, SecretHeader, StreamError,
    StreamItem as SharedStreamItem, Subscription as SharedSubscription, TransportError,
};

const RESPONSE_LIMIT: usize = 4 * 1024 * 1024;

#[derive(Clone, Default)]
pub(crate) struct CurlTransport;

impl CurlTransport {
    pub(crate) async fn request(
        &self,
        scope: ScopeId,
        endpoint: String,
        credential: Vec<u8>,
        request: Request,
        services: &HostServices,
        cancelled: Arc<AtomicBool>,
    ) -> Result<Response, RuntimeFailure> {
        let shared = shared_request(&request, credential, false)?;
        let response = HttpTransport::new()
            .execute(scope, &endpoint, shared, services, cancelled)
            .await
            .map_err(transport_failure)?;
        Ok(Response {
            status: response.status,
            _headers: response.headers,
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
    ) -> Result<Subscription, RuntimeFailure> {
        let shared = shared_request(&request, credential, true)?;
        let subscription = HttpTransport::new()
            .subscribe(
                scope,
                &endpoint,
                shared,
                SseDecoder::default(),
                services,
                cancelled,
            )
            .map_err(transport_failure)?;
        Ok(Subscription(subscription))
    }
}

#[derive(Debug)]
pub(crate) enum StreamItem {
    Headers(BTreeMap<String, String>),
    Frame(SseFrame),
}

pub(crate) struct Subscription(SharedSubscription<SseFrame, RuntimeFailure>);

impl Subscription {
    pub(crate) async fn close(self) -> Result<(), RuntimeFailure> {
        self.0.close().await.map_err(stream_failure)
    }

    pub(crate) fn poll_next(
        &mut self,
        context: &mut Context<'_>,
    ) -> Poll<Option<Result<StreamItem, RuntimeFailure>>> {
        match self.0.poll_next(context) {
            Poll::Ready(Some(Ok(SharedStreamItem::Headers(headers)))) => {
                Poll::Ready(Some(Ok(StreamItem::Headers(headers))))
            }
            Poll::Ready(Some(Ok(SharedStreamItem::Body(frame)))) => {
                Poll::Ready(Some(Ok(StreamItem::Frame(frame))))
            }
            Poll::Ready(Some(Err(error))) => Poll::Ready(Some(Err(stream_failure(error)))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl BodyDecoder for SseDecoder {
    type Item = SseFrame;
    type Error = RuntimeFailure;

    fn push(&mut self, chunk: &[u8]) -> Result<Vec<Self::Item>, Self::Error> {
        SseDecoder::push(self, chunk)
    }

    fn finish(self) -> Result<(), Self::Error> {
        SseDecoder::finish(self)
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
    };
    let mut shared = HttpRequest::new(method, request.path.as_str())
        .with_response_limit(RESPONSE_LIMIT)
        .capture_headers(capture_metadata)
        .with_secret_header(
            SecretHeader::prefixed("x-api-key", "", credential).map_err(transport_failure)?,
        )
        .with_header("anthropic-version", API_VERSION);
    for (name, value) in &request.query {
        shared = shared.with_query(name.as_str(), value.as_str());
    }
    if request.method == Method::Post {
        shared = shared
            .with_header("content-type", "application/json")
            .with_header("accept", "text/event-stream");
        if let Some(body) = &request.body {
            shared = shared.with_body(body.clone_bytes());
        }
    }
    if stream {
        shared = shared.stream_response_headers();
    }
    Ok(shared)
}

fn capture_metadata(name: &str) -> bool {
    name == "request-id" || name.starts_with("anthropic-ratelimit-")
}

fn transport_failure(error: TransportError) -> RuntimeFailure {
    match error {
        TransportError::BlockingServiceMissing => failure(
            "swallowtail.anthropic.blocking_service_missing",
            "Anthropic HTTP requires a blocking-work service",
        ),
        TransportError::BlockingResultMissing => failure(
            "swallowtail.anthropic.blocking_result_missing",
            "Anthropic blocking HTTP work returned no result",
        ),
        TransportError::EndpointInvalid => endpoint_failure(),
        TransportError::CredentialInvalid => failure(
            "swallowtail.anthropic.credential_invalid",
            "Anthropic API-key credential was invalid",
        ),
        TransportError::ResponseLimit => failure(
            "swallowtail.anthropic.response_limit",
            "Anthropic response exceeded the bounded input limit",
        ),
        TransportError::Cancelled => failure(
            "swallowtail.anthropic.request_cancelled",
            "Anthropic HTTP request was cancelled",
        ),
        TransportError::Transport => failure(
            "swallowtail.anthropic.transport_failed",
            "Anthropic HTTP transport failed",
        ),
    }
}

fn stream_failure(error: StreamError<RuntimeFailure>) -> RuntimeFailure {
    match error {
        StreamError::Transport(error) => transport_failure(error),
        StreamError::Decode(error) => error,
        StreamError::Status {
            status,
            headers,
            body,
        } => require_success(
            &Response {
                status,
                _headers: headers,
                body,
            },
            "message request",
        )
        .err()
        .unwrap_or_else(|| {
            failure(
                "swallowtail.anthropic.transport_failed",
                "Anthropic stream ended without a response",
            )
        }),
    }
}

fn endpoint_failure() -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.endpoint_invalid",
        "Host-approved Anthropic endpoint was not an eligible HTTP endpoint",
    )
}
