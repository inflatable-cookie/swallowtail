use super::KimiLocalServerPreparedBindingImport;
use crate::local_server::prepared::input::KimiLocalServerBindingImportInput;
use crate::local_server::prepared::{
    KimiLocalServerPreparationProbe, KimiLocalServerPreparedIntegration, preparation_failure,
    runtime_preparation_failure,
};
use crate::local_server::transport::session_path;
use swallowtail_core::{
    AccessRequirement, Capability, CredentialState, DriverRole, EndpointAuthorization,
    EntitlementState, ExecutionLayer, OperationRequirements, OperationShape, PreflightContext,
    RuntimeReadiness, preflight,
};
use swallowtail_runtime::{PreparationFailure, PreparationStage};

impl KimiLocalServerPreparedIntegration {
    /// Prepares cross-transport import into this exact local-server target.
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
        let requirements = OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            OperationShape::ProviderSessionManagement,
            DriverRole::ProviderSessionManagement,
            self.instance().execution_host_id().clone(),
            AccessRequirement::new(self.access_profile().id().clone())
                .with_credential_states([CredentialState::Ready])
                .with_entitlement_states([EntitlementState::Available])
                .with_endpoint_authorizations([EndpointAuthorization::Allowed])
                .with_runtime_readiness([RuntimeReadiness::Ready])
                .with_support_authorities([self.access_profile().support_authority()]),
        )
        .with_ownership_modes([self.instance().ownership()])
        .with_host_services(
            crate::kimi_local_server_descriptor()
                .required_host_services(DriverRole::ProviderSessionManagement),
        )
        .with_interface_versions([self.server().binding().clone()]);
        let descriptor = crate::kimi_local_server_descriptor();
        let plan = preflight(
            &PreflightContext::new(
                &descriptor,
                self.instance(),
                self.access_profile(),
                self.access_evidence().status(),
                self.available_host_services(),
            ),
            &requirements,
        )
        .map_err(|error| {
            PreparationFailure::new(
                PreparationStage::Preflight,
                swallowtail_core::Diagnostic::new(error.diagnostic().clone()),
            )
        })?;
        Ok(KimiLocalServerPreparedBindingImport {
            plan,
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
