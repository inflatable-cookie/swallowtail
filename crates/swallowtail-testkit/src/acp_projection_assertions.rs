use crate::{
    CallbackExchangeFixture, ConformanceAssertion, ConformanceReport, SyntheticProfile,
    assert_common_contract,
};
use std::sync::atomic::{AtomicBool, Ordering};
use swallowtail_core::{
    CancellationScope, HarnessConfigurationPosture, HarnessIsolation, ProviderRequestRef,
};
use swallowtail_runtime::{
    BoxFuture, CallbackId, CallbackPayload, CallbackRequest, CallbackResponse, CallbackResult,
    CancellationAcknowledgement, CancellationControl, CleanupOutcome, OperationContent,
    OperationPolicy, ProviderRetentionPolicy, RequestId, RuntimeFailure, RuntimeRunId,
    StructuredRunRequest, TerminalOutcome, TerminalStatus, WorkingResourceRef,
};

pub(crate) fn run() -> ConformanceReport {
    let profile = SyntheticProfile::LongLivedAcpHarness;
    let mut report = ConformanceReport::new(profile);
    assert_common_contract(profile, &mut report);

    let policy = OperationPolicy::offline()
        .with_provider_retention(ProviderRetentionPolicy::DurableAllowed)
        .with_harness_isolation(HarnessIsolation::AmbientHost)
        .with_harness_configuration_posture(HarnessConfigurationPosture::Ambient);
    let request = StructuredRunRequest::new(
        RequestId::new("acp-projection-run").expect("request id is valid"),
        OperationContent::new("private ACP prompt").expect("content is valid"),
        policy,
    )
    .with_working_resource(
        WorkingResourceRef::new("acp-projection-workspace").expect("resource is valid"),
    );
    assert_eq!(
        request.policy().provider_retention(),
        ProviderRetentionPolicy::DurableAllowed
    );
    assert_eq!(
        request.policy().harness_isolation(),
        Some(HarnessIsolation::AmbientHost)
    );
    assert_eq!(
        request.policy().harness_configuration_posture(),
        Some(HarnessConfigurationPosture::Ambient)
    );

    let run_id = RuntimeRunId::new("acp-projection-runtime-run").expect("run id is valid");
    let callback = CallbackRequest::run_tool_call(
        CallbackId::new("acp-projection-callback").expect("callback id is valid"),
        run_id.clone(),
        3,
        None,
        "fixture_tool",
        CallbackPayload::new(b"{}".to_vec(), 16).expect("payload is bounded"),
    )
    .expect("callback is valid")
    .with_provider_request_ref(
        ProviderRequestRef::new("acp-projection-provider-request")
            .expect("provider reference is valid"),
    );
    let mut exchange = CallbackExchangeFixture::new(callback);
    let response = CallbackResponse::for_run(
        exchange.request().callback_id().clone(),
        run_id,
        CallbackResult::Success(
            CallbackPayload::new(b"ok".to_vec(), 16).expect("payload is bounded"),
        ),
    );
    exchange
        .respond(response.clone())
        .expect("correlated callback succeeds once");
    assert!(exchange.respond(response).is_err());

    let cancellation = ProjectionCancellation::default();
    assert_eq!(cancellation.scope(), CancellationScope::StructuredRun);
    assert_eq!(
        crate::poll_immediate(cancellation.request()).expect("cancellation succeeds"),
        CancellationAcknowledgement::Requested
    );
    assert_eq!(
        crate::poll_immediate(cancellation.request()).expect("repeat cancellation succeeds"),
        CancellationAcknowledgement::AlreadyRequested
    );

    let terminal = TerminalOutcome::new(TerminalStatus::Completed, CleanupOutcome::Clean);
    assert_eq!(terminal.remote_resource_deletions().count(), 0);

    report.record(ConformanceAssertion::SessionLifecycle);
    report.record(ConformanceAssertion::CallbackExchange);
    report.record(ConformanceAssertion::DurableRetentionExplicit);
    report.record(ConformanceAssertion::NoTranscriptDeletionClaim);
    report
}

#[derive(Default)]
struct ProjectionCancellation {
    requested: AtomicBool,
}

impl CancellationControl for ProjectionCancellation {
    fn scope(&self) -> CancellationScope {
        CancellationScope::StructuredRun
    }

    fn request(&self) -> BoxFuture<'_, Result<CancellationAcknowledgement, RuntimeFailure>> {
        let prior = self.requested.swap(true, Ordering::SeqCst);
        Box::pin(async move {
            Ok(if prior {
                CancellationAcknowledgement::AlreadyRequested
            } else {
                CancellationAcknowledgement::Requested
            })
        })
    }
}
