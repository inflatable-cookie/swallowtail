use super::lifecycle::{complete_probe_work, complete_probe_work_outcome, terminal_failure};
use super::{
    OpenCodePreparationInput, OpenCodePreparationProbe, OpenCodePreparedServerObservation, failure,
    runtime_failure, validation,
};
use crate::protocol::{Request, observe_health};
use crate::transport::CurlTransport;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use swallowtail_runtime::{
    CredentialLease, EndpointRef, HostServices, PreparationFailure, PreparationStage,
};

pub(super) async fn observe_server(
    input: &OpenCodePreparationInput,
    probe: &OpenCodePreparationProbe,
    services: &HostServices,
) -> Result<OpenCodePreparedServerObservation, PreparationFailure> {
    let endpoint_ref = EndpointRef::from_instance_target(&input.endpoint_target);
    let audience = input.access_profile.endpoint_audience().clone();
    let cancelled = Arc::new(AtomicBool::new(false));
    let network = services.network().expect("validated network service");
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
            "swallowtail.opencode.preparation.network_grant_mismatch",
            "OpenCode preparation network grant did not match the selected target",
        ));
    }

    let credential_service = services.credential().expect("validated credential service");
    let credential_reference = input
        .access_profile
        .credential_reference()
        .expect("validated credential reference")
        .clone();
    let (credential, terminal) = complete_probe_work_outcome(
        credential_service.acquire(
            probe.scope_id.clone(),
            credential_reference.clone(),
            audience.clone(),
        ),
        probe,
        services,
        Arc::clone(&cancelled),
    )
    .await;
    if let Some(terminal) = terminal {
        let terminal = runtime_failure(PreparationStage::BoundedOutput, terminal_failure(terminal));
        return match credential {
            Ok(credential) => match credential_service.release(credential).await {
                swallowtail_runtime::CleanupOutcome::Clean
                | swallowtail_runtime::CleanupOutcome::NotApplicable => Err(terminal),
                _ => Err(cleanup_failure(terminal)),
            },
            Err(_) => Err(terminal),
        };
    }
    let credential =
        credential.map_err(|error| runtime_failure(PreparationStage::AccessEvidence, error))?;
    if credential.scope() != &probe.scope_id
        || credential.reference() != &credential_reference
        || credential.audience() != &audience
        || !matches!(credential, CredentialLease::Delegated(_))
    {
        let cleanup = credential_service.release(credential).await;
        let mismatch = failure(
            PreparationStage::AccessEvidence,
            "swallowtail.opencode.preparation.credential_lease_rejected",
            "OpenCode preparation requires a matching delegated credential lease",
        );
        return match cleanup {
            swallowtail_runtime::CleanupOutcome::Clean
            | swallowtail_runtime::CleanupOutcome::NotApplicable => Err(mismatch),
            _ => Err(cleanup_failure(mismatch)),
        };
    }

    let result = observe_health_response(
        grant.authorized().as_driver_value().to_owned(),
        probe,
        services,
        cancelled,
    )
    .await;
    let cleanup = credential_service.release(credential).await;
    match (result, cleanup) {
        (
            Ok(server),
            swallowtail_runtime::CleanupOutcome::Clean
            | swallowtail_runtime::CleanupOutcome::NotApplicable,
        ) => Ok(server),
        (Err(error), swallowtail_runtime::CleanupOutcome::Clean)
        | (Err(error), swallowtail_runtime::CleanupOutcome::NotApplicable) => Err(error),
        (Ok(_), _) => Err(failure(
            PreparationStage::Cleanup,
            "swallowtail.opencode.preparation.cleanup_failed",
            "OpenCode preparation credential cleanup failed",
        )),
        (Err(error), _) => Err(cleanup_failure(error)),
    }
}

async fn observe_health_response(
    endpoint: String,
    probe: &OpenCodePreparationProbe,
    services: &HostServices,
    cancelled: Arc<AtomicBool>,
) -> Result<OpenCodePreparedServerObservation, PreparationFailure> {
    let response = complete_probe_work(
        CurlTransport.request(
            probe.scope_id.clone(),
            endpoint,
            Request::get("/global/health"),
            services,
            Arc::clone(&cancelled),
        ),
        probe,
        services,
        cancelled,
    )
    .await
    .map_err(|error| runtime_failure(PreparationStage::BoundedOutput, error))?;
    let observation = observe_health(&response).map_err(validation::health_failure)?;
    Ok(OpenCodePreparedServerObservation::new(
        observation.binding().clone(),
        observation.assessment().clone(),
    ))
}

fn cleanup_failure(cause: PreparationFailure) -> PreparationFailure {
    failure(
        PreparationStage::Cleanup,
        "swallowtail.opencode.preparation.cleanup_failed",
        "OpenCode preparation credential cleanup failed",
    )
    .with_cause(cause)
}
