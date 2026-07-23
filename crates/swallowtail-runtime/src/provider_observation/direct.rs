use super::TokenUsage;
use crate::DirectInferenceAttemptId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectAttemptUsageObservation {
    attempt_id: DirectInferenceAttemptId,
    usage: TokenUsage,
}

impl DirectAttemptUsageObservation {
    #[must_use]
    pub const fn new(attempt_id: DirectInferenceAttemptId, usage: TokenUsage) -> Self {
        Self { attempt_id, usage }
    }

    #[must_use]
    pub const fn attempt_id(&self) -> &DirectInferenceAttemptId {
        &self.attempt_id
    }

    #[must_use]
    pub const fn usage(&self) -> TokenUsage {
        self.usage
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderFinishReason {
    Stop,
    Length,
    ContentFiltered,
    InsufficientResources,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectAttemptFinishObservation {
    attempt_id: DirectInferenceAttemptId,
    reason: ProviderFinishReason,
}

impl DirectAttemptFinishObservation {
    #[must_use]
    pub const fn new(attempt_id: DirectInferenceAttemptId, reason: ProviderFinishReason) -> Self {
        Self { attempt_id, reason }
    }

    #[must_use]
    pub const fn attempt_id(&self) -> &DirectInferenceAttemptId {
        &self.attempt_id
    }

    #[must_use]
    pub const fn reason(&self) -> ProviderFinishReason {
        self.reason
    }
}
