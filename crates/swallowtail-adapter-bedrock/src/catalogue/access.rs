use super::BedrockCatalogueBinding;
use crate::failure::failure;
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    CleanupOutcome, CredentialLease, EndpointRef, HostServices, RuntimeFailure, ScopeId,
};

pub(super) struct AccessLease {
    pub(super) endpoint: String,
    credential: Option<CredentialLease>,
}

impl AccessLease {
    pub(super) async fn acquire(
        plan: &PreflightPlan,
        binding: &BedrockCatalogueBinding,
        scope: ScopeId,
        services: &HostServices,
    ) -> Result<Self, RuntimeFailure> {
        let endpoint_ref = EndpointRef::from_instance_target(plan.instance_target_ref());
        let grant = services
            .network()
            .expect("validated")
            .authorize(
                scope.clone(),
                endpoint_ref.clone(),
                plan.endpoint_audience().clone(),
            )
            .await?;
        if grant.scope() != &scope
            || grant.endpoint() != &endpoint_ref
            || grant.audience() != plan.endpoint_audience()
        {
            return Err(failure(
                "swallowtail.bedrock.catalogue_network_grant_mismatch",
                "Bedrock catalogue network grant did not match preflight",
            ));
        }
        let credential = services
            .credential()
            .expect("validated")
            .acquire(
                scope.clone(),
                binding.credential().clone(),
                plan.endpoint_audience().clone(),
            )
            .await?;
        if credential.scope() != &scope
            || credential.reference() != binding.credential()
            || credential.audience() != plan.endpoint_audience()
            || !matches!(credential, CredentialLease::Delegated(_))
        {
            let _ = services
                .credential()
                .expect("validated")
                .release(credential)
                .await;
            return Err(failure(
                "swallowtail.bedrock.catalogue_credential_lease_rejected",
                "Bedrock catalogue requires an exact delegated credential lease",
            ));
        }
        Ok(Self {
            endpoint: grant.authorized().as_driver_value().to_owned(),
            credential: Some(credential),
        })
    }

    pub(super) async fn release(&mut self, services: &HostServices) -> CleanupOutcome {
        match self.credential.take() {
            Some(lease) => {
                services
                    .credential()
                    .expect("validated")
                    .release(lease)
                    .await
            }
            None => CleanupOutcome::NotApplicable,
        }
    }
}
