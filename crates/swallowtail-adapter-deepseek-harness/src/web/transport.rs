use crate::failure::failure;
use curl::easy::{Easy, List, WriteError};
use futures_channel::{mpsc, oneshot};
use futures_util::{Stream, future::poll_fn};
use serde_json::Value;
use std::net::TcpStream;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::Poll;
use std::time::Duration;
use swallowtail_runtime::{BoxFuture, Deadline, HostServices, RuntimeFailure, ScopeId};
use tungstenite::client::{IntoClientRequest, connect_with_config};
use tungstenite::http::{HeaderValue, header::ORIGIN};
use tungstenite::protocol::WebSocketConfig;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{Error as WebSocketError, Message, WebSocket};
use url::{Host, Url};

use super::protocol::{MAX_HTTP_BODY_BYTES, MAX_WEBSOCKET_FRAME_BYTES, decode_unary_response};

const HTTP_PATH_PREFIX: &str = "/api/";

#[derive(Clone, Default)]
pub(crate) struct WebApiTransport;

impl WebApiTransport {
    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn post_json(
        &self,
        scope: ScopeId,
        endpoint: String,
        path: String,
        body: Vec<u8>,
        rpc_id: String,
        services: &HostServices,
        deadline: Option<Deadline>,
        cancelled: Arc<AtomicBool>,
    ) -> Result<Value, RuntimeFailure> {
        let response = self
            .request(
                scope,
                endpoint,
                path,
                body,
                services,
                deadline,
                Arc::clone(&cancelled),
            )
            .await?;
        decode_unary_response(response.status, &response.body, &rpc_id)
    }

