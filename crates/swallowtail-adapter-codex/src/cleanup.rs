//! Cleanup-outcome composition shared by every Codex teardown path.

use swallowtail_runtime::CleanupOutcome;

/// Combines two cleanup outcomes, keeping the more severe one.
///
/// A failure beats a degraded result, which beats a clean one. Two
/// non-failing outcomes keep the first unless the second carries a positive
/// `Clean` where the first had none.
pub(crate) fn merge_cleanup(current: CleanupOutcome, next: CleanupOutcome) -> CleanupOutcome {
    match (&current, &next) {
        (CleanupOutcome::Failed(_), _) => current,
        (_, CleanupOutcome::Failed(_)) => next,
        (CleanupOutcome::Degraded(_), _) => current,
        (_, CleanupOutcome::Degraded(_)) => next,
        (CleanupOutcome::Clean, CleanupOutcome::NotApplicable) => current,
        (CleanupOutcome::NotApplicable, CleanupOutcome::Clean) => next,
        _ => current,
    }
}
