struct Fixture {
    host_id: ExecutionHostId,
    target: InstanceTargetRef,
    audience: EndpointAudience,
    credential: CredentialRef,
    host: LocalProcessHost,
    thread: ThreadServices,
    releases: Arc<AtomicUsize>,
}

impl Fixture {
    fn new(host: &str) -> Self {
        let host_id = ExecutionHostId::new(host).expect("host id is valid");
        let target = InstanceTargetRef::new("bedrock-runtime-endpoint").expect("target is valid");
        let audience = EndpointAudience::new("bedrock-runtime.amazonaws.com").expect("audience is valid");
        let credential = CredentialRef::new("aws-workload-identity").expect("credential is valid");
        let local = LocalProcessHost::builder(LocalProcessLimits::default())
            .approve_endpoint(EndpointRef::from_instance_target(&target), audience.clone(), "https://bedrock-runtime.fixture")
            .approve_delegated_credential(credential.clone(), audience.clone())
            .build();
        Self {
            host_id, target, audience, credential, host: local,
            thread: ThreadServices::new(), releases: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn binding(&self) -> BedrockDriverBinding {
        BedrockDriverBinding::new(
            ConfiguredInstanceId::new("bedrock.fixture").expect("instance is valid"),
            AccessProfileId::new("access.aws").expect("access is valid"),
            self.credential.clone(), self.host_id.clone(),
            BedrockRegion::new("eu-west-2").expect("region is valid"),
            BedrockCredentialProvider::new(Credentials::new("fixture-access-key", "fixture-secret-key", None, None, "fixture")),
        )
    }

    fn services(&self) -> HostServices {
        let thread = Arc::new(self.thread.clone());
        HostServices::new(self.host_id.clone())
            .with_task(Arc::clone(&thread) as Arc<dyn ScopedTaskService>)
            .with_blocking_work(Arc::clone(&thread) as Arc<dyn BlockingWorkService>)
            .with_time(thread as Arc<dyn TimeService>)
            .with_network(Arc::new(self.host.clone()) as Arc<dyn NetworkPolicyService>)
            .with_credential(Arc::new(TrackingCredential { host: self.host.clone(), releases: Arc::clone(&self.releases) }) as Arc<dyn CredentialService>)
    }

    fn plan(&self) -> PreflightPlan {
        let descriptor = bedrock_direct_descriptor();
        let access_id = AccessProfileId::new("access.aws").expect("access is valid");
        let requirements = vec![
            CapabilityRequirement::new(Capability::StructuredRun, []),
            CapabilityRequirement::new(Capability::StreamingEvents, []),
            CapabilityRequirement::new(Capability::UsageReporting, []),
            CapabilityRequirement::new(Capability::OutputTokenLimit, []),
        ];
        let capabilities = CapabilityProfile::new(requirements.clone());
        let instance = ConfiguredInstance::new(
            ConfiguredInstanceId::new("bedrock.fixture").expect("instance is valid"),
            InstanceRevision::new("1").expect("revision is valid"), descriptor.identity().id().clone(),
            self.host_id.clone(), self.target.clone(), InstanceOwnership::ExternalAttached,
            access_id.clone(), SupportAuthority::ProviderSupported,
            ProtocolFacadeId::new("bedrock-converse-stream").expect("facade is valid"),
            InstancePolicyId::new("aws-delegated").expect("policy is valid"), capabilities.clone(),
        );
        let route = ModelRoute::new(
            ModelRouteId::new("bedrock-fixture-route").expect("route is valid"),
            ModelRouteRevision::new("1").expect("revision is valid"), instance.id().clone(),
            ModelId::new("anthropic.claude-fixture-v1:0").expect("model is valid"), capabilities,
        ).with_provider_id(ProviderId::new("anthropic").expect("provider is valid"));
        let access = AccessProfile::new(
            access_id.clone(), CredentialMechanism::CloudProviderIdentity,
            EntitlementMetering::CloudAccountBilling, self.audience.clone(), SupportAuthority::ProviderSupported,
        ).with_credential_reference(self.credential.clone());
        let status = AccessStatus::new(
            access_id.clone(), CredentialState::Ready, EntitlementState::Available,
            EndpointAuthorization::Allowed, RuntimeReadiness::Ready, SupportAuthority::ProviderSupported,
        );
        let services: Vec<_> = descriptor.required_host_services(DriverRole::StructuredRun).collect();
        let operation = OperationRequirements::new(
            ExecutionLayer::DirectModelInference, OperationShape::StructuredRun,
            DriverRole::StructuredRun, self.host_id.clone(),
            AccessRequirement::new(access_id)
                .with_credential_states([CredentialState::Ready])
                .with_entitlement_states([EntitlementState::Available])
                .with_endpoint_authorizations([EndpointAuthorization::Allowed])
                .with_runtime_readiness([RuntimeReadiness::Ready])
                .with_support_authorities([SupportAuthority::ProviderSupported]),
        ).with_ownership_modes([InstanceOwnership::ExternalAttached])
            .with_host_services(services.clone()).with_capabilities(requirements).require_model_route();
        preflight(
            &PreflightContext::new(&descriptor, &instance, &access, &status, services).with_model_route(&route),
            &operation,
        ).expect("preflight succeeds")
    }

    fn request(&self, id: &str) -> StructuredRunRequest {
        StructuredRunRequest::new(
            RequestId::new(id).expect("request is valid"),
            OperationContent::new("fixture prompt").expect("content is valid"),
            swallowtail_runtime::OperationPolicy::offline(),
        ).with_maximum_output_tokens(NonZeroU64::new(64).expect("limit is nonzero"))
    }
}
