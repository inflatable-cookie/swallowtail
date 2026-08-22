use super::fixtures::{attempt_input, inventory_input, prepared, session_input};
use crate::support::{Fixture, FixtureServer, StreamFixture, VersionFixture};
use futures_executor::block_on;
use futures_util::StreamExt;
use swallowtail_adapter_ollama::{OllamaContextWindow, OllamaNativeAttachedDriver};
use swallowtail_core::{
    AttachedRuntimeResidency, Capability, CapabilityConstraint, ExecutionHostId, HostServiceKind,
    InstanceOwnership, ReasoningMode, StructuredOutputEnforcement,
};
use swallowtail_runtime::{
    CleanupOutcome, InteractiveSessionDriver, OperationContent, RuntimeTurnId, SchemaDocument,
    StructuredOutputDescriptor, StructuredRunDriver, TerminalStatus, TurnRequest,
    WorkingStateRestorationMethod, WorkingStateRestorationOutcome,
};
use swallowtail_testkit::{
    assert_observable_activity_not_applicable, assert_observable_activity_trace,
    assert_prepared_operation_evidence_matches_plan,
};

#[test]
fn prepared_inventory_and_inference_preserve_external_runtime_truth() {
    for host in [
        "ollama.prepared.local",
        "ollama.prepared.remote-authoritative",
    ] {
        let fixture = Fixture::with_host(host);
        let prepared = prepared(&fixture);
        assert_eq!(
            prepared.instance().ownership(),
            InstanceOwnership::ExternalAttached
        );
        assert!(matches!(
            prepared.runtime().compatibility(),
            swallowtail_core::InterfaceCompatibilityAssessment::Qualified(_)
        ));
        assert_eq!(prepared.runtime().installed().len(), 1);
        assert_eq!(prepared.runtime().running().len(), 1);
        assert_eq!(
            prepared.runtime().selected_detail().model_tag().as_str(),
            "fixture-model:8b"
        );
        for absent in [
            HostServiceKind::Process,
            HostServiceKind::Credential,
            HostServiceKind::ModelArtifact,
            HostServiceKind::ServingEndpoint,
        ] {
            assert!(
                !prepared
                    .available_host_services()
                    .any(|kind| kind == absent)
            );
        }

        let inventory = prepared
            .prepare_inventory(inventory_input("prepared-inventory"))
            .expect("inventory prepares");
        assert_prepared_operation_evidence_matches_plan(
            inventory.evidence().operation(),
            inventory.plan(),
        );
        assert_observable_activity_not_applicable(inventory.evidence().operation());
        assert_eq!(
            inventory.plan().model_id().unwrap().as_str(),
            "ollama.fixture.model"
        );
        let snapshot =
            block_on(inventory.observe_inventory(fixture.services())).expect("inventory observes");
        assert_eq!(snapshot.entries().len(), 1);
        assert_eq!(snapshot.installed().count(), 1);
        assert_eq!(snapshot.running().count(), 1);
        assert!(snapshot.selected_detail().is_some());
        assert_eq!(fixture.server.inference_attempts(), 0);

        let attempt = prepared
            .prepare_inference_attempt(attempt_input("prepared-inference"))
            .expect("inference prepares");
        assert_prepared_operation_evidence_matches_plan(
            attempt.evidence().operation(),
            attempt.plan(),
        );
        assert_eq!(
            attempt.request().policy().attached_runtime_residency(),
            Some(AttachedRuntimeResidency::RuntimeManaged)
        );
        let mut run =
            block_on(attempt.start_run(fixture.services())).expect("prepared inference starts");
        let mut events = run.take_events().expect("events exist");
        let terminal = run.take_terminal_outcome().expect("terminal exists");
        let (collected, outcome) = block_on(async {
            let mut collected = Vec::new();
            while let Some(event) = events.next().await {
                collected.push(event.expect("event succeeds"));
            }
            (collected, terminal.await)
        });
        assert_observable_activity_trace(attempt.evidence().observable_activity(), &collected);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(fixture.server.inference_attempts(), 1);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
        assert!(fixture.server.is_reachable());
        assert_eq!(
            attempt.plan().execution_host_id(),
            &ExecutionHostId::new(host).unwrap()
        );
    }
}

