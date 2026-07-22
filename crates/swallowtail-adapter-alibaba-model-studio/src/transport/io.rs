use crate::failure::protocol;
use crate::protocol::{Method, SseDecoder, parse_provider_failure};
use curl::easy::{Easy, List, WriteError};
use futures_channel::mpsc::TrySendError;
use std::cell::{Cell, RefCell};
use url::Url;

const RESPONSE_LIMIT: usize = 4 * 1024 * 1024;

fn request_url(endpoint: &str, request: &WireRequest) -> Result<Url, RuntimeFailure> {
    let mut url = Url::parse(endpoint).map_err(|_| endpoint_failure())?;
    if !matches!(url.scheme(), "http" | "https")
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err(endpoint_failure());
    }
    let base = url.path().trim_end_matches('/').to_owned();
    url.set_path(&format!("{base}/{}", request.path().trim_start_matches('/')));
    if let Some((path, query)) = request.path().split_once('?') {
        url.set_path(&format!("{base}/{}", path.trim_start_matches('/')));
        url.set_query(Some(query));
    }
    Ok(url)
}

fn configure(
    easy: &mut Easy,
    url: &Url,
    credential: &[u8],
    request: &WireRequest,
    sse: bool,
    cancelled: &Arc<AtomicBool>,
) -> Result<(), RuntimeFailure> {
    let credential = std::str::from_utf8(credential).map_err(|_| failure(
        "swallowtail.alibaba_model_studio.credential_invalid",
        "Alibaba Model Studio API-key credential was invalid",
    ))?;
    easy.url(url.as_str()).map_err(curl_failure)?;
    easy.follow_location(false).map_err(curl_failure)?;
    easy.proxy("").map_err(curl_failure)?;
    easy.progress(true).map_err(curl_failure)?;
    let progress_cancelled = Arc::clone(cancelled);
    easy.progress_function(move |_, _, _, _| !progress_cancelled.load(Ordering::SeqCst)).map_err(curl_failure)?;
    let mut headers = List::new();
    headers.append(&format!("authorization: Bearer {credential}")).map_err(curl_failure)?;
    headers.append(if sse { "accept: text/event-stream" } else { "accept: application/json" }).map_err(curl_failure)?;
    if request.body().is_some() { headers.append("content-type: application/json").map_err(curl_failure)?; }
    easy.http_headers(headers).map_err(curl_failure)?;
    match request.method() {
        Method::Get => {}
        Method::Post => {
            easy.post(true).map_err(curl_failure)?;
            let body = serde_json::to_vec(request.body().expect("POST body exists")).map_err(|_| protocol_failure())?;
            easy.post_fields_copy(&body).map_err(curl_failure)?;
        }
        Method::Delete => { easy.custom_request("DELETE").map_err(curl_failure)?; }
    }
    Ok(())
}

fn perform_request(
    url: Url,
    credential: Vec<u8>,
    request: WireRequest,
    cancelled: Arc<AtomicBool>,
) -> Result<Response, RuntimeFailure> {
    let mut easy = Easy::new();
    let credential = SecretCopy(credential);
    configure(&mut easy, &url, &credential.0, &request, false, &cancelled)?;
    let mut body = Vec::new();
    let overflow = Cell::new(false);
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|chunk| {
            if body.len().saturating_add(chunk.len()) > RESPONSE_LIMIT {
                overflow.set(true);
                return Err(WriteError::Pause);
            }
            body.extend_from_slice(chunk);
            Ok(chunk.len())
        }).map_err(curl_failure)?;
        let result = transfer.perform();
        if overflow.get() { return Err(response_limit()); }
        if cancelled.load(Ordering::SeqCst) { return Err(cancelled_failure()); }
        result.map_err(curl_failure)?;
    }
    let response = Response { status: easy.response_code().map_err(curl_failure)?, body };
    require_success(response)
}

fn perform_sse(
    url: Url,
    credential: Vec<u8>,
    request: WireRequest,
    mut sender: mpsc::Sender<StreamItem>,
    cancelled: Arc<AtomicBool>,
) -> Result<(), RuntimeFailure> {
    let mut easy = Easy::new();
    let credential = SecretCopy(credential);
    configure(&mut easy, &url, &credential.0, &request, true, &cancelled)?;
    let headers = RefCell::new(BTreeMap::new());
    let status = Cell::new(0_u32);
    let correlation_sent = Cell::new(false);
    let failure_slot = RefCell::new(None);
    let error_body = RefCell::new(Vec::new());
    let mut decoder = SseDecoder::default();
    {
        let mut transfer = easy.transfer();
        transfer.header_function(|line| {
            if line.starts_with(b"HTTP/") { status.set(parse_status(line).unwrap_or(0)); }
            capture_header(line, &mut headers.borrow_mut());
            true
        }).map_err(curl_failure)?;
        transfer.write_function(|chunk| {
            if !(200..300).contains(&status.get()) {
                let mut body = error_body.borrow_mut();
                if body.len().saturating_add(chunk.len()) > RESPONSE_LIMIT {
                    *failure_slot.borrow_mut() = Some(response_limit());
                    return Ok(0);
                }
                body.extend_from_slice(chunk);
                return Ok(chunk.len());
            }
            if !correlation_sent.get() {
                match request_correlation(&headers.borrow()) {
                    Ok(reference) => {
                        if send(&mut sender, StreamItem::Correlation(reference)).is_err() {
                            *failure_slot.borrow_mut() = Some(backpressure());
                            return Ok(0);
                        }
                        correlation_sent.set(true);
                    }
                    Err(error) => { *failure_slot.borrow_mut() = Some(error); return Ok(0); }
                }
            }
            match decoder.push(chunk) {
                Ok(frames) => for frame in frames {
                    if send(&mut sender, StreamItem::Frame(frame)).is_err() {
                        *failure_slot.borrow_mut() = Some(backpressure());
                        return Ok(0);
                    }
                },
                Err(error) => { *failure_slot.borrow_mut() = Some(protocol(error)); return Ok(0); }
            }
            Ok(chunk.len())
        }).map_err(curl_failure)?;
        let result = transfer.perform();
        if let Some(error) = failure_slot.borrow_mut().take() { return Err(error); }
        if cancelled.load(Ordering::SeqCst) { return Ok(()); }
        result.map_err(curl_failure)?;
    }
    let status = easy.response_code().map_err(curl_failure)?;
    if !(200..300).contains(&status) {
        return Err(provider_error(&error_body.into_inner()));
    }
    decoder.finish().map_err(protocol)
}

fn require_success(response: Response) -> Result<Response, RuntimeFailure> {
    if (200..300).contains(&response.status) { Ok(response) } else { Err(provider_error(&response.body)) }
}

fn provider_error(body: &[u8]) -> RuntimeFailure {
    parse_provider_failure(body).map_or_else(protocol, protocol)
}

fn request_correlation(headers: &BTreeMap<String, String>) -> Result<ProviderRequestRef, RuntimeFailure> {
    headers.get("x-request-id").ok_or_else(protocol_failure).and_then(|value| {
        ProviderRequestRef::new(value.clone()).map_err(|_| protocol_failure())
    })
}

fn send(sender: &mut mpsc::Sender<StreamItem>, item: StreamItem) -> Result<(), TrySendError<StreamItem>> { sender.try_send(item) }

include!("io/support.rs");
