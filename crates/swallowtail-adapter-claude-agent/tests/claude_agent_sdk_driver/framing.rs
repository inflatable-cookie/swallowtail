use crate::host_id;
use crate::sdk_support::{
    SdkFixtureHost, SdkScenario, cleanup_request, prepared_session, prepared_session_for,
    turn_request,
};
use futures_executor::block_on;
use futures_util::StreamExt;
use swallowtail_adapter_claude_agent::sdk::{
    ClaudeAgentSdkPermissionMode, ClaudeAgentSdkSessionProfile, ClaudeAgentSdkTool,
};
use swallowtail_runtime::{CallbackPayload, CallbackResponse, CallbackResult, TerminalStatus};

fn terminal_status(scenario: SdkScenario) -> TerminalStatus {
    let host = host_id("claude-agent-sdk.fixture.framing");
    let fixture = SdkFixtureHost::new(scenario);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let mut session =
        block_on(prepared.open_session(services.clone())).expect("SDK sidecar session opens");
    let mut turn = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
        .expect("SDK sidecar turn starts");
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    let _ = block_on(turn.close());
    let _ = block_on(session.close(cleanup_request(), services_for_cleanup.clone()));
    terminal.status().clone()
}

#[test]
fn unqualified_malformed_truncated_and_terminal_records_all_fail_closed() {
    for scenario in [
        SdkScenario::UnknownEvent,
        SdkScenario::Malformed,
        SdkScenario::Disconnect,
        SdkScenario::TerminalRecord,
        SdkScenario::ToolOrderingDrift,
    ] {
        let status = terminal_status(scenario);
        let TerminalStatus::RuntimeFailed(diagnostic) = &status else {
            panic!("{scenario:?} must fail the turn, got {status:?}");
        };
        assert!(
            !diagnostic.message().contains("/fixture/"),
            "{scenario:?} diagnostic must not carry a host path"
        );
    }
}

#[test]
fn every_sidecar_terminal_code_reaches_the_bounded_runtime_diagnostic() {
    for (scenario, code) in [
        (SdkScenario::TerminalRecordTooLarge, "record_too_large"),
        (SdkScenario::TerminalEmptyRecord, "empty_record"),
        (SdkScenario::TerminalMalformedJson, "malformed_json"),
        (SdkScenario::TerminalMissingType, "missing_type"),
        (SdkScenario::TerminalUnknownRecord, "unknown_record"),
        (SdkScenario::TerminalInvalidCommand, "invalid_command"),
        (SdkScenario::TerminalCommandIdReused, "command_id_reused"),
        (SdkScenario::TerminalTooManyPending, "too_many_pending"),
        (SdkScenario::TerminalCallbackUnknown, "callback_unknown"),
        (SdkScenario::TerminalCallbackInvalid, "callback_invalid"),
        (SdkScenario::TerminalInternalError, "internal_error"),
        (SdkScenario::TerminalUnknownMessage, "unknown_message"),
    ] {
        let status = terminal_status(scenario);
        let TerminalStatus::RuntimeFailed(diagnostic) = &status else {
            panic!("{scenario:?} must fail the turn, got {status:?}");
        };
        assert_eq!(
            diagnostic.code(),
            "swallowtail.claude-agent.sdk.sidecar_terminated"
        );
        assert!(
            diagnostic
                .message()
                .contains(&format!("sidecar_terminated: {code}")),
            "{scenario:?} must preserve its sidecar code: {}",
            diagnostic.message()
        );
        assert!(diagnostic.message().contains("stderr: native <path>"));
        assert!(diagnostic.message().contains("<redacted>"));
        assert!(diagnostic.message().ends_with("[stderr truncated]"));
        for forbidden in ["/private/fixture", "fixture-secret", "user@example.test"] {
            assert!(
                !diagnostic.message().contains(forbidden),
                "{scenario:?} diagnostic leaked {forbidden}: {}",
                diagnostic.message()
            );
        }
    }
}

