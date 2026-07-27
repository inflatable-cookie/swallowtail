use crate::{
    CancellationControl, HostServices, ImmediateCancellation, PreparationFailure,
    PreparedOperationEvidence, ProviderSessionManagementBinding, RateLimitObservation, RequestId,
    RuntimeFailure,
};
use std::sync::Arc;
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
    pub const fn binding(&self) -> &ProviderSessionManagementBinding {
        &self.binding
    }

    #[must_use]
    pub const fn action(&self) -> ProviderSessionManagementAction {
        self.action
    }

    #[must_use]
    pub const fn initial_state(&self) -> ProviderSessionInitialStateRequirement {
        self.initial_state
    }

    #[must_use]
    pub const fn affected_scope(&self) -> ProviderSessionAffectedScope {
        self.affected_scope
    }

    #[must_use]
    pub const fn activity(&self) -> ProviderSessionActivityEvidence {
        self.activity
    }

    #[must_use]
    pub const fn cancellation(&self) -> ProviderSessionCancellationPosture {
        self.cancellation
    }

    #[must_use]
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
    pub const fn preflight(&self) -> &PreflightPlan {
        &self.preflight
    }

    #[must_use]
    pub const fn agreement(&self) -> &ProviderSessionManagementAgreement {
        &self.agreement
    }
}

macro_rules! typed_request {
    ($name:ident, $matches:pat, $message:literal) => {
        #[derive(Clone, Debug)]
        pub struct $name {
            request_id: RequestId,
            agreement: ProviderSessionManagementAgreement,
            cancellation: Arc<ImmediateCancellation>,
        }

        impl $name {
            pub fn new(
                request_id: RequestId,
                agreement: ProviderSessionManagementAgreement,
                cancellation: Arc<ImmediateCancellation>,
            ) -> Result<Self, RuntimeFailure> {
                if !matches!(agreement.action(), $matches) {
                    return Err(failure(
                        "swallowtail.provider_session_management.request_action_mismatch",
                        $message,
                    ));
                }
                if cancellation.scope() != CancellationScope::ProviderSessionManagement {
                    return Err(failure(
                        "swallowtail.provider_session_management.cancellation_scope_mismatch",
                        "Provider-session request has the wrong cancellation scope",
                    ));
                }
                Ok(Self {
                    request_id,
                    agreement,
                    cancellation,
                })
            }

            pub fn from_plan(
                request_id: RequestId,
                plan: &ProviderSessionManagementPlan,
            ) -> Result<Self, RuntimeFailure> {
                Self::new(
                    request_id,
                    plan.agreement().clone(),
                    Arc::new(ImmediateCancellation::new(
                        CancellationScope::ProviderSessionManagement,
                    )),
                )
            }

            #[must_use]
            pub const fn request_id(&self) -> &RequestId {
                &self.request_id
            }

            #[must_use]
            pub const fn agreement(&self) -> &ProviderSessionManagementAgreement {
                &self.agreement
            }

            #[must_use]
            pub const fn cancellation(&self) -> &Arc<ImmediateCancellation> {
                &self.cancellation
            }
        }
    };
}

