#![deny(missing_docs)]

use crate::plan_family::{
    PlanRule, check_plan_rules, validate_agreement_matches_plan, validate_execution_services,
};
use crate::{
    CancellationControl, Deadline, HostServices, ProviderRecoveredResourceCleanupBinding,
    RuntimeFailure,
};
use std::collections::BTreeSet;
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, DriverRole, OperationShape,
    OwnedRemoteResourceKind, PreflightPlan, ProviderRecoveredResourceCleanupEffect, SafeDiagnostic,
};

/// Immutable authorization to clean resources left by one recovered run.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderRecoveredResourceCleanupAgreement {
    binding: ProviderRecoveredResourceCleanupBinding,
    deadline: Option<Deadline>,
}

impl ProviderRecoveredResourceCleanupAgreement {
    /// Creates cleanup authorization for an exact binding and deadline.
    #[must_use]
    pub const fn new(
        binding: ProviderRecoveredResourceCleanupBinding,
        deadline: Option<Deadline>,
    ) -> Self {
        Self { binding, deadline }
    }

    #[must_use]
    /// Returns the exact driver-owned resource binding.
    pub const fn binding(&self) -> &ProviderRecoveredResourceCleanupBinding {
        &self.binding
    }

    #[must_use]
    /// Returns the optional cleanup deadline.
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline
    }
}

plan_family!(@prepared {
    plan_type: ProviderRecoveredResourceCleanupPlan,
    prepared_type: PreparedProviderRecoveredResourceCleanupEvidence,
    agreement: ProviderRecoveredResourceCleanupAgreement,
    prepared_doc: "Prepared route and plan evidence for recovered-resource cleanup.",
    agreement_doc: "Returns the immutable cleanup agreement.",
});
use crate::plan_family::plan_family;

plan_family! {
    plan: {
        plan_type: ProviderRecoveredResourceCleanupPlan,
        prepared_type: PreparedProviderRecoveredResourceCleanupEvidence,
        agreement: ProviderRecoveredResourceCleanupAgreement,
        plan_doc: "Side-effect-free plan for cleaning resources left by one recovered run.",
        prepared_doc: "Prepared route and plan evidence for recovered-resource cleanup.",
        agreement_doc: "Returns the immutable cleanup agreement.",
    }
    requests: {
        plan_type: ProviderRecoveredResourceCleanupPlan,
        agreement: ProviderRecoveredResourceCleanupAgreement,
        agreement_doc: "Returns the immutable cleanup agreement.",
        scope: CancellationScope::ProviderRecoveredResourceCleanup,
        ns: "swallowtail.provider_recovered_resource_cleanup",
        requests: [
            ProviderRecoveredResourceCleanupRequest = "Typed request to clean resources left by one recovered provider run." {
                new_doc: "Creates a request after validating its cancellation scope.",
                new_arg: plan: &ProviderRecoveredResourceCleanupPlan,
                agreement_expr: plan.agreement().clone(),
                from_plan_doc: "Creates a request with a new cleanup-scoped cancellation control.",
                from_plan_arg: pass_plan,
                request_id_doc: "Returns the caller-assigned request identity.",
                extra: true,
                extra_code: "swallowtail.provider_recovered_resource_cleanup.cancellation_scope_mismatch",
                extra_message: "",
            }
        ]
    }
}

/// Exact effect truth for one recovered-resource cleanup attempt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderRecoveredResourceCleanupOutcome {
    binding: ProviderRecoveredResourceCleanupBinding,
    effect: ProviderRecoveredResourceCleanupEffect,
    diagnostic: Option<SafeDiagnostic>,
}

impl ProviderRecoveredResourceCleanupOutcome {
    /// Creates an outcome after verifying that the request matches its plan.
    pub fn new(
        plan: &ProviderRecoveredResourceCleanupPlan,
        request: &ProviderRecoveredResourceCleanupRequest,
        effect: ProviderRecoveredResourceCleanupEffect,
    ) -> Result<Self, RuntimeFailure> {
        validate_provider_recovered_resource_cleanup_request(plan, request)?;
        Ok(Self {
            binding: plan.agreement().binding().clone(),
            effect,
            diagnostic: None,
        })
    }

    #[must_use]
    /// Attaches a bounded diagnostic without changing effect truth.
    pub fn with_diagnostic(mut self, diagnostic: SafeDiagnostic) -> Self {
        self.diagnostic = Some(diagnostic);
        self
    }

    #[must_use]
    /// Returns the exact cleanup binding.
    pub const fn binding(&self) -> &ProviderRecoveredResourceCleanupBinding {
        &self.binding
    }

