#![deny(missing_docs)]

mod candidate;
mod catalogue;
mod failure;
mod import;
mod outcome;
mod prepared;
mod validation;

pub use candidate::{ProviderSessionCandidate, ProviderSessionCursor};
pub use catalogue::{
    ProviderSessionCatalogueAgreement, ProviderSessionCataloguePlan,
    ProviderSessionCatalogueRequest, ProviderSessionCatalogueScope,
};
pub use failure::{ProviderSessionOperationFailure, ProviderSessionOperationFailureStage};
pub use import::{
    ProviderSessionImportAgreement, ProviderSessionImportPlan, ProviderSessionImportRequest,
};
pub use outcome::{
    ProviderSessionCatalogueOutcome, ProviderSessionImportOutcome,
    ProviderSessionImportRevalidation,
};
pub use prepared::{
    PreparedProviderSessionCatalogueEvidence, PreparedProviderSessionImportEvidence,
};
pub use validation::{
    validate_provider_session_catalogue_execution, validate_provider_session_catalogue_request,
    validate_provider_session_import_execution, validate_provider_session_import_request,
};

use crate::{RuntimeFailure, WorkingResourceRef};
use swallowtail_core::{
    Capability, CapabilityConstraint, HostServiceKind, PreflightPlan, ResourceAccess,
    ResourceRepresentation, SafeDiagnostic,
};

fn failure(code: &'static str, message: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(SafeDiagnostic::new(code, message))
}

fn requires_capability(plan: &PreflightPlan, capability: Capability) -> bool {
    plan.requirements()
        .capabilities()
        .any(|required| required.capability() == capability)
}

fn requires_service(plan: &PreflightPlan, service: HostServiceKind) -> bool {
    plan.requirements()
        .host_services()
        .any(|required| required == service)
}

fn requires_read_only_working_resource(plan: &PreflightPlan) -> bool {
    let Some(requirement) = plan
        .requirements()
        .capabilities()
        .find(|required| required.capability() == Capability::WorkingResource)
    else {
        return false;
    };
    let constraints = requirement.constraints().collect::<Vec<_>>();
    constraints.contains(&&CapabilityConstraint::ResourceAccess(ResourceAccess::Read))
        && constraints.contains(&&CapabilityConstraint::ResourceRepresentation(
            ResourceRepresentation::Filesystem,
        ))
}

fn same_catalogue_and_import_binding(catalogue: &PreflightPlan, import: &PreflightPlan) -> bool {
    catalogue.driver_identity() == import.driver_identity()
        && catalogue.integration_family() == import.integration_family()
        && catalogue.transport_family() == import.transport_family()
        && catalogue.instance_id() == import.instance_id()
        && catalogue.instance_revision() == import.instance_revision()
        && catalogue.instance_target_ref() == import.instance_target_ref()
        && catalogue.protocol_facade_id() == import.protocol_facade_id()
        && catalogue.instance_policy_id() == import.instance_policy_id()
        && catalogue.execution_host_id() == import.execution_host_id()
        && catalogue.access_profile_id() == import.access_profile_id()
        && catalogue.access_status() == import.access_status()
        && catalogue.credential_mechanism() == import.credential_mechanism()
        && catalogue.credential_reference() == import.credential_reference()
        && catalogue.endpoint_audience() == import.endpoint_audience()
        && catalogue.ownership() == import.ownership()
        && catalogue
            .interface_versions()
            .eq(import.interface_versions())
}

fn same_working_resource(left: &WorkingResourceRef, right: &WorkingResourceRef) -> bool {
    left == right
}

#[cfg(test)]
mod tests;
