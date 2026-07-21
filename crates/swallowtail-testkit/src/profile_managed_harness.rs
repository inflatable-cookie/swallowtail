use crate::{
    CallbackExchangeFixture, ConformanceAssertion, ConformanceReport, ExecutionTopologyFixture,
    ProfilePreflightFixture, RecordedHostCall, RecordingHostServices, RecordingOutcome,
    SyntheticProfile, assert_common_contract, poll_immediate,
};
use std::num::{NonZeroU32, NonZeroU64};
use swallowtail_core::{
    DriverRole, ExecutionLayer, HostServiceKind, OperationShape, OwnedRemoteResourceKind,
    ProviderRequestRef,
};
use swallowtail_runtime::{
    CallbackId, CallbackPayload, CallbackRequest, CallbackResponse, CallbackResult, CleanupOutcome,
    CredentialRef, Deadline, EndpointRef, MonotonicInstant, OperationContent, OperationPolicy,
    ProviderCancellationOutcome, ProviderRecoveryPolicy, ProviderRetentionPolicy,
    RemoteResourceDeletionOutcome, RequestId, RuntimeRunId, ScopeId, StreamReattachmentPolicy,
    StructuredRunRequest, TerminalOutcome, TerminalStatus,
};

pub(crate) fn run() -> ConformanceReport {
    let profile = SyntheticProfile::ProviderManagedRemoteHarness;
    let mut report = ConformanceReport::new(profile);
    assert_common_contract(profile, &mut report);

    let fixture = ProfilePreflightFixture::new(profile);
    let plan = fixture
        .preflight()
        .expect("managed-harness preflight succeeds");
    assert_eq!(
        plan.requirements().execution_layer(),
        ExecutionLayer::HarnessInteraction
    );
    assert_eq!(
        plan.requirements().operation_shape(),
        OperationShape::StructuredRun
    );
    assert!(plan.provider_agent().is_some());
    let required = fixture
        .driver()
        .required_host_services(DriverRole::StructuredRun)
        .collect::<Vec<_>>();
    for service in [
        HostServiceKind::Task,
        HostServiceKind::BlockingWork,
        HostServiceKind::Time,
        HostServiceKind::Network,
        HostServiceKind::Credential,
    ] {
        assert!(required.contains(&service));
    }
    assert!(!required.contains(&HostServiceKind::Process));
    assert!(!required.contains(&HostServiceKind::WorkingResource));

    let policy = OperationPolicy::offline()
        .with_provider_retention(ProviderRetentionPolicy::DurableAllowed)
        .with_provider_recovery(ProviderRecoveryPolicy::ManagedAllowed)
        .with_stream_reattachment(StreamReattachmentPolicy::Bounded(
            NonZeroU32::new(1).expect("one is nonzero"),
        ));
    let request = StructuredRunRequest::new(
        RequestId::new("managed-harness-run").expect("request id is valid"),
        OperationContent::new("provider-managed task").expect("content is valid"),
        policy,
    )
    .with_maximum_output_tokens(NonZeroU64::new(64).expect("limit is nonzero"));
    assert!(request.working_resource().is_none());
    assert_eq!(
        request.policy().provider_retention(),
        ProviderRetentionPolicy::DurableAllowed
    );
    assert_eq!(
        request.policy().provider_recovery(),
        ProviderRecoveryPolicy::ManagedAllowed
    );
    assert_eq!(
        request.policy().stream_reattachment(),
        StreamReattachmentPolicy::Bounded(NonZeroU32::new(1).expect("one is nonzero"))
    );

    assert_run_callback_correlation();
    assert_remote_deletion_truth();
    assert_topology_and_cleanup_order();

    report.record(ConformanceAssertion::ProviderManagedHarnessLifecycle);
    report.record(ConformanceAssertion::DurableRetentionExplicit);
    report.record(ConformanceAssertion::ManagedRecoveryExplicit);
    report.record(ConformanceAssertion::OwnedRemoteDeletionTruth);
    report.record(ConformanceAssertion::CallbackExchange);
    report.record(ConformanceAssertion::HostTopologyPreserved);
    report
}

