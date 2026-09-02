use super::*;

pub(in crate::prepared_profile) type ClaudeAgentPreparedOpenLifecycleFuture =
    swallowtail_runtime::BoxFuture<
        'static,
        Result<
            (
                Box<dyn swallowtail_runtime::InteractiveSessionHandle>,
                ClaudeAgentOpenObservation,
            ),
            ClaudeAgentOpenRejection,
        >,
    >;

pub(in crate::prepared_profile) fn with_activity(
    capabilities: CapabilityProfile,
    activity: &swallowtail_core::ObservableActivityProfile,
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
            .expect("prepared Claude Agent activity is available"),
    );
    CapabilityProfile::new(requirements)
}

pub(in crate::prepared_profile) fn lifecycle_management_instance(
    prepared: &ClaudeAgentPreparedIntegration,
) -> swallowtail_core::ConfiguredInstance {
    instance_with_capabilities(
        prepared,
        CapabilityProfile::new([
            CapabilityRequirement::new(Capability::ProviderNativeSessionClose, []),
            CapabilityRequirement::new(Capability::ProviderSessionDelete, []),
        ]),
    )
}

pub(in crate::prepared_profile) fn validate_options(
    options: &SessionOptions,
    supports_config_options: bool,
) -> Result<(), PreparationFailure> {
    if options.developer_instructions().is_some() || options.tools().len() != 0 {
        return Err(failure(
            "swallowtail.claude_agent.preparation.session_options_unsupported",
            "Claude Agent ACP prepared sessions support only the portable reasoning option",
        ));
    }
    if options
        .reasoning_mode()
        .is_some_and(|mode| !REASONING_MODES.contains(&mode.as_str()))
    {
        return Err(failure(
            "swallowtail.claude_agent.preparation.reasoning_mode_unsupported",
            "Claude Agent ACP prepared session reasoning mode is unsupported",
        ));
    }
    if options.harness_mode().is_some() && !supports_config_options {
        return Err(failure(
            "swallowtail.claude_agent.preparation.harness_mode_unsupported",
            "Claude Agent ACP prepared session harness mode is unsupported",
        ));
    }
    Ok(())
}

pub(in crate::prepared_profile) fn reject_attachment_reasoning(
    options: &SessionOptions,
) -> Result<(), PreparationFailure> {
    if options.reasoning_mode().is_some() {
        Err(failure(
            "swallowtail.claude_agent.preparation.attachment_reasoning_unsupported",
            "Claude Agent load and resume cannot redeclare reasoning selection",
        ))
    } else {
        Ok(())
    }
}

pub(in crate::prepared_profile) fn operation_capabilities(
    available: &CapabilityProfile,
    options: &SessionOptions,
) -> Vec<CapabilityRequirement> {
    let mut capabilities = available
        .iter()
        .filter(|(capability, _)| {
            !matches!(
                capability,
                Capability::ReasoningSelection | Capability::HarnessModeSelection
            )
        })
        .map(|(capability, constraints)| {
            CapabilityRequirement::new(capability, constraints.iter().cloned())
        })
        .collect::<Vec<_>>();
    if let Some(mode) = options.reasoning_mode() {
        capabilities.push(CapabilityRequirement::new(
            Capability::ReasoningSelection,
            [swallowtail_core::CapabilityConstraint::ReasoningMode(
                mode.clone(),
            )],
        ));
    }
    if let Some(mode) = options.harness_mode() {
        capabilities.push(CapabilityRequirement::new(
            Capability::HarnessModeSelection,
            [swallowtail_core::CapabilityConstraint::HarnessMode(mode)],
        ));
    }
    capabilities
}
