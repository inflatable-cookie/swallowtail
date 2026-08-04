#![allow(dead_code)]

use swallowtail_adapter_anthropic::{
    AnthropicManagedAgentRunInput, AnthropicManagedPreparationInput,
    AnthropicManagedPreparedIntegration, AnthropicManagedRecoveredCleanupInput,
    AnthropicManagedRunReconciliationInput, AnthropicPreparedManagedAgentRun,
    AnthropicPreparedManagedRecoveredCleanup, AnthropicPreparedManagedRunReconciliation,
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

fn prepare_recoverable_run(
    integration: &AnthropicManagedPreparedIntegration,
    input: AnthropicManagedAgentRunInput,
) -> Result<AnthropicPreparedManagedAgentRun, PreparationFailure> {
    integration.prepare_managed_run(input.with_cross_process_recovery())
}

fn prepare_reconciliation(
    integration: &AnthropicManagedPreparedIntegration,
    input: AnthropicManagedRunReconciliationInput,
) -> Result<AnthropicPreparedManagedRunReconciliation, PreparationFailure> {
    integration.prepare_run_reconciliation(input)
}

fn prepare_recovered_cleanup(
    integration: &AnthropicManagedPreparedIntegration,
    input: AnthropicManagedRecoveredCleanupInput,
) -> Result<AnthropicPreparedManagedRecoveredCleanup, PreparationFailure> {
    integration.prepare_recovered_cleanup(input)
}

fn main() {}
