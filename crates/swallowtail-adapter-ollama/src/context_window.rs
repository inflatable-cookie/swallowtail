use crate::failure::failure;
use std::num::NonZeroU32;
use swallowtail_runtime::RuntimeFailure;

/// Minimum admitted `options.num_ctx` value on the qualified attached route.
pub const MINIMUM: u32 = 4;

/// Maximum admitted `options.num_ctx` value on the qualified attached route.
///
/// The tagged Ollama wire field is a signed Go `int`. This ceiling keeps dispatch
/// inside the positive range representable on every qualified host without
/// claiming effective allocation above model or runtime limits.
pub const MAXIMUM: u32 = i32::MAX as u32;

/// Exact positive Ollama native `options.num_ctx` selection.
///
/// This is an adapter-local runner option. It is not a portable context-window
/// capability, output-token limit, or catalogue-derived model claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OllamaContextWindow(NonZeroU32);

impl OllamaContextWindow {
    /// Creates a context-window selection from one admitted `u32`.
    pub fn new(value: NonZeroU32) -> Result<Self, RuntimeFailure> {
        Self::from_u64(u64::from(value.get()))
    }

    /// Returns the exact positive integer encoded on the native wire.
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
                    "swallowtail.ollama.context_window_invalid",
                    "Ollama context window exceeded the supported positive request range",
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
            OllamaContextWindow::from_u64(u64::from(MINIMUM))
                .expect("minimum is valid")
                .as_u32(),
            MINIMUM
        );
        assert_eq!(
            OllamaContextWindow::from_u64(u64::from(MAXIMUM))
                .expect("maximum is valid")
                .as_u32(),
            MAXIMUM
        );
    }

    #[test]
    fn below_minimum_zero_and_overflow_fail_closed() {
        for value in [0, 1, 2, 3, u64::from(MAXIMUM) + 1] {
            let error = OllamaContextWindow::from_u64(value).expect_err("out of domain");
            assert_eq!(
                error.diagnostic().code(),
                "swallowtail.ollama.context_window_invalid"
            );
        }
    }

    #[test]
    fn new_rejects_the_same_out_of_domain_values_as_from_u64() {
        for value in [1, 2, 3, u32::MAX] {
            let nonzero = NonZeroU32::new(value).expect("value is nonzero");
            let error = OllamaContextWindow::new(nonzero).expect_err("out of domain");
            assert_eq!(
                error.diagnostic().code(),
                "swallowtail.ollama.context_window_invalid"
            );
        }
    }
}
