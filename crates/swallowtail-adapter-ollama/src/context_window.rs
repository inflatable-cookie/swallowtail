use crate::failure::failure;
use std::num::NonZeroU32;
use swallowtail_runtime::RuntimeFailure;

/// Exact positive Ollama native `options.num_ctx` selection.
///
/// This is an adapter-local runner option. It is not a portable context-window
/// capability, output-token limit, or catalogue-derived model claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OllamaContextWindow(NonZeroU32);

impl OllamaContextWindow {
    /// Creates a context-window selection from one positive `u32`.
    #[must_use]
    pub const fn new(value: NonZeroU32) -> Self {
        Self(value)
    }

    /// Returns the exact positive integer encoded on the native wire.
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        self.0.get()
    }

    /// Validates one caller-supplied positive integer before preparation.
    pub fn from_u64(value: u64) -> Result<Self, RuntimeFailure> {
        let value = u32::try_from(value)
            .ok()
            .and_then(NonZeroU32::new)
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
    fn positive_domain_accepts_boundary_values() {
        assert_eq!(
            OllamaContextWindow::from_u64(1)
                .expect("minimum is valid")
                .as_u32(),
            1
        );
        assert_eq!(
            OllamaContextWindow::from_u64(u32::MAX as u64)
                .expect("maximum is valid")
                .as_u32(),
            u32::MAX
        );
    }

    #[test]
    fn zero_overflow_and_negative_representations_fail_closed() {
        for value in [0, u64::from(u32::MAX) + 1] {
            let error = OllamaContextWindow::from_u64(value).expect_err("out of domain");
            assert_eq!(
                error.diagnostic().code(),
                "swallowtail.ollama.context_window_invalid"
            );
        }
    }
}
