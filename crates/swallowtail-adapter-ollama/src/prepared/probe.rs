use super::*;

pub(super) async fn observe_runtime(
    input: &OllamaPreparationInput,
    probe: &OllamaPreparationProbe,
    services: &HostServices,
) -> Result<OllamaPreparedRuntimeObservation, PreparationFailure> {
    let network = services.network().expect("validated network service");
    let endpoint_ref = EndpointRef::from_instance_target(&input.endpoint_target);
    let audience = input.access_profile.endpoint_audience().clone();
    let cancelled = Arc::new(AtomicBool::new(false));
    let grant = complete_probe_work(
        network.authorize(
            probe.scope_id.clone(),
            endpoint_ref.clone(),
            audience.clone(),
        ),
        probe,
        services,
        Arc::clone(&cancelled),
    )
    .await
    .map_err(|error| runtime_failure(PreparationStage::TargetSelection, error))?;
    if grant.scope() != &probe.scope_id
        || grant.endpoint() != &endpoint_ref
        || grant.audience() != &audience
    {
        return Err(failure(
            PreparationStage::TargetSelection,
            "swallowtail.ollama.preparation.network_grant_mismatch",
            "Ollama preparation network grant did not match the selected target",
        ));
    }
    let endpoint = grant.authorized().as_driver_value().to_owned();
    observe_endpoint(input, probe, services, endpoint, cancelled).await
}

async fn observe_endpoint(
    input: &OllamaPreparationInput,
    probe: &OllamaPreparationProbe,
    services: &HostServices,
    endpoint: String,
    cancelled: Arc<AtomicBool>,
) -> Result<OllamaPreparedRuntimeObservation, PreparationFailure> {
    let transport = CurlTransport;
    let version_response = request(
        &transport,
        Request::version(),
        &endpoint,
        probe,
        services,
        Arc::clone(&cancelled),
    )
    .await?;
    let version = parse_version(&version_response).map_err(version_failure)?;
    let observed_at = services
        .time()
        .expect("validated time service")
        .catalog_now()
        .map_err(|error| runtime_failure(PreparationStage::BoundedOutput, error))?;
    let binding = ObservationBinding {
        instance_id: input.instance_id.clone(),
        execution_host_id: input.execution_host_id.clone(),
        runtime_version: version.clone(),
        observed_at,
    };
    let installed = installed_inventory(
        input,
        probe,
        services,
        &transport,
        &endpoint,
        &binding,
        Arc::clone(&cancelled),
    )
    .await?;
    let running_response = request(
        &transport,
        Request::running_models(),
        &endpoint,
        probe,
        services,
        Arc::clone(&cancelled),
    )
    .await?;
    let running = parse_inventory(
        &running_response,
        AttachedModelObservationScope::RunningInventory,
        &binding,
    )
    .map_err(catalogue_failure)?;
    validate_running_inventory(&installed, &running)?;
    let detail_response = request(
        &transport,
        Request::show(input.selected_model_tag.as_str()).map_err(catalogue_failure)?,
        &endpoint,
        probe,
        services,
        cancelled,
    )
    .await?;
    let detail = parse_model_detail(
        &detail_response,
        &binding,
        input.selected_model_tag.clone(),
        input.selected_manifest_digest.clone(),
    )
    .map_err(catalogue_failure)?;
    Ok(OllamaPreparedRuntimeObservation::new(
        version, installed, running, detail,
    ))
}

async fn installed_inventory(
    input: &OllamaPreparationInput,
    probe: &OllamaPreparationProbe,
    services: &HostServices,
    transport: &CurlTransport,
    endpoint: &str,
    binding: &ObservationBinding,
    cancelled: Arc<AtomicBool>,
) -> Result<Vec<swallowtail_core::AttachedModelObservation>, PreparationFailure> {
    let response = request(
        transport,
        Request::installed_models(),
        endpoint,
        probe,
        services,
        cancelled,
    )
    .await?;
    let installed = parse_inventory(
        &response,
        AttachedModelObservationScope::InstalledInventory,
        binding,
    )
    .map_err(catalogue_failure)?;
    let selected = installed
        .iter()
        .find(|item| item.model_tag() == &input.selected_model_tag)
        .ok_or_else(|| {
            failure(
                PreparationStage::BoundedOutput,
                "swallowtail.ollama.preparation.model_not_installed",
                "The selected Ollama model was not in installed inventory",
            )
        })?;
    if selected.manifest_digest() != Some(&input.selected_manifest_digest) {
        return Err(failure(
            PreparationStage::BoundedOutput,
            "swallowtail.ollama.preparation.manifest_mismatch",
            "The selected Ollama model manifest did not match preparation input",
        ));
    }
    Ok(installed)
}
