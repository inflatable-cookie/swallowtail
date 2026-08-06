#![deny(missing_docs)]

mod direct;

pub use direct::{
    DirectAttemptFinishObservation, DirectAttemptUsageObservation, ProviderFinishReason,
};

use crate::RuntimeTurnId;
use std::num::NonZeroU64;
use swallowtail_core::{AccessProfileId, ModelRouteId, ProviderRequestRef};

/// Provider-reported token counts with independent optional dimensions.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TokenUsage {
    input_tokens: Option<u64>,
    output_tokens: Option<u64>,
    reasoning_tokens: Option<u64>,
    cache_read_input_tokens: Option<u64>,
    cache_write_input_tokens: Option<u64>,
    cache_miss_input_tokens: Option<u64>,
}

impl TokenUsage {
    /// Creates usage from optional input and output counts.
    #[must_use]
    pub const fn new(input_tokens: Option<u64>, output_tokens: Option<u64>) -> Self {
        Self {
            input_tokens,
            output_tokens,
            reasoning_tokens: None,
            cache_read_input_tokens: None,
            cache_write_input_tokens: None,
            cache_miss_input_tokens: None,
        }
    }

    #[must_use]
    /// Adds optional cache-read and cache-write input counts.
    pub const fn with_cache_tokens(
        mut self,
        cache_read_input_tokens: Option<u64>,
        cache_write_input_tokens: Option<u64>,
    ) -> Self {
        self.cache_read_input_tokens = cache_read_input_tokens;
        self.cache_write_input_tokens = cache_write_input_tokens;
        self
    }

    #[must_use]
    /// Returns provider-reported input tokens.
    pub const fn input_tokens(&self) -> Option<u64> {
        self.input_tokens
    }

    #[must_use]
    /// Returns provider-reported output tokens.
    pub const fn output_tokens(&self) -> Option<u64> {
        self.output_tokens
    }

    #[must_use]
    /// Adds the optional provider-reported reasoning-token count.
    pub const fn with_reasoning_tokens(mut self, reasoning_tokens: Option<u64>) -> Self {
        self.reasoning_tokens = reasoning_tokens;
        self
    }

    #[must_use]
    /// Returns provider-reported reasoning tokens.
    pub const fn reasoning_tokens(&self) -> Option<u64> {
        self.reasoning_tokens
    }

    #[must_use]
    /// Returns provider-reported cache-read input tokens.
    pub const fn cache_read_input_tokens(&self) -> Option<u64> {
        self.cache_read_input_tokens
    }

    #[must_use]
    /// Returns provider-reported cache-write input tokens.
    pub const fn cache_write_input_tokens(&self) -> Option<u64> {
        self.cache_write_input_tokens
    }

    #[must_use]
    /// Adds the optional provider-reported cache-miss input count.
    pub const fn with_cache_miss_input_tokens(mut self, tokens: Option<u64>) -> Self {
        self.cache_miss_input_tokens = tokens;
        self
    }

    #[must_use]
    /// Returns provider-reported cache-miss input tokens.
    pub const fn cache_miss_input_tokens(&self) -> Option<u64> {
        self.cache_miss_input_tokens
    }

    /// Adds usage records known to describe disjoint provider work.
    #[must_use]
    pub fn checked_add_disjoint(self, other: Self) -> Option<Self> {
        Some(Self {
            input_tokens: checked_add_optional(self.input_tokens, other.input_tokens)?,
            output_tokens: checked_add_optional(self.output_tokens, other.output_tokens)?,
            reasoning_tokens: checked_add_optional(self.reasoning_tokens, other.reasoning_tokens)?,
            cache_read_input_tokens: checked_add_optional(
                self.cache_read_input_tokens,
                other.cache_read_input_tokens,
            )?,
            cache_write_input_tokens: checked_add_optional(
                self.cache_write_input_tokens,
                other.cache_write_input_tokens,
            )?,
            cache_miss_input_tokens: checked_add_optional(
                self.cache_miss_input_tokens,
                other.cache_miss_input_tokens,
            )?,
        })
    }
}

fn checked_add_optional(left: Option<u64>, right: Option<u64>) -> Option<Option<u64>> {
    match (left, right) {
        (Some(left), Some(right)) => left.checked_add(right).map(Some),
        (Some(value), None) | (None, Some(value)) => Some(Some(value)),
        (None, None) => Some(None),
    }
}

/// Unit constrained by one provider rate limit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RateLimitKind {
    /// Request count.
    Requests,
    /// Combined token count.
    Tokens,
    /// Input-token count.
    InputTokens,
    /// Output-token count.
    OutputTokens,
}

/// One provider-reported rate-limit snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RateLimitObservation {
    kind: RateLimitKind,
    limit: Option<u64>,
    remaining: Option<u64>,
    reset_after_milliseconds: Option<u64>,
}

impl RateLimitObservation {
    /// Creates a snapshot without inferring absent limit dimensions.
    #[must_use]
    pub const fn new(
        kind: RateLimitKind,
        limit: Option<u64>,
        remaining: Option<u64>,
        reset_after_milliseconds: Option<u64>,
    ) -> Self {
        Self {
            kind,
            limit,
            remaining,
            reset_after_milliseconds,
        }
    }