#[test]
fn failed_turn_end_carries_all_sanitized_fields_without_error_text() {
    let status = terminal_status(SdkScenario::TurnEndedError);
    let TerminalStatus::ProviderFailed(diagnostic) = status else {
        panic!("failed turn end must be provider failure");
    };
    for field in [
        "subtype=error_during_execution",
        "stopReason=error_during_execution",
        "isError=true",
        "numTurns=1",
        "durationMs=7",
        "errorTextPresent=true",
        "errorTextType=string",
        "apiErrorStatus=<null>",
        "terminalReason=<null>",
        "rateLimitStatus=<null>",
        "error=true",
        "is_error=true",
        "num_turns=true",
    ] {
        assert!(
            diagnostic.message().contains(field),
            "turn-ended diagnostic omitted {field}: {}",
            diagnostic.message()
        );
    }
    assert_eq!(
        diagnostic.code(),
        "swallowtail.claude-agent.sdk.provider_failed"
    );
    assert!(
        diagnostic
            .message()
            .contains("stderr: <redacted> failed result stderr")
    );
    assert!(!diagnostic.message().contains("provider error text"));
}

#[test]
fn billing_status_produces_an_entitlement_specific_diagnostic() {
    use swallowtail_core::{FailureKind, FailureOrigin, FailureRecovery};
    let status = terminal_status(SdkScenario::TurnEndedBilling402);
    let TerminalStatus::ProviderFailed(diagnostic) = status else {
        panic!("billing turn end must be provider failure, got {status:?}");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.claude-agent.sdk.provider_billing_unavailable"
    );
    for field in [
        "apiErrorStatus=402",
        "terminalReason=api_error",
        "rateLimitStatus=rejected",
    ] {
        assert!(
            diagnostic.message().contains(field),
            "billing diagnostic omitted {field}: {}",
            diagnostic.message()
        );
    }
    assert!(
        diagnostic.message().contains("stderr: <redacted>"),
        "billing diagnostic must carry its redacted stderr tail: {}",
        diagnostic.message()
    );
    assert!(!diagnostic.message().contains("private rejection detail"));
    let classification = diagnostic.failure_classification();
    assert_eq!(classification.origin(), FailureOrigin::Provider);
    assert_eq!(classification.kind(), FailureKind::EntitlementUnavailable);
    assert_eq!(
        classification.recovery(),
        FailureRecovery::ConfigurationChangeRequired
    );
}

#[test]
fn mixed_statuses_stay_explicitly_ambiguous_without_quota_claims() {
    use swallowtail_core::{FailureKind, FailureOrigin, FailureRecovery};
    for (scenario, code, summary, status_field) in [
        (
            SdkScenario::TurnEndedMixed400,
            "swallowtail.claude-agent.sdk.provider_invalid_request_or_spend_limit",
            "invalid request or spend limit",
            "apiErrorStatus=400",
        ),
        (
            SdkScenario::TurnEndedMixed429,
            "swallowtail.claude-agent.sdk.provider_rate_or_spend_limit",
            "rate or spend limit",
            "apiErrorStatus=429",
        ),
    ] {
        let status = terminal_status(scenario);
        let TerminalStatus::ProviderFailed(diagnostic) = status else {
            panic!("{scenario:?} must be provider failure, got {status:?}");
        };
        assert_eq!(diagnostic.code(), code);
        assert!(
            diagnostic.message().contains(summary),
            "{scenario:?} must name its ambiguity: {}",
            diagnostic.message()
        );
        assert!(
            diagnostic.message().contains(status_field),
            "{scenario:?} must carry its status: {}",
            diagnostic.message()
        );
        assert!(
            diagnostic.message().contains("terminalReason=api_error"),
            "{scenario:?} must carry its terminal reason: {}",
            diagnostic.message()
        );
        let classification = diagnostic.failure_classification();
        assert_eq!(classification.origin(), FailureOrigin::Provider);
        assert_eq!(classification.kind(), FailureKind::Unknown);
        assert_eq!(classification.recovery(), FailureRecovery::Unknown);
    }
}

