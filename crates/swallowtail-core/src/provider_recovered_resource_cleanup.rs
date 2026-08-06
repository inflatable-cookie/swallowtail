/// Truth known after one recovered owned-resource cleanup attempt.
///
/// The portable contract does not expose provider deletion order. A partial or
/// uncertain effect therefore cannot be mistaken for complete cleanup.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ProviderRecoveredResourceCleanupEffect {
    /// Every required owned resource was confirmed cleaned.
    Applied,
    /// Cleanup was rejected because operation activity was active or unknown.
    RejectedActiveOrUnknown,
    /// Attempt failed before any cleanup effect.
    FailedBeforeEffect,
    /// Some resources were cleaned before failure.
    PartiallyApplied,
    /// Cleanup may have occurred but terminal truth is unavailable.
    UnconfirmedAfterEffect,
}

impl ProviderRecoveredResourceCleanupEffect {
    #[must_use]
    /// Reports whether complete cleanup was confirmed.
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Applied)
    }
}

#[cfg(test)]
mod tests {
    use super::ProviderRecoveredResourceCleanupEffect;

    #[test]
    fn only_applied_cleanup_is_complete() {
        assert!(ProviderRecoveredResourceCleanupEffect::Applied.is_complete());
        for effect in [
            ProviderRecoveredResourceCleanupEffect::RejectedActiveOrUnknown,
            ProviderRecoveredResourceCleanupEffect::FailedBeforeEffect,
            ProviderRecoveredResourceCleanupEffect::PartiallyApplied,
            ProviderRecoveredResourceCleanupEffect::UnconfirmedAfterEffect,
        ] {
            assert!(!effect.is_complete());
        }
    }
}
