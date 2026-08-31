use super::failure::{ConsumerRouteProjectionFailure, ConsumerRouteProjectionFailureKind, failure};

/// Rejects blank, control-bearing, or over-long admitted projection text.
///
/// Every bound counts UTF-8 bytes and rejects rather than truncating.
pub(super) fn admit_text(
    value: &str,
    maximum: usize,
    kind: ConsumerRouteProjectionFailureKind,
    code: &'static str,
    message: &'static str,
) -> Result<(), ConsumerRouteProjectionFailure> {
    if value.trim().is_empty() || value.chars().any(char::is_control) {
        return Err(failure(
            ConsumerRouteProjectionFailureKind::IdentityInvalid,
            "swallowtail.consumer_route_projection.text_invalid",
            "Projected text must be non-blank and free of control characters",
        ));
    }
    if value.len() > maximum {
        return Err(failure(kind, code, message));
    }
    Ok(())
}
