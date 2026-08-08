#![deny(missing_docs)]

//! Read-only newest-first provider-session history pages (Contract 054).

use crate::{
    CancellationControl, Deadline, ImmediateCancellation, OperationContent,
    ProviderSessionHistoryId, RuntimeFailure, SessionReplayItem, SessionResumeBinding,
};
use std::fmt;
use std::num::{NonZeroU32, NonZeroU64};
use std::sync::Arc;
use swallowtail_core::{CancellationScope, PreflightPlan};

mod page;
mod validation;
mod window;

#[cfg(test)]
mod tests;

pub use page::ProviderSessionHistoryPage;
pub use validation::{
    validate_provider_session_history_execution, validate_provider_session_history_request,
};
pub use window::{ProviderSessionHistoryWindow, page_provider_session_history_window};

use validation::{failure, validate_plan};

/// Honest cardinality for UI chrome over provider history.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderSessionHistoryTotal {
    /// Exact item count is known for the bound session snapshot.
    Exact(u32),
    /// Only a lower bound is known.
    AtLeast(u32),
    /// No honest total is available.
    Unknown,
}

/// Immutable item, byte, cursor, and snapshot bounds for one history page plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProviderSessionHistoryBounds {
    maximum_page_items: NonZeroU32,
    maximum_page_bytes: NonZeroU64,
    maximum_cursor_bytes: NonZeroU32,
    maximum_snapshot_items: NonZeroU32,
}

impl ProviderSessionHistoryBounds {
    /// Creates positive page, cursor, and snapshot bounds.
    #[must_use]
    pub const fn new(
        maximum_page_items: NonZeroU32,
        maximum_page_bytes: NonZeroU64,
        maximum_cursor_bytes: NonZeroU32,
        maximum_snapshot_items: NonZeroU32,
    ) -> Self {
        Self {
            maximum_page_items,
            maximum_page_bytes,
            maximum_cursor_bytes,
            maximum_snapshot_items,
        }
    }

    /// Returns the maximum items in one returned page.
    #[must_use]
    pub const fn maximum_page_items(self) -> NonZeroU32 {
        self.maximum_page_items
    }

    /// Returns the maximum aggregate content bytes in one returned page.
    #[must_use]
    pub const fn maximum_page_bytes(self) -> NonZeroU64 {
        self.maximum_page_bytes
    }

    /// Returns the maximum opaque cursor byte length.
    #[must_use]
    pub const fn maximum_cursor_bytes(self) -> NonZeroU32 {
        self.maximum_cursor_bytes
    }

    /// Returns the maximum ascending snapshot size before paging fails closed.
    #[must_use]
    pub const fn maximum_snapshot_items(self) -> NonZeroU32 {
        self.maximum_snapshot_items
    }
}

/// Opaque older-page cursor bound to one exact history plan.
#[derive(Clone, Eq, PartialEq)]
pub struct ProviderSessionHistoryCursor {
    history_id: ProviderSessionHistoryId,
    source_preflight: PreflightPlan,
    /// Exclusive end index into an ascending snapshot for the next older page.
    older_end: usize,
    value: String,
}

impl ProviderSessionHistoryCursor {
    /// Creates a plan-bound cursor for synthetic or native older-page traversal.
    pub fn new(
        plan: &ProviderSessionHistoryPlan,
        older_end: usize,
        value: impl Into<String>,
    ) -> Result<Self, RuntimeFailure> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(failure(
                "swallowtail.provider_session_history.cursor_required",
                "Provider-session history cursor must not be empty",
            ));
        }
        if value.len() > plan.agreement().bounds().maximum_cursor_bytes().get() as usize {
            return Err(failure(
                "swallowtail.provider_session_history.cursor_limit_exceeded",
                "Provider-session history cursor exceeds its planned bound",
            ));
        }
        if older_end == 0 {
            return Err(failure(
                "swallowtail.provider_session_history.cursor_exhausted",
                "Provider-session history cursor does not advance toward older items",
            ));
        }
        Ok(Self {
            history_id: plan.agreement().history_id().clone(),
            source_preflight: plan.preflight().clone(),
            older_end,
            value,
        })
    }

    /// Builds a synthetic cursor from an exclusive older-end index.
    pub fn from_older_end(
        plan: &ProviderSessionHistoryPlan,
        older_end: usize,
    ) -> Result<Self, RuntimeFailure> {
        Self::new(plan, older_end, older_end.to_string())
    }

    /// Returns the exclusive end index used by synthetic snapshot paging.
    #[must_use]
    pub const fn older_end(&self) -> usize {
        self.older_end
    }

    /// Returns the provider-native or synthetic cursor value for adapters.
    #[must_use]
    pub fn as_provider_value(&self) -> &str {
        &self.value
    }

    pub(crate) fn matches_plan(&self, plan: &ProviderSessionHistoryPlan) -> bool {
        self.history_id == *plan.agreement().history_id()
            && self.source_preflight == *plan.preflight()
    }
}

