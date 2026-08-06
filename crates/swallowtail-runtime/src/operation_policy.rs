#![deny(missing_docs)]

use std::error::Error;
use std::fmt;
use std::num::NonZeroU32;
use swallowtail_core::{
    AttachedRuntimeResidency, ExternalNetworkPolicy, ExternalSearchPolicy,
    HarnessConfigurationPosture, HarnessIsolation, HarnessMode, PreflightPlan, ReasoningMode,
    SafeDiagnostic,
};

mod harness_configuration;

pub use harness_configuration::validate_harness_configuration_policy;

/// Explicit policy selected for one operation. Catalog defaults do not populate it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationPolicy {
    external_network: ExternalNetworkPolicy,
    external_search: ExternalSearchPolicy,
    reasoning_mode: Option<ReasoningMode>,
    harness_mode: Option<HarnessMode>,
    provider_execution: ProviderExecutionPolicy,
    provider_retention: ProviderRetentionPolicy,
    provider_recovery: ProviderRecoveryPolicy,
    stream_reattachment: StreamReattachmentPolicy,
    harness_isolation: Option<HarnessIsolation>,
    attached_runtime_residency: Option<AttachedRuntimeResidency>,
    harness_configuration_posture: Option<HarnessConfigurationPosture>,
}

/// Provider execution attachment posture selected before effects.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ProviderExecutionPolicy {
    /// Provider work remains attached to the initiating operation.
    #[default]
    Attached,
    /// Provider work may continue in provider-managed background execution.
    Background,
}

/// Provider-side retention posture selected before effects.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ProviderRetentionPolicy {
    /// Provider retention is not authorized.
    #[default]
    Prohibited,
    /// Temporary provider retention required by the selected operation is allowed.
    TemporaryAllowed,
    /// Durable provider retention is explicitly allowed.
    DurableAllowed,
}

/// Provider-managed recovery posture selected before effects.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ProviderRecoveryPolicy {
    /// Provider rescheduling or managed recovery is not authorized.
    #[default]
    Prohibited,
    /// Qualified provider-managed recovery is explicitly allowed.
    ManagedAllowed,
}

/// Bound on reattaching to one provider-owned event stream.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum StreamReattachmentPolicy {
    /// Stream reattachment is not authorized.
    #[default]
    Disabled,
    /// At most the contained nonzero number of reattachments is authorized.
    Bounded(NonZeroU32),
}

impl OperationPolicy {
    /// Creates an explicit network and search policy with all optional authority disabled.
    pub fn new(
        external_network: ExternalNetworkPolicy,
        external_search: ExternalSearchPolicy,
    ) -> Result<Self, IncompatibleOperationPolicy> {
        if external_network == ExternalNetworkPolicy::AmbientHost {
            return Err(IncompatibleOperationPolicy::ambient_network_requires_harness());
        }
        if external_search == ExternalSearchPolicy::Enabled
            && external_network == ExternalNetworkPolicy::Denied
        {
            return Err(IncompatibleOperationPolicy::search_requires_network());
        }
        Ok(Self {
            external_network,
            external_search,
            reasoning_mode: None,
            harness_mode: None,
            provider_execution: ProviderExecutionPolicy::Attached,
            provider_retention: ProviderRetentionPolicy::Prohibited,
            provider_recovery: ProviderRecoveryPolicy::Prohibited,
            stream_reattachment: StreamReattachmentPolicy::Disabled,
            harness_isolation: None,
            attached_runtime_residency: None,
            harness_configuration_posture: None,
        })
    }

    #[must_use]
    /// Creates an operation policy with provider-side network and search disabled.
    pub fn offline() -> Self {
        Self::new(
            ExternalNetworkPolicy::Denied,
            ExternalSearchPolicy::Disabled,
        )
        .expect("offline operation policy is internally valid")
    }

    #[must_use]
    /// Selects one exact reasoning mode.
    pub fn with_reasoning_mode(mut self, reasoning_mode: ReasoningMode) -> Self {
        self.reasoning_mode = Some(reasoning_mode);
        self
    }

    #[must_use]
    /// Selects one exact harness behavioral mode.
    pub const fn with_harness_mode(mut self, harness_mode: HarnessMode) -> Self {
        self.harness_mode = Some(harness_mode);
        self
    }

    #[must_use]
    /// Selects attached or provider-background execution.
    pub const fn with_provider_execution(mut self, policy: ProviderExecutionPolicy) -> Self {
        self.provider_execution = policy;
        self
    }

    #[must_use]
    /// Selects the provider-side retention posture.
    pub const fn with_provider_retention(mut self, policy: ProviderRetentionPolicy) -> Self {
        self.provider_retention = policy;
        self
    }

    #[must_use]
    /// Selects the provider-managed recovery posture.
    pub const fn with_provider_recovery(mut self, policy: ProviderRecoveryPolicy) -> Self {
        self.provider_recovery = policy;
        self
    }

    #[must_use]
    /// Selects whether bounded stream reattachment is authorized.
    pub const fn with_stream_reattachment(mut self, policy: StreamReattachmentPolicy) -> Self {
        self.stream_reattachment = policy;
        self
    }

