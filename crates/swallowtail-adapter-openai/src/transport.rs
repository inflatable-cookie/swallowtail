use crate::failure::failure;
use crate::protocol::{Method, Request, Response, SseDecoder, SseFrame, require_success};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::task::{Context, Poll};
use std::time::Duration;
use swallowtail_runtime::{HostServices, RuntimeFailure, ScopeId};
use swallowtail_transport_http::{
    BodyDecoder, HttpRequest, HttpTransport, Method as HttpMethod, SecretHeader, StreamError,
    StreamItem as SharedStreamItem, Subscription as SharedSubscription, TransportError,
};

const RESPONSE_LIMIT: usize = 4 * 1024 * 1024;
const MANAGEMENT_TIMEOUT: Duration = Duration::from_secs(5);

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
    Headers(BTreeMap),
    Frame(SseFrame),
}

type BTreeMap = std::collections::BTreeMap<String, String>;

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
        Method::Delete => HttpMethod::Delete,
        Method::Get => HttpMethod::Get,
        Method::Post => HttpMethod::Post,
    };
    let mut shared = HttpRequest::new(method, request.path.as_str())
        .with_response_limit(RESPONSE_LIMIT)
        .capture_headers(capture_metadata)
        .with_secret_header(SecretHeader::bearer(credential).map_err(transport_failure)?)
        .with_header("content-type", "application/json")
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
    if stream {
        shared = shared.stream_response_headers();
    } else {
        shared = shared.with_transfer_timeout(MANAGEMENT_TIMEOUT);
    }
    if let Some(body) = &request.body {
        shared = shared.with_body(body.clone());
    }
    Ok(shared)
}

fn capture_metadata(name: &str) -> bool {
    name == "x-request-id" || name.starts_with("x-ratelimit-")
}

fn transport_failure(error: TransportError) -> RuntimeFailure {
    match error {
        TransportError::BlockingServiceMissing => failure(
            "swallowtail.openai.blocking_service_missing",
            "OpenAI HTTP requires a blocking-work service",
        ),
        TransportError::BlockingResultMissing => failure(
            "swallowtail.openai.blocking_result_missing",
            "OpenAI blocking HTTP work returned no result",
        ),
        TransportError::EndpointInvalid => failure(
            "swallowtail.openai.endpoint_invalid",
            "Host-approved OpenAI endpoint was not an eligible HTTP endpoint",
        ),
        TransportError::CredentialInvalid => failure(
            "swallowtail.openai.credential_invalid",
            "OpenAI API-key credential was invalid",
        ),
        TransportError::ResponseLimit => failure(
            "swallowtail.openai.response_limit",
            "OpenAI response exceeded the bounded input limit",
        ),
        TransportError::Cancelled => failure(
            "swallowtail.openai.request_cancelled",
            "OpenAI HTTP request was cancelled",
        ),
        TransportError::Transport => failure(
            "swallowtail.openai.transport_failed",
            "OpenAI HTTP transport failed",
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
        } => require_success(&Response {
            status,
            headers,
            body,
        })
        .err()
        .unwrap_or_else(|| {
            failure(
                "swallowtail.openai.transport_failed",
                "OpenAI stream ended without a response",
            )
        }),
    }
}
