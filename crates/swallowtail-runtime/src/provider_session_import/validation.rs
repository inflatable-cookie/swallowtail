use super::{
    ProviderSessionCataloguePlan, ProviderSessionCatalogueRequest, ProviderSessionImportPlan,
    ProviderSessionImportRequest, failure,
};
use crate::RuntimeFailure;

pub fn validate_provider_session_catalogue_request(
    plan: &ProviderSessionCataloguePlan,
    request: &ProviderSessionCatalogueRequest,
) -> Result<(), RuntimeFailure> {
    if plan.agreement() == request.agreement()
        && request
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

pub fn validate_provider_session_import_request(
    plan: &ProviderSessionImportPlan,
    request: &ProviderSessionImportRequest,
) -> Result<(), RuntimeFailure> {
    if plan.agreement() == request.agreement() {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.provider_session_import.plan_mismatch",
            "Provider-session import request does not match its immutable plan",
        ))
    }
}
