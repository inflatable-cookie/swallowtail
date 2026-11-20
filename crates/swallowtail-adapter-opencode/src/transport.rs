use crate::failure::failure;
use crate::protocol::{Method, Request, Response, SseDecoder};
use std::collections::BTreeMap;
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
        let shared = shared_request(&request)?;
        let response = HttpTransport::new()
            .execute(scope, &endpoint, shared, services, cancelled)
            .await
            .map_err(transport_failure)?;
        Ok(Response {
            status: response.status,
            body: response.body,
            next_cursor: pagination_cursor(&response.headers)?,
        })
    }

    pub(crate) async fn subscribe(
        &self,
        scope: ScopeId,
        endpoint: String,
        directory: String,
        services: &HostServices,
        cancelled: Arc<AtomicBool>,
    ) -> Result<Subscription, RuntimeFailure> {
        let request = Request::get("/event").with_directory(&directory);
        let shared = shared_request(&request)?;
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

pub(crate) struct Subscription(SharedSubscription<Vec<u8>, RuntimeFailure>);

impl Subscription {
    pub(crate) async fn close(self) -> Result<(), RuntimeFailure> {
        self.0.close().await.map_err(stream_failure)
    }

    pub(crate) fn poll_next(
        &mut self,
        context: &mut Context<'_>,
    ) -> Poll<Option<Result<Vec<u8>, RuntimeFailure>>> {
        loop {
            match self.0.poll_next(context) {
                Poll::Ready(Some(Ok(SharedStreamItem::Body(event)))) => {
                    return Poll::Ready(Some(Ok(event)));
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
    type Item = Vec<u8>;
    type Error = RuntimeFailure;

    fn push(&mut self, chunk: &[u8]) -> Result<Vec<Self::Item>, Self::Error> {
        SseDecoder::push(self, chunk)
    }

    fn finish(self) -> Result<(), Self::Error> {
        SseDecoder::finish(self)
    }
}

fn shared_request(request: &Request) -> Result<HttpRequest, RuntimeFailure> {
    let method = match request.method {
        Method::Delete => HttpMethod::Delete,
        Method::Get => HttpMethod::Get,
        Method::Post => HttpMethod::Post,
    };
    let mut shared = HttpRequest::new(method, request.path.as_str())
        .with_response_limit(RESPONSE_LIMIT)
        .capture_headers(capture_cursor);
    for (name, value) in &request.query {
        shared = shared.with_query(name.as_str(), value.as_str());
    }
    match request.method {
        Method::Post => {
            if let Some(body) = &request.body {
                shared = shared
                    .with_header("content-type", "application/json")
                    .with_body(body.clone());
            }
        }
        Method::Get if request.path == "/event" => {
            shared = shared.with_header("accept", "text/event-stream");
        }
        Method::Get | Method::Delete => {}
    }
    Ok(shared)
}

fn capture_cursor(name: &str) -> bool {
    name == "x-next-cursor"
}

fn pagination_cursor(headers: &BTreeMap<String, String>) -> Result<Option<String>, RuntimeFailure> {
    let Some(value) = headers.get("x-next-cursor") else {
        return Ok(None);
    };
    if value.is_empty() || value.len() > 4096 || !value.bytes().all(|byte| byte.is_ascii_graphic())
    {
        return Err(failure(
            "swallowtail.opencode.pagination_cursor_invalid",
            "OpenCode returned an invalid pagination cursor",
        ));
    }
    Ok(Some(value.clone()))
}

fn transport_failure(error: TransportError) -> RuntimeFailure {
    match error {
        TransportError::BlockingServiceMissing => failure(
            "swallowtail.opencode.blocking_service_missing",
            "OpenCode HTTP requires a blocking-work service",
        ),
        TransportError::BlockingResultMissing => failure(
            "swallowtail.opencode.blocking_result_missing",
            "OpenCode blocking HTTP work did not return a result",
        ),
        TransportError::EndpointInvalid => failure(
            "swallowtail.opencode.endpoint_invalid",
            "Host-approved OpenCode endpoint is not an eligible HTTP endpoint",
        ),
        TransportError::CredentialInvalid => failure(
            "swallowtail.opencode.credential_invalid",
            "OpenCode request carried an invalid credential",
        ),
        TransportError::ResponseLimit => failure(
            "swallowtail.opencode.response_limit",
            "OpenCode HTTP response exceeded the bounded input limit",
        ),
        TransportError::Cancelled => failure(
            "swallowtail.opencode.request_cancelled",
            "OpenCode HTTP request was cancelled",
        ),
        TransportError::Transport => failure(
            "swallowtail.opencode.transport_failed",
            "OpenCode HTTP transport failed",
        ),
    }
}

fn stream_failure(error: StreamError<RuntimeFailure>) -> RuntimeFailure {
    match error {
        StreamError::Transport(error) => transport_failure(error),
        StreamError::Decode(error) => error,
        StreamError::Status { .. } => failure(
            "swallowtail.opencode.sse_http_failed",
            "OpenCode SSE subscription failed",
        ),
    }
}

include!("transport/tests.rs");
