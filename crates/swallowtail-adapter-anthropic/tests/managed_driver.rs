mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use std::num::NonZeroU32;
use std::sync::Arc;
use support::{ManagedFixtureServer, ManagedStreamFixture, ThreadServices};
use swallowtail_adapter_anthropic::{
    AnthropicManagedAgentDriver, anthropic_managed_agent_descriptor,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, Capability,
    CapabilityConstraint, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, CredentialMechanism, CredentialState, DriverRole, EndpointAudience,
    EndpointAuthorization, EntitlementMetering, EntitlementState, ExecutionHostId, ExecutionLayer,
    InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef, ModelId, ModelRoute,
    ModelRouteId, ModelRouteRevision, OperationRequirements, OperationShape,
    OwnedRemoteResourceKind, PreflightContext, ProtocolFacadeId, ProviderAgentBinding,
    ProviderAgentId, ProviderAgentVersion, ProviderId, RuntimeReadiness, SupportAuthority,
    preflight,
};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    BlockingWorkService, CallbackPayload, CallbackResponse, CallbackResult, CleanupOutcome,
    CredentialRef, CredentialService, Deadline, EndpointRef, HostServices, MonotonicInstant,
    NetworkPolicyService, OperationContent, OperationPolicy, ProviderObservation,
    ProviderRecoveryPolicy, ProviderRetentionPolicy, RemoteResourceDeletionOutcome, RequestId,
    SchemaDocument, ScopedTaskService, StreamReattachmentPolicy, StructuredRunDriver,
    StructuredRunRequest, TerminalStatus, TimeService, ToolDeclaration,
};
use swallowtail_testkit::{
    ConformanceAssertion, ExecutionTopologyFixture, run_provider_managed_harness_profile,
};

include!("managed_driver/fixture.rs");

#[test]
fn exact_managed_run_streams_usage_and_deletes_owned_resources_in_order() {
    let fixture = Fixture::new();
    let (run, events, outcome) = complete(&fixture, fixture.request("managed-success"));

    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome.output().expect("output exists").as_str(),
        "Fixture complete."
    );
    assert_eq!(
        outcome.remote_resource_deletion(OwnedRemoteResourceKind::Session),
        Some(RemoteResourceDeletionOutcome::Confirmed)
    );
    assert_eq!(
        outcome.remote_resource_deletion(OwnedRemoteResourceKind::Environment),
        Some(RemoteResourceDeletionOutcome::Confirmed)
    );
    assert!(events.iter().any(|event| matches!(
        event.kind(),
        swallowtail_runtime::RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(_))
    )));
    let state = fixture.server.state();
    assert_eq!(state.session_creations, 1);
    assert_eq!(state.stream_attachments, 1);
    assert!(state.session_deleted && state.environment_deleted);
    let targets = fixture
        .server
        .requests()
        .into_iter()
        .map(|request| format!("{} {}", request.method, request.target))
        .collect::<Vec<_>>();
    let session_delete = targets
        .iter()
        .position(|target| target == "DELETE /v1/sessions/session_fixture")
        .expect("session deletion exists");
    let environment_delete = targets
        .iter()
        .position(|target| target == "DELETE /v1/environments/env_fixture")
        .expect("environment deletion exists");
    assert!(session_delete < environment_delete);
    assert_eq!(fixture.credential_releases(), 1);
    assert!(matches!(block_on(run.close()), CleanupOutcome::Clean));
}

include!("managed_driver/topology_tests.rs");

#[test]
fn custom_tool_callback_is_relayed_without_adapter_execution() {
    let fixture = Fixture::with_stream(ManagedStreamFixture::RequiresActionThenSuccess);
    let mut run = block_on(
        AnthropicManagedAgentDriver::new().start_run(
            fixture.plan(),
            fixture
                .request("managed-callback")
                .with_tools([fixture_tool()]),
            fixture.services(),
        ),
    )
    .expect("run starts");
    let mut callbacks = run.take_callbacks().expect("callbacks are available");
    let mut requests = callbacks
        .take_requests()
        .expect("callback requests are available");
    let request = block_on(requests.next())
        .expect("callback request arrives")
        .expect("callback request is valid");
    assert_eq!(request.run_id(), Some(run.run_id()));
    let response = CallbackResponse::for_run(
        request.callback_id().clone(),
        run.run_id().clone(),
        CallbackResult::Success(
            CallbackPayload::new(b"fixture-value".to_vec(), 128).expect("payload is bounded"),
        ),
    );
    let responder = callbacks.responder();
    let mismatched = CallbackResponse::for_run(
        request.callback_id().clone(),
        swallowtail_runtime::RuntimeRunId::new("another-managed-run").expect("run id is valid"),
        CallbackResult::Success(
            CallbackPayload::new(b"wrong".to_vec(), 128).expect("payload is bounded"),
        ),
    );
    assert!(block_on(responder.respond(mismatched)).is_err());
    block_on(responder.respond(response.clone())).expect("callback response is accepted");
    let terminal = run.take_terminal_outcome().expect("terminal is available");
    let outcome = block_on(terminal);
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(fixture.server.state().tool_results, 1);
    assert_eq!(fixture.server.state().stream_attachments, 2);
    assert!(block_on(responder.respond(response)).is_err());
    assert!(matches!(block_on(run.close()), CleanupOutcome::Clean));
}

