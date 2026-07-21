use std::error::Error;
use std::fmt;
use std::num::NonZeroU32;
use swallowtail_core::{
    ExternalNetworkPolicy, ExternalSearchPolicy, HarnessIsolation, PreflightPlan, ReasoningMode,
    SafeDiagnostic,
};

/// Explicit policy selected for one operation. Catalog defaults do not populate it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationPolicy {
    external_network: ExternalNetworkPolicy,
    external_search: ExternalSearchPolicy,
    reasoning_mode: Option<ReasoningMode>,
    provider_execution: ProviderExecutionPolicy,
    provider_retention: ProviderRetentionPolicy,
    provider_recovery: ProviderRecoveryPolicy,
    stream_reattachment: StreamReattachmentPolicy,
    harness_isolation: Option<HarnessIsolation>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ProviderExecutionPolicy {
    #[default]
    Attached,
    Background,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ProviderRetentionPolicy {
    #[default]
    Prohibited,
    TemporaryAllowed,
    DurableAllowed,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ProviderRecoveryPolicy {
    #[default]
    Prohibited,
    ManagedAllowed,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum StreamReattachmentPolicy {
    #[default]
    Disabled,
    Bounded(NonZeroU32),
}

impl OperationPolicy {
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
            provider_execution: ProviderExecutionPolicy::Attached,
            provider_retention: ProviderRetentionPolicy::Prohibited,
            provider_recovery: ProviderRecoveryPolicy::Prohibited,
            stream_reattachment: StreamReattachmentPolicy::Disabled,
            harness_isolation: None,
        })
    }

    #[must_use]
    pub fn offline() -> Self {
        Self::new(
            ExternalNetworkPolicy::Denied,
            ExternalSearchPolicy::Disabled,
        )
        .expect("offline operation policy is internally valid")
    }

    #[must_use]
    pub fn with_reasoning_mode(mut self, reasoning_mode: ReasoningMode) -> Self {
        self.reasoning_mode = Some(reasoning_mode);
        self
    }

    #[must_use]
    pub const fn with_provider_execution(mut self, policy: ProviderExecutionPolicy) -> Self {
        self.provider_execution = policy;
        self
    }

    #[must_use]
    pub const fn with_provider_retention(mut self, policy: ProviderRetentionPolicy) -> Self {
        self.provider_retention = policy;
        self
    }

    #[must_use]
    pub const fn with_provider_recovery(mut self, policy: ProviderRecoveryPolicy) -> Self {
        self.provider_recovery = policy;
        self
    }

    #[must_use]
    pub const fn with_stream_reattachment(mut self, policy: StreamReattachmentPolicy) -> Self {
        self.stream_reattachment = policy;
        self
    }

    #[must_use]
    pub const fn with_harness_isolation(mut self, isolation: HarnessIsolation) -> Self {
        self.harness_isolation = Some(isolation);
        self
    }

    #[must_use]
    pub const fn external_network(&self) -> ExternalNetworkPolicy {
        self.external_network
    }

    #[must_use]
    pub const fn external_search(&self) -> ExternalSearchPolicy {
        self.external_search
    }

    #[must_use]
    pub const fn reasoning_mode(&self) -> Option<&ReasoningMode> {
        self.reasoning_mode.as_ref()
    }

    #[must_use]
    pub const fn provider_execution(&self) -> ProviderExecutionPolicy {
        self.provider_execution
    }

    #[must_use]
    pub const fn provider_retention(&self) -> ProviderRetentionPolicy {
        self.provider_retention
    }

    #[must_use]
    pub const fn provider_recovery(&self) -> ProviderRecoveryPolicy {
        self.provider_recovery
    }

    #[must_use]
    pub const fn stream_reattachment(&self) -> StreamReattachmentPolicy {
        self.stream_reattachment
    }

    #[must_use]
    pub const fn harness_isolation(&self) -> Option<HarnessIsolation> {
        self.harness_isolation
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

    #[must_use]
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
