use super::*;
use futures_util::StreamExt;

#[path = "session/activity.rs"]
mod activity;

#[test]
fn plan_mode_is_gated_by_the_exact_codex_release() {
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.84.0",
        &RecordingHostServices::default(),
        false,
    );
    let failure = prepared_app
        .prepare_read_only_session(CodexSessionProfileInput::new(
            RequestId::new("plan-mode-before-support").unwrap(),
            model(),
            working_resource(),
            None,
            SessionOptions::default().with_harness_mode(HarnessMode::Plan),
        ))
        .expect_err("pre-plan-mode Codex release is rejected");
    assert_eq!(failure.stage(), PreparationStage::Preflight);
    assert_eq!(
        failure.diagnostic().safe().code(),
        "swallowtail.codex.preparation.harness_mode_unsupported"
    );
}

#[test]
fn prepared_session_exchanges_typed_user_input_without_enabling_approvals() {
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.146.0",
        &RecordingHostServices::default(),
        false,
    );
    let profile = prepared_app
        .prepare_read_only_session(
            CodexSessionProfileInput::new(
                RequestId::new("user-input").unwrap(),
                model(),
                working_resource(),
                None,
                SessionOptions::default(),
            )
            .with_user_input_exchange(),
        )
        .expect("user-input exchange prepares");
    let policy = profile.request().access_policy();
    assert_eq!(
        policy
            .provider_requests()
            .handling_for(&swallowtail_adapter_codex::codex_user_input_request_extension()),
        swallowtail_core::ProviderRequestHandling::Exchange
    );
    assert_eq!(
        policy
            .provider_requests()
            .handling_for(&swallowtail_adapter_codex::codex_approval_request_extension()),
        swallowtail_core::ProviderRequestHandling::Reject
    );

    let (process, state) = ScriptedAppServer::gate_enforcing(AppServerMode::ExchangeUserInput);
    let services = support::host_services(process);
    let mut session = block_on(profile.open_session(services.clone()))
        .expect("prepared user-input session opens");
    let mut turn = block_on(session.start_turn(
        TurnRequest::new(
            RuntimeTurnId::new("user-input-turn").unwrap(),
            OperationContent::new("ask me").unwrap(),
        ),
        services,
    ))
    .expect("turn starts");
    let mut callbacks = turn.take_callbacks().expect("callback exchange is exposed");
    let mut requests = callbacks
        .take_requests()
        .expect("request stream is exposed");
    let request = block_on(requests.next())
        .expect("question arrives")
        .expect("question is valid");
    let user_input = match request.kind() {
        swallowtail_runtime::CallbackRequestKind::HarnessUserInput(request) => request,
        kind => panic!("expected typed user input, got {kind:?}"),
    };
    assert_eq!(user_input.questions().len(), 1);
    assert_eq!(user_input.auto_resolution_ms(), Some(60_000));
    assert!(!format!("{request:?}").contains("Choose a scope"));

    let invalid = HarnessUserInputResponse::new(
        [HarnessUserInputAnswer::selected(
            HarnessQuestionId::new("scope").unwrap(),
            [HarnessQuestionOptionId::new("not-offered").unwrap()],
            None,
        )],
        16,
        64 * 1024,
    )
    .unwrap();
    let failure = block_on(callbacks.responder().respond(
        swallowtail_runtime::CallbackResponse::new(
            request.callback_id().clone(),
            request.turn_id().expect("turn callback").clone(),
            swallowtail_runtime::CallbackResult::UserInput(invalid),
        ),
    ))
    .expect_err("an unoffered option is rejected");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.codex.app_server.user_input_response_invalid"
    );

    let response = HarnessUserInputResponse::new(
        [HarnessUserInputAnswer::selected(
            HarnessQuestionId::new("scope").unwrap(),
            [HarnessQuestionOptionId::new("Tests").unwrap()],
            None,
        )],
        16,
        64 * 1024,
    )
    .unwrap();
    block_on(
        callbacks
            .responder()
            .respond(swallowtail_runtime::CallbackResponse::new(
                request.callback_id().clone(),
                request.turn_id().expect("turn callback").clone(),
                swallowtail_runtime::CallbackResult::UserInput(response),
            )),
    )
    .expect("typed response is accepted");
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(
        terminal.status(),
        &swallowtail_runtime::TerminalStatus::Completed
    );
    let provider_response = state
        .messages()
        .into_iter()
        .find(|message| {
            message.get("id").and_then(serde_json::Value::as_str) == Some("input-900")
                && message.get("result").is_some()
        })
        .expect("provider response is sent");
    assert_eq!(
        provider_response["result"]["answers"]["scope"]["answers"],
        serde_json::json!(["Tests"])
    );
    assert_eq!(block_on(turn.close()), CleanupOutcome::NotApplicable);
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn prepared_session_derives_tool_bounds_from_bounded_declarations() {
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.145.0",
        &RecordingHostServices::default(),
        false,
    );
    let larger_schema = serde_json::to_vec(&serde_json::json!({
        "type": "object",
        "description": "x".repeat(8 * 1024),
    }))
    .expect("schema serializes");
    let larger_schema_bytes = u64::try_from(larger_schema.len()).expect("schema length fits");
    let larger_tool = ToolDeclaration::new(
        "large_lookup",
        SchemaDocument::inline(larger_schema, 16 * 1024).expect("schema is bounded"),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .expect("tool declaration is valid");
    let profile = prepared_app
        .prepare_read_only_session(CodexSessionProfileInput::new(
            RequestId::new("bounded-tools").unwrap(),
            model(),
            working_resource(),
            None,
            SessionOptions::default().with_tools([tool("lookup"), larger_tool]),
        ))
        .expect("bounded tool declarations prepare");
    let requirement = profile
        .plan()
        .requirements()
        .capabilities()
        .find(|requirement| requirement.capability() == Capability::ToolCalls)
        .expect("tool capability is planned");

    assert!(
        requirement
            .constraints()
            .any(|constraint| constraint == &CapabilityConstraint::ToolMaximumCount(2))
    );
    assert!(requirement.constraints().any(|constraint| {
        constraint == &CapabilityConstraint::ToolMaximumSchemaBytes(larger_schema_bytes)
    }));

    let (process, state) = ScriptedAppServer::gate_enforcing(AppServerMode::CompleteTurn);
    let handle = block_on(profile.open_session(support::host_services(process)))
        .expect("prepared session opens");
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    let thread_start = state
        .messages()
        .into_iter()
        .find(|message| {
            message.get("method").and_then(serde_json::Value::as_str) == Some("thread/start")
        })
        .expect("thread start is captured");
    assert_eq!(
        thread_start
            .pointer("/params/dynamicTools")
            .and_then(serde_json::Value::as_array)
            .map(Vec::len),
        Some(2)
    );
    assert!(state.waited());
}

