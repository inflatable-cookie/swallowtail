#[allow(dead_code)]
#[path = "driver/fixture.rs"]
mod fixture;
#[allow(dead_code)]
mod support;

use fixture::Fixture;
use futures_executor::block_on;
use futures_util::StreamExt;
use swallowtail_adapter_deepseek::{
    DEEPSEEK_ENDPOINT_AUDIENCE, DEEPSEEK_MODEL_ID, DeepSeekCatalogueProfileInput,
    DeepSeekDirectDriver, DeepSeekModelSelection, DeepSeekRunProfileInput,
    DeepSeekSessionProfileInput, DeepSeekThinkingMode, deepseek_v4_config, prepare_deepseek_direct,
};
use swallowtail_core::{
    Capability, CapabilityConstraint, DriverRole, ModelId, ModelRouteId, ModelRouteRevision,
    ProviderInferenceCachePolicy, ReasoningMode,
};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, DirectContinuationTurnRequest, DirectToolResult,
    DirectToolResultContent, InteractiveSessionDriver, MonotonicInstant,
    OpenDirectContinuationSessionRequest, OperationContent, OperationPolicy, RequestId,
    RuntimeTurnId, SchemaDocument, SessionOptions, StructuredRunDriver, StructuredRunRequest,
    TerminalStatus, ToolDeclaration,
};
use swallowtail_testkit::{
    ExecutionTopologyFixture, assert_observable_activity_not_applicable,
    assert_observable_activity_trace, assert_prepared_operation_evidence_matches_plan,
};

#[test]
fn catalogue_and_consumer_authorized_continuation_run_on_both_host_topologies() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let fixture = Fixture::for_topology(&topology);
        let prepared = prepare_deepseek_direct(fixture.preparation_input(), &fixture.services())
            .expect("DeepSeek integration prepares");
        assert_eq!(
            prepared.access_profile().endpoint_audience().as_str(),
            DEEPSEEK_ENDPOINT_AUDIENCE
        );

        let catalogue = prepared
            .prepare_catalogue(DeepSeekCatalogueProfileInput::new(
                RequestId::new("prepared-catalogue").expect("request id"),
            ))
            .expect("catalogue prepares");
        assert_eq!(
            catalogue.plan().requirements().driver_role(),
            DriverRole::ModelCatalog
        );
        assert_prepared_operation_evidence_matches_plan(
            catalogue.evidence().operation(),
            catalogue.plan(),
        );
        assert_observable_activity_not_applicable(catalogue.evidence().operation());
        let models =
            block_on(catalogue.list_models(fixture.services())).expect("catalogue succeeds");
        assert!(
            models
                .iter()
                .any(|model| model.id().as_str() == DEEPSEEK_MODEL_ID)
        );
        assert_eq!(fixture.server.attempts(), 0);

        let prepared_session = prepared
            .prepare_session(session_input("prepared-session", DEEPSEEK_MODEL_ID))
            .expect("session prepares");
        assert_eq!(
            prepared_session.plan().requirements().driver_role(),
            DriverRole::InteractiveSession
        );
        assert_prepared_operation_evidence_matches_plan(
            prepared_session.evidence().operation(),
            prepared_session.plan(),
        );
        let mut session = block_on(prepared_session.open_session(fixture.services()))
            .expect("prepared session opens");
        let mut turn = block_on(session.start_direct_continuation_turn(
            DirectContinuationTurnRequest::new(
                RuntimeTurnId::new("prepared-turn").expect("turn id"),
                OperationContent::new("What is the fixture weather in London?").expect("content"),
                Deadline::at(MonotonicInstant::from_ticks(5_000)),
            ),
            fixture.services(),
        ))
        .expect("first consumer-authorized attempt starts");
        let mut exchange = turn
            .take_direct_tool_exchange()
            .expect("provider tool call pauses the turn");
        let mut calls = exchange.take_calls().expect("tool calls");
        let call = block_on(calls.next())
            .expect("one tool call")
            .expect("valid tool call");
        assert_eq!(fixture.server.attempts(), 1);
        block_on(exchange.submitter().submit(vec![DirectToolResult::new(
            call.call_id().clone(),
            DirectToolResultContent::new(
                br#"{"temperature_c":18,"condition":"clear"}"#.to_vec(),
                65_536,
            )
            .expect("bounded result"),
        )]))
        .expect("correlated result explicitly authorizes attempt two");
        let mut events = turn.take_events().expect("events");
        let terminal = turn.take_terminal_outcome().expect("terminal");
        let (collected, outcome) = block_on(async {
            let mut collected = Vec::new();
            while let Some(event) = events.next().await {
                collected.push(event.expect("event succeeds"));
            }
            (collected, terminal.await)
        });
        assert_observable_activity_trace(
            prepared_session.evidence().observable_activity(),
            &collected,
        );
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(fixture.server.attempts(), 2);
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
        assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
        assert_eq!(fixture.releases(), 2);
        assert_eq!(fixture.release_after_blocking(), [1, 3]);
    }
}

