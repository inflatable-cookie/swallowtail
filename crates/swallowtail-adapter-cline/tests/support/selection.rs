pub struct FixtureSelection {
    pub plan: PreflightPlan,
    pub resource: WorkingResourceRef,
}

pub fn selection(host: ExecutionHostId) -> FixtureSelection {
    selection_with_access(host, ResourceAccess::Read)
}

#[allow(dead_code)]
pub fn plan_selection(host: ExecutionHostId) -> FixtureSelection {
    selection_with_mode(host, ResourceAccess::Read, true)
}

pub fn selection_with_access(
    host: ExecutionHostId,
    resource_access: ResourceAccess,
) -> FixtureSelection {
    selection_with_mode(host, resource_access, false)
}

fn selection_with_mode(
    host: ExecutionHostId,
    resource_access: ResourceAccess,
    plan_mode: bool,
) -> FixtureSelection {
    let descriptor = cline_acp_descriptor();
    let access_id = AccessProfileId::new("cline.fixture.local-account").expect("valid access id");
    let instance_id = ConfiguredInstanceId::new("cline.fixture.instance").expect("valid instance");
    let mut capability_requirements = vec![
        CapabilityRequirement::new(Capability::InteractiveSession, []),
        CapabilityRequirement::new(Capability::StreamingEvents, []),
        CapabilityRequirement::new(
            Capability::Interruption,
            [CapabilityConstraint::CancellationScope(
                swallowtail_core::CancellationScope::ActiveTurn,
            )],
        ),
        CapabilityRequirement::new(
            Capability::WorkingResource,
            [
                CapabilityConstraint::ResourceAccess(resource_access),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        ),
    ];
    if plan_mode {
        capability_requirements.push(CapabilityRequirement::new(
            Capability::HarnessModeSelection,
            [CapabilityConstraint::HarnessMode(
                swallowtail_core::HarnessMode::Plan,
            )],
        ));
    }
    let capabilities = CapabilityProfile::new(capability_requirements);
    let version_binding = cline_package_binding("3.0.55").expect("fixture version is valid");
    let instance = ConfiguredInstance::new(
        instance_id.clone(),
        InstanceRevision::new("fixture-revision").expect("valid revision"),
        AdapterId::new("swallowtail.cline.acp").expect("valid adapter"),
        host.clone(),
        InstanceTargetRef::new("cline.fixture.executable").expect("valid target"),
        InstanceOwnership::HostOwnedEphemeral,
        access_id.clone(),
        SupportAuthority::ProviderSupported,
        ProtocolFacadeId::new("acp-v1").expect("valid facade"),
        InstancePolicyId::new("cline.fixture.isolated").expect("valid policy"),
        capabilities.clone(),
    )
    .with_interface_versions([version_binding.clone()])
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let access = cline_local_account_access_profile(access_id.clone());
    let status = AccessStatus::new(
        access_id.clone(),
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    );
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::InteractiveSession,
        DriverRole::InteractiveSession,
        host,
        AccessRequirement::new(access_id)
            .with_credential_states([CredentialState::NotRequired])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::ProviderSupported]),
    )
    .with_ownership_modes([InstanceOwnership::HostOwnedEphemeral])
    .with_host_services([
        swallowtail_core::HostServiceKind::Task,
        swallowtail_core::HostServiceKind::Process,
        swallowtail_core::HostServiceKind::WorkingResource,
    ])
    .with_capabilities(capabilities.iter().map(|(capability, constraints)| {
        CapabilityRequirement::new(capability, constraints.iter().cloned())
    }))
    .with_interface_versions([version_binding])
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
    .with_session_access_policy(SessionAccessPolicy::ambient_harness(resource_access))
    .with_session_provider_state_policy(SessionProviderStatePolicy::Prohibited);
    let context = PreflightContext::new(
        &descriptor,
        &instance,
        &access,
        &status,
        [
            swallowtail_core::HostServiceKind::Task,
            swallowtail_core::HostServiceKind::Process,
            swallowtail_core::HostServiceKind::WorkingResource,
        ],
    );
    let plan = preflight(&context, &requirements).expect("Cline fixture preflight succeeds");
    FixtureSelection {
        plan,
        resource: WorkingResourceRef::new("cline.fixture.workspace").expect("valid resource"),
    }
}
