#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Executor-neutral point on an execution host's monotonic clock.
///
/// Runtime monotonic ticks are nanoseconds of the host's clock: the host owns
/// the clock origin, the runtime fixes the unit so one duration-to-tick
/// boundary exists. Use [`MonotonicInstant::saturating_add_duration`] rather
/// than adding raw durations to host ticks.
pub struct MonotonicInstant(u64);

impl MonotonicInstant {
    #[must_use]
    /// Creates an instant from host-defined monotonic ticks.
    pub const fn from_ticks(ticks: u64) -> Self {
        Self(ticks)
    }

    #[must_use]
    /// Returns the host-defined monotonic tick value.
    pub const fn ticks(self) -> u64 {
        self.0
    }

    #[must_use]
    /// Returns the instant one duration later on the runtime monotonic clock.
    ///
    /// This is the single duration-to-tick conversion boundary: callers never
    /// add raw duration values to host ticks themselves. Saturation matches
    /// the host deadline rule, so an unrepresentable sum clamps to the latest
    /// representable instant.
    pub fn saturating_add_duration(self, duration: std::time::Duration) -> Self {
        let ticks = u64::try_from(duration.as_nanos()).unwrap_or(u64::MAX);
        Self(self.0.saturating_add(ticks))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Exact monotonic deadline selected for one operation.
pub struct Deadline(MonotonicInstant);

impl Deadline {
    #[must_use]
    /// Creates a deadline at an exact monotonic instant.
    pub const fn at(instant: MonotonicInstant) -> Self {
        Self(instant)
    }

    #[must_use]
    /// Returns the deadline's monotonic instant.
    pub const fn instant(self) -> MonotonicInstant {
        self.0
    }
}

/// Evidence that the host monotonic clock reached one operation deadline.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeadlineObservation {
    deadline: Deadline,
    observed_at: MonotonicInstant,
}

impl DeadlineObservation {
    #[must_use]
    /// Creates evidence that the deadline was observed at a monotonic instant.
    pub const fn new(deadline: Deadline, observed_at: MonotonicInstant) -> Self {
        Self {
            deadline,
            observed_at,
        }
    }

    #[must_use]
    /// Returns the deadline that was reached.
    pub const fn deadline(self) -> Deadline {
        self.deadline
    }

    #[must_use]
    /// Returns the monotonic instant at which the host observed it.
    pub const fn observed_at(self) -> MonotonicInstant {
        self.observed_at
    }
}

#[cfg(test)]
mod tests {
    use super::MonotonicInstant;
    use std::time::Duration;

    #[test]
    fn duration_addition_uses_the_one_nanosecond_tick_boundary() {
        let instant = MonotonicInstant::from_ticks(10);
        assert_eq!(
            instant
                .saturating_add_duration(Duration::from_nanos(25))
                .ticks(),
            35
        );
        assert_eq!(
            instant
                .saturating_add_duration(Duration::from_secs(1))
                .ticks(),
            1_000_000_010
        );
        assert_eq!(instant.saturating_add_duration(Duration::ZERO).ticks(), 10);
    }

    #[test]
    fn duration_addition_saturates_at_the_latest_instant() {
        let instant = MonotonicInstant::from_ticks(u64::MAX - 5);
        assert_eq!(
            instant
                .saturating_add_duration(Duration::from_nanos(10))
                .ticks(),
            u64::MAX
        );
        assert_eq!(
            MonotonicInstant::from_ticks(0)
                .saturating_add_duration(Duration::new(u64::MAX, 999_999_999))
                .ticks(),
            u64::MAX
        );
    }
}
