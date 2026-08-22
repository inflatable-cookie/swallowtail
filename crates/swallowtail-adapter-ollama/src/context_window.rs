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
    #[must_use]
    pub const fn new(value: NonZeroU32) -> Self {
        Self(value)
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

pub(crate) fn validate_context_window_agreement(
    driver: Option<OllamaContextWindow>,
    evidence: Option<OllamaContextWindow>,
) -> Result<(), RuntimeFailure> {
    if driver == evidence {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.ollama.context_window_binding_mismatch",
            "Ollama context window binding did not match prepared evidence",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::OllamaNativeAttachedDriver;

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
    fn mismatched_driver_and_evidence_fail_closed() {
        let driver = OllamaNativeAttachedDriver::new().with_context_window(
            OllamaContextWindow::new(NonZeroU32::new(4096).expect("value is nonzero")),
        );
        let error = validate_context_window_agreement(
            driver.context_window(),
            Some(OllamaContextWindow::new(
                NonZeroU32::new(8192).expect("value is nonzero"),
            )),
        )
        .expect_err("mismatch fails");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.ollama.context_window_binding_mismatch"
        );
    }
}
