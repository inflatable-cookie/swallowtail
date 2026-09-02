use crate::support;

use futures_executor::block_on;
use support::{
    FakeProcessService, exec_policy_for_version, host_services, plan_with_version, working_resource,
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
        exec_policy_for_version("0.149.1"),
    )
    .with_working_resource(working_resource());
    let handle = block_on(
        driver()
            .with_model_verbosity(CodexModelVerbosity::High)
            .start_run(
                plan_with_version("0.149.1", [], []),
                request,
                host_services(process),
            ),
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
    let policy = exec_policy_for_version("0.149.1").with_reasoning_mode(reasoning.clone());
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
                plan_with_version(
                    "0.149.1",
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
fn unretrieved_version_identities_reject_verbosity_before_process_start() {
    for version in [
        "0.122.0",
        "0.146.99",
        "0.147.0+build.1",
        "0.147.1",
        "0.148.1",
        "0.149.1+build.1",
        "0.152.2",
    ] {
        let (process, state) = FakeProcessService::completed("");
        let request = StructuredRunRequest::new(
            RequestId::new("request-rejected-verbosity").expect("request id is valid"),
            OperationContent::new("private prompt").expect("content is valid"),
            exec_policy_for_version(version),
        )
        .with_working_resource(working_resource());
        let failure = block_on(
            driver()
                .with_model_verbosity(CodexModelVerbosity::Low)
                .start_run(
                    plan_with_version(version, [], []),
                    request,
                    host_services(process),
                ),
        )
        .err()
        .expect("unsupported version must fail");
        assert_eq!(
            failure.diagnostic().code(),
            "swallowtail.codex.exec.model_verbosity_unsupported",
            "unexpected diagnostic for {version}",
        );
        assert!(!state.started(), "process started for {version}");
    }
}

#[test]
fn later_qualified_versions_reject_verbosity_before_process_start() {
    for version in ["0.150.0", "0.150.1", "0.151.0", "0.152.0", "0.152.1"] {
        let (process, state) = FakeProcessService::completed("");
        let request = StructuredRunRequest::new(
            RequestId::new("request-later-qualified-verbosity").expect("request id is valid"),
            OperationContent::new("private prompt").expect("content is valid"),
            exec_policy_for_version(version),
        )
        .with_working_resource(working_resource());
        let failure = block_on(
            driver()
                .with_model_verbosity(CodexModelVerbosity::Low)
                .start_run(
                    plan_with_version(version, [], []),
                    request,
                    host_services(process),
                ),
        )
        .err()
        .expect("later qualified version must fail closed for verbosity");
        assert_eq!(
            failure.diagnostic().code(),
            "swallowtail.codex.exec.model_verbosity_unsupported",
            "unexpected diagnostic for {version}",
        );
        assert!(!state.started(), "process started for {version}");
    }
}

fn driver() -> CodexExecDriver {
    CodexExecDriver::new(EnvironmentRef::new("codex-saved-login").expect("environment is valid"))
}