#[test]
fn model_substitution_and_unaccepted_cache_posture_fail_before_effects() {
    let fixture = Fixture::new();
    let prepared = prepare_deepseek_direct(fixture.preparation_input(), &fixture.services())
        .expect("integration prepares");
    let failure = prepared
        .prepare_session(session_input("wrong-model", "deepseek-v4-flash"))
        .expect_err("alternate model rejects");
    assert_eq!(
        failure.stage(),
        swallowtail_runtime::PreparationStage::Preflight
    );

    let failure = prepared
        .prepare_session(DeepSeekSessionProfileInput::new(
            RequestId::new("cache-not-accepted").expect("request id"),
            model(DEEPSEEK_MODEL_ID),
            ReasoningMode::new("high").expect("reasoning"),
            [tool()],
            ProviderInferenceCachePolicy::Prohibited,
        ))
        .expect_err("unaccepted cache posture rejects");
    assert_eq!(
        failure.stage(),
        swallowtail_runtime::PreparationStage::Preflight
    );

    let failure = prepared
        .prepare_run(DeepSeekRunProfileInput::new(
            RequestId::new("run-cache-not-accepted").expect("request id"),
            model(DEEPSEEK_MODEL_ID),
            OperationContent::new("must reject").expect("content"),
            ReasoningMode::new("high").expect("reasoning"),
            std::num::NonZeroU64::new(512).expect("maximum"),
            ProviderInferenceCachePolicy::Prohibited,
        ))
        .expect_err("structured run cache posture must also be explicit");
    assert_eq!(
        failure.stage(),
        swallowtail_runtime::PreparationStage::Preflight
    );
    assert!(fixture.server.requests().is_empty());
    assert_eq!(fixture.releases(), 0);
}

#[test]
fn exact_reasoning_selection_repeats_in_prepared_plan_request_and_evidence() {
    let fixture = Fixture::new();
    let prepared = prepare_deepseek_direct(fixture.preparation_input(), &fixture.services())
        .expect("integration prepares");

    for mode in ["low", "high", "max"] {
        let run = prepared
            .prepare_run(DeepSeekRunProfileInput::new(
                RequestId::new(format!("prepared-{mode}-run")).expect("request id"),
                model(DEEPSEEK_MODEL_ID),
                OperationContent::new("one exact selection").expect("content"),
                ReasoningMode::new(mode).expect("reasoning"),
                std::num::NonZeroU64::new(512).expect("maximum"),
                ProviderInferenceCachePolicy::AcceptedWithoutManagementAuthority,
            ))
            .expect("exact run effort prepares");
        assert_eq!(
            run.request()
                .policy()
                .reasoning_mode()
                .expect("request reasoning")
                .as_str(),
            mode
        );
        assert_plan_binds_reasoning(run.plan(), mode);
        assert_eq!(
            run.evidence()
                .reasoning_mode()
                .map(|selected| selected.as_str()),
            Some(mode)
        );
        assert_prepared_operation_evidence_matches_plan(run.evidence().operation(), run.plan());

        let session = prepared
            .prepare_session(session_input_with_reasoning(
                &format!("prepared-{mode}-session"),
                DEEPSEEK_MODEL_ID,
                mode,
            ))
            .expect("exact session effort prepares");
        assert_eq!(
            session
                .request()
                .options()
                .reasoning_mode()
                .expect("session reasoning")
                .as_str(),
            mode
        );
        assert_plan_binds_reasoning(session.plan(), mode);
        assert_eq!(
            session
                .evidence()
                .reasoning_mode()
                .map(|selected| selected.as_str()),
            Some(mode)
        );
        assert_prepared_operation_evidence_matches_plan(
            session.evidence().operation(),
            session.plan(),
        );
    }
    assert!(fixture.server.requests().is_empty());
}

