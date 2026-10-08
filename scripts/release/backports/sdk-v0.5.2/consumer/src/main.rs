use swallowtail_adapter_claude_agent::sdk::{
    ClaudeAgentSdkSessionPreparation, prepare_claude_agent_sdk_session,
};
use swallowtail_core::{
    AccessProfileId, ConfiguredInstanceId, CredentialRef, ExecutionHostId, InstanceRevision,
    InstanceTargetRef, ModelId, ModelRouteId, ModelRouteRevision,
};
use swallowtail_runtime::{
    Deadline, EnvironmentRef, MonotonicInstant, PreparedRegisteredToolBinding, ProviderObservation,
    RegisteredToolBridgeLease, RegisteredToolOpenRequest, RegisteredToolPreparation, RequestId,
    SessionOptions, TokenUsage, WorkingResourceRef,
};

fn lease_api(lease: &RegisteredToolBridgeLease) -> (Option<Deadline>, Deadline) {
    (lease.lease_deadline(), lease.next_call_deadline())
}

fn prepared_binding_api(binding: &PreparedRegisteredToolBinding) -> Option<Deadline> {
    binding.lease_deadline()
}

fn open_request_api(request: RegisteredToolOpenRequest) -> Option<Deadline> {
    request.lease_deadline()
}

fn main() {
    let input = ClaudeAgentSdkSessionPreparation::new(
        ConfiguredInstanceId::new("consumer.instance").expect("instance"),
        InstanceRevision::new("consumer-revision").expect("revision"),
        ExecutionHostId::new("consumer.host").expect("host"),
        InstanceTargetRef::new("consumer.launch-recipe").expect("target"),
        EnvironmentRef::new("consumer.environment").expect("environment"),
        CredentialRef::new("consumer.opaque-credential-reference").expect("credential reference"),
        AccessProfileId::new("consumer.access").expect("access"),
        ModelRouteId::new("consumer.route").expect("route"),
        ModelRouteRevision::new("consumer-route-revision").expect("route revision"),
        ModelId::new("claude-sonnet-5").expect("model"),
        WorkingResourceRef::new("consumer.workspace").expect("workspace"),
        RequestId::new("consumer.prepare").expect("request"),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
    );
    let prepared = prepare_claude_agent_sdk_session(input, SessionOptions::default())
        .expect("provider-free prepared facade construction");
    drop(prepared);

    let usage = TokenUsage::new(Some(21), Some(5)).with_cache_tokens(Some(8), Some(9));
    assert_eq!(usage.input_tokens(), Some(21));
    assert_eq!(usage.cache_read_input_tokens(), Some(8));
    assert_eq!(usage.cache_write_input_tokens(), Some(9));
    let observation = ProviderObservation::Usage(usage);
    assert!(matches!(observation, ProviderObservation::Usage(_)));
    let _ = RegisteredToolPreparation::prepare_with_lease_deadline;
    let _ = lease_api as fn(&RegisteredToolBridgeLease) -> (Option<Deadline>, Deadline);
    let _ = prepared_binding_api as fn(&PreparedRegisteredToolBinding) -> Option<Deadline>;
    let _ = open_request_api as fn(RegisteredToolOpenRequest) -> Option<Deadline>;
}
