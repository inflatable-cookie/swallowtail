use crate::access::{AccessProfile, AccessStatus};
use crate::diagnostic::SafeDiagnostic;
use crate::instance::{ConfiguredInstance, ModelRoute};
use crate::registration::DriverDescriptor;
use crate::requirement::OperationRequirements;
use crate::runtime_identity::{
    AccessProfileId, ConfiguredInstanceId, ExecutionHostId, HostServiceKind, InstanceOwnership,
    InstanceRevision, InstanceTargetRef, ModelRouteId,
};
use crate::{
    AdapterIdentity, Capability, CapabilityConstraint, ModelId, ResourceAccess,
    ResourceRepresentation, SessionAccessPolicy,
};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

pub struct PreflightContext<'a> {
    driver: &'a DriverDescriptor,
    instance: &'a ConfiguredInstance,
    model_route: Option<&'a ModelRoute>,
    access_profile: &'a AccessProfile,
    access_status: &'a AccessStatus,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl<'a> PreflightContext<'a> {
    #[must_use]
    pub fn new(
        driver: &'a DriverDescriptor,
        instance: &'a ConfiguredInstance,
        access_profile: &'a AccessProfile,
        access_status: &'a AccessStatus,
        available_host_services: impl IntoIterator<Item = HostServiceKind>,
    ) -> Self {
        Self {
            driver,
            instance,
            model_route: None,
            access_profile,
            access_status,
            available_host_services: available_host_services.into_iter().collect(),
        }
    }

    #[must_use]
    pub const fn with_model_route(mut self, model_route: &'a ModelRoute) -> Self {
        self.model_route = Some(model_route);
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreflightDimension {
    Driver,
    Instance,
    Role,
    ExecutionLayer,
    OperationShape,
    ModelRoute,
    Access,
    SupportAuthority,
    Ownership,
    Topology,
    HostService,
    Capability,
    Constraint,
    Extension,
    SessionAccess,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreflightFailure {
    dimension: PreflightDimension,
    diagnostic: SafeDiagnostic,
}

impl PreflightFailure {
    fn new(dimension: PreflightDimension, message: impl Into<String>) -> Self {
        Self {
            dimension,
            diagnostic: SafeDiagnostic::new("swallowtail.preflight_rejected", message),
        }
    }

    #[must_use]
    pub const fn dimension(&self) -> PreflightDimension {
        self.dimension
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for PreflightFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for PreflightFailure {}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PlanBinding {
    driver: DriverDescriptor,
    instance: ConfiguredInstance,
    model_route: Option<ModelRoute>,
    access_profile: AccessProfile,
    access_status: AccessStatus,
}

impl PlanBinding {
    fn from_context(context: &PreflightContext<'_>) -> Self {
        Self {
            driver: context.driver.clone(),
            instance: context.instance.clone(),
            model_route: context.model_route.cloned(),
            access_profile: context.access_profile.clone(),
            access_status: context.access_status.clone(),
        }
    }
}

/// Immutable evidence that one explicit selection passed preflight.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreflightPlan {
    binding: PlanBinding,
    requirements: OperationRequirements,
}

impl PreflightPlan {
    #[must_use]
    pub const fn driver_identity(&self) -> &AdapterIdentity {
        self.binding.driver.identity()
    }

    #[must_use]
    pub const fn instance_id(&self) -> &ConfiguredInstanceId {
        self.binding.instance.id()
    }

    #[must_use]
    pub const fn instance_revision(&self) -> &InstanceRevision {
        self.binding.instance.revision()
    }

    #[must_use]
    pub const fn instance_target_ref(&self) -> &InstanceTargetRef {
        self.binding.instance.target_reference()
    }

    #[must_use]
    pub fn model_route_id(&self) -> Option<&ModelRouteId> {
        self.binding.model_route.as_ref().map(ModelRoute::id)
    }

    #[must_use]
    pub fn model_id(&self) -> Option<&ModelId> {
        self.binding.model_route.as_ref().map(ModelRoute::model_id)
    }

    #[must_use]
    pub const fn access_profile_id(&self) -> &AccessProfileId {
        self.binding.access_profile.id()
    }

    #[must_use]
    pub const fn ownership(&self) -> InstanceOwnership {
        self.binding.instance.ownership()
    }

    #[must_use]
    pub const fn execution_host_id(&self) -> &ExecutionHostId {
        self.binding.instance.execution_host_id()
    }

    #[must_use]
    pub const fn requirements(&self) -> &OperationRequirements {
        &self.requirements
    }

    /// Rejects execution if a material preflight binding changed.
    pub fn validate_current(
        &self,
        context: &PreflightContext<'_>,
    ) -> Result<(), StalePreflightPlan> {
        validate(context, &self.requirements).map_err(StalePreflightPlan::preflight_failed)?;
        let current = PlanBinding::from_context(context);
        if current == self.binding {
            Ok(())
        } else {
            Err(StalePreflightPlan::binding_changed())
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StalePreflightPlan {
    diagnostic: SafeDiagnostic,
}

impl StalePreflightPlan {
    fn binding_changed() -> Self {
        Self {
            diagnostic: SafeDiagnostic::new(
                "swallowtail.preflight_plan_stale",
                "A material preflight binding changed",
            ),
        }
    }

    fn preflight_failed(failure: PreflightFailure) -> Self {
        Self {
            diagnostic: SafeDiagnostic::new(
                "swallowtail.preflight_plan_stale",
                format!(
                    "Current state no longer satisfies {:?} preflight requirements",
                    failure.dimension()
                ),
            ),
        }
    }

    #[must_use]
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for StalePreflightPlan {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for StalePreflightPlan {}

pub fn preflight(
    context: &PreflightContext<'_>,
    requirements: &OperationRequirements,
) -> Result<PreflightPlan, PreflightFailure> {
    validate(context, requirements)?;
    Ok(PreflightPlan {
        binding: PlanBinding::from_context(context),
        requirements: requirements.clone(),
    })
}

fn validate(
    context: &PreflightContext<'_>,
    requirements: &OperationRequirements,
) -> Result<(), PreflightFailure> {
    if context.instance.driver_id() != context.driver.identity().id() {
        return Err(PreflightFailure::new(
            PreflightDimension::Driver,
            "Configured instance does not use the selected driver",
        ));
    }
    if !context.driver.supports_role(requirements.driver_role()) {
        return Err(PreflightFailure::new(
            PreflightDimension::Role,
            "Selected driver does not implement the required role",
        ));
    }
    if !context
        .driver
        .supports_execution_layer(requirements.execution_layer())
    {
        return Err(PreflightFailure::new(
            PreflightDimension::ExecutionLayer,
            "Selected driver does not support the required execution layer",
        ));
    }
    if !context
        .driver
        .supports_operation_shape(requirements.operation_shape())
    {
        return Err(PreflightFailure::new(
            PreflightDimension::OperationShape,
            "Selected driver does not support the required operation shape",
        ));
    }
    if context.instance.execution_host_id() != requirements.execution_host_id() {
        return Err(PreflightFailure::new(
            PreflightDimension::Topology,
            "Configured instance is placed on a different execution host",
        ));
    }
    if !requirements.accepts_ownership(context.instance.ownership()) {
        return Err(PreflightFailure::new(
            PreflightDimension::Ownership,
            "Configured instance ownership is not accepted",
        ));
    }

    validate_access(context, requirements)?;
    validate_route(context, requirements)?;
    validate_host_services(context, requirements)?;
    validate_session_access(requirements)?;
    validate_capabilities(context, requirements)?;

    for namespace in requirements.extension_namespaces() {
        if !context.driver.supports_extension(namespace) {
            return Err(PreflightFailure::new(
                PreflightDimension::Extension,
                format!("Required extension '{}' is unsupported", namespace.as_str()),
            ));
        }
    }

    Ok(())
}

fn validate_session_access(requirements: &OperationRequirements) -> Result<(), PreflightFailure> {
    match (
        requirements.operation_shape(),
        requirements.session_access_policy(),
    ) {
        (crate::OperationShape::InteractiveSession, Some(policy)) => {
            validate_session_resource_access(requirements, policy)?;
            validate_session_network(requirements, policy)?;
            validate_session_provider_requests(requirements, policy)?;
        }
        (crate::OperationShape::InteractiveSession, None) => {
            return Err(session_access_failure(
                "Interactive session access policy is missing",
            ));
        }
        (_, Some(_)) => {
            return Err(session_access_failure(
                "Session access policy is bound to a non-interactive operation",
            ));
        }
        (_, None) => {}
    }
    Ok(())
}

fn validate_session_resource_access(
    requirements: &OperationRequirements,
    policy: &SessionAccessPolicy,
) -> Result<(), PreflightFailure> {
    let capability = capability_requirement(requirements, Capability::WorkingResource);
    match policy.resource_access() {
        ResourceAccess::Read if capability.is_none() => Ok(()),
        access => {
            let capability = capability.ok_or_else(|| {
                session_access_failure("Session working-resource access is not capability-bound")
            })?;
            let mut access_constraints = capability.constraints().filter_map(|constraint| {
                if let CapabilityConstraint::ResourceAccess(value) = constraint {
                    Some(*value)
                } else {
                    None
                }
            });
            if access_constraints.next() != Some(access) || access_constraints.next().is_some() {
                return Err(session_access_failure(
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
                return Err(session_access_failure(
                    "Session working resource is not bound to one filesystem representation",
                ));
            }
            require_session_host_service(
                requirements,
                HostServiceKind::WorkingResource,
                "Session working-resource access requires a working-resource service",
            )
        }
    }?;

    if policy.resource_access() == ResourceAccess::ReadWrite
        && capability_requirement(requirements, Capability::ToolCalls).is_some()
    {
        return Err(session_access_failure(
            "Bounded writable sessions cannot declare consumer tools",
        ));
    }
    Ok(())
}

fn validate_session_network(
    requirements: &OperationRequirements,
    policy: &SessionAccessPolicy,
) -> Result<(), PreflightFailure> {
    let network_required =
        capability_requirement(requirements, Capability::ProviderExternalNetwork).is_some();
    let network_enabled = policy.external_network() == crate::ExternalNetworkPolicy::HostApproved;
    if network_required != network_enabled {
        return Err(session_access_failure(
            "Session provider-network policy does not match its capability requirement",
        ));
    }
    if network_enabled {
        require_session_host_service(
            requirements,
            HostServiceKind::Network,
            "Host-approved provider network requires a network service",
        )?;
    }

    let search_required =
        capability_requirement(requirements, Capability::ExternalSearch).is_some();
    let search_enabled = policy.external_search() == crate::ExternalSearchPolicy::Enabled;
    if search_required != search_enabled {
        return Err(session_access_failure(
            "Session external-search policy does not match its capability requirement",
        ));
    }
    Ok(())
}

fn validate_session_provider_requests(
    requirements: &OperationRequirements,
    policy: &SessionAccessPolicy,
) -> Result<(), PreflightFailure> {
    for namespace in policy.provider_requests().observed_extensions() {
        if !requirements
            .extension_namespaces()
            .any(|required| required == namespace)
        {
            return Err(session_access_failure(format!(
                "Observed provider request extension '{}' is not preflight-bound",
                namespace.as_str()
            )));
        }
    }
    Ok(())
}

fn require_session_host_service(
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
        Err(session_access_failure(message))
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

fn session_access_failure(message: impl Into<String>) -> PreflightFailure {
    PreflightFailure::new(PreflightDimension::SessionAccess, message)
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
        return Err(PreflightFailure::new(
            PreflightDimension::Access,
            "Selected access profile does not match the instance and access status",
        ));
    }
    if context.instance.support_authority() != context.access_profile.support_authority()
        || context.access_status.support_authority() != context.access_profile.support_authority()
        || !required.accepts_support_authority(context.access_profile.support_authority())
    {
        return Err(PreflightFailure::new(
            PreflightDimension::SupportAuthority,
            "Selected support authority is not accepted",
        ));
    }
    if !required.accepts_credential(context.access_status.credential())
        || !required.accepts_entitlement(context.access_status.entitlement())
        || !required.accepts_endpoint_authorization(context.access_status.endpoint_authorization())
        || !required.accepts_runtime_readiness(context.access_status.runtime_readiness())
    {
        return Err(PreflightFailure::new(
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
        return Err(PreflightFailure::new(
            PreflightDimension::ModelRoute,
            "Operation requires an explicit model route",
        ));
    }
    if let Some(route) = context.model_route
        && route.instance_id() != context.instance.id()
    {
        return Err(PreflightFailure::new(
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
            return Err(PreflightFailure::new(
                PreflightDimension::HostService,
                format!("Required host service {service:?} is unavailable"),
            ));
        }
    }
    Ok(())
}

fn validate_capabilities(
    context: &PreflightContext<'_>,
    requirements: &OperationRequirements,
) -> Result<(), PreflightFailure> {
    for requirement in requirements.capabilities() {
        let capability = requirement.capability();
        validate_capability_profile(context.instance.capabilities(), capability, requirement)?;
        if let Some(route) = context.model_route {
            validate_capability_profile(route.capabilities(), capability, requirement)?;
        }
    }
    Ok(())
}

fn validate_capability_profile(
    profile: &crate::requirement::CapabilityProfile,
    capability: Capability,
    requirement: &crate::requirement::CapabilityRequirement,
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
