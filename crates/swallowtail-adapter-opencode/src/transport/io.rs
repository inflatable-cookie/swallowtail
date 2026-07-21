fn request_url(endpoint: &str, request: &Request) -> Result<Url, RuntimeFailure> {
    let mut url = Url::parse(endpoint).map_err(|_| {
        failure(
            "swallowtail.opencode.endpoint_invalid",
            "Host-approved OpenCode endpoint is not a valid URL",
        )
    })?;
    if !matches!(url.scheme(), "http" | "https")
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err(failure(
            "swallowtail.opencode.endpoint_invalid",
            "Host-approved OpenCode endpoint is not an eligible HTTP endpoint",
        ));
    }
    let base = url.path().trim_end_matches('/');
    let route = request.path.trim_start_matches('/');
    url.set_path(&format!("{base}/{route}"));
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
    cancelled: &Arc<AtomicBool>,
) -> Result<(), RuntimeFailure> {
    easy.url(url.as_str()).map_err(curl_failure)?;
    easy.follow_location(false).map_err(curl_failure)?;
    easy.proxy("").map_err(curl_failure)?;
    easy.progress(true).map_err(curl_failure)?;
    let progress_cancelled = Arc::clone(cancelled);
    easy.progress_function(move |_, _, _, _| !progress_cancelled.load(Ordering::SeqCst))
        .map_err(curl_failure)?;
    Ok(())
}

fn perform_request(
    url: Url,
    request: Request,
    cancelled: Arc<AtomicBool>,
) -> Result<Response, RuntimeFailure> {
    let mut easy = Easy::new();
    configure(&mut easy, &url, &cancelled)?;
    match request.method {
        Method::Get => {}
        Method::Post => {
            easy.post(true).map_err(curl_failure)?;
            if let Some(body) = &request.body {
                let mut headers = List::new();
                headers
                    .append("Content-Type: application/json")
                    .map_err(curl_failure)?;
                easy.http_headers(headers).map_err(curl_failure)?;
                easy.post_fields_copy(body).map_err(curl_failure)?;
            } else {
                easy.post_fields_copy(&[]).map_err(curl_failure)?;
            }
        }
    }
    let mut body = Vec::new();
    let overflow = Arc::new(AtomicBool::new(false));
    {
        let callback_overflow = Arc::clone(&overflow);
        let mut transfer = easy.transfer();
        transfer
            .write_function(|chunk| {
                if body.len().saturating_add(chunk.len()) > RESPONSE_LIMIT {
                    callback_overflow.store(true, Ordering::SeqCst);
                    return Err(WriteError::Pause);
                }
                body.extend_from_slice(chunk);
                Ok(chunk.len())
            })
            .map_err(curl_failure)?;
        let result = transfer.perform();
        if overflow.load(Ordering::SeqCst) {
            return Err(failure(
                "swallowtail.opencode.response_limit",
                "OpenCode HTTP response exceeded the bounded input limit",
            ));
        }
        if cancelled.load(Ordering::SeqCst) {
            return Err(failure(
                "swallowtail.opencode.request_cancelled",
                "OpenCode HTTP request was cancelled",
            ));
        }
        result.map_err(curl_failure)?;
    }
    let status = easy.response_code().map_err(curl_failure)?;
    Ok(Response { status, body })
}

fn perform_sse(
    url: Url,
    mut sender: mpsc::Sender<Result<Vec<u8>, RuntimeFailure>>,
    cancelled: Arc<AtomicBool>,
) -> Result<(), RuntimeFailure> {
    let mut easy = Easy::new();
    configure(&mut easy, &url, &cancelled)?;
    let mut headers = List::new();
    headers
        .append("Accept: text/event-stream")
        .map_err(curl_failure)?;
    easy.http_headers(headers).map_err(curl_failure)?;
    let parser_failure = Arc::new(Mutex::new(None));
    let callback_failure = Arc::clone(&parser_failure);
    let mut decoder = SseDecoder::default();
    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|chunk| match decoder.push(chunk) {
                Ok(events) => {
                    for event in events {
                        if sender.try_send(Ok(event)).is_err() {
                            *callback_failure.lock().expect("SSE failure lock poisoned") =
                                Some(failure(
                                    "swallowtail.opencode.sse_backpressure",
                                    "OpenCode SSE delivery exceeded its bounded capacity",
                                ));
                            return Ok(0);
                        }
                    }
                    Ok(chunk.len())
                }
                Err(error) => {
                    *callback_failure.lock().expect("SSE failure lock poisoned") = Some(error);
                    Ok(0)
                }
            })
            .map_err(curl_failure)?;
        let result = transfer.perform();
        if let Some(error) = parser_failure
            .lock()
            .expect("SSE failure lock poisoned")
            .take()
        {
            return Err(error);
        }
        if cancelled.load(Ordering::SeqCst) {
            return Ok(());
        }
        result.map_err(curl_failure)?;
    }
    let status = easy.response_code().map_err(curl_failure)?;
    if !(200..300).contains(&status) {
        return Err(failure(
            "swallowtail.opencode.sse_http_failed",
            "OpenCode SSE subscription failed",
        ));
    }
    decoder.finish()
}

fn curl_failure(_: curl::Error) -> RuntimeFailure {
    failure(
        "swallowtail.opencode.transport_failed",
        "OpenCode HTTP transport failed",
    )
}


