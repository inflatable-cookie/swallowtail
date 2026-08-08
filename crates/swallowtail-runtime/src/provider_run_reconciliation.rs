#![deny(missing_docs)]

use crate::plan_family::{
    PlanRule, check_plan_rules, failure as plan_failure, validate_agreement_matches_plan,
    validate_execution_services,
};
use crate::{
    CancellationControl, CleanupOutcome, Deadline, HostServices, OperationContent,
    ProviderRunCheckpoint, RuntimeFailure, RuntimeRunId, TokenUsage,
};
use std::num::NonZeroU64;
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, DriverRole, OperationShape, PreflightPlan,
    RunRef,
};

/// Read-only observed state of a structured run whose handle was lost.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InterruptedRunState {
    /// Provider evidence shows the run remains active.
    Active,
    /// Provider evidence shows the run is waiting for provider input.
    WaitingForProviderInput,
    /// Exact provider-run evidence shows successful completion.
    Completed,
    /// Exact provider-run evidence shows failure.
    Failed,
    /// Exact provider-run evidence shows cancellation.
    Cancelled,
    /// Provider work is inactive but terminal truth remains unresolved.
    InactiveUnresolved,
    /// Available provider evidence cannot classify the state safely.
    Unknown,
}

impl InterruptedRunState {
    /// Returns whether the observed state is terminal.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }
}

plan_family!(@prepared {
    plan_type: ProviderRunReconciliationPlan,
    prepared_type: PreparedProviderRunReconciliationEvidence,
    agreement: ProviderRunReconciliationAgreement,
    prepared_doc: "Prepared route and access evidence for run reconciliation.",
    agreement_doc: "Returns the immutable run reconciliation agreement.",
});
/// Exact run checkpoint, recovered-output bound, and deadline to reconcile.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderRunReconciliationAgreement {
    checkpoint: ProviderRunCheckpoint,
    maximum_output_bytes: NonZeroU64,
    deadline: Option<Deadline>,
}

impl ProviderRunReconciliationAgreement {
    /// Creates an immutable run reconciliation agreement.
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
    /// Returns the durable provider-run checkpoint.
    pub const fn checkpoint(&self) -> &ProviderRunCheckpoint {
        &self.checkpoint
    }

    #[must_use]
    /// Returns the interrupted consumer run.
    pub const fn interrupted_run_id(&self) -> &RuntimeRunId {
        self.checkpoint.runtime_run_id()
    }

    #[must_use]
    /// Returns the exact provider run.
    pub const fn provider_run_ref(&self) -> &RunRef {
        self.checkpoint.provider_run_ref()
    }

    #[must_use]
    /// Returns the maximum recovered output bytes.
    pub const fn maximum_output_bytes(&self) -> NonZeroU64 {
        self.maximum_output_bytes
    }

    #[must_use]
    /// Returns the observation deadline when present.
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline
    }
}

use crate::plan_family::plan_family;

plan_family! {
    plan: {
        plan_type: ProviderRunReconciliationPlan,
        prepared_type: PreparedProviderRunReconciliationEvidence,
        agreement: ProviderRunReconciliationAgreement,
        plan_doc: "Validated preflight plan and immutable run reconciliation agreement.",
        prepared_doc: "Prepared route and access evidence for run reconciliation.",
        agreement_doc: "Returns the immutable run reconciliation agreement.",
    }
    requests: {
        plan_type: ProviderRunReconciliationPlan,
        agreement: ProviderRunReconciliationAgreement,
        agreement_doc: "Returns the immutable run reconciliation agreement.",
        scope: CancellationScope::ProviderRunReconciliation,
        ns: "swallowtail.provider_run_reconciliation",
        requests: [
            ProviderRunReconciliationRequest = "One execution request derived from a run reconciliation plan." {
                new_doc: "Creates a request with an explicitly scoped cancellation control.",
                new_arg: plan: &ProviderRunReconciliationPlan,
                agreement_expr: plan.agreement().clone(),
                from_plan_doc: "Creates a request with a fresh correctly scoped cancellation control.",
                from_plan_arg: pass_plan,
                request_id_doc: "Returns the caller-assigned request identity.",
                extra: true,
                extra_code: "swallowtail.provider_run_reconciliation.cancellation_scope_mismatch",
                extra_message: "",
            }
        ]
    }
}

/// Adapter-produced exact-run observation before outcome validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderRunReconciliationObservation {
    state: InterruptedRunState,
    provider_run_ref: RunRef,
    output: Option<OperationContent>,
    usage: Option<TokenUsage>,
}

impl ProviderRunReconciliationObservation {
    /// Creates an observation and rejects terminal payload on non-terminal state.
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
    /// Returns the observed provider-run state.
    pub const fn state(&self) -> InterruptedRunState {
        self.state
    }

    #[must_use]
    /// Returns the exact provider run.
    pub const fn provider_run_ref(&self) -> &RunRef {
        &self.provider_run_ref
    }

