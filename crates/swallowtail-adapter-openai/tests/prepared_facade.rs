use crate::{fixture, server, services};

use fixture::Fixture;
use futures_executor::block_on;
use futures_util::StreamExt;
use serde_json::Value;
use server::ServerMode;
use services::TimeMode;
use std::num::{NonZeroU32, NonZeroU64};
use swallowtail_adapter_openai::{
    OPENAI_BACKGROUND_FACADE_REVISION, OPENAI_BACKGROUND_MODEL_ID,
    OPENAI_BACKGROUND_MODEL_ROUTE_ID, OpenAiBackgroundModelSelection,
    OpenAiBackgroundRunProfileInput, OpenAiBackgroundServiceTier, prepare_openai_background,
};
use swallowtail_core::{
    Capability, CapabilityConstraint, ModelId, ModelRouteId, ModelRouteRevision,
    OwnedRemoteResourceKind, ReasoningMode, StructuredOutputEnforcement,
};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, MonotonicInstant, OperationContent, ProviderCancellationOutcome,
    ProviderExecutionPolicy, ProviderObservation, ProviderRetentionPolicy,
    RemoteResourceDeletionOutcome, RequestId, SchemaDocument, StreamReattachmentPolicy,
    StructuredOutputDescriptor, TerminalStatus,
};
use swallowtail_testkit::assert_observable_activity_trace;

#[test]
fn prepared_background_run_preserves_one_attempt_and_one_reattachment_on_both_hosts() {
    for host in ["host.local", "host.remote-authoritative"] {
        let fixture = Fixture::new(ServerMode::Success, host, TimeMode::Pending);
        let prepared = prepare_openai_background(fixture.preparation_input(), &fixture.services())
            .expect("OpenAI background integration prepares");
        let run = prepared
            .prepare_background_run(profile("prepared-success"))
            .expect("background run prepares");

        assert!(run.request().policy().reasoning_mode().is_none());
        assert_eq!(run.evidence().service_tier(), None);
        assert_eq!(
            run.plan().protocol_facade_id().as_str(),
            OPENAI_BACKGROUND_FACADE_REVISION
        );
        assert_eq!(
            run.request().policy().provider_execution(),
            ProviderExecutionPolicy::Background
        );
        assert_eq!(
            run.request().policy().provider_retention(),
            ProviderRetentionPolicy::TemporaryAllowed
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
        assert_eq!(compatibility.len(), 1);
        assert!(matches!(
            compatibility[0].assessment(),
            swallowtail_core::InterfaceCompatibilityAssessment::Qualified(_)
        ));

        let activity_profile = run.evidence().observable_activity().clone();
        let (run, events, outcome) = complete(run.start_run(fixture.services()));
        assert_observable_activity_trace(&activity_profile, &events);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(
            outcome.remote_resource_deletion(OwnedRemoteResourceKind::Response),
            Some(RemoteResourceDeletionOutcome::Confirmed)
        );
        assert_eq!(outcome.output().expect("output exists").as_str(), "Hello");
        assert!(events.iter().any(|event| matches!(
            event.kind(),
            swallowtail_runtime::RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(
                _
            ))
        )));
        assert!(events.iter().any(|event| matches!(
            event.kind(),
            swallowtail_runtime::RuntimeEventKind::ProviderObservation(
                ProviderObservation::RateLimit(_)
            )
        )));
        let targets: Vec<_> = fixture
            .server
            .requests()
            .into_iter()
            .map(|request| request.target)
            .collect();
        assert_eq!(
            targets,
            [
                "/v1/responses",
                "/v1/responses/resp_fixture_123?stream=true&starting_after=3",
                "/v1/responses/resp_fixture_123",
            ]
        );
        assert_eq!(fixture.server.inference_attempts(), 1);
        assert_eq!(fixture.releases(), 1);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
    }
}

