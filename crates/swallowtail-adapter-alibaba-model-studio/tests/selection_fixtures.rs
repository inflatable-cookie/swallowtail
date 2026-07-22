use swallowtail_adapter_alibaba_model_studio::{
    ACCESS_PROFILE_ID, ENDPOINT_AUDIENCE, EXACT_MODEL_ID, REGION, WORKSPACE_ENDPOINT_TEMPLATE,
    alibaba_model_studio_access_profile, alibaba_model_studio_descriptor,
    alibaba_model_studio_instance, alibaba_model_studio_requirements, alibaba_model_studio_route,
    validate_alibaba_model_studio_plan,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, Capability, CredentialMechanism, CredentialState,
    EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
    ExecutionHostId, HostServiceKind, ModelId, ModelRoute, ModelRouteId, ModelRouteRevision,
    PreflightContext, RuntimeReadiness, SupportAuthority, preflight,
};

#[test]
fn descriptor_access_route_and_plan_bind_the_exact_singapore_workspace_subset() {
    let host = host();
    let descriptor = alibaba_model_studio_descriptor();
    let instance = alibaba_model_studio_instance(host.clone());
    let route = alibaba_model_studio_route();
    let access = alibaba_model_studio_access_profile();
    let status = ready_status(access.id().clone());
    let requirements = alibaba_model_studio_requirements(host.clone());
    let plan = preflight(
        &PreflightContext::new(&descriptor, &instance, &access, &status, host_services())
            .with_model_route(&route),
        &requirements,
    )
    .expect("exact selection passes pure preflight");
    validate_alibaba_model_studio_plan(&plan).expect("adapter selection is exact");

    assert_eq!(REGION, "ap-southeast-1");
    assert_eq!(
        WORKSPACE_ENDPOINT_TEMPLATE,
        "https://{WorkspaceId}.ap-southeast-1.maas.aliyuncs.com"
    );
    assert_eq!(access.id().as_str(), ACCESS_PROFILE_ID);
    assert_eq!(access.endpoint_audience().as_str(), ENDPOINT_AUDIENCE);
    assert_eq!(route.model_id().as_str(), EXACT_MODEL_ID);
    assert!(
        !requirements
            .capabilities()
            .any(|requirement| requirement.capability() == Capability::Resume)
    );
}

#[test]
fn aliases_other_regions_and_coding_plan_access_reject_before_effects() {
    let host = host();
    let descriptor = alibaba_model_studio_descriptor();
    let instance = alibaba_model_studio_instance(host.clone());
    let access = alibaba_model_studio_access_profile();
    let status = ready_status(access.id().clone());
    let requirements = alibaba_model_studio_requirements(host);
    let alias = ModelRoute::new(
        ModelRouteId::new("alibaba-model-studio.sg.qwen3.7-plus-alias").expect("route id is valid"),
        ModelRouteRevision::new("fixture-alias").expect("revision is valid"),
        instance.id().clone(),
        ModelId::new("qwen3.7-plus").expect("model is valid"),
        instance.capabilities().clone(),
    );
    let alias_plan = preflight(
        &PreflightContext::new(&descriptor, &instance, &access, &status, host_services())
            .with_model_route(&alias),
        &requirements,
    )
    .expect("generic preflight accepts a structurally valid route");
    assert!(validate_alibaba_model_studio_plan(&alias_plan).is_err());

    for profile in [
        incompatible_access(
            "alibaba-model-studio.beijing.workspace.api-key.payg",
            "model-studio.workspace.cn-beijing",
            EntitlementMetering::PayAsYouGo,
        ),
        incompatible_access(
            "alibaba-model-studio.coding-plan",
            "model-studio.coding-plan",
            EntitlementMetering::SubscriptionAllowance,
        ),
    ] {
        let incompatible_status = ready_status(profile.id().clone());
        assert!(
            preflight(
                &PreflightContext::new(
                    &descriptor,
                    &instance,
                    &profile,
                    &incompatible_status,
                    host_services(),
                )
                .with_model_route(&alibaba_model_studio_route()),
                &requirements,
            )
            .is_err()
        );
    }
}

fn incompatible_access(id: &str, audience: &str, metering: EntitlementMetering) -> AccessProfile {
    AccessProfile::new(
        AccessProfileId::new(id).expect("profile id is valid"),
        CredentialMechanism::ApiKey,
        metering,
        EndpointAudience::new(audience).expect("audience is valid"),
        SupportAuthority::ProviderSupported,
    )
}

fn ready_status(profile_id: AccessProfileId) -> AccessStatus {
    AccessStatus::new(
        profile_id,
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    )
}

fn host() -> ExecutionHostId {
    ExecutionHostId::new("fixture.alibaba-host").expect("host id is valid")
}

fn host_services() -> [HostServiceKind; 5] {
    [
        HostServiceKind::Task,
        HostServiceKind::BlockingWork,
        HostServiceKind::Time,
        HostServiceKind::Network,
        HostServiceKind::Credential,
    ]
}
