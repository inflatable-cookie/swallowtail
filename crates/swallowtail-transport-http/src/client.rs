use crate::TransportError;
use crate::request::{HttpRequest, Method, SecretHeader};
use crate::response::HttpResponse;
use crate::stream::{BodyDecoder, ChannelSender, StreamError, StreamItem, Subscription, channel};
use curl::easy::{Easy, List};
use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use swallowtail_runtime::{HostServices, ScopeId};
use url::Url;
use zeroize::Zeroizing;

/// Bounded streaming capacity in buffered items.
pub const STREAM_CAPACITY: usize = 64;

/// Provider-neutral bounded HTTP and SSE transport.
///
/// Each exchange moves its blocking socket work onto the host's
/// blocking-work service, so adapters stay executor-neutral.
#[derive(Clone, Copy, Debug, Default)]
pub struct HttpTransport;

impl HttpTransport {
    /// Creates the stateless transport.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Performs one bounded request and returns its completed response.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError`] when the endpoint, credential, host service,
    /// transport, cancellation or response bound rejects the exchange.
    pub async fn execute(
        &self,
        scope: ScopeId,
        endpoint: &str,
        request: HttpRequest,
        services: &HostServices,
        cancelled: Arc<AtomicBool>,
    ) -> Result<HttpResponse, TransportError> {
        let blocking = services
            .blocking_work()
            .cloned()
            .ok_or(TransportError::BlockingServiceMissing)?;
        let url = request.resolve(endpoint)?;
        let (sender, receiver) = crate::stream::result_channel();
        blocking
            .run(
                scope,
                Box::new(move || {
                    let result = perform_request(url, &request, &cancelled);
                    let _ = sender.send(result);
                    Ok(())
                }),
            )
            .await
            .map_err(|_| TransportError::Transport)?;
        receiver
            .recv()
            .await
            .ok_or(TransportError::BlockingResultMissing)?
    }

    /// Performs one bounded request on the calling blocking worker.
    ///
    /// This is the same exchange as [`Self::execute`] for adapters that already
    /// own a blocking-work slot and must not dispatch a second job.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError`] under the same conditions as [`Self::execute`],
    /// including [`TransportError::Cancelled`] when the signal is raised.
    pub fn execute_blocking(
        &self,
        endpoint: &str,
        request: HttpRequest,
        cancelled: &Arc<AtomicBool>,
    ) -> Result<HttpResponse, TransportError> {
        let url = request.resolve(endpoint)?;
        perform_request(url, &request, cancelled)
    }

    /// Starts one bounded streaming exchange decoded by `decoder`.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError`] when the endpoint, credential or host
    /// service rejects the exchange before it starts.
    pub fn subscribe<Decoder>(
        &self,
        scope: ScopeId,
        endpoint: &str,
        request: HttpRequest,
        decoder: Decoder,
        services: &HostServices,
        cancelled: Arc<AtomicBool>,
    ) -> Result<Subscription<Decoder::Item, Decoder::Error>, TransportError>
    where
        Decoder: BodyDecoder,
    {
        let blocking = services
            .blocking_work()
            .cloned()
            .ok_or(TransportError::BlockingServiceMissing)?;
        let url = request.resolve(endpoint)?;
        let (sender, receiver) = channel(STREAM_CAPACITY);
        let (terminal_sender, terminal_receiver) = channel(1);
        let worker_cancelled = Arc::clone(&cancelled);
        let work = blocking.run(
            scope,
            Box::new(move || {
                let result = perform_stream(url, request, decoder, sender, worker_cancelled);
                let _ = terminal_sender.send(result);
                Ok(())
            }),
        );
        let work: Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>> =
            Box::pin(async move {
                let _ = work.await;
            });
        Ok(Subscription::new(
            receiver,
            terminal_receiver,
            cancelled,
            work,
        ))
    }
}

fn configure(
    easy: &mut Easy,
    url: &Url,
    request: &HttpRequest,
    cancelled: &Arc<AtomicBool>,
    streaming: bool,
) -> Result<(), TransportError> {
    easy.url(url.as_str())
        .map_err(|_| TransportError::Transport)?;
    easy.follow_location(false)
        .map_err(|_| TransportError::Transport)?;
    easy.proxy("").map_err(|_| TransportError::Transport)?;
    if let Some(timeout) = request.effective_connect_timeout() {
        easy.connect_timeout(timeout)
            .map_err(|_| TransportError::Transport)?;
    }
    if let Some(timeout) = request.effective_transfer_timeout(streaming) {
        easy.timeout(timeout)
            .map_err(|_| TransportError::Transport)?;
    }
    easy.progress(true).map_err(|_| TransportError::Transport)?;
    let progress_cancelled = Arc::clone(cancelled);
    easy.progress_function(move |_, _, _, _| !progress_cancelled.load(Ordering::SeqCst))
        .map_err(|_| TransportError::Transport)?;
    let mut headers = List::new();
    for (name, value) in request.headers() {
        append_header(&mut headers, &format!("{name}: {value}"))?;
    }
    for secret in request.secret_headers() {
        append_secret_header(&mut headers, secret)?;
    }
    easy.http_headers(headers)
        .map_err(|_| TransportError::Transport)?;
    match request.method() {
        Method::Get => {}
        Method::Post => {
            easy.post(true).map_err(|_| TransportError::Transport)?;
            easy.post_fields_copy(request.body().unwrap_or_default())
                .map_err(|_| TransportError::Transport)?;
        }
        method => {
            if let Some(verb) = method.curl_verb() {
                easy.custom_request(verb)
                    .map_err(|_| TransportError::Transport)?;
            }
        }
    }
    Ok(())
}