#[test]
fn prepared_cancel_and_deadline_preserve_remote_truth_and_cleanup() {
    let raced = Fixture::new(ServerMode::CancelRace, "host.local", TimeMode::Pending);
    let prepared = prepare_openai_background(raced.preparation_input(), &raced.services())
        .expect("OpenAI background integration prepares");
    let run = prepared
        .prepare_background_run(profile("prepared-cancel-race"))
        .expect("background run prepares");
    let mut handle = block_on(run.start_run(raced.services())).expect("run starts");
    assert_eq!(
        block_on(handle.cancellation().request()).expect("cancel request succeeds"),
        swallowtail_runtime::CancellationAcknowledgement::Requested
    );
    let outcome = consume(&mut handle);
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome.provider_cancellation(),
        Some(ProviderCancellationOutcome::RacedWithCompletion)
    );
    assert_eq!(raced.server.inference_attempts(), 1);
    assert_eq!(raced.releases(), 1);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);

    let deadline = Fixture::new(ServerMode::HoldForCancel, "host.remote", TimeMode::Delayed);
    let prepared = prepare_openai_background(deadline.preparation_input(), &deadline.services())
        .expect("OpenAI background integration prepares");
    let run = prepared
        .prepare_background_run(profile("prepared-deadline"))
        .expect("background run prepares");
    let (handle, _events, outcome) = complete(run.start_run(deadline.services()));
    assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
    assert_eq!(
        outcome.provider_cancellation(),
        Some(ProviderCancellationOutcome::Confirmed)
    );
    assert_eq!(deadline.server.inference_attempts(), 1);
    assert_eq!(deadline.releases(), 1);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
}

#[test]
fn prepared_policy_and_route_drift_fail_before_endpoint_or_credential_effects() {
    let fixture = Fixture::new(ServerMode::Success, "host.local", TimeMode::Pending);
    let prepared = prepare_openai_background(fixture.preparation_input(), &fixture.services())
        .expect("OpenAI background integration prepares");

    for input in [
        explicit(
            "attached",
            selection(OPENAI_BACKGROUND_MODEL_ROUTE_ID, OPENAI_BACKGROUND_MODEL_ID),
            ProviderExecutionPolicy::Attached,
            ProviderRetentionPolicy::TemporaryAllowed,
            reattachment(),
        ),
        explicit(
            "retention",
            selection(OPENAI_BACKGROUND_MODEL_ROUTE_ID, OPENAI_BACKGROUND_MODEL_ID),
            ProviderExecutionPolicy::Background,
            ProviderRetentionPolicy::Prohibited,
            reattachment(),
        ),
        explicit(
            "reattach",
            selection(OPENAI_BACKGROUND_MODEL_ROUTE_ID, OPENAI_BACKGROUND_MODEL_ID),
            ProviderExecutionPolicy::Background,
            ProviderRetentionPolicy::TemporaryAllowed,
            StreamReattachmentPolicy::Disabled,
        ),
        explicit(
            "route",
            selection("openai.public.other", OPENAI_BACKGROUND_MODEL_ID),
            ProviderExecutionPolicy::Background,
            ProviderRetentionPolicy::TemporaryAllowed,
            reattachment(),
        ),
        explicit(
            "model",
            selection(OPENAI_BACKGROUND_MODEL_ROUTE_ID, "gpt-other"),
            ProviderExecutionPolicy::Background,
            ProviderRetentionPolicy::TemporaryAllowed,
            reattachment(),
        ),
    ] {
        assert!(prepared.prepare_background_run(input).is_err());
    }

    assert!(fixture.server.requests().is_empty());
    assert_eq!(fixture.releases(), 0);
}

#[test]
fn prepared_background_reasoning_values_agree_across_plan_evidence_policy_driver_and_wire() {
    for (index, value) in ["none", "low", "medium", "high", "xhigh", "max"]
        .iter()
        .copied()
        .enumerate()
    {
        let fixture = Fixture::new(
            ServerMode::ReasoningVocabulary,
            format!("host.reasoning.{index}").as_str(),
            TimeMode::Pending,
        );
        let prepared = prepare_openai_background(fixture.preparation_input(), &fixture.services())
            .expect("OpenAI background integration prepares");
        let reasoning = ReasoningMode::new(value).expect("reasoning is valid");
        let run = prepared
            .prepare_background_run(
                profile(&format!("prepared-reasoning-{index}"))
                    .with_reasoning_mode(reasoning.clone()),
            )
            .expect("supported reasoning prepares");

        assert_eq!(run.request().policy().reasoning_mode(), Some(&reasoning));
        assert!(run.plan().requirements().capabilities().any(|requirement| {
            requirement.capability() == Capability::ReasoningSelection
                && requirement
                    .constraints()
                    .eq([&CapabilityConstraint::ReasoningMode(reasoning.clone())])
        }));
        let compatibility: Vec<_> = run
            .evidence()
            .operation()
            .interface_compatibility()
            .collect();
        assert_eq!(compatibility.len(), 1);
        assert_eq!(
            compatibility[0].binding().version().as_str(),
            OPENAI_BACKGROUND_FACADE_REVISION
        );
        assert_eq!(
            compatibility[0]
                .assessment()
                .behavior_revision()
                .expect("qualified behavior revision")
                .as_str(),
            "openai.responses-background-v3"
        );

        let (handle, _events, outcome) = complete(run.start_run(fixture.services()));
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        let request: Value = serde_json::from_slice(&fixture.server.requests()[0].body)
            .expect("create request is JSON");
        assert_eq!(request["reasoning"]["effort"], value);
        assert_eq!(fixture.server.inference_attempts(), 1);
        assert_eq!(fixture.releases(), 1);
        assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    }
}

