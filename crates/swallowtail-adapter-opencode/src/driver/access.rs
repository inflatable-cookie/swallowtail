struct AccessLeases {
    endpoint: String,
    credential: Option<CredentialLease>,
    resource: Option<ResourceLease>,
    directory: Option<String>,
}

/// The plan-derived inputs one OpenCode access acquisition needs.
struct AccessBinding<'a> {
    scope: ScopeId,
    endpoint: EndpointRef,
    audience: swallowtail_core::EndpointAudience,
    credential: swallowtail_core::CredentialRef,
    resource: Option<(&'a swallowtail_runtime::WorkingResourceRef, &'a SessionAccessPolicy)>,
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
        Self::acquire_binding(
            AccessBinding {
                scope,
                endpoint: EndpointRef::from_instance_target(plan.instance_target_ref()),
                audience: plan.endpoint_audience().clone(),
                credential: plan.credential_reference().expect("validated").clone(),
                resource,
            },
            services,
        )
        .await
    }

    async fn acquire_binding(
        binding: AccessBinding<'_>,
        services: &HostServices,
    ) -> Result<Self, RuntimeFailure> {
        let AccessBinding {
            scope,
            endpoint: endpoint_ref,
            audience,
            credential: credential_ref,
            resource,
        } = binding;
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
        let grant = network
            .authorize(scope.clone(), endpoint_ref.clone(), audience.clone())
            .await?;
        if grant.scope() != &scope || grant.endpoint() != &endpoint_ref || grant.audience() != &audience
        {
            return Err(failure(
                "swallowtail.opencode.network_grant_mismatch",
                "OpenCode network grant does not match preflight",
            ));
        }
        let credential = credential_service
            .acquire(scope.clone(), credential_ref.clone(), audience.clone())
            .await?;
        if credential.scope() != &scope
            || credential.reference() != &credential_ref
            || credential.audience() != &audience
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
        if let Some((reference, policy)) = resource
            && let Err(error) = leases
                .attach_resource(scope, services, reference, policy)
                .await
        {
            // The credential lease is already held: release it before returning
            // so a re-failed resource attachment cannot leak it.
            let _ = leases.release(services).await;
            return Err(error);
        }
        Ok(leases)
    }

    /// Resolves the session working resource and binds its directory. Every
    /// failure past the credential acquisition releases that credential lease
    /// through the caller's `release`.
    async fn attach_resource(
        &mut self,
        scope: ScopeId,
        services: &HostServices,
        reference: &swallowtail_runtime::WorkingResourceRef,
        policy: &SessionAccessPolicy,
    ) -> Result<(), RuntimeFailure> {
        let resource_service = services.working_resource().cloned().ok_or_else(|| {
            failure(
                "swallowtail.opencode.resource_service_missing",
                "OpenCode session requires a working-resource service",
            )
        })?;
        let access = policy.resource_access().ok_or_else(|| {
            failure(
                "swallowtail.opencode.resource_access_missing",
                "OpenCode working-resource policy requires explicit access",
            )
        })?;
        let lease = resource_service
            .resolve(
                scope,
                reference.clone(),
                access,
                ResourceRepresentation::Filesystem,
            )
            .await?;
        if let Err(error) = validate_session_resource_lease(policy, reference, &lease) {
            let _ = resource_service.release(lease).await;
            return Err(error);
        }
        self.directory = lease
            .filesystem()
            .map(|filesystem| filesystem.as_driver_value().to_owned());
        self.resource = Some(lease);
        Ok(())
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
        CleanupOutcome::merge(resource, credential)
    }
}

#[cfg(test)]
mod access_lease_tests {
    use super::{AccessBinding, AccessLeases};
    use futures_executor::block_on;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use swallowtail_core::{
        CredentialRef, EndpointAudience, ExecutionHostId, InstanceTargetRef, ResourceAccess,
        SessionAccessPolicy,
    };
    use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
    use swallowtail_runtime::{
        CleanupOutcome, CredentialLease, CredentialService, EndpointRef, HostServices,
        NetworkPolicyService, RuntimeFailure, ScopeId, WorkingResourceRef,
    };

    struct TrackingCredential {
        inner: LocalProcessHost,
        releases: Arc<AtomicUsize>,
    }

    impl CredentialService for TrackingCredential {
        fn acquire(
            &self,
            scope: ScopeId,
            reference: CredentialRef,
            audience: EndpointAudience,
        ) -> swallowtail_runtime::BoxFuture<'static, Result<CredentialLease, RuntimeFailure>>
        {
            self.inner.acquire(scope, reference, audience)
        }

        fn release(
            &self,
            lease: CredentialLease,
        ) -> swallowtail_runtime::BoxFuture<'static, CleanupOutcome> {
            let release = CredentialService::release(&self.inner, lease);
            let releases = Arc::clone(&self.releases);
            Box::pin(async move {
                let outcome = release.await;
                releases.fetch_add(1, Ordering::SeqCst);
                outcome
            })
        }
    }

    #[test]
    fn missing_resource_service_releases_the_acquired_credential() {
        let host_id = ExecutionHostId::new("opencode.access.unit").expect("host id is valid");
        let target = InstanceTargetRef::new("opencode.access.unit.endpoint").expect("target");
        let audience = EndpointAudience::new("opencode.access.unit.audience").expect("audience");
        let credential = CredentialRef::new("opencode.access.unit.delegated").expect("credential");
        let host = LocalProcessHost::builder(LocalProcessLimits::default())
            .approve_endpoint(
                EndpointRef::from_instance_target(&target),
                audience.clone(),
                "http://127.0.0.1:1",
            )
            .approve_delegated_credential(credential.clone(), audience.clone())
            .build();
        let releases = Arc::new(AtomicUsize::new(0));
        // The working-resource service is deliberately absent: acquisition must
        // fail after the credential lease is held, and must release it.
        let services = HostServices::new(host_id)
            .with_network(Arc::new(host.clone()) as Arc<dyn NetworkPolicyService>)
            .with_credential(Arc::new(TrackingCredential {
                inner: host,
                releases: Arc::clone(&releases),
            }) as Arc<dyn CredentialService>);
        let resource = WorkingResourceRef::new("opencode.access.unit.workspace").expect("resource");
        let policy = SessionAccessPolicy::ambient_harness(ResourceAccess::Read);
        let result = block_on(AccessLeases::acquire_binding(
            AccessBinding {
                scope: ScopeId::new("opencode.access.unit.scope").expect("scope"),
                endpoint: EndpointRef::from_instance_target(&target),
                audience,
                credential,
                resource: Some((&resource, &policy)),
            },
            &services,
        ));
        let error = match result {
            Ok(_) => panic!("a missing working-resource service must fail acquisition"),
            Err(error) => error,
        };
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.opencode.resource_service_missing"
        );
        assert_eq!(
            releases.load(Ordering::SeqCst),
            1,
            "the acquired credential lease must be released on an early error"
        );
    }
}