#[test]
fn unlisted_status_stays_generic_without_new_evidence() {
    use swallowtail_core::{FailureKind, FailureOrigin, FailureRecovery};
    let status = terminal_status(SdkScenario::TurnEndedUnknownStatus);
    let TerminalStatus::ProviderFailed(diagnostic) = status else {
        panic!("unlisted status must be provider failure, got {status:?}");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.claude-agent.sdk.provider_failed"
    );
    assert!(
        diagnostic.message().contains("apiErrorStatus=503"),
        "unlisted status must still be visible: {}",
        diagnostic.message()
    );
    let classification = diagnostic.failure_classification();
    assert_eq!(classification.origin(), FailureOrigin::Provider);
    assert_eq!(classification.kind(), FailureKind::Unknown);
    assert_eq!(classification.recovery(), FailureRecovery::Unknown);
}

#[test]
fn malformed_structured_facts_fail_closed_without_provider_prose() {
    for scenario in [
        SdkScenario::TurnEndedMalformedStatus,
        SdkScenario::TurnEndedMalformedReason,
    ] {
        let status = terminal_status(scenario);
        let TerminalStatus::RuntimeFailed(diagnostic) = &status else {
            panic!("{scenario:?} must fail closed, got {status:?}");
        };
        assert!(
            !diagnostic.message().contains("API Error"),
            "{scenario:?} diagnostic must not carry provider prose: {}",
            diagnostic.message()
        );
        assert!(
            !diagnostic.message().contains("402"),
            "{scenario:?} diagnostic must not carry the rejected value: {}",
            diagnostic.message()
        );
    }
}

