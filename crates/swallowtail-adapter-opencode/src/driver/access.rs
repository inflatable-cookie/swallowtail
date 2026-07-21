struct AccessLeases {
    endpoint: String,
    credential: Option<CredentialLease>,
    resource: Option<ResourceLease>,
    directory: Option<String>,
}

impl AccessLeases {
    async fn acquire(
        plan: &PreflightPlan,
        scope: ScopeId,
        services: &HostServices,
        resource: Option<(
            &swallowtail_runtime::WorkingResourceRef,
            &SessionAccessPolicy,
        )>,
    ) -> Result<Self, RuntimeFailure> {
        let network = services.network().cloned().ok_or_else(|| {
            failure(
                "swallowtail.opencode.network_service_missing",
                "OpenCode HTTP requires a network policy service",
            )
        })?;
        let credential_service = services.credential().cloned().ok_or_else(|| {
            failure(
                "swallowtail.opencode.credential_service_missing",
                "OpenCode HTTP requires a credential service",
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
        {
            return Err(failure(
                "swallowtail.opencode.network_grant_mismatch",
                "OpenCode network grant does not match preflight",
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
        if credential.scope() != &scope
            || credential.reference() != &credential_ref
            || credential.audience() != plan.endpoint_audience()
            || !matches!(credential, CredentialLease::Delegated(_))
        {
            let _ = credential_service.release(credential).await;
            return Err(failure(
                "swallowtail.opencode.credential_lease_rejected",
                "OpenCode requires a matching delegated credential lease",
            ));
        }
        let mut leases = Self {
            endpoint: grant.authorized().as_driver_value().to_owned(),
            credential: Some(credential),
            resource: None,
            directory: None,
        };
        if let Some((reference, policy)) = resource {
            let resource_service = services.working_resource().cloned().ok_or_else(|| {
                failure(
                    "swallowtail.opencode.resource_service_missing",
                    "OpenCode session requires a working-resource service",
                )
            })?;
            let lease = match resource_service
                .resolve(
                    scope,
                    reference.clone(),
                    ResourceAccess::Read,
                    ResourceRepresentation::Filesystem,
                )
                .await
            {
                Ok(lease) => lease,
                Err(error) => {
                    let _ = leases.release(services).await;
                    return Err(error);
                }
            };
            if let Err(error) = validate_session_resource_lease(policy, reference, &lease) {
                let _ = resource_service.release(lease).await;
                let _ = leases.release(services).await;
                return Err(error);
            }
            leases.directory = lease
                .filesystem()
                .map(|filesystem| filesystem.as_driver_value().to_owned());
            leases.resource = Some(lease);
        }
        Ok(leases)
    }

    async fn release(&mut self, services: &HostServices) -> CleanupOutcome {
        let resource = match self.resource.take() {
            Some(lease) => match services.working_resource() {
                Some(service) => service.release(lease).await,
                None => CleanupOutcome::Failed(SafeDiagnostic::new(
                    "swallowtail.opencode.resource_release_failed",
                    "OpenCode working-resource service disappeared during cleanup",
                )),
            },
            None => CleanupOutcome::NotApplicable,
        };
        let credential = match self.credential.take() {
            Some(lease) => match services.credential() {
                Some(service) => service.release(lease).await,
                None => CleanupOutcome::Failed(SafeDiagnostic::new(
                    "swallowtail.opencode.credential_release_failed",
                    "OpenCode credential service disappeared during cleanup",
                )),
            },
            None => CleanupOutcome::NotApplicable,
        };
        merge_cleanup(resource, credential)
    }
}


