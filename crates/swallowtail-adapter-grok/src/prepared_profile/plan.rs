fn instance_with_capabilities(
    prepared: &GrokPreparedIntegration,
    capabilities: CapabilityProfile,
) -> ConfiguredInstance {
    swallowtail_runtime::instance_with_capabilities(prepared.instance(), capabilities)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
}

fn operation_requirements(
    prepared: &GrokPreparedIntegration,
    shape: OperationShape,
    role: swallowtail_core::DriverRole,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
    host_services: impl IntoIterator<Item = HostServiceKind>,
) -> OperationRequirements {
    OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        shape,
        role,
        prepared.instance().execution_host_id().clone(),
        AccessRequirement::new(prepared.access_profile().id().clone())
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([prepared.access_profile().support_authority()]),
    )
    .with_ownership_modes([prepared.instance().ownership()])
    .with_host_services(host_services)
    .with_capabilities(capabilities)
    .with_interface_versions([prepared.observation().version().clone()])
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient)
    .require_model_route()
}

fn session_requirements(
    prepared: &GrokPreparedIntegration,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
) -> OperationRequirements {
    operation_requirements(
        prepared,
        OperationShape::InteractiveSession,
        swallowtail_core::DriverRole::InteractiveSession,
        capabilities,
        [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::Credential,
            HostServiceKind::WorkingResource,
            HostServiceKind::WorkingResourceIo,
        ],
    )
    .with_session_access_policy(SessionAccessPolicy::ambient_harness(
        ResourceAccess::ReadWrite,
    ))
    .with_session_provider_state_policy(SessionProviderStatePolicy::DurableProviderSessionPreserved)
}

fn run_requirements(
    prepared: &GrokPreparedIntegration,
    capabilities: impl IntoIterator<Item = CapabilityRequirement>,
) -> OperationRequirements {
    operation_requirements(
        prepared,
        OperationShape::StructuredRun,
        swallowtail_core::DriverRole::StructuredRun,
        capabilities,
        [
            HostServiceKind::Task,
            HostServiceKind::Time,
            HostServiceKind::Process,
            HostServiceKind::Credential,
            HostServiceKind::WorkingResource,
            HostServiceKind::WorkingResourceIo,
        ],
    )
}

fn build_plan(
    prepared: &GrokPreparedIntegration,
    instance: &ConfiguredInstance,
    route: &ModelRoute,
    requirements: &OperationRequirements,
) -> Result<PreflightPlan, PreparationFailure> {
    swallowtail_runtime::build_plan(
        &crate::grok_build_acp_descriptor(),
        instance,
        Some(route),
        requirements,
        prepared.access_profile(),
        prepared.access_evidence().status(),
        prepared.available_host_services(),
    )
}

fn activity_profile(
    prepared: &GrokPreparedIntegration,
) -> Result<ObservableActivityProfile, PreparationFailure> {
    let behavior_revision = match prepared.observation().compatibility() {
        swallowtail_core::InstalledExecutableCompatibility::Qualified(assessment) => {
            assessment.behavior_revision().clone()
        }
        swallowtail_core::InstalledExecutableCompatibility::UnverifiedNewer(assessment) => {
            assessment.behavior_revision().clone()
        }
        swallowtail_core::InstalledExecutableCompatibility::Incompatible => {
            return Err(preparation_failure(
                "swallowtail.grok.preparation.activity_version_incompatible",
                "Grok activity requires a permitted executable version",
            ));
        }
    };
    ObservableActivityProfile::available(
        [ActivityInterfaceBasis::new(
            prepared.observation().version().axis().clone(),
            behavior_revision,
        )],
        [
            kind(
                ActivityKindClass::AssistantMessage,
                ActivityLifecycleFidelity::UpdateAndCompletion,
                [ActivityContentStream::FinalAnswerText],
                ActivityDisclosure::ProviderDisplayContent,
            )?,
            kind(
                ActivityKindClass::ReasoningSummary,
                ActivityLifecycleFidelity::UpdateAndCompletion,
                [ActivityContentStream::ReasoningSummaryText],
                ActivityDisclosure::ProviderDisplayContent,
            )?,
            task_list_kind(
                ActivityKindClass::Plan,
                ActivityLifecycleFidelity::UpdateAndCompletion,
                [ActivityContentStream::PlanText],
                ActivityDisclosure::ProviderDisplayContent,
            )?,
            kind(
                ActivityKindClass::ProviderOwnedTool,
                ActivityLifecycleFidelity::CompleteLifecycle,
                [ActivityContentStream::ProviderToolDisplay],
                ActivityDisclosure::ProviderDisplayContent,
            )?,
            kind(
                ActivityKindClass::Unknown,
                ActivityLifecycleFidelity::CompletionOnly,
                [],
                ActivityDisclosure::IdentityAndLifecycleOnly,
            )?,
        ],
        ActivityUnknownEventPosture::PreserveNamespaced,
    )
    .map_err(|_| {
        preparation_failure(
            "swallowtail.grok.preparation.activity_profile_invalid",
            "Grok activity profile could not be derived",
        )
    })
}