impl fmt::Debug for ProviderSessionHistoryCursor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ProviderSessionHistoryCursor")
            .field("history_id", &self.history_id)
            .field("older_end", &self.older_end)
            .field(
                "value",
                &format_args!("<opaque:{} bytes>", self.value.len()),
            )
            .finish()
    }
}

/// Exact durable binding, bounds, and deadline for one history-page operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSessionHistoryAgreement {
    history_id: ProviderSessionHistoryId,
    binding: SessionResumeBinding,
    bounds: ProviderSessionHistoryBounds,
    deadline: Option<Deadline>,
}

impl ProviderSessionHistoryAgreement {
    /// Creates an agreement for one bound session history traversal.
    #[must_use]
    pub const fn new(
        history_id: ProviderSessionHistoryId,
        binding: SessionResumeBinding,
        bounds: ProviderSessionHistoryBounds,
        deadline: Option<Deadline>,
    ) -> Self {
        Self {
            history_id,
            binding,
            bounds,
            deadline,
        }
    }

    /// Returns the history operation identity.
    #[must_use]
    pub const fn history_id(&self) -> &ProviderSessionHistoryId {
        &self.history_id
    }

    /// Returns the durable provider-session binding.
    #[must_use]
    pub const fn binding(&self) -> &SessionResumeBinding {
        &self.binding
    }

    /// Returns the page and snapshot bounds.
    #[must_use]
    pub const fn bounds(&self) -> ProviderSessionHistoryBounds {
        self.bounds
    }

    /// Returns the optional operation deadline.
    #[must_use]
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline
    }
}

use crate::plan_family::plan_family;

plan_family!(@prepared {
    plan_type: ProviderSessionHistoryPlan,
    prepared_type: PreparedProviderSessionHistoryEvidence,
    agreement: ProviderSessionHistoryAgreement,
    prepared_doc: "Prepared route and access evidence for history paging.",
    agreement_doc: "Returns the immutable history-page agreement.",
});

plan_family! {
    plan: {
        plan_type: ProviderSessionHistoryPlan,
        prepared_type: PreparedProviderSessionHistoryEvidence,
        agreement: ProviderSessionHistoryAgreement,
        plan_doc: "Validated preflight plan and immutable history-page agreement.",
        prepared_doc: "Prepared route and access evidence for history paging.",
        agreement_doc: "Returns the immutable history-page agreement.",
    }
}

/// Typed request for one newest-first provider-session history page.
#[derive(Clone, Debug)]
pub struct ProviderSessionHistoryRequest {
    request_id: crate::RequestId,
    agreement: ProviderSessionHistoryAgreement,
    older_cursor: Option<ProviderSessionHistoryCursor>,
    cancellation: Arc<ImmediateCancellation>,
}

impl ProviderSessionHistoryRequest {
    /// Creates a request after validating cancellation and cursor scope.
    pub fn new(
        request_id: crate::RequestId,
        plan: &ProviderSessionHistoryPlan,
        older_cursor: Option<ProviderSessionHistoryCursor>,
        cancellation: Arc<ImmediateCancellation>,
    ) -> Result<Self, RuntimeFailure> {
        if cancellation.scope() != CancellationScope::ProviderSessionHistory {
            return Err(failure(
                "swallowtail.provider_session_history.cancellation_scope_mismatch",
                "Provider-session history cancellation scope does not match",
            ));
        }
        if older_cursor
            .as_ref()
            .is_some_and(|cursor| !cursor.matches_plan(plan))
        {
            return Err(failure(
                "swallowtail.provider_session_history.cursor_plan_mismatch",
                "Provider-session history cursor does not match its immutable plan",
            ));
        }
        Ok(Self {
            request_id,
            agreement: plan.agreement().clone(),
            older_cursor,
            cancellation,
        })
    }

    /// Creates a request with a fresh correctly scoped cancellation control.
    pub fn from_plan(
        request_id: crate::RequestId,
        plan: &ProviderSessionHistoryPlan,
        older_cursor: Option<ProviderSessionHistoryCursor>,
    ) -> Result<Self, RuntimeFailure> {
        Self::new(
            request_id,
            plan,
            older_cursor,
            Arc::new(ImmediateCancellation::new(
                CancellationScope::ProviderSessionHistory,
            )),
        )
    }

    /// Returns the caller-assigned request identity.
    #[must_use]
    pub const fn request_id(&self) -> &crate::RequestId {
        &self.request_id
    }

    /// Returns the immutable agreement copied from the plan.
    #[must_use]
    pub const fn agreement(&self) -> &ProviderSessionHistoryAgreement {
        &self.agreement
    }

    /// Returns the older-page cursor when requesting a later page.
    #[must_use]
    pub const fn older_cursor(&self) -> Option<&ProviderSessionHistoryCursor> {
        self.older_cursor.as_ref()
    }

    /// Returns the history-scoped cancellation control.
    #[must_use]
    pub const fn cancellation(&self) -> &Arc<ImmediateCancellation> {
        &self.cancellation
    }
}

pub(crate) fn content_bytes(item: &SessionReplayItem) -> usize {
    item.content().map_or(0, OperationContent::byte_len)
}
