use super::{FixtureServer, ScriptedOwnedServices, services::ThreadServices};
use std::sync::Arc;
use swallowtail_adapter_llama_cpp::llama_cpp_owned_descriptor;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, CancellationScope, Capability,
    CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, CredentialMechanism, CredentialState, EndpointAudience,
    EndpointAuthorization, EntitlementMetering, EntitlementState, ExecutionHostId, ExecutionLayer,
    InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef, ModelArtifactBinding,
    ModelArtifactDescriptor, ModelArtifactDigest, ModelArtifactFormat, ModelArtifactId,
    ModelArtifactRef, ModelArtifactRevision, ModelId, ModelRoute, ModelRouteId, ModelRouteRevision,
    OperationRequirements, OperationShape, PreflightContext, ProtocolFacadeId, RuntimeReadiness,
    SupportAuthority, preflight,
};
use swallowtail_runtime::{
    BlockingWorkService, HostServices, ModelArtifactService, NetworkPolicyService, ProcessService,
    ScopedTaskService, ServingEndpointService, TimeService,
};

pub struct OwnedFixture {
    pub server: FixtureServer,
    pub owned: Arc<ScriptedOwnedServices>,
    thread: Arc<ThreadServices>,
    host_id: ExecutionHostId,
    audience: EndpointAudience,
    artifact: ModelArtifactBinding,
}

impl OwnedFixture {
    pub fn new(server: FixtureServer, owned: ScriptedOwnedServices) -> Self {
        Self::for_host(
            server,
            owned,
            ExecutionHostId::new("host.llama-cpp-owned").expect("host id is valid"),
        )
    }

    pub fn for_host(
        server: FixtureServer,
        owned: ScriptedOwnedServices,
        host_id: ExecutionHostId,
    ) -> Self {
        Self {
            server,
            owned: Arc::new(owned),
            thread: Arc::new(ThreadServices::new()),
            host_id,
            audience: EndpointAudience::new("llama.cpp.owned-loopback").expect("audience is valid"),
            artifact: artifact(),
        }
    }

    pub fn services(&self) -> HostServices {
        HostServices::new(self.host_id.clone())
            .with_task(Arc::clone(&self.thread) as Arc<dyn ScopedTaskService>)
            .with_blocking_work(Arc::clone(&self.thread) as Arc<dyn BlockingWorkService>)
            .with_time(Arc::clone(&self.owned) as Arc<dyn TimeService>)
            .with_process(Arc::clone(&self.owned) as Arc<dyn ProcessService>)
            .with_network(Arc::clone(&self.owned) as Arc<dyn NetworkPolicyService>)
            .with_model_artifact(Arc::clone(&self.owned) as Arc<dyn ModelArtifactService>)
            .with_serving_endpoint(Arc::clone(&self.owned) as Arc<dyn ServingEndpointService>)
    }

    pub fn plan(&self) -> swallowtail_core::PreflightPlan {
        let descriptor = llama_cpp_owned_descriptor();
        let access_id = AccessProfileId::new("access.llama-cpp-owned").expect("access id is valid");
        let requirements = capabilities();
        let profile = CapabilityProfile::new(requirements.clone());
        let instance = ConfiguredInstance::new(
            ConfiguredInstanceId::new("llama-cpp.b10069.owned").expect("instance id is valid"),
            InstanceRevision::new("1").expect("revision is valid"),
            descriptor.identity().id().clone(),
            self.host_id.clone(),
            InstanceTargetRef::new("llama-server.b10069").expect("target is valid"),
            InstanceOwnership::HostOwnedEphemeral,
            access_id.clone(),
            SupportAuthority::IntegrationMaintainerSupported,
            ProtocolFacadeId::new("llama.cpp.openai-chat-completions.b10069")
                .expect("facade is valid"),
            InstancePolicyId::new("owned-offline-loopback").expect("policy is valid"),
            profile.clone(),
        );
        let route = ModelRoute::new(
            ModelRouteId::new("llama-cpp-b10069/stories260k").expect("route id is valid"),
            ModelRouteRevision::new("1").expect("revision is valid"),
            instance.id().clone(),
            ModelId::new("swallowtail-fixture-stories260k").expect("model id is valid"),
            profile,
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
        let services: Vec<_> = descriptor
            .required_host_services(swallowtail_core::DriverRole::ServingInstanceLifecycle)
            .collect();
        let operation = OperationRequirements::new(
            ExecutionLayer::DirectModelInference,
            OperationShape::StructuredRun,
            swallowtail_core::DriverRole::ServingInstanceLifecycle,
            self.host_id.clone(),
            AccessRequirement::new(access_id)
                .with_credential_states([CredentialState::NotRequired])
                .with_entitlement_states([EntitlementState::Available])
                .with_endpoint_authorizations([EndpointAuthorization::Allowed])
                .with_runtime_readiness([RuntimeReadiness::Ready])
                .with_support_authorities([SupportAuthority::IntegrationMaintainerSupported]),
        )
        .with_ownership_modes([InstanceOwnership::HostOwnedEphemeral])
        .with_host_services(services.iter().copied())
        .with_capabilities(requirements)
        .require_model_route();
        preflight(
            &PreflightContext::new(&descriptor, &instance, &access, &status, services)
                .with_model_route(&route)
                .with_model_artifact(&self.artifact),
            &operation,
        )
        .expect("owned preflight succeeds")
    }

    pub fn artifact(&self) -> ModelArtifactBinding {
        self.artifact.clone()
    }
}

fn capabilities() -> Vec<CapabilityRequirement> {
    vec![
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                CancellationScope::OwnedServingInstance,
            )],
        ),
    ]
}

fn artifact() -> ModelArtifactBinding {
    ModelArtifactBinding::new(
        ModelArtifactRef::new("artifact.stories260k").expect("reference is valid"),
        ModelArtifactDescriptor::new(
            ModelArtifactId::new("stories260k-q8").expect("id is valid"),
            ModelArtifactFormat::new("gguf").expect("format is valid"),
            ModelArtifactRevision::new("fixture-1").expect("revision is valid"),
            ModelArtifactDigest::new(
                "sha256:3a6eb0790f39ac87c94f3856b2dd2c5d110e6811602261a9a923d3bb23adc8b7",
            )
            .expect("digest is valid"),
        ),
    )
}
