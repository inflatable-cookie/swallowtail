use crate::RuntimeTurnId;
use std::num::NonZeroU64;
use swallowtail_core::{AccessProfileId, ModelRouteId, ProviderRequestRef};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TokenUsage {
    input_tokens: Option<u64>,
    output_tokens: Option<u64>,
    cache_read_input_tokens: Option<u64>,
    cache_write_input_tokens: Option<u64>,
}

impl TokenUsage {
    #[must_use]
    pub const fn new(input_tokens: Option<u64>, output_tokens: Option<u64>) -> Self {
        Self {
            input_tokens,
            output_tokens,
            cache_read_input_tokens: None,
            cache_write_input_tokens: None,
        }
    }

    #[must_use]
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
    pub const fn input_tokens(&self) -> Option<u64> {
        self.input_tokens
    }

    #[must_use]
    pub const fn output_tokens(&self) -> Option<u64> {
        self.output_tokens
    }

    #[must_use]
    pub const fn cache_read_input_tokens(&self) -> Option<u64> {
        self.cache_read_input_tokens
    }

    #[must_use]
    pub const fn cache_write_input_tokens(&self) -> Option<u64> {
        self.cache_write_input_tokens
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RateLimitKind {
    Requests,
    InputTokens,
    OutputTokens,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RateLimitObservation {
    kind: RateLimitKind,
    limit: Option<u64>,
    remaining: Option<u64>,
    reset_after_milliseconds: Option<u64>,
}

impl RateLimitObservation {
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
    pub const fn kind(&self) -> RateLimitKind {
        self.kind
    }

    #[must_use]
    pub const fn limit(&self) -> Option<u64> {
        self.limit
    }

    #[must_use]
    pub const fn remaining(&self) -> Option<u64> {
        self.remaining
    }

    #[must_use]
    pub const fn reset_after_milliseconds(&self) -> Option<u64> {
        self.reset_after_milliseconds
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuotaState {
    Unknown,
    Available,
    Exhausted,
    Restricted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QuotaObservation {
    state: QuotaState,
}

impl QuotaObservation {
    #[must_use]
    pub const fn new(state: QuotaState) -> Self {
        Self { state }
    }

    #[must_use]
    pub const fn state(&self) -> QuotaState {
        self.state
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Currency {
    Usd,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BilledCostSource {
    ProviderReported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BilledCostSemantics {
    CumulativeReplacement,
}

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
    pub const fn amount(&self) -> u64 {
        self.amount
    }

    #[must_use]
    pub const fn currency(&self) -> Currency {
        self.currency
    }

    #[must_use]
    pub const fn units_per_currency(&self) -> NonZeroU64 {
        self.units_per_currency
    }

    #[must_use]
    pub const fn source(&self) -> BilledCostSource {
        self.source
    }

    #[must_use]
    pub const fn semantics(&self) -> BilledCostSemantics {
        self.semantics
    }

    #[must_use]
    pub const fn turn_id(&self) -> &RuntimeTurnId {
        &self.turn_id
    }

    #[must_use]
    pub const fn model_route_id(&self) -> &ModelRouteId {
        &self.model_route_id
    }

    #[must_use]
    pub const fn access_profile_id(&self) -> &AccessProfileId {
        &self.access_profile_id
    }

    #[must_use]
    pub const fn provider_attempt(&self) -> NonZeroU64 {
        self.provider_attempt
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProviderObservation {
    Usage(TokenUsage),
    BilledCost(BilledCostObservation),
    RateLimit(RateLimitObservation),
    Quota(QuotaObservation),
    RequestCorrelation(ProviderRequestRef),
}

#[cfg(test)]
mod tests;
