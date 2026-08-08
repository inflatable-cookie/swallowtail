#![deny(missing_docs)]

use crate::plan_family::{
    PlanRule, check_plan_rules, validate_agreement_matches_plan, validate_execution_services,
};
use crate::{
    CancellationControl, HostServices, PreparationFailure, PreparedOperationEvidence,
    ProviderSessionManagementBinding, RateLimitObservation, RuntimeFailure,
};
use swallowtail_core::{
    CancellationScope, DriverRole, HostServiceKind, OperationShape, PreflightPlan,
    ProviderRequestRef, ProviderSessionActivityEvidence, ProviderSessionAffectedScope,
    ProviderSessionCancellationPosture, ProviderSessionInitialStateRequirement,
    ProviderSessionManagementAction, ProviderSessionManagementEffect, SafeDiagnostic,
};

/// Immutable action agreement shared by a management plan and typed request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSessionManagementAgreement {
    binding: ProviderSessionManagementBinding,
    action: ProviderSessionManagementAction,
    initial_state: ProviderSessionInitialStateRequirement,
    affected_scope: ProviderSessionAffectedScope,
    activity: ProviderSessionActivityEvidence,
    cancellation: ProviderSessionCancellationPosture,
    deadline: Option<crate::Deadline>,
}

impl ProviderSessionManagementAgreement {
    /// Creates the immutable authorization shared by a plan and request.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        binding: ProviderSessionManagementBinding,
        action: ProviderSessionManagementAction,
        initial_state: ProviderSessionInitialStateRequirement,
        affected_scope: ProviderSessionAffectedScope,
        activity: ProviderSessionActivityEvidence,
        cancellation: ProviderSessionCancellationPosture,
        deadline: Option<crate::Deadline>,
    ) -> Self {
        Self {
            binding,
            action,
            initial_state,
            affected_scope,
            activity,
            cancellation,
            deadline,
        }
    }

    #[must_use]
    /// Returns the exact inactive-session management binding.
    pub const fn binding(&self) -> &ProviderSessionManagementBinding {
        &self.binding
    }

    #[must_use]
    /// Returns the typed lifecycle action.
    pub const fn action(&self) -> ProviderSessionManagementAction {
        self.action
    }

    #[must_use]
    /// Returns the provider lifecycle state required before dispatch.
    pub const fn initial_state(&self) -> ProviderSessionInitialStateRequirement {
        self.initial_state
    }

    #[must_use]
    /// Returns the target or provider-defined descendant scope.
    pub const fn affected_scope(&self) -> ProviderSessionAffectedScope {
        self.affected_scope
    }

    #[must_use]
    /// Returns the evidence that the target is inactive.
    pub const fn activity(&self) -> ProviderSessionActivityEvidence {
        self.activity
    }

    #[must_use]
    /// Returns the cancellation posture fixed before effects.
    pub const fn cancellation(&self) -> ProviderSessionCancellationPosture {
        self.cancellation
    }

    #[must_use]
    /// Returns the optional operation deadline.
    pub const fn deadline(&self) -> Option<crate::Deadline> {
        self.deadline
    }
}

/// Side-effect-free authorization plan for one inactive provider session.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSessionManagementPlan {
    preflight: PreflightPlan,
    agreement: ProviderSessionManagementAgreement,
}

impl ProviderSessionManagementPlan {
    /// Validates and creates a side-effect-free management plan.
    pub fn new(
        preflight: PreflightPlan,
        agreement: ProviderSessionManagementAgreement,
    ) -> Result<Self, RuntimeFailure> {
        validate_plan(&preflight, &agreement)?;
        Ok(Self {
            preflight,
            agreement,
        })
    }

    #[must_use]
    /// Returns the exact route preflight plan.
    pub const fn preflight(&self) -> &PreflightPlan {
        &self.preflight
    }

    #[must_use]
    /// Returns the immutable lifecycle agreement.
    pub const fn agreement(&self) -> &ProviderSessionManagementAgreement {
        &self.agreement
    }
}

use crate::plan_family::plan_family;