#[test]
fn prepared_background_generation_controls_are_exact_and_fail_before_effects() {
    let fixture = Fixture::new(ServerMode::Success, "host.local", TimeMode::Pending);
    let prepared = prepare_openai_background(fixture.preparation_input(), &fixture.services())
        .expect("OpenAI background integration prepares");
    let reasoning = ReasoningMode::new("high").expect("reasoning is valid");
    let run = prepared
        .prepare_background_run(
            profile("prepared-controls")
                .with_reasoning_mode(reasoning.clone())
                .with_structured_output(schema()),
        )
        .expect("generation controls prepare");
    assert_eq!(run.request().policy().reasoning_mode(), Some(&reasoning));
    assert!(run.request().structured_output().is_some());
    assert!(run.plan().requirements().capabilities().any(|requirement| {
        requirement.capability() == Capability::OutputTokenLimit
            && requirement
                .constraints()
                .eq([&CapabilityConstraint::OutputTokenMaximum(64)])
    }));
    assert!(run.plan().requirements().capabilities().any(|requirement| {
        requirement.capability() == Capability::ReasoningSelection
            && requirement
                .constraints()
                .eq([&CapabilityConstraint::ReasoningMode(reasoning.clone())])
    }));
    assert!(run.plan().requirements().capabilities().any(|requirement| {
        requirement.capability() == Capability::StructuredOutput
            && requirement.constraints().any(|constraint| {
                constraint
                    == &CapabilityConstraint::StructuredOutputEnforcement(
                        StructuredOutputEnforcement::ProviderNative,
                    )
            })
    }));

    for (index, value) in ["minimal", "ultra"].iter().copied().enumerate() {
        let error = prepared
            .prepare_background_run(
                profile(&format!("prepared-controls-unsupported-{index}")).with_reasoning_mode(
                    ReasoningMode::new(value).expect("mode is syntactically valid"),
                ),
            )
            .expect_err("unsupported reasoning fails");
        assert_eq!(
            error.diagnostic().safe().code(),
            "swallowtail.openai.preparation.reasoning_unsupported"
        );
    }
    assert!(fixture.server.requests().is_empty());
    assert_eq!(fixture.releases(), 0);
}

#[test]
fn prepared_background_default_service_tier_agrees_across_evidence_driver_and_wire() {
    let selected = OpenAiBackgroundServiceTier::standard();
    let fixture = Fixture::new(ServerMode::Success, "host.local", TimeMode::Pending);
    let prepared = prepare_openai_background(fixture.preparation_input(), &fixture.services())
        .expect("OpenAI background integration prepares");
    let run = prepared
        .prepare_background_run(profile("prepared-service-tier").with_service_tier(selected))
        .expect("standard service tier prepares");
    assert_eq!(run.evidence().service_tier(), Some(selected));
    assert_eq!(
        run.plan().protocol_facade_id().as_str(),
        OPENAI_BACKGROUND_FACADE_REVISION
    );
    let compatibility: Vec<_> = run
        .evidence()
        .operation()
        .interface_compatibility()
        .collect();
    assert_eq!(compatibility.len(), 1);
    assert_eq!(
        compatibility[0]
            .assessment()
            .behavior_revision()
            .expect("qualified behavior revision")
            .as_str(),
        "openai.responses-background-v3"
    );

    let (handle, _events, outcome) = complete(run.start_run(fixture.services()));
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    let request: Value =
        serde_json::from_slice(&fixture.server.requests()[0].body).expect("create request is JSON");
    assert_eq!(request["service_tier"], "default");
    assert!(!request["service_tier"].is_null());
    assert_eq!(
        fixture.server.requests()[0].target.as_str(),
        "/v1/responses"
    );
    assert_eq!(fixture.server.inference_attempts(), 1);
    assert_eq!(fixture.releases(), 1);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
}

