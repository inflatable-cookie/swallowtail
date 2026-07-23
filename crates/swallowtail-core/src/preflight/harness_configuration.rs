use super::{PreflightContext, PreflightDimension, PreflightFailure};
use crate::{ExecutionLayer, HarnessConfigurationPosture, OperationRequirements};

pub(super) fn validate_harness_configuration(
    context: &PreflightContext<'_>,
    requirements: &OperationRequirements,
) -> Result<(), PreflightFailure> {
    let Some(required) = requirements.harness_configuration_posture() else {
        return Ok(());
    };
    if requirements.execution_layer() != ExecutionLayer::HarnessInteraction {
        return Err(failure(
            "Direct model inference cannot declare harness configuration",
        ));
    }
    if context.instance.harness_configuration_posture() != Some(required) {
        return Err(failure(
            "Configured instance does not match the required harness configuration posture",
        ));
    }
    if required == HarnessConfigurationPosture::ProviderSuppressed
        && requirements.interface_versions().next().is_none()
    {
        return Err(failure(
            "Provider-suppressed harness configuration requires exact interface-version evidence",
        ));
    }
    if required == HarnessConfigurationPosture::HostScoped {
        return Err(failure(
            "Host-scoped harness configuration requires a separately bound host lease",
        ));
    }
    Ok(())
}

fn failure(message: impl Into<String>) -> PreflightFailure {
    PreflightFailure::new(PreflightDimension::HarnessConfiguration, message)
}
