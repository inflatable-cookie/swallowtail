use super::validation::{failure, validate_provider_session_reconciliation_request};
use super::{
    InterruptedTurnAttribution, InterruptedTurnState, ProviderSessionReconciliationBounds,
    ProviderSessionReconciliationObservation, ProviderSessionReconciliationPlan,
    ProviderSessionReconciliationRequest,
};
use crate::{CleanupOutcome, RuntimeFailure, RuntimeTurnId, SessionReplayItem};
use swallowtail_core::TurnRef;

/// Keeps the newest complete replay items within one immutable reconciliation
/// bound and reports whether the input snapshot was preserved in full.
#[must_use]
pub fn bound_provider_session_replay_tail(
    replay: Vec<SessionReplayItem>,
    bounds: ProviderSessionReconciliationBounds,
) -> (Vec<SessionReplayItem>, bool) {
    let original_len = replay.len();
    let item_limit = usize::try_from(bounds.maximum_replay_items().get()).unwrap_or(usize::MAX);
    let byte_limit = usize::try_from(bounds.maximum_replay_bytes().get()).unwrap_or(usize::MAX);
    let mut bytes = 0usize;
    let mut selected = Vec::new();
    for item in replay.into_iter().rev() {
        let item_bytes = item.content().map_or(0, |content| content.byte_len());
        if selected.len() == item_limit || bytes.saturating_add(item_bytes) > byte_limit {
            break;
        }
        bytes += item_bytes;
        selected.push(item);
    }
    let complete = selected.len() == original_len;
    selected.reverse();
    (selected, complete)
}

/// Validated reconciliation result with replay and joined-cleanup truth.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSessionReconciliationOutcome {
    interrupted_turn_id: RuntimeTurnId,
    observation: ProviderSessionReconciliationObservation,
    cleanup: CleanupOutcome,
}

impl ProviderSessionReconciliationOutcome {
    /// Validates request correlation, attribution, replay bounds, and cleanup.
    pub fn new(
        plan: &ProviderSessionReconciliationPlan,
        request: &ProviderSessionReconciliationRequest,
        observation: ProviderSessionReconciliationObservation,
        cleanup: CleanupOutcome,
    ) -> Result<Self, RuntimeFailure> {
        validate_provider_session_reconciliation_request(plan, request)?;
        if !matches!(
            cleanup,
            CleanupOutcome::Clean | CleanupOutcome::NotApplicable
        ) {
            return Err(failure(
                "swallowtail.provider_session_reconciliation.cleanup_incomplete",
                "Provider-session reconciliation cleanup did not complete",
            ));
        }
        let agreement = plan.agreement();
        let exact = observation.attribution == InterruptedTurnAttribution::ExactProviderTurn;
        if exact != observation.provider_turn_ref.is_some()
            || exact != agreement.provider_turn_ref().is_some()
            || (exact && observation.provider_turn_ref.as_ref() != agreement.provider_turn_ref())
        {
            return Err(failure(
                "swallowtail.provider_session_reconciliation.attribution_mismatch",
                "Provider-session reconciliation attribution is not exact",
            ));
        }
        let limit = agreement.bounds();
        let replay_bytes = observation.replay.iter().try_fold(0usize, |total, item| {
            total.checked_add(item.content().map_or(0, |content| content.byte_len()))
        });
        if observation.replay.len()
            > usize::try_from(limit.maximum_replay_items().get()).unwrap_or(usize::MAX)
            || replay_bytes.is_none_or(|bytes| {
                u64::try_from(bytes).unwrap_or(u64::MAX) > limit.maximum_replay_bytes().get()
            })
            || observation.replay.iter().any(|item| {
                item.provider_session_ref() != agreement.binding().provider_session_ref()
            })
            || observation
                .replay
                .windows(2)
                .any(|pair| pair[0].sequence() >= pair[1].sequence())
        {
            return Err(failure(
                "swallowtail.provider_session_reconciliation.replay_invalid",
                "Provider-session reconciliation replay exceeds or contradicts its bound",
            ));
        }
        if observation.state.is_terminal() && !exact {
            return Err(failure(
                "swallowtail.provider_session_reconciliation.terminal_attribution_required",
                "Terminal reconciliation requires exact provider-turn attribution",
            ));
        }
        Ok(Self {
            interrupted_turn_id: agreement.interrupted_turn_id().clone(),
            observation,
            cleanup,
        })
    }

