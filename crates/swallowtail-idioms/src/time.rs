/// Executor-neutral point on an execution host's monotonic clock.
///
/// Ticks are host-defined; the same tick unit must be used for `as-of`,
/// signal, and decay times within one store. Values are plain so fixtures
/// can pin exact decay behavior.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MonotonicInstant(u64);

impl MonotonicInstant {
    /// Creates an instant from host-defined monotonic ticks.
    #[must_use]
    pub const fn from_ticks(ticks: u64) -> Self {
        Self(ticks)
    }

    /// Returns the host-defined monotonic tick value.
    #[must_use]
    pub const fn ticks(self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::MonotonicInstant;

    #[test]
    fn instant_is_plain_and_ordered() {
        let a = MonotonicInstant::from_ticks(1);
        let b = MonotonicInstant::from_ticks(2);
        assert_eq!(a.ticks(), 1);
        assert!(a < b);
        assert_eq!(a, MonotonicInstant::from_ticks(1));
    }
}
