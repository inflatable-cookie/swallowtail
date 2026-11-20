use super::{PreflightDimension, PreflightFailure};
use crate::{
    Capability, CapabilityConstraint, DriverRole, HostServiceKind, OperationRequirements,
    OperationShape, ResourceAccess, ResourceRepresentation, SessionAccessPolicy,
};

pub(super) fn validate_session_access(
    requirements: &OperationRequirements,
) -> Result<(), PreflightFailure> {
    match (
        requirements.operation_shape(),
        requirements.session_access_policy(),
    ) {
        (
            OperationShape::InteractiveSession
            | OperationShape::ProviderSessionImport
            | OperationShape::ProviderSessionReconciliation
            | OperationShape::ProviderSessionHistory,
            Some(policy),
        ) => {
            validate_resource_access(requirements, policy)?;
            validate_network(requirements, policy)?;
            validate_provider_requests(requirements, policy)?;
        }
        (OperationShape::InteractiveSession, None)
            if requirements.driver_role() != DriverRole::ModelCatalog =>
        {
            return Err(failure("Interactive session access policy is missing"));
        }
        (OperationShape::ProviderSessionImport, None) => {
            return Err(failure("Provider-session import access policy is missing"));
        }
        (OperationShape::ProviderSessionReconciliation, None) => {
            return Err(failure(
                "Provider-session reconciliation access policy is missing",
            ));
        }
        (OperationShape::ProviderSessionHistory, None) => {
            return Err(failure("Provider-session history access policy is missing"));
        }
        (OperationShape::InteractiveSession, None) => {}
        (_, Some(_)) => {
            return Err(failure(
                "Session access policy is bound to a non-interactive operation",
            ));
        }
        (_, None) => {}
    }
    Ok(())
}

fn validate_resource_access(
    requirements: &OperationRequirements,
    policy: &SessionAccessPolicy,
) -> Result<(), PreflightFailure> {
    let declarations = capability_requirements(requirements, Capability::WorkingResource);
    match policy.resource_access() {
        None => {
            if declarations.is_empty() {
                Ok(())
            } else {
                Err(failure(
                    "Resource-free session policy cannot declare a working-resource capability",
                ))
            }
        }
        Some(ResourceAccess::Read) if declarations.is_empty() => Ok(()),
        Some(access) => {
            if declarations.is_empty() {
                return Err(failure(
                    "Session working-resource access is not capability-bound",
                ));
            }
            // Every matching declaration must agree: a duplicate, conflicting
            // entry cannot hide behind the first one.
            for declaration in declarations {
                validate_working_resource_declaration(declaration, access)?;
            }
            require_host_service(
                requirements,
                HostServiceKind::WorkingResource,
                "Session working-resource access requires a working-resource service",
            )
        }
    }?;

    if policy.filesystem_boundary().is_some()
        && policy.resource_access() == Some(ResourceAccess::ReadWrite)
        && !capability_requirements(requirements, Capability::ToolCalls).is_empty()
    {
        return Err(failure(
            "Bounded writable sessions cannot declare consumer tools",
        ));
    }
    Ok(())
}

fn validate_working_resource_declaration(
    capability: &crate::CapabilityRequirement,
    access: ResourceAccess,
) -> Result<(), PreflightFailure> {
    let mut access_constraints = capability.constraints().filter_map(|constraint| {
        if let CapabilityConstraint::ResourceAccess(value) = constraint {
            Some(*value)
        } else {
            None
        }
    });
    if access_constraints.next() != Some(access) || access_constraints.next().is_some() {
        return Err(failure(
            "Session working-resource access does not match its capability constraint",
        ));
    }
    let mut representations = capability.constraints().filter_map(|constraint| {
        if let CapabilityConstraint::ResourceRepresentation(value) = constraint {
            Some(*value)
        } else {
            None
        }
    });
    if representations.next() != Some(ResourceRepresentation::Filesystem)
        || representations.next().is_some()
    {
        return Err(failure(
            "Session working resource is not bound to one filesystem representation",
        ));
    }
    Ok(())
}

