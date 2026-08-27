use crate::failure::failure;
use std::num::NonZeroU32;
use swallowtail_runtime::RuntimeFailure;

/// Exact admitted `--max-turns` selection for native Claude Code headless runs.
///
/// This is adapter-local Claude Code configuration. It is not a portable agent
/// budget, an output-token limit, a tool-call budget, a cost cap, a wall-time
/// deadline, a context bound, or a retry count. One counted turn is one
/// tool-use round trip; a final text-only response is not counted.
///
/// Research 226 closes the domain to positive integers because the native
/// parser does not. Exact `2.1.220..=2.1.241` coerces the argument with
/// `Number` and rejects only `NaN`, so zero, negatives, fractions, `Infinity`,
/// exponent and hexadecimal forms, grouped digits, and the empty string all
/// pass parsing. The agent loop then guards with `maxTurns && next > maxTurns`,
/// a truthiness test under which a resolved `0` disables enforcement outright
/// and a negative value stops after the first tool-use turn. Only a positive
/// integer produces the documented bound, so only a positive integer is
/// admitted here.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ClaudeCodeMaximumTurns(NonZeroU32);

impl ClaudeCodeMaximumTurns {
    /// Creates a maximum-turn selection from one admitted positive integer.
    #[must_use]
    pub const fn new(value: NonZeroU32) -> Self {
        Self(value)
    }

    /// Validates one caller-supplied integer before preparation.
    ///
    /// Zero and any value above [`u32::MAX`] fail closed. Every admitted value
    /// is exactly representable as an IEEE-754 double, so the native `Number`
    /// coercion round-trips the dispatched decimal literal without loss.
    pub fn from_u64(value: u64) -> Result<Self, RuntimeFailure> {
        u32::try_from(value)
            .ok()
            .and_then(NonZeroU32::new)
            .map(Self)
            .ok_or_else(|| {
                failure(
                    "swallowtail.claude_code.headless.maximum_turns_invalid",
                    "Claude Code headless maximum turns must be a positive 32-bit integer",
                )
            })
    }

    /// Returns the exact positive integer emitted after `--max-turns`.
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        self.0.get()
    }
}

#[cfg(test)]
mod tests {
    use super::ClaudeCodeMaximumTurns;

    #[test]
    fn admitted_domain_accepts_positive_integers() {
        for admitted in [1, 3, 30, u64::from(u32::MAX)] {
            assert_eq!(
                ClaudeCodeMaximumTurns::from_u64(admitted)
                    .expect("positive value is admitted")
                    .as_u32(),
                u32::try_from(admitted).expect("value fits u32")
            );
        }
    }

    #[test]
    fn zero_and_overflow_fail_closed() {
        for rejected in [0, u64::from(u32::MAX) + 1, u64::MAX] {
            let error =
                ClaudeCodeMaximumTurns::from_u64(rejected).expect_err("value is not admitted");
            assert_eq!(
                error.diagnostic().code(),
                "swallowtail.claude_code.headless.maximum_turns_invalid"
            );
        }
    }
}
