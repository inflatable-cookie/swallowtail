use crate::{
    CancellationControl, CleanupOutcome, Deadline, HostServices, ImmediateCancellation,
    OperationContent, PreparationFailure, PreparedAccessEvidence, PreparedOperationEvidence,
    ProviderRunCheckpoint, RequestId, RuntimeFailure, RuntimeRunId, TokenUsage,
};
use std::num::NonZeroU64;
use std::sync::Arc;
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, DriverRole, OperationShape, PreflightPlan,
    RunRef, SafeDiagnostic,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InterruptedRunState {
    Active,
    Completed,
    Failed,
    Cancelled,
    InactiveUnresolved,
    Unknown,
}

impl InterruptedRunState {
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderRunReconciliationAgreement {
    checkpoint: ProviderRunCheckpoint,
    maximum_output_bytes: NonZeroU64,
    deadline: Option<Deadline>,
}

impl ProviderRunReconciliationAgreement {
    #[must_use]
    pub const fn new(
        checkpoint: ProviderRunCheckpoint,
        maximum_output_bytes: NonZeroU64,
        deadline: Option<Deadline>,
    ) -> Self {
        Self {
            checkpoint,
            maximum_output_bytes,
            deadline,
        }
    }

    #[must_use]
    pub const fn checkpoint(&self) -> &ProviderRunCheckpoint {
        &self.checkpoint
    }

    #[must_use]
    pub const fn interrupted_run_id(&self) -> &RuntimeRunId {
        self.checkpoint.runtime_run_id()
    }

    #[must_use]
    pub const fn provider_run_ref(&self) -> &RunRef {
        self.checkpoint.provider_run_ref()
    }

    #[must_use]
    pub const fn maximum_output_bytes(&self) -> NonZeroU64 {
        self.maximum_output_bytes
    }

