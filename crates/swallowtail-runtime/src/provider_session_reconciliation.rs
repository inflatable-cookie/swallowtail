#![deny(missing_docs)]

use crate::{
    CancellationControl, Deadline, ProviderOperationCheckpoint, RuntimeFailure, RuntimeTurnId,
    SessionReplayItem, SessionResumeBinding,
};
use std::num::{NonZeroU32, NonZeroU64};
use swallowtail_core::{CancellationScope, TurnRef};

mod outcome;
mod validation;

use validation::{failure, validate_plan};

pub use outcome::{ProviderSessionReconciliationOutcome, bound_provider_session_replay_tail};
pub use validation::{
    validate_provider_session_reconciliation_execution,
    validate_provider_session_reconciliation_request,
};

/// Read-only observed state of a turn whose runtime handle was lost.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InterruptedTurnState {
    /// Provider evidence shows the turn remains active.
    Active,
    /// Provider evidence shows the turn is waiting for provider input.
    WaitingForProviderInput,
    /// Exact provider-turn evidence shows successful completion.
    Completed,
    /// Exact provider-turn evidence shows failure.
    Failed,
    /// Exact provider-turn evidence shows cancellation.
    Cancelled,
    /// The session is inactive but the interrupted turn remains unresolved.
    InactiveUnresolved,
    /// Available provider evidence cannot classify the state safely.
    Unknown,
}

impl InterruptedTurnState {
    /// Returns whether exact provider-turn attribution is required.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }
}

plan_family!(@prepared {
    plan_type: ProviderSessionReconciliationPlan,
    prepared_type: PreparedProviderSessionReconciliationEvidence,
    agreement: ProviderSessionReconciliationAgreement,
    prepared_doc: "Prepared route and access evidence for session reconciliation.",
    agreement_doc: "Returns the immutable reconciliation agreement.",
});
/// Strength of provider attribution for an interrupted-turn observation.
/// Strength of provider attribution for an interrupted-turn observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InterruptedTurnAttribution {
    /// Evidence identifies the exact provider turn.
    ExactProviderTurn,
    /// Evidence describes only the bound provider session.
    ProviderSession,
}

/// Immutable item and byte bounds for reconciliation replay.
/// Immutable item and byte bounds for reconciliation replay.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProviderSessionReconciliationBounds {
    maximum_replay_items: NonZeroU32,
    maximum_replay_bytes: NonZeroU64,
}

impl ProviderSessionReconciliationBounds {
    /// Creates positive replay item and content bounds.
    #[must_use]
    pub const fn new(maximum_replay_items: NonZeroU32, maximum_replay_bytes: NonZeroU64) -> Self {
        Self {
            maximum_replay_items,
            maximum_replay_bytes,
        }
    }

    #[must_use]
    /// Returns the maximum replay item count.
    pub const fn maximum_replay_items(self) -> NonZeroU32 {
        self.maximum_replay_items
    }

    #[must_use]
    /// Returns the maximum aggregate replay content bytes.
    pub const fn maximum_replay_bytes(self) -> NonZeroU64 {
        self.maximum_replay_bytes
    }
}

/// Exact durable binding, interrupted turn, bounds, and deadline to reconcile.
/// Exact durable binding, interrupted turn, bounds, and deadline to reconcile.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSessionReconciliationAgreement {
    binding: SessionResumeBinding,
    interrupted_turn_id: RuntimeTurnId,
    provider_turn_ref: Option<TurnRef>,
    bounds: ProviderSessionReconciliationBounds,
    deadline: Option<Deadline>,
    checkpoint: Option<ProviderOperationCheckpoint>,
}

impl ProviderSessionReconciliationAgreement {
    /// Creates an agreement without an event-position checkpoint.
    #[must_use]
    pub const fn new(
        binding: SessionResumeBinding,
        interrupted_turn_id: RuntimeTurnId,
        provider_turn_ref: Option<TurnRef>,
        bounds: ProviderSessionReconciliationBounds,
        deadline: Option<Deadline>,
    ) -> Self {
        Self {
            binding,
            interrupted_turn_id,
            provider_turn_ref,
            bounds,
            deadline,
            checkpoint: None,
        }
    }

