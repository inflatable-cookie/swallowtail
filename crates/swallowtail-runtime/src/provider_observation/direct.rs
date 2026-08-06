use super::TokenUsage;
use crate::DirectInferenceAttemptId;

/// Token usage attributed to one direct-inference attempt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectAttemptUsageObservation {
    attempt_id: DirectInferenceAttemptId,
    usage: TokenUsage,
}

impl DirectAttemptUsageObservation {
    /// Creates usage evidence for the exact attempt identity.
    #[must_use]
    pub const fn new(attempt_id: DirectInferenceAttemptId, usage: TokenUsage) -> Self {
        Self { attempt_id, usage }
    }

    #[must_use]
    /// Returns the direct-inference attempt identity.
    pub const fn attempt_id(&self) -> &DirectInferenceAttemptId {
        &self.attempt_id
    }

    #[must_use]
    /// Returns the provider-reported token usage.
    pub const fn usage(&self) -> TokenUsage {
        self.usage
    }
}

/// Portable provider finish reason for a direct-inference attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderFinishReason {
    /// The provider reached a normal stop condition.
    Stop,
    /// The provider reached an output or context length bound.
    Length,
    /// Provider safety policy filtered the response.
    ContentFiltered,
    /// The provider could not allocate required capacity or quota.
    InsufficientResources,
}

/// Terminal finish metadata attributed to one direct-inference attempt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectAttemptFinishObservation {
    attempt_id: DirectInferenceAttemptId,
    reason: ProviderFinishReason,
}

impl DirectAttemptFinishObservation {
    /// Creates finish evidence for the exact attempt identity.
    #[must_use]
    pub const fn new(attempt_id: DirectInferenceAttemptId, reason: ProviderFinishReason) -> Self {
        Self { attempt_id, reason }
    }

    #[must_use]
    /// Returns the direct-inference attempt identity.
    pub const fn attempt_id(&self) -> &DirectInferenceAttemptId {
        &self.attempt_id
    }

    #[must_use]
    /// Returns the portable provider finish reason.
    pub const fn reason(&self) -> ProviderFinishReason {
        self.reason
    }
}
