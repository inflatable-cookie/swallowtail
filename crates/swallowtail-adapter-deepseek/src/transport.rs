use crate::failure::{failure, provider};
use crate::protocol::{HttpRequest as ProtocolRequest, HttpResponse, Method, ProviderFailureKind};
use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::task::{Context, Poll};
use swallowtail_runtime::{HostServices, RuntimeFailure, ScopeId};
use swallowtail_transport_http::{
    BodyDecoder, HttpRequest, HttpTransport, Method as HttpMethod, SecretHeader, StreamError,
    StreamItem as SharedStreamItem, Subscription as SharedSubscription, TransportError,
};

const RESPONSE_LIMIT: usize = 1024 * 1024;

#[derive(Clone, Default)]
pub(crate) struct CurlTransport;

impl CurlTransport {
    pub(crate) async fn request(
        &self,
        scope: ScopeId,
        endpoint: String,
        credential: Vec<u8>,
        request: ProtocolRequest,
        services: &HostServices,
        cancelled: Arc<AtomicBool>,
    ) -> Result<HttpResponse, RuntimeFailure> {
        let shared = shared_request(&request, credential, false)?;
        let response = HttpTransport::new()
            .execute(scope, &endpoint, shared, services, cancelled)
            .await
            .map_err(transport_failure)?;
        Ok(HttpResponse {
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
        request: ProtocolRequest,
        services: &HostServices,
        cancelled: Arc<AtomicBool>,
    ) -> Result<Subscription, RuntimeFailure> {
        let shared = shared_request(&request, credential, true)?;
        let subscription = HttpTransport::new()
            .subscribe(scope, &endpoint, shared, ChunkDecoder, services, cancelled)
            .map_err(transport_failure)?;
        Ok(Subscription(subscription))
    }
}

#[derive(Debug)]
pub(crate) enum StreamItem {
    Metadata(BTreeMap<String, String>),
    Data(Vec<u8>),
}

pub(crate) struct Subscription(SharedSubscription<Vec<u8>, RuntimeFailure>);

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
                Poll::Ready(Some(Ok(StreamItem::Metadata(headers))))
            }
            Poll::Ready(Some(Ok(SharedStreamItem::Body(chunk)))) => {
                Poll::Ready(Some(Ok(StreamItem::Data(chunk))))
            }
            Poll::Ready(Some(Err(error))) => Poll::Ready(Some(Err(stream_failure(error)))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// DeepSeek keeps its own stream grammar, so the transport only frames bytes.
struct ChunkDecoder;

impl BodyDecoder for ChunkDecoder {
    type Item = Vec<u8>;
    type Error = RuntimeFailure;

    fn push(&mut self, chunk: &[u8]) -> Result<Vec<Self::Item>, Self::Error> {
        Ok(vec![chunk.to_vec()])
    }

    fn finish(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

fn shared_request(
    request: &ProtocolRequest,
    credential: Vec<u8>,
    stream: bool,
) -> Result<HttpRequest, RuntimeFailure> {
    let method = match request.method {
        Method::Get => HttpMethod::Get,
        Method::Post => HttpMethod::Post,
    };
    let mut shared = HttpRequest::new(method, request.path)
        .with_response_limit(RESPONSE_LIMIT)
        .capture_headers(capture_all)
        .with_secret_header(SecretHeader::bearer(credential).map_err(transport_failure)?);
    if request.method == Method::Post {
        shared = shared
            .with_header("content-type", "application/json")
            .with_header(
                "accept",
                if request.stream {
                    "text/event-stream"
                } else {
                    "application/json"
                },
            );
        if stream {
            shared = shared.stream_response_headers();
        }
        if let Some(body) = &request.body {
            shared = shared.with_body(body.clone());
        }
    }
    Ok(shared)
}

fn capture_all(_name: &str) -> bool {
    true
}

fn transport_failure(error: TransportError) -> RuntimeFailure {
    match error {
        TransportError::BlockingServiceMissing => failure(
            "swallowtail.deepseek.blocking_service_missing",
            "DeepSeek HTTP requires a blocking-work service",
        ),
        TransportError::BlockingResultMissing => failure(
            "swallowtail.deepseek.blocking_result_missing",
            "DeepSeek blocking HTTP work returned no result",
        ),
        TransportError::EndpointInvalid => failure(
            "swallowtail.deepseek.endpoint_invalid",
            "DeepSeek endpoint was invalid",
        ),
        TransportError::CredentialInvalid => failure(
            "swallowtail.deepseek.credential_invalid",
            "DeepSeek API-key credential was invalid",
        ),
        TransportError::ResponseLimit => failure(
            "swallowtail.deepseek.response_limit",
            "DeepSeek response exceeded its byte bound",
        ),
        TransportError::Cancelled => failure(
            "swallowtail.deepseek.cancelled",
            "DeepSeek request was cancelled locally",
        ),
        TransportError::Transport => failure(
            "swallowtail.deepseek.transport_failed",
            "DeepSeek HTTP transport failed",
        ),
    }
}

fn stream_failure(error: StreamError<RuntimeFailure>) -> RuntimeFailure {
    match error {
        StreamError::Transport(error) => transport_failure(error),
        StreamError::Decode(error) => error,
        StreamError::Status { status, .. } => provider(
            crate::protocol::classify_failure(status as u16)
                .unwrap_or(ProviderFailureKind::Provider),
        ),
    }
}
