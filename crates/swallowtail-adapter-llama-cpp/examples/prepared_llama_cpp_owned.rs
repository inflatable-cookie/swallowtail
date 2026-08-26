#![allow(dead_code)]

use swallowtail_adapter_llama_cpp::{
    LlamaCppContextSize, LlamaCppOwnedPreparationInput, LlamaCppOwnedPreparedIntegration,
    LlamaCppOwnedServingSelection, LlamaCppPreparedServingStart, LlamaCppReasoningSelection,
    prepare_llama_cpp_owned,
};
use swallowtail_runtime::{Deadline, HostServices, PreparationFailure, ScopeId, ServingInstanceId};

fn prepare_integration(
    input: LlamaCppOwnedPreparationInput,
    services: &HostServices,
) -> Result<LlamaCppOwnedPreparedIntegration, PreparationFailure> {
    prepare_llama_cpp_owned(input, services)
}

fn prepare_start(
    integration: &LlamaCppOwnedPreparedIntegration,
    scope: ScopeId,
    serving_instance_id: ServingInstanceId,
    deadline: Deadline,
) -> Result<LlamaCppPreparedServingStart, PreparationFailure> {
    integration.prepare_serving_start(scope, serving_instance_id, deadline)
}

fn with_context_size(
    serving: LlamaCppOwnedServingSelection,
    context_size: LlamaCppContextSize,
) -> LlamaCppOwnedServingSelection {
    serving.with_context_size(context_size)
}

fn with_reasoning_disabled(
    serving: LlamaCppOwnedServingSelection,
) -> LlamaCppOwnedServingSelection {
    serving.with_reasoning(LlamaCppReasoningSelection::Disabled)
}

fn main() {}