plan_family! {
    requests: {
        plan_type: ProviderSessionManagementPlan,
        agreement: ProviderSessionManagementAgreement,
        agreement_doc: "Returns the immutable management agreement.",
        scope: CancellationScope::ProviderSessionManagement,
        ns: "swallowtail.provider_session_management",
        requests: [
            ArchiveProviderSessionRequest = "Typed request to archive one inactive provider session." {
                new_doc: "Creates a typed request after validating action and cancellation scope.",
                new_arg: agreement: ProviderSessionManagementAgreement,
                agreement_expr: agreement.clone(),
                from_plan_doc: "Creates a typed request from a validated management plan.",
                from_plan_arg: plan_agreement,
                request_id_doc: "Returns the consumer-unique request identity.",
                extra: matches!(agreement.action(), ProviderSessionManagementAction::Archive),
                extra_code: "swallowtail.provider_session_management.request_action_mismatch",
                extra_message: "Archive request does not contain an archive agreement",
            }
            RestoreProviderSessionRequest = "Typed request to restore one inactive provider session." {
                new_doc: "Creates a typed request after validating action and cancellation scope.",
                new_arg: agreement: ProviderSessionManagementAgreement,
                agreement_expr: agreement.clone(),
                from_plan_doc: "Creates a typed request from a validated management plan.",
                from_plan_arg: plan_agreement,
                request_id_doc: "Returns the consumer-unique request identity.",
                extra: matches!(agreement.action(), ProviderSessionManagementAction::Restore),
                extra_code: "swallowtail.provider_session_management.request_action_mismatch",
                extra_message: "Restore request does not contain a restore agreement",
            }
            DeleteProviderSessionRequest = "Typed request to delete provider-session data at the agreed strength." {
                new_doc: "Creates a typed request after validating action and cancellation scope.",
                new_arg: agreement: ProviderSessionManagementAgreement,
                agreement_expr: agreement.clone(),
                from_plan_doc: "Creates a typed request from a validated management plan.",
                from_plan_arg: plan_agreement,
                request_id_doc: "Returns the consumer-unique request identity.",
                extra: matches!(agreement.action(), ProviderSessionManagementAction::Delete(_)),
                extra_code: "swallowtail.provider_session_management.request_action_mismatch",
                extra_message: "Delete request does not contain a delete agreement",
            }
        ]
    }
}

/// Exact provider effect truth plus safe request and rate evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSessionManagementOutcome {
    binding: ProviderSessionManagementBinding,
    effect: ProviderSessionManagementEffect,
    provider_request_ref: Option<ProviderRequestRef>,
    rate_limits: Vec<RateLimitObservation>,
    diagnostic: Option<SafeDiagnostic>,
}

impl ProviderSessionManagementOutcome {
    /// Creates an outcome from exact provider effect truth.
    #[must_use]
    pub const fn new(
        binding: ProviderSessionManagementBinding,
        effect: ProviderSessionManagementEffect,
    ) -> Self {
        Self {
            binding,
            effect,
            provider_request_ref: None,
            rate_limits: Vec::new(),
            diagnostic: None,
        }
    }

    #[must_use]
    /// Attaches the safe provider request reference, when observed.
    pub fn with_provider_request_ref(mut self, reference: ProviderRequestRef) -> Self {
        self.provider_request_ref = Some(reference);
        self
    }

    #[must_use]
    /// Attaches safe rate-limit observations.
    pub fn with_rate_limits(
        mut self,
        observations: impl IntoIterator<Item = RateLimitObservation>,
    ) -> Self {
        self.rate_limits = observations.into_iter().collect();
        self
    }

    #[must_use]
    /// Attaches a bounded diagnostic without changing effect truth.
    pub fn with_diagnostic(mut self, diagnostic: SafeDiagnostic) -> Self {
        self.diagnostic = Some(diagnostic);
        self
    }

    #[must_use]
    /// Returns the exact managed-session binding.
    pub const fn binding(&self) -> &ProviderSessionManagementBinding {
        &self.binding
    }

    #[must_use]
    /// Returns the provider effect reported by the adapter.
    pub const fn effect(&self) -> ProviderSessionManagementEffect {
        self.effect
    }

    #[must_use]
    /// Returns the safe provider request reference, when observed.
    pub const fn provider_request_ref(&self) -> Option<&ProviderRequestRef> {
        self.provider_request_ref.as_ref()
    }

    /// Iterates safe rate-limit observations.
    pub fn rate_limits(&self) -> impl ExactSizeIterator<Item = &RateLimitObservation> {
        self.rate_limits.iter()
    }

    #[must_use]
    /// Returns the optional bounded diagnostic.
    pub const fn diagnostic(&self) -> Option<&SafeDiagnostic> {
        self.diagnostic.as_ref()
    }
}

/// Shared evidence for adapter-local prepared lifecycle operations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedProviderSessionManagementEvidence {
    operation: PreparedOperationEvidence,
    plan: ProviderSessionManagementPlan,
}

