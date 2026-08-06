#![deny(missing_docs)]

use crate::access::{AccessProfile, AccessStatus};
use crate::diagnostic::SafeDiagnostic;
use crate::instance::{ConfiguredInstance, ModelRoute};
use crate::registration::DriverDescriptor;
use crate::requirement::OperationRequirements;
use crate::runtime_identity::HostServiceKind;
use crate::{AttachedModelObservation, ModelArtifactBinding};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

mod artifact;
mod attached_runtime;
mod capability;
mod direct_continuation;
mod harness_configuration;
mod plan;
mod planned_connection_rollover;
mod realtime_media;
mod session_access;
mod session_provider_state;
mod validation;

use validation::validate;

/// Borrowed provider, route, access, and host evidence evaluated before dispatch.
pub struct PreflightContext<'a> {
    driver: &'a DriverDescriptor,
    instance: &'a ConfiguredInstance,
    model_route: Option<&'a ModelRoute>,
    model_artifact: Option<&'a ModelArtifactBinding>,
    attached_model_observation: Option<&'a AttachedModelObservation>,
    access_profile: &'a AccessProfile,
    access_status: &'a AccessStatus,
    available_host_services: BTreeSet<HostServiceKind>,
}

impl<'a> PreflightContext<'a> {
    /// Creates a context without optional model or attached-runtime evidence.
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
            attached_model_observation: None,
            access_profile,
            access_status,
            available_host_services: available_host_services.into_iter().collect(),
        }
    }

    #[must_use]
    /// Binds the exact model route selected for the operation.
    pub const fn with_model_route(mut self, model_route: &'a ModelRoute) -> Self {
        self.model_route = Some(model_route);
        self
    }

    #[must_use]
    /// Binds observations from the exact attached model runtime.
    pub const fn with_attached_model_observation(
        mut self,
        observation: &'a AttachedModelObservation,
    ) -> Self {
        self.attached_model_observation = Some(observation);
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Admission dimension responsible for a preflight rejection.
pub enum PreflightDimension {
    /// Driver identity or descriptor mismatch.
    Driver,
    /// Configured provider instance mismatch.
    Instance,
    /// Required runtime role is absent.
    Role,
    /// Execution layer does not match the operation.
    ExecutionLayer,
    /// Operation shape is unsupported.
    OperationShape,
    /// Model route is absent or mismatched.
    ModelRoute,
    /// Model artifact is absent or mismatched.
    ModelArtifact,
    /// Access evidence does not meet the requirement.
    Access,
    /// Support authority does not meet the requirement.
    SupportAuthority,
    /// Configured-instance ownership is not admitted.
    Ownership,
    /// Execution host or target topology is inconsistent.
    Topology,
    /// Required host service is unavailable.
    HostService,
    /// Required capability is unavailable.
    Capability,
    /// Capability parameter is unavailable or mismatched.
    Constraint,
    /// Provider extension namespace is not admitted.
    Extension,
    /// Harness isolation does not meet the requirement.
    HarnessIsolation,
    /// Session resource or permission access is incompatible.
    SessionAccess,
    /// Session provider-state policy is incompatible.
    SessionProviderState,
    /// Realtime-media configuration is incompatible.
    RealtimeMedia,
    /// Planned connection rollover is incompatible.
    PlannedConnectionRollover,
    /// Direct tool-continuation requirements are incompatible.
    DirectContinuation,
    /// Attached-runtime evidence is incompatible.
    AttachedRuntime,
    /// Interface version is unsupported or unverified where forbidden.
    InterfaceVersion,
    /// Harness-RPC policy is incompatible.
    HarnessRpcPolicy,
    /// Harness-configuration posture is incompatible.
    HarnessConfiguration,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Safe pre-dispatch rejection tied to one admission dimension.
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
    /// Returns the admission dimension that rejected the operation.
    pub const fn dimension(&self) -> PreflightDimension {
        self.dimension
    }

    #[must_use]
    /// Returns the redacted preflight diagnostic.
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
    attached_model_observation: Option<AttachedModelObservation>,
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
            attached_model_observation: context.attached_model_observation.cloned(),
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
/// Rejection raised when current evidence no longer matches a prepared plan.
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
    /// Returns the redacted stale-plan diagnostic.
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

/// Validates exact requirements and freezes the successful selection as a plan.
///
/// This function performs no provider or host side effects.
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
