fn perform_get(
    endpoint: &str,
    page_token: Option<&str>,
    mut secret: Vec<u8>,
    cancelled: Arc<AtomicBool>,
) -> Result<Vec<u8>, RuntimeFailure> {
    let mut url = Url::parse(endpoint).map_err(|_| protocol_failure())?;
    if url.scheme() != "https"
        || url.host_str() != Some("generativelanguage.googleapis.com")
        || !matches!(url.path(), "" | "/")
    {
        return Err(protocol_failure());
    }
    url.set_path("/v1beta/models");
    url.query_pairs_mut().append_pair("pageSize", "1000");
    if let Some(token) = page_token {
        url.query_pairs_mut().append_pair("pageToken", token);
    }
    let key = std::str::from_utf8(&secret).map_err(|_| protocol_failure())?;
    if key.is_empty() || key.bytes().any(|byte| matches!(byte, b'\r' | b'\n')) {
        secret.fill(0);
        return Err(protocol_failure());
    }
    let mut easy = Easy::new();
    easy.url(url.as_str()).map_err(|_| transport_failure())?;
    easy.follow_location(false)
        .map_err(|_| transport_failure())?;
    easy.proxy("").map_err(|_| transport_failure())?;
    easy.timeout(Duration::from_secs(10))
        .map_err(|_| transport_failure())?;
    easy.progress(true).map_err(|_| transport_failure())?;
    let progress = Arc::clone(&cancelled);
    easy.progress_function(move |_, _, _, _| !progress.load(Ordering::SeqCst))
        .map_err(|_| transport_failure())?;
    let mut headers = List::new();
    headers
        .append(&format!("x-goog-api-key: {key}"))
        .map_err(|_| transport_failure())?;
    headers
        .append("accept: application/json")
        .map_err(|_| transport_failure())?;
    secret.fill(0);
    easy.http_headers(headers)
        .map_err(|_| transport_failure())?;
    let mut body = Vec::new();
    let overflow = Arc::new(AtomicBool::new(false));
    {
        let callback_overflow = Arc::clone(&overflow);
        let mut transfer = easy.transfer();
        transfer
            .write_function(|chunk| {
                if body.len().saturating_add(chunk.len()) > MAXIMUM_BODY_BYTES {
                    callback_overflow.store(true, Ordering::SeqCst);
                    return Err(WriteError::Pause);
                }
                body.extend_from_slice(chunk);
                Ok(chunk.len())
            })
            .map_err(|_| transport_failure())?;
        transfer.perform().map_err(|_| transport_failure())?;
    }
    if overflow.load(Ordering::SeqCst) {
        return Err(protocol_failure());
    }
    let status = easy.response_code().map_err(|_| transport_failure())?;
    if !(200..300).contains(&status) {
        return Err(failure(
            "swallowtail.gemini.models.provider_rejected",
            "Gemini rejected model catalogue discovery",
        ));
    }
    Ok(body)
}

async fn complete_before_deadline<T, F>(
    work: F,
    deadline: Option<swallowtail_runtime::Deadline>,
    services: &HostServices,
    cancelled: Arc<AtomicBool>,
) -> Result<T, RuntimeFailure>
where
    F: Future<Output = Result<T, RuntimeFailure>>,
{
    let Some(deadline) = deadline else {
        return work.await;
    };
    let time = services.time().expect("validated time service");
    if time.now() >= deadline.instant() {
        return Err(failure(
            "swallowtail.gemini.models.deadline_elapsed",
            "Gemini Models deadline elapsed before dispatch",
        ));
    }
    let mut work = Box::pin(work);
    let mut wait = time.wait_until(deadline);
    let mut timed_out = false;
    let result = poll_fn(|context| {
        if let Poll::Ready(result) = work.as_mut().poll(context) {
            return Poll::Ready(result);
        }
        if !timed_out && wait.as_mut().poll(context).is_ready() {
            timed_out = true;
            cancelled.store(true, Ordering::SeqCst);
            context.waker().wake_by_ref();
        }
        Poll::Pending
    })
    .await;
    if timed_out {
        Err(failure(
            "swallowtail.gemini.models.timed_out",
            "Gemini Models operation timed out",
        ))
    } else {
        result
    }
}

