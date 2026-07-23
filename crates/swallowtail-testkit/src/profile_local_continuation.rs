use crate::{
    ConformanceAssertion, ConformanceReport, ExecutionTopologyFixture, ProfilePreflightFixture,
    RecordedHostCall, RecordingHostServices, RecordingOutcome, SyntheticProfile,
    assert_common_contract, poll_immediate,
};
use std::num::NonZeroU64;
use swallowtail_core::{
    DirectAttemptTransport, ExecutionLayer, OperationShape, ProviderInferenceCachePolicy,
};
use swallowtail_runtime::{
    CleanupOutcome, CredentialRef, Deadline, DirectContinuationBinding, DirectContinuationState,
    DirectContinuationTurnRequest, DirectToolArguments, DirectToolCall, DirectToolCallId,
    DirectToolResult, DirectToolResultContent, EndpointRef, MonotonicInstant,
    OpenDirectContinuationSessionRequest, OperationContent, ProviderPrivateContinuationRecord,
    RequestId, RuntimeSessionId, RuntimeTurnId, ScopeId, validate_direct_continuation_plan,
};

pub(crate) fn run() -> ConformanceReport {
    let profile = SyntheticProfile::LocallyContinuedDirectSession;
    let mut report = ConformanceReport::new(profile);
    assert_common_contract(profile, &mut report);

    let fixture = ProfilePreflightFixture::new(profile);
    let plan = fixture
        .preflight()
        .expect("direct-continuation preflight succeeds");
    let required = plan
        .requirements()
        .direct_continuation()
        .expect("continuation policy is bound");
    assert_eq!(
        plan.requirements().execution_layer(),
        ExecutionLayer::DirectModelInference
    );
    assert_eq!(
        plan.requirements().operation_shape(),
        OperationShape::InteractiveSession
    );
    assert_eq!(
        required.config().initial_attempt_transport(),
        DirectAttemptTransport::Buffered
    );
    assert_eq!(
        required.config().continued_attempt_transport(),
        DirectAttemptTransport::ServerSentEvents
    );
    assert_eq!(
        required.config().provider_cache_policy(),
        ProviderInferenceCachePolicy::AcceptedWithoutManagementAuthority
    );

    let open = OpenDirectContinuationSessionRequest::new(
        RequestId::new("continued-open").expect("request id is valid"),
        required.config().clone(),
    );
    validate_direct_continuation_plan(&plan, &open).expect("request agrees with plan");

    assert_explicit_attempt_authorization(required.config().clone());
    assert_private_record_binding(&plan);
    assert_joined_request_cleanup(&plan);
    assert_topology_authority();

    report.record(ConformanceAssertion::SessionLifecycle);
    report.record(ConformanceAssertion::HostTopologyPreserved);
    report.record(ConformanceAssertion::HostedEndpointCredentialBinding);
    report.record(ConformanceAssertion::DirectSessionNoResource);
    report.record(ConformanceAssertion::NoImplicitSessionRecovery);
    report.record(ConformanceAssertion::ExplicitAttemptAuthorization);
    report.record(ConformanceAssertion::ConsumerToolExchange);
    report.record(ConformanceAssertion::PrivateContinuationBounded);
    report.record(ConformanceAssertion::ProviderCachePosture);
    report.record(ConformanceAssertion::RequestScopedLeaseLifecycle);
    report
}