#[test]
fn prepared_session_replays_only_cleanly_committed_history() {
    let fixture = Fixture::with_server(FixtureServer::start_with(
        VersionFixture::Expected,
        StreamFixture::InteractiveSequence,
    ));
    let prepared = prepared(&fixture);
    let profile = prepared
        .prepare_session(session_input("ollama-session"))
        .expect("session prepares");
    assert_prepared_operation_evidence_matches_plan(profile.evidence().operation(), profile.plan());
    let services = fixture.services();
    let activity_profile = profile.evidence().observable_activity().clone();
    let mut session = block_on(profile.open_session(services.clone())).expect("session opens");
    assert!(session.provider_session_ref().is_none());
    assert!(session.resume_binding().is_none());

    for (id, content) in [
        ("ollama-turn-1", "First fixture turn"),
        ("ollama-turn-2", "Second fixture turn"),
    ] {
        let mut turn = block_on(session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new(id).expect("valid turn"),
                OperationContent::new(content).expect("valid content"),
            ),
            services.clone(),
        ))
        .expect("turn starts");
        let mut events = turn.take_events().expect("events are available");
        let terminal = turn
            .take_terminal_outcome()
            .expect("terminal outcome is available");
        let (collected, outcome) = block_on(async {
            let mut collected = Vec::new();
            while let Some(event) = events.next().await {
                collected.push(event.expect("event succeeds"));
            }
            (collected, terminal.await)
        });
        assert_observable_activity_trace(&activity_profile, &collected);
        assert_eq!(outcome.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    }

    let bodies = fixture.server.inference_bodies();
    assert_eq!(bodies.len(), 2);
    let first: serde_json::Value = serde_json::from_slice(&bodies[0]).expect("first body is JSON");
    let second: serde_json::Value =
        serde_json::from_slice(&bodies[1]).expect("second body is JSON");
    assert_eq!(
        first,
        serde_json::from_str::<serde_json::Value>(include_str!(
            "../fixtures/ollama-native-v0.14.0-v0.32.1/interactive-turn-1-request.json"
        ))
        .expect("first fixture request is JSON")
    );
    assert_eq!(
        second,
        serde_json::from_str::<serde_json::Value>(include_str!(
            "../fixtures/ollama-native-v0.14.0-v0.32.1/interactive-turn-2-request.json"
        ))
        .expect("second fixture request is JSON")
    );
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
    assert!(fixture.server.is_reachable());
}

#[test]
fn prepared_session_restoration_opens_an_empty_replacement() {
    let fixture = Fixture::new();
    let profile = prepared(&fixture)
        .prepare_session(session_input("ollama-restoration"))
        .expect("session prepares");
    let interrupted = RuntimeTurnId::new("ollama-interrupted").expect("turn id");
    let restoration = profile.prepare_working_state_restoration(interrupted.clone());
    assert_eq!(
        restoration.method(),
        WorkingStateRestorationMethod::FreshSessionReplacement
    );
    let restored = block_on(restoration.restore(fixture.services())).expect("replacement opens");
    let WorkingStateRestorationOutcome::SessionReplaced(replacement) = restored else {
        panic!("fresh session replacement expected");
    };
    assert_eq!(replacement.interrupted_turn_id(), &interrupted);
    let (_, replacement) = replacement.into_parts();
    assert!(replacement.provider_session_ref().is_none());
    assert_eq!(block_on(replacement.close()), CleanupOutcome::Clean);
    assert_eq!(fixture.server.inference_attempts(), 0);
}