    /// Adds a checkpoint that exactly matches the session and interrupted turn.
    pub fn with_checkpoint(
        mut self,
        checkpoint: ProviderOperationCheckpoint,
    ) -> Result<Self, RuntimeFailure> {
        if checkpoint.provider_session_ref() != self.binding.provider_session_ref()
            || checkpoint.runtime_turn_id() != &self.interrupted_turn_id
            || self.provider_turn_ref.as_ref() != Some(checkpoint.provider_turn_ref())
        {
            return Err(failure(
                "swallowtail.provider_session_reconciliation.checkpoint_mismatch",
                "Provider operation checkpoint does not match the reconciliation agreement",
            ));
        }
        self.checkpoint = Some(checkpoint);
        Ok(self)
    }

    #[must_use]
    /// Returns the durable provider-session binding.
    pub const fn binding(&self) -> &SessionResumeBinding {
        &self.binding
    }

    #[must_use]
    /// Returns the consumer turn whose handle was lost.
    pub const fn interrupted_turn_id(&self) -> &RuntimeTurnId {
        &self.interrupted_turn_id
    }

    #[must_use]
    /// Returns the exact provider turn when known before observation.
    pub const fn provider_turn_ref(&self) -> Option<&TurnRef> {
        self.provider_turn_ref.as_ref()
    }

    #[must_use]
    /// Returns the replay bounds.
    pub const fn bounds(&self) -> ProviderSessionReconciliationBounds {
        self.bounds
    }

    #[must_use]
    /// Returns the observation deadline when present.
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline
    }

    #[must_use]
    /// Returns the exact durable event position when supplied.
    pub const fn checkpoint(&self) -> Option<&ProviderOperationCheckpoint> {
        self.checkpoint.as_ref()
    }
}

use crate::plan_family::plan_family;

plan_family! {
    plan: {
        plan_type: ProviderSessionReconciliationPlan,
        prepared_type: PreparedProviderSessionReconciliationEvidence,
        agreement: ProviderSessionReconciliationAgreement,
        plan_doc: "Validated preflight plan and immutable reconciliation agreement.",
        prepared_doc: "Prepared route and access evidence for session reconciliation.",
        agreement_doc: "Returns the immutable reconciliation agreement.",
    }
    requests: {
        plan_type: ProviderSessionReconciliationPlan,
        agreement: ProviderSessionReconciliationAgreement,
        agreement_doc: "Returns the immutable reconciliation agreement.",
        scope: CancellationScope::ProviderSessionReconciliation,
        ns: "swallowtail.provider_session_reconciliation",
        requests: [
            ProviderSessionReconciliationRequest = "One execution request derived from a reconciliation plan." {
                new_doc: "Creates a request with an explicitly scoped cancellation control.",
                new_arg: plan: &ProviderSessionReconciliationPlan,
                agreement_expr: plan.agreement().clone(),
                from_plan_doc: "Creates a request with a fresh correctly scoped cancellation control.",
                from_plan_arg: pass_plan,
                request_id_doc: "Returns the caller-assigned request identity.",
                extra: true,
                extra_code: "swallowtail.provider_session_reconciliation.cancellation_scope_mismatch",
                extra_message: "",
            }
        ]
    }
}

/// Adapter-produced read-only observation before outcome validation.
/// Adapter-produced read-only observation before outcome validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSessionReconciliationObservation {
    attribution: InterruptedTurnAttribution,
    state: InterruptedTurnState,
    provider_turn_ref: Option<TurnRef>,
    replay: Vec<SessionReplayItem>,
    replay_complete: bool,
}

impl ProviderSessionReconciliationObservation {
    /// Creates a session-scoped observation with bounded replay.
    #[must_use]
    pub fn session_scoped(
        state: InterruptedTurnState,
        replay: Vec<SessionReplayItem>,
        replay_complete: bool,
    ) -> Self {
        Self {
            attribution: InterruptedTurnAttribution::ProviderSession,
            state,
            provider_turn_ref: None,
            replay,
            replay_complete,
        }
    }

    #[must_use]
    /// Creates an observation attributed to one exact provider turn.
    pub fn exact_turn(
        state: InterruptedTurnState,
        provider_turn_ref: TurnRef,
        replay: Vec<SessionReplayItem>,
        replay_complete: bool,
    ) -> Self {
        Self {
            attribution: InterruptedTurnAttribution::ExactProviderTurn,
            state,
            provider_turn_ref: Some(provider_turn_ref),
            replay,
            replay_complete,
        }
    }
}
