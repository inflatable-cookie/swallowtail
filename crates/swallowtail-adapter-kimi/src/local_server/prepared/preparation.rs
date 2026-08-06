use super::input::{KimiLocalServerAttachedInput, KimiLocalServerPreparationProbe};
use super::instance::build_prepared;
use super::probe::{complete_probe_work, terminal_failure};
use super::validation::validate_input;
use super::{
    KimiLocalServerObservation, KimiLocalServerPreparedIntegration, preparation_failure,
    runtime_preparation_failure,
};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use swallowtail_core::InstanceOwnership;
use swallowtail_runtime::{
    CleanupOutcome, CredentialLease, EndpointRef, HostServices, NetworkGrant, PreparationFailure,
    PreparationStage,
};

use crate::local_server::protocol::{decode_health, decode_metadata};
use crate::local_server::selection::corroborate_versions;
use crate::local_server::transport::{CurlTransport, Request, require_loopback_endpoint};

/// Probes and prepares one caller-owned attached Kimi local server.
pub async fn prepare_kimi_local_server_attached(
    input: KimiLocalServerAttachedInput,
    probe: KimiLocalServerPreparationProbe,
    services: HostServices,
) -> Result<KimiLocalServerPreparedIntegration, PreparationFailure> {
    validate_input(&input, &probe, &services, false)?;
    let available = services.available_kinds();
    let server = observe_server(&input, &probe, &services).await?;
    Ok(build_prepared(
        input,
        InstanceOwnership::ExternalAttached,
        None,
        server,
        available,
    ))
}

pub(super) async fn authorize_selected_endpoint(
    input: &KimiLocalServerAttachedInput,
    probe: &KimiLocalServerPreparationProbe,
    services: &HostServices,
) -> Result<NetworkGrant, PreparationFailure> {
    let endpoint_ref = EndpointRef::from_instance_target(&input.endpoint_target);
    let audience = input.access_profile.endpoint_audience().clone();
    let (grant, terminal) = complete_probe_work(
        services
            .network()
            .expect("validated network service")
            .authorize(
                probe.scope_id.clone(),
                endpoint_ref.clone(),
                audience.clone(),
            ),
        probe,
        services,
        None,
    )
    .await;
    if let Some(terminal) = terminal {
        return Err(terminal_failure(terminal));
    }
    let grant = grant
        .map_err(|error| runtime_preparation_failure(PreparationStage::TargetSelection, error))?;
    if grant.scope() != &probe.scope_id
        || grant.endpoint() != &endpoint_ref
        || grant.audience() != &audience
        || require_loopback_endpoint(grant.authorized().as_driver_value()).is_err()
    {
        return Err(preparation_failure(
            PreparationStage::TargetSelection,
            "swallowtail.kimi.local_server.preparation.network_grant_mismatch",
            "Kimi local-server network grant did not match the selected loopback target",
        ));
    }
    Ok(grant)
}

