mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use std::num::NonZeroU32;
use std::sync::Arc;
use support::{ManagedFixtureServer, ManagedStreamFixture, ThreadServices};
use swallowtail_adapter_anthropic::{
    ANTHROPIC_MANAGED_ACCESS_PROFILE_ID, ANTHROPIC_MANAGED_FACADE_REVISION,
    AnthropicManagedAgentRunInput, AnthropicManagedModelSelection,
    AnthropicManagedPreparationInput, anthropic_managed_agent_descriptor,
    anthropic_managed_requirements, prepare_anthropic_managed_agent,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, CapabilityProfile,
    CapabilityRequirement, ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism,
    CredentialState, DriverRole, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, ExecutionLayer, InstanceOwnership, InstancePolicyId,
    InstanceRevision, InstanceTargetRef, InterfaceCompatibilityAssessment, ModelId, ModelRoute,
    ModelRouteId, ModelRouteRevision, ObservableActivityAvailability, OperationRequirements,
    OperationShape, OwnedRemoteResourceKind, PreflightContext, ProtocolFacadeId,
    ProviderAgentBinding, ProviderAgentId, ProviderAgentVersion, ProviderId, RuntimeReadiness,
    SupportAuthority, preflight,
};
use swallowtail_host_local::{LocalProcessHost, LocalProcessLimits};
use swallowtail_runtime::{
    BlockingWorkService, CallbackPayload, CallbackResponse, CallbackResult, CleanupOutcome,
    CredentialRef, CredentialService, Deadline, EndpointRef, HostServices, MonotonicInstant,
    NetworkPolicyService, OperationContent, OperationPolicy, PreparedAccessEvidence,
    ProviderObservation, ProviderRecoveryPolicy, ProviderRetentionPolicy,
    RemoteResourceDeletionOutcome, RequestId, SchemaDocument, ScopedTaskService,
    StreamReattachmentPolicy, StructuredRunRequest, TerminalStatus, TimeService, ToolDeclaration,
};
use swallowtail_testkit::ExecutionTopologyFixture;

include!("managed_driver/fixture.rs");
include!("managed_driver/prepared_fixture.rs");

#[test]
fn prepared_managed_run_preserves_resources_recovery_and_cleanup_on_both_hosts() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let fixture = Fixture::for_topology(topology.clone());
        let prepared =
            prepare_anthropic_managed_agent(fixture.preparation_input(), &fixture.services())
                .expect("managed integration prepares");
        let run = prepared
            .prepare_managed_run(fixture.prepared_run_input("prepared-managed", []))
            .expect("managed run prepares");

        assert_eq!(
            run.plan().protocol_facade_id().as_str(),
            ANTHROPIC_MANAGED_FACADE_REVISION
        );
        assert_eq!(
            run.plan().requirements().execution_layer(),
            ExecutionLayer::HarnessInteraction
        );
        assert_ne!(
            run.plan().driver_identity().id(),
            swallowtail_adapter_anthropic::anthropic_direct_descriptor()
                .identity()
                .id()
        );
        assert_eq!(
            run.plan()
                .provider_agent()
                .expect("agent is bound")
                .version()
                .as_str(),
            "7"
        );
        assert_eq!(
            run.request().policy().provider_retention(),
            ProviderRetentionPolicy::DurableAllowed
        );
        assert_eq!(
            run.request().policy().provider_recovery(),
            ProviderRecoveryPolicy::ManagedAllowed
        );
        assert_eq!(
            run.request().policy().stream_reattachment(),
            StreamReattachmentPolicy::Bounded(NonZeroU32::new(1).expect("one is non-zero"))
        );
        let compatibility: Vec<_> = run
            .evidence()
            .operation()
            .interface_compatibility()
            .collect();
        assert!(matches!(
            compatibility[0].assessment(),
            InterfaceCompatibilityAssessment::Qualified(_)
        ));
        assert_eq!(
            run.evidence()
                .operation()
                .observable_activity()
                .availability(),
            ObservableActivityAvailability::Available
        );

        let (run, events, outcome) = complete_prepared(run.start_run(fixture.services()));
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert!(events.iter().any(|event| matches!(
            event.kind(),
            swallowtail_runtime::RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(
                _
            ))
        )));
        assert_eq!(
            outcome.remote_resource_deletion(OwnedRemoteResourceKind::Session),
            Some(RemoteResourceDeletionOutcome::Confirmed)
        );
        assert_eq!(
            outcome.remote_resource_deletion(OwnedRemoteResourceKind::Environment),
            Some(RemoteResourceDeletionOutcome::Confirmed)
        );
        let state = fixture.server.state();
        assert_eq!(state.session_creations, 1);
        assert!(state.session_deleted && state.environment_deleted);
        assert_delete_order(&fixture);
        assert_eq!(fixture.credential_releases(), 1);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    }
}