fn validate_network(
    requirements: &OperationRequirements,
    policy: &SessionAccessPolicy,
) -> Result<(), PreflightFailure> {
    let network_required =
        !capability_requirements(requirements, Capability::ProviderExternalNetwork).is_empty();
    let network_enabled = policy.external_network() == crate::ExternalNetworkPolicy::HostApproved;
    if network_required != network_enabled {
        return Err(failure(
            "Session provider-network policy does not match its capability requirement",
        ));
    }
    if network_enabled {
        require_host_service(
            requirements,
            HostServiceKind::Network,
            "Host-approved provider network requires a network service",
        )?;
    }

    let search_required =
        !capability_requirements(requirements, Capability::ExternalSearch).is_empty();
    let search_enabled = policy.external_search() == crate::ExternalSearchPolicy::Enabled;
    if search_required != search_enabled {
        return Err(failure(
            "Session external-search policy does not match its capability requirement",
        ));
    }
    Ok(())
}

fn validate_provider_requests(
    requirements: &OperationRequirements,
    policy: &SessionAccessPolicy,
) -> Result<(), PreflightFailure> {
    for namespace in policy
        .provider_requests()
        .observed_extensions()
        .chain(policy.provider_requests().exchanged_extensions())
    {
        if !requirements
            .extension_namespaces()
            .any(|required| required == namespace)
        {
            return Err(failure(format!(
                "Provider request extension '{}' is not preflight-bound",
                namespace.as_str()
            )));
        }
    }
    Ok(())
}

fn require_host_service(
    requirements: &OperationRequirements,
    service: HostServiceKind,
    message: &str,
) -> Result<(), PreflightFailure> {
    if requirements
        .host_services()
        .any(|required| required == service)
    {
        Ok(())
    } else {
        Err(failure(message))
    }
}

fn capability_requirements(
    requirements: &OperationRequirements,
    capability: Capability,
) -> Vec<&crate::CapabilityRequirement> {
    requirements
        .capabilities()
        .filter(|requirement| requirement.capability() == capability)
        .collect()
}

fn failure(message: impl Into<String>) -> PreflightFailure {
    PreflightFailure::new(PreflightDimension::SessionAccess, message)
}

#[cfg(test)]
mod tests {
    use super::validate_session_access;
    use crate::{
        AccessProfileId, AccessRequirement, Capability, CapabilityConstraint,
        CapabilityRequirement, DriverRole, ExecutionHostId, ExecutionLayer, HostServiceKind,
        OperationRequirements, OperationShape, PreflightDimension, ResourceAccess,
        ResourceRepresentation, SessionAccessPolicy,
    };

    fn requirements(
        capabilities: Vec<CapabilityRequirement>,
        policy: SessionAccessPolicy,
    ) -> OperationRequirements {
        OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            OperationShape::InteractiveSession,
            DriverRole::InteractiveSession,
            ExecutionHostId::new("fixture.host.local").expect("host id is valid"),
            AccessRequirement::new(AccessProfileId::new("fixture.access.local").expect("profile")),
        )
        .with_host_services([HostServiceKind::WorkingResource])
        .with_capabilities(capabilities)
        .with_session_access_policy(policy)
    }

    fn working_resource(access: ResourceAccess) -> CapabilityRequirement {
        CapabilityRequirement::new(
            Capability::WorkingResource,
            [
                CapabilityConstraint::ResourceAccess(access),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        )
    }

    #[test]
    fn a_single_matching_working_resource_declaration_still_passes() {
        let requirements = requirements(
            vec![working_resource(ResourceAccess::Read)],
            SessionAccessPolicy::read_only(),
        );
        validate_session_access(&requirements).expect("one matching declaration is admitted");
    }

    #[test]
    fn duplicate_working_resource_declarations_must_all_agree() {
        let requirements = requirements(
            vec![
                working_resource(ResourceAccess::Read),
                working_resource(ResourceAccess::ReadWrite),
            ],
            SessionAccessPolicy::read_only(),
        );
        let failure = validate_session_access(&requirements)
            .expect_err("a conflicting duplicate declaration cannot hide behind the first one");
        assert_eq!(failure.dimension(), PreflightDimension::SessionAccess);
    }
}
