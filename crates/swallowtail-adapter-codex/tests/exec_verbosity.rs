use crate::support;

use futures_executor::block_on;
use support::{
    FakeProcessService, current_exec_policy, exec_policy_for_version, host_services, plan,
    plan_with, plan_with_version, working_resource,
};
use swallowtail_adapter_codex::{CodexExecDriver, CodexModelVerbosity};
use swallowtail_core::{Capability, CapabilityConstraint, CapabilityRequirement, ReasoningMode};
use swallowtail_runtime::{
    EnvironmentRef, OperationContent, RequestId, StructuredRunDriver, StructuredRunRequest,
};

const COMPLETED_JSONL: &str = concat!(
    "{\"type\":\"thread.started\",\"thread_id\":\"private-thread\"}\n",
    "{\"type\":\"turn.started\"}\n",
    "{\"type\":\"item.completed\",\"item\":{\"id\":\"message-1\",\"type\":\"agent_message\",\"text\":\"finished\"}}\n",
    "{\"type\":\"turn.completed\"}\n"
);

#[test]
fn explicit_model_verbosity_appends_quoted_config_without_changing_omission() {
    let (process, state) = FakeProcessService::completed(COMPLETED_JSONL);
    let request = StructuredRunRequest::new(
        RequestId::new("request-verbosity").expect("request id is valid"),
        OperationContent::new("private prompt").expect("content is valid"),
        current_exec_policy(),
    )
    .with_working_resource(working_resource());
    let handle = block_on(
        driver()
            .with_model_verbosity(CodexModelVerbosity::High)
            .start_run(plan(), request, host_services(process)),
    )
    .expect("run starts");
    assert_eq!(
        block_on(handle.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    );
    let arguments = state.request().arguments;
    assert!(
        arguments
            .windows(2)
            .any(|pair| pair == ["--config", "model_verbosity=\"high\""])
    );
}

#[test]
fn model_verbosity_composes_with_reasoning_without_serializing_a_default() {
    let (process, state) = FakeProcessService::completed(COMPLETED_JSONL);
    let reasoning = ReasoningMode::new("high").expect("reasoning mode is valid");
    let policy = current_exec_policy().with_reasoning_mode(reasoning.clone());
    let request = StructuredRunRequest::new(
        RequestId::new("request-verbosity-compose").expect("request id is valid"),
        OperationContent::new("private prompt").expect("content is valid"),
        policy,
    )
    .with_working_resource(working_resource());
    let handle = block_on(
        driver()
            .with_model_verbosity(CodexModelVerbosity::Low)
            .start_run(
                plan_with(
                    [CapabilityRequirement::new(
                        Capability::ReasoningSelection,
                        [CapabilityConstraint::reasoning_mode(reasoning)],
                    )],
                    [],
                ),
                request,
                host_services(process),
            ),
    )
    .expect("composed run starts");
    assert_eq!(
        block_on(handle.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    );
    let arguments = state.request().arguments;
    assert!(
        arguments
            .windows(2)
            .any(|pair| pair == ["--config", "model_reasoning_effort=\"high\""])
    );
    assert!(
        arguments
            .windows(2)
            .any(|pair| pair == ["--config", "model_verbosity=\"low\""])
    );
}

#[test]
fn older_maintained_version_rejects_verbosity_before_process_start() {
    let (process, state) = FakeProcessService::completed("");
    let request = StructuredRunRequest::new(
        RequestId::new("request-old-verbosity").expect("request id is valid"),
        OperationContent::new("private prompt").expect("content is valid"),
        exec_policy_for_version("0.122.0"),
    )
    .with_working_resource(working_resource());
    let failure = block_on(
        driver()
            .with_model_verbosity(CodexModelVerbosity::Low)
            .start_run(
                plan_with_version("0.122.0", [], []),
                request,
                host_services(process),
            ),
    )
    .err()
    .expect("unsupported version must fail");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.codex.exec.model_verbosity_unsupported"
    );
    assert!(!state.started());
}

fn driver() -> CodexExecDriver {
    CodexExecDriver::new(EnvironmentRef::new("codex-saved-login").expect("environment is valid"))
}
