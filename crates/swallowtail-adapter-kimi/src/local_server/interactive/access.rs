use crate::failure::failure;
use std::sync::Arc;
use swallowtail_core::{PreflightPlan, ResourceAccess, ResourceRepresentation, SafeDiagnostic};
use swallowtail_runtime::{
    CleanupOutcome, CredentialLease, EndpointRef, HostServices, ResourceLease, ScopeId,
    SessionAccessPolicy, WorkingResourceRef, validate_session_resource_lease,
};

pub(super) struct SessionAccess {
    pub(super) endpoint: String,
    pub(super) directory: String,
    pub(super) secret: Arc<SecretMaterial>,
    credential: Option<CredentialLease>,
    resource: Option<ResourceLease>,
}

pub(super) struct SecretMaterial(Vec<u8>);

impl SecretMaterial {
    pub(super) fn new(bytes: &[u8]) -> Self {
        Self(bytes.to_vec())
    }

    pub(super) fn copy(&self) -> Vec<u8> {
        self.0.clone()
    }
}

impl Drop for SecretMaterial {
    fn drop(&mut self) {
        self.0.fill(0);
    }
}

impl SessionAccess {
    pub(super) async fn acquire(
        plan: &PreflightPlan,
        scope: ScopeId,
        services: &HostServices,
        resource_ref: &WorkingResourceRef,
        policy: &SessionAccessPolicy,
    ) -> Result<Self, swallowtail_runtime::RuntimeFailure> {
        let network = services.network().cloned().ok_or_else(|| {
            failure(
                "swallowtail.kimi.local_server.network_service_missing",
                "Kimi local-server session requires a network policy service",
            )
        })?;
        let credential_service = services.credential().cloned().ok_or_else(|| {
            failure(
                "swallowtail.kimi.local_server.credential_service_missing",
                "Kimi local-server session requires a credential service",
            )
        })?;
        let endpoint_ref = EndpointRef::from_instance_target(plan.instance_target_ref());
        let grant = network
            .authorize(
                scope.clone(),
                endpoint_ref.clone(),
                plan.endpoint_audience().clone(),
            )
            .await?;
        if grant.scope() != &scope
            || grant.endpoint() != &endpoint_ref
            || grant.audience() != plan.endpoint_audience()
            || crate::local_server::transport::require_loopback_endpoint(
                grant.authorized().as_driver_value(),
            )
            .is_err()
        {
            return Err(failure(
                "swallowtail.kimi.local_server.network_grant_mismatch",
                "Kimi local-server network grant does not match session preflight",
            ));
        }
        let credential_ref = plan.credential_reference().expect("validated").clone();
        let credential = credential_service
            .acquire(
                scope.clone(),
                credential_ref.clone(),
                plan.endpoint_audience().clone(),
            )
            .await?;
        let secret = match &credential {
            CredentialLease::Secret(secret)
                if secret.scope() == &scope
                    && secret.reference() == &credential_ref
                    && secret.audience() == plan.endpoint_audience() =>
            {
                Arc::new(SecretMaterial::new(secret.expose_secret()))
            }
            CredentialLease::Secret(_) | CredentialLease::Delegated(_) => {
                let _ = credential_service.release(credential).await;
                return Err(failure(
                    "swallowtail.kimi.local_server.credential_lease_rejected",
                    "Kimi local-server session requires a matching bearer lease",
                ));
            }
        };
        let resource_service = services.working_resource().cloned().ok_or_else(|| {
            failure(
                "swallowtail.kimi.local_server.resource_service_missing",
                "Kimi local-server session requires a working-resource service",
            )
        })?;
        let resource = match resource_service
            .resolve(
                scope,
                resource_ref.clone(),
                ResourceAccess::ReadWrite,
                ResourceRepresentation::Filesystem,
            )
            .await
        {
            Ok(resource) => resource,
            Err(error) => {
                drop(secret);
                let _ = credential_service.release(credential).await;
                return Err(error);
            }
        };
        if let Err(error) = validate_session_resource_lease(policy, resource_ref, &resource) {
            let _ = resource_service.release(resource).await;
            drop(secret);
            let _ = credential_service.release(credential).await;
            return Err(error);
        }
        let directory = resource
            .filesystem()
            .expect("validated filesystem resource")
            .as_driver_value()
            .to_owned();
        Ok(Self {
            endpoint: grant.authorized().as_driver_value().to_owned(),
            directory,
            secret,
            credential: Some(credential),
            resource: Some(resource),
        })
    }

    pub(super) async fn release(&mut self, services: &HostServices) -> CleanupOutcome {
        let resource = match self.resource.take() {
            Some(lease) => match services.working_resource() {
                Some(service) => service.release(lease).await,
                None => CleanupOutcome::Failed(SafeDiagnostic::new(
                    "swallowtail.kimi.local_server.resource_release_failed",
                    "Kimi local-server working-resource service disappeared during cleanup",
                )),
            },
            None => CleanupOutcome::NotApplicable,
        };
        let secret = std::mem::replace(&mut self.secret, Arc::new(SecretMaterial(Vec::new())));
        drop(secret);
        let credential = match self.credential.take() {
            Some(lease) => match services.credential() {
                Some(service) => service.release(lease).await,
                None => CleanupOutcome::Failed(SafeDiagnostic::new(
                    "swallowtail.kimi.local_server.credential_release_failed",
                    "Kimi local-server credential service disappeared during cleanup",
                )),
            },
            None => CleanupOutcome::NotApplicable,
        };
        merge(resource, credential)
    }
}

pub(super) fn merge(current: CleanupOutcome, next: CleanupOutcome) -> CleanupOutcome {
    match (&current, &next) {
        (CleanupOutcome::Failed(_), _) => current,
        (_, CleanupOutcome::Failed(_)) => next,
        (CleanupOutcome::Degraded(_), _) => current,
        (_, CleanupOutcome::Degraded(_)) => next,
        (CleanupOutcome::Clean, _) => current,
        (CleanupOutcome::NotApplicable, CleanupOutcome::Clean) => next,
        _ => current,
    }
}