#[test]
fn failed_ollama_turn_does_not_mutate_the_private_transcript() {
    let fixture = Fixture::with_server(FixtureServer::start_with(
        VersionFixture::Expected,
        StreamFixture::InteractiveFailureThenSuccess,
    ));
    let prepared = prepared(&fixture);
    let profile = prepared
        .prepare_session(session_input("ollama-transaction"))
        .expect("session prepares");
    let services = fixture.services();
    let mut session = block_on(profile.open_session(services.clone())).expect("session opens");
    for (index, content, expected) in [
        (1, "First fixture turn", TerminalStatus::Completed),
        (
            2,
            "failed fixture turn",
            TerminalStatus::ProviderFailed(swallowtail_core::SafeDiagnostic::new(
                "swallowtail.ollama.stream_failed",
                "Ollama reported a stream failure",
            )),
        ),
        (3, "retry fixture turn", TerminalStatus::Completed),
    ] {
        let mut turn = block_on(session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new(format!("ollama-transaction-{index}")).expect("valid turn"),
                OperationContent::new(content).expect("valid content"),
            ),
            services.clone(),
        ))
        .expect("turn starts");
        let outcome = block_on(
            turn.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        assert_eq!(outcome.status(), &expected);
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    }
    let bodies = fixture.server.inference_bodies();
    let retry: serde_json::Value =
        serde_json::from_slice(&bodies[2]).expect("retry request is JSON");
    let messages = retry["messages"]
        .as_array()
        .expect("retry messages are an array");
    assert_eq!(messages.len(), 3);
    assert_eq!(messages[0]["content"], "First fixture turn");
    assert_eq!(messages[1]["content"], "First answer");
    assert_eq!(messages[2]["content"], "retry fixture turn");
    assert!(!retry.to_string().contains("partial must not commit"));
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn prepared_generation_controls_require_model_evidence_and_exact_constraints() {
    let fixture = Fixture::new();
    let prepared = prepared(&fixture);
    assert!(
        prepared
            .runtime()
            .selected_model_supports(swallowtail_adapter_ollama::OllamaModelCapability::Thinking)
    );

    for mode in ["off", "low", "medium", "high"] {
        let attempt = prepared
            .prepare_inference_attempt(
                attempt_input(&format!("prepared-{mode}"))
                    .with_reasoning_mode(ReasoningMode::new(mode).expect("mode is valid"))
                    .with_structured_output(schema()),
            )
            .expect("qualified generation controls prepare");
        assert!(
            attempt
                .plan()
                .requirements()
                .capabilities()
                .any(|requirement| {
                    requirement.capability() == Capability::ReasoningSelection
                        && requirement.constraints().any(|constraint| {
                            constraint
                                == &CapabilityConstraint::ReasoningMode(
                                    ReasoningMode::new(mode).expect("mode is valid"),
                                )
                        })
                })
        );
        assert!(
            attempt
                .plan()
                .requirements()
                .capabilities()
                .any(|requirement| {
                    requirement.capability() == Capability::StructuredOutput
                        && requirement.constraints().any(|constraint| {
                            constraint
                                == &CapabilityConstraint::StructuredOutputEnforcement(
                                    StructuredOutputEnforcement::ProviderNative,
                                )
                        })
                })
        );
    }

    let error =
        prepared
            .prepare_inference_attempt(attempt_input("prepared-unsupported").with_reasoning_mode(
                ReasoningMode::new("max").expect("mode is syntactically valid"),
            ))
            .expect_err("unsupported mode fails");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.ollama.preparation.reasoning_unsupported"
    );
    assert_eq!(fixture.server.inference_attempts(), 0);
}