#[test]
fn session_resume_agreement_is_derived_and_unsupported_deadlines_fail_in_preparation() {
    let recording = RecordingHostServices::default();
    let prepared_app = prepared(CodexPreparedDriver::AppServer, "0.145.0", &recording, false);
    let profile = prepared_app
        .prepare_read_only_session(CodexSessionProfileInput::new(
            RequestId::new("open").unwrap(),
            model(),
            working_resource(),
            None,
            SessionOptions::default(),
        ))
        .expect("read-only session prepares");
    let binding = support::session_resume_binding(profile.plan(), "thread-1");
    let resume = profile
        .resume_request(RequestId::new("resume").unwrap(), binding.clone())
        .expect("resume request derives immutable agreement");
    assert_eq!(resume.access_policy(), profile.request().access_policy());
    assert_eq!(
        resume.harness_configuration_posture(),
        profile.request().harness_configuration_posture()
    );
    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let loaded = block_on(
        profile
            .load_session(
                RequestId::new("bound-load").unwrap(),
                binding.clone(),
                support::host_services(process),
            )
            .expect("bound load prepares"),
    )
    .expect("bound load opens");
    assert_eq!(
        loaded
            .replay()
            .filter_map(|item| item.content().map(|content| content.as_str()))
            .collect::<Vec<_>>(),
        ["Earlier question.", "Earlier answer."]
    );
    let (_, loaded_handle) = loaded.into_parts();
    assert_eq!(
        loaded_handle
            .management_binding()
            .expect("prepared load returns management authority")
            .origin(),
        swallowtail_core::ProviderSessionBindingOrigin::Loaded
    );
    assert_eq!(block_on(loaded_handle.close()), CleanupOutcome::Clean);
    assert!(state.methods().contains(&"thread/resume".to_owned()));
    assert!(state.waited());

    let (process, state) = ScriptedAppServer::new(AppServerMode::CompleteTurn);
    let handle = block_on(
        profile
            .resume_session(
                RequestId::new("bound-resume").unwrap(),
                binding,
                support::host_services(process),
            )
            .expect("bound resume prepares"),
    )
    .expect("bound resume opens");
    assert_eq!(
        handle
            .management_binding()
            .expect("prepared resume returns management authority")
            .origin(),
        swallowtail_core::ProviderSessionBindingOrigin::Resumed
    );
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    assert!(state.methods().contains(&"thread/resume".to_owned()));
    assert!(state.waited());

    let failure = prepared_app
        .prepare_read_only_session(CodexSessionProfileInput::new(
            RequestId::new("deadline").unwrap(),
            model(),
            working_resource(),
            Some(Deadline::at(MonotonicInstant::from_ticks(200))),
            SessionOptions::default(),
        ))
        .expect_err("unsupported session deadline fails during preparation");
    assert_eq!(failure.stage(), PreparationStage::Preflight);
}

#[test]
fn prepared_profile_keeps_exact_version_and_access_provenance_visible() {
    let prepared_app = prepared(
        CodexPreparedDriver::AppServer,
        "0.146.0",
        &RecordingHostServices::default(),
        false,
    );
    let profile = prepared_app
        .prepare_catalogue(RequestId::new("catalogue-evidence").unwrap(), None)
        .expect("catalogue prepares");

    assert_eq!(
        profile
            .evidence()
            .observation()
            .version()
            .version()
            .as_str(),
        "0.146.0"
    );
    assert_eq!(
        profile.evidence().access().provenance(),
        &swallowtail_runtime::AccessEvidenceProvenance::CallerAsserted
    );
    swallowtail_testkit::assert_prepared_operation_evidence_matches_plan(
        profile.evidence().operation(),
        profile.plan(),
    );
    assert_eq!(
        profile
            .evidence()
            .operation()
            .observable_activity()
            .availability(),
        swallowtail_core::ObservableActivityAvailability::NotApplicable
    );
    assert_eq!(
        profile.evidence().operation().binding().driver_role(),
        DriverRole::ModelCatalog
    );
    assert_eq!(
        profile
            .evidence()
            .operation()
            .interface_compatibility()
            .count(),
        1
    );
    let compatibility = profile
        .evidence()
        .operation()
        .interface_compatibility()
        .next()
        .expect("Codex version evidence is present");
    let swallowtail_core::InterfaceCompatibilityAssessment::Qualified(matched) =
        compatibility.assessment()
    else {
        panic!("Codex 0.146.0 must be qualified");
    };
    assert_eq!(
        matched.support_status(),
        swallowtail_core::InterfaceSupportStatus::Maintained
    );
}
