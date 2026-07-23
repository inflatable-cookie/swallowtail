use crate::exec::failure;
use crate::selection::CodexExecBehavior;
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityRequirement, HarnessConfigurationPosture,
    HostServiceKind, PreflightPlan,
};
use swallowtail_runtime::{
    ExternalNetworkPolicy, ExternalSearchPolicy, HostServices, ProviderRetentionPolicy,
    RuntimeFailure, StructuredRunRequest, validate_harness_configuration_policy,
};

const JSON_SCHEMA_MEDIA_TYPE: &str = "application/schema+json";

pub(crate) fn validate(
    plan: &PreflightPlan,
    request: &StructuredRunRequest,
    services: &HostServices,
    behavior: CodexExecBehavior,
) -> Result<(), RuntimeFailure> {
    validate_behavior_policy(plan, request, behavior)?;
    if request.policy().provider_execution()
        != swallowtail_runtime::ProviderExecutionPolicy::Attached
        || request.policy().provider_recovery()
            != swallowtail_runtime::ProviderRecoveryPolicy::Prohibited
        || request.policy().stream_reattachment()
            != swallowtail_runtime::StreamReattachmentPolicy::Disabled
    {
        return Err(unsupported("provider-managed background execution"));
    }
    if request.tools().len() != 0 {
        return Err(unsupported("structured-run tools"));
    }
    validate_feature_binding(
        plan,
        request.attachments().len() != 0,
        Capability::Attachments,
        "attachments",
    )?;
    validate_feature_binding(
        plan,
        request.structured_output().is_some(),
        Capability::StructuredOutput,
        "structured output",
    )?;
    validate_feature_binding(
        plan,
        request.policy().reasoning_mode().is_some(),
        Capability::ReasoningSelection,
        "reasoning selection",
    )?;
    validate_feature_binding(
        plan,
        request.policy().external_search() == ExternalSearchPolicy::Enabled,
        Capability::ExternalSearch,
        "external search",
    )?;
    validate_feature_binding(
        plan,
        request.maximum_output_tokens().is_some(),
        Capability::OutputTokenLimit,
        "maximum output tokens",
    )?;
    if request.maximum_output_tokens().is_some() {
        return Err(unsupported("maximum output tokens"));
    }

    if request.attachments().len() > 1 {
        return Err(unsupported("more than one image attachment"));
    }
    if let Some(requirement) = capability_requirement(plan, Capability::Attachments) {
        validate_attachment(requirement, request)?;
        require_planned_service(plan, HostServiceKind::Attachment, "attachments")?;
        require_available(services.attachment().is_some(), "attachment")?;
    }
    if let Some(output) = request.structured_output() {
        if output.media_type() != JSON_SCHEMA_MEDIA_TYPE {
            return Err(unsupported("non-JSON Schema structured output"));
        }
        let requirement = capability_requirement(plan, Capability::StructuredOutput)
            .expect("feature binding was validated");
        if !requirement.constraints().any(|constraint| {
            matches!(constraint, CapabilityConstraint::SchemaDialect(value) if value == output.dialect())
        }) {
            return Err(plan_mismatch("structured-output schema dialect"));
        }
        require_planned_service(plan, HostServiceKind::Schema, "structured output")?;
        require_available(services.schema().is_some(), "schema")?;
    }
    if let Some(mode) = request.policy().reasoning_mode() {
        let requirement = capability_requirement(plan, Capability::ReasoningSelection)
            .expect("feature binding was validated");
        if !requirement.constraints().any(|constraint| {
            matches!(constraint, CapabilityConstraint::ReasoningMode(value) if value == mode)
        }) {
            return Err(plan_mismatch("reasoning mode"));
        }
    }

    match (
        request.policy().external_network(),
        request.policy().external_search(),
    ) {
        (ExternalNetworkPolicy::Denied, ExternalSearchPolicy::Disabled) => {}
        (ExternalNetworkPolicy::HostApproved, ExternalSearchPolicy::Enabled) => {
            require_planned_service(plan, HostServiceKind::Network, "external search")?;
            require_available(services.network().is_some(), "network")?;
        }
        (ExternalNetworkPolicy::HostApproved, ExternalSearchPolicy::Disabled) => {
            return Err(unsupported(
                "provider-side external network access without external search",
            ));
        }
        (ExternalNetworkPolicy::Denied, ExternalSearchPolicy::Enabled) => {
            return Err(plan_mismatch("external-search network policy"));
        }
        (ExternalNetworkPolicy::AmbientHost, _) => {
            return Err(unsupported("ambient host network authority"));
        }
    }

    if request.deadline().is_some() {
        require_planned_service(plan, HostServiceKind::Time, "deadlines")?;
        require_available(services.time().is_some(), "time")?;
    }
    Ok(())
}