pub(super) async fn observe_server(
    input: &KimiLocalServerAttachedInput,
    probe: &KimiLocalServerPreparationProbe,
    services: &HostServices,
) -> Result<KimiLocalServerObservation, PreparationFailure> {
    let grant = authorize_selected_endpoint(input, probe, services).await?;
    let audience = input.access_profile.endpoint_audience().clone();
    let reference = input
        .access_profile
        .credential_reference()
        .expect("validated credential reference")
        .clone();
    let credential_service = services.credential().expect("validated credential service");
    let (lease, terminal) = complete_probe_work(
        credential_service.acquire(probe.scope_id.clone(), reference.clone(), audience.clone()),
        probe,
        services,
        None,
    )
    .await;
    if let Some(terminal) = terminal {
        if let Ok(lease) = lease {
            let cleanup = credential_service.release(lease).await;
            if !matches!(
                cleanup,
                CleanupOutcome::Clean | CleanupOutcome::NotApplicable
            ) {
                return Err(cleanup_failure());
            }
        }
        return Err(terminal_failure(terminal));
    }
    let lease = lease
        .map_err(|error| runtime_preparation_failure(PreparationStage::AccessEvidence, error))?;
    let bearer = match &lease {
        CredentialLease::Secret(secret)
            if secret.scope() == &probe.scope_id
                && secret.reference() == &reference
                && secret.audience() == &audience =>
        {
            secret.expose_secret().to_vec()
        }
        CredentialLease::Secret(_) | CredentialLease::Delegated(_) => {
            let cleanup = credential_service.release(lease).await;
            return if matches!(
                cleanup,
                CleanupOutcome::Clean | CleanupOutcome::NotApplicable
            ) {
                Err(preparation_failure(
                    PreparationStage::AccessEvidence,
                    "swallowtail.kimi.local_server.preparation.credential_lease_rejected",
                    "Kimi local-server preparation requires a matching secret bearer lease",
                ))
            } else {
                Err(cleanup_failure())
            };
        }
    };

    let cancelled = Arc::new(AtomicBool::new(false));
    let result = async {
        let endpoint = grant.authorized().as_driver_value().to_owned();
        let (health, terminal) = complete_probe_work(
            CurlTransport.request(
                probe.scope_id.clone(),
                endpoint.clone(),
                Request::get("/api/v1/healthz"),
                None,
                services,
                Arc::clone(&cancelled),
            ),
            probe,
            services,
            Some(Arc::clone(&cancelled)),
        )
        .await;
        if let Some(terminal) = terminal {
            return Err(terminal_failure(terminal));
        }
        let health = health
            .map_err(|error| runtime_preparation_failure(PreparationStage::BoundedOutput, error))?;
        if health.status != 200 {
            return Err(preparation_failure(
                PreparationStage::BoundedOutput,
                "swallowtail.kimi.local_server.preparation.health_rejected",
                "Kimi local-server health probe did not succeed",
            ));
        }
        decode_health(&health.body)
            .map_err(|error| runtime_preparation_failure(PreparationStage::BoundedOutput, error))?;

        let (metadata, terminal) = complete_probe_work(
            CurlTransport.request(
                probe.scope_id.clone(),
                endpoint,
                Request::get("/api/v1/meta"),
                Some(bearer),
                services,
                Arc::clone(&cancelled),
            ),
            probe,
            services,
            Some(cancelled),
        )
        .await;
        if let Some(terminal) = terminal {
            return Err(terminal_failure(terminal));
        }
        let metadata = metadata
            .map_err(|error| runtime_preparation_failure(PreparationStage::BoundedOutput, error))?;
        if metadata.status != 200 {
            return Err(preparation_failure(
                PreparationStage::AccessEvidence,
                "swallowtail.kimi.local_server.preparation.metadata_rejected",
                "Authenticated Kimi local-server metadata probe did not succeed",
            ));
        }
        let metadata = decode_metadata(&metadata.body)
            .map_err(|error| runtime_preparation_failure(PreparationStage::BoundedOutput, error))?;
        if metadata.backend != "v2" || !metadata.websocket {
            return Err(preparation_failure(
                PreparationStage::CompatibilityClassification,
                "swallowtail.kimi.local_server.preparation.protocol_incompatible",
                "Kimi local server did not expose the required REST and WebSocket v2 surface",
            ));
        }
        let compatibility = corroborate_versions(&input.executable_version, &metadata.version)
            .map_err(|error| {
                runtime_preparation_failure(PreparationStage::CompatibilityClassification, error)
            })?;
        Ok(KimiLocalServerObservation::new(
            input.executable_version.clone(),
            compatibility,
        ))
    }
    .await;
    let cleanup = credential_service.release(lease).await;
    match (result, cleanup) {
        (Ok(observation), CleanupOutcome::Clean | CleanupOutcome::NotApplicable) => Ok(observation),
        (Err(error), CleanupOutcome::Clean | CleanupOutcome::NotApplicable) => Err(error),
        _ => Err(cleanup_failure()),
    }
}

fn cleanup_failure() -> PreparationFailure {
    preparation_failure(
        PreparationStage::Cleanup,
        "swallowtail.kimi.local_server.preparation.cleanup_failed",
        "Kimi local-server preparation cleanup could not be joined",
    )
}