    #[allow(clippy::too_many_arguments)]
    async fn request(
        &self,
        scope: ScopeId,
        endpoint: String,
        path: String,
        body: Vec<u8>,
        services: &HostServices,
        deadline: Option<Deadline>,
        cancelled: Arc<AtomicBool>,
    ) -> Result<Response, RuntimeFailure> {
        validate_path(&path)?;
        let blocking = services.blocking_work().cloned().ok_or_else(|| {
            failure(
                "swallowtail.deepseek_harness.web.blocking_service_missing",
                "DeepSeek Harness Web HTTP requires a blocking-work service",
            )
        })?;
        let (sender, receiver) = oneshot::channel();
        let job_endpoint = endpoint.clone();
        let job_path = path.clone();
        let job_body = body.clone();
        let job_cancelled = Arc::clone(&cancelled);
        let work = blocking.run(
            scope,
            Box::new(move || {
                let result = perform_request(&job_endpoint, &job_path, &job_body, &job_cancelled);
                let _ = sender.send(result);
                Ok(())
            }),
        );
        wait_blocking(work, deadline, services, cancelled).await?;
        let result = receiver.await.map_err(|_| {
            failure(
                "swallowtail.deepseek_harness.web.blocking_result_missing",
                "DeepSeek Harness Web HTTP did not return a bounded result",
            )
        })??;
        Ok(result)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Response {
    pub(crate) status: u16,
    pub(crate) body: Vec<u8>,
}

pub(crate) struct WebSocketSubscription {
    updates: mpsc::Receiver<Result<Vec<u8>, RuntimeFailure>>,
    work: Option<BoxFuture<'static, Result<(), RuntimeFailure>>>,
    cancelled: Arc<AtomicBool>,
    first: Option<Vec<u8>>,
}

impl WebSocketSubscription {
    pub(crate) async fn open(
        scope: ScopeId,
        endpoint: String,
        path: String,
        services: &HostServices,
        deadline: Option<Deadline>,
        cancelled: Arc<AtomicBool>,
    ) -> Result<Self, RuntimeFailure> {
        validate_websocket_path(&path)?;
        let blocking = services.blocking_work().cloned().ok_or_else(|| {
            failure(
                "swallowtail.deepseek_harness.web.blocking_service_missing",
                "DeepSeek Harness WebSocket requires a blocking-work service",
            )
        })?;
        let (sender, receiver) = mpsc::channel(128);
        let worker_cancelled = Arc::clone(&cancelled);
        let work = blocking.run(
            scope,
            Box::new(move || run_websocket(endpoint, path, sender, worker_cancelled)),
        );
        let mut subscription = Self {
            updates: receiver,
            work: Some(work),
            cancelled,
            first: None,
        };
        if let Err(error) = subscription.wait_ready(deadline, services).await {
            let _ = subscription.close().await;
            return Err(error);
        }
        Ok(subscription)
    }

    pub(crate) async fn next_controlled(
        &mut self,
        deadline: Option<Deadline>,
        services: &HostServices,
    ) -> Result<Option<Vec<u8>>, RuntimeFailure> {
        if self.cancelled.load(Ordering::SeqCst) {
            return Err(cancelled_failure());
        }
        if let Some(first) = self.first.take() {
            return Ok(Some(first));
        }
        let mut timer = deadline
            .map(|deadline| {
                services
                    .time()
                    .ok_or_else(|| {
                        failure(
                            "swallowtail.deepseek_harness.web.time_service_missing",
                            "Deadline-bound DeepSeek Harness WebSocket requires a time service",
                        )
                    })
                    .map(|time| time.wait_until(deadline))
            })
            .transpose()?;
        poll_fn(|context| {
            if let Poll::Ready(item) = Pin::new(&mut self.updates).poll_next(context) {
                return Poll::Ready(match item {
                    Some(Ok(frame)) => Ok(Some(frame)),
                    Some(Err(error)) => Err(error),
                    None => Ok(None),
                });
            }
            if let Some(work) = self.work.as_mut() {
                match work.as_mut().poll(context) {
                    Poll::Ready(Ok(())) => return Poll::Ready(Ok(None)),
                    Poll::Ready(Err(error)) => return Poll::Ready(Err(error)),
                    Poll::Pending => {}
                }
            }
            if timer
                .as_mut()
                .is_some_and(|wait| wait.as_mut().poll(context).is_ready())
            {
                self.cancelled.store(true, Ordering::SeqCst);
                return Poll::Ready(Err(failure(
                    "swallowtail.deepseek_harness.web.websocket_timeout",
                    "DeepSeek Harness WebSocket operation exceeded its deadline",
                )));
            }
            Poll::Pending
        })
        .await
    }

    pub(crate) async fn close(mut self) -> Result<(), RuntimeFailure> {
        self.cancelled.store(true, Ordering::SeqCst);
        match self.work.take() {
            Some(work) => work.await,
            None => Ok(()),
        }
    }

    async fn wait_ready(
        &mut self,
        deadline: Option<Deadline>,
        services: &HostServices,
    ) -> Result<(), RuntimeFailure> {
        let mut timer = deadline
            .map(|deadline| {
                services
                    .time()
                    .ok_or_else(|| {
                        failure(
                            "swallowtail.deepseek_harness.web.time_service_missing",
                            "Deadline-bound DeepSeek Harness WebSocket requires a time service",
                        )
                    })
                    .map(|time| time.wait_until(deadline))
            })
            .transpose()?;
        poll_fn(|context| {
            if let Poll::Ready(item) = Pin::new(&mut self.updates).poll_next(context) {
                return Poll::Ready(match item {
                    Some(Ok(frame)) => {
                        self.first = Some(frame);
                        Ok(())
                    }
                    Some(Err(error)) => Err(error),
                    None => Err(disconnected()),
                });
            }
            if let Some(work) = self.work.as_mut() {
                match work.as_mut().poll(context) {
                    Poll::Ready(Ok(())) => return Poll::Ready(Err(disconnected())),
                    Poll::Ready(Err(error)) => return Poll::Ready(Err(error)),
                    Poll::Pending => {}
                }
            }
            if timer
                .as_mut()
                .is_some_and(|wait| wait.as_mut().poll(context).is_ready())
            {
                self.cancelled.store(true, Ordering::SeqCst);
                return Poll::Ready(Err(failure(
                    "swallowtail.deepseek_harness.web.websocket_timeout",
                    "DeepSeek Harness WebSocket did not open before its deadline",
                )));
            }
            Poll::Pending
        })
        .await
    }
}

type Socket = WebSocket<MaybeTlsStream<TcpStream>>;

fn run_websocket(
    endpoint: String,
    path: String,
    mut sender: mpsc::Sender<Result<Vec<u8>, RuntimeFailure>>,
    cancelled: Arc<AtomicBool>,
) -> Result<(), RuntimeFailure> {
    let mut url = require_loopback_endpoint(&endpoint)?;
    let origin = format!(
        "http://{}:{}",
        url.host_str().unwrap_or("127.0.0.1"),
        url.port().ok_or_else(endpoint_failure)?,
    );
    url.set_scheme("ws").map_err(|_| endpoint_failure())?;
    url.set_path(&path);
    let mut request = url
        .as_str()
        .into_client_request()
        .map_err(|_| endpoint_failure())?;
    request.headers_mut().insert(
        ORIGIN,
        HeaderValue::from_str(&origin).map_err(|_| endpoint_failure())?,
    );
    let config = WebSocketConfig::default()
        .read_buffer_size(MAX_WEBSOCKET_FRAME_BYTES)
        .max_message_size(Some(MAX_WEBSOCKET_FRAME_BYTES))
        .max_frame_size(Some(MAX_WEBSOCKET_FRAME_BYTES));
    let (mut socket, _) =
        connect_with_config(request, Some(config), 0).map_err(|_| disconnected())?;
    set_read_timeout(&mut socket)?;
    loop {
        if cancelled.load(Ordering::SeqCst) {
            let _ = socket.close(None);
            return Ok(());
        }
        match socket.read() {
            Ok(Message::Text(frame)) => {
                if sender.try_send(Ok(frame.as_bytes().to_vec())).is_err() {
                    return Err(failure(
                        "swallowtail.deepseek_harness.web.websocket_backpressure",
                        "DeepSeek Harness WebSocket event buffer is full",
                    ));
                }
            }
            Ok(Message::Ping(bytes)) => socket
                .send(Message::Pong(bytes))
                .map_err(|_| disconnected())?,
            Ok(Message::Pong(_)) => {}
            Ok(Message::Close(_)) => return Ok(()),
            Ok(Message::Binary(_) | Message::Frame(_)) => return Err(protocol_failure()),
            Err(WebSocketError::Io(error))
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) => {}
            Err(WebSocketError::ConnectionClosed | WebSocketError::AlreadyClosed) => return Ok(()),
            Err(_) => return Err(disconnected()),
        }
    }
}

fn perform_request(
    endpoint: &str,
    path: &str,
    body: &[u8],
    cancelled: &Arc<AtomicBool>,
) -> Result<Response, RuntimeFailure> {
    let mut url = require_loopback_endpoint(endpoint)?;
    url.set_path(path);
    if body.is_empty() || body.len() > MAX_HTTP_BODY_BYTES {
        return Err(failure(
            "swallowtail.deepseek_harness.web.request_limit",
            "DeepSeek Harness Web request body exceeds its bound",
        ));
    }
    let mut easy = Easy::new();
    easy.url(url.as_str()).map_err(|_| transport_failure())?;
    easy.proxy("").map_err(|_| transport_failure())?;
    easy.follow_location(false)
        .map_err(|_| transport_failure())?;
    easy.post(true).map_err(|_| transport_failure())?;
    easy.post_fields_copy(body)
        .map_err(|_| transport_failure())?;
    let mut headers = List::new();
    headers
        .append("Content-Type: application/json")
        .map_err(|_| transport_failure())?;
    headers
        .append("Sec-Fetch-Site: same-origin")
        .map_err(|_| transport_failure())?;
    headers
        .append(&format!(
            "Origin: http://{}:{}",
            url.host_str().unwrap_or("127.0.0.1"),
            url.port().ok_or_else(transport_failure)?,
        ))
        .map_err(|_| transport_failure())?;
    easy.http_headers(headers)
        .map_err(|_| transport_failure())?;
    easy.progress(true).map_err(|_| transport_failure())?;
    let mut response_body = Vec::new();
    let overflow = Arc::new(AtomicBool::new(false));
    {
        let overflow = Arc::clone(&overflow);
        let mut transfer = easy.transfer();
        let progress_cancelled = Arc::clone(cancelled);
        transfer
            .progress_function(move |_, _, _, _| !progress_cancelled.load(Ordering::SeqCst))
            .map_err(|_| transport_failure())?;
        transfer
            .write_function(|chunk| {
                if response_body.len().saturating_add(chunk.len()) > MAX_HTTP_BODY_BYTES {
                    overflow.store(true, Ordering::SeqCst);
                    return Err(WriteError::Pause);
                }
                response_body.extend_from_slice(chunk);
                Ok(chunk.len())
            })
            .map_err(|_| transport_failure())?;
        let perform_result = transfer.perform();
        if overflow.load(Ordering::SeqCst) {
            return Err(failure(
                "swallowtail.deepseek_harness.web.response_limit",
                "DeepSeek Harness Web response exceeds its bound",
            ));
        }
        perform_result.map_err(|_| transport_failure())?;
    }
    if cancelled.load(Ordering::SeqCst) {
        return Err(failure(
            "swallowtail.deepseek_harness.web.request_cancelled",
            "DeepSeek Harness Web request was cancelled",
        ));
    }
    if overflow.load(Ordering::SeqCst) {
        return Err(failure(
            "swallowtail.deepseek_harness.web.response_limit",
            "DeepSeek Harness Web response exceeds its bound",
        ));
    }
    let status = easy.response_code().map_err(|_| transport_failure())?;
    let status = u16::try_from(status).map_err(|_| transport_failure())?;
    Ok(Response {
        status,
        body: response_body,
    })
}

async fn wait_blocking(
    work: BoxFuture<'static, Result<(), RuntimeFailure>>,
    deadline: Option<Deadline>,
    services: &HostServices,
    cancelled: Arc<AtomicBool>,
) -> Result<(), RuntimeFailure> {
    let mut work = work;
    let mut timer = deadline
        .map(|deadline| {
            services
                .time()
                .ok_or_else(|| {
                    failure(
                        "swallowtail.deepseek_harness.web.time_service_missing",
                        "Deadline-bound DeepSeek Harness Web request requires a time service",
                    )
                })
                .map(|time| time.wait_until(deadline))
        })
        .transpose()?;
    let mut control_error = None;
    let result = poll_fn(|context| {
        if let Poll::Ready(result) = work.as_mut().poll(context) {
            return Poll::Ready(result);
        }
        if control_error.is_none() && cancelled.load(Ordering::SeqCst) {
            control_error = Some(failure(
                "swallowtail.deepseek_harness.web.request_cancelled",
                "DeepSeek Harness Web request was cancelled",
            ));
        }
        if control_error.is_none()
            && timer
                .as_mut()
                .is_some_and(|wait| wait.as_mut().poll(context).is_ready())
        {
            cancelled.store(true, Ordering::SeqCst);
            control_error = Some(failure(
                "swallowtail.deepseek_harness.web.request_timeout",
                "DeepSeek Harness Web request exceeded its deadline",
            ));
        }
        if control_error.is_some() {
            context.waker().wake_by_ref();
        }
        Poll::Pending
    })
    .await;
    match control_error {
        Some(error) => Err(error),
        None => result,
    }
}

fn set_read_timeout(socket: &mut Socket) -> Result<(), RuntimeFailure> {
    match socket.get_ref() {
        MaybeTlsStream::Plain(stream) => stream
            .set_read_timeout(Some(Duration::from_millis(100)))
            .map_err(|_| disconnected()),
        MaybeTlsStream::Rustls(stream) => stream
            .sock
            .set_read_timeout(Some(Duration::from_millis(100)))
            .map_err(|_| disconnected()),
        _ => Err(endpoint_failure()),
    }
}

fn validate_path(path: &str) -> Result<(), RuntimeFailure> {
    if !path.starts_with(HTTP_PATH_PREFIX)
        || path.len() <= HTTP_PATH_PREFIX.len()
        || path.contains(['?', '#'])
    {
        Err(failure(
            "swallowtail.deepseek_harness.web.route_invalid",
            "DeepSeek Harness Web route is invalid",
        ))
    } else {
        Ok(())
    }
}

fn validate_websocket_path(path: &str) -> Result<(), RuntimeFailure> {
    if matches!(path, "/api/events.mux" | "/api/events.host") {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.deepseek_harness.web.websocket_route_invalid",
            "DeepSeek Harness WebSocket route is invalid",
        ))
    }
}