#[test]
fn tool_admission_crosses_the_wire_as_a_correlated_consumer_decision() {
    let host = host_id("claude-agent-sdk.fixture.admission");
    let fixture = SdkFixtureHost::new(SdkScenario::ToolAdmission);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let mut session =
        block_on(prepared.open_session(services.clone())).expect("SDK sidecar session opens");
    let mut turn = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
        .expect("SDK sidecar turn starts");

    let mut callbacks = turn.take_callbacks().expect("turn exposes tool admission");
    let mut requests = callbacks
        .take_requests()
        .expect("admission requests are available once");
    let request = block_on(requests.next())
        .expect("one admission request arrives")
        .expect("admission request is healthy");
    let swallowtail_runtime::CallbackRequestKind::Extension(extension) = request.kind() else {
        panic!("tool admission is a route-local extension callback");
    };
    assert_eq!(
        extension.namespace().as_str(),
        "claude-agent-sdk/can-use-tool"
    );
    let payload: serde_json::Value =
        serde_json::from_slice(extension.payload()).expect("payload is JSON");
    assert_eq!(payload, serde_json::json!({"toolName": "Read"}));
    assert_eq!(payload.as_object().expect("object").len(), 1);

    block_on(
        callbacks.responder().respond(CallbackResponse::new(
            request.callback_id().clone(),
            swallowtail_runtime::RuntimeTurnId::new("turn-1").expect("valid turn"),
            CallbackResult::Success(
                CallbackPayload::new(br#"{"decision":"allow"}"#.to_vec(), 4096)
                    .expect("payload fits the bound"),
            ),
        )),
    )
    .expect("admission response reaches the sidecar");

    let decision = fixture
        .inputs()
        .into_iter()
        .find(|value| value["type"] == "callback_response")
        .expect("a callback response crosses the wire");
    assert_eq!(decision["id"], "cb-1");
    assert_eq!(decision["decision"], "allow");
    let _ = block_on(session.close(cleanup_request(), services_for_cleanup.clone()));
}

#[test]
fn bash_admission_crosses_the_wire_with_its_bounded_command_view() {
    let host = host_id("claude-agent-sdk.fixture.bash-admission");
    let fixture = SdkFixtureHost::new(SdkScenario::BashAdmission);
    let profile = ClaudeAgentSdkSessionProfile::new(
        [ClaudeAgentSdkTool::Bash],
        ClaudeAgentSdkPermissionMode::Default,
    )
    .expect("explicit Bash profile is admissible");
    let prepared = prepared_session_for(host.clone(), profile);
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let mut session =
        block_on(prepared.open_session(services.clone())).expect("SDK sidecar session opens");
    let mut turn = block_on(session.start_turn(turn_request("turn-1", "inspect it"), services))
        .expect("SDK sidecar turn starts");

    let mut callbacks = turn.take_callbacks().expect("turn exposes tool admission");
    let mut requests = callbacks
        .take_requests()
        .expect("admission requests are available once");
    let request = block_on(requests.next())
        .expect("one admission request arrives")
        .expect("admission request is healthy");
    let swallowtail_runtime::CallbackRequestKind::Extension(extension) = request.kind() else {
        panic!("tool admission is a route-local extension callback");
    };
    let payload: serde_json::Value =
        serde_json::from_slice(extension.payload()).expect("payload is JSON");
    assert_eq!(payload["toolName"], "Bash");
    assert_eq!(payload["command"], "git status --porcelain");
    assert_eq!(payload["commandByteLength"], 22);
    assert_eq!(payload["description"], "inspect the working tree");
    assert_eq!(payload["truncated"], false);
    assert_eq!(payload.as_object().expect("object").len(), 5);

    block_on(
        callbacks.responder().respond(CallbackResponse::new(
            request.callback_id().clone(),
            swallowtail_runtime::RuntimeTurnId::new("turn-1").expect("valid turn"),
            CallbackResult::Success(
                CallbackPayload::new(br#"{"decision":"allow"}"#.to_vec(), 4096)
                    .expect("payload fits the bound"),
            ),
        )),
    )
    .expect("admission response reaches the sidecar");

    let decision = fixture
        .inputs()
        .into_iter()
        .find(|value| value["type"] == "callback_response")
        .expect("a callback response crosses the wire");
    assert_eq!(decision["id"], "cb-1");
    assert_eq!(decision["decision"], "allow");
    let _ = block_on(session.close(cleanup_request(), services_for_cleanup));
}

#[test]
fn admission_requests_beyond_the_bound_fail_rather_than_buffer() {
    let status = terminal_status(SdkScenario::ToolAdmissionOverflow);
    let TerminalStatus::RuntimeFailed(diagnostic) = &status else {
        panic!("overflowing admission must fail the turn, got {status:?}");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.claude-agent.sdk.tool_admission_invalid"
    );
}

#[test]
fn events_outside_an_active_turn_fail_closed() {
    let host = host_id("claude-agent-sdk.fixture.unsolicited");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let session = block_on(prepared.open_session(services)).expect("SDK sidecar session opens");
    // No turn exists at injection, so there is no consumer event stream that
    // can receive this frame.
    fixture.emit(serde_json::json!({"type": "event", "event": "output_delta", "delta": "x"}));
    let outcome = block_on(session.close(cleanup_request(), services_for_cleanup.clone()));
    let swallowtail_runtime::CleanupOutcome::Degraded(diagnostic) = outcome else {
        panic!("an unsolicited event breaks the wire, so close is degraded: {outcome:?}");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.claude-agent.sdk.close_root_only_degraded"
    );
    assert!(
        fixture
            .cleanup_events()
            .contains(&crate::sdk_support::CleanupEvent::ProcessForceStop)
    );
    let cleanup = fixture.cleanup_events();
    let positions = [
        crate::sdk_support::CleanupEvent::ProcessForceStop,
        crate::sdk_support::CleanupEvent::ProcessWait,
        crate::sdk_support::CleanupEvent::ResourceRelease,
        crate::sdk_support::CleanupEvent::CredentialRelease,
    ]
    .map(|event| {
        cleanup
            .iter()
            .position(|candidate| *candidate == event)
            .unwrap_or_else(|| panic!("cleanup event {event:?} is missing from {cleanup:?}"))
    });
    assert!(
        positions.windows(2).all(|pair| pair[0] < pair[1]),
        "process and lease cleanup must settle in order: {cleanup:?}"
    );
    assert!(
        fixture.spawned_registered_courier().is_none(),
        "the fixture opened no registered courier to leave behind"
    );
    assert!(
        fixture
            .inputs()
            .iter()
            .all(|input| input["command"] != "query"),
        "no turn command was opened for consumer event delivery"
    );
    // The fixture's outer host owner joins any reservation-backed worker that
    // legitimately outlives its public cleanup result.
    fixture.reaper().shutdown();
}

#[test]
fn the_launch_surface_carries_only_opaque_host_owned_references() {
    let host = host_id("claude-agent-sdk.fixture.launch");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let session = block_on(prepared.open_session(services)).expect("SDK sidecar session opens");
    let environment = fixture.process_environment();
    assert_eq!(environment, ["claude-agent-sdk.fixture.environment"]);
    let inputs = fixture.inputs();
    let serialized = serde_json::to_string(&inputs).expect("inputs serialize");
    for forbidden in [
        "ANTHROPIC_API_KEY",
        "apiKeyHelper",
        "accessToken",
        "worker_jwt",
        "oauth",
    ] {
        assert!(
            !serialized.contains(forbidden),
            "no outbound record may carry {forbidden}"
        );
    }
    let _ = block_on(session.close(cleanup_request(), services_for_cleanup.clone()));
}

#[test]
fn an_admission_request_outside_the_read_only_set_never_reaches_the_consumer() {
    // The allow-list is enforced on both sides of the wire. Its arrival here is
    // a transport failure, not a decision to delegate to the consumer.
    let status = terminal_status(SdkScenario::UnadmittedToolAdmission);
    let TerminalStatus::RuntimeFailed(diagnostic) = &status else {
        panic!("an unadmitted tool must fail the turn, got {status:?}");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.claude-agent.sdk.admission_tool_unadmitted"
    );
}

#[test]
fn closing_a_session_with_a_live_turn_resolves_it_instead_of_waiting_on_its_deadline() {
    // Regression: the turn's host-deadline task waits on completion or expiry,
    // so close must end the turn before joining that task. Joining first waits
    // for an event close itself prevented.
    let host = host_id("claude-agent-sdk.fixture.close-live-turn");
    let fixture = SdkFixtureHost::new(SdkScenario::ToolAdmission);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let mut session =
        block_on(prepared.open_session(services.clone())).expect("SDK sidecar session opens");
    let mut turn = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
        .expect("SDK sidecar turn starts");
    let terminal = turn
        .take_terminal_outcome()
        .expect("terminal outcome exists");

    let outcome = block_on(session.close(cleanup_request(), services_for_cleanup.clone()));
    assert!(
        matches!(outcome, swallowtail_runtime::CleanupOutcome::Degraded(_)),
        "closing over a live turn still reports the tree honestly"
    );
    assert_eq!(block_on(terminal).status(), &TerminalStatus::Cancelled);
}

#[test]
fn a_turn_that_never_ends_resolves_on_the_host_deadline() {
    let host = host_id("claude-agent-sdk.fixture.turn-deadline");
    let fixture = SdkFixtureHost::new(SdkScenario::ToolAdmission);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let mut session =
        block_on(prepared.open_session(services.clone())).expect("SDK sidecar session opens");
    let mut turn = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
        .expect("SDK sidecar turn starts");
    // The open bound already passed; only the turn's own deadline fires here.
    fixture.fire_deadlines();
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    assert_eq!(terminal.status(), &TerminalStatus::TimedOut);
    // The terminal outcome never waits on the receipt, but the interrupt is
    // still requested afterwards.
    fixture.wait_for_command("interrupt");
    let _ = block_on(turn.close());
    let _ = block_on(session.close(cleanup_request(), services_for_cleanup.clone()));
}

#[test]
fn a_query_that_is_never_answered_returns_on_the_turn_deadline() {
    // The public start-turn future is raced against the caller's turn deadline,
    // so a sidecar that stops answering cannot hold it open.
    let host = host_id("claude-agent-sdk.fixture.query-hold");
    let fixture = SdkFixtureHost::new(SdkScenario::QueryHold);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let mut session =
        block_on(prepared.open_session(services.clone())).expect("SDK sidecar session opens");
    fixture.fire_deadlines();
    let Err(error) = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
    else {
        panic!("an unanswered query must return on the turn deadline");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude-agent.sdk.turn_deadline_elapsed"
    );
    let _ = block_on(session.close(cleanup_request(), services_for_cleanup));
}

#[test]
fn an_interrupt_receipt_that_never_arrives_still_returns_requested() {
    // Cancellation is a request, never a claim of provider truth: the wire
    // write is issued, and only the receipt is bounded.
    let host = host_id("claude-agent-sdk.fixture.interrupt-bound");
    let fixture = SdkFixtureHost::new(SdkScenario::ToolAdmission);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let mut session =
        block_on(prepared.open_session(services.clone())).expect("SDK sidecar session opens");
    let turn = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
        .expect("SDK sidecar turn starts");
    fixture.hold_responses();
    fixture.fire_deadlines();
    assert_eq!(
        block_on(turn.cancellation().request()).expect("cancellation is a bounded request"),
        swallowtail_runtime::CancellationAcknowledgement::Requested
    );
    assert!(
        fixture
            .inputs()
            .iter()
            .any(|value| value["command"] == "interrupt"),
        "the interrupt still crosses the wire"
    );
    let _ = block_on(session.close(cleanup_request(), services_for_cleanup));
}

#[test]
fn native_aborted_streaming_is_cancelled_only_after_a_caller_request() {
    for (cancel, reason, expected_cancelled) in [
        (true, "aborted_streaming", true),
        (false, "aborted_streaming", false),
        (true, "api_error", false),
        (false, "api_error", false),
    ] {
        let host = host_id("claude-agent-sdk.fixture.original-interrupt");
        let fixture = SdkFixtureHost::new(SdkScenario::ToolAdmission);
        let services = fixture.services(host.clone());
        let mut session =
            block_on(prepared_session(host).open_session(services.clone())).expect("open");
        let mut turn =
            block_on(session.start_turn(turn_request("turn", "owned"), services.clone()))
                .expect("turn");
        if cancel {
            block_on(turn.cancellation().request()).expect("caller cancellation");
        }
        fixture.emit(failed_native_result(reason));
        let terminal = block_on(turn.take_terminal_outcome().expect("terminal"));
        if expected_cancelled {
            assert_eq!(terminal.status(), &TerminalStatus::Cancelled);
        } else {
            assert!(matches!(
                terminal.status(),
                TerminalStatus::ProviderFailed(_)
            ));
        }
        let _ = block_on(turn.close());
        let _ = block_on(session.close(cleanup_request(), services));
    }
}

fn failed_native_result(reason: &str) -> serde_json::Value {
    let presence: serde_json::Map<String, serde_json::Value> =
        crate::sdk_support::SDK_RESULT_FIELD_NAMES
            .iter()
            .map(|name| ((*name).to_owned(), serde_json::json!(true)))
            .collect();
    serde_json::json!({"type":"event","event":"turn_ended", "subtype":"error_during_execution",
        "stopReason":"error_during_execution","isError":true,"numTurns":1,"durationMs":1,
        "errorTextPresent":true,"errorTextType":"array","usage":{"inputTokens":0,"outputTokens":0,"cacheReadInputTokens":0,"cacheWriteInputTokens":0},
        "resultFieldPresence":presence,"apiErrorStatus":null,"terminalReason":reason,"rateLimitStatus":null})
}

#[test]
fn timed_out_turn_drains_one_abort_terminal_for_cooperative_close_only() {
    for tail in ["abort", "duplicate", "unexpected_output"] {
        let host = host_id("claude-agent-sdk.fixture.deadline-drain");
        let fixture = SdkFixtureHost::new(SdkScenario::ToolAdmission);
        let services = fixture.services(host.clone());
        let mut session =
            block_on(prepared_session(host).open_session(services.clone())).expect("open");
        let mut turn = block_on(session.start_turn(
            turn_request("turn", "owned").with_deadline(swallowtail_runtime::Deadline::at(
                swallowtail_runtime::MonotonicInstant::from_ticks(1_100),
            )),
            services.clone(),
        ))
        .expect("turn");
        // Query acceptance precedes asynchronous native start dispatch. Hold
        // virtual time until the intended already-started interleaving exists.
        fixture
            .emit_and_wait_until_dispatched(serde_json::json!({"type":"event","event":"progress"}));
        fixture.advance_time_to_ticks(1_100);
        let terminal = block_on(turn.take_terminal_outcome().expect("terminal"));
        assert_eq!(terminal.status(), &TerminalStatus::TimedOut);
        let _ = block_on(turn.close());
        if tail == "unexpected_output" {
            fixture.emit_and_wait_until_dispatched(
                serde_json::json!({"type":"event","event":"output_delta","delta":"late"}),
            );
        } else {
            fixture.emit_and_wait_until_dispatched(failed_native_result("aborted_streaming"));
            if tail == "duplicate" {
                fixture.emit_and_wait_until_dispatched(failed_native_result("aborted_streaming"));
            }
        }
        // A new cleanup deadline, not the already-expired turn deadline.
        let outcome = block_on(session.close(cleanup_request(), services));
        let diagnostic = outcome
            .diagnostic()
            .expect("fixture reports root-only completion");
        assert_eq!(
            diagnostic
                .message()
                .contains("close_evidence: nativeExitObserved=true"),
            tail == "abort",
            "{tail}: {outcome:?}"
        );
    }
}

#[test]
fn timed_out_recipient_blocks_reuse_until_its_exact_terminal_is_drained() {
    for tail in ["drained", "missing", "duplicate", "raced"] {
        let host = host_id("claude-agent-sdk.fixture.deadline-reuse");
        let fixture = SdkFixtureHost::new(SdkScenario::ToolAdmission);
        let services = fixture.services(host.clone());
        let mut session =
            block_on(prepared_session(host).open_session(services.clone())).expect("open");
        let mut first = block_on(session.start_turn(
            turn_request("old", "owned").with_deadline(swallowtail_runtime::Deadline::at(
                swallowtail_runtime::MonotonicInstant::from_ticks(1_100),
            )),
            services.clone(),
        ))
        .expect("old turn");
        let first_events = first.take_events().expect("old events");
        // Query acceptance precedes asynchronous native start dispatch. Hold
        // virtual time until the intended already-started interleaving exists.
        fixture
            .emit_and_wait_until_dispatched(serde_json::json!({"type":"event","event":"progress"}));
        fixture.advance_time_to_ticks(1_100);
        let outcome = block_on(first.take_terminal_outcome().expect("old terminal"));
        assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
        assert!(
            first.take_terminal_outcome().is_none(),
            "public outcome is single-use"
        );
        let _ = block_on(first.close());
        fixture.wait_for_command("interrupt");
        let refused =
            block_on(session.start_turn(turn_request("premature", "owned"), services.clone()));
        assert_eq!(
            refused
                .err()
                .expect("pending drain is busy")
                .diagnostic()
                .code(),
            "swallowtail.claude-agent.sdk.turn_active"
        );
        assert_eq!(
            fixture
                .inputs()
                .iter()
                .filter(|v| v["command"] == "query")
                .count(),
            1
        );
        if tail != "missing" {
            if tail == "raced" {
                let gate = std::sync::Arc::new(std::sync::Barrier::new(2));
                let peer = fixture.clone();
                let release = gate.clone();
                let writer = std::thread::spawn(move || {
                    release.wait();
                    peer.emit_and_wait_until_dispatched(failed_native_result("aborted_streaming"));
                });
                gate.wait();
                let raced =
                    block_on(session.start_turn(turn_request("racing", "owned"), services.clone()));
                writer.join().expect("terminal writer joins");
                if let Err(error) = &raced {
                    assert_eq!(
                        error.diagnostic().code(),
                        "swallowtail.claude-agent.sdk.turn_active"
                    );
                }
                if let Ok(mut next) = raced {
                    fixture.emit_and_wait_until_dispatched(completed_native_result());
                    assert_eq!(
                        block_on(next.take_terminal_outcome().expect("new terminal")).status(),
                        &TerminalStatus::Completed
                    );
                    let _ = block_on(next.close());
                }
            } else {
                fixture.emit_and_wait_until_dispatched(failed_native_result("aborted_streaming"));
            }
            if tail == "duplicate" {
                fixture.emit_and_wait_until_dispatched(failed_native_result("aborted_streaming"));
                assert!(
                    block_on(session.start_turn(turn_request("stale", "owned"), services.clone()))
                        .is_err()
                );
                assert_eq!(
                    fixture
                        .inputs()
                        .iter()
                        .filter(|v| v["command"] == "query")
                        .count(),
                    1
                );
            } else {
                let mut next = block_on(
                    session.start_turn(turn_request("genuine-next", "owned"), services.clone()),
                )
                .expect("drained recipient can be replaced");
                fixture.emit_and_wait_until_dispatched(completed_native_result());
                assert_eq!(
                    block_on(next.take_terminal_outcome().expect("next terminal")).status(),
                    &TerminalStatus::Completed
                );
                let _ = block_on(next.close());
            }
        }
        assert_eq!(
            outcome.status(),
            &TerminalStatus::TimedOut,
            "old outcome never changes"
        );
        let old_events = block_on(futures_util::StreamExt::collect::<Vec<_>>(first_events));
        assert!(
            !old_events.iter().any(|e| e.as_ref().is_ok_and(|e| matches!(
                e.kind(),
                swallowtail_runtime::RuntimeEventKind::OutputDelta
            )))
        );
        let cleanup = block_on(session.close(cleanup_request(), services));
        assert!(
            matches!(cleanup, swallowtail_runtime::CleanupOutcome::Degraded(_)),
            "{tail}: {cleanup:?}"
        );
        assert!(
            fixture
                .cleanup_events()
                .contains(&crate::sdk_support::CleanupEvent::ProcessWait)
        );
    }
}

fn completed_native_result() -> serde_json::Value {
    let mut value = failed_native_result("unused");
    value["subtype"] = serde_json::json!("success");
    value["stopReason"] = serde_json::json!("success");
    value["isError"] = serde_json::json!(false);
    value["terminalReason"] = serde_json::Value::Null;
    value
}

#[test]
fn ordinary_completed_recipient_allows_normal_subsequent_reuse() {
    let host = host_id("claude-agent-sdk.fixture.normal-reuse");
    let fixture = SdkFixtureHost::new(SdkScenario::ToolAdmission);
    let services = fixture.services(host.clone());
    let mut session =
        block_on(prepared_session(host).open_session(services.clone())).expect("open");
    for id in ["first", "next"] {
        let mut turn = block_on(session.start_turn(turn_request(id, "owned"), services.clone()))
            .expect("normal turn");
        fixture.emit_and_wait_until_dispatched(completed_native_result());
        assert_eq!(
            block_on(turn.take_terminal_outcome().expect("terminal")).status(),
            &TerminalStatus::Completed
        );
        let _ = block_on(turn.close());
    }
    assert_eq!(
        fixture
            .inputs()
            .iter()
            .filter(|v| v["command"] == "query")
            .count(),
        2
    );
    let _ = block_on(session.close(cleanup_request(), services));
    assert!(
        fixture
            .cleanup_events()
            .contains(&crate::sdk_support::CleanupEvent::ProcessWait)
    );
}
