use super::*;
use swallowtail_core::{
    AccessProfile, AccessStatus, AdapterId, AdapterIdentity, AdapterVersion, CapabilityProfile,
    ConfiguredInstance, ConfiguredInstanceId, DriverDescriptor, EndpointAudience,
    EntitlementMetering, InstancePolicyId, InstanceRevision, InstanceTargetRef,
    IntegrationFamilyId, ModelRoute, ModelRouteId, ModelRouteRevision, PreflightContext,
    ProtocolFacadeId, ProviderId, TransportFamilyId, preflight,
};
use swallowtail_runtime::{OperationContent, RequestId, SessionOptions};

fn fixture() -> (
    DriverDescriptor,
    ConfiguredInstance,
    ModelRoute,
    AccessProfile,
    AccessStatus,
    OperationRequirements,
) {
    let host = ExecutionHostId::new("fixture.host.deepseek").unwrap();
    let access_id = AccessProfileId::new("fixture.access.deepseek").unwrap();
    let requirements = deepseek_v4_requirements(host.clone(), access_id.clone());
    let profile = CapabilityProfile::new(requirements.capabilities().cloned());
    let adapter_id = AdapterId::new("swallowtail.deepseek.direct").unwrap();
    let driver = DriverDescriptor::new(
        AdapterIdentity::new(adapter_id.clone(), AdapterVersion::new("0.1.0").unwrap()),
        IntegrationFamilyId::new("deepseek").unwrap(),
        TransportFamilyId::new("openai-chat-http-sse").unwrap(),
    )
    .with_roles([DriverRole::InteractiveSession])
    .with_execution_layers([ExecutionLayer::DirectModelInference])
    .with_operation_shapes([OperationShape::InteractiveSession])
    .with_required_host_services(DriverRole::InteractiveSession, requirements.host_services())
    .with_interface_compatibility(deepseek_facade_claim());
    let instance_id = ConfiguredInstanceId::new("fixture.instance.deepseek").unwrap();
    let instance = ConfiguredInstance::new(
        instance_id.clone(),
        InstanceRevision::new("revision-1").unwrap(),
        adapter_id,
        host,
        InstanceTargetRef::new(DEEPSEEK_ENDPOINT).unwrap(),
        InstanceOwnership::ExternalAttached,
        access_id.clone(),
        SupportAuthority::ProviderSupported,
        ProtocolFacadeId::new(DEEPSEEK_FACADE_REVISION).unwrap(),
        InstancePolicyId::new("deepseek-v4-policy").unwrap(),
        profile.clone(),
    )
    .with_interface_versions([deepseek_facade_binding()]);
    let route = ModelRoute::new(
        ModelRouteId::new("deepseek.v4-pro").unwrap(),
        ModelRouteRevision::new("2026-07-22").unwrap(),
        instance_id,
        ModelId::new(DEEPSEEK_MODEL_ID).unwrap(),
        profile,
    )
    .with_provider_id(ProviderId::new(DEEPSEEK_PROVIDER_ID).unwrap());
    let access = AccessProfile::new(
        access_id.clone(),
        CredentialMechanism::ApiKey,
        EntitlementMetering::PayAsYouGo,
        EndpointAudience::new(DEEPSEEK_AUDIENCE).unwrap(),
        SupportAuthority::ProviderSupported,
    );
    let status = AccessStatus::new(
        access_id,
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    );
    (driver, instance, route, access, status, requirements)
}

#[test]
fn exact_facade_and_request_agree_before_effects() {
    let (driver, instance, route, access, status, requirements) = fixture();
    let plan = preflight(
        &PreflightContext::new(
            &driver,
            &instance,
            &access,
            &status,
            requirements.host_services(),
        )
        .with_model_route(&route),
        &requirements,
    )
    .unwrap();
    let request = OpenDirectContinuationSessionRequest::new(
        RequestId::new("deepseek-open").unwrap(),
        deepseek_v4_config(),
    )
    .with_options(
        SessionOptions::default()
            .with_reasoning_mode(swallowtail_core::ReasoningMode::new("high").unwrap()),
    );
    validate_deepseek_request_plan(&plan, &request).unwrap();

    let wrong =
        OpenDirectContinuationSessionRequest::new(
            RequestId::new("deepseek-wrong").unwrap(),
            deepseek_v4_config(),
        )
        .with_options(SessionOptions::default().with_developer_instructions(
            OperationContent::new("unsupported developer role").unwrap(),
        ));
    assert!(validate_deepseek_request_plan(&plan, &wrong).is_err());
}

#[test]
fn opaque_facade_claim_rejects_every_unqualified_revision() {
    let claim = deepseek_facade_claim();
    assert!(claim.supports(deepseek_facade_binding().version()));
    assert!(!claim.supports(&InterfaceVersion::new("deepseek-openai-chat-2026-07-23").unwrap()));
}