#[test]
fn prepared_session_context_window_replays_fixed_value_across_turns_and_restoration() {
    let fixture = Fixture::with_server(FixtureServer::start_with(
        VersionFixture::Expected,
        StreamFixture::InteractiveFailureThenSuccess,
    ));
    let prepared = prepared(&fixture);
    let context = OllamaContextWindow::from_u64(8192).expect("representative value");
    let profile = prepared
        .prepare_session(session_input("ollama-context-session").with_context_window(context))
        .expect("session prepares");
    let services = fixture.services();
    let mut session = block_on(profile.open_session(services.clone())).expect("session opens");

    for (index, content, expected) in [
        (1, "First fixture turn", TerminalStatus::Completed),
        (
            2,
            "failed fixture turn",
            TerminalStatus::ProviderFailed(swallowtail_core::SafeDiagnostic::new(
                "swallowtail.ollama.stream_failed",
                "Ollama reported a stream failure",
            )),
        ),
        (3, "retry fixture turn", TerminalStatus::Completed),
    ] {
        let mut turn = block_on(session.start_turn(
            TurnRequest::new(
                RuntimeTurnId::new(format!("ollama-context-{index}")).expect("valid turn"),
                OperationContent::new(content).expect("valid content"),
            ),
            services.clone(),
        ))
        .expect("turn starts");
        let outcome = block_on(
            turn.take_terminal_outcome()
                .expect("terminal outcome is available"),
        );
        assert_eq!(outcome.status(), &expected);
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    }

    for body in fixture.server.inference_bodies() {
        let request: serde_json::Value =
            serde_json::from_slice(&body).expect("request body is JSON");
        assert_eq!(request["options"]["num_ctx"], 8192);
    }

    let retry: serde_json::Value = serde_json::from_slice(
        fixture
            .server
            .inference_bodies()
            .last()
            .expect("retry request exists"),
    )
    .expect("retry request is JSON");
    let messages = retry["messages"]
        .as_array()
        .expect("retry messages are an array");
    assert_eq!(messages.len(), 3);
    assert_eq!(messages[0]["content"], "First fixture turn");
    assert_eq!(messages[2]["content"], "retry fixture turn");
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);

    let restoration = profile.prepare_working_state_restoration(
        RuntimeTurnId::new("ollama-context-interrupted").expect("turn id"),
    );
    let restored = block_on(restoration.restore(services.clone())).expect("replacement opens");
    let WorkingStateRestorationOutcome::SessionReplaced(replacement) = restored else {
        panic!("fresh session replacement expected");
    };
    let (_, mut replacement) = replacement.into_parts();
    let mut turn = block_on(replacement.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("ollama-context-restored").expect("valid turn"),
            OperationContent::new("restored fixture turn").expect("valid content"),
        ),
        services,
    ))
    .expect("restored turn starts");
    let outcome = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    assert_eq!(block_on(replacement.close()), CleanupOutcome::Clean);

    let restored_body: serde_json::Value = serde_json::from_slice(
        fixture
            .server
            .inference_bodies()
            .last()
            .expect("restored request exists"),
    )
    .expect("restored request is JSON");
    assert_eq!(restored_body["options"]["num_ctx"], 8192);
    assert_eq!(
        restored_body["messages"]
            .as_array()
            .expect("messages")
            .len(),
        1
    );
}

#[test]
fn prepared_context_window_binds_evidence_driver_and_native_body() {
    let fixture = Fixture::new();
    let prepared = prepared(&fixture);
    let context = OllamaContextWindow::from_u64(8192).expect("representative value");
    let attempt = prepared
        .prepare_inference_attempt(
            attempt_input("prepared-context-window").with_context_window(context),
        )
        .expect("context window prepares");
    assert_eq!(attempt.evidence().context_window(), Some(context));

    let mut run = block_on(attempt.start_run(fixture.services())).expect("run starts");
    let terminal = run.take_terminal_outcome().expect("terminal exists");
    block_on(terminal);
    let bodies = fixture.server.inference_bodies();
    let body = bodies.last().expect("request body");
    let request: serde_json::Value = serde_json::from_slice(body).expect("request body is JSON");
    assert_eq!(request["options"]["num_ctx"], 8192);
    assert_eq!(request["options"]["num_predict"], 8);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);

    let session = prepared
        .prepare_session(
            session_input("prepared-session-context-window").with_context_window(context),
        )
        .expect("session context window prepares");
    assert_eq!(session.evidence().context_window(), Some(context));
    assert_eq!(fixture.server.inference_attempts(), 1);
}

#[test]
fn prepared_context_window_rejects_out_of_domain_values() {
    for value in [1u64, 3, u64::from(u32::MAX)] {
        let error = OllamaContextWindow::from_u64(value).expect_err("out of domain");
        assert_eq!(
            error.diagnostic().code(),
            "swallowtail.ollama.context_window_invalid"
        );
    }
}

