#![allow(dead_code)]

use swallowtail_adapter_kimi::{
    KimiLocalServerOwnedInput, KimiLocalServerPreparationProbe,
    KimiLocalServerSessionManagementInput, start_kimi_local_server_owned,
};
use swallowtail_runtime::{
    CleanupOutcome, HostServices, ProviderSessionManagementBinding,
    ProviderSessionManagementOutcome, RequestId,
};

async fn archive_with_owned_foreground_server(
    input: KimiLocalServerOwnedInput,
    probe: KimiLocalServerPreparationProbe,
    binding: ProviderSessionManagementBinding,
    request_id: RequestId,
    services: HostServices,
) -> Result<(ProviderSessionManagementOutcome, CleanupOutcome), String> {
    let owned = start_kimi_local_server_owned(input, probe, services.clone())
        .await
        .map_err(|error| error.to_string())?;
    let archive = owned
        .prepared()
        .prepare_archive_session(KimiLocalServerSessionManagementInput::new(
            request_id, binding,
        ))
        .map_err(|error| error.to_string())?;
    let outcome = archive
        .execute(services)
        .await
        .map_err(|error| error.to_string())?;
    Ok((outcome, owned.close().await))
}

fn main() {}
