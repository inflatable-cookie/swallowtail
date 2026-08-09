use crate::time::MonotonicInstant;

/// Default decay half-life in host monotonic ticks (one day at second
/// granularity; hosts choose a tick unit and may pass their own half-life).
pub const DEFAULT_DECAY_HALF_LIFE_TICKS: u64 = 86_400;

/// Maximum halvings applied before effective confidence floors at zero.
pub const MAX_DECAY_HALVINGS: u32 = 7;

/// Why a confidence record was rejected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfidenceErrorKind {
    /// The stored value was outside `0..=100`.
    OutOfRange,
    /// The decay half-life was zero.
    ZeroHalfLife,
}

/// Stored confidence with time-based decay (Contract 055).
///
/// Effective confidence is a pure deterministic function of the stored
/// value, the `as-of` instant, the fixed half-life, and the evaluation
/// instant. The same record and elapsed time always yield the same value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Confidence {
    value: u8,
    as_of: MonotonicInstant,
    half_life_ticks: u64,
}

impl Confidence {
    /// Creates a confidence record after rejecting out-of-range values and a
    /// zero half-life.
    pub fn new(
        value: u8,
        as_of: MonotonicInstant,
        half_life_ticks: u64,
    ) -> Result<Self, ConfidenceErrorKind> {
        if value > 100 {
            return Err(ConfidenceErrorKind::OutOfRange);
        }
        if half_life_ticks == 0 {
            return Err(ConfidenceErrorKind::ZeroHalfLife);
        }
        Ok(Self {
            value,
            as_of,
            half_life_ticks,
        })
    }

    /// Returns the stored (undecayed) confidence value.
    #[must_use]
    pub const fn value(self) -> u8 {
        self.value
    }

    /// Returns the instant at which the value was last set by a signal.
    #[must_use]
    pub const fn as_of(self) -> MonotonicInstant {
        self.as_of
    }

    /// Returns the fixed decay half-life in host monotonic ticks.
    #[must_use]
    pub const fn half_life_ticks(self) -> u64 {
        self.half_life_ticks
    }

    /// Returns the effective confidence at an evaluation instant.
    ///
    /// Effective confidence halves per elapsed half-life and floors at zero
    /// after `MAX_DECAY_HALVINGS`. Evaluation earlier than `as-of` treats
    /// the stored value as effective (no future credit).
    #[must_use]
    pub fn effective(self, at: MonotonicInstant) -> u8 {
        let elapsed = at.ticks().saturating_sub(self.as_of.ticks());
        let halvings = (elapsed / self.half_life_ticks).min(MAX_DECAY_HALVINGS as u64);
        self.value >> halvings
    }
}

#[cfg(test)]
impl Confidence {
    /// Builds a confidence without validation so lint fixtures can exercise
    /// shapes constructors reject.
    pub(crate) fn unchecked(value: u8, as_of: MonotonicInstant, half_life_ticks: u64) -> Self {
        Self {
            value,
            as_of,
            half_life_ticks,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Confidence, ConfidenceErrorKind, DEFAULT_DECAY_HALF_LIFE_TICKS, MAX_DECAY_HALVINGS,
    };
    use crate::time::MonotonicInstant;

    fn at(ticks: u64) -> MonotonicInstant {
        MonotonicInstant::from_ticks(ticks)
    }

    #[test]
    fn rejects_out_of_range_and_zero_half_life() {
        assert_eq!(
            Confidence::new(101, at(0), 1).expect_err("out of range"),
            ConfidenceErrorKind::OutOfRange
        );
        assert_eq!(
            Confidence::new(50, at(0), 0).expect_err("zero half-life"),
            ConfidenceErrorKind::ZeroHalfLife
        );
    }

    #[test]
    fn effective_is_full_before_one_half_life() {
        let confidence = Confidence::new(100, at(1_000), DEFAULT_DECAY_HALF_LIFE_TICKS)
            .expect("valid confidence");
        assert_eq!(confidence.effective(at(1_000)), 100);
        assert_eq!(
            confidence.effective(at(1_000 + DEFAULT_DECAY_HALF_LIFE_TICKS - 1)),
            100
        );
    }

    #[test]
    fn effective_halves_per_half_life() {
        let confidence =
            Confidence::new(100, at(0), DEFAULT_DECAY_HALF_LIFE_TICKS).expect("valid confidence");
        let one = DEFAULT_DECAY_HALF_LIFE_TICKS;
        assert_eq!(confidence.effective(at(one)), 50);
        assert_eq!(confidence.effective(at(2 * one)), 25);
        assert_eq!(confidence.effective(at(3 * one)), 12);
    }

    #[test]
    fn effective_floors_at_zero() {
        let confidence = Confidence::new(100, at(0), 1).expect("valid confidence");
        assert_eq!(confidence.effective(at(MAX_DECAY_HALVINGS as u64)), 0);
        assert_eq!(confidence.effective(at(u64::MAX)), 0);
    }

    #[test]
    fn effective_never_credits_future_time() {
        let confidence = Confidence::new(80, at(500), 100).expect("valid confidence");
        assert_eq!(confidence.effective(at(0)), 80);
    }

    #[test]
    fn decay_is_pure_and_deterministic() {
        let confidence = Confidence::new(100, at(10), 50).expect("valid confidence");
        assert_eq!(confidence.effective(at(60)), confidence.effective(at(60)));
        assert_eq!(confidence.effective(at(110)), 25);
    }
}
