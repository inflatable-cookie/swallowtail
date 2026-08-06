use crate::{Capability, CapabilityConstraint, CapabilityRequirement, ModelId};
use std::num::{NonZeroU32, NonZeroU64};

/// Wire shape fixed for one provider inference attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DirectAttemptTransport {
    /// One buffered response body.
    Buffered,
    /// Incremental server-sent event stream.
    ServerSentEvents,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Provider tool-selection behavior fixed for direct continuation.
pub enum DirectToolSelection {
    /// Provider selects among declared tools automatically.
    ProviderAutomatic,
}

/// Provider-managed inference cache posture accepted by the consumer.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ProviderInferenceCachePolicy {
    /// Provider-managed inference caching is not accepted.
    #[default]
    Prohibited,
    /// Provider caching is accepted without inspect or mutation authority.
    AcceptedWithoutManagementAuthority,
}

impl ProviderInferenceCachePolicy {
    #[must_use]
    /// Reports whether provider-managed caching may occur.
    pub const fn accepts_provider_cache(self) -> bool {
        matches!(self, Self::AcceptedWithoutManagementAuthority)
    }

    #[must_use]
    /// Reports whether cache management authority is granted; always false.
    pub const fn permits_management(self) -> bool {
        false
    }
}

/// Positive bounds and fixed transport choices for one locally continued session.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectContinuationConfig {
    maximum_user_turns: NonZeroU32,
    maximum_inference_attempts: NonZeroU32,
    maximum_declared_tools: NonZeroU32,
    maximum_returned_tool_calls: NonZeroU32,
    maximum_tool_argument_bytes: NonZeroU64,
    maximum_tool_result_bytes: NonZeroU64,
    maximum_private_continuation_bytes: NonZeroU64,
    maximum_private_history_bytes: NonZeroU64,
    maximum_stream_records_per_attempt: NonZeroU32,
    maximum_output_tokens_per_attempt: NonZeroU64,
    initial_attempt_transport: DirectAttemptTransport,
    continued_attempt_transport: DirectAttemptTransport,
    tool_selection: DirectToolSelection,
    provider_cache_policy: ProviderInferenceCachePolicy,
}