fn assert_run_callback_correlation() {
    let run_id = RuntimeRunId::new("managed-harness-runtime-run").expect("run id is valid");
    let deadline = Deadline::at(MonotonicInstant::from_ticks(100));
    let request = CallbackRequest::run_tool_call(
        CallbackId::new("managed-harness-callback").expect("callback id is valid"),
        run_id.clone(),
        4,
        Some(deadline),
        "fixture_tool",
        CallbackPayload::new(b"{}".to_vec(), 16).expect("payload is bounded"),
    )
    .expect("callback request is valid")
    .with_provider_request_ref(
        ProviderRequestRef::new("provider-event-4").expect("provider reference is valid"),
    );
    let mut exchange = CallbackExchangeFixture::new(request);
    assert_eq!(exchange.request().run_id(), Some(&run_id));
    assert_eq!(exchange.deadline(), Some(deadline));
    assert!(!format!("{:?}", exchange.request()).contains("provider-event-4"));
    let response = CallbackResponse::for_run(
        exchange.request().callback_id().clone(),
        run_id,
        CallbackResult::Success(
            CallbackPayload::new(b"ok".to_vec(), 16).expect("payload is bounded"),
        ),
    );
    exchange
        .respond(response.clone())
        .expect("run callback response correlates");
    assert!(exchange.respond(response).is_err());
}

fn assert_remote_deletion_truth() {
    let complete = TerminalOutcome::new(TerminalStatus::Completed, CleanupOutcome::Clean)
        .with_remote_resource_deletion(
            OwnedRemoteResourceKind::Session,
            RemoteResourceDeletionOutcome::Confirmed,
        )
        .with_remote_resource_deletion(
            OwnedRemoteResourceKind::Environment,
            RemoteResourceDeletionOutcome::Confirmed,
        );
    assert_eq!(
        complete.remote_resource_deletion(OwnedRemoteResourceKind::Session),
        Some(RemoteResourceDeletionOutcome::Confirmed)
    );

    let ambiguous = TerminalOutcome::new(
        TerminalStatus::TimedOut,
        CleanupOutcome::Degraded(swallowtail_core::SafeDiagnostic::new(
            "fixture.remote_deletion_unconfirmed",
            "Remote deletion was not confirmed",
        )),
    )
    .with_provider_cancellation(ProviderCancellationOutcome::Unconfirmed)
    .with_remote_resource_deletion(
        OwnedRemoteResourceKind::Session,
        RemoteResourceDeletionOutcome::Unconfirmed,
    )
    .with_remote_resource_deletion(
        OwnedRemoteResourceKind::Environment,
        RemoteResourceDeletionOutcome::Unconfirmed,
    );
    assert!(matches!(ambiguous.cleanup(), CleanupOutcome::Degraded(_)));
    assert_eq!(
        ambiguous.provider_cancellation(),
        Some(ProviderCancellationOutcome::Unconfirmed)
    );
}

fn assert_topology_and_cleanup_order() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let fixture = ProfilePreflightFixture::for_host(
            SyntheticProfile::ProviderManagedRemoteHarness,
            topology.execution_host_id().clone(),
        );
        let plan = fixture.preflight().expect("topology preflight succeeds");
        let recording = RecordingHostServices::for_host(
            topology.execution_host_id().clone(),
            RecordingOutcome::Succeed,
        );
        let services = recording.services();
        services
            .require_execution_host(plan.execution_host_id())
            .expect("host authority is preserved");
        let scope = ScopeId::new("managed-harness-topology").expect("scope is valid");
        let audience = plan.endpoint_audience().clone();
        poll_immediate(
            services
                .network()
                .expect("network service exists")
                .authorize(
                    scope.clone(),
                    EndpointRef::new("managed-harness-endpoint").expect("endpoint is valid"),
                    audience.clone(),
                ),
        )
        .expect("endpoint is authorized");
        let credential = poll_immediate(
            services
                .credential()
                .expect("credential service exists")
                .acquire(
                    scope.clone(),
                    CredentialRef::new("managed-harness-credential")
                        .expect("credential reference is valid"),
                    audience,
                ),
        )
        .expect("credential is acquired");
        poll_immediate(
            services
                .blocking_work()
                .expect("blocking service exists")
                .run(scope.clone(), Box::new(|| Ok(()))),
        )
        .expect("transport worker finishes");
        let task = services
            .task()
            .expect("task service exists")
            .spawn(scope, Box::pin(async {}))
            .expect("child task starts");
        poll_immediate(task.join()).expect("child task joins");
        assert_eq!(
            poll_immediate(
                services
                    .credential()
                    .expect("credential service exists")
                    .release(credential)
            ),
            CleanupOutcome::Clean
        );
        let calls = recording.calls();
        assert!(
            position(&calls, RecordedHostCall::BlockingWork)
                < position(&calls, RecordedHostCall::CredentialRelease)
        );
        assert!(
            position(&calls, RecordedHostCall::TaskJoin)
                < position(&calls, RecordedHostCall::CredentialRelease)
        );
        assert_eq!(recording.count(RecordedHostCall::ProcessStart), 0);
        assert_eq!(recording.count(RecordedHostCall::WorkingResourceResolve), 0);
    }
}

fn position(calls: &[RecordedHostCall], expected: RecordedHostCall) -> usize {
    calls
        .iter()
        .position(|call| *call == expected)
        .expect("expected host call exists")
}