#[test]
fn unsupported_reasoning_modes_fail_during_preparation_without_effects() {
    let fixture = Fixture::new();
    let prepared = prepare_deepseek_direct(fixture.preparation_input(), &fixture.services())
        .expect("integration prepares");

    for mode in ["medium", "xhigh", "provider-high", "disabled", "unknown"] {
        let failure = prepared
            .prepare_run(DeepSeekRunProfileInput::new(
                RequestId::new(format!("reject-{mode}-run")).expect("request id"),
                model(DEEPSEEK_MODEL_ID),
                OperationContent::new("must reject").expect("content"),
                ReasoningMode::new(mode).expect("reasoning"),
                std::num::NonZeroU64::new(512).expect("maximum"),
                ProviderInferenceCachePolicy::AcceptedWithoutManagementAuthority,
            ))
            .expect_err("unsupported run reasoning rejects");
        assert_eq!(
            failure.stage(),
            swallowtail_runtime::PreparationStage::Preflight
        );

        let failure = prepared
            .prepare_session(session_input_with_reasoning(
                &format!("reject-{mode}-session"),
                DEEPSEEK_MODEL_ID,
                mode,
            ))
            .expect_err("unsupported session reasoning rejects");
        assert_eq!(
            failure.stage(),
            swallowtail_runtime::PreparationStage::Preflight
        );
    }

    assert!(fixture.server.requests().is_empty());
    assert_eq!(fixture.releases(), 0);
}

#[test]
fn supported_reasoning_substitution_fails_against_prepared_plan_before_effects() {
    let fixture = Fixture::new();
    let prepared = prepare_deepseek_direct(fixture.preparation_input(), &fixture.services())
        .expect("integration prepares");
    let run = prepared
        .prepare_run(DeepSeekRunProfileInput::new(
            RequestId::new("bound-low-run").expect("request id"),
            model(DEEPSEEK_MODEL_ID),
            OperationContent::new("must reject substitution").expect("content"),
            ReasoningMode::new("low").expect("reasoning"),
            std::num::NonZeroU64::new(512).expect("maximum"),
            ProviderInferenceCachePolicy::AcceptedWithoutManagementAuthority,
        ))
        .expect("low run prepares");
    let run_request = StructuredRunRequest::new(
        RequestId::new("substituted-high-run").expect("request id"),
        OperationContent::new("must reject substitution").expect("content"),
        OperationPolicy::offline().with_reasoning_mode(ReasoningMode::new("high").expect("mode")),
    )
    .with_maximum_output_tokens(std::num::NonZeroU64::new(512).expect("maximum"));
    let error = block_on(DeepSeekDirectDriver::new().start_run(
        run.plan().clone(),
        run_request,
        fixture.services(),
    ))
    .err()
    .expect("supported reasoning substitution rejects");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.deepseek.unsupported"
    );

    let session = prepared
        .prepare_session(session_input_with_reasoning(
            "bound-low-session",
            DEEPSEEK_MODEL_ID,
            "low",
        ))
        .expect("low session prepares");
    let session_request = OpenDirectContinuationSessionRequest::new(
        RequestId::new("substituted-high-session").expect("request id"),
        deepseek_v4_config(),
    )
    .with_options(
        SessionOptions::default()
            .with_reasoning_mode(ReasoningMode::new("high").expect("mode"))
            .with_tools([tool()]),
    );
    let error = block_on(
        DeepSeekDirectDriver::new().open_direct_continuation_session(
            session.plan().clone(),
            session_request,
            fixture.services(),
        ),
    )
    .err()
    .expect("supported reasoning substitution rejects");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.deepseek.request_plan_mismatch"
    );
    assert!(fixture.server.requests().is_empty());
    assert_eq!(fixture.releases(), 0);
}

#[test]
fn one_request_structured_run_prepares_on_both_host_topologies() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let fixture =
            Fixture::with_topology_scenario(&topology, support::ServerScenario::StructuredSuccess);
        let prepared = prepare_deepseek_direct(fixture.preparation_input(), &fixture.services())
            .expect("DeepSeek integration prepares");
        let run = prepared
            .prepare_run(DeepSeekRunProfileInput::new(
                RequestId::new("prepared-run").expect("request id"),
                model(DEEPSEEK_MODEL_ID),
                OperationContent::new("one prepared request").expect("content"),
                ReasoningMode::new("high").expect("reasoning"),
                std::num::NonZeroU64::new(512).expect("maximum"),
                ProviderInferenceCachePolicy::AcceptedWithoutManagementAuthority,
            ))
            .expect("structured run prepares");
        assert_eq!(
            run.plan().requirements().driver_role(),
            DriverRole::StructuredRun
        );
        assert_prepared_operation_evidence_matches_plan(run.evidence().operation(), run.plan());
        let mut handle = block_on(run.start_run(fixture.services())).expect("run starts");
        let mut events = handle.take_events().expect("events");
        let terminal = handle.take_terminal_outcome().expect("terminal");
        let (collected, outcome) = block_on(async {
            let mut collected = Vec::new();
            while let Some(event) = events.next().await {
                collected.push(event.expect("event succeeds"));
            }
            (collected, terminal.await)
        });
        assert_observable_activity_trace(run.evidence().observable_activity(), &collected);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
        assert_eq!(fixture.server.attempts(), 1);
        assert_eq!(fixture.releases(), 1);
    }
}

