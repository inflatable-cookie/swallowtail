#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MonotonicInstant(u64);

impl MonotonicInstant {
    #[must_use]
    pub const fn from_ticks(ticks: u64) -> Self {
        Self(ticks)
    }

    #[must_use]
    pub const fn ticks(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Deadline(MonotonicInstant);

impl Deadline {
    #[must_use]
    pub const fn at(instant: MonotonicInstant) -> Self {
        Self(instant)
    }

    #[must_use]
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
    pub const fn new(deadline: Deadline, observed_at: MonotonicInstant) -> Self {
        Self {
            deadline,
            observed_at,
        }
    }

    #[must_use]
    pub const fn deadline(self) -> Deadline {
        self.deadline
    }

    #[must_use]
    pub const fn observed_at(self) -> MonotonicInstant {
        self.observed_at
    }
}
