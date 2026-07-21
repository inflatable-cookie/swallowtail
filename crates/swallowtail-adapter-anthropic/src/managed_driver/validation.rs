fn validate_run(
    plan: &PreflightPlan,
    request: &StructuredRunRequest,
    services: &HostServices,
) -> Result<u64, RuntimeFailure> {
    if plan.model_route_id().is_none()
        || plan.model_id().is_none()
        || plan.provider_id().is_none_or(|provider| provider.as_str() != "anthropic")
    {
        return Err(binding_failure("model route"));
    }
    let agent = plan
        .provider_agent()
        .ok_or_else(|| binding_failure("provider agent"))?;
    let agent_version = agent
        .version()
        .as_str()
        .parse::<u64>()
        .map_err(|_| binding_failure("provider agent version"))?;
    let required = [
        Capability::StructuredRun,
        Capability::StreamingEvents,
        Capability::ToolCalls,
        Capability::UsageReporting,
        Capability::ProviderDurableRetention,
        Capability::ProviderManagedRecovery,
        Capability::OwnedRemoteResourceDeletion,
        Capability::Interruption,
        Capability::StreamReattachment,
    ];
    if required.into_iter().any(|capability| !requires(plan, capability))
        || plan
            .requirements()
            .capabilities()
            .any(|requirement| !required.contains(&requirement.capability()))
    {
        return Err(binding_failure("capability requirements"));
    }
    validate_constraints(plan)?;
    if request.deadline().is_none() {
        return Err(failure(
            "swallowtail.anthropic.managed.deadline_missing",
            "Anthropic Managed Agents requires an operation deadline",
        ));
    }
    if services.time().expect("validated time").now()
        >= request.deadline().expect("deadline exists").instant()
    {
        return Err(failure(
            "swallowtail.anthropic.managed.deadline_elapsed",
            "Anthropic Managed Agents deadline elapsed before provider work",
        ));
    }
    if request.working_resource().is_some() {
        return Err(unsupported("a working resource"));
    }
    if request.attachments().len() != 0 {
        return Err(unsupported("attachments"));
    }
    if request.structured_output().is_some() {
        return Err(unsupported("structured output"));
    }
    if request.maximum_output_tokens().is_some() {
        return Err(unsupported("a per-run output token limit"));
    }
    if request.policy().reasoning_mode().is_some()
        || request.policy().external_network() != ExternalNetworkPolicy::Denied
        || request.policy().external_search() != ExternalSearchPolicy::Disabled
        || request.policy().provider_execution() != ProviderExecutionPolicy::Attached
        || request.policy().provider_retention() != ProviderRetentionPolicy::DurableAllowed
        || request.policy().provider_recovery() != ProviderRecoveryPolicy::ManagedAllowed
        || request.policy().stream_reattachment()
            != StreamReattachmentPolicy::Bounded(
                std::num::NonZeroU32::new(1).expect("one is non-zero"),
            )
    {
        return Err(unsupported(
            "network, search, reasoning, retention, recovery, or reattachment policy drift",
        ));
    }
    Ok(agent_version)
}

fn validate_constraints(plan: &PreflightPlan) -> Result<(), RuntimeFailure> {
    let tool_constraints = [
        CapabilityConstraint::ToolSchemaDialect("json-schema-2020-12".to_owned()),
        CapabilityConstraint::ToolMaximumSchemaBytes(16_384),
        CapabilityConstraint::ToolMaximumCount(8),
    ];
    let deletion_constraints = [
        CapabilityConstraint::OwnedRemoteResource(OwnedRemoteResourceKind::Environment),
        CapabilityConstraint::OwnedRemoteResource(OwnedRemoteResourceKind::Session),
    ];
    let interruption = CapabilityConstraint::CancellationScope(CancellationScope::StructuredRun);
    let reattachment = CapabilityConstraint::ReattachmentMaximumCount(1);
    let valid = plan.requirements().capabilities().all(|requirement| {
        let constraints: Vec<_> = requirement.constraints().collect();
        match requirement.capability() {
            Capability::ToolCalls => constraints == tool_constraints.iter().collect::<Vec<_>>(),
            Capability::OwnedRemoteResourceDeletion => {
                constraints == deletion_constraints.iter().collect::<Vec<_>>()
            }
            Capability::Interruption => constraints == [&interruption],
            Capability::StreamReattachment => constraints == [&reattachment],
            _ => constraints.is_empty(),
        }
    });
    if valid {
        Ok(())
    } else {
        Err(binding_failure("capability constraints"))
    }
}

fn binding_failure(subject: &str) -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.managed.binding_rejected",
        format!("Anthropic Managed Agents {subject} did not match the bounded driver subset"),
    )
}