include!("managed_driver/failure_tests.rs");

#[test]
fn disconnect_reconciles_history_and_reattaches_without_replacing_the_session() {
    let fixture = Fixture::with_stream(ManagedStreamFixture::DisconnectThenSuccess);
    let (run, _events, outcome) = complete(&fixture, fixture.request("managed-reconnect"));

    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome.output().expect("output exists").as_str(),
        "Fixture complete."
    );
    let state = fixture.server.state();
    assert_eq!(state.session_creations, 1);
    assert_eq!(state.stream_attachments, 2);
    assert_eq!(
        fixture
            .server
            .requests()
            .iter()
            .filter(|request| request.target.contains("events?limit=1000"))
            .count(),
        1
    );
    assert!(matches!(block_on(run.close()), CleanupOutcome::Clean));
}

#[test]
fn rescheduling_is_observed_inside_one_provider_session_without_swallowtail_retry() {
    let fixture = Fixture::with_stream(ManagedStreamFixture::Rescheduling);
    let (run, _events, outcome) = complete(&fixture, fixture.request("managed-reschedule"));

    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome.output().expect("output exists").as_str(),
        "Recovered once."
    );
    assert_eq!(fixture.server.state().session_creations, 1);
    assert_eq!(fixture.server.state().stream_attachments, 1);
    assert!(matches!(block_on(run.close()), CleanupOutcome::Clean));
}

#[test]
fn cancellation_interrupts_then_deletes_before_releasing_the_credential() {
    let fixture = Fixture::with_stream(ManagedStreamFixture::WaitForInterrupt);
    let mut run = block_on(AnthropicManagedAgentDriver::new().start_run(
        fixture.plan(),
        fixture.request("managed-cancel"),
        fixture.services(),
    ))
    .expect("run starts");
    let terminal = run.take_terminal_outcome().expect("terminal is available");
    for _ in 0..200 {
        if fixture.server.state().stream_attachments == 1 {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    block_on(run.cancellation().request()).expect("cancellation is accepted");
    let outcome = block_on(terminal);
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(fixture.server.state().interrupts, 1);
    assert!(fixture.server.state().session_deleted);
    assert!(fixture.server.state().environment_deleted);
    assert_eq!(fixture.credential_releases(), 1);
    assert!(matches!(block_on(run.close()), CleanupOutcome::Clean));
}

#[test]
fn weaker_operation_policy_fails_before_network_or_credential_effects() {
    let fixture = Fixture::new();
    let request = StructuredRunRequest::new(
        RequestId::new("managed-policy-reject").expect("request id is valid"),
        OperationContent::new("Return the fixture summary.").expect("content is valid"),
        OperationPolicy::offline(),
    )
    .with_deadline(fixture.deadline());
    let error = block_on(AnthropicManagedAgentDriver::new().start_run(
        fixture.plan(),
        request,
        fixture.services(),
    ))
    .err()
    .expect("weaker policy fails");

    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.anthropic.unsupported_input"
    );
    assert!(fixture.server.requests().is_empty());
    assert_eq!(fixture.credential_releases(), 0);
}

fn complete(
    fixture: &Fixture,
    request: StructuredRunRequest,
) -> (
    Box<dyn swallowtail_runtime::RunHandle>,
    Vec<swallowtail_runtime::RuntimeEvent>,
    swallowtail_runtime::TerminalOutcome,
) {
    let mut run = block_on(AnthropicManagedAgentDriver::new().start_run(
        fixture.plan(),
        request,
        fixture.services(),
    ))
    .expect("run starts");
    let mut stream = run.take_events().expect("events are available");
    let terminal = run.take_terminal_outcome().expect("terminal is available");
    let (events, outcome) = block_on(async {
        let mut events = Vec::new();
        while let Some(event) = stream.next().await {
            events.push(event.expect("event is valid"));
        }
        (events, terminal.await)
    });
    (run, events, outcome)
}

fn fixture_tool() -> ToolDeclaration {
    ToolDeclaration::new(
        "lookup_fixture",
        SchemaDocument::inline(
            br#"{"type":"object","properties":{"key":{"type":"string"}},"required":["key"]}"#
                .to_vec(),
            1024,
        )
        .expect("schema is bounded"),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .expect("tool is valid")
    .with_description(
        OperationContent::new("Return one deterministic fixture value.")
            .expect("description is valid"),
    )
}
