use super::{
    ProviderSessionReconciliationAgreement, ProviderSessionReconciliationPlan,
    ProviderSessionReconciliationRequest,
};
use crate::{HostServices, RuntimeFailure};
use swallowtail_core::{
    Capability, CapabilityConstraint, DriverRole, ExecutionLayer, HostServiceKind, OperationShape,
    PreflightPlan, SafeDiagnostic,
};

/// Verifies that execution input still matches its immutable plan.
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

/// Verifies request, execution host, and required host-service availability.
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

pub(super) fn validate_plan(
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

pub(super) fn failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}
