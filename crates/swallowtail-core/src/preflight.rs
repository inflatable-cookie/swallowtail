use crate::ModelArtifactBinding;
use crate::access::{AccessProfile, AccessStatus};
use crate::diagnostic::SafeDiagnostic;
use crate::instance::{ConfiguredInstance, ModelRoute};
use crate::registration::DriverDescriptor;
use crate::requirement::OperationRequirements;
use crate::runtime_identity::HostServiceKind;
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

mod artifact;
mod capability;
mod plan;
mod planned_connection_rollover;
mod realtime_media;
mod session_access;
mod session_provider_state;
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
    HarnessIsolation,
    SessionAccess,
    SessionProviderState,
    RealtimeMedia,
    PlannedConnectionRollover,
    InterfaceVersion,
    HarnessRpcPolicy,
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