fn kind(
    class: ActivityKindClass,
    lifecycle: ActivityLifecycleFidelity,
    streams: impl IntoIterator<Item = ActivityContentStream>,
    disclosure: ActivityDisclosure,
) -> Result<ActivityKindProfile, PreparationFailure> {
    ActivityKindProfile::new(class, lifecycle, streams, disclosure, []).map_err(|_| {
        preparation_failure(
            "swallowtail.grok.preparation.activity_profile_invalid",
            "Grok activity profile could not be derived",
        )
    })
}

fn task_list_kind(
    class: ActivityKindClass,
    lifecycle: ActivityLifecycleFidelity,
    streams: impl IntoIterator<Item = ActivityContentStream>,
    disclosure: ActivityDisclosure,
) -> Result<ActivityKindProfile, PreparationFailure> {
    kind(class, lifecycle, streams, disclosure)?
        .with_task_list_snapshots()
        .map_err(|_| {
            preparation_failure(
                "swallowtail.grok.preparation.activity_profile_invalid",
                "Grok activity profile could not be derived",
            )
        })
}

fn with_activity(
    capabilities: CapabilityProfile,
    activity: &ObservableActivityProfile,
) -> CapabilityProfile {
    let mut requirements = capabilities
        .iter()
        .map(|(capability, constraints)| {
            CapabilityRequirement::new(capability, constraints.iter().cloned())
        })
        .collect::<Vec<_>>();
    requirements.push(
        activity
            .capability_requirement()
            .expect("prepared Grok activity is available"),
    );
    CapabilityProfile::new(requirements)
}

fn profile_requirements(capabilities: &CapabilityProfile) -> Vec<CapabilityRequirement> {
    capabilities
        .iter()
        .map(|(capability, constraints)| {
            CapabilityRequirement::new(capability, constraints.iter().cloned())
        })
        .collect()
}

fn validate_prepared_model(
    prepared: &GrokPreparedIntegration,
    model_id: &str,
) -> Result<(), PreparationFailure> {
    let expected = crate::grok_build_model_for_version(prepared.observation().version().version())
        .ok_or_else(|| {
            preparation_failure(
                "swallowtail.grok.preparation.model_unsupported",
                "Grok prepared operations require a permitted executable version",
            )
        })?;
    if model_id != expected {
        return Err(preparation_failure(
            "swallowtail.grok.preparation.model_unsupported",
            "Grok prepared operations require the qualified model for this executable behavior",
        ));
    }
    Ok(())
}

fn validate_options(options: &SessionOptions) -> Result<(), PreparationFailure> {
    if !options.is_empty() {
        return Err(preparation_failure(
            "swallowtail.grok.preparation.session_options_unsupported",
            "Grok prepared sessions do not yet map portable session options",
        ));
    }
    Ok(())
}

fn preparation_failure(code: &'static str, message: &'static str) -> PreparationFailure {
    PreparationFailure::new(
        PreparationStage::Preflight,
        Diagnostic::new(SafeDiagnostic::new(code, message)),
    )
}
