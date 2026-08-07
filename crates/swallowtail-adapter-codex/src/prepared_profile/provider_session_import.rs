use super::plan::failure;
use crate::{CodexPreparedIntegration, selection::supports_thread_catalogue_version};
use swallowtail_core::{Capability, CapabilityConstraint, CapabilityRequirement, ResourceAccess, ResourceRepresentation};
use swallowtail_runtime::PreparationFailure;

mod catalogue;
mod import;
mod reconciliation;

pub use catalogue::CodexPreparedSessionCatalogue;
pub use import::CodexPreparedSessionImport;
pub use reconciliation::CodexPreparedSessionReconciliation;

pub(super) fn require_catalogue_version(
    prepared: &CodexPreparedIntegration,
) -> Result<(), PreparationFailure> {
    if supports_thread_catalogue_version(prepared.observation().version().version()) {
        Ok(())
    } else {
        Err(failure(
            "swallowtail.codex.preparation.thread_catalogue_version_unsupported",
            "Prepared Codex version does not support the qualified thread catalogue",
        ))
    }
}

pub(super) fn read_only_working_resource_capability() -> CapabilityRequirement {
    CapabilityRequirement::new(
        Capability::WorkingResource,
        [
            CapabilityConstraint::ResourceAccess(ResourceAccess::Read),
            CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
        ],
    )
}
