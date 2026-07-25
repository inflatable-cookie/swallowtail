use swallowtail_core::AttachedModelObservation;
use swallowtail_runtime::RuntimeFailure;

fn validate_input(
    input: &OllamaPreparationInput,
    services: &HostServices,
) -> Result<(), PreparationFailure> {
    if services.execution_host_id() != &input.execution_host_id
        || services.blocking_work().is_none()
        || services.time().is_none()
        || services.network().is_none()
    {
        return Err(failure(
            PreparationStage::TargetSelection,
            "swallowtail.ollama.preparation.host_services_rejected",
            "Ollama preparation requires matching network, time, and blocking services",
        ));
    }
    if input.access_profile.credential_mechanism()
        != &CredentialMechanism::LocalUnauthenticated
        || input.access_profile.credential_reference().is_some()
        || input.access_profile.entitlement_metering() != &EntitlementMetering::LocalCompute
        || input.access_profile.support_authority()
            != SupportAuthority::IntegrationMaintainerSupported
        || input.access_profile.endpoint_audience().as_str() != ENDPOINT_AUDIENCE
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.ollama.preparation.access_profile_rejected",
            "Ollama attached preparation requires local unauthenticated access",
        ));
    }
    let status = input.access_evidence.status();
    if status.profile_id() != input.access_profile.id()
        || status.support_authority() != input.access_profile.support_authority()
    {
        return Err(failure(
            PreparationStage::AccessEvidence,
            "swallowtail.ollama.preparation.access_evidence_mismatch",
            "Ollama access evidence does not match the selected access profile",
        ));
    }
    Ok(())
}

fn validate_probe(
    probe: &OllamaPreparationProbe,
    services: &HostServices,
) -> Result<(), PreparationFailure> {
    if probe.cancellation.is_requested() {
        return Err(failure(
            PreparationStage::BoundedOutput,
            "swallowtail.ollama.preparation.cancelled",
            "Ollama preparation was cancelled",
        ));
    }
    if services.time().expect("validated time service").now() >= probe.deadline.instant() {
        return Err(failure(
            PreparationStage::BoundedOutput,
            "swallowtail.ollama.preparation.deadline_elapsed",
            "Ollama preparation deadline elapsed before endpoint work",
        ));
    }
    Ok(())
}

async fn request(
    transport: &CurlTransport,
    request: Request,
    endpoint: &str,
    probe: &OllamaPreparationProbe,
    services: &HostServices,
    cancelled: Arc<AtomicBool>,
) -> Result<crate::protocol::Response, PreparationFailure> {
    complete_probe_work(
        transport.request(
            probe.scope_id.clone(),
            endpoint.to_owned(),
            request,
            services,
            Arc::clone(&cancelled),
        ),
        probe,
        services,
        cancelled,
    )
    .await
    .map_err(|error| runtime_failure(PreparationStage::BoundedOutput, error))
}

fn validate_running_inventory(
    installed: &[AttachedModelObservation],
    running: &[AttachedModelObservation],
) -> Result<(), PreparationFailure> {
    if running.iter().any(|candidate| {
        !installed.iter().any(|item| {
            item.model_tag() == candidate.model_tag()
                && item.manifest_digest() == candidate.manifest_digest()
        })
    }) {
        return Err(failure(
            PreparationStage::BoundedOutput,
            "swallowtail.ollama.preparation.inventory_drift",
            "Ollama running inventory did not match installed inventory",
        ));
    }
    Ok(())
}

fn version_failure(error: RuntimeFailure) -> PreparationFailure {
    let stage = match error.diagnostic().code() {
        "swallowtail.ollama.version_unsupported" => {
            PreparationStage::CompatibilityClassification
        }
        "swallowtail.ollama.protocol_invalid" => PreparationStage::VersionParse,
        _ => PreparationStage::BoundedOutput,
    };
    runtime_failure(stage, error)
}

fn catalogue_failure(error: RuntimeFailure) -> PreparationFailure {
    runtime_failure(PreparationStage::BoundedOutput, error)
}

fn runtime_failure(stage: PreparationStage, error: RuntimeFailure) -> PreparationFailure {
    PreparationFailure::new(
        stage,
        swallowtail_core::Diagnostic::new(error.diagnostic().clone()),
    )
}

pub(crate) fn failure(
    stage: PreparationStage,
    code: &'static str,
    message: &'static str,
) -> PreparationFailure {
    PreparationFailure::new(
        stage,
        swallowtail_core::Diagnostic::new(swallowtail_core::SafeDiagnostic::new(code, message)),
    )
}
