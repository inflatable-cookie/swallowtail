use crate::failure::failure;
use std::num::NonZeroU32;
use swallowtail_runtime::RuntimeFailure;

/// Minimum admitted `--ctx-size` value on exact owned `b10069`.
pub const MINIMUM: u32 = 1;

/// Maximum admitted `--ctx-size` value on exact owned `b10069`.
///
/// Tagged storage is signed `int32_t`. This ceiling keeps dispatch inside that
/// positive range without claiming pad, train-cap, allocation, or model fit.
pub const MAXIMUM: u32 = i32::MAX as u32;

/// Exact positive llama.cpp owned-serving `--ctx-size` selection.
///
/// This is adapter-local serving configuration. It is not a portable
/// context-window capability, output-token limit, or catalogue-derived model
/// claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LlamaCppContextSize(NonZeroU32);

impl LlamaCppContextSize {
    /// Creates a context-size selection from one admitted `u32`.
    pub fn new(value: NonZeroU32) -> Result<Self, RuntimeFailure> {
        Self::from_u64(u64::from(value.get()))
    }

    /// Returns the exact positive integer emitted as `--ctx-size`.
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        self.0.get()
    }

    /// Validates one caller-supplied integer before preparation.
    pub fn from_u64(value: u64) -> Result<Self, RuntimeFailure> {
        let value = u32::try_from(value)
            .ok()
            .and_then(NonZeroU32::new)
            .filter(|value| {
                let value = value.get();
                (MINIMUM..=MAXIMUM).contains(&value)
            })
            .ok_or_else(|| {
                failure(
                    "swallowtail.llama_cpp.context_size_invalid",
                    "Owned llama.cpp context size exceeded the supported positive request range",
                )
            })?;
        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admitted_domain_accepts_boundary_values() {
        assert_eq!(
            LlamaCppContextSize::from_u64(u64::from(MINIMUM))
                .expect("minimum is valid")
                .as_u32(),
            MINIMUM
        );
        assert_eq!(
            LlamaCppContextSize::from_u64(u64::from(MAXIMUM))
                .expect("maximum is valid")
                .as_u32(),
            MAXIMUM
        );
        assert_eq!(
            LlamaCppContextSize::from_u64(4096)
                .expect("representative value is valid")
                .as_u32(),
            4096
        );
    }

    #[test]
    fn zero_and_overflow_fail_closed() {
        for value in [0, u64::from(MAXIMUM) + 1, u64::from(u32::MAX)] {
            let error = LlamaCppContextSize::from_u64(value).expect_err("out of domain");
            assert_eq!(
                error.diagnostic().code(),
                "swallowtail.llama_cpp.context_size_invalid"
            );
        }
    }

    #[test]
    fn new_rejects_the_same_overflow_as_from_u64() {
        let error = LlamaCppContextSize::new(NonZeroU32::new(u32::MAX).expect("nonzero"))
            .expect_err("u32::MAX exceeds i32::MAX");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.llama_cpp.context_size_invalid"
        );
    }
}
