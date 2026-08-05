use crate::{
    CancellationControl, CleanupOutcome, Deadline, HostServices, ImmediateCancellation,
    PreparationFailure, PreparedAccessEvidence, PreparedOperationEvidence,
    ProviderOperationCheckpoint, RequestId, RuntimeFailure, RuntimeTurnId, SessionReplayItem,
    SessionResumeBinding,
};
use std::num::{NonZeroU32, NonZeroU64};
use std::sync::Arc;
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, DriverRole, ExecutionLayer,
    HostServiceKind, OperationShape, PreflightPlan, SafeDiagnostic, TurnRef,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InterruptedTurnState {
    Active,
    WaitingForProviderInput,
    Completed,
    Failed,
    Cancelled,
    InactiveUnresolved,
    Unknown,
}

impl InterruptedTurnState {
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InterruptedTurnAttribution {
    ExactProviderTurn,
    ProviderSession,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProviderSessionReconciliationBounds {
    maximum_replay_items: NonZeroU32,
    maximum_replay_bytes: NonZeroU64,
}

impl ProviderSessionReconciliationBounds {
    #[must_use]
    pub const fn new(maximum_replay_items: NonZeroU32, maximum_replay_bytes: NonZeroU64) -> Self {
        Self {
            maximum_replay_items,
            maximum_replay_bytes,
        }
    }

    #[must_use]
    pub const fn maximum_replay_items(self) -> NonZeroU32 {
        self.maximum_replay_items
    }

    #[must_use]
    pub const fn maximum_replay_bytes(self) -> NonZeroU64 {
        self.maximum_replay_bytes
    }
}

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
    pub const fn binding(&self) -> &SessionResumeBinding {
        &self.binding
    }

    #[must_use]
    pub const fn interrupted_turn_id(&self) -> &RuntimeTurnId {
        &self.interrupted_turn_id
    }

    #[must_use]
    pub const fn provider_turn_ref(&self) -> Option<&TurnRef> {
        self.provider_turn_ref.as_ref()
    }

    #[must_use]
    pub const fn bounds(&self) -> ProviderSessionReconciliationBounds {
        self.bounds
    }

    #[must_use]
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline
    }