#[test]
fn prepared_background_omitted_service_tier_keeps_prior_create_bytes() {
    let fixture = Fixture::new(ServerMode::Success, "host.local", TimeMode::Pending);
    let prepared = prepare_openai_background(fixture.preparation_input(), &fixture.services())
        .expect("OpenAI background integration prepares");
    let run = prepared
        .prepare_background_run(profile("prepared-omitted-service-tier"))
        .expect("omitted service tier prepares");
    assert_eq!(run.evidence().service_tier(), None);
    let (handle, _events, outcome) = complete(run.start_run(fixture.services()));
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    let request: Value =
        serde_json::from_slice(&fixture.server.requests()[0].body).expect("create request is JSON");
    assert!(request.get("service_tier").is_none());
    let expected: Value = serde_json::from_slice(include_bytes!(
        "fixtures/openai-responses-2026-07-21/create-request.json"
    ))
    .expect("omitted fixture is JSON");
    assert_eq!(request, expected);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
}

#[test]
fn prepared_background_default_service_tier_composes_with_reasoning_and_structured_output() {
    for (index, value) in ["none", "low", "medium", "high", "xhigh", "max"]
        .iter()
        .copied()
        .enumerate()
    {
        let fixture = Fixture::new(
            ServerMode::ReasoningVocabulary,
            format!("host.service-tier.reasoning.{index}").as_str(),
            TimeMode::Pending,
        );
        let prepared = prepare_openai_background(fixture.preparation_input(), &fixture.services())
            .expect("OpenAI background integration prepares");
        let reasoning = ReasoningMode::new(value).expect("reasoning is valid");
        let run = prepared
            .prepare_background_run(
                profile(&format!("prepared-service-tier-reasoning-{index}"))
                    .with_reasoning_mode(reasoning.clone())
                    .with_service_tier(OpenAiBackgroundServiceTier::standard()),
            )
            .expect("composed reasoning and service tier prepare");
        assert_eq!(run.request().policy().reasoning_mode(), Some(&reasoning));
        assert_eq!(
            run.evidence().service_tier(),
            Some(OpenAiBackgroundServiceTier::standard())
        );
        let (handle, _events, outcome) = complete(run.start_run(fixture.services()));
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        let request: Value = serde_json::from_slice(&fixture.server.requests()[0].body)
            .expect("create request is JSON");
        assert_eq!(request["reasoning"]["effort"], value);
        assert_eq!(request["service_tier"], "default");
        assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    }

    let fixture = Fixture::new(ServerMode::Success, "host.local", TimeMode::Pending);
    let prepared = prepare_openai_background(fixture.preparation_input(), &fixture.services())
        .expect("OpenAI background integration prepares");
    let run = prepared
        .prepare_background_run(
            profile("prepared-service-tier-schema")
                .with_structured_output(schema())
                .with_service_tier(OpenAiBackgroundServiceTier::standard()),
        )
        .expect("composed structured output and service tier prepare");
    assert!(run.request().structured_output().is_some());
    assert_eq!(
        run.evidence().service_tier(),
        Some(OpenAiBackgroundServiceTier::standard())
    );
    assert!(fixture.server.requests().is_empty());
    assert_eq!(fixture.releases(), 0);
}

#[test]
fn prepared_background_service_tier_rejects_detachment_before_effects() {
    let fixture = Fixture::new(ServerMode::Success, "host.local", TimeMode::Pending);
    let prepared = prepare_openai_background(fixture.preparation_input(), &fixture.services())
        .expect("OpenAI background integration prepares");
    let error = prepared
        .prepare_background_run(
            profile("prepared-service-tier-detach")
                .with_service_tier(OpenAiBackgroundServiceTier::standard())
                .with_active_run_detachment(),
        )
        .expect_err("service tier with detachment fails");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.openai.preparation.service_tier_profile_unsupported"
    );
    assert!(fixture.server.requests().is_empty());
    assert_eq!(fixture.releases(), 0);
}