#[test]
fn disabled_structured_run_binds_without_reasoning_selection_or_effort() {
    let fixture = Fixture::with_scenario(support::ServerScenario::StructuredNonThinkingSuccess);
    let prepared = prepare_deepseek_direct(fixture.preparation_input(), &fixture.services())
        .expect("DeepSeek integration prepares");
    let run = prepared
        .prepare_run(DeepSeekRunProfileInput::new_with_thinking_mode(
            RequestId::new("prepared-disabled-run").expect("request id"),
            model(DEEPSEEK_MODEL_ID),
            OperationContent::new("one non-thinking request").expect("content"),
            DeepSeekThinkingMode::disabled(),
            std::num::NonZeroU64::new(512).expect("maximum"),
            ProviderInferenceCachePolicy::AcceptedWithoutManagementAuthority,
        ))
        .expect("disabled structured run prepares");

    assert!(run.request().policy().reasoning_mode().is_none());
    assert_plan_has_no_reasoning(run.plan());
    assert_eq!(
        run.evidence().thinking_mode(),
        Some(DeepSeekThinkingMode::disabled())
    );
    assert!(run.evidence().reasoning_mode().is_none());
    assert_prepared_operation_evidence_matches_plan(run.evidence().operation(), run.plan());

    let mut handle = block_on(run.start_run(fixture.services())).expect("run starts");
    let mut events = handle.take_events().expect("events");
    let terminal = handle.take_terminal_outcome().expect("terminal");
    let (collected, outcome) = block_on(async {
        let mut collected = Vec::new();
        while let Some(event) = events.next().await {
            collected.push(event.expect("event succeeds"));
        }
        (collected, terminal.await)
    });
    assert_observable_activity_trace(run.evidence().observable_activity(), &collected);
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert_eq!(fixture.server.attempts(), 1);
    assert_eq!(fixture.releases(), 1);

    let body: serde_json::Value =
        serde_json::from_slice(&fixture.server.requests()[0].body).expect("request JSON");
    assert_eq!(body["thinking"]["type"], "disabled");
    assert!(body.get("reasoning_effort").is_none());
    assert_eq!(body["tools"], serde_json::json!([]));
}

fn session_input(id: &str, model_id: &str) -> DeepSeekSessionProfileInput {
    session_input_with_reasoning(id, model_id, "high")
}

fn session_input_with_reasoning(
    id: &str,
    model_id: &str,
    reasoning: &str,
) -> DeepSeekSessionProfileInput {
    DeepSeekSessionProfileInput::new(
        RequestId::new(id).expect("request id"),
        model(model_id),
        ReasoningMode::new(reasoning).expect("reasoning"),
        [tool()],
        ProviderInferenceCachePolicy::AcceptedWithoutManagementAuthority,
    )
}

fn assert_plan_binds_reasoning(plan: &swallowtail_core::PreflightPlan, mode: &str) {
    let mut requirements = plan
        .requirements()
        .capabilities()
        .filter(|requirement| requirement.capability() == Capability::ReasoningSelection);
    let requirement = requirements.next().expect("reasoning requirement");
    assert!(requirements.next().is_none());
    let mut constraints = requirement.constraints();
    assert!(matches!(
        constraints.next(),
        Some(CapabilityConstraint::ReasoningMode(selected)) if selected.as_str() == mode
    ));
    assert!(constraints.next().is_none());
}

fn assert_plan_has_no_reasoning(plan: &swallowtail_core::PreflightPlan) {
    assert!(
        !plan
            .requirements()
            .capabilities()
            .any(|requirement| requirement.capability() == Capability::ReasoningSelection)
    );
}

fn model(model_id: &str) -> DeepSeekModelSelection {
    DeepSeekModelSelection::new(
        ModelRouteId::new("deepseek.prepared.v4-pro").expect("route id"),
        ModelRouteRevision::new("2026-07-22").expect("route revision"),
        ModelId::new(model_id).expect("model id"),
    )
}

fn tool() -> ToolDeclaration {
    ToolDeclaration::new(
        "lookup_weather",
        SchemaDocument::inline(
            br#"{"type":"object","properties":{"city":{"type":"string"}},"required":["city"],"additionalProperties":false}"#.to_vec(),
            1_024,
        )
        .expect("schema"),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .expect("tool")
    .with_description(OperationContent::new("Return fixture weather").expect("description"))
}
