#![allow(dead_code)]

use swallowtail_adapter_deepseek_harness::{
    DeepSeekHarnessWebPreparationInput, DeepSeekHarnessWebPreparationProbe,
    DeepSeekHarnessWebPreparedIntegration, DeepSeekHarnessWebPreparedRun,
    DeepSeekHarnessWebRunProfileInput, prepare_deepseek_harness_web,
};
use swallowtail_runtime::{HostServices, PreparationFailure};

async fn prepare_installation(
    input: DeepSeekHarnessWebPreparationInput,
    probe: DeepSeekHarnessWebPreparationProbe,
    services: HostServices,
) -> Result<DeepSeekHarnessWebPreparedIntegration, PreparationFailure> {
    prepare_deepseek_harness_web(input, probe, services).await
}

fn prepare_run(
    integration: &DeepSeekHarnessWebPreparedIntegration,
    input: DeepSeekHarnessWebRunProfileInput,
) -> Result<DeepSeekHarnessWebPreparedRun, PreparationFailure> {
    integration.prepare_run(input)
}

fn main() {}