    #[must_use]
    pub const fn checkpoint(&self) -> Option<&ProviderOperationCheckpoint> {
        self.checkpoint.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSessionReconciliationPlan {
    preflight: PreflightPlan,
    agreement: ProviderSessionReconciliationAgreement,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedProviderSessionReconciliationEvidence {
    operation: PreparedOperationEvidence,
    plan: ProviderSessionReconciliationPlan,
}

impl PreparedProviderSessionReconciliationEvidence {
    pub fn from_plan(
        plan: ProviderSessionReconciliationPlan,
        access: PreparedAccessEvidence,
    ) -> Result<Self, PreparationFailure> {
        let operation = PreparedOperationEvidence::from_plan(plan.preflight().clone(), access)?;
        Ok(Self { operation, plan })
    }

    #[must_use]
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionReconciliationPlan {
        &self.plan
    }
}

impl ProviderSessionReconciliationPlan {
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
    pub const fn preflight(&self) -> &PreflightPlan {
        &self.preflight
    }

    #[must_use]
    pub const fn agreement(&self) -> &ProviderSessionReconciliationAgreement {
        &self.agreement
    }
}

#[derive(Clone, Debug)]
pub struct ProviderSessionReconciliationRequest {
    request_id: RequestId,
    agreement: ProviderSessionReconciliationAgreement,
    cancellation: Arc<ImmediateCancellation>,
}

impl ProviderSessionReconciliationRequest {
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
    pub const fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    #[must_use]
    pub const fn agreement(&self) -> &ProviderSessionReconciliationAgreement {
        &self.agreement
    }

    #[must_use]
    pub const fn cancellation(&self) -> &Arc<ImmediateCancellation> {
        &self.cancellation
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSessionReconciliationObservation {
    attribution: InterruptedTurnAttribution,
    state: InterruptedTurnState,
    provider_turn_ref: Option<TurnRef>,
    replay: Vec<SessionReplayItem>,
    replay_complete: bool,
}

impl ProviderSessionReconciliationObservation {
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSessionReconciliationOutcome {
    interrupted_turn_id: RuntimeTurnId,
    observation: ProviderSessionReconciliationObservation,
    cleanup: CleanupOutcome,
}

impl ProviderSessionReconciliationOutcome {
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
    pub const fn interrupted_turn_id(&self) -> &RuntimeTurnId {
        &self.interrupted_turn_id
    }

    #[must_use]
    pub const fn attribution(&self) -> InterruptedTurnAttribution {
        self.observation.attribution
    }

    #[must_use]
    pub const fn state(&self) -> InterruptedTurnState {
        self.observation.state
    }

    #[must_use]
    pub const fn provider_turn_ref(&self) -> Option<&TurnRef> {
        self.observation.provider_turn_ref.as_ref()
    }

    pub fn replay(&self) -> impl ExactSizeIterator<Item = &SessionReplayItem> {
        self.observation.replay.iter()
    }

    #[must_use]
    pub const fn replay_complete(&self) -> bool {
        self.observation.replay_complete
    }

    #[must_use]
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

pub fn validate_provider_session_reconciliation_request(
    plan: &ProviderSessionReconciliationPlan,
    request: &ProviderSessionReconciliationRequest,
) -> Result<(), RuntimeFailure> {
    if plan.agreement() == request.agreement() {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.provider_session_reconciliation.plan_mismatch",
            "Provider-session reconciliation request does not match its immutable plan",
        ))
    }
}

pub fn validate_provider_session_reconciliation_execution(
    plan: &ProviderSessionReconciliationPlan,
    request: &ProviderSessionReconciliationRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    validate_provider_session_reconciliation_request(plan, request)?;
    services.require_execution_host(plan.preflight().execution_host_id())?;
    let available = services.available_kinds();
    if plan
        .preflight()
        .requirements()
        .host_services()
        .any(|required| !available.contains(&required))
    {
        return Err(failure(
            "swallowtail.provider_session_reconciliation.service_unavailable",
            "Provider-session reconciliation host services are unavailable",
        ));
    }
    Ok(())
}

fn validate_plan(
    preflight: &PreflightPlan,
    agreement: &ProviderSessionReconciliationAgreement,
) -> Result<(), RuntimeFailure> {
    let requirements = preflight.requirements();
    if requirements.execution_layer() != ExecutionLayer::HarnessInteraction
        || requirements.driver_role() != DriverRole::ProviderSessionReconciliation
        || requirements.operation_shape() != OperationShape::ProviderSessionReconciliation
        || !agreement.binding().matches_plan(preflight)
        || requirements.session_access_policy()
            != Some(&swallowtail_core::SessionAccessPolicy::ambient_harness(
                swallowtail_core::ResourceAccess::Read,
            ))
        || !requirements
            .capabilities()
            .any(|required| required.capability() == Capability::ProviderSessionReconciliation)
        || !requirements
            .host_services()
            .any(|required| required == HostServiceKind::WorkingResource)
    {
        return Err(failure(
            "swallowtail.provider_session_reconciliation.plan_mismatch",
            "Provider-session reconciliation does not match its immutable binding",
        ));
    }
    let bounds = agreement.bounds();
    let capability = requirements
        .capabilities()
        .find(|required| required.capability() == Capability::ProviderSessionReconciliation)
        .expect("checked capability");
    let expected = [
        CapabilityConstraint::ReplayMaximumItems(bounds.maximum_replay_items().get()),
        CapabilityConstraint::ReplayMaximumBytes(bounds.maximum_replay_bytes().get()),
    ];
    if expected
        .iter()
        .any(|constraint| !capability.constraints().any(|actual| actual == constraint))
        || capability.constraints().count() != expected.len()
    {
        return Err(failure(
            "swallowtail.provider_session_reconciliation.bound_mismatch",
            "Provider-session reconciliation bounds differ from its capability plan",
        ));
    }
    if agreement.deadline().is_some()
        && !requirements
            .host_services()
            .any(|required| required == HostServiceKind::Time)
    {
        return Err(failure(
            "swallowtail.provider_session_reconciliation.time_service_required",
            "Deadline-bound provider-session reconciliation requires time service",
        ));
    }
    Ok(())
}

fn failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
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
