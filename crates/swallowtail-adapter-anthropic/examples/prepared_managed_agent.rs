#![allow(dead_code)]

use swallowtail_adapter_anthropic::{
    AnthropicManagedAgentRunInput, AnthropicManagedPreparationInput,
    AnthropicManagedPreparedIntegration, AnthropicPreparedManagedAgentRun,
    prepare_anthropic_managed_agent,
};
use swallowtail_runtime::{HostServices, PreparationFailure};

fn prepare_integration(
    input: AnthropicManagedPreparationInput,
    services: &HostServices,
) -> Result<AnthropicManagedPreparedIntegration, PreparationFailure> {
    prepare_anthropic_managed_agent(input, services)
}

fn prepare_run(
    integration: &AnthropicManagedPreparedIntegration,
    input: AnthropicManagedAgentRunInput,
) -> Result<AnthropicPreparedManagedAgentRun, PreparationFailure> {
    integration.prepare_managed_run(input)
}

fn main() {}
