use crate::protocol::{ChatDecoder, Method, require_success};
use curl::easy::{Easy, List, WriteError};
use std::cell::{Cell, RefCell};
use url::Url;

const RESPONSE_LIMIT: usize = 4 * 1024 * 1024;

fn request_url(endpoint: &str, request: &Request) -> Result<Url, RuntimeFailure> {
    let mut url = Url::parse(endpoint).map_err(|_| endpoint_failure())?;
    if url.scheme() != "http"
        || !matches!(url.host_str(), Some("127.0.0.1" | "::1"))
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
    request: &Request,
    cancelled: &Arc<AtomicBool>,
) -> Result<(), RuntimeFailure> {
    easy.url(url.as_str()).map_err(curl_failure)?;
    easy.follow_location(false).map_err(curl_failure)?;
    easy.proxy("").map_err(curl_failure)?;
    easy.progress(true).map_err(curl_failure)?;
    let progress_cancelled = Arc::clone(cancelled);
    easy.progress_function(move |_, _, _, _| !progress_cancelled.load(Ordering::SeqCst))
        .map_err(curl_failure)?;
    if request.method == Method::Post {
        let mut headers = List::new();
        headers
            .append("content-type: application/json")
            .map_err(curl_failure)?;
        headers
            .append("accept: application/x-ndjson")
            .map_err(curl_failure)?;
        easy.http_headers(headers).map_err(curl_failure)?;
        easy.post(true).map_err(curl_failure)?;
        easy.post_fields_copy(request.body.as_deref().unwrap_or_default())
            .map_err(curl_failure)?;
    }
    Ok(())
}

fn perform_request(
    url: Url,
    request: Request,
    cancelled: Arc<AtomicBool>,
) -> Result<Response, RuntimeFailure> {
    let mut easy = Easy::new();
    configure(&mut easy, &url, &request, &cancelled)?;
    let mut body = Vec::new();
    let overflow = Cell::new(false);
    {
        let mut transfer = easy.transfer();
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
        body,
    })
}

fn perform_ndjson(
    url: Url,
    request: Request,
    model: String,
    mut sender: mpsc::Sender<NativeEvent>,
    cancelled: Arc<AtomicBool>,
) -> Result<(), RuntimeFailure> {
    let mut easy = Easy::new();
    configure(&mut easy, &url, &request, &cancelled)?;
    let status = Cell::new(0_u32);
    let failure_slot = RefCell::new(None);
    let error_body = RefCell::new(Vec::new());
    let mut decoder = Some(ChatDecoder::new(model));
    {
        let mut transfer = easy.transfer();
        transfer
            .header_function(|line| {
                if line.starts_with(b"HTTP/") {
                    status.set(parse_status(line).unwrap_or(0));
                }
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
                let result = decoder
                    .as_mut()
                    .expect("decoder remains present during transfer")
                    .push(chunk);
                match result {
                    Ok(events) => {
                        for event in events {
                            if sender.try_send(event).is_err() {
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
    require_success(
        &Response {
            status: easy.response_code().map_err(curl_failure)?,
            body: error_body.into_inner(),
        },
        "chat request",
    )?;
    decoder
        .take()
        .expect("decoder remains present after transfer")
        .finish()
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
        "swallowtail.ollama.endpoint_invalid",
        "Ollama endpoint grant was invalid",
    )
}

fn curl_failure(_: curl::Error) -> RuntimeFailure {
    failure(
        "swallowtail.ollama.transport_failed",
        "Ollama HTTP transport failed",
    )
}

fn response_limit() -> RuntimeFailure {
    failure(
        "swallowtail.ollama.response_limit",
        "Ollama HTTP response exceeded its limit",
    )
}

fn cancelled_failure() -> RuntimeFailure {
    failure(
        "swallowtail.ollama.cancelled",
        "Ollama HTTP work was cancelled",
    )
}

fn backpressure() -> RuntimeFailure {
    failure(
        "swallowtail.ollama.stream_backpressure",
        "Ollama stream exceeded its bounded consumer capacity",
    )
}
