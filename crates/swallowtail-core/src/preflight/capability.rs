use super::{PreflightContext, PreflightDimension, PreflightFailure};
use crate::{Capability, OperationRequirements};

pub(super) fn validate_capabilities(
    context: &PreflightContext<'_>,
    requirements: &OperationRequirements,
) -> Result<(), PreflightFailure> {
    for requirement in requirements.capabilities() {
        let capability = requirement.capability();
        validate_profile(context.instance.capabilities(), capability, requirement)?;
        if let Some(route) = context.model_route {
            validate_profile(route.capabilities(), capability, requirement)?;
        }
    }
    Ok(())
}

fn validate_profile(
    profile: &crate::CapabilityProfile,
    capability: Capability,
    requirement: &crate::CapabilityRequirement,
) -> Result<(), PreflightFailure> {
    if !profile.supports(capability) {
        return Err(PreflightFailure::new(
            PreflightDimension::Capability,
            format!("Required capability {capability:?} is unsupported"),
        ));
    }
    for constraint in requirement.constraints() {
        if !profile.supports_constraint(capability, constraint) {
            return Err(PreflightFailure::new(
                PreflightDimension::Constraint,
                format!("Required constraint for {capability:?} is unsupported"),
            ));
        }
    }
    Ok(())
}
