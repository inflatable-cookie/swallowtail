mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use std::num::NonZeroU64;
use std::sync::Arc;
use support::{FixtureServer, StreamFixture, ThreadServices};
use swallowtail_adapter_anthropic::{AnthropicDirectDriver, anthropic_direct_descriptor};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, Capability, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism,
    CredentialState, DriverRole, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, ExecutionLayer, InstanceOwnership, InstancePolicyId,
    InstanceRevision, InstanceTargetRef, ModelId, ModelRoute, ModelRouteId, ModelRouteRevision,
    OperationRequirements, OperationShape, PreflightContext, ProtocolFacadeId, ProviderId,
    RuntimeReadiness, SupportAuthority, preflight,
};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    BlockingWorkService, CredentialRef, CredentialService, EndpointRef, HostServices,
    ModelCatalogDriver, ModelCatalogRequest, NetworkPolicyService, OperationContent,
    OperationPolicy, ProviderObservation, RequestId, ScopedTaskService, StructuredRunDriver,
    StructuredRunRequest, TerminalStatus, TimeService,
};

include!("direct_driver/fixture.rs");

#[test]
fn catalogue_and_one_attempt_message_stream_use_only_hosted_services() {
    let fixture = Fixture::new();
    let driver = AnthropicDirectDriver::new();
    let models = block_on(driver.list_models(
        fixture.plan(DriverRole::ModelCatalog),
        ModelCatalogRequest::new(RequestId::new("catalog").expect("request id is valid")),
        fixture.services(),
    ))
    .expect("catalogue succeeds");
    assert_eq!(models.len(), 3);
    assert!(models[2].metadata().token_limits().is_none());

    let request = StructuredRunRequest::new(
        RequestId::new("run").expect("request id is valid"),
        OperationContent::new("fixture prompt").expect("content is valid"),
        OperationPolicy::offline(),
    )
    .with_maximum_output_tokens(NonZeroU64::new(64).expect("limit is nonzero"));
    let mut run = block_on(driver.start_run(
        fixture.plan(DriverRole::StructuredRun),
        request,
        fixture.services(),
    ))
    .expect("run starts");
    let mut events = run.take_events().expect("events are available");
    let terminal = run.take_terminal_outcome().expect("terminal is available");
    let (events, outcome) = block_on(async {
        let mut observed = Vec::new();
        while let Some(event) = events.next().await {
            observed.push(event.expect("event succeeds"));
        }
        (observed, terminal.await)
    });
    assert_eq!(
        outcome.status(),
        &TerminalStatus::Completed,
        "requests: {:?}",
        fixture.server.requests()
    );
    assert_eq!(
        outcome.output().expect("output exists").as_str(),
        "Hello world"
    );
    assert!(events.iter().any(|event| matches!(
        event.kind(),
        swallowtail_runtime::RuntimeEventKind::ProviderObservation(
            ProviderObservation::RequestCorrelation(_)
        )
    )));
    assert!(events.iter().any(|event| matches!(
        event.kind(),
        swallowtail_runtime::RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(_))
    )));
    assert_eq!(fixture.server.inference_attempts(), 1);
    assert_eq!(fixture.credential_releases(), 2);
    assert!(matches!(
        block_on(run.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    ));
}

#[test]
fn missing_output_bound_fails_before_endpoint_or_credential_work() {
    let fixture = Fixture::new();
    let driver = AnthropicDirectDriver::new();
    let request = StructuredRunRequest::new(
        RequestId::new("missing-limit").expect("request id is valid"),
        OperationContent::new("fixture prompt").expect("content is valid"),
        OperationPolicy::offline(),
    );
    let error = block_on(driver.start_run(
        fixture.plan(DriverRole::StructuredRun),
        request,
        fixture.services(),
    ))
    .err()
    .expect("missing limit fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.anthropic.output_limit_missing"
    );
    assert!(fixture.server.requests().is_empty());
}

#[test]
fn midstream_error_fails_once_without_exposing_provider_payload() {
    let fixture = Fixture::with_stream(StreamFixture::MidstreamError);
    let (run, events, outcome) = complete_run(&fixture);
    assert!(matches!(
        outcome.status(),
        TerminalStatus::ProviderFailed(_)
    ));
    assert_eq!(fixture.server.inference_attempts(), 1);
    assert_eq!(fixture.credential_releases(), 1);
    assert!(!format!("{:?}", outcome.status()).contains("raw-provider-message"));
    assert!(events.iter().any(|event| matches!(
        event.kind(),
        swallowtail_runtime::RuntimeEventKind::OutputDelta
    )));
    assert!(matches!(
        block_on(run.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    ));
}

#[test]
fn unknown_top_level_event_completes_without_becoming_output() {
    let fixture = Fixture::with_stream(StreamFixture::Unknown);
    let (run, _events, outcome) = complete_run(&fixture);
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome.output().expect("output exists").as_str(),
        "known output"
    );
    assert_eq!(fixture.credential_releases(), 1);
    assert!(matches!(
        block_on(run.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    ));
}

#[test]
fn cancellation_closes_local_stream_and_remains_cancelled() {
    let fixture = Fixture::with_stream(StreamFixture::WaitForCancel);
    let request = StructuredRunRequest::new(
        RequestId::new("cancel-run").expect("request id is valid"),
        OperationContent::new("fixture prompt").expect("content is valid"),
        OperationPolicy::offline(),
    )
    .with_maximum_output_tokens(NonZeroU64::new(64).expect("limit is nonzero"));
    let mut run = block_on(AnthropicDirectDriver::new().start_run(
        fixture.plan(DriverRole::StructuredRun),
        request,
        fixture.services(),
    ))
    .expect("run starts");
    let _events = run.take_events().expect("events are available");
    let terminal = run.take_terminal_outcome().expect("terminal is available");
    for _ in 0..100 {
        if fixture.server.inference_attempts() == 1 {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    block_on(run.cancellation().request()).expect("cancellation is accepted");
    let outcome = block_on(terminal);
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(fixture.server.inference_attempts(), 1);
    assert_eq!(fixture.credential_releases(), 1);
    assert!(matches!(
        block_on(run.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    ));
}

fn complete_run(
    fixture: &Fixture,
) -> (
    Box<dyn swallowtail_runtime::RunHandle>,
    Vec<swallowtail_runtime::RuntimeEvent>,
    swallowtail_runtime::TerminalOutcome,
) {
    let request = StructuredRunRequest::new(
        RequestId::new("fixture-run").expect("request id is valid"),
        OperationContent::new("fixture prompt").expect("content is valid"),
        OperationPolicy::offline(),
    )
    .with_maximum_output_tokens(NonZeroU64::new(64).expect("limit is nonzero"));
    let mut run = block_on(AnthropicDirectDriver::new().start_run(
        fixture.plan(DriverRole::StructuredRun),
        request,
        fixture.services(),
    ))
    .expect("run starts");
    let mut stream = run.take_events().expect("events are available");
    let terminal = run.take_terminal_outcome().expect("terminal is available");
    let (events, outcome) = block_on(async {
        let mut events = Vec::new();
        while let Some(event) = stream.next().await {
            events.push(event.expect("event succeeds"));
        }
        (events, terminal.await)
    });
    (run, events, outcome)
}
