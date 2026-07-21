struct Fixture {
    host_id: ExecutionHostId,
    target: InstanceTargetRef,
    audience: EndpointAudience,
    credential: CredentialRef,
    resource: WorkingResourceRef,
    host: LocalProcessHost,
    thread: ThreadServices,
}

impl Fixture {
    fn new(endpoint: &str, host_id: &str) -> Self {
        let host_id = ExecutionHostId::new(host_id).expect("host id is valid");
        let target = InstanceTargetRef::new("opencode-fixture-endpoint").expect("target is valid");
        let audience = EndpointAudience::new("opencode-fixture").expect("audience is valid");
        let credential = CredentialRef::new("opencode-delegated").expect("credential is valid");
        let resource = WorkingResourceRef::new("fixture-workspace").expect("resource is valid");
        let host = LocalProcessHost::builder(LocalProcessLimits::default())
            .approve_endpoint(
                EndpointRef::from_instance_target(&target),
                audience.clone(),
                endpoint,
            )
            .approve_delegated_credential(credential.clone(), audience.clone())
            .approve_working_resource(resource.clone(), std::env::temp_dir())
            .build();
        Self {
            host_id,
            target,
            audience,
            credential,
            resource,
            host,
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
            .with_credential(Arc::new(self.host.clone()) as Arc<dyn CredentialService>)
            .with_working_resource(Arc::new(self.host.clone()) as Arc<dyn WorkingResourceService>)
    }

    fn plan(&self, role: DriverRole) -> PreflightPlan {
        let descriptor = opencode_http_descriptor();
        let access_id = AccessProfileId::new("access.opencode").expect("access id is valid");
        let capability = if role == DriverRole::ModelCatalog {
            Capability::ModelCatalog
        } else {
            Capability::InteractiveSession
        };
        let capability_requirements = vec![CapabilityRequirement::new(capability, [])];
        let capabilities = CapabilityProfile::new(capability_requirements.clone());
        let instance = ConfiguredInstance::new(
            ConfiguredInstanceId::new("opencode.attached").expect("instance id is valid"),
            InstanceRevision::new("1").expect("revision is valid"),
            descriptor.identity().id().clone(),
            self.host_id.clone(),
            self.target.clone(),
            InstanceOwnership::ExternalAttached,
            access_id.clone(),
            SupportAuthority::IntegrationMaintainerSupported,
            ProtocolFacadeId::new("opencode-http-1.14.48").expect("facade is valid"),
            InstancePolicyId::new("read-only-deny-first").expect("policy is valid"),
            capabilities.clone(),
        );
        let route = ModelRoute::new(
            ModelRouteId::new("opencode-anthropic-sonnet").expect("route id is valid"),
            ModelRouteRevision::new("1").expect("route revision is valid"),
            instance.id().clone(),
            ModelId::new("claude-sonnet").expect("model id is valid"),
            capabilities,
        )
        .with_provider_id(ProviderId::new("anthropic").expect("provider id is valid"));
        let access = AccessProfile::new(
            access_id.clone(),
            CredentialMechanism::ProviderSpecific(
                ExtensionNamespace::new("opencode/delegated-auth").expect("namespace is valid"),
            ),
            EntitlementMetering::Unknown,
            self.audience.clone(),
            SupportAuthority::IntegrationMaintainerSupported,
        )
        .with_credential_reference(self.credential.clone());
        let status = AccessStatus::new(
            access_id.clone(),
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::IntegrationMaintainerSupported,
        );
        let host_services: Vec<_> = descriptor.required_host_services(role).collect();
        let requirements = OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            OperationShape::InteractiveSession,
            role,
            self.host_id.clone(),
            AccessRequirement::new(access_id)
                .with_credential_states([CredentialState::Ready])
                .with_entitlement_states([EntitlementState::Available])
                .with_endpoint_authorizations([EndpointAuthorization::Allowed])
                .with_runtime_readiness([RuntimeReadiness::Ready])
                .with_support_authorities([SupportAuthority::IntegrationMaintainerSupported]),
        )
        .with_ownership_modes([InstanceOwnership::ExternalAttached])
        .with_host_services(host_services.clone())
        .with_capabilities(capability_requirements)
        .with_session_access_policy(SessionAccessPolicy::ambient_harness(
            swallowtail_core::ResourceAccess::Read,
        ));
        let context =
            PreflightContext::new(&descriptor, &instance, &access, &status, host_services);
        if role == DriverRole::ModelCatalog {
            preflight(&context, &requirements).expect("catalogue preflight succeeds")
        } else {
            preflight(
                &context.with_model_route(&route),
                &requirements.require_model_route(),
            )
            .expect("session preflight succeeds")
        }
    }
}
