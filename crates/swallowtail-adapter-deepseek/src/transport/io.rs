use crate::failure::provider;
use crate::protocol::{Method, ProviderFailureKind};
use curl::easy::{Easy, List, WriteError};
use futures_channel::mpsc::TrySendError;
use std::cell::{Cell, RefCell};
use url::Url;

const RESPONSE_LIMIT: usize = 1024 * 1024;

fn request_url(endpoint: &str, request: &HttpRequest) -> Result<Url, RuntimeFailure> {
    let mut url = Url::parse(endpoint).map_err(|_| endpoint_failure())?;
    if !matches!(url.scheme(), "http" | "https")
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err(endpoint_failure());
    }
    let base = url.path().trim_end_matches('/');
    url.set_path(&format!("{base}/{}", request.path.trim_start_matches('/')));
    Ok(url)
}

fn configure(
    easy: &mut Easy,
    url: &Url,
    credential: &[u8],
    request: &HttpRequest,
    cancelled: &Arc<AtomicBool>,
) -> Result<(), RuntimeFailure> {
    let credential = std::str::from_utf8(credential).map_err(|_| {
        failure(
            "swallowtail.deepseek.credential_invalid",
            "DeepSeek API-key credential was invalid",
        )
    })?;
    easy.url(url.as_str()).map_err(curl_failure)?;
    easy.follow_location(false).map_err(curl_failure)?;
    easy.proxy("").map_err(curl_failure)?;
    easy.progress(true).map_err(curl_failure)?;
    let progress_cancelled = Arc::clone(cancelled);
    easy.progress_function(move |_, _, _, _| !progress_cancelled.load(Ordering::SeqCst))
        .map_err(curl_failure)?;
    let mut headers = List::new();
    headers
        .append(&format!("authorization: Bearer {credential}"))
        .map_err(curl_failure)?;
    if request.method == Method::Post {
        headers
            .append("content-type: application/json")
            .map_err(curl_failure)?;
        headers
            .append(if request.stream {
                "accept: text/event-stream"
            } else {
                "accept: application/json"
            })
            .map_err(curl_failure)?;
        easy.post(true).map_err(curl_failure)?;
        easy.post_fields_copy(request.body.as_deref().unwrap_or_default())
            .map_err(curl_failure)?;
    }
    easy.http_headers(headers).map_err(curl_failure)?;
    Ok(())
}

fn perform_request(
    url: Url,
    credential: Vec<u8>,
    request: HttpRequest,
    cancelled: Arc<AtomicBool>,
) -> Result<HttpResponse, RuntimeFailure> {
    let mut easy = Easy::new();
    let credential = SecretCopy(credential);
    configure(&mut easy, &url, &credential.0, &request, &cancelled)?;
    let mut body = Vec::new();
    let mut headers = BTreeMap::new();
    let overflow = Cell::new(false);
    {
        let mut transfer = easy.transfer();
        transfer
            .header_function(|line| {
                capture_header(line, &mut headers);
                true
            })
            .map_err(curl_failure)?;
        transfer
            .write_function(|chunk| {
                if body.len().saturating_add(chunk.len()) > RESPONSE_LIMIT {
                    overflow.set(true);
                    return Err(WriteError::Pause);
                }
                body.extend_from_slice(chunk);
                Ok(chunk.len())
            })
            .map_err(curl_failure)?;
        let result = transfer.perform();
        if overflow.get() {
            return Err(response_limit());
        }
        if cancelled.load(Ordering::SeqCst) {
            return Err(cancelled_failure());
        }
        result.map_err(curl_failure)?;
    }
    Ok(HttpResponse {
        status: easy.response_code().map_err(curl_failure)?,
        headers,
        body,
    })
}