#[test]
fn mismatched_low_level_run_driver_fails_before_network() {
    let fixture = Fixture::new();
    let prepared = prepared(&fixture);
    let context = OllamaContextWindow::from_u64(8192).expect("representative value");
    let attempt = prepared
        .prepare_inference_attempt(
            attempt_input("context-window-run-mismatch").with_context_window(context),
        )
        .expect("context window prepares");

    let error = OllamaNativeAttachedDriver::new()
        .validate_against_prepared_evidence(attempt.evidence())
        .expect_err("unbound driver must reject prepared context evidence");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.ollama.context_window_binding_mismatch"
    );

    let session_profile = prepared
        .prepare_session(session_input("context-window-run-mismatch-session"))
        .expect("session prepares");
    let driver = OllamaNativeAttachedDriver::bound_to_prepared_inference_attempt(&attempt);
    let error = match block_on(driver.start_run(
        session_profile.plan().clone(),
        attempt.request().clone(),
        fixture.services(),
    )) {
        Err(error) => error,
        Ok(_) => panic!("bound driver must reject alien preflight before transport"),
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.ollama.context_window_binding_mismatch"
    );
    assert_eq!(fixture.server.inference_attempts(), 0);
}

#[test]
fn mismatched_low_level_session_driver_fails_before_network() {
    let fixture = Fixture::new();
    let prepared = prepared(&fixture);
    let context = OllamaContextWindow::from_u64(8192).expect("representative value");
    let profile = prepared
        .prepare_session(
            session_input("context-window-session-mismatch").with_context_window(context),
        )
        .expect("session context window prepares");

    let error = OllamaNativeAttachedDriver::new()
        .validate_against_prepared_evidence(profile.evidence())
        .expect_err("unbound driver must reject prepared context evidence");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.ollama.context_window_binding_mismatch"
    );

    let inference_attempt = prepared
        .prepare_inference_attempt(attempt_input("context-window-session-mismatch-run"))
        .expect("inference prepares");
    let driver = OllamaNativeAttachedDriver::bound_to_prepared_session(&profile);
    let error = match block_on(driver.open_session(
        inference_attempt.plan().clone(),
        profile.request().clone(),
        fixture.services(),
    )) {
        Err(error) => error,
        Ok(_) => panic!("bound driver must reject alien preflight before transport"),
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.ollama.context_window_binding_mismatch"
    );
    assert_eq!(fixture.server.inference_attempts(), 0);
}

#[test]
fn mismatched_bound_driver_rejects_different_prepared_evidence_at_start_run() {
    let fixture = Fixture::new();
    let prepared = prepared(&fixture);
    let with_context = prepared
        .prepare_inference_attempt(
            attempt_input("context-window-evidence-a")
                .with_context_window(OllamaContextWindow::from_u64(8192).expect("value")),
        )
        .expect("first attempt prepares");
    let other_context = prepared
        .prepare_inference_attempt(
            attempt_input("context-window-evidence-b")
                .with_context_window(OllamaContextWindow::from_u64(4096).expect("value")),
        )
        .expect("second attempt prepares");
    let driver = OllamaNativeAttachedDriver::bound_to_prepared_inference_attempt(&with_context);
    let error = match block_on(driver.start_run(
        other_context.plan().clone(),
        other_context.request().clone(),
        fixture.services(),
    )) {
        Err(error) => error,
        Ok(_) => panic!("bound driver must reject alien prepared request before transport"),
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.ollama.context_window_binding_mismatch"
    );
    assert_eq!(fixture.server.inference_attempts(), 0);
}

#[test]
fn mismatched_bound_session_driver_rejects_different_prepared_evidence_at_open_session() {
    let fixture = Fixture::new();
    let prepared = prepared(&fixture);
    let with_context = prepared
        .prepare_session(
            session_input("context-window-session-evidence-a")
                .with_context_window(OllamaContextWindow::from_u64(8192).expect("value")),
        )
        .expect("first session prepares");
    let other_context = prepared
        .prepare_session(
            session_input("context-window-session-evidence-b")
                .with_context_window(OllamaContextWindow::from_u64(4096).expect("value")),
        )
        .expect("second session prepares");
    let driver = OllamaNativeAttachedDriver::bound_to_prepared_session(&with_context);
    let error = match block_on(driver.open_session(
        other_context.plan().clone(),
        other_context.request().clone(),
        fixture.services(),
    )) {
        Err(error) => error,
        Ok(_) => panic!("bound driver must reject alien prepared request before transport"),
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.ollama.context_window_binding_mismatch"
    );
    assert_eq!(fixture.server.inference_attempts(), 0);
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
