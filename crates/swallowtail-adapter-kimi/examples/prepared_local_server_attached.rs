#![allow(dead_code)]

use swallowtail_adapter_kimi::{
    KimiLocalServerAttachedInput, KimiLocalServerPreparationProbe,
    KimiLocalServerPreparedIntegration, prepare_kimi_local_server_attached,
};
use swallowtail_runtime::{HostServices, PreparationFailure};

async fn prepare_attached_local_server(
    input: KimiLocalServerAttachedInput,
    probe: KimiLocalServerPreparationProbe,
    services: HostServices,
) -> Result<KimiLocalServerPreparedIntegration, PreparationFailure> {
    prepare_kimi_local_server_attached(input, probe, services).await
}

fn main() {}
