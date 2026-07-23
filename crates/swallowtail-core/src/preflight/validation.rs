use super::attached_runtime::validate_attached_runtime;
use super::capability::validate_capabilities;
use super::direct_continuation::validate_direct_continuation;
use super::planned_connection_rollover::validate_planned_connection_rollover;
use super::realtime_media::validate_realtime_media;
use super::session_access::validate_session_access;
use super::session_provider_state::validate_session_provider_state;
use super::{PreflightContext, PreflightDimension, PreflightFailure};
use crate::{ExecutionLayer, HostServiceKind, OperationRequirements, OperationShape};

pub(super) fn validate(
    context: &PreflightContext<'_>,
    requirements: &OperationRequirements,
) -> Result<(), PreflightFailure> {
    if context.instance.driver_id() != context.driver.identity().id() {
        return Err(failure(
            PreflightDimension::Driver,
            "Configured instance does not use the selected driver",
        ));
    }
    if !context.driver.supports_role(requirements.driver_role()) {
        return Err(failure(
            PreflightDimension::Role,
            "Selected driver does not implement the required role",
        ));
    }
    if !context
        .driver
        .supports_execution_layer(requirements.execution_layer())
    {
        return Err(failure(
            PreflightDimension::ExecutionLayer,
            "Selected driver does not support the required execution layer",
        ));
    }
    if !context
        .driver
        .supports_operation_shape(requirements.operation_shape())
    {
        return Err(failure(
            PreflightDimension::OperationShape,
            "Selected driver does not support the required operation shape",
        ));
    }
    if context.instance.execution_host_id() != requirements.execution_host_id() {
        return Err(failure(
            PreflightDimension::Topology,
            "Configured instance is placed on a different execution host",
        ));
    }
    if !requirements.accepts_ownership(context.instance.ownership()) {
        return Err(failure(
            PreflightDimension::Ownership,
            "Configured instance ownership is not accepted",
        ));
    }

    validate_access(context, requirements)?;
    validate_interface_versions(context, requirements)?;
    validate_harness_rpc_policy(context, requirements)?;
    validate_route(context, requirements)?;
    validate_artifact(context, requirements)?;
    validate_host_services(context, requirements)?;
    validate_harness_isolation(requirements)?;
    validate_session_access(requirements)?;
    validate_session_provider_state(requirements)?;
    validate_realtime_media(context, requirements)?;
    validate_planned_connection_rollover(requirements)?;
    validate_direct_continuation(context, requirements)?;
    validate_attached_runtime(context, requirements)?;
    validate_capabilities(context, requirements)?;

    for namespace in requirements.extension_namespaces() {
        if !context.driver.supports_extension(namespace) {
            return Err(failure(
                PreflightDimension::Extension,
                format!("Required extension '{}' is unsupported", namespace.as_str()),
            ));
        }
    }
    Ok(())
}

fn validate_harness_rpc_policy(
    context: &PreflightContext<'_>,
    requirements: &OperationRequirements,
) -> Result<(), PreflightFailure> {
    let Some(required) = requirements.harness_rpc_policy() else {
        return Ok(());
    };
    if requirements.execution_layer() != ExecutionLayer::HarnessInteraction
        || requirements.operation_shape() != OperationShape::InteractiveSession
    {
        return Err(failure(
            PreflightDimension::HarnessRpcPolicy,
            "Harness RPC policy requires an interactive harness operation",
        ));
    }
    if context.instance.harness_rpc_policy() != Some(required) {
        return Err(failure(
            PreflightDimension::HarnessRpcPolicy,
            "Configured instance does not match the required harness RPC policy",
        ));
    }
    Ok(())
}

fn validate_interface_versions(
    context: &PreflightContext<'_>,
    requirements: &OperationRequirements,
) -> Result<(), PreflightFailure> {
    for required in requirements.interface_versions() {
        if !context.instance.has_interface_version(required) {
            return Err(failure(
                PreflightDimension::InterfaceVersion,
                format!(
                    "Configured instance does not bind required interface axis '{}'",
                    required.axis().as_str()
                ),
            ));
        }
        if !context.driver.supports_interface_version(required) {
            return Err(failure(
                PreflightDimension::InterfaceVersion,
                format!(
                    "Selected driver is not qualified for interface axis '{}'",
                    required.axis().as_str()
                ),
            ));
        }
    }
    Ok(())
}

