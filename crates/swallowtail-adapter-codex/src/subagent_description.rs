//! Bounded subagent description text shared by the app-server and exec projections.

use swallowtail_runtime::{ActivityContent, OperationContent, RuntimeFailure};

/// Largest subagent description the projections forward, in bytes.
const MAX_DESCRIPTION_BYTES: usize = 16 * 1024;

/// Builds bounded activity content from one provider description.
///
/// A description longer than the bound is cut at the last character boundary
/// that fits. `malformed` supplies the failure for text the runtime rejects.
pub(crate) fn bounded_description(
    value: &str,
    malformed: impl Fn() -> RuntimeFailure,
) -> Result<ActivityContent, RuntimeFailure> {
    let value = if value.len() <= MAX_DESCRIPTION_BYTES {
        value
    } else {
        &value[..value.floor_char_boundary(MAX_DESCRIPTION_BYTES)]
    };
    ActivityContent::new(
        OperationContent::new(value).map_err(|_| malformed())?,
        MAX_DESCRIPTION_BYTES,
    )
    .map_err(|_| malformed())
}

#[cfg(test)]
mod tests {
    use super::{MAX_DESCRIPTION_BYTES, bounded_description};
    use swallowtail_core::SafeDiagnostic;
    use swallowtail_runtime::RuntimeFailure;

    fn failure() -> RuntimeFailure {
        RuntimeFailure::new(SafeDiagnostic::new(
            "swallowtail.codex.test.malformed",
            "malformed",
        ))
    }

    #[test]
    fn short_descriptions_pass_through_unchanged() {
        assert!(bounded_description("short task", failure).is_ok());
    }

    #[test]
    fn long_descriptions_truncate_on_a_character_boundary() {
        let long = format!("{}é", "a".repeat(MAX_DESCRIPTION_BYTES - 1));
        assert_eq!(long.len(), MAX_DESCRIPTION_BYTES + 1);
        assert!(bounded_description(&long, failure).is_ok());
    }
}
