
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
            "swallowtail.xai.models.deadline_elapsed",
            "xAI Models deadline elapsed before dispatch",
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
            "swallowtail.xai.models.timed_out",
            "xAI Models operation timed out",
        ))
    } else {
        result
    }
}


async fn http_get(
    scope: ScopeId,
    endpoint: &str,
    secret: Vec<u8>,
    cancelled: Arc<AtomicBool>,
    services: &HostServices,
) -> Result<Vec<u8>, RuntimeFailure> {
    let request = HttpRequest::new(HttpMethod::Get, "/v1/language-models")
        .with_response_limit(MAXIMUM_BODY_BYTES)
        .with_transfer_timeout(Duration::from_secs(10))
        .with_header("accept", "application/json")
        .with_secret_header(
            SecretHeader::bearer(secret).map_err(|_| protocol_failure())?,
        );
    let url = request.resolve(endpoint).map_err(|_| protocol_failure())?;
    if url.scheme() != "https"
        || url.host_str() != Some("api.x.ai")
        || url.path() != "/v1/language-models"
    {
        return Err(protocol_failure());
    }
    let response = HttpTransport::new()
        .execute(scope, endpoint, request, services, cancelled)
        .await
        .map_err(|error| {
            if error == TransportError::ResponseLimit {
                protocol_failure()
            } else {
                transport_failure()
            }
        })?;
    if !(200..300).contains(&response.status) {
        return Err(failure(
            "swallowtail.xai.models.provider_rejected",
            "xAI rejected model catalogue discovery",
        ));
    }
    Ok(response.body)
}
