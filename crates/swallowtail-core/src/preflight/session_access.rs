use super::{PreflightDimension, PreflightFailure};
use crate::{
    Capability, CapabilityConstraint, HostServiceKind, OperationRequirements, ResourceAccess,
    ResourceRepresentation, SessionAccessPolicy,
};

pub(super) fn validate_session_access(
    requirements: &OperationRequirements,
) -> Result<(), PreflightFailure> {
    match (
        requirements.operation_shape(),
        requirements.session_access_policy(),
    ) {
        (crate::OperationShape::InteractiveSession, Some(policy)) => {
            validate_resource_access(requirements, policy)?;
            validate_network(requirements, policy)?;
            validate_provider_requests(requirements, policy)?;
        }
        (crate::OperationShape::InteractiveSession, None) => {
            return Err(failure("Interactive session access policy is missing"));
        }
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
    let capability = capability_requirement(requirements, Capability::WorkingResource);
    match (policy.resource_access(), capability) {
        (None, None) => Ok(()),
        (None, Some(_)) => Err(failure(
            "Resource-free session policy cannot declare a working-resource capability",
        )),
        (Some(ResourceAccess::Read), None) => Ok(()),
        (Some(access), Some(capability)) => {
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
            require_host_service(
                requirements,
                HostServiceKind::WorkingResource,
                "Session working-resource access requires a working-resource service",
            )
        }
        (Some(_), None) => Err(failure(
            "Session working-resource access is not capability-bound",
        )),
    }?;

    if policy.resource_access() == Some(ResourceAccess::ReadWrite)
        && capability_requirement(requirements, Capability::ToolCalls).is_some()
    {
        return Err(failure(
            "Bounded writable sessions cannot declare consumer tools",
        ));
    }
    Ok(())
}

fn validate_network(
    requirements: &OperationRequirements,
    policy: &SessionAccessPolicy,
) -> Result<(), PreflightFailure> {
    let network_required =
        capability_requirement(requirements, Capability::ProviderExternalNetwork).is_some();
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
        capability_requirement(requirements, Capability::ExternalSearch).is_some();
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
    for namespace in policy.provider_requests().observed_extensions() {
        if !requirements
            .extension_namespaces()
            .any(|required| required == namespace)
        {
            return Err(failure(format!(
                "Observed provider request extension '{}' is not preflight-bound",
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

fn capability_requirement(
    requirements: &OperationRequirements,
    capability: Capability,
) -> Option<&crate::CapabilityRequirement> {
    requirements
        .capabilities()
        .find(|requirement| requirement.capability() == capability)
}

fn failure(message: impl Into<String>) -> PreflightFailure {
    PreflightFailure::new(PreflightDimension::SessionAccess, message)
}