fn validate_behavior_policy(
    plan: &PreflightPlan,
    request: &StructuredRunRequest,
    behavior: CodexExecBehavior,
) -> Result<(), RuntimeFailure> {
    let configuration = if behavior == CodexExecBehavior::EphemeralSuppressed {
        HarnessConfigurationPosture::ProviderSuppressed
    } else {
        HarnessConfigurationPosture::Ambient
    };
    if plan.harness_configuration_posture() != Some(configuration) {
        return Err(plan_mismatch("harness configuration posture"));
    }
    validate_harness_configuration_policy(plan, request.policy())
        .map_err(|_| plan_mismatch("harness configuration posture"))?;

    let retention = match behavior {
        CodexExecBehavior::RetainedBooleanSearch | CodexExecBehavior::RetainedSearchMode => {
            ProviderRetentionPolicy::DurableAllowed
        }
        CodexExecBehavior::EphemeralAmbient | CodexExecBehavior::EphemeralSuppressed => {
            ProviderRetentionPolicy::Prohibited
        }
    };
    if request.policy().provider_retention() != retention {
        return Err(unsupported("provider retention policy"));
    }
    Ok(())
}

fn validate_attachment(
    requirement: &CapabilityRequirement,
    request: &StructuredRunRequest,
) -> Result<(), RuntimeFailure> {
    let count = u32::try_from(request.attachments().len()).unwrap_or(u32::MAX);
    let count_supported = requirement.constraints().any(
        |constraint| matches!(constraint, CapabilityConstraint::AttachmentMaximumCount(maximum) if count <= *maximum),
    );
    if !count_supported {
        return Err(plan_mismatch("attachment count"));
    }
    for attachment in request.attachments() {
        if !attachment.media_type().starts_with("image/") {
            return Err(unsupported("non-image attachments"));
        }
        if !requirement.constraints().any(|constraint| {
            matches!(constraint, CapabilityConstraint::AttachmentMediaType(value) if value == attachment.media_type())
        }) {
            return Err(plan_mismatch("attachment media type"));
        }
        if let Some(length) = attachment.known_length()
            && !requirement.constraints().any(|constraint| {
                matches!(constraint, CapabilityConstraint::AttachmentMaximumBytes(maximum) if length <= *maximum)
            })
        {
            return Err(plan_mismatch("attachment size"));
        }
    }
    Ok(())
}

fn validate_feature_binding(
    plan: &PreflightPlan,
    requested: bool,
    capability: Capability,
    feature: &str,
) -> Result<(), RuntimeFailure> {
    if requested == capability_requirement(plan, capability).is_some() {
        Ok(())
    } else {
        Err(plan_mismatch(feature))
    }
}

fn capability_requirement(
    plan: &PreflightPlan,
    capability: Capability,
) -> Option<&CapabilityRequirement> {
    plan.requirements()
        .capabilities()
        .find(|requirement| requirement.capability() == capability)
}

fn require_planned_service(
    plan: &PreflightPlan,
    service: HostServiceKind,
    feature: &str,
) -> Result<(), RuntimeFailure> {
    if plan
        .requirements()
        .host_services()
        .any(|value| value == service)
    {
        Ok(())
    } else {
        Err(plan_mismatch(feature))
    }
}

fn require_available(available: bool, service: &str) -> Result<(), RuntimeFailure> {
    if available {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.codex.exec.host_service_missing",
            format!("Codex exec requires the preflight-bound {service} service"),
        ))
    }
}

fn plan_mismatch(feature: &str) -> RuntimeFailure {
    failure(
        "swallowtail.codex.exec.request_plan_mismatch",
        format!("Codex exec request does not match its preflight-bound {feature}"),
    )
}

fn unsupported(feature: &str) -> RuntimeFailure {
    failure(
        "swallowtail.codex.exec.unsupported_input",
        format!("Codex exec does not support {feature}"),
    )
}