typed_request!(
    ArchiveProviderSessionRequest,
    ProviderSessionManagementAction::Archive,
    "Archive request does not contain an archive agreement"
);
typed_request!(
    RestoreProviderSessionRequest,
    ProviderSessionManagementAction::Restore,
    "Restore request does not contain a restore agreement"
);
typed_request!(
    DeleteProviderSessionRequest,
    ProviderSessionManagementAction::Delete(_),
    "Delete request does not contain a delete agreement"
);

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
    pub fn with_provider_request_ref(mut self, reference: ProviderRequestRef) -> Self {
        self.provider_request_ref = Some(reference);
        self
    }

    #[must_use]
    pub fn with_rate_limits(
        mut self,
        observations: impl IntoIterator<Item = RateLimitObservation>,
    ) -> Self {
        self.rate_limits = observations.into_iter().collect();
        self
    }

    #[must_use]
    pub fn with_diagnostic(mut self, diagnostic: SafeDiagnostic) -> Self {
        self.diagnostic = Some(diagnostic);
        self
    }

    #[must_use]
    pub const fn binding(&self) -> &ProviderSessionManagementBinding {
        &self.binding
    }

    #[must_use]
    pub const fn effect(&self) -> ProviderSessionManagementEffect {
        self.effect
    }

    #[must_use]
    pub const fn provider_request_ref(&self) -> Option<&ProviderRequestRef> {
        self.provider_request_ref.as_ref()
    }

    pub fn rate_limits(&self) -> impl ExactSizeIterator<Item = &RateLimitObservation> {
        self.rate_limits.iter()
    }

    #[must_use]
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
    pub fn from_plan(plan: ProviderSessionManagementPlan) -> Result<Self, PreparationFailure> {
        let operation = PreparedOperationEvidence::from_plan(
            plan.preflight().clone(),
            plan.agreement().binding().access().clone(),
        )?;
        Ok(Self { operation, plan })
    }

    #[must_use]
    pub const fn operation(&self) -> &PreparedOperationEvidence {
        &self.operation
    }

    #[must_use]
    pub const fn plan(&self) -> &ProviderSessionManagementPlan {
        &self.plan
    }
}

pub fn validate_provider_session_management_request(
    plan: &ProviderSessionManagementPlan,
    agreement: &ProviderSessionManagementAgreement,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    if plan.agreement() != agreement {
        return Err(failure(
            "swallowtail.provider_session_management.plan_mismatch",
            "Provider-session request does not match its immutable management plan",
        ));
    }
    services.require_execution_host(plan.preflight().execution_host_id())?;
    let available = services.available_kinds();
    if plan
        .preflight()
        .requirements()
        .host_services()
        .any(|required| !available.contains(&required))
    {
        return Err(failure(
            "swallowtail.provider_session_management.service_unavailable",
            "Provider-session management host services are unavailable",
        ));
    }
    Ok(())
}

fn validate_plan(
    preflight: &PreflightPlan,
    agreement: &ProviderSessionManagementAgreement,
) -> Result<(), RuntimeFailure> {
    if preflight.requirements().driver_role() != DriverRole::ProviderSessionManagement
        || preflight.requirements().operation_shape() != OperationShape::ProviderSessionManagement
    {
        return Err(plan_mismatch());
    }
    if !agreement.binding().matches_preflight_plan(preflight) {
        return Err(plan_mismatch());
    }

    let capability = agreement.action().required_capability();
    if !agreement.binding().supports(capability)
        || !preflight
            .requirements()
            .capabilities()
            .any(|required| required.capability() == capability)
    {
        return Err(failure(
            "swallowtail.provider_session_management.capability_mismatch",
            "Provider-session plan does not authorize the requested action",
        ));
    }
    if !initial_state_matches_action(agreement.action(), agreement.initial_state()) {
        return Err(failure(
            "swallowtail.provider_session_management.initial_state_mismatch",
            "Provider-session initial state does not match the requested action",
        ));
    }
    if !preflight
        .requirements()
        .host_services()
        .any(|service| service == HostServiceKind::Task)
    {
        return Err(failure(
            "swallowtail.provider_session_management.task_service_required",
            "Provider-session management requires scoped task service",
        ));
    }
    if agreement.deadline().is_some()
        && !preflight
            .requirements()
            .host_services()
            .any(|service| service == HostServiceKind::Time)
    {
        return Err(failure(
            "swallowtail.provider_session_management.time_service_required",
            "Deadline-bound provider-session management requires time service",
        ));
    }
    Ok(())
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

fn plan_mismatch() -> RuntimeFailure {
    failure(
        "swallowtail.provider_session_management.plan_mismatch",
        "Provider-session management binding does not match its immutable preflight plan",
    )
}

fn failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

#[cfg(test)]
#[path = "provider_session_operation/tests.rs"]
mod tests;
