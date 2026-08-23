//! Caller-decreasing Mistral Vibe headless maximum turns.

/// Caller-decreasing Vibe `--max-turns` bound for one child process.
///
/// Omitted selection keeps the current argv byte `8`. Only `1..=8` construct.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MistralVibeMaxTurns(u8);

impl MistralVibeMaxTurns {
    /// Smallest admitted turn bound.
    pub const MIN: u8 = 1;
    /// Largest admitted turn bound; omission uses this same current argv.
    pub const MAX: u8 = 8;

    /// Accepts only `1..=8`.
    #[must_use]
    pub const fn try_new(value: u8) -> Option<Self> {
        if value >= Self::MIN && value <= Self::MAX {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Returns the admitted integer.
    #[must_use]
    pub const fn get(self) -> u8 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::MistralVibeMaxTurns;

    #[test]
    fn constructors_admit_only_research_199_one_through_eight() {
        assert!(MistralVibeMaxTurns::try_new(0).is_none());
        assert_eq!(
            MistralVibeMaxTurns::try_new(1).map(MistralVibeMaxTurns::get),
            Some(1)
        );
        assert_eq!(
            MistralVibeMaxTurns::try_new(8).map(MistralVibeMaxTurns::get),
            Some(8)
        );
        assert!(MistralVibeMaxTurns::try_new(9).is_none());
    }
}