    #[must_use]
    /// Returns the interrupted consumer turn.
    pub const fn interrupted_turn_id(&self) -> &RuntimeTurnId {
        &self.interrupted_turn_id
    }

    #[must_use]
    /// Returns the observation's provider-attribution strength.
    pub const fn attribution(&self) -> InterruptedTurnAttribution {
        self.observation.attribution
    }

    #[must_use]
    /// Returns the observed interrupted-turn state.
    pub const fn state(&self) -> InterruptedTurnState {
        self.observation.state
    }

    #[must_use]
    /// Returns the exact provider turn when attribution permits it.
    pub const fn provider_turn_ref(&self) -> Option<&TurnRef> {
        self.observation.provider_turn_ref.as_ref()
    }

    /// Iterates over the bounded replacement replay snapshot.
    pub fn replay(&self) -> impl ExactSizeIterator<Item = &SessionReplayItem> {
        self.observation.replay.iter()
    }

    #[must_use]
    /// Returns whether replay contains the complete qualified snapshot.
    pub const fn replay_complete(&self) -> bool {
        self.observation.replay_complete
    }

    #[must_use]
    /// Returns joined-cleanup truth for the observation operation.
    pub const fn cleanup(&self) -> &CleanupOutcome {
        &self.cleanup
    }

    #[cfg(test)]
    pub(crate) fn fixture(state: InterruptedTurnState) -> Self {
        let exact = state.is_terminal();
        Self {
            interrupted_turn_id: RuntimeTurnId::new("fixture-interrupted-turn")
                .expect("fixture turn id is valid"),
            observation: ProviderSessionReconciliationObservation {
                attribution: if exact {
                    InterruptedTurnAttribution::ExactProviderTurn
                } else {
                    InterruptedTurnAttribution::ProviderSession
                },
                state,
                provider_turn_ref: exact.then(|| {
                    TurnRef::new("fixture-provider-turn").expect("fixture turn ref is valid")
                }),
                replay: Vec::new(),
                replay_complete: true,
            },
            cleanup: CleanupOutcome::Clean,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ProviderSessionReconciliationBounds, bound_provider_session_replay_tail};
    use crate::{OperationContent, SessionReplayItem, SessionReplayKind};
    use std::num::{NonZeroU32, NonZeroU64};
    use swallowtail_core::SessionRef;

    fn replay_item(sequence: u64, content: &str) -> SessionReplayItem {
        SessionReplayItem::with_content(
            SessionRef::new("provider-session").expect("session ref is valid"),
            sequence,
            SessionReplayKind::AgentMessage,
            OperationContent::new(content).expect("content is valid"),
        )
    }

    #[test]
    fn replay_tail_keeps_the_newest_contiguous_items() {
        let (replay, complete) = bound_provider_session_replay_tail(
            vec![
                replay_item(0, "first"),
                replay_item(1, "second"),
                replay_item(2, "third"),
            ],
            ProviderSessionReconciliationBounds::new(
                NonZeroU32::new(2).expect("bound is non-zero"),
                NonZeroU64::new(64).expect("bound is non-zero"),
            ),
        );

        assert!(!complete);
        assert_eq!(
            replay
                .iter()
                .map(SessionReplayItem::sequence)
                .collect::<Vec<_>>(),
            [1, 2]
        );
    }

    #[test]
    fn oversized_newest_item_does_not_create_a_non_contiguous_snapshot() {
        let (replay, complete) = bound_provider_session_replay_tail(
            vec![replay_item(0, "fits"), replay_item(1, "does-not-fit")],
            ProviderSessionReconciliationBounds::new(
                NonZeroU32::new(2).expect("bound is non-zero"),
                NonZeroU64::new(4).expect("bound is non-zero"),
            ),
        );

        assert!(replay.is_empty());
        assert!(!complete);
    }
}