#[test]
fn prepared_background_default_service_tier_preserves_cancel_and_reattachment() {
    let raced = Fixture::new(ServerMode::CancelRace, "host.local", TimeMode::Pending);
    let prepared = prepare_openai_background(raced.preparation_input(), &raced.services())
        .expect("OpenAI background integration prepares");
    let run = prepared
        .prepare_background_run(
            profile("prepared-service-tier-cancel")
                .with_service_tier(OpenAiBackgroundServiceTier::standard()),
        )
        .expect("standard service tier prepares");
    let mut handle = block_on(run.start_run(raced.services())).expect("run starts");
    assert_eq!(
        block_on(handle.cancellation().request()).expect("cancel request succeeds"),
        swallowtail_runtime::CancellationAcknowledgement::Requested
    );
    let outcome = consume(&mut handle);
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    let request: Value =
        serde_json::from_slice(&raced.server.requests()[0].body).expect("create request is JSON");
    assert_eq!(request["service_tier"], "default");
    assert_eq!(raced.server.inference_attempts(), 1);
    assert_eq!(raced.releases(), 1);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
}

fn profile(id: &str) -> OpenAiBackgroundRunProfileInput {
    OpenAiBackgroundRunProfileInput::background_with_temporary_retention_and_one_reattachment(
        RequestId::new(id).expect("request id is valid"),
        selection(OPENAI_BACKGROUND_MODEL_ROUTE_ID, OPENAI_BACKGROUND_MODEL_ID),
        OperationContent::new("Say hello").expect("content is valid"),
        NonZeroU64::new(64).expect("limit is non-zero"),
        Deadline::at(MonotonicInstant::from_ticks(100_000)),
    )
}

fn explicit(
    id: &str,
    model: OpenAiBackgroundModelSelection,
    execution: ProviderExecutionPolicy,
    retention: ProviderRetentionPolicy,
    reattachment: StreamReattachmentPolicy,
) -> OpenAiBackgroundRunProfileInput {
    OpenAiBackgroundRunProfileInput::new(
        RequestId::new(id).expect("request id is valid"),
        model,
        OperationContent::new("Say hello").expect("content is valid"),
        NonZeroU64::new(64).expect("limit is non-zero"),
        Deadline::at(MonotonicInstant::from_ticks(100_000)),
        execution,
        retention,
        reattachment,
    )
}

fn selection(route: &str, model: &str) -> OpenAiBackgroundModelSelection {
    OpenAiBackgroundModelSelection::new(
        ModelRouteId::new(route).expect("route id is valid"),
        ModelRouteRevision::new("prepared-1").expect("route revision is valid"),
        ModelId::new(model).expect("model id is valid"),
    )
}

fn reattachment() -> StreamReattachmentPolicy {
    StreamReattachmentPolicy::Bounded(NonZeroU32::new(1).expect("one is non-zero"))
}

fn schema() -> StructuredOutputDescriptor {
    StructuredOutputDescriptor::new(
        SchemaDocument::inline(
            br#"{"type":"object","properties":{"result":{"type":"string"}},"required":["result"],"additionalProperties":false}"#,
            4096,
        )
        .expect("schema is bounded"),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .expect("schema descriptor is valid")
}

fn complete(
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
    let mut events = run.take_events().expect("events exist");
    let terminal = run.take_terminal_outcome().expect("terminal exists");
    let (events, outcome) = block_on(async {
        let mut observed = Vec::new();
        while let Some(event) = events.next().await {
            observed.push(event.expect("event succeeds"));
        }
        (observed, terminal.await)
    });
    (run, events, outcome)
}

fn consume(
    run: &mut Box<dyn swallowtail_runtime::RunHandle>,
) -> swallowtail_runtime::TerminalOutcome {
    let mut events = run.take_events().expect("events exist");
    let terminal = run.take_terminal_outcome().expect("terminal exists");
    block_on(async {
        while let Some(event) = events.next().await {
            event.expect("event succeeds");
        }
        terminal.await
    })
}
