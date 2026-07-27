use super::KimiLocalServerPreparedBindingImport;
use crate::local_server::prepared::input::KimiLocalServerBindingImportTarget;
use crate::local_server::prepared::probe::{ProbeTerminal, complete_probe_work, terminal_failure};
use crate::local_server::prepared::{
    KimiLocalServerPreparationProbe, preparation_failure, runtime_preparation_failure,
};
use crate::local_server::protocol::{RestFailureKind, RestReply, decode_rest, decode_session};
use crate::local_server::transport::{
    CurlTransport, Request, require_loopback_endpoint, session_path,
};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use swallowtail_core::ProviderSessionBindingOrigin;
use swallowtail_runtime::{
    CleanupOutcome, CredentialLease, EndpointRef, HostServices, NetworkGrant, PreparationFailure,
    PreparationStage, ProviderSessionManagementBinding,
};

pub(super) async fn execute(
    prepared: KimiLocalServerPreparedBindingImport,
    services: HostServices,
) -> Result<ProviderSessionManagementBinding, PreparationFailure> {
    validate_host_services(&prepared.target, &prepared.probe, &services)?;
    let grant = authorize_endpoint(&prepared.target, &prepared.probe, &services).await?;
    let credential_service = services.credential().expect("validated credential service");
    let audience = prepared.target.access_profile.endpoint_audience().clone();
    let reference = prepared
        .target
        .access_profile
        .credential_reference()
        .expect("prepared local-server target binds a credential")
        .clone();
    let (lease, terminal) = complete_probe_work(
        credential_service.acquire(
            prepared.probe.scope_id.clone(),
            reference.clone(),
            audience.clone(),
        ),
        &prepared.probe,
        &services,
        None,
    )
    .await;
    if let Some(terminal) = terminal {
        if let Ok(lease) = lease {
            require_clean_release(credential_service.release(lease).await)?;
        }
        return Err(terminal_failure(terminal));
    }
    let lease = lease
        .map_err(|error| runtime_preparation_failure(PreparationStage::AccessEvidence, error))?;
    let bearer = match secret_bytes(&lease, &prepared.probe, &reference, &audience) {
        Ok(bearer) => bearer,
        Err(error) => {
            require_clean_release(credential_service.release(lease).await)?;
            return Err(error);
        }
    };

    let lookup = lookup_target(&prepared, grant, bearer, &services).await;
    require_clean_release(credential_service.release(lease).await)?;
    lookup?;

    ProviderSessionManagementBinding::from_bound_session(
        prepared.provider_session_ref,
        &crate::kimi_local_server_descriptor(),
        &prepared.target.instance,
        prepared.target.access_evidence,
        Some(prepared.target.state_root),
        ProviderSessionBindingOrigin::ExplicitlyImported,
    )
    .map_err(|_| {
        preparation_failure(
            PreparationStage::Preflight,
            "swallowtail.kimi.local_server.import.binding_invalid",
            "Kimi local-server import could not issue a route-bound management binding",
        )
    })
}

async fn lookup_target(
    prepared: &KimiLocalServerPreparedBindingImport,
    grant: NetworkGrant,
    bearer: Vec<u8>,
    services: &HostServices,
) -> Result<(), PreparationFailure> {
    let path = session_path(prepared.provider_session_ref.as_provider_value())
        .map_err(|error| runtime_preparation_failure(PreparationStage::Preflight, error))?;
    let cancelled = Arc::new(AtomicBool::new(false));
    let (response, terminal) = complete_probe_work(
        CurlTransport.request(
            prepared.probe.scope_id.clone(),
            grant.authorized().as_driver_value().to_owned(),
            Request::get(path),
            Some(bearer),
            services,
            Arc::clone(&cancelled),
        ),
        &prepared.probe,
        services,
        Some(cancelled),
    )
    .await;
    if let Some(terminal) = terminal {
        return Err(terminal_failure(terminal));
    }
    let response = response
        .map_err(|error| runtime_preparation_failure(PreparationStage::BoundedOutput, error))?;
    match decode_rest(response.status, &response.body)
        .map_err(|error| runtime_preparation_failure(PreparationStage::BoundedOutput, error))?
    {
        RestReply::Success(_) => validate_lookup(prepared, &response.body),
        RestReply::Failure(RestFailureKind::Unauthorized) => Err(preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.kimi.local_server.import.access_rejected",
            "Kimi local-server rejected authenticated target lookup",
        )),
        RestReply::Failure(RestFailureKind::Missing) => Err(preparation_failure(
            PreparationStage::Preflight,
            "swallowtail.kimi.local_server.import.target_missing",
            "Kimi local-server target session does not exist",
        )),
        RestReply::Failure(_) => Err(preparation_failure(
            PreparationStage::BoundedOutput,
            "swallowtail.kimi.local_server.import.lookup_rejected",
            "Kimi local-server target lookup did not succeed",
        )),
    }
}

