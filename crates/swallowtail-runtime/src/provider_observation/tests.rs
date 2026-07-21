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
