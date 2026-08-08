use super::validation::{failure, validate_provider_session_history_request};
use super::{
    ProviderSessionHistoryCursor, ProviderSessionHistoryPlan, ProviderSessionHistoryRequest,
    ProviderSessionHistoryTotal, content_bytes,
};
use crate::{RuntimeFailure, SessionReplayItem};

/// One newest-first page window sliced from an ascending provider snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSessionHistoryWindow {
    items: Vec<SessionReplayItem>,
    has_older: bool,
    older_cursor: Option<ProviderSessionHistoryCursor>,
    total: ProviderSessionHistoryTotal,
}

impl ProviderSessionHistoryWindow {
    /// Returns the ascending items for this page window.
    pub fn items(&self) -> impl ExactSizeIterator<Item = &SessionReplayItem> {
        self.items.iter()
    }

    /// Returns how many items this window contains.
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

    pub(super) fn into_parts(
        self,
    ) -> (
        Vec<SessionReplayItem>,
        bool,
        Option<ProviderSessionHistoryCursor>,
        ProviderSessionHistoryTotal,
    ) {
        (self.items, self.has_older, self.older_cursor, self.total)
    }
}

/// Slices one newest-first page from an ascending provider-session snapshot.
///
/// The snapshot must already be projected into portable [`SessionReplayItem`]
/// values in ascending replay order. Traversal is newest-window-first: the
/// first request without a cursor returns the newest bound window; each older
/// cursor moves strictly toward earlier history. Within the returned window,
/// items remain ascending so consumers can prepend older pages without
/// re-sorting.
pub fn page_provider_session_history_window(
    plan: &ProviderSessionHistoryPlan,
    request: &ProviderSessionHistoryRequest,
    ascending_snapshot: Vec<SessionReplayItem>,
    total: ProviderSessionHistoryTotal,
) -> Result<ProviderSessionHistoryWindow, RuntimeFailure> {
    validate_provider_session_history_request(plan, request)?;
    let bounds = plan.agreement().bounds();
    let snapshot_limit =
        usize::try_from(bounds.maximum_snapshot_items().get()).unwrap_or(usize::MAX);
    if ascending_snapshot.len() > snapshot_limit {
        return Err(failure(
            "swallowtail.provider_session_history.snapshot_limit_exceeded",
            "Provider-session history snapshot exceeds its planned bound",
        ));
    }
    validate_snapshot(plan, &ascending_snapshot)?;
    validate_total(total, ascending_snapshot.len())?;

    let exclusive_end = match request.older_cursor() {
        None => ascending_snapshot.len(),
        Some(cursor) => {
            if cursor.older_end() > ascending_snapshot.len() {
                return Err(failure(
                    "swallowtail.provider_session_history.cursor_invalid",
                    "Provider-session history cursor is outside the bound snapshot",
                ));
            }
            cursor.older_end()
        }
    };

    let item_limit = usize::try_from(bounds.maximum_page_items().get()).unwrap_or(usize::MAX);
    let byte_limit = usize::try_from(bounds.maximum_page_bytes().get()).unwrap_or(usize::MAX);
    let mut bytes = 0usize;
    let mut selected = Vec::new();
    let mut start = exclusive_end;
    while start > 0 && selected.len() < item_limit {
        let candidate = &ascending_snapshot[start - 1];
        let item_bytes = content_bytes(candidate);
        if bytes.saturating_add(item_bytes) > byte_limit {
            break;
        }
        bytes += item_bytes;
        start -= 1;
        selected.push(candidate.clone());
    }
    selected.reverse();

    if selected.is_empty() && exclusive_end > 0 {
        return Err(failure(
            if request.older_cursor().is_some() {
                "swallowtail.provider_session_history.empty_continuation"
            } else {
                "swallowtail.provider_session_history.page_limit_exceeded"
            },
            if request.older_cursor().is_some() {
                "Provider-session history continuation page is empty"
            } else {
                "Provider-session history page cannot fit the next older item"
            },
        ));
    }

    let has_older = start > 0;
    let older_cursor = if has_older {
        Some(ProviderSessionHistoryCursor::from_older_end(plan, start)?)
    } else {
        None
    };

    Ok(ProviderSessionHistoryWindow {
        items: selected,
        has_older,
        older_cursor,
        total,
    })
}

fn validate_snapshot(
    plan: &ProviderSessionHistoryPlan,
    ascending_snapshot: &[SessionReplayItem],
) -> Result<(), RuntimeFailure> {
    let session = plan.agreement().binding().provider_session_ref();
    if ascending_snapshot
        .iter()
        .any(|item| item.provider_session_ref() != session)
    {
        return Err(failure(
            "swallowtail.provider_session_history.session_mismatch",
            "Provider-session history item does not match the bound session",
        ));
    }
    if ascending_snapshot
        .windows(2)
        .any(|pair| pair[0].sequence() >= pair[1].sequence())
    {
        return Err(failure(
            "swallowtail.provider_session_history.replay_order_invalid",
            "Provider-session history snapshot is not strictly ascending",
        ));
    }
    Ok(())
}

fn validate_total(
    total: ProviderSessionHistoryTotal,
    snapshot_len: usize,
) -> Result<(), RuntimeFailure> {
    let snapshot_len = u32::try_from(snapshot_len).map_err(|_| {
        failure(
            "swallowtail.provider_session_history.total_invalid",
            "Provider-session history total exceeds portable cardinality",
        )
    })?;
    match total {
        ProviderSessionHistoryTotal::Exact(count) if count != snapshot_len => Err(failure(
            "swallowtail.provider_session_history.total_invalid",
            "Provider-session history Exact total does not match the snapshot",
        )),
        ProviderSessionHistoryTotal::AtLeast(count) if count < snapshot_len => Err(failure(
            "swallowtail.provider_session_history.total_invalid",
            "Provider-session history AtLeast total is below the snapshot",
        )),
        ProviderSessionHistoryTotal::Exact(_)
        | ProviderSessionHistoryTotal::AtLeast(_)
        | ProviderSessionHistoryTotal::Unknown => Ok(()),
    }
}
