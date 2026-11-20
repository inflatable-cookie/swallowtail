use crate::AlibabaProtocolFailure;
use crate::failure::protocol;
use crate::protocol::{Method, SseDecoder, SseFrame, WireRequest, parse_provider_failure};
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::task::{Context, Poll};
use swallowtail_core::{
    FailureClassification, FailureKind, FailureOrigin, FailureRecovery, ProviderRequestRef,
};
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
        request: WireRequest,
        services: &HostServices,
        cancelled: Arc<AtomicBool>,
    ) -> Result<Response, RuntimeFailure> {
        let shared = shared_request(&request, credential, false)?;
        let response = HttpTransport::new()
            .execute(scope, &endpoint, shared, services, cancelled)
            .await
            .map_err(transport_failure)?;
        require_success(Response {
            status: response.status,
            body: response.body,
        })
    }

    pub(crate) fn subscribe(
        &self,
        scope: ScopeId,
        endpoint: String,
        credential: Vec<u8>,
        request: WireRequest,
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

pub(crate) struct Response {
    pub(crate) status: u32,
    pub(crate) body: Vec<u8>,
}

pub(crate) enum StreamItem {
    Correlation(ProviderRequestRef),
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
                match request_correlation(&headers) {
                    Ok(reference) => Poll::Ready(Some(Ok(StreamItem::Correlation(reference)))),
                    Err(error) => Poll::Ready(Some(Err(error))),
                }
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
        SseDecoder::push(self, chunk).map_err(protocol_runtime)
    }

    fn finish(self) -> Result<(), Self::Error> {
        SseDecoder::finish(self).map_err(protocol_runtime)
    }
}

fn shared_request(
    request: &WireRequest,
    credential: Vec<u8>,
    sse: bool,
) -> Result<HttpRequest, RuntimeFailure> {
    let method = match request.method() {
        Method::Get => HttpMethod::Get,
        Method::Post => HttpMethod::Post,
        Method::Delete => HttpMethod::Delete,
    };
    let mut shared = HttpRequest::new(method, request.path())
        .with_response_limit(RESPONSE_LIMIT)
        .capture_headers(|_| true)
        .with_secret_header(SecretHeader::bearer(credential).map_err(transport_failure)?)
        .with_header(
            "accept",
            if sse {
                "text/event-stream"
            } else {
                "application/json"
            },
        );
    if request.body().is_some() {
        shared = shared.with_header("content-type", "application/json");
    }
    if request.method() == Method::Post {
        let body = serde_json::to_vec(request.body().expect("POST body exists"))
            .map_err(|_| protocol_failure())?;
        shared = shared.with_body(body);
    }
    if sse {
        shared = shared.stream_response_headers();
    }
    Ok(shared)
}

fn request_correlation(
    headers: &BTreeMap<String, String>,
) -> Result<ProviderRequestRef, RuntimeFailure> {
    headers
        .get("x-request-id")
        .ok_or_else(protocol_failure)
        .and_then(|value| ProviderRequestRef::new(value.clone()).map_err(|_| protocol_failure()))
}

fn require_success(response: Response) -> Result<Response, RuntimeFailure> {
    match response.status {
        200..=299 => Ok(response),
        404 => Err(failure(
            "swallowtail.alibaba_model_studio.provider_resource_missing",
            "Alibaba Model Studio reported that the requested provider resource was missing",
        )
        .with_failure_classification(FailureClassification::new(
            FailureOrigin::Provider,
            FailureKind::ResourceNotFound,
            FailureRecovery::ConfigurationChangeRequired,
        ))),
        401 | 403 => Err(failure(
            "swallowtail.alibaba_model_studio.provider_access_rejected",
            "Alibaba Model Studio rejected access to the requested provider resource",
        )
        .with_failure_classification(FailureClassification::new(
            FailureOrigin::Provider,
            FailureKind::AuthorizationDenied,
            FailureRecovery::ReauthenticationRequired,
        ))),
        _ => Err(provider_error(&response.body)),
    }
}

fn provider_error(body: &[u8]) -> RuntimeFailure {
    parse_provider_failure(body).map_or_else(protocol_runtime, protocol_runtime)
}

fn protocol_runtime(error: AlibabaProtocolFailure) -> RuntimeFailure {
    protocol(error)
}

fn failure(code: &'static str, message: impl Into<String>) -> RuntimeFailure {
    RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(code, message))
}

fn endpoint_failure() -> RuntimeFailure {
    failure(
        "swallowtail.alibaba_model_studio.endpoint_invalid",
        "Alibaba Model Studio authorized endpoint was invalid",
    )
}

fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.alibaba_model_studio.http_protocol_invalid",
        "Alibaba Model Studio HTTP response was invalid",
    )
}

fn transport_failure(error: TransportError) -> RuntimeFailure {
    match error {
        TransportError::BlockingServiceMissing => failure(
            "swallowtail.alibaba_model_studio.blocking_service_missing",
            "Alibaba Model Studio HTTP requires a blocking-work service",
        ),
        TransportError::BlockingResultMissing => failure(
            "swallowtail.alibaba_model_studio.blocking_result_missing",
            "Alibaba Model Studio blocking HTTP work returned no result",
        ),
        TransportError::EndpointInvalid => endpoint_failure(),
        TransportError::CredentialInvalid => failure(
            "swallowtail.alibaba_model_studio.credential_invalid",
            "Alibaba Model Studio API-key credential was invalid",
        ),
        TransportError::ResponseLimit => failure(
            "swallowtail.alibaba_model_studio.response_limit_exceeded",
            "Alibaba Model Studio response exceeded its byte limit",
        ),
        TransportError::Cancelled => failure(
            "swallowtail.alibaba_model_studio.connection_cancelled",
            "Alibaba Model Studio local connection was cancelled",
        ),
        TransportError::Transport => failure(
            "swallowtail.alibaba_model_studio.http_failed",
            "Alibaba Model Studio HTTP work failed",
        ),
    }
}

fn stream_failure(error: StreamError<RuntimeFailure>) -> RuntimeFailure {
    match error {
        StreamError::Transport(error) => transport_failure(error),
        StreamError::Decode(error) => error,
        StreamError::Status { body, .. } => provider_error(&body),
    }
}
