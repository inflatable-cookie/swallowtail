use super::validation::{failure, validate_provider_session_history_request};
use super::window::ProviderSessionHistoryWindow;
use super::{
    ProviderSessionHistoryCursor, ProviderSessionHistoryPlan, ProviderSessionHistoryRequest,
    ProviderSessionHistoryTotal,
};
use crate::{CleanupOutcome, RuntimeFailure, SessionReplayItem};

/// Validated newest-first history page with metadata and joined cleanup truth.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSessionHistoryPage {
    items: Vec<SessionReplayItem>,
    has_older: bool,
    older_cursor: Option<ProviderSessionHistoryCursor>,
    total: ProviderSessionHistoryTotal,
    cleanup: CleanupOutcome,
}

impl ProviderSessionHistoryPage {
    /// Validates request correlation, window honesty, and cleanup completion.
    pub fn new(
        plan: &ProviderSessionHistoryPlan,
        request: &ProviderSessionHistoryRequest,
        window: ProviderSessionHistoryWindow,
        cleanup: CleanupOutcome,
    ) -> Result<Self, RuntimeFailure> {
        validate_provider_session_history_request(plan, request)?;
        if !matches!(
            cleanup,
            CleanupOutcome::Clean | CleanupOutcome::NotApplicable
        ) {
            return Err(failure(
                "swallowtail.provider_session_history.cleanup_incomplete",
                "Provider-session history cleanup did not complete",
            ));
        }
        if window.has_older() != window.older_cursor().is_some() {
            return Err(failure(
                "swallowtail.provider_session_history.cursor_invalid",
                "Provider-session history older cursor does not match has_older",
            ));
        }
        if let Some(cursor) = window.older_cursor()
            && !cursor.matches_plan(plan)
        {
            return Err(failure(
                "swallowtail.provider_session_history.cursor_plan_mismatch",
                "Provider-session history older cursor does not match its immutable plan",
            ));
        }
        let (items, has_older, older_cursor, total) = window.into_parts();
        let page_bound = plan.agreement().bounds().maximum_page_items().get() as usize;
        let page_bytes = plan.agreement().bounds().maximum_page_bytes().get();
        let content_bytes = items.iter().try_fold(0u64, |total, item| {
            total.checked_add(u64::try_from(super::content_bytes(item)).ok()?)
        });
        if items.len() > page_bound
            || content_bytes.is_none_or(|bytes| bytes > page_bytes)
            || (request.older_cursor().is_some() && items.is_empty())
            || (!has_older && older_cursor.is_some())
        {
            return Err(failure(
                "swallowtail.provider_session_history.page_invalid",
                "Provider-session history page exceeds or contradicts its bound",
            ));
        }
        Ok(Self {
            items,
            has_older,
            older_cursor,
            total,
            cleanup,
        })
    }

    /// Iterates over the ascending items in this page window.
    pub fn items(&self) -> impl ExactSizeIterator<Item = &SessionReplayItem> {
        self.items.iter()
    }

    /// Returns how many items this page contains.
    #[must_use]
    pub fn fetched_count(&self) -> u32 {
        u32::try_from(self.items.len()).unwrap_or(u32::MAX)
    }

    /// Returns whether another older page may be requested.
    #[must_use]
    pub const fn has_older(&self) -> bool {
        self.has_older
    }

    /// Returns the plan-bound older cursor when `has_older` is true.
    #[must_use]
    pub const fn older_cursor(&self) -> Option<&ProviderSessionHistoryCursor> {
        self.older_cursor.as_ref()
    }

    /// Returns the honest total cardinality for UI chrome.
    #[must_use]
    pub const fn total(&self) -> ProviderSessionHistoryTotal {
        self.total
    }

    /// Returns joined-cleanup truth for the history-page operation.
    #[must_use]
    pub const fn cleanup(&self) -> &CleanupOutcome {
        &self.cleanup
    }
}