#[test]
fn prepared_managed_recovery_and_callback_retain_authoritative_correlation() {
    let recovery = Fixture::with_stream(ManagedStreamFixture::DisconnectThenSuccess);
    let prepared =
        prepare_anthropic_managed_agent(recovery.preparation_input(), &recovery.services())
            .expect("managed integration prepares");
    let run = prepared
        .prepare_managed_run(recovery.prepared_run_input("prepared-recovery", []))
        .expect("managed run prepares");
    let (run, _events, outcome) = complete_prepared(run.start_run(recovery.services()));
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(recovery.server.state().session_creations, 1);
    assert_eq!(recovery.server.state().stream_attachments, 2);
    assert_eq!(
        recovery
            .server
            .requests()
            .iter()
            .filter(|request| request.target.contains("events?limit=1000"))
            .count(),
        1
    );
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);

    let callback = Fixture::with_stream(ManagedStreamFixture::RequiresActionThenSuccess);
    let prepared =
        prepare_anthropic_managed_agent(callback.preparation_input(), &callback.services())
            .expect("managed integration prepares");
    let run = prepared
        .prepare_managed_run(callback.prepared_run_input("prepared-callback", [fixture_tool()]))
        .expect("managed run prepares");
    let mut handle = block_on(run.start_run(callback.services())).expect("run starts");
    let mut callbacks = handle.take_callbacks().expect("callbacks exist");
    let mut requests = callbacks.take_requests().expect("callback requests exist");
    let request = block_on(requests.next())
        .expect("callback arrives")
        .expect("callback is valid");
    let response = CallbackResponse::for_run(
        request.callback_id().clone(),
        handle.run_id().clone(),
        CallbackResult::Success(
            CallbackPayload::new(b"fixture-value".to_vec(), 128).expect("payload is bounded"),
        ),
    );
    block_on(callbacks.responder().respond(response)).expect("response is correlated");
    let terminal = handle.take_terminal_outcome().expect("terminal exists");
    let outcome = block_on(terminal);
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(callback.server.state().tool_results, 1);
    assert_eq!(callback.server.state().session_creations, 1);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
}

#[test]
fn prepared_interrupt_deletes_owned_resources_before_credential_release() {
    let fixture = Fixture::with_stream(ManagedStreamFixture::WaitForInterrupt);
    let prepared =
        prepare_anthropic_managed_agent(fixture.preparation_input(), &fixture.services())
            .expect("managed integration prepares");
    let run = prepared
        .prepare_managed_run(fixture.prepared_run_input("prepared-interrupt", []))
        .expect("managed run prepares");
    let mut handle = block_on(run.start_run(fixture.services())).expect("run starts");
    let terminal = handle.take_terminal_outcome().expect("terminal exists");
    for _ in 0..200 {
        if fixture.server.state().stream_attachments == 1 {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    block_on(handle.cancellation().request()).expect("cancellation is accepted");
    let outcome = block_on(terminal);
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert_eq!(fixture.server.state().interrupts, 1);
    assert_delete_order(&fixture);
    assert_eq!(fixture.credential_releases(), 1);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
}

include!("managed_driver/prepared_policy_tests.rs");

fn reattachment() -> StreamReattachmentPolicy {
    StreamReattachmentPolicy::Bounded(NonZeroU32::new(1).expect("one is non-zero"))
}

fn assert_delete_order(fixture: &Fixture) {
    let targets: Vec<_> = fixture
        .server
        .requests()
        .into_iter()
        .map(|request| format!("{} {}", request.method, request.target))
        .collect();
    let session = targets
        .iter()
        .position(|target| target == "DELETE /v1/sessions/session_fixture")
        .expect("session deletion exists");
    let environment = targets
        .iter()
        .position(|target| target == "DELETE /v1/environments/env_fixture")
        .expect("environment deletion exists");
    assert!(session < environment);
}

fn complete_prepared(
    run: swallowtail_runtime::BoxFuture<
        'static,
        Result<Box<dyn swallowtail_runtime::RunHandle>, swallowtail_runtime::RuntimeFailure>,
    >,
) -> (
    Box<dyn swallowtail_runtime::RunHandle>,
    Vec<swallowtail_runtime::RuntimeEvent>,
    swallowtail_runtime::TerminalOutcome,
) {
    let mut run = block_on(run).expect("run starts");
    let mut stream = run.take_events().expect("events exist");
    let terminal = run.take_terminal_outcome().expect("terminal exists");
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
