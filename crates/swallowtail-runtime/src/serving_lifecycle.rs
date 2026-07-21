use crate::{HostServices, RuntimeFailure, StartServingRequest};
use swallowtail_core::{DriverRole, InstanceOwnership, PreflightPlan, SafeDiagnostic};

/// Rejects owned-serving binding and host-service drift before driver effects.
pub fn validate_owned_serving_start(
    plan: &PreflightPlan,
    request: &StartServingRequest,
    services: &HostServices,
) -> Result<(), RuntimeFailure> {
    services.require_execution_host(plan.execution_host_id())?;

    if plan.requirements().driver_role() != DriverRole::ServingInstanceLifecycle
        || plan.ownership() != InstanceOwnership::HostOwnedEphemeral
    {
        return Err(failure(
            "swallowtail.owned_serving_plan_mismatch",
            "Preflight plan does not authorize ephemeral owned serving",
        ));
    }

    if plan.model_artifact_binding() != Some(request.artifact()) {
        return Err(failure(
            "swallowtail.model_artifact_mismatch",
            "Start request does not match the preflight-bound model artifact",
        ));
    }

    let available = services.available_kinds();
    if plan
        .requirements()
        .host_services()
        .any(|required| !available.contains(&required))
    {
        return Err(failure(
            "swallowtail.owned_serving_service_unavailable",
            "Owned serving host services are unavailable",
        ));
    }

    if services.model_artifact().is_none() || services.serving_endpoint().is_none() {
        return Err(failure(
            "swallowtail.owned_serving_service_unavailable",
            "Owned serving host services are unavailable",
        ));
    }

    Ok(())
}

fn failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}
