use super::FixtureServer;
use super::services::ThreadServices;
use std::sync::Arc;
use swallowtail_adapter_ollama::{
    OLLAMA_NATIVE_FACADE, ollama_native_descriptor, ollama_runtime_binding,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, AttachedModelObservation,
    AttachedModelObservationScope, AttachedModelTag, AttachedRuntimeRequirements,
    AttachedRuntimeResidency, Capability, CapabilityProfile, CapabilityRequirement,
    CatalogTimestamp, ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism,
    CredentialState, DriverRole, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, ExecutionLayer, InstanceOwnership, InstancePolicyId,
    InstanceRevision, InstanceTargetRef, ModelId, ModelManifestDigest, ModelRoute, ModelRouteId,
    ModelRouteRevision, OperationRequirements, OperationShape, PreflightContext, ProtocolFacadeId,
    RuntimeReadiness, SupportAuthority, preflight,
};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    BlockingWorkService, EndpointRef, HostServices, NetworkPolicyService, ScopedTaskService,
    TimeService,
};

const MODEL: &str = "fixture-model:8b";
const DIGEST: &str = "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

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
        Self::with_server_and_host(server, "host.ollama")
    }

    fn with_server_and_host(server: FixtureServer, host_id: &str) -> Self {
        let host_id = ExecutionHostId::new(host_id).expect("host id is valid");
        let target = InstanceTargetRef::new("ollama-fixture-endpoint").expect("target is valid");
        let audience = EndpointAudience::new("ollama.attached").expect("audience is valid");
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
        self.services_with_time(self.thread.clone())
    }

    pub fn services_with_join_failure(&self) -> HostServices {
        self.services_with_time(self.thread.clone().with_join_failure())
    }

    fn services_with_time(&self, thread: ThreadServices) -> HostServices {
        let thread = Arc::new(thread);
        HostServices::new(self.host_id.clone())
            .with_task(Arc::clone(&thread) as Arc<dyn ScopedTaskService>)
            .with_blocking_work(Arc::clone(&thread) as Arc<dyn BlockingWorkService>)
            .with_time(thread as Arc<dyn TimeService>)
            .with_network(Arc::new(self.host.clone()) as Arc<dyn NetworkPolicyService>)
    }

    pub fn plan(&self, role: DriverRole) -> swallowtail_core::PreflightPlan {
        let descriptor = ollama_native_descriptor();
        let access_id = AccessProfileId::new("access.ollama").expect("access id is valid");
        let version = ollama_runtime_binding("0.30.0");
        let tag = AttachedModelTag::new(MODEL).expect("tag is valid");
        let digest = ModelManifestDigest::new(DIGEST).expect("digest is valid");
        let capability_requirements = capability_requirements(role);
        let capabilities = CapabilityProfile::new(capability_requirements.clone());
        let instance = ConfiguredInstance::new(
            ConfiguredInstanceId::new("ollama.0.30.0.attached").expect("instance id is valid"),
            InstanceRevision::new("1").expect("revision is valid"),
            descriptor.identity().id().clone(),
            self.host_id.clone(),
            self.target.clone(),
            InstanceOwnership::ExternalAttached,
            access_id.clone(),
            SupportAuthority::IntegrationMaintainerSupported,
            ProtocolFacadeId::new(OLLAMA_NATIVE_FACADE).expect("facade is valid"),
            InstancePolicyId::new("attached-text-only").expect("policy is valid"),
            capabilities.clone(),
        )
        .with_interface_versions([version.clone()]);
        let route = ModelRoute::new(
            ModelRouteId::new("ollama/fixture-model").expect("route id is valid"),
            ModelRouteRevision::new("1").expect("revision is valid"),
            instance.id().clone(),
            ModelId::new(MODEL).expect("model id is valid"),
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
        .with_capabilities(capability_requirements)
        .with_interface_versions([version.clone()])
        .with_attached_runtime(AttachedRuntimeRequirements::new(
            version.clone(),
            route.model_id().clone(),
            tag.clone(),
            digest.clone(),
            AttachedRuntimeResidency::RuntimeManaged,
        ))
        .require_model_route();
        let observation = AttachedModelObservation::new(
            AttachedModelObservationScope::SelectedModelDetail,
            instance.id().clone(),
            self.host_id.clone(),
            version,
            CatalogTimestamp::new(1_700_000_000, 0).expect("timestamp is valid"),
            tag,
        )
        .with_manifest_digest(digest);
        preflight(
            &PreflightContext::new(&descriptor, &instance, &access, &status, host_services)
                .with_model_route(&route)
                .with_attached_model_observation(&observation),
            &operation,
        )
        .expect("Ollama preflight succeeds")
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
