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
    AdapterIdentity, CredentialMechanism, CredentialRef, EndpointAudience, ModelArtifactBinding,
    ModelId, ProviderId,
};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

mod artifact;
mod capability;
mod session_access;
mod validation;

use validation::validate;

pub struct PreflightContext<'a> {
    driver: &'a DriverDescriptor,
    instance: &'a ConfiguredInstance,
    model_route: Option<&'a ModelRoute>,
    model_artifact: Option<&'a ModelArtifactBinding>,
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
            model_artifact: None,
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
    ModelArtifact,
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
    model_artifact: Option<ModelArtifactBinding>,
    access_profile: AccessProfile,
    access_status: AccessStatus,
}

impl PlanBinding {
    fn from_context(context: &PreflightContext<'_>) -> Self {
        Self {
            driver: context.driver.clone(),
            instance: context.instance.clone(),
            model_route: context.model_route.cloned(),
            model_artifact: context.model_artifact.cloned(),
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
    pub fn provider_id(&self) -> Option<&ProviderId> {
        self.binding
            .model_route
            .as_ref()
            .and_then(ModelRoute::provider_id)
    }

    #[must_use]
    pub const fn access_profile_id(&self) -> &AccessProfileId {
        self.binding.access_profile.id()
    }

    #[must_use]
    pub const fn credential_mechanism(&self) -> &CredentialMechanism {
        self.binding.access_profile.credential_mechanism()
    }

    #[must_use]
    pub const fn credential_reference(&self) -> Option<&CredentialRef> {
        self.binding.access_profile.credential_reference()
    }

    #[must_use]
    pub const fn endpoint_audience(&self) -> &EndpointAudience {
        self.binding.access_profile.endpoint_audience()
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
