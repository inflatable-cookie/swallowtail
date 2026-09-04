#[path = "import/execution.rs"]
mod execution;
#[path = "import/validation.rs"]
mod validation;

use super::KimiLocalServerPreparationProbe;
use super::input::KimiLocalServerBindingImportTarget;
use swallowtail_runtime::{
    BoxFuture, HostServices, PreparationFailure, ProviderSessionManagementBinding,
};

#[derive(Clone, Debug)]
/// Prepared cross-transport import of Kimi ACP session authority.
pub struct KimiLocalServerPreparedBindingImport {
    pub(crate) plan: swallowtail_core::PreflightPlan,
    pub(super) request_id: swallowtail_runtime::RequestId,
    pub(super) target: KimiLocalServerBindingImportTarget,
    pub(super) provider_session_ref: swallowtail_core::SessionRef,
    pub(super) probe: KimiLocalServerPreparationProbe,
}

impl KimiLocalServerPreparedBindingImport {
    pub(crate) const fn projection_plan(&self) -> &swallowtail_core::PreflightPlan {
        &self.plan
    }
    /// Returns the request identity bound to this import.
    #[must_use]
    pub const fn request_id(&self) -> &swallowtail_runtime::RequestId {
        &self.request_id
    }

    /// Executes the bounded binding import and returns ordinary resume authority.
    pub fn execute(
        &self,
        services: HostServices,
    ) -> BoxFuture<'static, Result<ProviderSessionManagementBinding, PreparationFailure>> {
        let prepared = self.clone();
        Box::pin(async move { execution::execute(prepared, services).await })
    }
}
