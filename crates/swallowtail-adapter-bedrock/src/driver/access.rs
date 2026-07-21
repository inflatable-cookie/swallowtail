struct AccessLease {
    endpoint: String,
    credential: Option<CredentialLease>,
}

struct PendingRun {
    access: AccessLease,
    job: BlockingJob,
}

impl AccessLease {
    async fn acquire(
        plan: &PreflightPlan,
        binding: &BedrockDriverBinding,
        scope: ScopeId,
        services: &HostServices,
    ) -> Result<Self, RuntimeFailure> {
        let endpoint_ref = EndpointRef::from_instance_target(plan.instance_target_ref());
        let grant = services.network().expect("validated").authorize(
            scope.clone(),
            endpoint_ref.clone(),
            plan.endpoint_audience().clone(),
        ).await?;
        if grant.scope() != &scope || grant.endpoint() != &endpoint_ref
            || grant.audience() != plan.endpoint_audience()
        {
            return Err(failure("swallowtail.bedrock.network_grant_mismatch", "Bedrock Runtime network grant did not match preflight"));
        }
        let credential = services.credential().expect("validated").acquire(
            scope.clone(),
            binding.credential().clone(),
            plan.endpoint_audience().clone(),
        ).await?;
        if credential.scope() != &scope || credential.reference() != binding.credential()
            || credential.audience() != plan.endpoint_audience()
            || !matches!(credential, CredentialLease::Delegated(_))
        {
            let _ = services.credential().expect("validated").release(credential).await;
            return Err(failure("swallowtail.bedrock.credential_lease_rejected", "Bedrock Runtime requires an exact delegated credential lease"));
        }
        Ok(Self {
            endpoint: grant.authorized().as_driver_value().to_owned(),
            credential: Some(credential),
        })
    }

    async fn release(&mut self, services: &HostServices) -> CleanupOutcome {
        match self.credential.take() {
            Some(lease) => services.credential().expect("validated").release(lease).await,
            None => CleanupOutcome::NotApplicable,
        }
    }
}
