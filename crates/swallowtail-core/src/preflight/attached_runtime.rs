use super::{PreflightContext, PreflightDimension, PreflightFailure};
use crate::{
    AttachedModelObservationScope, ExecutionLayer, InstanceOwnership, OperationRequirements,
    OperationShape,
};

pub(super) fn validate_attached_runtime(
    context: &PreflightContext<'_>,
    requirements: &OperationRequirements,
) -> Result<(), PreflightFailure> {
    let Some(required) = requirements.attached_runtime() else {
        return Ok(());
    };
    if requirements.execution_layer() != ExecutionLayer::DirectModelInference
        || requirements.operation_shape() != OperationShape::StructuredRun
        || context.instance.ownership() != InstanceOwnership::ExternalAttached
    {
        return Err(failure(
            "Attached-runtime requirements need structured direct inference against an external attached instance",
        ));
    }
    let Some(route) = context.model_route else {
        return Err(failure(
            "Attached-runtime requirements need an explicit model route",
        ));
    };
    let Some(observed) = context.attached_model_observation else {
        return Err(failure(
            "Attached-runtime requirements need selected-model detail evidence",
        ));
    };
    if observed.scope() != AttachedModelObservationScope::SelectedModelDetail
        || observed.instance_id() != context.instance.id()
        || observed.execution_host_id() != context.instance.execution_host_id()
        || observed.runtime_version() != required.runtime_version()
        || !context
            .instance
            .has_interface_version(required.runtime_version())
        || route.model_id() != required.model_id()
        || observed.model_tag() != required.model_tag()
        || observed.manifest_digest() != Some(required.manifest_digest())
    {
        return Err(failure(
            "Attached-runtime instance, topology, version, model, or manifest evidence does not match",
        ));
    }
    Ok(())
}

fn failure(message: &'static str) -> PreflightFailure {
    PreflightFailure::new(PreflightDimension::AttachedRuntime, message)
}
