use crate::protocol::{Method, SseDecoder, require_success};
use curl::easy::{Easy, List, WriteError};
use futures_channel::mpsc::TrySendError;
use std::cell::{Cell, RefCell};
use url::Url;

const RESPONSE_LIMIT: usize = 4 * 1024 * 1024;

fn request_url(endpoint: &str, request: &Request) -> Result<Url, RuntimeFailure> {
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
    request: &Request,
    cancelled: &Arc<AtomicBool>,
) -> Result<(), RuntimeFailure> {
    let credential = std::str::from_utf8(credential).map_err(|_| {
        failure(
            "swallowtail.kimi_platform.credential_invalid",
            "Kimi Platform API-key credential was invalid",
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
            .append("accept: text/event-stream")
            .map_err(curl_failure)?;
    }
    easy.http_headers(headers).map_err(curl_failure)?;
    if request.method == Method::Post {
        easy.post(true).map_err(curl_failure)?;
        easy.post_fields_copy(request.body.as_deref().unwrap_or_default())
            .map_err(curl_failure)?;
    }
    Ok(())
}

fn perform_request(
    url: Url,
    credential: Vec<u8>,
    request: Request,
    cancelled: Arc<AtomicBool>,
) -> Result<Response, RuntimeFailure> {
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
    Ok(Response {
        status: easy.response_code().map_err(curl_failure)?,
        _headers: headers,
        body,
    })
}

fn perform_sse(
    url: Url,
    credential: Vec<u8>,
    request: Request,
    mut sender: mpsc::Sender<StreamItem>,
    cancelled: Arc<AtomicBool>,
) -> Result<(), RuntimeFailure> {
    let mut easy = Easy::new();
    let credential = SecretCopy(credential);
    configure(&mut easy, &url, &credential.0, &request, &cancelled)?;
    let headers = RefCell::new(BTreeMap::new());
    let status = Cell::new(0_u32);
    let failure_slot = RefCell::new(None);
    let error_body = RefCell::new(Vec::new());
    let mut decoder = SseDecoder::default();
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
                match decoder.push(chunk) {
                    Ok(frames) => {
                        for frame in frames {
                            if send(&mut sender, StreamItem::Frame(frame)).is_err() {
                                *failure_slot.borrow_mut() = Some(backpressure());
                                return Ok(0);
                            }
                        }
                        Ok(chunk.len())
                    }
                    Err(error) => {
                        *failure_slot.borrow_mut() = Some(error);
                        Ok(0)
                    }
                }
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
    let response = Response {
        status: easy.response_code().map_err(curl_failure)?,
        _headers: headers.into_inner(),
        body: error_body.into_inner(),
    };
    require_success(&response, "message request")?;
    decoder.finish()
}

fn send(
    sender: &mut mpsc::Sender<StreamItem>,
    item: StreamItem,
) -> Result<(), TrySendError<StreamItem>> {
    sender.try_send(item)
}

include!("io/support.rs");
