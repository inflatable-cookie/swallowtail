#![deny(missing_docs)]

use crate::{
    CancellationControl, Deadline, ImmediateCancellation, PreparationFailure,
    PreparedAccessEvidence, PreparedOperationEvidence, ProviderOperationCheckpoint, RequestId,
    RuntimeFailure, RuntimeTurnId, SessionReplayItem, SessionResumeBinding,
};
use std::num::{NonZeroU32, NonZeroU64};
use std::sync::Arc;
use swallowtail_core::{CancellationScope, PreflightPlan, TurnRef};

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

/// Validated preflight plan and immutable reconciliation agreement.
/// Validated preflight plan and immutable reconciliation agreement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSessionReconciliationPlan {
    preflight: PreflightPlan,
    agreement: ProviderSessionReconciliationAgreement,
}

/// Prepared route and access evidence for session reconciliation.
/// Prepared route and access evidence for session reconciliation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedProviderSessionReconciliationEvidence {
    operation: PreparedOperationEvidence,
    plan: ProviderSessionReconciliationPlan,
}

impl PreparedProviderSessionReconciliationEvidence {
    /// Binds prepared access evidence to a validated reconciliation plan.
    pub fn from_plan(
        plan: ProviderSessionReconciliationPlan,
        access: PreparedAccessEvidence,
    ) -> Result<Self, PreparationFailure> {
        let operation = PreparedOperationEvidence::from_plan(plan.preflight().clone(), access)?;
        Ok(Self { operation, plan })
    }

    #[must_use]
    /// Returns the common prepared-operation evidence.
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    #[must_use]
    /// Returns the exact reconciliation plan.
    pub const fn plan(&self) -> &ProviderSessionReconciliationPlan {
        &self.plan
    }
}

impl ProviderSessionReconciliationPlan {
    /// Validates a preflight plan against the reconciliation agreement.
    pub fn new(
        preflight: PreflightPlan,
        agreement: ProviderSessionReconciliationAgreement,
    ) -> Result<Self, RuntimeFailure> {
        validate_plan(&preflight, &agreement)?;
        Ok(Self {
            preflight,
            agreement,
        })
    }

    #[must_use]
    /// Returns the immutable preflight plan.
    pub const fn preflight(&self) -> &PreflightPlan {
        &self.preflight
    }

    #[must_use]
    /// Returns the immutable reconciliation agreement.
    pub const fn agreement(&self) -> &ProviderSessionReconciliationAgreement {
        &self.agreement
    }
}

/// One execution request derived from a reconciliation plan.
/// One execution request derived from a reconciliation plan.
#[derive(Clone, Debug)]
pub struct ProviderSessionReconciliationRequest {
    request_id: RequestId,
    agreement: ProviderSessionReconciliationAgreement,
    cancellation: Arc<ImmediateCancellation>,
}

impl ProviderSessionReconciliationRequest {
    /// Creates a request with an explicitly scoped cancellation control.
    pub fn new(
        request_id: RequestId,
        plan: &ProviderSessionReconciliationPlan,
        cancellation: Arc<ImmediateCancellation>,
    ) -> Result<Self, RuntimeFailure> {
        if cancellation.scope() != CancellationScope::ProviderSessionReconciliation {
            return Err(failure(
                "swallowtail.provider_session_reconciliation.cancellation_scope_mismatch",
                "Provider-session reconciliation request has the wrong cancellation scope",
            ));
        }
        Ok(Self {
            request_id,
            agreement: plan.agreement().clone(),
            cancellation,
        })
    }

    /// Creates a request with a fresh correctly scoped cancellation control.
    pub fn from_plan(
        request_id: RequestId,
        plan: &ProviderSessionReconciliationPlan,
    ) -> Result<Self, RuntimeFailure> {
        Self::new(
            request_id,
            plan,
            Arc::new(ImmediateCancellation::new(
                CancellationScope::ProviderSessionReconciliation,
            )),
        )
    }

    #[must_use]
    /// Returns the caller-assigned request identity.
    pub const fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    #[must_use]
    /// Returns the copied immutable agreement.
    pub const fn agreement(&self) -> &ProviderSessionReconciliationAgreement {
        &self.agreement
    }

    #[must_use]
    /// Returns the reconciliation-scoped cancellation control.
    pub const fn cancellation(&self) -> &Arc<ImmediateCancellation> {
        &self.cancellation
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
