use swallowtail_runtime::{CleanupOutcome, CredentialLease, HostServices, ResourceLease};

use super::cleanup_failure;

pub(super) async fn release_resource(
    lease: Option<ResourceLease>,
    services: &HostServices,
) -> CleanupOutcome {
    match (lease, services.working_resource()) {
        (Some(lease), Some(service)) => service.release(lease).await,
        (Some(_), None) => cleanup_failure(
            "resource_release_failed",
            "Pi RPC working-resource service disappeared during cleanup",
        ),
        (None, _) => CleanupOutcome::NotApplicable,
    }
}

pub(super) async fn release_credential(
    lease: Option<CredentialLease>,
    services: &HostServices,
) -> CleanupOutcome {
    match (lease, services.credential()) {
        (Some(lease), Some(service)) => service.release(lease).await,
        (Some(_), None) => cleanup_failure(
            "credential_release_failed",
            "Pi RPC credential service disappeared during cleanup",
        ),
        (None, _) => CleanupOutcome::NotApplicable,
    }
}

pub(super) fn merge_cleanup(left: CleanupOutcome, right: CleanupOutcome) -> CleanupOutcome {
    match (left, right) {
        (CleanupOutcome::Failed(error), _) | (_, CleanupOutcome::Failed(error)) => {
            CleanupOutcome::Failed(error)
        }
        (CleanupOutcome::Degraded(error), _) | (_, CleanupOutcome::Degraded(error)) => {
            CleanupOutcome::Degraded(error)
        }
        (CleanupOutcome::Clean, CleanupOutcome::Clean | CleanupOutcome::NotApplicable)
        | (CleanupOutcome::NotApplicable, CleanupOutcome::Clean) => CleanupOutcome::Clean,
        (CleanupOutcome::NotApplicable, CleanupOutcome::NotApplicable) => {
            CleanupOutcome::NotApplicable
        }
    }
}
