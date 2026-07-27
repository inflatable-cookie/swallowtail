#![allow(dead_code)]

use swallowtail_adapter_kimi::{
    KimiLocalServerBindingImportInput, KimiLocalServerPreparedIntegration,
    KimiLocalServerSessionManagementInput,
};
use swallowtail_runtime::{HostServices, ProviderSessionManagementOutcome, RequestId};

async fn import_inactive_acp_session_and_archive(
    prepared: &KimiLocalServerPreparedIntegration,
    import: KimiLocalServerBindingImportInput,
    archive_request_id: RequestId,
    services: HostServices,
) -> Result<ProviderSessionManagementOutcome, String> {
    let binding = prepared
        .prepare_binding_import(import)
        .map_err(|error| error.to_string())?
        .execute(services.clone())
        .await
        .map_err(|error| error.to_string())?;
    prepared
        .prepare_archive_session(KimiLocalServerSessionManagementInput::new(
            archive_request_id,
            binding,
        ))
        .map_err(|error| error.to_string())?
        .execute(services)
        .await
        .map_err(|error| error.to_string())
}

fn main() {}
