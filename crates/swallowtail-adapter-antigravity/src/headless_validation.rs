use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, CredentialMechanism,
    HarnessConfigurationPosture, HarnessIsolation, HostServiceKind, InstanceOwnership,
    PreflightPlan, ReasoningMode, ResourceAccess, ResourceRepresentation,
    StructuredOutputEnforcement, SupportAuthority,
};
use swallowtail_runtime::{
    ExternalNetworkPolicy, ExternalSearchPolicy, HostServices, ProviderExecutionPolicy,
    ProviderRecoveryPolicy, ProviderRetentionPolicy, RuntimeFailure, SchemaDocument,
    StreamReattachmentPolicy, StructuredRunRequest, validate_harness_configuration_policy,
    validate_harness_isolation_policy,
};

const MAXIMUM_SCHEMA_BYTES: usize = 16 * 1024;

pub(crate) struct ValidatedHeadlessInput {
    pub(crate) access: ResourceAccess,
    pub(crate) isolation: HarnessIsolation,
    pub(crate) effort: Option<ReasoningMode>,
    pub(crate) schema: Option<String>,
}

pub(crate) fn validate(
    plan: &PreflightPlan,
    request: &StructuredRunRequest,
    services: &HostServices,
) -> Result<ValidatedHeadlessInput, RuntimeFailure> {
    if plan.driver_identity().id().as_str() != crate::HEADLESS_DRIVER_ID {
        return Err(plan_mismatch("driver"));
    }
    crate::selection::validate_antigravity_headless_plan(plan)?;
    services.require_execution_host(plan.execution_host_id())?;
    require_service(
        plan,
        services.task().is_some(),
        HostServiceKind::Task,
        "task",
    )?;
    require_service(
        plan,
        services.process().is_some(),
        HostServiceKind::Process,
        "process",
    )?;
    require_service(
        plan,
        services.time().is_some(),
        HostServiceKind::Time,
        "time",
    )?;
    if plan.ownership() != InstanceOwnership::HostOwnedEphemeral
        || plan.credential_mechanism() != &CredentialMechanism::LocalUnauthenticated
        || plan.credential_reference().is_some()
        || plan.endpoint_audience().as_str() != crate::ANTIGRAVITY_PERSONAL_GOOGLE_AUDIENCE
        || plan.access_status().support_authority() != SupportAuthority::ProviderSupported
        || plan.model_id().is_none()
        || plan.model_route_id().is_none()
    {
        return Err(plan_mismatch("instance, access, or model route"));
    }
    validate_harness_configuration_policy(plan, request.policy())
        .map_err(|_| plan_mismatch("harness configuration posture"))?;
    validate_harness_isolation_policy(plan, request.policy())
        .map_err(|_| plan_mismatch("harness isolation"))?;
    let isolation = request
        .policy()
        .harness_isolation()
        .ok_or_else(|| plan_mismatch("harness isolation"))?;
    if !matches!(
        isolation,
        HarnessIsolation::AmbientHost | HarnessIsolation::ProviderEnforced
    ) || request.policy().harness_configuration_posture()
        != Some(HarnessConfigurationPosture::Ambient)
        || request.policy().provider_execution() != ProviderExecutionPolicy::Attached
        || request.policy().provider_retention() != ProviderRetentionPolicy::DurableAllowed
        || request.policy().provider_recovery() != ProviderRecoveryPolicy::Prohibited
        || request.policy().stream_reattachment() != StreamReattachmentPolicy::Disabled
    {
        return Err(unsupported(
            "provider lifecycle or harness authority policy",
        ));
    }
    if request.policy().external_network() != ExternalNetworkPolicy::Denied
        || request.policy().external_search() != ExternalSearchPolicy::Disabled
        || request.policy().harness_mode().is_some()
    {
        return Err(unsupported("consumer network, search, or harness mode"));
    }
    if request.working_resource().is_none() || request.deadline().is_none() {
        return Err(unsupported("missing working resource or host deadline"));
    }
    if request.attachments().len() != 0
        || request.tools().len() != 0
        || request.maximum_output_tokens().is_some()
    {
        return Err(unsupported(
            "attachments, consumer tools, or output-token limit",
        ));
    }
    require_capability(plan, Capability::StructuredRun)?;
    require_capability(plan, Capability::StreamingEvents)?;
    require_capability(plan, Capability::ObservableActivity)?;
    require_capability(plan, Capability::UsageReporting)?;
    require_capability(plan, Capability::ProviderDurableRetention)?;
    require_constraint(
        plan,
        Capability::Interruption,
        CapabilityConstraint::CancellationScope(CancellationScope::StructuredRun),
    )?;
    require_constraint(
        plan,
        Capability::WorkingResource,
        CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
    )?;
    let read = has_constraint(
        plan,
        Capability::WorkingResource,
        &CapabilityConstraint::ResourceAccess(ResourceAccess::Read),
    );
    let write = has_constraint(
        plan,
        Capability::WorkingResource,
        &CapabilityConstraint::ResourceAccess(ResourceAccess::ReadWrite),
    );
    let access = match (read, write) {
        (true, false) => ResourceAccess::Read,
        (false, true) => ResourceAccess::ReadWrite,
        _ => return Err(plan_mismatch("working-resource authority")),
    };
    let effort = validate_effort(plan, request.policy().reasoning_mode())?;
    let schema = validate_schema(plan, request)?;
    Ok(ValidatedHeadlessInput {
        access,
        isolation,
        effort,
        schema,
    })
}

