use super::{
    ProviderSessionCataloguePlan, ProviderSessionCatalogueRequest, ProviderSessionImportPlan,
    ProviderSessionImportRequest, ProviderSessionOperationFailure,
    ProviderSessionOperationFailureStage, failure,
};
use crate::plan_family::{validate_agreement_matches_plan, validate_execution_services};
use crate::{HostServices, RuntimeFailure};

/// Verifies that a catalogue request matches its immutable plan and cursor.
pub fn validate_provider_session_catalogue_request(
    plan: &ProviderSessionCataloguePlan,
    request: &ProviderSessionCatalogueRequest,
) -> Result<(), RuntimeFailure> {
    validate_agreement_matches_plan(
        plan.agreement(),
        request.agreement(),
        "swallowtail.provider_session_catalogue.plan_mismatch",
        "Provider-session catalogue request does not match its immutable plan",
    )?;
    if request
        .cursor()
        .is_none_or(|cursor| cursor.matches_plan(plan))
    {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.provider_session_catalogue.plan_mismatch",
            "Provider-session catalogue request does not match its immutable plan",
        ))
    }
}

/// Verifies that an import request matches its immutable plan.
pub fn validate_provider_session_import_request(
    plan: &ProviderSessionImportPlan,
    request: &ProviderSessionImportRequest,
) -> Result<(), RuntimeFailure> {
    validate_agreement_matches_plan(
        plan.agreement(),
        request.agreement(),
        "swallowtail.provider_session_import.plan_mismatch",
        "Provider-session import request does not match its immutable plan",
    )
}

/// Verifies a catalogue request and the host services needed to execute it.
pub fn validate_provider_session_catalogue_execution(
    plan: &ProviderSessionCataloguePlan,
    request: &ProviderSessionCatalogueRequest,
    services: &HostServices,
) -> Result<(), ProviderSessionOperationFailure> {
    validate_provider_session_catalogue_request(plan, request).map_err(before_dispatch_failure)?;
    validate_services(plan.preflight(), services)
}

/// Verifies an import request and the host services needed to execute it.
pub fn validate_provider_session_import_execution(
    plan: &ProviderSessionImportPlan,
    request: &ProviderSessionImportRequest,
    services: &HostServices,
) -> Result<(), ProviderSessionOperationFailure> {
    validate_provider_session_import_request(plan, request).map_err(before_dispatch_failure)?;
    validate_services(plan.preflight(), services)
}

fn validate_services(
    plan: &swallowtail_core::PreflightPlan,
    services: &HostServices,
) -> Result<(), ProviderSessionOperationFailure> {
    validate_execution_services(
        plan,
        services,
        "swallowtail.provider_session_operation.service_unavailable",
        "Provider-session operation host services are unavailable",
    )
    .map_err(before_dispatch_failure)
}

fn before_dispatch_failure(failure: RuntimeFailure) -> ProviderSessionOperationFailure {
    ProviderSessionOperationFailure::from_runtime(
        ProviderSessionOperationFailureStage::BeforeDispatch,
        failure,
    )
}