impl DirectContinuationConfig {
    /// Creates exact positive continuation bounds and transport choices.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        maximum_user_turns: NonZeroU32,
        maximum_inference_attempts: NonZeroU32,
        maximum_declared_tools: NonZeroU32,
        maximum_returned_tool_calls: NonZeroU32,
        maximum_tool_argument_bytes: NonZeroU64,
        maximum_tool_result_bytes: NonZeroU64,
        maximum_private_continuation_bytes: NonZeroU64,
        maximum_private_history_bytes: NonZeroU64,
        maximum_stream_records_per_attempt: NonZeroU32,
        maximum_output_tokens_per_attempt: NonZeroU64,
        initial_attempt_transport: DirectAttemptTransport,
        continued_attempt_transport: DirectAttemptTransport,
        tool_selection: DirectToolSelection,
        provider_cache_policy: ProviderInferenceCachePolicy,
    ) -> Self {
        Self {
            maximum_user_turns,
            maximum_inference_attempts,
            maximum_declared_tools,
            maximum_returned_tool_calls,
            maximum_tool_argument_bytes,
            maximum_tool_result_bytes,
            maximum_private_continuation_bytes,
            maximum_private_history_bytes,
            maximum_stream_records_per_attempt,
            maximum_output_tokens_per_attempt,
            initial_attempt_transport,
            continued_attempt_transport,
            tool_selection,
            provider_cache_policy,
        }
    }

    #[must_use]
    /// Returns the maximum user turns.
    pub const fn maximum_user_turns(&self) -> NonZeroU32 {
        self.maximum_user_turns
    }

    #[must_use]
    /// Returns the maximum provider inference attempts.
    pub const fn maximum_inference_attempts(&self) -> NonZeroU32 {
        self.maximum_inference_attempts
    }

    #[must_use]
    /// Returns the maximum tools declared to the provider.
    pub const fn maximum_declared_tools(&self) -> NonZeroU32 {
        self.maximum_declared_tools
    }

    #[must_use]
    /// Returns the maximum tool calls accepted from responses.
    pub const fn maximum_returned_tool_calls(&self) -> NonZeroU32 {
        self.maximum_returned_tool_calls
    }

    #[must_use]
    /// Returns the maximum encoded tool-argument bytes.
    pub const fn maximum_tool_argument_bytes(&self) -> NonZeroU64 {
        self.maximum_tool_argument_bytes
    }

    #[must_use]
    /// Returns the maximum encoded tool-result bytes.
    pub const fn maximum_tool_result_bytes(&self) -> NonZeroU64 {
        self.maximum_tool_result_bytes
    }

    #[must_use]
    /// Returns the maximum private continuation-state bytes.
    pub const fn maximum_private_continuation_bytes(&self) -> NonZeroU64 {
        self.maximum_private_continuation_bytes
    }

    #[must_use]
    /// Returns the maximum private history bytes.
    pub const fn maximum_private_history_bytes(&self) -> NonZeroU64 {
        self.maximum_private_history_bytes
    }

    #[must_use]
    /// Returns the maximum stream records retained per attempt.
    pub const fn maximum_stream_records_per_attempt(&self) -> NonZeroU32 {
        self.maximum_stream_records_per_attempt
    }

    #[must_use]
    /// Returns the maximum output tokens requested per attempt.
    pub const fn maximum_output_tokens_per_attempt(&self) -> NonZeroU64 {
        self.maximum_output_tokens_per_attempt
    }

    #[must_use]
    /// Returns transport used for the first inference attempt.
    pub const fn initial_attempt_transport(&self) -> DirectAttemptTransport {
        self.initial_attempt_transport
    }

    #[must_use]
    /// Returns transport used after a tool result.
    pub const fn continued_attempt_transport(&self) -> DirectAttemptTransport {
        self.continued_attempt_transport
    }

    #[must_use]
    /// Returns provider tool-selection behavior.
    pub const fn tool_selection(&self) -> DirectToolSelection {
        self.tool_selection
    }

    #[must_use]
    /// Returns accepted provider cache posture.
    pub const fn provider_cache_policy(&self) -> ProviderInferenceCachePolicy {
        self.provider_cache_policy
    }

    #[must_use]
    /// Projects configuration into exact capability requirements.
    pub fn capability_requirements(&self) -> Vec<CapabilityRequirement> {
        let mut requirements = vec![CapabilityRequirement::new(
            Capability::DirectToolContinuation,
            [
                CapabilityConstraint::MaximumTurns(self.maximum_user_turns.get()),
                CapabilityConstraint::MaximumInferenceAttempts(
                    self.maximum_inference_attempts.get(),
                ),
                CapabilityConstraint::ToolMaximumCount(self.maximum_declared_tools.get()),
                CapabilityConstraint::MaximumToolCalls(self.maximum_returned_tool_calls.get()),
                CapabilityConstraint::ToolArgumentMaximumBytes(
                    self.maximum_tool_argument_bytes.get(),
                ),
                CapabilityConstraint::ToolResultMaximumBytes(self.maximum_tool_result_bytes.get()),
                CapabilityConstraint::PrivateContinuationMaximumBytes(
                    self.maximum_private_continuation_bytes.get(),
                ),
                CapabilityConstraint::PrivateHistoryMaximumBytes(
                    self.maximum_private_history_bytes.get(),
                ),
                CapabilityConstraint::StreamRecordMaximumCount(
                    self.maximum_stream_records_per_attempt.get(),
                ),
                CapabilityConstraint::OutputTokenMaximum(
                    self.maximum_output_tokens_per_attempt.get(),
                ),
            ],
        )];
        if self.provider_cache_policy.accepts_provider_cache() {
            requirements.push(CapabilityRequirement::new(
                Capability::ProviderManagedInferenceCache,
                [],
            ));
        }
        requirements
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Model identity and bounds required for locally continued direct inference.
pub struct DirectContinuationRequirements {
    model_id: ModelId,
    config: DirectContinuationConfig,
}

impl DirectContinuationRequirements {
    #[must_use]
    /// Creates requirements for one exact model and configuration.
    pub const fn new(model_id: ModelId, config: DirectContinuationConfig) -> Self {
        Self { model_id, config }
    }

    #[must_use]
    /// Returns the required model identity.
    pub const fn model_id(&self) -> &ModelId {
        &self.model_id
    }

    #[must_use]
    /// Returns exact continuation configuration.
    pub const fn config(&self) -> &DirectContinuationConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DirectAttemptTransport, DirectContinuationConfig, DirectToolSelection,
        ProviderInferenceCachePolicy,
    };
    use crate::Capability;
    use std::num::{NonZeroU32, NonZeroU64};

    #[test]
    fn bounds_are_positive_and_cache_acceptance_grants_no_management() {
        assert!(NonZeroU32::new(0).is_none());
        let config = DirectContinuationConfig::new(
            NonZeroU32::new(2).unwrap(),
            NonZeroU32::new(3).unwrap(),
            NonZeroU32::new(8).unwrap(),
            NonZeroU32::new(1).unwrap(),
            NonZeroU64::new(65_536).unwrap(),
            NonZeroU64::new(65_536).unwrap(),
            NonZeroU64::new(262_144).unwrap(),
            NonZeroU64::new(1_048_576).unwrap(),
            NonZeroU32::new(4_096).unwrap(),
            NonZeroU64::new(8_192).unwrap(),
            DirectAttemptTransport::Buffered,
            DirectAttemptTransport::ServerSentEvents,
            DirectToolSelection::ProviderAutomatic,
            ProviderInferenceCachePolicy::AcceptedWithoutManagementAuthority,
        );

        assert!(config.provider_cache_policy().accepts_provider_cache());
        assert!(!config.provider_cache_policy().permits_management());
        assert!(config.capability_requirements().iter().any(
            |requirement| requirement.capability() == Capability::ProviderManagedInferenceCache
        ));
    }
}