fn perform_sse(
    url: Url,
    credential: Vec<u8>,
    request: HttpRequest,
    mut sender: mpsc::Sender<StreamItem>,
    cancelled: Arc<AtomicBool>,
) -> Result<(), RuntimeFailure> {
    let mut easy = Easy::new();
    let credential = SecretCopy(credential);
    configure(&mut easy, &url, &credential.0, &request, &cancelled)?;
    let headers = RefCell::new(BTreeMap::new());
    let status = Cell::new(0_u32);
    let sent_metadata = Cell::new(false);
    let failure_slot = RefCell::new(None);
    let error_body = RefCell::new(Vec::new());
    {
        let mut transfer = easy.transfer();
        transfer
            .header_function(|line| {
                if line.starts_with(b"HTTP/") {
                    status.set(parse_status(line).unwrap_or(0));
                }
                capture_header(line, &mut headers.borrow_mut());
                true
            })
            .map_err(curl_failure)?;
        transfer
            .write_function(|chunk| {
                if !(200..300).contains(&status.get()) {
                    let mut body = error_body.borrow_mut();
                    if body.len().saturating_add(chunk.len()) > RESPONSE_LIMIT {
                        *failure_slot.borrow_mut() = Some(response_limit());
                        return Ok(0);
                    }
                    body.extend_from_slice(chunk);
                    return Ok(chunk.len());
                }
                if !sent_metadata.replace(true)
                    && send(
                        &mut sender,
                        StreamItem::Metadata(headers.borrow().clone()),
                    )
                    .is_err()
                {
                    *failure_slot.borrow_mut() = Some(backpressure());
                    return Ok(0);
                }
                if send(&mut sender, StreamItem::Data(chunk.to_vec())).is_err() {
                    *failure_slot.borrow_mut() = Some(backpressure());
                    return Ok(0);
                }
                Ok(chunk.len())
            })
            .map_err(curl_failure)?;
        let result = transfer.perform();
        if let Some(error) = failure_slot.borrow_mut().take() {
            return Err(error);
        }
        if cancelled.load(Ordering::SeqCst) {
            return Ok(());
        }
        result.map_err(curl_failure)?;
    }
    let status = easy.response_code().map_err(curl_failure)?;
    if !(200..300).contains(&status) {
        let kind = crate::protocol::classify_failure(status as u16)
            .unwrap_or(ProviderFailureKind::Provider);
        return Err(provider(kind));
    }
    if !sent_metadata.get() {
        send(&mut sender, StreamItem::Metadata(headers.into_inner()))
            .map_err(|_| backpressure())?;
    }
    Ok(())
}

fn send(
    sender: &mut mpsc::Sender<StreamItem>,
    item: StreamItem,
) -> Result<(), TrySendError<StreamItem>> {
    sender.try_send(item)
}

fn capture_header(line: &[u8], headers: &mut BTreeMap<String, String>) {
    let Ok(text) = std::str::from_utf8(line) else {
        return;
    };
    let Some((name, value)) = text.split_once(':') else {
        return;
    };
    headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_owned());
}

fn parse_status(line: &[u8]) -> Option<u32> {
    std::str::from_utf8(line)
        .ok()?
        .split_whitespace()
        .nth(1)?
        .parse()
        .ok()
}

fn endpoint_failure() -> RuntimeFailure {
    failure(
        "swallowtail.deepseek.endpoint_invalid",
        "DeepSeek endpoint was invalid",
    )
}

fn curl_failure<T>(_error: T) -> RuntimeFailure {
    failure(
        "swallowtail.deepseek.transport_failed",
        "DeepSeek HTTP transport failed",
    )
}

fn response_limit() -> RuntimeFailure {
    failure(
        "swallowtail.deepseek.response_limit",
        "DeepSeek response exceeded its byte bound",
    )
}

fn backpressure() -> RuntimeFailure {
    failure(
        "swallowtail.deepseek.stream_backpressure",
        "DeepSeek stream exceeded its bounded transport channel",
    )
}

fn cancelled_failure() -> RuntimeFailure {
    failure(
        "swallowtail.deepseek.cancelled",
        "DeepSeek request was cancelled locally",
    )
}

struct SecretCopy(Vec<u8>);

impl Drop for SecretCopy {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}
