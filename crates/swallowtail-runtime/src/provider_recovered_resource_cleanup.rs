use crate::{
    CancellationControl, Deadline, HostServices, ImmediateCancellation, PreparationFailure,
    PreparedAccessEvidence, PreparedOperationEvidence, ProviderRecoveredResourceCleanupBinding,
    RequestId, RuntimeFailure,
};
use std::collections::BTreeSet;
use std::sync::Arc;
use swallowtail_core::{
    CancellationScope, Capability, CapabilityConstraint, DriverRole, OperationShape,
    OwnedRemoteResourceKind, PreflightPlan, ProviderRecoveredResourceCleanupEffect, SafeDiagnostic,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderRecoveredResourceCleanupAgreement {
    binding: ProviderRecoveredResourceCleanupBinding,
    deadline: Option<Deadline>,
}

impl ProviderRecoveredResourceCleanupAgreement {
    #[must_use]
    pub const fn new(
        binding: ProviderRecoveredResourceCleanupBinding,
        deadline: Option<Deadline>,
    ) -> Self {
        Self { binding, deadline }
    }

    #[must_use]
    pub const fn binding(&self) -> &ProviderRecoveredResourceCleanupBinding {
        &self.binding
    }

    #[must_use]
    pub const fn deadline(&self) -> Option<Deadline> {
        self.deadline
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderRecoveredResourceCleanupPlan {
    preflight: PreflightPlan,
    agreement: ProviderRecoveredResourceCleanupAgreement,
}

impl ProviderRecoveredResourceCleanupPlan {
    pub fn new(
        preflight: PreflightPlan,
        agreement: ProviderRecoveredResourceCleanupAgreement,
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
    pub const fn agreement(&self) -> &ProviderRecoveredResourceCleanupAgreement {
        &self.agreement
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreparedProviderRecoveredResourceCleanupEvidence {
    operation: PreparedOperationEvidence,
    plan: ProviderRecoveredResourceCleanupPlan,
}

impl PreparedProviderRecoveredResourceCleanupEvidence {
    pub fn from_plan(
        plan: ProviderRecoveredResourceCleanupPlan,
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
    pub const fn plan(&self) -> &ProviderRecoveredResourceCleanupPlan {
        &self.plan
    }
}

#[derive(Clone, Debug)]
pub struct ProviderRecoveredResourceCleanupRequest {
    request_id: RequestId,
    agreement: ProviderRecoveredResourceCleanupAgreement,
    cancellation: Arc<ImmediateCancellation>,
}

impl ProviderRecoveredResourceCleanupRequest {
    pub fn new(
        request_id: RequestId,
        plan: &ProviderRecoveredResourceCleanupPlan,
        cancellation: Arc<ImmediateCancellation>,
    ) -> Result<Self, RuntimeFailure> {
        if cancellation.scope() != CancellationScope::ProviderRecoveredResourceCleanup {
            return Err(failure(
                "swallowtail.provider_recovered_resource_cleanup.cancellation_scope_mismatch",
                "Recovered-resource cleanup request has the wrong cancellation scope",
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
        plan: &ProviderRecoveredResourceCleanupPlan,
    ) -> Result<Self, RuntimeFailure> {
        Self::new(
            request_id,
            plan,
            Arc::new(ImmediateCancellation::new(
                CancellationScope::ProviderRecoveredResourceCleanup,
            )),
        )
    }

    #[must_use]
    pub const fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    #[must_use]
    pub const fn agreement(&self) -> &ProviderRecoveredResourceCleanupAgreement {
        &self.agreement
    }

    #[must_use]
    pub const fn cancellation(&self) -> &Arc<ImmediateCancellation> {
        &self.cancellation
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderRecoveredResourceCleanupOutcome {
    binding: ProviderRecoveredResourceCleanupBinding,
    effect: ProviderRecoveredResourceCleanupEffect,
    diagnostic: Option<SafeDiagnostic>,
}

impl ProviderRecoveredResourceCleanupOutcome {
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
    pub fn with_diagnostic(mut self, diagnostic: SafeDiagnostic) -> Self {
        self.diagnostic = Some(diagnostic);
        self
    }

    #[must_use]
    pub const fn binding(&self) -> &ProviderRecoveredResourceCleanupBinding {
        &self.binding
    }

    #[must_use]
    pub const fn effect(&self) -> ProviderRecoveredResourceCleanupEffect {
        self.effect
    }

    #[must_use]
    pub const fn diagnostic(&self) -> Option<&SafeDiagnostic> {
        self.diagnostic.as_ref()
    }
}

pub fn validate_provider_recovered_resource_cleanup_request(
    plan: &ProviderRecoveredResourceCleanupPlan,
    request: &ProviderRecoveredResourceCleanupRequest,
) -> Result<(), RuntimeFailure> {
    if plan.agreement() == request.agreement() {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.provider_recovered_resource_cleanup.plan_mismatch",
            "Recovered-resource cleanup request does not match its immutable plan",
        ))
    }
}

pub fn validate_provider_recovered_resource_cleanup_execution(
    plan: &ProviderRecoveredResourceCleanupPlan,
    request: &ProviderRecoveredResourceCleanupRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    validate_provider_recovered_resource_cleanup_request(plan, request)?;
    services.require_execution_host(plan.preflight().execution_host_id())?;
    let available = services.available_kinds();
    if plan
        .preflight()
        .requirements()
        .host_services()
        .any(|required| !available.contains(&required))
    {
        return Err(failure(
            "swallowtail.provider_recovered_resource_cleanup.service_unavailable",
            "Recovered-resource cleanup host services are unavailable",
        ));
    }
    Ok(())
}

fn validate_plan(
    preflight: &PreflightPlan,
    agreement: &ProviderRecoveredResourceCleanupAgreement,
) -> Result<(), RuntimeFailure> {
    let requirements = preflight.requirements();
    if requirements.driver_role() != DriverRole::ProviderRecoveredResourceCleanup
        || requirements.operation_shape() != OperationShape::ProviderRecoveredResourceCleanup
        || !agreement.binding().matches_plan(preflight)
    {
        return Err(plan_mismatch());
    }
    let capability = requirements
        .capabilities()
        .find(|required| required.capability() == Capability::ProviderRecoveredResourceCleanup)
        .ok_or_else(plan_mismatch)?;
    let declared = capability
        .constraints()
        .map(|constraint| match constraint {
            CapabilityConstraint::OwnedRemoteResource(kind) => Ok(*kind),
            _ => Err(plan_mismatch()),
        })
        .collect::<Result<BTreeSet<OwnedRemoteResourceKind>, RuntimeFailure>>()?;
    let bound = agreement
        .binding()
        .resource_kinds()
        .collect::<BTreeSet<_>>();
    if declared != bound || capability.constraints().count() != bound.len() {
        return Err(failure(
            "swallowtail.provider_recovered_resource_cleanup.resource_scope_mismatch",
            "Recovered-resource cleanup scope differs from its capability plan",
        ));
    }
    if agreement.deadline().is_some()
        && !requirements
            .host_services()
            .any(|required| required == swallowtail_core::HostServiceKind::Time)
    {
        return Err(failure(
            "swallowtail.provider_recovered_resource_cleanup.time_service_required",
            "Deadline-bound recovered-resource cleanup requires time service",
        ));
    }
    Ok(())
}

fn plan_mismatch() -> RuntimeFailure {
    failure(
        "swallowtail.provider_recovered_resource_cleanup.plan_mismatch",
        "Recovered-resource cleanup does not match its immutable binding",
    )
}

fn failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

#[cfg(test)]
mod tests;