fn validate_lookup(
    prepared: &KimiLocalServerPreparedBindingImport,
    body: &[u8],
) -> Result<(), PreparationFailure> {
    let session = decode_session(body)
        .map_err(|error| runtime_preparation_failure(PreparationStage::BoundedOutput, error))?;
    if session.id != prepared.provider_session_ref.as_provider_value() || session.archived {
        return Err(preparation_failure(
            PreparationStage::Preflight,
            "swallowtail.kimi.local_server.import.target_ineligible",
            "Kimi local-server target lookup did not confirm the exact unarchived session",
        ));
    }
    Ok(())
}

async fn authorize_endpoint(
    target: &KimiLocalServerBindingImportTarget,
    probe: &KimiLocalServerPreparationProbe,
    services: &HostServices,
) -> Result<NetworkGrant, PreparationFailure> {
    let endpoint = EndpointRef::from_instance_target(target.instance.target_reference());
    let audience = target.access_profile.endpoint_audience().clone();
    let (grant, terminal) = complete_probe_work(
        services
            .network()
            .expect("validated network service")
            .authorize(probe.scope_id.clone(), endpoint.clone(), audience.clone()),
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
        || grant.endpoint() != &endpoint
        || grant.audience() != &audience
        || require_loopback_endpoint(grant.authorized().as_driver_value()).is_err()
    {
        return Err(preparation_failure(
            PreparationStage::TargetSelection,
            "swallowtail.kimi.local_server.import.network_grant_mismatch",
            "Kimi local-server import network grant does not match its prepared endpoint",
        ));
    }
    Ok(grant)
}

fn validate_host_services(
    target: &KimiLocalServerBindingImportTarget,
    probe: &KimiLocalServerPreparationProbe,
    services: &HostServices,
) -> Result<(), PreparationFailure> {
    if services.execution_host_id() != target.instance.execution_host_id()
        || services.blocking_work().is_none()
        || services.time().is_none()
        || services.network().is_none()
        || services.credential().is_none()
    {
        return Err(preparation_failure(
            PreparationStage::TargetSelection,
            "swallowtail.kimi.local_server.import.host_services_rejected",
            "Kimi local-server import requires matching bounded host services",
        ));
    }
    if probe.cancellation.is_requested() {
        return Err(terminal_failure(ProbeTerminal::Cancelled));
    }
    if services.time().expect("validated time").now() >= probe.deadline.instant() {
        return Err(preparation_failure(
            PreparationStage::BoundedOutput,
            "swallowtail.kimi.local_server.import.deadline_elapsed",
            "Kimi local-server import deadline elapsed before host work",
        ));
    }
    Ok(())
}

fn secret_bytes(
    lease: &CredentialLease,
    probe: &KimiLocalServerPreparationProbe,
    reference: &swallowtail_runtime::CredentialRef,
    audience: &swallowtail_core::EndpointAudience,
) -> Result<Vec<u8>, PreparationFailure> {
    match lease {
        CredentialLease::Secret(secret)
            if secret.scope() == &probe.scope_id
                && secret.reference() == reference
                && secret.audience() == audience =>
        {
            Ok(secret.expose_secret().to_vec())
        }
        CredentialLease::Secret(_) | CredentialLease::Delegated(_) => Err(preparation_failure(
            PreparationStage::AccessEvidence,
            "swallowtail.kimi.local_server.import.credential_lease_rejected",
            "Kimi local-server import requires its exact bearer credential lease",
        )),
    }
}

fn require_clean_release(cleanup: CleanupOutcome) -> Result<(), PreparationFailure> {
    if matches!(
        cleanup,
        CleanupOutcome::Clean | CleanupOutcome::NotApplicable
    ) {
        Ok(())
    } else {
        Err(preparation_failure(
            PreparationStage::Cleanup,
            "swallowtail.kimi.local_server.import.cleanup_failed",
            "Kimi local-server import credential cleanup could not be joined",
        ))
    }
}
