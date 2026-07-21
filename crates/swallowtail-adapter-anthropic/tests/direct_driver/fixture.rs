struct Fixture {
    server: FixtureServer,
    host_id: ExecutionHostId,
    target: InstanceTargetRef,
    audience: EndpointAudience,
    credential: CredentialRef,
    host: LocalProcessHost,
    credential_releases: Arc<std::sync::atomic::AtomicUsize>,
    thread: ThreadServices,
}

impl Fixture {
    fn new() -> Self {
        Self::with_stream(StreamFixture::Success)
    }

    fn with_stream(stream: StreamFixture) -> Self {
        let server = FixtureServer::start_with(stream);
        let host_id = ExecutionHostId::new("host.anthropic").expect("host id is valid");
        let target = InstanceTargetRef::new("anthropic-fixture-endpoint").expect("target is valid");
        let audience = EndpointAudience::new("api.anthropic.com").expect("audience is valid");
        let credential = CredentialRef::new("anthropic-fixture-key").expect("credential is valid");
        let host = LocalProcessHost::builder(LocalProcessLimits::default())
            .approve_endpoint(
                EndpointRef::from_instance_target(&target),
                audience.clone(),
                server.endpoint(),
            )
            .approve_secret_credential(
                credential.clone(),
                audience.clone(),
                b"fixture-secret".to_vec(),
            )
            .build();
        let credential_releases = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        Self {
            server,
            host_id,
            target,
            audience,
            credential,
            host,
            credential_releases,
            thread: ThreadServices::new(),
        }
    }

    fn services(&self) -> HostServices {
        let thread = Arc::new(self.thread.clone());
        HostServices::new(self.host_id.clone())
            .with_task(Arc::clone(&thread) as Arc<dyn ScopedTaskService>)
            .with_blocking_work(Arc::clone(&thread) as Arc<dyn BlockingWorkService>)
            .with_time(thread as Arc<dyn TimeService>)
            .with_network(Arc::new(self.host.clone()) as Arc<dyn NetworkPolicyService>)
            .with_credential(Arc::new(TrackingCredential {
                inner: self.host.clone(),
                releases: Arc::clone(&self.credential_releases),
            }) as Arc<dyn CredentialService>)
    }

    fn credential_releases(&self) -> usize {
        self.credential_releases
            .load(std::sync::atomic::Ordering::SeqCst)
    }

    fn plan(&self, role: DriverRole) -> swallowtail_core::PreflightPlan {
        let descriptor = anthropic_direct_descriptor();
        let access_id = AccessProfileId::new("access.anthropic").expect("access id is valid");
        let requirements = capability_requirements(role);
        let capabilities = CapabilityProfile::new(requirements.clone());
        let instance = ConfiguredInstance::new(
            ConfiguredInstanceId::new("anthropic.public").expect("instance id is valid"),
            InstanceRevision::new("1").expect("revision is valid"),
            descriptor.identity().id().clone(),
            self.host_id.clone(),
            self.target.clone(),
            InstanceOwnership::ExternalAttached,
            access_id.clone(),
            SupportAuthority::ProviderSupported,
            ProtocolFacadeId::new("anthropic-2023-06-01").expect("facade is valid"),
            InstancePolicyId::new("public-api-key").expect("policy is valid"),
            capabilities.clone(),
        );
        let route = ModelRoute::new(
            ModelRouteId::new("anthropic-fixture-primary").expect("route id is valid"),
            ModelRouteRevision::new("1").expect("revision is valid"),
            instance.id().clone(),
            ModelId::new("claude-fixture-primary").expect("model id is valid"),
            capabilities,
        )
        .with_provider_id(ProviderId::new("anthropic").expect("provider id is valid"));
        let access = AccessProfile::new(
            access_id.clone(),
            CredentialMechanism::ApiKey,
            EntitlementMetering::PayAsYouGo,
            self.audience.clone(),
            SupportAuthority::ProviderSupported,
        )
        .with_credential_reference(self.credential.clone());
        let status = AccessStatus::new(
            access_id.clone(),
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        );
        let host_services: Vec<_> = descriptor.required_host_services(role).collect();
        let operation = OperationRequirements::new(
            ExecutionLayer::DirectModelInference,
            OperationShape::StructuredRun,
            role,
            self.host_id.clone(),
            AccessRequirement::new(access_id)
                .with_credential_states([CredentialState::Ready])
                .with_entitlement_states([EntitlementState::Available])
                .with_endpoint_authorizations([EndpointAuthorization::Allowed])
                .with_runtime_readiness([RuntimeReadiness::Ready])
                .with_support_authorities([SupportAuthority::ProviderSupported]),
        )
        .with_ownership_modes([InstanceOwnership::ExternalAttached])
        .with_host_services(host_services.clone())
        .with_capabilities(requirements);
        let context =
            PreflightContext::new(&descriptor, &instance, &access, &status, host_services);
        if role == DriverRole::ModelCatalog {
            preflight(&context, &operation).expect("catalogue preflight succeeds")
        } else {
            preflight(
                &context.with_model_route(&route),
                &operation.require_model_route(),
            )
            .expect("run preflight succeeds")
        }
    }
}

struct TrackingCredential {
    inner: LocalProcessHost,
    releases: Arc<std::sync::atomic::AtomicUsize>,
}

impl CredentialService for TrackingCredential {
    fn acquire(
        &self,
        scope: swallowtail_runtime::ScopeId,
        reference: CredentialRef,
        audience: EndpointAudience,
    ) -> swallowtail_runtime::BoxFuture<
        'static,
        Result<swallowtail_runtime::CredentialLease, swallowtail_runtime::RuntimeFailure>,
    > {
        self.inner.acquire(scope, reference, audience)
    }

    fn release(
        &self,
        lease: swallowtail_runtime::CredentialLease,
    ) -> swallowtail_runtime::BoxFuture<'static, swallowtail_runtime::CleanupOutcome> {
        let release = self.inner.release(lease);
        let releases = Arc::clone(&self.releases);
        Box::pin(async move {
            let outcome = release.await;
            releases.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            outcome
        })
    }
}

fn capability_requirements(role: DriverRole) -> Vec<CapabilityRequirement> {
    if role == DriverRole::ModelCatalog {
        vec![CapabilityRequirement::new(Capability::ModelCatalog, [])]
    } else {
        vec![
            CapabilityRequirement::new(Capability::StructuredRun, []),
            CapabilityRequirement::new(Capability::StreamingEvents, []),
            CapabilityRequirement::new(Capability::UsageReporting, []),
            CapabilityRequirement::new(Capability::OutputTokenLimit, []),
        ]
    }
}