    #[must_use]
    /// Returns the provider cleanup effect reported by the adapter.
    pub const fn effect(&self) -> ProviderRecoveredResourceCleanupEffect {
        self.effect
    }

    #[must_use]
    /// Returns the optional bounded diagnostic.
    pub const fn diagnostic(&self) -> Option<&SafeDiagnostic> {
        self.diagnostic.as_ref()
    }
}

/// Verifies that a cleanup request matches its immutable plan.
pub fn validate_provider_recovered_resource_cleanup_request(
    plan: &ProviderRecoveredResourceCleanupPlan,
    request: &ProviderRecoveredResourceCleanupRequest,
) -> Result<(), RuntimeFailure> {
    validate_agreement_matches_plan(
        plan.agreement(),
        request.agreement(),
        "swallowtail.provider_recovered_resource_cleanup.plan_mismatch",
        "Recovered-resource cleanup request does not match its immutable plan",
    )
}

/// Verifies a cleanup request and the host services needed to execute it.
pub fn validate_provider_recovered_resource_cleanup_execution(
    plan: &ProviderRecoveredResourceCleanupPlan,
    request: &ProviderRecoveredResourceCleanupRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    validate_provider_recovered_resource_cleanup_request(plan, request)?;
    validate_execution_services(
        plan.preflight(),
        services,
        "swallowtail.provider_recovered_resource_cleanup.service_unavailable",
        "Recovered-resource cleanup host services are unavailable",
    )
}

/// Ordered per-role validation rules for a recovered-resource cleanup plan.
const CLEANUP_PLAN_RULES: [PlanRule<ProviderRecoveredResourceCleanupAgreement>; 6] = [
    PlanRule::new(
        "swallowtail.provider_recovered_resource_cleanup.plan_mismatch",
        "Recovered-resource cleanup does not match its immutable binding",
        |preflight, _| {
            preflight.requirements().driver_role()
                == DriverRole::ProviderRecoveredResourceCleanup
        },
    ),
    PlanRule::new(
        "swallowtail.provider_recovered_resource_cleanup.plan_mismatch",
        "Recovered-resource cleanup does not match its immutable binding",
        |preflight, _| {
            preflight.requirements().operation_shape()
                == OperationShape::ProviderRecoveredResourceCleanup
        },
    ),
    PlanRule::new(
        "swallowtail.provider_recovered_resource_cleanup.plan_mismatch",
        "Recovered-resource cleanup does not match its immutable binding",
        |preflight, agreement| agreement.binding().matches_plan(preflight),
    ),
    PlanRule::new(
        "swallowtail.provider_recovered_resource_cleanup.plan_mismatch",
        "Recovered-resource cleanup does not match its immutable binding",
        |preflight, _| {
            preflight
                .requirements()
                .capabilities()
                .any(|required| {
                    required.capability() == Capability::ProviderRecoveredResourceCleanup
                })
        },
    ),
    PlanRule::new(
        "swallowtail.provider_recovered_resource_cleanup.resource_scope_mismatch",
        "Recovered-resource cleanup scope differs from its capability plan",
        |preflight, agreement| {
            let Some(capability) = preflight
                .requirements()
                .capabilities()
                .find(|required| {
                    required.capability() == Capability::ProviderRecoveredResourceCleanup
                })
            else {
                return false;
            };
            let declared = capability
                .constraints()
                .map(|constraint| match constraint {
                    CapabilityConstraint::OwnedRemoteResource(kind) => Ok(*kind),
                    _ => Err(()),
                })
                .collect::<Result<BTreeSet<OwnedRemoteResourceKind>, ()>>();
            let Ok(declared) = declared else {
                return false;
            };
            let bound = agreement
                .binding()
                .resource_kinds()
                .collect::<BTreeSet<_>>();
            declared == bound && capability.constraints().count() == bound.len()
        },
    ),
    PlanRule::new(
        "swallowtail.provider_recovered_resource_cleanup.time_service_required",
        "Deadline-bound recovered-resource cleanup requires time service",
        |preflight, agreement| {
            agreement.deadline().is_none()
                || preflight.requirements().host_services().any(|required| {
                    required == swallowtail_core::HostServiceKind::Time
                })
        },
    ),
];

fn validate_plan(
    preflight: &PreflightPlan,
    agreement: &ProviderRecoveredResourceCleanupAgreement,
) -> Result<(), RuntimeFailure> {
    check_plan_rules(preflight, agreement, &CLEANUP_PLAN_RULES)
}

#[cfg(test)]
mod tests;