impl PreparedProviderSessionManagementEvidence {
    /// Builds prepared evidence from a validated management plan.
    pub fn from_plan(plan: ProviderSessionManagementPlan) -> Result<Self, PreparationFailure> {
        let operation = PreparedOperationEvidence::from_plan(
            plan.preflight().clone(),
            plan.agreement().binding().access().clone(),
        )?;
        Ok(Self { operation, plan })
    }

    #[must_use]
    /// Returns the shared prepared-operation evidence.
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    #[must_use]
    /// Returns the validated management plan.
    pub const fn plan(&self) -> &ProviderSessionManagementPlan {
        &self.plan
    }
}

/// Verifies that a typed request matches its plan and available host services.
pub fn validate_provider_session_management_request(
    plan: &ProviderSessionManagementPlan,
    agreement: &ProviderSessionManagementAgreement,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    validate_agreement_matches_plan(
        agreement,
        plan.agreement(),
        "swallowtail.provider_session_management.plan_mismatch",
        "Provider-session request does not match its immutable management plan",
    )?;
    validate_execution_services(
        plan.preflight(),
        services,
        "swallowtail.provider_session_management.service_unavailable",
        "Provider-session management host services are unavailable",
    )
}

/// Ordered per-role validation rules for a provider-session management plan.
///
/// Management requires a scoped task service and action-specific
/// initial-state rules in addition to the shared role, shape, binding, and
/// capability checks.
const MANAGEMENT_PLAN_RULES: [PlanRule<ProviderSessionManagementAgreement>; 7] = [
    PlanRule::new(
        "swallowtail.provider_session_management.plan_mismatch",
        "Provider-session management binding does not match its immutable preflight plan",
        |preflight, _| {
            preflight.requirements().driver_role() == DriverRole::ProviderSessionManagement
        },
    ),
    PlanRule::new(
        "swallowtail.provider_session_management.plan_mismatch",
        "Provider-session management binding does not match its immutable preflight plan",
        |preflight, _| {
            preflight.requirements().operation_shape()
                == OperationShape::ProviderSessionManagement
        },
    ),
    PlanRule::new(
        "swallowtail.provider_session_management.plan_mismatch",
        "Provider-session management binding does not match its immutable preflight plan",
        |preflight, agreement| agreement.binding().matches_preflight_plan(preflight),
    ),
    PlanRule::new(
        "swallowtail.provider_session_management.capability_mismatch",
        "Provider-session plan does not authorize the requested action",
        |preflight, agreement| {
            let capability = agreement.action().required_capability();
            agreement.binding().supports(capability)
                && preflight
                    .requirements()
                    .capabilities()
                    .any(|required| required.capability() == capability)
        },
    ),
    PlanRule::new(
        "swallowtail.provider_session_management.initial_state_mismatch",
        "Provider-session initial state does not match the requested action",
        |_, agreement| initial_state_matches_action(agreement.action(), agreement.initial_state()),
    ),
    PlanRule::new(
        "swallowtail.provider_session_management.task_service_required",
        "Provider-session management requires scoped task service",
        |preflight, _| {
            preflight
                .requirements()
                .host_services()
                .any(|service| service == HostServiceKind::Task)
        },
    ),
    PlanRule::new(
        "swallowtail.provider_session_management.time_service_required",
        "Deadline-bound provider-session management requires time service",
        |preflight, agreement| {
            agreement.deadline().is_none()
                || preflight
                    .requirements()
                    .host_services()
                    .any(|service| service == HostServiceKind::Time)
        },
    ),
];

fn validate_plan(
    preflight: &PreflightPlan,
    agreement: &ProviderSessionManagementAgreement,
) -> Result<(), RuntimeFailure> {
    check_plan_rules(preflight, agreement, &MANAGEMENT_PLAN_RULES)
}

const fn initial_state_matches_action(
    action: ProviderSessionManagementAction,
    initial_state: ProviderSessionInitialStateRequirement,
) -> bool {
    matches!(
        (action, initial_state),
        (
            ProviderSessionManagementAction::Archive,
            ProviderSessionInitialStateRequirement::Unarchived
        ) | (
            ProviderSessionManagementAction::Restore,
            ProviderSessionInitialStateRequirement::Archived
        ) | (
            ProviderSessionManagementAction::Delete(_),
            ProviderSessionInitialStateRequirement::Unarchived
                | ProviderSessionInitialStateRequirement::Archived
                | ProviderSessionInitialStateRequirement::UnarchivedOrArchived
        )
    )
}


#[cfg(test)]
#[path = "provider_session_operation/tests.rs"]
mod tests;
