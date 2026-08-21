#![allow(dead_code)]

use swallowtail_adapter_claude_agent::{
    CLAUDE_AGENT_ACP_AXIS, CLAUDE_AGENT_ACP_BINARY_PATH_FIELD_ID,
    CLAUDE_AGENT_ACP_ENVIRONMENT_FIELD_ID, ClaudeAgentPreparationInput,
    ClaudeAgentPreparationProbe, ClaudeAgentPreparedIntegration,
    claude_agent_acp_addable_route_descriptor, claude_agent_acp_claim,
    claude_agent_acp_subscription_access_profile, prepare_claude_agent,
};
use swallowtail_core::{
    AccessProfileId, AccessStatus, AdmittedInstanceRecord, ConfigFieldId, ConfigFieldRef,
    ConfiguredInstanceId, ExecutionHostId, InstalledExecutableObservation, InstanceRevision,
    InstanceUpdateObservation, IntegrationFamilyId, InterfaceVersionAxis,
    InvalidInstanceUpdateObservation, ModelId, OverlayMarker,
};
use swallowtail_host_local::MemoryConnectionLifecycleStore;
use swallowtail_runtime::{
    AddableRouteCatalog, ConfiguredProviderInstanceRecord, ConnectionLifecycleStore,
    ConnectionLifecycleStoreFailure, Deadline, DiscoveryCancellation, ExecutableRef, HostServices,
    InstalledExecutableTarget, InstanceAdmissionFailure, InstanceAdmissionRequest,
    ModelPresentationOverlay, ModelPresentationOverlayFailure, PreparationFailure,
    PreparedAccessEvidence, ReadinessRefreshRequest, RequestId, ScopeId, admit_instance,
    apply_stored_model_presentation_overlay, observe_instance_update, refresh_readiness,
};

fn admit_claude_agent_acp(
    services: &HostServices,
    store: &MemoryConnectionLifecycleStore,
    instance_id: ConfiguredInstanceId,
) -> Result<AdmittedInstanceRecord, InstanceAdmissionFailure> {
    let descriptor = claude_agent_acp_addable_route_descriptor(services);
    let route_id = descriptor.id().clone();
    let catalog = AddableRouteCatalog::from_descriptors([descriptor]).expect("catalog assembles");
    admit_instance(
        &catalog,
        store,
        InstanceAdmissionRequest::new(
            instance_id,
            IntegrationFamilyId::new("claude-agent").expect("family id is valid"),
            route_id,
        )
        .with_config_refs([
            (
                ConfigFieldId::new(CLAUDE_AGENT_ACP_BINARY_PATH_FIELD_ID)
                    .expect("config id is valid"),
                ConfigFieldRef::new("claude-agent.acp").expect("config ref is valid"),
            ),
            (
                ConfigFieldId::new(CLAUDE_AGENT_ACP_ENVIRONMENT_FIELD_ID)
                    .expect("config id is valid"),
                ConfigFieldRef::new("claude-agent.work.environment").expect("config ref is valid"),
            ),
        ]),
    )
}

fn refresh_claude_agent_acp(
    store: &MemoryConnectionLifecycleStore,
    instance_id: ConfiguredInstanceId,
    access_status: AccessStatus,
) -> Result<AdmittedInstanceRecord, swallowtail_runtime::ReadinessRefreshFailure> {
    refresh_readiness(
        store,
        ReadinessRefreshRequest::new(instance_id, access_status),
    )
}

async fn prepare_after_admission(
    admitted: &AdmittedInstanceRecord,
    host: ExecutionHostId,
    access_status: AccessStatus,
    deadline: Deadline,
    services: HostServices,
) -> Result<ClaudeAgentPreparedIntegration, PreparationFailure> {
    let profile = claude_agent_acp_subscription_access_profile(access_status.profile_id().clone());
    let input = ClaudeAgentPreparationInput::from_admitted(
        admitted,
        InstanceRevision::new("1").expect("revision is valid"),
        host,
        profile,
        PreparedAccessEvidence::caller_asserted(access_status),
    )?;
    let probe = ClaudeAgentPreparationProbe::new(
        RequestId::new("claude-agent-lifecycle-prepare").expect("request id is valid"),
        ScopeId::new("claude-agent-lifecycle-prepare").expect("scope is valid"),
        deadline,
        DiscoveryCancellation::new(),
    );
    prepare_claude_agent(input, probe, services).await
}

fn observe_claude_agent_update(
    installed: Option<&InstalledExecutableObservation>,
) -> Result<InstanceUpdateObservation, InvalidInstanceUpdateObservation> {
    observe_instance_update(&claude_agent_acp_claim(), installed.cloned())
}

fn store_instance_model_overlay(
    store: &MemoryConnectionLifecycleStore,
    instance_id: ConfiguredInstanceId,
    model_id: ModelId,
) -> Result<(), ConnectionLifecycleStoreFailure> {
    store.put_overlay_marker(
        OverlayMarker::without_provider(instance_id, model_id).with_favourite(true),
    )
}

fn project_overlay(
    store: &MemoryConnectionLifecycleStore,
    record: &ConfiguredProviderInstanceRecord,
) -> Result<ModelPresentationOverlay, ModelPresentationOverlayFailure> {
    apply_stored_model_presentation_overlay(store, record)
}

fn claude_agent_acp_target() -> InstalledExecutableTarget {
    InstalledExecutableTarget::new(
        ExecutableRef::new("claude-agent.acp").expect("executable ref is valid"),
        InterfaceVersionAxis::new(CLAUDE_AGENT_ACP_AXIS).expect("version axis is valid"),
    )
}

fn subscription_profile_id() -> AccessProfileId {
    AccessProfileId::new("claude-agent.work.subscription").expect("access id is valid")
}

fn main() {}
