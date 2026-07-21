use crate::protocol::{Method, SseDecoder, require_success};
use curl::easy::{Easy, List, WriteError};
use futures_channel::mpsc::TrySendError;
use std::cell::{Cell, RefCell};
use std::time::Duration;
use url::Url;

const RESPONSE_LIMIT: usize = 4 * 1024 * 1024;
const MANAGEMENT_TIMEOUT: Duration = Duration::from_secs(5);

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
    if !request.query.is_empty() {
        url.query_pairs_mut().extend_pairs(
            request
                .query
                .iter()
                .map(|(key, value)| (key.as_str(), value.as_str())),
        );
    }
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
            "swallowtail.openai.credential_invalid",
            "OpenAI API-key credential was invalid",
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
    headers
        .append("content-type: application/json")
        .map_err(curl_failure)?;
    headers
        .append(if request.expects_stream() {
            "accept: text/event-stream"
        } else {
            "accept: application/json"
        })
        .map_err(curl_failure)?;
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
    easy.timeout(MANAGEMENT_TIMEOUT).map_err(curl_failure)?;
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
        headers,
        body,
    })
}

include!("io/sse.rs");
include!("io/support.rs");
