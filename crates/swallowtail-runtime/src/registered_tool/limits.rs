//! Contract 063 first-implementation bounds for the registered-tool profile.

use super::failure::{RegisteredToolFailure, RegisteredToolFailureKind, reject};
use std::time::Duration;

/// Maximum outstanding calls per lease in the first tranche.
pub const MAX_REGISTERED_TOOL_OUTSTANDING_CALLS: usize = 1;
/// Maximum selected tools in one prepared selection.
pub const MAX_REGISTERED_TOOL_SELECTED_TOOLS: usize = 64;
/// Maximum bytes for one declared schema document.
pub const MAX_REGISTERED_TOOL_SCHEMA_BYTES: usize = 64 * 1024;
/// Maximum aggregate declared schema bytes in one snapshot.
pub const MAX_REGISTERED_TOOL_AGGREGATE_SCHEMA_BYTES: usize = 1024 * 1024;
/// Maximum bytes for one call's bounded arguments.
pub const MAX_REGISTERED_TOOL_ARGUMENT_BYTES: usize = 256 * 1024;
/// Maximum bytes for one bounded result payload.
pub const MAX_REGISTERED_TOOL_RESULT_BYTES: usize = 256 * 1024;
/// Maximum bytes for one bounded progress item.
pub const MAX_REGISTERED_TOOL_PROGRESS_ITEM_BYTES: usize = 64 * 1024;
/// Maximum queued progress items per call.
pub const MAX_REGISTERED_TOOL_QUEUED_PROGRESS_ITEMS: usize = 32;
/// Maximum bytes for a bounded identity or reason-code value.
pub const MAX_REGISTERED_TOOL_IDENTITY_BYTES: usize = 256;
/// Maximum declarations carried by one registration snapshot.
pub const MAX_REGISTERED_TOOL_DECLARATIONS: usize = 64;
/// Maximum required host-service kinds declared by one snapshot.
pub const MAX_REGISTERED_TOOL_REQUIRED_SERVICES: usize = 32;
/// Maximum credential references declared by one snapshot.
pub const MAX_REGISTERED_TOOL_CREDENTIAL_REFERENCES: usize = 16;
/// Maximum process or environment recipe references declared by one snapshot.
pub const MAX_REGISTERED_TOOL_RECIPE_REFERENCES: usize = 16;
/// Maximum supported transports declared by one snapshot.
pub const MAX_REGISTERED_TOOL_TRANSPORTS: usize = 8;
/// Maximum qualified protocol versions declared per transport.
pub const MAX_REGISTERED_TOOL_PROTOCOL_VERSIONS: usize = 8;

/// Longest call lifetime before expiry, independent of the operation deadline.
pub const REGISTERED_TOOL_MAX_CALL_DURATION: Duration = Duration::from_secs(60);
/// Bounded budget for opening one lease.
pub const REGISTERED_TOOL_OPEN_BUDGET: Duration = Duration::from_secs(10);
/// Bounded budget for one joined cleanup attempt.
pub const REGISTERED_TOOL_CLEANUP_BUDGET: Duration = Duration::from_secs(10);

/// Positive per-lease bounds declared by a registration snapshot.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RegisteredToolBounds {
    max_outstanding_calls: usize,
    max_argument_bytes: usize,
    max_result_bytes: usize,
    max_progress_item_bytes: usize,
    max_queued_progress_items: usize,
    max_call_duration: Duration,
}

impl RegisteredToolBounds {
    /// Creates positive bounds that never widen the first-tranche ceilings.
    pub fn new(
        max_outstanding_calls: usize,
        max_argument_bytes: usize,
        max_result_bytes: usize,
        max_progress_item_bytes: usize,
        max_queued_progress_items: usize,
        max_call_duration: Duration,
    ) -> Result<Self, RegisteredToolFailure> {
        let bounds = Self {
            max_outstanding_calls,
            max_argument_bytes,
            max_result_bytes,
            max_progress_item_bytes,
            max_queued_progress_items,
            max_call_duration,
        };
        bounds.validate()?;
        Ok(bounds)
    }

    /// Returns the widest bounds this slice admits.
    #[must_use]
    pub const fn ceiling() -> Self {
        Self {
            max_outstanding_calls: MAX_REGISTERED_TOOL_OUTSTANDING_CALLS,
            max_argument_bytes: MAX_REGISTERED_TOOL_ARGUMENT_BYTES,
            max_result_bytes: MAX_REGISTERED_TOOL_RESULT_BYTES,
            max_progress_item_bytes: MAX_REGISTERED_TOOL_PROGRESS_ITEM_BYTES,
            max_queued_progress_items: MAX_REGISTERED_TOOL_QUEUED_PROGRESS_ITEMS,
            max_call_duration: REGISTERED_TOOL_MAX_CALL_DURATION,
        }
    }

    fn validate(self) -> Result<(), RegisteredToolFailure> {
        let positive = self.max_outstanding_calls > 0
            && self.max_argument_bytes > 0
            && self.max_result_bytes > 0
            && self.max_progress_item_bytes > 0
            && self.max_queued_progress_items > 0
            && !self.max_call_duration.is_zero();
        if !positive {
            return Err(reject(RegisteredToolFailureKind::IdentityRejected));
        }
        let ceiling = Self::ceiling();
        let within = self.max_outstanding_calls <= ceiling.max_outstanding_calls
            && self.max_argument_bytes <= ceiling.max_argument_bytes
            && self.max_result_bytes <= ceiling.max_result_bytes
            && self.max_progress_item_bytes <= ceiling.max_progress_item_bytes
            && self.max_queued_progress_items <= ceiling.max_queued_progress_items
            && self.max_call_duration <= ceiling.max_call_duration;
        if within {
            Ok(())
        } else {
            Err(reject(RegisteredToolFailureKind::LimitExceeded))
        }
    }

