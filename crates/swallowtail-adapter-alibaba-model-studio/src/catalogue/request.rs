
async fn list_sources(
    transport: &CurlTransport,
    scope: ScopeId,
    endpoint: &str,
    mut secret: Vec<u8>,
    request: &ModelCatalogRequest,
    services: &HostServices,
) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
    let mut models = Vec::new();
    let mut identities = BTreeSet::new();
    for source in ["base", "custom"] {
        let mut page = 1_u32;
        loop {
            ensure_before_deadline(request, services)?;
            let cancelled = Arc::new(AtomicBool::new(false));
            let response = complete_before_deadline(
                transport.request(
                    ScopeId::new(format!("{}:{source}:{page}", scope.as_str()))
                        .map_err(|_| protocol_failure())?,
                    endpoint.to_owned(),
                    secret.clone(),
                    WireRequest::deployable_models(page, source),
                    services,
                    Arc::clone(&cancelled),
                ),
                request.deadline(),
                services,
                cancelled,
            )
            .await?;
            let parsed = parse_page(&response.body, page)?;
            for model in parsed.models {
                if !identities.insert(model.id().as_str().to_owned())
                    || models.len() >= MAXIMUM_MODELS
                {
                    secret.fill(0);
                    return Err(protocol_failure());
                }
                models.push(model);
            }
            if parsed.last_page {
                break;
            }
            page = page.checked_add(1).ok_or_else(protocol_failure)?;
            if page > MAXIMUM_PAGES_PER_SOURCE {
                secret.fill(0);
                return Err(protocol_failure());
            }
        }
    }
    secret.fill(0);
    Ok(models)
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
            "swallowtail.alibaba_model_studio.models.deadline_elapsed",
            "Alibaba deployable-model deadline elapsed before dispatch",
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
            "swallowtail.alibaba_model_studio.models.timed_out",
            "Alibaba deployable-model operation timed out",
        ))
    } else {
        result
    }
}

