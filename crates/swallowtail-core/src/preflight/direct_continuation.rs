use super::{PreflightContext, PreflightDimension, PreflightFailure};
use crate::{
    Capability, DriverRole, ExecutionLayer, OperationRequirements, OperationShape,
    SessionProviderStatePolicy,
};

pub(super) fn validate_direct_continuation(
    context: &PreflightContext<'_>,
    requirements: &OperationRequirements,
) -> Result<(), PreflightFailure> {
    let declarations = requirements
        .capabilities()
        .filter(|requirement| requirement.capability() == Capability::DirectToolContinuation)
        .count();
    let Some(direct) = requirements.direct_continuation() else {
        return if declarations == 0 {
            Ok(())
        } else {
            Err(failure(
                "Direct-continuation capability requires direct-continuation policy",
            ))
        };
    };

    if requirements.driver_role() != DriverRole::InteractiveSession
        || requirements.execution_layer() != ExecutionLayer::DirectModelInference
        || requirements.operation_shape() != OperationShape::InteractiveSession
    {
        return Err(failure(
            "Direct continuation requires an interactive direct-inference role",
        ));
    }
    if declarations != 1 {
        return Err(failure(
            "Direct continuation requires exactly one capability declaration",
        ));
    }
    if requirements
        .session_access_policy()
        .is_none_or(|policy| policy.resource_access().is_some())
    {
        return Err(failure("Direct continuation must be resource-free"));
    }
    if requirements.session_provider_state_policy() != Some(SessionProviderStatePolicy::Prohibited)
    {
        return Err(failure(
            "Direct continuation cannot create provider conversation state",
        ));
    }
    if requirements
        .capabilities()
        .any(|requirement| requirement.capability() == Capability::Resume)
    {
        return Err(failure("Direct continuation cannot require resume"));
    }

    let route = context.model_route.ok_or_else(|| {
        PreflightFailure::new(
            PreflightDimension::ModelRoute,
            "Direct continuation requires an exact model route",
        )
    })?;
    if route.model_id() != direct.model_id() {
        return Err(PreflightFailure::new(
            PreflightDimension::ModelRoute,
            "Direct-continuation model does not match the selected route",
        ));
    }

    let expected = direct.config().capability_requirements();
    for required in expected {
        let declared = requirements.capabilities().find(|candidate| {
            candidate.capability() == required.capability()
                && candidate.constraints().eq(required.constraints())
        });
        if declared.is_none() {
            return Err(failure(
                "Direct-continuation capability does not match its exact policy bounds",
            ));
        }
    }
    Ok(())
}

fn failure(message: &'static str) -> PreflightFailure {
    PreflightFailure::new(PreflightDimension::DirectContinuation, message)
}