    /// Returns the narrower of two bound sets, field by field.
    #[must_use]
    pub fn narrowed(self, other: Self) -> Self {
        Self {
            max_outstanding_calls: self.max_outstanding_calls.min(other.max_outstanding_calls),
            max_argument_bytes: self.max_argument_bytes.min(other.max_argument_bytes),
            max_result_bytes: self.max_result_bytes.min(other.max_result_bytes),
            max_progress_item_bytes: self
                .max_progress_item_bytes
                .min(other.max_progress_item_bytes),
            max_queued_progress_items: self
                .max_queued_progress_items
                .min(other.max_queued_progress_items),
            max_call_duration: self.max_call_duration.min(other.max_call_duration),
        }
    }

    /// Returns the maximum outstanding calls for one lease.
    #[must_use]
    pub const fn max_outstanding_calls(self) -> usize {
        self.max_outstanding_calls
    }

    /// Returns the maximum bounded argument bytes for one call.
    #[must_use]
    pub const fn max_argument_bytes(self) -> usize {
        self.max_argument_bytes
    }

    /// Returns the maximum bounded result bytes for one call.
    #[must_use]
    pub const fn max_result_bytes(self) -> usize {
        self.max_result_bytes
    }

    /// Returns the maximum bounded bytes for one progress item.
    #[must_use]
    pub const fn max_progress_item_bytes(self) -> usize {
        self.max_progress_item_bytes
    }

    /// Returns the maximum queued progress items for one call.
    #[must_use]
    pub const fn max_queued_progress_items(self) -> usize {
        self.max_queued_progress_items
    }

    /// Returns the maximum lifetime of one call.
    #[must_use]
    pub const fn max_call_duration(self) -> Duration {
        self.max_call_duration
    }
}

/// Consumer-selected narrowing applied when a preparation is constructed.
///
/// Consumer limits may only narrow snapshot-declared bounds; they never widen
/// them and never raise a first-tranche ceiling.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RegisteredToolLimits {
    bounds: RegisteredToolBounds,
}

impl RegisteredToolLimits {
    /// Creates consumer limits from positive bounds.
    #[must_use]
    pub const fn new(bounds: RegisteredToolBounds) -> Self {
        Self { bounds }
    }

    /// Returns limits that apply no narrowing beyond the first-tranche ceiling.
    #[must_use]
    pub const fn ceiling() -> Self {
        Self {
            bounds: RegisteredToolBounds::ceiling(),
        }
    }

    /// Returns the consumer-selected bounds.
    #[must_use]
    pub const fn bounds(self) -> RegisteredToolBounds {
        self.bounds
    }
}

/// Maximum aggregate bytes of one selected skill body and its references.
///
/// Contract 063 fixes 64 KiB of total selected skill/reference content. No
/// single body may exceed the aggregate it is counted against.
pub const MAX_SELECTED_SKILL_CONTENT_BYTES: usize = 64 * 1024;
/// Maximum UTF-8 bytes of one opaque selected-content reference.
///
/// Contract 062 fixes 512 bytes for an opaque reference. The bound is enforced
/// where a reference is declared, before prepare and before any provider work,
/// so an over-long capability can never reach a host resolution or a projected
/// row.
pub const MAX_SELECTED_CONTENT_REFERENCE_BYTES: usize = 512;
/// Maximum required references one selected skill bundle may declare.
///
/// This mirrors the Contract 062 approved-source maximum. A bundle that needs
/// more references is rejected before provider work rather than truncated.
pub const MAX_SELECTED_SKILL_REQUIRED_REFERENCES: usize = 32;

/// Positive consumer-declared bounds for one selected skill bundle.
///
/// Consumer bounds may only narrow the first-tranche ceilings.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SelectedSkillBundleBounds {
    max_required_references: usize,
    max_aggregate_content_bytes: usize,
}

impl SelectedSkillBundleBounds {
    /// Creates positive bundle bounds that never widen the fixed ceilings.
    pub fn new(
        max_required_references: usize,
        max_aggregate_content_bytes: usize,
    ) -> Result<Self, RegisteredToolFailure> {
        if max_aggregate_content_bytes == 0 {
            return Err(reject(RegisteredToolFailureKind::IdentityRejected));
        }
        if max_required_references > MAX_SELECTED_SKILL_REQUIRED_REFERENCES
            || max_aggregate_content_bytes > MAX_SELECTED_SKILL_CONTENT_BYTES
        {
            return Err(reject(RegisteredToolFailureKind::LimitExceeded));
        }
        Ok(Self {
            max_required_references,
            max_aggregate_content_bytes,
        })
    }

    /// Returns the widest bundle bounds this slice admits.
    #[must_use]
    pub const fn ceiling() -> Self {
        Self {
            max_required_references: MAX_SELECTED_SKILL_REQUIRED_REFERENCES,
            max_aggregate_content_bytes: MAX_SELECTED_SKILL_CONTENT_BYTES,
        }
    }

    /// Returns the maximum required references one bundle may declare.
    #[must_use]
    pub const fn max_required_references(self) -> usize {
        self.max_required_references
    }

    /// Returns the maximum aggregate selected-content bytes.
    #[must_use]
    pub const fn max_aggregate_content_bytes(self) -> usize {
        self.max_aggregate_content_bytes
    }
}