    #[must_use]
    /// Returns bounded terminal output when supplied.
    pub const fn output(&self) -> Option<&OperationContent> {
        self.output.as_ref()
    }

    #[must_use]
    /// Returns terminal provider usage when supplied.
    pub const fn usage(&self) -> Option<&TokenUsage> {
        self.usage.as_ref()
    }
}

/// Validated exact-run reconciliation result with joined-cleanup truth.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderRunReconciliationOutcome {
    interrupted_run_id: RuntimeRunId,
    observation: ProviderRunReconciliationObservation,
    cleanup: CleanupOutcome,
}

impl ProviderRunReconciliationOutcome {
    /// Validates request correlation, provider identity, output bounds, and cleanup.
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
    /// Returns the interrupted consumer run.
    pub const fn interrupted_run_id(&self) -> &RuntimeRunId {
        &self.interrupted_run_id
    }

    #[must_use]
    /// Returns the validated exact-run observation.
    pub const fn observation(&self) -> &ProviderRunReconciliationObservation {
        &self.observation
    }

    #[must_use]
    /// Returns joined-cleanup truth for the observation operation.
    pub const fn cleanup(&self) -> &CleanupOutcome {
        &self.cleanup
    }
}

/// Ordered per-role validation rules for a run-reconciliation plan.
const RUN_RECONCILIATION_PLAN_RULES: [PlanRule<ProviderRunReconciliationAgreement>; 6] = [
    PlanRule::new(
        "swallowtail.provider_run_reconciliation.plan_mismatch",
        "Provider-run reconciliation does not match its immutable binding",
        |preflight, _| {
            preflight.requirements().driver_role() == DriverRole::ProviderRunReconciliation
        },
    ),
    PlanRule::new(
        "swallowtail.provider_run_reconciliation.plan_mismatch",
        "Provider-run reconciliation does not match its immutable binding",
        |preflight, _| {
            preflight.requirements().operation_shape() == OperationShape::ProviderRunReconciliation
        },
    ),
    PlanRule::new(
        "swallowtail.provider_run_reconciliation.plan_mismatch",
        "Provider-run reconciliation does not match its immutable binding",
        |preflight, agreement| agreement.checkpoint().matches_plan(preflight),
    ),
    PlanRule::new(
        "swallowtail.provider_run_reconciliation.plan_mismatch",
        "Provider-run reconciliation capability is missing",
        |preflight, _| {
            preflight
                .requirements()
                .capabilities()
                .any(|required| required.capability() == Capability::ProviderRunReconciliation)
        },
    ),
    PlanRule::new(
        "swallowtail.provider_run_reconciliation.bound_mismatch",
        "Provider-run reconciliation output bound differs from its capability plan",
        |preflight, agreement| {
            let Some(capability) = preflight
                .requirements()
                .capabilities()
                .find(|required| required.capability() == Capability::ProviderRunReconciliation)
            else {
                return false;
            };
            let expected = CapabilityConstraint::RecoveredOutputMaximumBytes(
                agreement.maximum_output_bytes().get(),
            );
            capability.constraints().count() == 1
                && capability.constraints().any(|actual| actual == &expected)
        },
    ),
    PlanRule::new(
        "swallowtail.provider_run_reconciliation.time_service_required",
        "Deadline-bound provider-run reconciliation requires time service",
        |preflight, agreement| {
            agreement.deadline().is_none()
                || preflight
                    .requirements()
                    .host_services()
                    .any(|required| required == swallowtail_core::HostServiceKind::Time)
        },
    ),
];

/// Verifies that execution input still matches its immutable plan.
pub fn validate_provider_run_reconciliation_request(
    plan: &ProviderRunReconciliationPlan,
    request: &ProviderRunReconciliationRequest,
) -> Result<(), RuntimeFailure> {
    validate_agreement_matches_plan(
        plan.agreement(),
        request.agreement(),
        "swallowtail.provider_run_reconciliation.plan_mismatch",
        "Provider-run reconciliation request does not match its immutable plan",
    )
}

/// Verifies request, execution host, and required host-service availability.
pub fn validate_provider_run_reconciliation_execution(
    plan: &ProviderRunReconciliationPlan,
    request: &ProviderRunReconciliationRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    validate_provider_run_reconciliation_request(plan, request)?;
    validate_execution_services(
        plan.preflight(),
        services,
        "swallowtail.provider_run_reconciliation.service_unavailable",
        "Provider-run reconciliation host services are unavailable",
    )
}

fn validate_plan(
    preflight: &PreflightPlan,
    agreement: &ProviderRunReconciliationAgreement,
) -> Result<(), RuntimeFailure> {
    check_plan_rules(preflight, agreement, &RUN_RECONCILIATION_PLAN_RULES)
}

fn failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    plan_failure(code, message)
}
