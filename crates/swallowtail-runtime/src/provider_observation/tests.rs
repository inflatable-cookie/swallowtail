use super::{
    BilledCostObservation, BilledCostSemantics, BilledCostSource, Currency, ProviderObservation,
    QuotaObservation, QuotaState, RateLimitKind, RateLimitObservation, TokenUsage,
};
use crate::RuntimeTurnId;
use std::num::NonZeroU64;
use swallowtail_core::{AccessProfileId, ModelRouteId, ProviderRequestRef};

#[test]
fn usage_rate_and_quota_remain_distinct_observations() {
    let observations = [
        ProviderObservation::Usage(TokenUsage::new(Some(12), Some(4))),
        ProviderObservation::RateLimit(RateLimitObservation::new(
            RateLimitKind::Requests,
            Some(100),
            Some(99),
            Some(1_000),
        )),
        ProviderObservation::Quota(QuotaObservation::new(QuotaState::Available)),
    ];

    assert!(matches!(observations[0], ProviderObservation::Usage(_)));
    assert!(matches!(observations[1], ProviderObservation::RateLimit(_)));
    assert!(matches!(observations[2], ProviderObservation::Quota(_)));

    let correlation = ProviderObservation::RequestCorrelation(
        ProviderRequestRef::new("fixture-request").expect("request reference is valid"),
    );
    assert!(matches!(
        correlation,
        ProviderObservation::RequestCorrelation(_)
    ));
}

#[test]
fn disjoint_usage_addition_preserves_dimensions_and_rejects_overflow() {
    let first = TokenUsage::new(Some(12), Some(4))
        .with_reasoning_tokens(Some(2))
        .with_cache_tokens(Some(3), Some(1));
    let second = TokenUsage::new(Some(8), Some(6))
        .with_reasoning_tokens(Some(1))
        .with_cache_tokens(Some(1), Some(0));
    let total = first
        .checked_add_disjoint(second)
        .expect("bounded components add");

    assert_eq!(total.input_tokens(), Some(20));
    assert_eq!(total.output_tokens(), Some(10));
    assert_eq!(total.reasoning_tokens(), Some(3));
    assert_eq!(total.cache_read_input_tokens(), Some(4));
    assert_eq!(total.cache_write_input_tokens(), Some(1));
    assert!(
        TokenUsage::new(Some(u64::MAX), None)
            .checked_add_disjoint(TokenUsage::new(Some(1), None))
            .is_none()
    );
}

#[test]
fn billed_cost_is_exact_scoped_provider_evidence() {
    let observation = BilledCostObservation::provider_reported(
        125_000,
        Currency::Usd,
        NonZeroU64::new(10_000_000_000).unwrap(),
        RuntimeTurnId::new("turn-1").unwrap(),
        ModelRouteId::new("route-1").unwrap(),
        AccessProfileId::new("access-1").unwrap(),
        NonZeroU64::new(1).unwrap(),
    );

    assert_eq!(observation.amount(), 125_000);
    assert_eq!(observation.currency(), Currency::Usd);
    assert_eq!(observation.units_per_currency().get(), 10_000_000_000);
    assert_eq!(observation.source(), BilledCostSource::ProviderReported);
    assert_eq!(
        observation.semantics(),
        BilledCostSemantics::CumulativeReplacement
    );
    assert_eq!(observation.provider_attempt().get(), 1);
    assert!(matches!(
        ProviderObservation::BilledCost(observation),
        ProviderObservation::BilledCost(_)
    ));
}
