use super::{PreflightContext, PreflightDimension, PreflightFailure};
use crate::{Capability, DriverRole, ExecutionLayer, OperationRequirements, OperationShape};

pub(super) fn validate_realtime_media(
    context: &PreflightContext<'_>,
    requirements: &OperationRequirements,
) -> Result<(), PreflightFailure> {
    let role = requirements.driver_role();
    let Some(media) = requirements.realtime_media() else {
        return if role == DriverRole::RealtimeMediaSession {
            Err(failure(
                PreflightDimension::RealtimeMedia,
                "Realtime-media session requirements are missing",
            ))
        } else {
            Ok(())
        };
    };

    if role != DriverRole::RealtimeMediaSession {
        return Err(failure(
            PreflightDimension::RealtimeMedia,
            "Realtime-media requirements are bound to a different driver role",
        ));
    }
    if requirements.execution_layer() != ExecutionLayer::DirectModelInference {
        return Err(failure(
            PreflightDimension::ExecutionLayer,
            "Realtime media requires direct model inference",
        ));
    }
    if requirements.operation_shape() != OperationShape::InteractiveSession {
        return Err(failure(
            PreflightDimension::OperationShape,
            "Realtime media requires an interactive session",
        ));
    }

    let route = context.model_route.ok_or_else(|| {
        failure(
            PreflightDimension::ModelRoute,
            "Realtime media requires an exact model route",
        )
    })?;
    if route.model_id() != media.model_id() {
        return Err(failure(
            PreflightDimension::ModelRoute,
            "Realtime media model does not match the selected route",
        ));
    }

    let required = media.config().capability_requirement();
    validate_profile(context.instance.capabilities(), &required)?;
    validate_profile(route.capabilities(), &required)
}

fn validate_profile(
    profile: &crate::CapabilityProfile,
    required: &crate::CapabilityRequirement,
) -> Result<(), PreflightFailure> {
    if !profile.supports(Capability::RealtimeMedia) {
        return Err(failure(
            PreflightDimension::Capability,
            "Required realtime-media capability is unsupported",
        ));
    }
    for constraint in required.constraints() {
        if !profile.supports_constraint(Capability::RealtimeMedia, constraint) {
            return Err(failure(
                PreflightDimension::Constraint,
                "Required realtime-media format or bound is unsupported",
            ));
        }
    }
    Ok(())
}

fn failure(dimension: PreflightDimension, message: &'static str) -> PreflightFailure {
    PreflightFailure::new(dimension, message)
}
