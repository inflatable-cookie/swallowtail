use super::KimiPreparedSession;
use super::plan::failure;
use swallowtail_core::{ExecutionHostId, InterfaceVersionBinding, SessionRef};
use swallowtail_runtime::{PreparationFailure, SessionResumeBinding, WorkingResourceRef};

/// Opaque evidence authorizing one Kimi ACP session for an explicit
/// cross-transport management-binding import.
///
/// This is not a local-server management binding. It only carries the exact
/// source-route evidence required by the later import preflight.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KimiAcpSessionImportAuthority {
    pub(crate) provider_session_ref: SessionRef,
    pub(crate) execution_host_id: ExecutionHostId,
    pub(crate) executable_version: InterfaceVersionBinding,
    pub(crate) state_root: WorkingResourceRef,
    pub(crate) qualified_version: bool,
}

impl KimiPreparedSession {
    /// Issues import authority only from a matching durable ACP resume
    /// binding and an explicitly bound Kimi state root.
    pub fn authorize_local_server_import(
        &self,
        binding: SessionResumeBinding,
    ) -> Result<KimiAcpSessionImportAuthority, PreparationFailure> {
        let working_resource = self
            .request()
            .working_resource()
            .expect("prepared Kimi session binds a working resource");
        if !binding.matches_attachment(
            self.plan(),
            working_resource,
            self.request().access_policy(),
        ) {
            return Err(failure(
                "swallowtail.kimi.import.acp_binding_mismatch",
                "Kimi ACP resume binding does not match its prepared session",
            ));
        }
        let state_root = self.evidence().state_root().cloned().ok_or_else(|| {
            failure(
                "swallowtail.kimi.import.state_root_missing",
                "Kimi ACP preparation did not bind an opaque state-root identity",
            )
        })?;
        let observation = self.evidence().observation();
        Ok(KimiAcpSessionImportAuthority {
            provider_session_ref: binding.provider_session_ref().clone(),
            execution_host_id: observation.execution_host_id().clone(),
            executable_version: observation.version().clone(),
            state_root,
            qualified_version: observation.is_qualified(),
        })
    }
}
