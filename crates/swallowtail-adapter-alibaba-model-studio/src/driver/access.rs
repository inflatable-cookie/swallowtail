use crate::failure::failure;
use swallowtail_core::PreflightPlan;
use swallowtail_runtime::{
    CleanupOutcome, CredentialLease, EndpointRef, HostServices, RuntimeFailure, ScopeId,
};

pub(super) struct AccessLeases {
    pub(super) endpoint: String,
    credential: Option<CredentialLease>,
}

impl AccessLeases {
    pub(super) async fn acquire(
        plan: &PreflightPlan,
        scope: ScopeId,
        services: &HostServices,
    ) -> Result<Self, RuntimeFailure> {
        let network = services.network().ok_or_else(|| missing("network"))?;
        let credentials = services.credential().ok_or_else(|| missing("credential"))?;
        let endpoint = EndpointRef::from_instance_target(plan.instance_target_ref());
        let grant = network
            .authorize(
                scope.clone(),
                endpoint.clone(),
                plan.endpoint_audience().clone(),
            )
            .await?;
        if grant.scope() != &scope
            || grant.endpoint() != &endpoint
            || grant.audience() != plan.endpoint_audience()
        {
            return Err(failure(
                "swallowtail.alibaba_model_studio.network_grant_mismatch",
                "Alibaba Model Studio network grant did not match preflight",
            ));
        }
        let reference = plan.credential_reference().expect("validated").clone();
        let credential = credentials
            .acquire(
                scope.clone(),
                reference.clone(),
                plan.endpoint_audience().clone(),
            )
            .await?;
        if credential.scope() != &scope
            || credential.reference() != &reference
            || credential.audience() != plan.endpoint_audience()
            || !matches!(credential, CredentialLease::Secret(_))
        {
            let _ = credentials.release(credential).await;
            return Err(failure(
                "swallowtail.alibaba_model_studio.credential_lease_rejected",
                "Alibaba Model Studio credential lease did not match preflight",
            ));
        }
        Ok(Self {
            endpoint: grant.authorized().as_driver_value().to_owned(),
            credential: Some(credential),
        })
    }

    pub(super) fn secret(&self) -> Result<Vec<u8>, RuntimeFailure> {
        match self.credential.as_ref() {
            Some(CredentialLease::Secret(secret)) => Ok(secret.expose_secret().to_vec()),
            _ => Err(missing("secret credential")),
        }
    }

    pub(super) async fn release(&mut self, services: &HostServices) -> CleanupOutcome {
        match self.credential.take() {
            Some(lease) => match services.credential() {
                Some(service) => service.release(lease).await,
                None => CleanupOutcome::Failed(swallowtail_core::SafeDiagnostic::new(
                    "swallowtail.alibaba_model_studio.credential_release_failed",
                    "Alibaba Model Studio credential service disappeared during cleanup",
                )),
            },
            None => CleanupOutcome::NotApplicable,
        }
    }
}

fn missing(service: &'static str) -> RuntimeFailure {
    failure(
        "swallowtail.alibaba_model_studio.host_service_missing",
        format!("Alibaba Model Studio requires the {service} service"),
    )
}
