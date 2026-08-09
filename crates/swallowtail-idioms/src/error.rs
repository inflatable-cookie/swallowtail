use swallowtail_core::{FailureClassification, SafeDiagnostic};

use crate::bounded::BoundedText;
use crate::confidence::Confidence;
use crate::time::MonotonicInstant;

/// Stable classification of an idiom record construction failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdiomErrorKind {
    /// Required idiom text was blank.
    BlankText,
    /// Idiom text exceeded its byte bound.
    TextTooLong,
    /// A confidence value was outside `0..=100`.
    ConfidenceOutOfRange,
    /// A confidence decay half-life was zero.
    ZeroHalfLife,
}

/// Bounded idiom record error without raw payload content.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdiomError {
    kind: IdiomErrorKind,
    safe: SafeDiagnostic,
}

impl IdiomError {
    pub(crate) fn new(kind: IdiomErrorKind, field: &'static str) -> Self {
        let (code, message) = match kind {
            IdiomErrorKind::BlankText => (
                "swallowtail.idioms.blank_text",
                format!("{field} must not be empty"),
            ),
            IdiomErrorKind::TextTooLong => (
                "swallowtail.idioms.text_too_long",
                format!("{field} exceeds its byte bound"),
            ),
            IdiomErrorKind::ConfidenceOutOfRange => (
                "swallowtail.idioms.confidence_out_of_range",
                format!("{field} must be between 0 and 100"),
            ),
            IdiomErrorKind::ZeroHalfLife => (
                "swallowtail.idioms.zero_half_life",
                format!("{field} must be greater than zero"),
            ),
        };
        Self {
            kind,
            safe: SafeDiagnostic::new(code, message),
        }
    }

    /// Returns the stable failure classification.
    #[must_use]
    pub const fn kind(&self) -> IdiomErrorKind {
        self.kind
    }

    /// Returns the operator-safe diagnostic for this record error.
    #[must_use]
    pub const fn diagnostic(&self) -> &SafeDiagnostic {
        &self.safe
    }

    #[must_use]
    /// Returns the stable classification used by `SafeDiagnostic`.
    pub const fn failure_classification(&self) -> FailureClassification {
        self.safe.failure_classification()
    }
}

impl std::fmt::Display for IdiomError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.safe.fmt(formatter)
    }
}

impl std::error::Error for IdiomError {}

pub(crate) fn require_bounded(
    field: &'static str,
    value: impl Into<String>,
    maximum_bytes: usize,
) -> Result<BoundedText, IdiomError> {
    BoundedText::new(value, maximum_bytes).map_err(|bounded_error| {
        let kind = match bounded_error {
            crate::bounded::BoundedTextError::Blank => IdiomErrorKind::BlankText,
            crate::bounded::BoundedTextError::TooLong(_) => IdiomErrorKind::TextTooLong,
        };
        IdiomError::new(kind, field)
    })
}

pub(crate) fn require_confidence(
    field: &'static str,
    value: u8,
    as_of: MonotonicInstant,
    half_life_ticks: u64,
) -> Result<Confidence, IdiomError> {
    Confidence::new(value, as_of, half_life_ticks).map_err(|kind| {
        let error_kind = match kind {
            crate::confidence::ConfidenceErrorKind::OutOfRange => {
                IdiomErrorKind::ConfidenceOutOfRange
            }
            crate::confidence::ConfidenceErrorKind::ZeroHalfLife => IdiomErrorKind::ZeroHalfLife,
        };
        IdiomError::new(error_kind, field)
    })
}

#[cfg(test)]
mod tests {
    use super::{IdiomErrorKind, require_bounded};

    #[test]
    fn bounded_text_rejects_blank() {
        let error = require_bounded("idiom id", "   ", 64).expect_err("blank must fail");
        assert_eq!(error.kind(), IdiomErrorKind::BlankText);
        assert_eq!(error.diagnostic().code(), "swallowtail.idioms.blank_text");
    }

    #[test]
    fn bounded_text_rejects_overflow() {
        let error =
            require_bounded("pattern", "x".repeat(513), 512).expect_err("overflow must fail");
        assert_eq!(error.kind(), IdiomErrorKind::TextTooLong);
    }
}