fn validate_effort(
    plan: &PreflightPlan,
    requested: Option<&ReasoningMode>,
) -> Result<Option<ReasoningMode>, RuntimeFailure> {
    let Some(requested) = requested else {
        return Ok(None);
    };
    if !matches!(requested.as_str(), "low" | "medium" | "high")
        || !has_constraint(
            plan,
            Capability::ReasoningSelection,
            &CapabilityConstraint::ReasoningMode(requested.clone()),
        )
    {
        return Err(unsupported("unplanned reasoning effort"));
    }
    Ok(Some(requested.clone()))
}

fn validate_schema(
    plan: &PreflightPlan,
    request: &StructuredRunRequest,
) -> Result<Option<String>, RuntimeFailure> {
    let Some(output) = request.structured_output() else {
        return Ok(None);
    };
    if output.media_type() != "application/schema+json"
        || output.dialect() != "json-schema-2020-12"
        || !has_constraint(
            plan,
            Capability::StructuredOutput,
            &CapabilityConstraint::SchemaDialect("json-schema-2020-12".to_owned()),
        )
        || !has_constraint(
            plan,
            Capability::StructuredOutput,
            &CapabilityConstraint::StructuredOutputEnforcement(
                StructuredOutputEnforcement::ProviderNative,
            ),
        )
    {
        return Err(unsupported("unplanned structured output"));
    }
    let SchemaDocument::Inline(bytes) = output.document() else {
        return Err(unsupported("referenced structured output schema"));
    };
    if bytes.is_empty() || bytes.len() > MAXIMUM_SCHEMA_BYTES {
        return Err(unsupported("empty or oversized structured output schema"));
    }
    let value: serde_json::Value = serde_json::from_slice(bytes)
        .map_err(|_| unsupported("malformed structured output schema"))?;
    if !value.is_object() {
        return Err(unsupported("non-object structured output schema"));
    }
    String::from_utf8(bytes.clone())
        .map(Some)
        .map_err(|_| unsupported("non-UTF-8 structured output schema"))
}

fn require_service(
    plan: &PreflightPlan,
    available: bool,
    service: HostServiceKind,
    name: &str,
) -> Result<(), RuntimeFailure> {
    if !plan
        .requirements()
        .host_services()
        .any(|required| required == service)
    {
        Err(plan_mismatch(name))
    } else if !available {
        Err(crate::failure::failure(
            "swallowtail.antigravity.headless.host_service_missing",
            format!("Antigravity headless requires the preflight-bound {name} service"),
        ))
    } else {
        Ok(())
    }
}

fn require_capability(plan: &PreflightPlan, capability: Capability) -> Result<(), RuntimeFailure> {
    if plan
        .requirements()
        .capabilities()
        .any(|required| required.capability() == capability)
    {
        Ok(())
    } else {
        Err(plan_mismatch("capability"))
    }
}

fn require_constraint(
    plan: &PreflightPlan,
    capability: Capability,
    constraint: CapabilityConstraint,
) -> Result<(), RuntimeFailure> {
    if has_constraint(plan, capability, &constraint) {
        Ok(())
    } else {
        Err(plan_mismatch("capability constraint"))
    }
}

fn has_constraint(
    plan: &PreflightPlan,
    capability: Capability,
    constraint: &CapabilityConstraint,
) -> bool {
    plan.requirements().capabilities().any(|required| {
        required.capability() == capability
            && required
                .constraints()
                .any(|required| required == constraint)
    })
}

fn plan_mismatch(dimension: &str) -> RuntimeFailure {
    crate::failure::failure(
        "swallowtail.antigravity.headless.request_plan_mismatch",
        format!("Antigravity headless request does not match its preflight-bound {dimension}"),
    )
}

use crate::failure::unsupported;
