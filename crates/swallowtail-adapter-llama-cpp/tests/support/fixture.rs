use super::FixtureServer;
use super::services::ThreadServices;
use std::sync::Arc;
use swallowtail_adapter_llama_cpp::llama_cpp_attached_descriptor;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, Capability, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism,
    CredentialState, DriverRole, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, ExecutionLayer, InstanceOwnership, InstancePolicyId,
    InstanceRevision, InstanceTargetRef, ModelId, ModelRoute, ModelRouteId, ModelRouteRevision,
    OperationRequirements, OperationShape, PreflightContext, ProtocolFacadeId, RuntimeReadiness,
    SupportAuthority, preflight,
};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    BlockingWorkService, EndpointRef, HostServices, NetworkPolicyService, ScopedTaskService,
    TimeService,
};

pub struct Fixture {
    pub server: FixtureServer,
    host_id: ExecutionHostId,
    target: InstanceTargetRef,
    audience: EndpointAudience,
    host: LocalProcessHost,
    pub thread: ThreadServices,
}

impl Fixture {
    pub fn new() -> Self {
        Self::with_server(FixtureServer::start())
    }

    pub fn with_host(host_id: &str) -> Self {
        Self::with_server_and_host(FixtureServer::start(), host_id)
    }

    pub fn with_server(server: FixtureServer) -> Self {
        Self::with_server_and_host(server, "host.llama-cpp")
    }

    fn with_server_and_host(server: FixtureServer, host_id: &str) -> Self {
        let host_id = ExecutionHostId::new(host_id).expect("host id is valid");
        let target = InstanceTargetRef::new("llama-cpp-fixture-endpoint").expect("target is valid");
        let audience = EndpointAudience::new("llama.cpp.attached").expect("audience is valid");
        let host = LocalProcessHost::builder(LocalProcessLimits::default())
            .approve_endpoint(
                EndpointRef::from_instance_target(&target),
                audience.clone(),
                server.endpoint(),
            )
            .build();
        Self {
            server,
            host_id,
            target,
            audience,
            host,
            thread: ThreadServices::new(),
        }
    }

    pub fn services(&self) -> HostServices {
        let thread = Arc::new(self.thread.clone());
        HostServices::new(self.host_id.clone())
            .with_task(Arc::clone(&thread) as Arc<dyn ScopedTaskService>)
            .with_blocking_work(Arc::clone(&thread) as Arc<dyn BlockingWorkService>)
            .with_time(thread as Arc<dyn TimeService>)
            .with_network(Arc::new(self.host.clone()) as Arc<dyn NetworkPolicyService>)
    }

    pub fn plan(&self, role: DriverRole) -> swallowtail_core::PreflightPlan {
        let descriptor = llama_cpp_attached_descriptor();
        let access_id = AccessProfileId::new("access.llama-cpp").expect("access id is valid");
        let requirements = capability_requirements(role);
        let capabilities = CapabilityProfile::new(requirements.clone());
        let instance = ConfiguredInstance::new(
            ConfiguredInstanceId::new("llama-cpp.b9910.attached").expect("instance id is valid"),
            InstanceRevision::new("1").expect("revision is valid"),
            descriptor.identity().id().clone(),
            self.host_id.clone(),
            self.target.clone(),
            InstanceOwnership::ExternalAttached,
            access_id.clone(),
            SupportAuthority::IntegrationMaintainerSupported,
            ProtocolFacadeId::new("llama.cpp.openai-chat-completions.b9910")
                .expect("facade is valid"),
            InstancePolicyId::new("attached-text-only").expect("policy is valid"),
            capabilities.clone(),
        );
        let route = ModelRoute::new(
            ModelRouteId::new("llama-cpp-b9910/stories260k").expect("route id is valid"),
            ModelRouteRevision::new("1").expect("revision is valid"),
            instance.id().clone(),
            ModelId::new("swallowtail-fixture-stories260k").expect("model id is valid"),
            capabilities,
        );
        let access = AccessProfile::new(
            access_id.clone(),
            CredentialMechanism::LocalUnauthenticated,
            EntitlementMetering::LocalCompute,
            self.audience.clone(),
            SupportAuthority::IntegrationMaintainerSupported,
        );
        let status = AccessStatus::new(
            access_id.clone(),
            CredentialState::NotRequired,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::IntegrationMaintainerSupported,
        );
        let host_services: Vec<_> = descriptor.required_host_services(role).collect();
        let operation = OperationRequirements::new(
            ExecutionLayer::DirectModelInference,
            OperationShape::StructuredRun,
            role,
            self.host_id.clone(),
            AccessRequirement::new(access_id)
                .with_credential_states([CredentialState::NotRequired])
                .with_entitlement_states([EntitlementState::Available])
                .with_endpoint_authorizations([EndpointAuthorization::Allowed])
                .with_runtime_readiness([RuntimeReadiness::Ready])
                .with_support_authorities([SupportAuthority::IntegrationMaintainerSupported]),
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