pub(crate) fn require_loopback_endpoint(endpoint: &str) -> Result<Url, RuntimeFailure> {
    let url = Url::parse(endpoint).map_err(|_| endpoint_failure())?;
    if url.scheme() != "http"
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || !matches!(url.host(), Some(Host::Ipv4(address)) if address.is_loopback() && address == std::net::Ipv4Addr::LOCALHOST)
        || url.port().is_none()
        || !matches!(url.path(), "" | "/")
    {
        return Err(endpoint_failure());
    }
    Ok(url)
}

fn endpoint_failure() -> RuntimeFailure {
    failure(
        "swallowtail.deepseek_harness.web.endpoint_invalid",
        "DeepSeek Harness Web requires one explicit loopback HTTP endpoint",
    )
}

fn disconnected() -> RuntimeFailure {
    failure(
        "swallowtail.deepseek_harness.web.websocket_disconnected",
        "DeepSeek Harness WebSocket disconnected before completion",
    )
}

fn cancelled_failure() -> RuntimeFailure {
    failure(
        "swallowtail.deepseek_harness.web.websocket_cancelled",
        "DeepSeek Harness WebSocket operation was cancelled",
    )
}

fn protocol_failure() -> RuntimeFailure {
    failure(
        "swallowtail.deepseek_harness.web.websocket_protocol_invalid",
        "DeepSeek Harness WebSocket returned an unsupported frame",
    )
}

fn transport_failure() -> RuntimeFailure {
    failure(
        "swallowtail.deepseek_harness.web.transport_failed",
        "DeepSeek Harness Web transport failed",
    )
}