    #[must_use]
    /// Returns the constrained unit.
    pub const fn kind(&self) -> RateLimitKind {
        self.kind
    }

    #[must_use]
    /// Returns the maximum value when reported.
    pub const fn limit(&self) -> Option<u64> {
        self.limit
    }

    #[must_use]
    /// Returns the remaining value when reported.
    pub const fn remaining(&self) -> Option<u64> {
        self.remaining
    }

    #[must_use]
    /// Returns milliseconds until reset when reported.
    pub const fn reset_after_milliseconds(&self) -> Option<u64> {
        self.reset_after_milliseconds
    }
}

/// Provider-reported quota availability.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuotaState {
    /// The provider did not disclose quota state.
    Unknown,
    /// Quota is presently available.
    Available,
    /// Quota is exhausted.
    Exhausted,
    /// Account or policy restrictions prevent ordinary use.
    Restricted,
}

/// One provider-reported quota snapshot.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QuotaObservation {
    state: QuotaState,
}

impl QuotaObservation {
    /// Creates an observation from the exact reported state.
    #[must_use]
    pub const fn new(state: QuotaState) -> Self {
        Self { state }
    }

    #[must_use]
    /// Returns the quota state.
    pub const fn state(&self) -> QuotaState {
        self.state
    }
}

/// Currency used by provider-reported billed cost.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Currency {
    /// United States dollars.
    Usd,
}

/// Authority that supplied a billed-cost value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BilledCostSource {
    /// Supplied directly by the provider interface.
    ProviderReported,
}

/// Update semantics of a billed-cost observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BilledCostSemantics {
    /// Each value replaces the cumulative total for its exact attempt scope.
    CumulativeReplacement,
}

/// Provider-reported billed cost bound to one turn, route, access profile, and attempt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BilledCostObservation {
    amount: u64,
    currency: Currency,
    units_per_currency: NonZeroU64,
    source: BilledCostSource,
    semantics: BilledCostSemantics,
    turn_id: RuntimeTurnId,
    model_route_id: ModelRouteId,
    access_profile_id: AccessProfileId,
    provider_attempt: NonZeroU64,
}

impl BilledCostObservation {
    /// Creates a cumulative provider-reported cost observation.
    #[must_use]
    pub const fn provider_reported(
        amount: u64,
        currency: Currency,
        units_per_currency: NonZeroU64,
        turn_id: RuntimeTurnId,
        model_route_id: ModelRouteId,
        access_profile_id: AccessProfileId,
        provider_attempt: NonZeroU64,
    ) -> Self {
        Self {
            amount,
            currency,
            units_per_currency,
            source: BilledCostSource::ProviderReported,
            semantics: BilledCostSemantics::CumulativeReplacement,
            turn_id,
            model_route_id,
            access_profile_id,
            provider_attempt,
        }
    }

    #[must_use]
    /// Returns the integer amount in the declared sub-currency units.
    pub const fn amount(&self) -> u64 {
        self.amount
    }

    #[must_use]
    /// Returns the billed currency.
    pub const fn currency(&self) -> Currency {
        self.currency
    }

    #[must_use]
    /// Returns the number of integer units representing one currency unit.
    pub const fn units_per_currency(&self) -> NonZeroU64 {
        self.units_per_currency
    }

    #[must_use]
    /// Returns who supplied the cost.
    pub const fn source(&self) -> BilledCostSource {
        self.source
    }

    #[must_use]
    /// Returns how this observation updates prior cost state.
    pub const fn semantics(&self) -> BilledCostSemantics {
        self.semantics
    }

    #[must_use]
    /// Returns the exact runtime turn being billed.
    pub const fn turn_id(&self) -> &RuntimeTurnId {
        &self.turn_id
    }

    #[must_use]
    /// Returns the selected model route.
    pub const fn model_route_id(&self) -> &ModelRouteId {
        &self.model_route_id
    }

    #[must_use]
    /// Returns the selected access profile.
    pub const fn access_profile_id(&self) -> &AccessProfileId {
        &self.access_profile_id
    }

    #[must_use]
    /// Returns the provider-attempt ordinal within the turn.
    pub const fn provider_attempt(&self) -> NonZeroU64 {
        self.provider_attempt
    }
}

/// Portable provider metadata observed during an operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProviderObservation {
    /// Route-level token usage.
    Usage(TokenUsage),
    /// Token usage bound to one direct-inference attempt.
    DirectAttemptUsage(DirectAttemptUsageObservation),
    /// Finish reason bound to one direct-inference attempt.
    DirectAttemptFinish(DirectAttemptFinishObservation),
    /// Provider-reported billed cost.
    BilledCost(BilledCostObservation),
    /// Provider rate-limit state.
    RateLimit(RateLimitObservation),
    /// Provider quota state.
    Quota(QuotaObservation),
    /// Representation-aware correlation with a provider request.
    RequestCorrelation(ProviderRequestRef),
}

#[cfg(test)]
mod tests;
