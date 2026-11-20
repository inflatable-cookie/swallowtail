use crate::failure::failure;
use crate::protocol::{Method, Request, Response, SseDecoder, SseFrame, require_success};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::task::{Context, Poll};
use swallowtail_runtime::{HostServices, RuntimeFailure, ScopeId};
use swallowtail_transport_http::{
    BodyDecoder, HttpRequest, HttpTransport, Method as HttpMethod, StreamError,
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
        request: Request,
        services: &HostServices,
        cancelled: Arc<AtomicBool>,
    ) -> Result<Response, RuntimeFailure> {
        let shared = shared_request(&request, false)?;
        check_endpoint(&shared, &endpoint)?;
        let response = HttpTransport::new()
            .execute(scope, &endpoint, shared, services, cancelled)
            .await
            .map_err(transport_failure)?;
        Ok(Response {
            status: response.status,
            body: response.body,
        })
    }

    pub(crate) fn subscribe(
        &self,
        scope: ScopeId,
        endpoint: String,
        request: Request,
        services: &HostServices,
        cancelled: Arc<AtomicBool>,
    ) -> Result<Subscription, RuntimeFailure> {
        let shared = shared_request(&request, false)?;
        check_endpoint(&shared, &endpoint)?;
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

pub(crate) struct Subscription(SharedSubscription<SseFrame, RuntimeFailure>);

impl Subscription {
    pub(crate) async fn close(self) -> Result<(), RuntimeFailure> {
        self.0.close().await.map_err(stream_failure)
    }

    pub(crate) fn poll_next(
        &mut self,
        context: &mut Context<'_>,
    ) -> Poll<Option<Result<SseFrame, RuntimeFailure>>> {
        loop {
            match self.0.poll_next(context) {
                Poll::Ready(Some(Ok(SharedStreamItem::Body(frame)))) => {
                    return Poll::Ready(Some(Ok(frame)));
                }
                Poll::Ready(Some(Ok(SharedStreamItem::Headers(_)))) => {}
                Poll::Ready(Some(Err(error))) => {
                    return Poll::Ready(Some(Err(stream_failure(error))));
                }
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Pending => return Poll::Pending,
            }
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

fn shared_request(request: &Request, _stream: bool) -> Result<HttpRequest, RuntimeFailure> {
    let method = match request.method {
        Method::Get => HttpMethod::Get,
        Method::Post => HttpMethod::Post,
    };
    let mut shared =
        HttpRequest::new(method, request.path.as_str()).with_response_limit(RESPONSE_LIMIT);
    if request.method == Method::Post {
        shared = shared
            .with_header("content-type", "application/json")
            .with_header("accept", "text/event-stream");
        if let Some(body) = &request.body {
            shared = shared.with_body(body.clone());
        }
    }
    Ok(shared)
}

fn check_endpoint(request: &HttpRequest, endpoint: &str) -> Result<(), RuntimeFailure> {
    let url = request.resolve(endpoint).map_err(transport_failure)?;
    if url.scheme() != "http" || url.host_str() != Some("127.0.0.1") {
        return Err(endpoint_failure());
    }
    Ok(())
}

fn endpoint_failure() -> RuntimeFailure {
    failure(
        "swallowtail.llama_cpp.endpoint_invalid",
        "llama.cpp endpoint grant was invalid",
    )
}

fn transport_failure(error: TransportError) -> RuntimeFailure {
    match error {
        TransportError::BlockingServiceMissing => failure(
            "swallowtail.llama_cpp.blocking_service_missing",
            "llama.cpp HTTP requires a blocking-work service",
        ),
        TransportError::BlockingResultMissing => failure(
            "swallowtail.llama_cpp.blocking_result_missing",
            "llama.cpp blocking HTTP work returned no result",
        ),
        TransportError::EndpointInvalid => endpoint_failure(),
        TransportError::CredentialInvalid => failure(
            "swallowtail.llama_cpp.credential_invalid",
            "llama.cpp request carried an invalid credential",
        ),
        TransportError::ResponseLimit => failure(
            "swallowtail.llama_cpp.response_limit",
            "llama.cpp HTTP response exceeded its limit",
        ),
        TransportError::Cancelled => failure(
            "swallowtail.llama_cpp.cancelled",
            "llama.cpp HTTP work was cancelled",
        ),
        TransportError::Transport => failure(
            "swallowtail.llama_cpp.transport_failed",
            "llama.cpp HTTP transport failed",
        ),
    }
}

fn stream_failure(error: StreamError<RuntimeFailure>) -> RuntimeFailure {
    match error {
        StreamError::Transport(error) => transport_failure(error),
        StreamError::Decode(error) => error,
        StreamError::Status { status, body, .. } => {
            require_success(&Response { status, body }, "chat request")
                .err()
                .unwrap_or_else(|| {
                    failure(
                        "swallowtail.llama_cpp.transport_failed",
                        "llama.cpp stream ended without a response",
                    )
                })
        }
    }
}