    #[must_use]
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderRunReconciliationPlan {
    preflight: PreflightPlan,
    agreement: ProviderRunReconciliationAgreement,
}

impl ProviderRunReconciliationPlan {
    pub fn new(
        preflight: PreflightPlan,
        agreement: ProviderRunReconciliationAgreement,
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
    pub const fn agreement(&self) -> &ProviderRunReconciliationAgreement {
        &self.agreement
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedProviderRunReconciliationEvidence {
    operation: PreparedOperationEvidence,
    plan: ProviderRunReconciliationPlan,
}

impl PreparedProviderRunReconciliationEvidence {
    pub fn from_plan(
        plan: ProviderRunReconciliationPlan,
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
    pub const fn plan(&self) -> &ProviderRunReconciliationPlan {
        &self.plan
    }
}

#[derive(Clone, Debug)]
pub struct ProviderRunReconciliationRequest {
    request_id: RequestId,
    agreement: ProviderRunReconciliationAgreement,
    cancellation: Arc<ImmediateCancellation>,
}

impl ProviderRunReconciliationRequest {
    pub fn new(
        request_id: RequestId,
        plan: &ProviderRunReconciliationPlan,
        cancellation: Arc<ImmediateCancellation>,
    ) -> Result<Self, RuntimeFailure> {
        if cancellation.scope() != CancellationScope::ProviderRunReconciliation {
            return Err(failure(
                "swallowtail.provider_run_reconciliation.cancellation_scope_mismatch",
                "Provider-run reconciliation request has the wrong cancellation scope",
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
        plan: &ProviderRunReconciliationPlan,
    ) -> Result<Self, RuntimeFailure> {
        Self::new(
            request_id,
            plan,
            Arc::new(ImmediateCancellation::new(
                CancellationScope::ProviderRunReconciliation,
            )),
        )
    }

    #[must_use]
    pub const fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    #[must_use]
    pub const fn agreement(&self) -> &ProviderRunReconciliationAgreement {
        &self.agreement
    }

    #[must_use]
    pub const fn cancellation(&self) -> &Arc<ImmediateCancellation> {
        &self.cancellation
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderRunReconciliationObservation {
    state: InterruptedRunState,
    provider_run_ref: RunRef,
    output: Option<OperationContent>,
    usage: Option<TokenUsage>,
}

impl ProviderRunReconciliationObservation {
    pub fn new(
        state: InterruptedRunState,
        provider_run_ref: RunRef,
        output: Option<OperationContent>,
        usage: Option<TokenUsage>,
    ) -> Result<Self, RuntimeFailure> {
        if !state.is_terminal() && (output.is_some() || usage.is_some()) {
            return Err(failure(
                "swallowtail.provider_run_reconciliation.nonterminal_payload",
                "Non-terminal provider-run reconciliation cannot carry terminal payload",
            ));
        }
        Ok(Self {
            state,
            provider_run_ref,
            output,
            usage,
        })
    }

    #[must_use]
    pub const fn state(&self) -> InterruptedRunState {
        self.state
    }

    #[must_use]
    pub const fn provider_run_ref(&self) -> &RunRef {
        &self.provider_run_ref
    }

    #[must_use]
    pub const fn output(&self) -> Option<&OperationContent> {
        self.output.as_ref()
    }

    #[must_use]
    pub const fn usage(&self) -> Option<&TokenUsage> {
        self.usage.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderRunReconciliationOutcome {
    interrupted_run_id: RuntimeRunId,
    observation: ProviderRunReconciliationObservation,
    cleanup: CleanupOutcome,
}

impl ProviderRunReconciliationOutcome {
    pub fn new(
        plan: &ProviderRunReconciliationPlan,
        request: &ProviderRunReconciliationRequest,
        observation: ProviderRunReconciliationObservation,
        cleanup: CleanupOutcome,
    ) -> Result<Self, RuntimeFailure> {
        validate_provider_run_reconciliation_request(plan, request)?;
        if !matches!(
            cleanup,
            CleanupOutcome::Clean | CleanupOutcome::NotApplicable
        ) {
            return Err(failure(
                "swallowtail.provider_run_reconciliation.cleanup_incomplete",
                "Provider-run reconciliation cleanup did not complete",
            ));
        }
        let agreement = plan.agreement();
        if observation.provider_run_ref() != agreement.provider_run_ref() {
            return Err(failure(
                "swallowtail.provider_run_reconciliation.attribution_mismatch",
                "Provider-run reconciliation observed a different provider run",
            ));
        }
        if observation.output().is_some_and(|output| {
            u64::try_from(output.byte_len()).unwrap_or(u64::MAX)
                > agreement.maximum_output_bytes().get()
        }) {
            return Err(failure(
                "swallowtail.provider_run_reconciliation.output_oversized",
                "Provider-run reconciliation output exceeds its bound",
            ));
        }
        Ok(Self {
            interrupted_run_id: agreement.interrupted_run_id().clone(),
            observation,
            cleanup,
        })
    }

    #[must_use]
    pub const fn interrupted_run_id(&self) -> &RuntimeRunId {
        &self.interrupted_run_id
    }

    #[must_use]
    pub const fn observation(&self) -> &ProviderRunReconciliationObservation {
        &self.observation
    }

    #[must_use]
    pub const fn cleanup(&self) -> &CleanupOutcome {
        &self.cleanup
    }
}

pub fn validate_provider_run_reconciliation_request(
    plan: &ProviderRunReconciliationPlan,
    request: &ProviderRunReconciliationRequest,
) -> Result<(), RuntimeFailure> {
    if plan.agreement() == request.agreement() {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.provider_run_reconciliation.plan_mismatch",
            "Provider-run reconciliation request does not match its immutable plan",
        ))
    }
}

pub fn validate_provider_run_reconciliation_execution(
    plan: &ProviderRunReconciliationPlan,
    request: &ProviderRunReconciliationRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    validate_provider_run_reconciliation_request(plan, request)?;
    services.require_execution_host(plan.preflight().execution_host_id())?;
    let available = services.available_kinds();
    if plan
        .preflight()
        .requirements()
        .host_services()
        .any(|required| !available.contains(&required))
    {
        return Err(failure(
            "swallowtail.provider_run_reconciliation.service_unavailable",
            "Provider-run reconciliation host services are unavailable",
        ));
    }
    Ok(())
}

fn validate_plan(
    preflight: &PreflightPlan,
    agreement: &ProviderRunReconciliationAgreement,
) -> Result<(), RuntimeFailure> {
    let requirements = preflight.requirements();
    if requirements.driver_role() != DriverRole::ProviderRunReconciliation
        || requirements.operation_shape() != OperationShape::ProviderRunReconciliation
        || !agreement.checkpoint().matches_plan(preflight)
    {
        return Err(failure(
            "swallowtail.provider_run_reconciliation.plan_mismatch",
            "Provider-run reconciliation does not match its immutable binding",
        ));
    }
    let capability = requirements
        .capabilities()
        .find(|required| required.capability() == Capability::ProviderRunReconciliation)
        .ok_or_else(|| {
            failure(
                "swallowtail.provider_run_reconciliation.plan_mismatch",
                "Provider-run reconciliation capability is missing",
            )
        })?;
    let expected =
        CapabilityConstraint::RecoveredOutputMaximumBytes(agreement.maximum_output_bytes().get());
    if capability.constraints().count() != 1
        || !capability.constraints().any(|actual| actual == &expected)
    {
        return Err(failure(
            "swallowtail.provider_run_reconciliation.bound_mismatch",
            "Provider-run reconciliation output bound differs from its capability plan",
        ));
    }
    if agreement.deadline().is_some()
        && !requirements
            .host_services()
            .any(|required| required == swallowtail_core::HostServiceKind::Time)
    {
        return Err(failure(
            "swallowtail.provider_run_reconciliation.time_service_required",
            "Deadline-bound provider-run reconciliation requires time service",
        ));
    }
    Ok(())
}

fn failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}
