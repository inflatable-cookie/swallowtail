async fn list_pages(
    scope: ScopeId,
    endpoint: &str,
    mut secret: Vec<u8>,
    request: &ModelCatalogRequest,
    services: &HostServices,
) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
    let mut models = Vec::new();
    let mut identities = BTreeSet::new();
    let mut page_token = None;
    for page in 0..MAXIMUM_PAGES {
        ensure_before_deadline(request, services)?;
        let cancelled = Arc::new(AtomicBool::new(false));
        let response = complete_before_deadline(
            http_get(
                ScopeId::new(format!("{}:page-{page}", scope.as_str()))
                    .map_err(|_| protocol_failure())?,
                endpoint,
                page_token.as_deref(),
                secret.clone(),
                Arc::clone(&cancelled),
                services,
            ),
            request.deadline(),
            services,
            cancelled,
        )
        .await?;
        let (page_models, next) = parse_page(&response)?;
        for model in page_models {
            if !identities.insert(model.id().as_str().to_owned()) || models.len() >= MAXIMUM_MODELS
            {
                secret.fill(0);
                return Err(protocol_failure());
            }
            models.push(model);
        }
        match next {
            Some(next) => page_token = Some(next),
            None => {
                secret.fill(0);
                return Ok(models);
            }
        }
    }
    secret.fill(0);
    Err(protocol_failure())
}

fn parse_page(bytes: &[u8]) -> Result<(Vec<ModelCatalogEntry>, Option<String>), RuntimeFailure> {
    let value: Value = serde_json::from_slice(bytes).map_err(|_| protocol_failure())?;
    let models = value
        .get("models")
        .and_then(Value::as_array)
        .ok_or_else(protocol_failure)?;
    if models.len() > 1_000 {
        return Err(protocol_failure());
    }
    let models = models
        .iter()
        .map(|model| {
            let id = bounded_text(model, "baseModelId")?;
            let mut metadata = match optional_bounded_text(model, "displayName")? {
                Some(display) => {
                    ModelMetadata::with_display_name(display).map_err(|_| protocol_failure())?
                }
                None => ModelMetadata::default(),
            };
            if let Some(description) = optional_bounded_text(model, "description")? {
                metadata = metadata
                    .with_description(description)
                    .map_err(|_| protocol_failure())?;
            }
            let input = optional_positive_u64(model, "inputTokenLimit")?;
            let output = optional_positive_u64(model, "outputTokenLimit")?;
            if input.is_some() || output.is_some() {
                metadata = metadata.with_token_limits(ModelTokenLimits::new(input, output));
            }
            if let Some(reasoning) = optional_bool(model, "thinking")? {
                metadata = metadata.with_catalog_observations(
                    ModelCatalogObservations::new(
                        IntegrationFamilyId::new("gemini").expect("static family id is valid"),
                    )
                    .with_reasoning_supported(reasoning),
                );
            }
            Ok(
                ModelCatalogEntry::new(ModelId::new(id).map_err(|_| protocol_failure())?, metadata)
                    .with_provider_id(
                        ProviderId::new("google").expect("static provider id is valid"),
                    ),
            )
        })
        .collect::<Result<Vec<_>, RuntimeFailure>>()?;
    let next = match value.get("nextPageToken") {
        None | Some(Value::Null) => None,
        Some(Value::String(next)) => {
            bounded(next)?;
            Some(next.clone())
        }
        Some(_) => return Err(protocol_failure()),
    };
    Ok((models, next))
}

async fn http_get(
    scope: ScopeId,
    endpoint: &str,
    page_token: Option<&str>,
    secret: Vec<u8>,
    cancelled: Arc<AtomicBool>,
    services: &HostServices,
) -> Result<Vec<u8>, RuntimeFailure> {
    let blocking = services
        .blocking_work()
        .cloned()
        .expect("validated blocking-work service");
    let endpoint = endpoint.to_owned();
    let page_token = page_token.map(str::to_owned);
    let (sender, receiver) = oneshot::channel();
    blocking
        .run(
            scope,
            Box::new(move || {
                let result = perform_get(&endpoint, page_token.as_deref(), secret, cancelled);
                let _ = sender.send(result);
                Ok(())
            }),
        )
        .await?;
    receiver.await.map_err(|_| {
        failure(
            "swallowtail.gemini.models.blocking_result_missing",
            "Gemini Models blocking HTTP work returned no result",
        )
    })?
}

