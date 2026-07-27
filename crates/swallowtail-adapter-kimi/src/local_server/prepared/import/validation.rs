use super::KimiLocalServerPreparedBindingImport;
use crate::local_server::prepared::input::KimiLocalServerBindingImportInput;
use crate::local_server::prepared::{
    KimiLocalServerPreparationProbe, KimiLocalServerPreparedIntegration, preparation_failure,
    runtime_preparation_failure,
};
use crate::local_server::transport::session_path;
use swallowtail_core::Capability;
use swallowtail_runtime::{PreparationFailure, PreparationStage};

impl KimiLocalServerPreparedIntegration {
    pub fn prepare_binding_import(
        &self,
        input: KimiLocalServerBindingImportInput,
    ) -> Result<KimiLocalServerPreparedBindingImport, PreparationFailure> {
        if input.target != self.binding_import_target() {
            return Err(preparation_failure(
                PreparationStage::Preflight,
                "swallowtail.kimi.local_server.import.target_mismatch",
                "Kimi local-server import target does not match the prepared integration",
            ));
        }
        if input.source.execution_host_id != *self.instance().execution_host_id() {
            return Err(preparation_failure(
                PreparationStage::Preflight,
                "swallowtail.kimi.local_server.import.host_mismatch",
                "Kimi ACP and local-server execution hosts do not match",
            ));
        }
        if input.source.executable_version != *self.server().binding() {
            return Err(preparation_failure(
                PreparationStage::CompatibilityClassification,
                "swallowtail.kimi.local_server.import.version_mismatch",
                "Kimi ACP and local-server executable releases do not match",
            ));
        }
        if input.source.state_root != *self.state_root() {
            return Err(preparation_failure(
                PreparationStage::Preflight,
                "swallowtail.kimi.local_server.import.state_root_mismatch",
                "Kimi ACP and local-server state-root identities do not match",
            ));
        }
        if (!input.source.qualified_version || !self.server().is_qualified())
            && !input.allow_unverified_newer
        {
            return Err(preparation_failure(
                PreparationStage::CompatibilityClassification,
                "swallowtail.kimi.local_server.import.unverified_newer",
                "Unverified newer Kimi import requires explicit consumer acceptance",
            ));
        }
        if !self
            .instance()
            .capabilities()
            .supports(Capability::ProviderSessionArchive)
            || !self
                .instance()
                .capabilities()
                .supports(Capability::ProviderSessionRestore)
            || self
                .instance()
                .capabilities()
                .supports(Capability::ProviderSessionDelete)
        {
            return Err(preparation_failure(
                PreparationStage::Preflight,
                "swallowtail.kimi.local_server.import.capability_mismatch",
                "Kimi local-server import target does not expose the qualified lifecycle set",
            ));
        }
        session_path(input.source.provider_session_ref.as_provider_value())
            .map_err(|error| runtime_preparation_failure(PreparationStage::Preflight, error))?;
        Ok(KimiLocalServerPreparedBindingImport {
            request_id: input.request_id,
            target: input.target,
            provider_session_ref: input.source.provider_session_ref,
            probe: KimiLocalServerPreparationProbe::new(
                input.scope_id,
                input.deadline,
                input.cancellation,
            ),
        })
    }
}