fn assert_explicit_attempt_authorization(config: swallowtail_core::DirectContinuationConfig) {
    let maximum_arguments = usize::try_from(config.maximum_tool_argument_bytes().get()).unwrap();
    let maximum_result = usize::try_from(config.maximum_tool_result_bytes().get()).unwrap();
    let mut state = DirectContinuationState::new(config);
    let turn = DirectContinuationTurnRequest::new(
        RuntimeTurnId::new("continued-turn-1").unwrap(),
        OperationContent::new("private question").unwrap(),
        Deadline::at(MonotonicInstant::from_ticks(20)),
    );
    let first = state
        .authorize_user_turn(&turn)
        .expect("user action authorizes attempt");
    let call = DirectToolCall::new(
        DirectToolCallId::new("continued-call-1").unwrap(),
        first.attempt_id().clone(),
        "lookup",
        DirectToolArguments::new(br#"{"subject":"private"}"#.to_vec(), maximum_arguments).unwrap(),
    )
    .unwrap();
    state
        .pause_for_tool_calls(&first, std::slice::from_ref(&call))
        .unwrap();
    assert_eq!(state.pending_tool_calls(), 1);
    assert!(state.authorize_tool_results(&[]).is_err());
    let result = DirectToolResult::new(
        call.call_id().clone(),
        DirectToolResultContent::new(b"private answer".to_vec(), maximum_result).unwrap(),
    );
    let second = state.authorize_tool_results(&[result]).unwrap();
    assert_eq!(second.ordinal().get(), 2);
    state.complete_turn().unwrap();
}

fn assert_private_record_binding(plan: &swallowtail_core::PreflightPlan) {
    let session_id = RuntimeSessionId::new("continued-session").unwrap();
    let binding = DirectContinuationBinding::from_plan(plan, session_id.clone()).unwrap();
    let mut state = DirectContinuationState::new(
        plan.requirements()
            .direct_continuation()
            .unwrap()
            .config()
            .clone(),
    );
    let attempt = state
        .authorize_user_turn(&DirectContinuationTurnRequest::new(
            RuntimeTurnId::new("continued-binding-turn").unwrap(),
            OperationContent::new("private").unwrap(),
            Deadline::at(MonotonicInstant::from_ticks(30)),
        ))
        .unwrap();
    let record = ProviderPrivateContinuationRecord::new(
        binding,
        attempt.attempt_id().clone(),
        NonZeroU64::new(128).unwrap(),
        NonZeroU64::new(256).unwrap(),
    )
    .unwrap();
    assert!(record.matches_plan(plan, &session_id));
    let rendered = format!("{record:?}");
    assert!(rendered.contains("<private:128 bytes>"));
    assert!(!rendered.contains("continued-session"));
    assert!(
        ProviderPrivateContinuationRecord::new(
            DirectContinuationBinding::from_plan(plan, session_id).unwrap(),
            attempt.attempt_id().clone(),
            NonZeroU64::new(257).unwrap(),
            NonZeroU64::new(256).unwrap(),
        )
        .is_err()
    );
}

fn assert_joined_request_cleanup(plan: &swallowtail_core::PreflightPlan) {
    let recording = RecordingHostServices::for_host(
        plan.execution_host_id().clone(),
        RecordingOutcome::Succeed,
    );
    let services = recording.services();
    let scope = ScopeId::new("continued-session-scope").unwrap();
    let audience = plan.endpoint_audience().clone();
    let _grant = poll_immediate(services.network().unwrap().authorize(
        scope.clone(),
        EndpointRef::new("continued-endpoint").unwrap(),
        audience.clone(),
    ))
    .unwrap();
    let credential = poll_immediate(services.credential().unwrap().acquire(
        scope.clone(),
        CredentialRef::new("continued-credential").unwrap(),
        audience,
    ))
    .unwrap();
    let task = services
        .task()
        .unwrap()
        .spawn(scope, Box::pin(async {}))
        .unwrap();
    poll_immediate(task.join()).unwrap();
    assert_eq!(
        poll_immediate(services.credential().unwrap().release(credential)),
        CleanupOutcome::Clean
    );
    let calls = recording.calls();
    assert!(
        position(&calls, RecordedHostCall::TaskJoin)
            < position(&calls, RecordedHostCall::CredentialRelease)
    );
    assert_eq!(recording.count(RecordedHostCall::ProcessStart), 0);
    assert_eq!(recording.count(RecordedHostCall::WorkingResourceResolve), 0);
}

fn assert_topology_authority() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        RecordingHostServices::for_host(
            topology.execution_host_id().clone(),
            RecordingOutcome::Succeed,
        )
        .services()
        .require_execution_host(topology.execution_host_id())
        .expect("topology authority is retained");
    }
}

fn position(calls: &[RecordedHostCall], expected: RecordedHostCall) -> usize {
    calls.iter().position(|call| *call == expected).unwrap()
}