fn append_header(headers: &mut List, line: &str) -> Result<(), TransportError> {
    headers.append(line).map_err(|_| TransportError::Transport)
}

fn append_secret_header(headers: &mut List, secret: &SecretHeader) -> Result<(), TransportError> {
    let mut line = Zeroizing::new(format!("{}: {}", secret.name(), secret.value()));
    append_header(headers, line.as_str())?;
    line.clear();
    Ok(())
}

fn perform_request(
    url: Url,
    request: &HttpRequest,
    cancelled: &Arc<AtomicBool>,
) -> Result<HttpResponse, TransportError> {
    let mut easy = Easy::new();
    configure(&mut easy, &url, request, cancelled, false)?;
    let mut body = Vec::new();
    let mut headers = BTreeMap::new();
    let overflow = Cell::new(false);
    {
        let mut transfer = easy.transfer();
        transfer
            .header_function(|line| {
                capture_header(line, &mut headers, request.header_filter());
                true
            })
            .map_err(|_| TransportError::Transport)?;
        transfer
            .write_function(|chunk| {
                if body.len().saturating_add(chunk.len()) > request.response_limit() {
                    overflow.set(true);
                    return Ok(0);
                }
                body.extend_from_slice(chunk);
                Ok(chunk.len())
            })
            .map_err(|_| TransportError::Transport)?;
        let result = transfer.perform();
        if overflow.get() {
            return Err(TransportError::ResponseLimit);
        }
        if cancelled.load(Ordering::SeqCst) {
            return Err(TransportError::Cancelled);
        }
        result.map_err(|_| TransportError::Transport)?;
    }
    Ok(HttpResponse {
        status: easy
            .response_code()
            .map_err(|_| TransportError::Transport)?,
        headers,
        body,
    })
}

#[allow(clippy::too_many_lines)]
fn perform_stream<Decoder>(
    url: Url,
    request: HttpRequest,
    mut decoder: Decoder,
    sender: ChannelSender<StreamItem<Decoder::Item>>,
    cancelled: Arc<AtomicBool>,
) -> Result<(), StreamError<Decoder::Error>>
where
    Decoder: BodyDecoder,
{
    let mut easy = Easy::new();
    configure(&mut easy, &url, &request, &cancelled, true).map_err(StreamError::Transport)?;
    let headers = RefCell::new(BTreeMap::new());
    let status = Cell::new(0_u32);
    let sent_headers = Cell::new(false);
    let failure: RefCell<Option<StreamError<Decoder::Error>>> = RefCell::new(None);
    let error_body = RefCell::new(Vec::new());
    let response_limit = request.response_limit();
    let emit_headers = request.emits_headers();
    let header_filter = request.header_filter();
    {
        let mut transfer = easy.transfer();
        transfer
            .header_function(|line| {
                if line.starts_with(b"HTTP/") {
                    status.set(parse_status(line).unwrap_or(0));
                }
                capture_header(line, &mut headers.borrow_mut(), header_filter);
                true
            })
            .map_err(|_| StreamError::Transport(TransportError::Transport))?;
        transfer
            .write_function(|chunk| {
                if !(200..300).contains(&status.get()) {
                    let mut body = error_body.borrow_mut();
                    if body.len().saturating_add(chunk.len()) > response_limit {
                        *failure.borrow_mut() =
                            Some(StreamError::Transport(TransportError::ResponseLimit));
                        return Ok(0);
                    }
                    body.extend_from_slice(chunk);
                    return Ok(chunk.len());
                }
                if emit_headers && !sent_headers.replace(true) {
                    let observed = headers.borrow().clone();
                    if !sender.send(StreamItem::Headers(observed)) {
                        *failure.borrow_mut() =
                            Some(StreamError::Transport(TransportError::Cancelled));
                        return Ok(0);
                    }
                }
                match decoder.push(chunk) {
                    Ok(items) => {
                        for item in items {
                            if !sender.send(StreamItem::Body(item)) {
                                *failure.borrow_mut() =
                                    Some(StreamError::Transport(TransportError::Cancelled));
                                return Ok(0);
                            }
                        }
                        Ok(chunk.len())
                    }
                    Err(error) => {
                        *failure.borrow_mut() = Some(StreamError::Decode(error));
                        Ok(0)
                    }
                }
            })
            .map_err(|_| StreamError::Transport(TransportError::Transport))?;
        let result = transfer.perform();
        if let Some(error) = failure.borrow_mut().take() {
            return Err(error);
        }
        if cancelled.load(Ordering::SeqCst) {
            return Ok(());
        }
        result.map_err(|_| StreamError::Transport(TransportError::Transport))?;
    }
    let code = easy
        .response_code()
        .map_err(|_| StreamError::Transport(TransportError::Transport))?;
    if !(200..300).contains(&code) {
        return Err(StreamError::Status {
            status: code,
            headers: headers.into_inner(),
            body: error_body.into_inner(),
        });
    }
    decoder.finish().map_err(StreamError::Decode)
}

fn capture_header(
    line: &[u8],
    headers: &mut BTreeMap<String, String>,
    filter: Option<fn(&str) -> bool>,
) {
    let Some(filter) = filter else {
        return;
    };
    let Ok(line) = std::str::from_utf8(line) else {
        return;
    };
    let Some((name, value)) = line.split_once(':') else {
        return;
    };
    let name = name.trim().to_ascii_lowercase();
    if filter(&name) {
        headers.insert(name, value.trim().to_owned());
    }
}

fn parse_status(line: &[u8]) -> Option<u32> {
    std::str::from_utf8(line)
        .ok()?
        .split_whitespace()
        .nth(1)?
        .parse()
        .ok()
}
