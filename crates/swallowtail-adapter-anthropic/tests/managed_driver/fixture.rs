struct Fixture {
    server: ManagedFixtureServer,
    host_id: ExecutionHostId,
    instance_id: ConfiguredInstanceId,
    target: InstanceTargetRef,
    audience: EndpointAudience,
    credential: CredentialRef,
    host: LocalProcessHost,
    credential_releases: Arc<std::sync::atomic::AtomicUsize>,
    thread: ThreadServices,
}

impl Fixture {
    fn new() -> Self {
        Self::with_stream(ManagedStreamFixture::Success)
    }

    fn with_stream(stream: ManagedStreamFixture) -> Self {
        Self::with_identity(
            stream,
            ExecutionHostId::new("host.anthropic-managed").expect("host id is valid"),
            ConfiguredInstanceId::new("anthropic.managed").expect("instance id is valid"),
            InstanceTargetRef::new("anthropic-managed-fixture").expect("target is valid"),
        )
    }

    fn for_topology(topology: ExecutionTopologyFixture) -> Self {
        Self::with_identity(
            ManagedStreamFixture::Success,
            topology.execution_host_id().clone(),
            topology.configured_instance_id().clone(),
            topology.instance_target().clone(),
        )
    }

    fn with_identity(
        stream: ManagedStreamFixture,
        host_id: ExecutionHostId,
        instance_id: ConfiguredInstanceId,
        target: InstanceTargetRef,
    ) -> Self {
        let server = ManagedFixtureServer::start_with(stream);
        let audience = EndpointAudience::new("api.anthropic.com").expect("audience is valid");
        let credential =
            CredentialRef::new("anthropic-managed-key").expect("credential is valid");
        let host = LocalProcessHost::builder(LocalProcessLimits::default())
            .approve_endpoint(
                EndpointRef::from_instance_target(&target),
                audience.clone(),
                server.endpoint(),
            )
            .approve_secret_credential(
                credential.clone(),
                audience.clone(),
                b"managed-fixture-secret".to_vec(),
            )
            .build();
        Self {
            server,
            host_id,
            instance_id,
            target,
            audience,
            credential,
            host,
            credential_releases: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
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

    fn deadline(&self) -> Deadline {
        self.deadline_after(5_000)
    }

    fn deadline_after(&self, milliseconds: u64) -> Deadline {
        Deadline::at(MonotonicInstant::from_ticks(
            self.thread.now().ticks() + milliseconds,
        ))
    }

    fn request(&self, id: &str) -> StructuredRunRequest {
        StructuredRunRequest::new(
            RequestId::new(id).expect("request id is valid"),
            OperationContent::new("Return the fixture summary.").expect("content is valid"),
            OperationPolicy::offline()
                .with_provider_retention(ProviderRetentionPolicy::DurableAllowed)
                .with_provider_recovery(ProviderRecoveryPolicy::ManagedAllowed)
                .with_stream_reattachment(StreamReattachmentPolicy::Bounded(
                    NonZeroU32::new(1).expect("one is non-zero"),
                )),
        )
        .with_deadline(self.deadline())
    }

    fn credential_releases(&self) -> usize {
        self.credential_releases
            .load(std::sync::atomic::Ordering::SeqCst)
    }

    fn plan(&self) -> swallowtail_core::PreflightPlan {
        let descriptor = anthropic_managed_agent_descriptor();
        let access_id = AccessProfileId::new("access.anthropic-managed").expect("access id valid");
        let requirements = managed_capabilities();
        let capabilities = CapabilityProfile::new(requirements.clone());
        let instance = ConfiguredInstance::new(
            self.instance_id.clone(),
            InstanceRevision::new("1").expect("revision is valid"),
            descriptor.identity().id().clone(),
            self.host_id.clone(),
            self.target.clone(),
            InstanceOwnership::ExternalAttached,
            access_id.clone(),
            SupportAuthority::ProviderSupported,
            ProtocolFacadeId::new("managed-agents-2026-04-01").expect("facade is valid"),
            InstancePolicyId::new("managed-resource-free").expect("policy is valid"),
            capabilities.clone(),
        )
        .with_provider_agent(ProviderAgentBinding::new(
            ProviderAgentId::new("agent_fixture").expect("agent id is valid"),
            ProviderAgentVersion::new("7").expect("agent version is valid"),
        ));
        let route = ModelRoute::new(
            ModelRouteId::new("anthropic-managed-fixture").expect("route id is valid"),
            ModelRouteRevision::new("1").expect("route revision is valid"),
            instance.id().clone(),
            ModelId::new("claude-fixture-model").expect("model id is valid"),
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
        let host_services: Vec<_> = descriptor
            .required_host_services(DriverRole::StructuredRun)
            .collect();
        let operation = OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            OperationShape::StructuredRun,
            DriverRole::StructuredRun,
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
        .with_capabilities(requirements)
        .require_model_route();
        preflight(
            &PreflightContext::new(&descriptor, &instance, &access, &status, host_services)
                .with_model_route(&route),
            &operation,
        )
        .expect("managed run preflight succeeds")
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
    ) -> swallowtail_runtime::BoxFuture<'static, CleanupOutcome> {
        let release = self.inner.release(lease);
        let releases = Arc::clone(&self.releases);
        Box::pin(async move {
            let outcome = release.await;
            releases.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            outcome
        })
    }
}

fn managed_capabilities() -> Vec<CapabilityRequirement> {
    vec![
        CapabilityRequirement::new(Capability::StructuredRun, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(
            Capability::ToolCalls,
            [
                CapabilityConstraint::ToolSchemaDialect("json-schema-2020-12".to_owned()),
                CapabilityConstraint::ToolMaximumSchemaBytes(16_384),
                CapabilityConstraint::ToolMaximumCount(8),
            ],
        ),
        CapabilityRequirement::new(Capability::UsageReporting, []),
        CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
        CapabilityRequirement::new(Capability::ProviderManagedRecovery, []),
        CapabilityRequirement::new(
            Capability::OwnedRemoteResourceDeletion,
            [
                CapabilityConstraint::OwnedRemoteResource(OwnedRemoteResourceKind::Environment),
                CapabilityConstraint::OwnedRemoteResource(OwnedRemoteResourceKind::Session),
            ],
        ),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                swallowtail_core::CancellationScope::StructuredRun,
            )],
        ),
        CapabilityRequirement::new(
            Capability::StreamReattachment,
            [CapabilityConstraint::ReattachmentMaximumCount(1)],
        ),
    ]
}