fn validate_harness_isolation(
    requirements: &OperationRequirements,
) -> Result<(), PreflightFailure> {
    let Some(isolation) = requirements.harness_isolation() else {
        return Ok(());
    };

    if requirements.execution_layer() != ExecutionLayer::HarnessInteraction {
        return Err(failure(
            PreflightDimension::HarnessIsolation,
            "Direct model inference cannot declare harness isolation",
        ));
    }

    if requirements.operation_shape() == OperationShape::InteractiveSession {
        let session_isolation = requirements
            .session_access_policy()
            .and_then(|policy| policy.harness_isolation());
        if session_isolation != Some(isolation) {
            return Err(failure(
                PreflightDimension::HarnessIsolation,
                "Harness isolation does not match interactive session access policy",
            ));
        }
    }

    Ok(())
}

fn validate_artifact(
    context: &PreflightContext<'_>,
    requirements: &OperationRequirements,
) -> Result<(), PreflightFailure> {
    if requirements
        .host_services()
        .any(|service| service == HostServiceKind::ModelArtifact)
        && context.model_artifact.is_none()
    {
        return Err(failure(
            PreflightDimension::ModelArtifact,
            "Operation requires an explicit model artifact",
        ));
    }
    Ok(())
}

fn validate_access(
    context: &PreflightContext<'_>,
    requirements: &OperationRequirements,
) -> Result<(), PreflightFailure> {
    let required = requirements.access();
    if context.instance.access_profile_id() != context.access_profile.id()
        || context.access_status.profile_id() != context.access_profile.id()
        || required.profile_id() != context.access_profile.id()
    {
        return Err(failure(
            PreflightDimension::Access,
            "Selected access profile does not match the instance and access status",
        ));
    }
    if context.instance.support_authority() != context.access_profile.support_authority()
        || context.access_status.support_authority() != context.access_profile.support_authority()
        || !required.accepts_support_authority(context.access_profile.support_authority())
    {
        return Err(failure(
            PreflightDimension::SupportAuthority,
            "Selected support authority is not accepted",
        ));
    }
    if !required.accepts_credential(context.access_status.credential())
        || !required.accepts_entitlement(context.access_status.entitlement())
        || !required.accepts_endpoint_authorization(context.access_status.endpoint_authorization())
        || !required.accepts_runtime_readiness(context.access_status.runtime_readiness())
    {
        return Err(failure(
            PreflightDimension::Access,
            "Access state does not satisfy the operation requirements",
        ));
    }
    Ok(())
}

fn validate_route(
    context: &PreflightContext<'_>,
    requirements: &OperationRequirements,
) -> Result<(), PreflightFailure> {
    if requirements.model_route_required() && context.model_route.is_none() {
        return Err(failure(
            PreflightDimension::ModelRoute,
            "Operation requires an explicit model route",
        ));
    }
    if let Some(route) = context.model_route
        && route.instance_id() != context.instance.id()
    {
        return Err(failure(
            PreflightDimension::ModelRoute,
            "Model route belongs to a different configured instance",
        ));
    }
    Ok(())
}

fn validate_host_services(
    context: &PreflightContext<'_>,
    requirements: &OperationRequirements,
) -> Result<(), PreflightFailure> {
    let driver_services = context
        .driver
        .required_host_services(requirements.driver_role());
    for service in driver_services.chain(requirements.host_services()) {
        if !context.available_host_services.contains(&service) {
            return Err(failure(
                PreflightDimension::HostService,
                format!("Required host service {service:?} is unavailable"),
            ));
        }
    }
    Ok(())
}

fn failure(dimension: PreflightDimension, message: impl Into<String>) -> PreflightFailure {
    PreflightFailure::new(dimension, message)
}
