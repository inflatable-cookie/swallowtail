struct ManagedAccessLeases {
    endpoint: String,
    credential: Option<CredentialLease>,
}

impl ManagedAccessLeases {
    async fn acquire(
        plan: &PreflightPlan,
        scope: ScopeId,
        services: &HostServices,
    ) -> Result<Self, RuntimeFailure> {
        let network = services.network().ok_or_else(|| missing_access("network"))?;
        let credentials = services
            .credential()
            .ok_or_else(|| missing_access("credential"))?;
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
        {
            return Err(failure(
                "swallowtail.anthropic.managed.network_grant_mismatch",
                "Anthropic Managed Agents network grant did not match preflight",
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
                "swallowtail.anthropic.managed.credential_lease_rejected",
                "Anthropic Managed Agents credential lease did not match preflight",
            ));
        }
        Ok(Self {
            endpoint: grant.authorized().as_driver_value().to_owned(),
            credential: Some(credential),
        })
    }

    fn secret(&self) -> Result<&[u8], RuntimeFailure> {
        match self.credential.as_ref() {
            Some(CredentialLease::Secret(secret)) => Ok(secret.expose_secret()),
            _ => Err(missing_access("credential lease")),
        }
    }

    async fn release(&mut self, services: &HostServices) -> CleanupOutcome {
        match self.credential.take() {
            Some(lease) => match services.credential() {
                Some(service) => service.release(lease).await,
                None => CleanupOutcome::Failed(swallowtail_core::SafeDiagnostic::new(
                    "swallowtail.anthropic.managed.credential_release_failed",
                    "Anthropic Managed Agents credential service disappeared during cleanup",
                )),
            },
            None => CleanupOutcome::NotApplicable,
        }
    }
}

fn missing_access(subject: &str) -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.managed.access_unavailable",
        format!("Anthropic Managed Agents {subject} was unavailable"),
    )
}
