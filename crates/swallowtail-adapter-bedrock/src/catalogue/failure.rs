use super::projection::CatalogueProjectionError;
use crate::failure::failure;
use swallowtail_runtime::RuntimeFailure;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum CatalogueFailureKind {
    PermissionDenied,
    InvalidRequest,
    RateLimited,
    ProviderUnavailable,
    ProviderFailed,
}

pub(super) fn provider_failure(kind: CatalogueFailureKind) -> RuntimeFailure {
    let (code, message) = match kind {
        CatalogueFailureKind::PermissionDenied => (
            "swallowtail.bedrock.catalogue_access_denied",
            "Bedrock catalogue rejected authentication or authorization",
        ),
        CatalogueFailureKind::InvalidRequest => (
            "swallowtail.bedrock.catalogue_invalid_request",
            "Bedrock catalogue rejected the request",
        ),
        CatalogueFailureKind::RateLimited => (
            "swallowtail.bedrock.catalogue_rate_limited",
            "Bedrock catalogue rate limit was reached",
        ),
        CatalogueFailureKind::ProviderUnavailable => (
            "swallowtail.bedrock.catalogue_unavailable",
            "Bedrock catalogue was unavailable",
        ),
        CatalogueFailureKind::ProviderFailed => (
            "swallowtail.bedrock.catalogue_provider_failed",
            "Bedrock catalogue returned an unknown failure",
        ),
    };
    failure(code, message)
}

pub(super) fn projection_failure(error: CatalogueProjectionError) -> RuntimeFailure {
    let (code, message) = match error {
        CatalogueProjectionError::TooManyEntries => (
            "swallowtail.bedrock.catalogue_entry_limit",
            "Bedrock catalogue exceeded its bounded entry limit",
        ),
        CatalogueProjectionError::TooManyObservationValues => (
            "swallowtail.bedrock.catalogue_observation_limit",
            "Bedrock catalogue exceeded an observation-value limit",
        ),
        CatalogueProjectionError::InvalidModelId
        | CatalogueProjectionError::FieldBoundExceeded
        | CatalogueProjectionError::InvalidProviderObservation => (
            "swallowtail.bedrock.catalogue_projection_failed",
            "Bedrock catalogue returned invalid or unsafe model metadata",
        ),
    };
    failure(code, message)
}