    #[must_use]
    /// Selects one exact harness-isolation posture.
    pub const fn with_harness_isolation(mut self, isolation: HarnessIsolation) -> Self {
        self.harness_isolation = Some(isolation);
        self
    }

    #[must_use]
    /// Selects the attached-runtime residency posture.
    pub const fn with_attached_runtime_residency(
        mut self,
        residency: AttachedRuntimeResidency,
    ) -> Self {
        self.attached_runtime_residency = Some(residency);
        self
    }

    #[must_use]
    /// Selects how provider harness configuration is inherited or suppressed.
    pub const fn with_harness_configuration_posture(
        mut self,
        posture: HarnessConfigurationPosture,
    ) -> Self {
        self.harness_configuration_posture = Some(posture);
        self
    }

    #[must_use]
    /// Returns the provider-side external-network policy.
    pub const fn external_network(&self) -> ExternalNetworkPolicy {
        self.external_network
    }

    #[must_use]
    /// Returns the provider-side external-search policy.
    pub const fn external_search(&self) -> ExternalSearchPolicy {
        self.external_search
    }

    #[must_use]
    /// Returns the exact reasoning selection, when any.
    pub const fn reasoning_mode(&self) -> Option<&ReasoningMode> {
        self.reasoning_mode.as_ref()
    }

    #[must_use]
    /// Returns the exact harness mode, when any.
    pub const fn harness_mode(&self) -> Option<HarnessMode> {
        self.harness_mode
    }

    #[must_use]
    /// Returns the provider execution posture.
    pub const fn provider_execution(&self) -> ProviderExecutionPolicy {
        self.provider_execution
    }

    #[must_use]
    /// Returns the provider retention posture.
    pub const fn provider_retention(&self) -> ProviderRetentionPolicy {
        self.provider_retention
    }

    #[must_use]
    /// Returns the provider-managed recovery posture.
    pub const fn provider_recovery(&self) -> ProviderRecoveryPolicy {
        self.provider_recovery
    }

    #[must_use]
    /// Returns the stream reattachment bound.
    pub const fn stream_reattachment(&self) -> StreamReattachmentPolicy {
        self.stream_reattachment
    }

    #[must_use]
    /// Returns the selected harness-isolation posture, when any.
    pub const fn harness_isolation(&self) -> Option<HarnessIsolation> {
        self.harness_isolation
    }

    #[must_use]
    /// Returns the attached-runtime residency posture, when applicable.
    pub const fn attached_runtime_residency(&self) -> Option<AttachedRuntimeResidency> {
        self.attached_runtime_residency
    }

    #[must_use]
    /// Returns the harness configuration posture, when applicable.
    pub const fn harness_configuration_posture(&self) -> Option<HarnessConfigurationPosture> {
        self.harness_configuration_posture
    }
}

/// Compares the request posture with its pure preflight binding.
pub fn validate_harness_isolation_policy(
    plan: &PreflightPlan,
    policy: &OperationPolicy,
) -> Result<(), IncompatibleOperationPolicy> {
    if plan.requirements().harness_isolation() == policy.harness_isolation() {
        Ok(())
    } else {
        Err(IncompatibleOperationPolicy::harness_isolation_mismatch())
    }
}

/// Rejects requests that omit or change the preflight-bound residency posture.
pub fn validate_attached_runtime_residency_policy(
    plan: &PreflightPlan,
    policy: &OperationPolicy,
) -> Result<(), IncompatibleOperationPolicy> {
    let required = plan
        .requirements()
        .attached_runtime()
        .map(swallowtail_core::AttachedRuntimeRequirements::residency);
    if required == policy.attached_runtime_residency() {
        Ok(())
    } else {
        Err(IncompatibleOperationPolicy::attached_runtime_residency_mismatch())
    }
}

/// Safe failure returned when operation policy is internally inconsistent or
/// disagrees with immutable preflight requirements.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IncompatibleOperationPolicy {
    diagnostic: SafeDiagnostic,
}

impl IncompatibleOperationPolicy {
    fn search_requires_network() -> Self {
        Self {
            diagnostic: SafeDiagnostic::new(
                "swallowtail.operation_policy_rejected",
                "External search requires host-approved external network access",
            ),
        }
    }

    fn ambient_network_requires_harness() -> Self {
        Self {
            diagnostic: SafeDiagnostic::new(
                "swallowtail.operation_policy_rejected",
                "Ambient host network authority is valid only for a harness session",
            ),
        }
    }

    fn harness_isolation_mismatch() -> Self {
        Self {
            diagnostic: SafeDiagnostic::new(
                "swallowtail.operation_policy_rejected",
                "Harness isolation does not match the preflight-bound posture",
            ),
        }
    }

    fn attached_runtime_residency_mismatch() -> Self {
        Self {
            diagnostic: SafeDiagnostic::new(
                "swallowtail.operation_policy_rejected",
                "Attached-runtime residency does not match the preflight-bound posture",
            ),
        }
    }

    #[must_use]
    /// Returns the bounded, redacted policy diagnostic.
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.diagnostic
    }
}

impl fmt::Display for IncompatibleOperationPolicy {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.diagnostic.fmt(formatter)
    }
}

impl Error for IncompatibleOperationPolicy {}

#[cfg(test)]
mod tests;
