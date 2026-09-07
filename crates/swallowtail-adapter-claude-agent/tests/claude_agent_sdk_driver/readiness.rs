use crate::host_id;
use crate::sdk_support::{
    CleanupEvent, SDK_RESULT_FIELD_NAMES, SanitizedCaptureJournal, SanitizedWireCapture,
    SdkFixtureHost, SdkScenario, captured_services, cleanup_request, prepared_session,
    prepared_session_with, record_open_failure, record_success, turn_request,
};
use futures_executor::block_on;
use futures_util::StreamExt;
use serde_json::Value;
use std::fs;
use std::io;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use swallowtail_core::Diagnostic;
use swallowtail_runtime::{
    DebugObservation, DebugObservationKind, DiagnosticObserver, InteractiveSessionHandle,
    ProcessExit, SessionOptions,
};

const CAPTURE_CHILD_ENV: &str = "SWALLOWTAIL_CARD100_CAPTURE_CHILD";
const CAPTURE_JOURNAL_ENV: &str = "SWALLOWTAIL_CARD100_CAPTURE_JOURNAL";
static NEXT_CAPTURE_JOURNAL: AtomicU64 = AtomicU64::new(0);

#[derive(Default)]
struct RecordingObserver {
    debug: Mutex<Vec<DebugObservation>>,
}

impl RecordingObserver {
    fn observations(&self) -> Vec<DebugObservation> {
        self.debug
            .lock()
            .expect("recording observer lock poisoned")
            .clone()
    }
}

impl DiagnosticObserver for RecordingObserver {
    fn observe(&self, _diagnostic: &Diagnostic) {}

    fn observe_debug(&self, observation: &DebugObservation) {
        self.debug
            .lock()
            .expect("recording observer lock poisoned")
            .push(observation.clone());
    }
}

struct ReapOnDrop {
    child: Option<Child>,
}

impl ReapOnDrop {
    fn new(child: Child) -> Self {
        Self { child: Some(child) }
    }

    fn kill_and_wait(&mut self) -> io::Result<ExitStatus> {
        let child = self.child.as_mut().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "capture wrapper already reaped")
        })?;
        let _ = child.kill();
        let status = child.wait()?;
        self.child = None;
        Ok(status)
    }
}

impl Drop for ReapOnDrop {
    fn drop(&mut self) {
        if let Some(child) = self.child.as_mut() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

fn open_failure(scenario: SdkScenario) -> (String, Vec<CleanupEvent>) {
    let host = host_id("claude-agent-sdk.fixture.readiness");
    let fixture = SdkFixtureHost::new(scenario);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let Err(error) = block_on(prepared.open_session(services)) else {
        panic!("open must fail closed");
    };
    (
        error.diagnostic().code().to_owned(),
        fixture.cleanup_events(),
    )
}

#[test]
fn wrapper_death_preserves_partial_capture_journal() {
    if std::env::var_os(CAPTURE_CHILD_ENV).is_some() {
        let path = std::env::var_os(CAPTURE_JOURNAL_ENV).expect("journal path is passed");
        let mut journal = SanitizedCaptureJournal::create(path).expect("journal is created");
        let capture = SanitizedWireCapture {
            open_sidecar_code: Some("construction_failed".to_owned()),
            ..SanitizedWireCapture::default()
        };
        journal
            .append_snapshot(&capture)
            .expect("partial capture is persisted");
        loop {
            thread::sleep(Duration::from_millis(25));
        }
    }

    let sequence = NEXT_CAPTURE_JOURNAL.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "swallowtail-card100-capture-{}-{}.jsonl",
        std::process::id(),
        sequence
    ));
    let mut child = ReapOnDrop::new(
        Command::new(std::env::current_exe().expect("test binary path"))
            .arg("wrapper_death_preserves_partial_capture_journal")
            .env(CAPTURE_CHILD_ENV, "1")
            .env(CAPTURE_JOURNAL_ENV, &path)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("capture wrapper child starts"),
    );

    let deadline = Instant::now() + Duration::from_secs(5);
    while !path.exists() {
        assert!(
            Instant::now() < deadline,
            "capture wrapper did not persist before the kill"
        );
        thread::sleep(Duration::from_millis(10));
    }
    let status = child
        .kill_and_wait()
        .expect("capture wrapper status is observed");
    assert!(!status.success(), "killed wrapper must not report success");

    let line = fs::read_to_string(&path)
        .expect("partial capture remains readable after wrapper death")
        .lines()
        .next()
        .expect("partial capture has one durable record")
        .to_owned();
    let record: Value = serde_json::from_str(&line).expect("durable record is JSON");
    assert_eq!(
        record["openSidecarCode"],
        Value::String("construction_failed".to_owned())
    );
    assert_eq!(record["stderrTailPresent"], Value::Bool(false));
    fs::remove_file(path).expect("capture journal is removed");
}

fn first_turn_failure(scenario: SdkScenario) -> String {
    let host = host_id("claude-agent-sdk.fixture.first-turn-readiness");
    let fixture = SdkFixtureHost::new(scenario);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let mut session = block_on(prepared.open_route_session(services.clone()))
        .expect("initialize-served open must succeed before first-turn evidence");
    let Err(error) = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
    else {
        panic!("first-turn readiness must fail closed");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude-agent.sdk.query_rejected"
    );
    let code = error.diagnostic().message().rsplit_once(": ").map_or_else(
        || error.diagnostic().message().to_owned(),
        |(_, code)| code.to_owned(),
    );
    let _ = block_on(Box::new(session).close(cleanup_request(), cleanup_services));
    code
}

#[test]
fn first_party_without_subscription_evidence_still_opens() {
    let host = host_id("claude-agent-sdk.fixture.account-observations");
    let fixture = SdkFixtureHost::new(SdkScenario::AccountNotSubscription);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let session = block_on(prepared.open_route_session(services))
        .expect("first-party open must not gate on subscriptionType");
    let outcome = block_on(Box::new(session).close(cleanup_request(), cleanup_services));
    assert!(matches!(
        outcome,
        swallowtail_runtime::CleanupOutcome::Clean
            | swallowtail_runtime::CleanupOutcome::Degraded(_)
    ));
}

#[test]
fn a_delegated_cloud_provider_is_not_first_party_readiness() {
    let (code, _) = open_failure(SdkScenario::AccountNotFirstParty);
    assert_eq!(code, "swallowtail.claude-agent.sdk.account_not_first_party");
}

#[test]
fn account_identity_fields_are_refused_rather_than_recorded() {
    let (code, _) = open_failure(SdkScenario::AccountIdentityLeak);
    assert_eq!(code, "swallowtail.claude-agent.sdk.account_not_ready");
}

#[test]
fn command_rejections_keep_their_fixed_sidecar_code() {
    let host = host_id("claude-agent-sdk.fixture.command-rejections");
    let fixture = SdkFixtureHost::new(SdkScenario::QueryRejected);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let mut session = block_on(prepared.open_route_session(services.clone()))
        .expect("session opens before query rejection");
    let Err(error) = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
    else {
        panic!("query rejection must fail");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude-agent.sdk.query_rejected"
    );
    assert!(error.diagnostic().message().ends_with(": turn_active"));
    let _ = block_on(Box::new(session).close(cleanup_request(), cleanup_services));

    let host = host_id("claude-agent-sdk.fixture.interrupt-rejection");
    let fixture = SdkFixtureHost::new(SdkScenario::InterruptRejected);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let mut session = block_on(prepared.open_route_session(services.clone()))
        .expect("session opens before interrupt rejection");
    let turn = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
        .expect("turn starts before interrupt rejection");
    let error = block_on(turn.cancellation().request()).expect_err("interrupt must reject");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude-agent.sdk.interrupt_rejected"
    );
    assert!(error.diagnostic().message().ends_with(": interrupt_failed"));
    let _ = block_on(Box::new(session).close(cleanup_request(), cleanup_services));

    let host = host_id("claude-agent-sdk.fixture.close-rejection-code");
    let fixture = SdkFixtureHost::new(SdkScenario::CloseRejected);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let session = block_on(prepared.open_route_session(services))
        .expect("session opens before close rejection");
    let outcome = block_on(Box::new(session).close(cleanup_request(), cleanup_services));
    let swallowtail_runtime::CleanupOutcome::Failed(diagnostic) = outcome else {
        panic!("close rejection must fail cleanup");
    };
    assert_eq!(
        diagnostic.code(),
        "swallowtail.claude-agent.sdk.close_rejected"
    );
    assert!(diagnostic.message().ends_with(": invalid_command"));
}

#[test]
fn first_turn_rejection_makes_retry_terminal_without_replay_or_provider_work() {
    let host = host_id("claude-agent-sdk.fixture.first-turn-terminal");
    let fixture = SdkFixtureHost::new(SdkScenario::UnsupportedModel);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let mut session = block_on(prepared.open_route_session(services.clone()))
        .expect("session opens before first-turn model rejection");

    let Err(first_error) =
        block_on(session.start_turn(turn_request("turn-1", "read it"), services.clone()))
    else {
        panic!("unsupported model must reject the first turn");
    };
    assert_eq!(
        first_error.diagnostic().code(),
        "swallowtail.claude-agent.sdk.query_rejected"
    );
    assert!(
        first_error
            .diagnostic()
            .message()
            .ends_with(": supported_model_rejected")
    );
    let inputs_before_retry = fixture.inputs();
    assert_eq!(
        inputs_before_retry
            .iter()
            .filter(|input| input["command"] == "query")
            .count(),
        1
    );
    let acquisitions_before_retry = fixture.credential_acquisitions();

    let Err(retry_error) = block_on(session.start_turn(turn_request("turn-2", "retry"), services))
    else {
        panic!("a rejected session must stay terminal");
    };
    assert_eq!(
        retry_error.diagnostic().code(),
        "swallowtail.claude-agent.sdk.session_rejected_terminal"
    );
    assert!(
        retry_error
            .diagnostic()
            .message()
            .ends_with(": supported_model_rejected")
    );
    assert_eq!(
        fixture.inputs(),
        inputs_before_retry,
        "retry did not replay the wire"
    );
    assert_eq!(
        fixture.credential_acquisitions(),
        acquisitions_before_retry,
        "retry did not perform provider work"
    );

    let outcome = block_on(Box::new(session).close(cleanup_request(), cleanup_services));
    assert!(matches!(
        outcome,
        swallowtail_runtime::CleanupOutcome::Clean
            | swallowtail_runtime::CleanupOutcome::Degraded(_)
    ));
    fixture.wait_for_cleanup(CleanupEvent::ResourceRelease);
    fixture.wait_for_cleanup(CleanupEvent::CredentialRelease);
    fixture.reaper().shutdown();
}

#[test]
fn off_point_identity_resource_and_tool_sets_fail_closed() {
    for scenario in [
        SdkScenario::IdentityMismatch,
        SdkScenario::CwdMismatch,
        SdkScenario::ToolsWidened,
    ] {
        let (code, _) = open_failure(scenario);
        assert_eq!(
            code, "swallowtail.claude-agent.sdk.open_mismatch",
            "{scenario:?} must fail closed"
        );
    }
}

#[test]
fn an_interrupt_receipt_requires_a_runtime_advertised_capability() {
    let host = host_id("claude-agent-sdk.fixture.receipt");
    let fixture = SdkFixtureHost::new(SdkScenario::UnadvertisedInterruptReceipt);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let mut session =
        block_on(prepared.open_session(services.clone())).expect("SDK sidecar session opens");
    let turn = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
        .expect("SDK sidecar turn starts");
    let error = block_on(turn.cancellation().request())
        .expect_err("an unadvertised receipt must fail closed");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude-agent.sdk.interrupt_receipt_unadvertised"
    );
    let _ = block_on(session.close(cleanup_request(), services_for_cleanup.clone()));
}

#[test]
fn preparation_admits_no_session_options_in_this_layer() {
    let host = host_id("claude-agent-sdk.fixture.options");
    let options = SessionOptions::default().with_reasoning_mode(
        swallowtail_core::ReasoningMode::new("high").expect("valid reasoning mode"),
    );
    let Err(failure) = prepared_session_with(host, options) else {
        panic!("reasoning selection is a later layer");
    };
    assert_eq!(
        failure.diagnostic().safe().code(),
        "swallowtail.claude-agent.sdk.preparation.unsupported_options"
    );
}

#[test]
fn a_canonical_effective_model_is_accepted_and_published() {
    let host = host_id("claude-agent-sdk.fixture.canonical-model");
    let fixture = SdkFixtureHost::new(SdkScenario::CanonicalModel);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let mut session =
        block_on(prepared.open_route_session(services.clone())).expect("session opens");
    assert_eq!(session.readiness_state(), "requested-with-supported-list");
    let mut turn = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
        .expect("first turn confirms init");
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    assert_eq!(
        terminal.status(),
        &swallowtail_runtime::TerminalStatus::Completed
    );
    let _ = block_on(turn.close());
    assert_eq!(session.requested_model(), "claude-sonnet-5");
    assert_eq!(session.effective_model(), "claude-sonnet-5-20250929");
    assert_eq!(session.readiness_state(), "confirmed");
    assert_eq!(session.node_version(), "22.23.2");
    assert_eq!(session.node_version_posture(), "Qualified");
    let _ = block_on(Box::new(session).close(cleanup_request(), cleanup_services));
}

#[test]
fn missing_and_unsupported_effective_models_fail_closed() {
    assert_eq!(
        first_turn_failure(SdkScenario::MissingModel),
        "model_missing"
    );
    assert_eq!(
        first_turn_failure(SdkScenario::UnsupportedModel),
        "supported_model_rejected"
    );
}

#[test]
fn model_qualification_evidence_reaches_the_recording_observer() {
    for (scenario, rejected, requested_membership, effective_membership) in [
        (SdkScenario::AliasOnly, true, true, false),
        (SdkScenario::CanonicalOnly, false, false, true),
        (SdkScenario::BothIds, false, true, true),
        (SdkScenario::NeitherIds, true, false, false),
    ] {
        let host = host_id("claude-agent-sdk.fixture.model-qualification-evidence");
        let fixture = SdkFixtureHost::new(scenario);
        let observer = Arc::new(RecordingObserver::default());
        let prepared = prepared_session(host.clone());
        let services = fixture
            .services(host)
            .with_diagnostic_observer(observer.clone());
        let cleanup_services = services.clone();
        let mut session = block_on(prepared.open_route_session(services.clone()))
            .expect("model qualification fixture opens");

        if rejected {
            let Err(error) =
                block_on(session.start_turn(turn_request("turn-1", "read it"), services))
            else {
                panic!("{scenario:?} must reject its effective model");
            };
            assert_eq!(
                error.diagnostic().code(),
                "swallowtail.claude-agent.sdk.supported_model_rejected"
            );
        } else {
            let mut turn =
                block_on(session.start_turn(turn_request("turn-1", "read it"), services))
                    .expect("listed effective model starts a turn");
            let terminal = block_on(
                turn.take_terminal_outcome()
                    .expect("terminal outcome exists"),
            );
            assert_eq!(
                terminal.status(),
                &swallowtail_runtime::TerminalStatus::Completed
            );
            let _ = block_on(turn.close());
        }

        let observations = observer.observations();
        if rejected {
            assert_eq!(
                observations.len(),
                1,
                "one qualification observation for {scenario:?}: {observations:?}"
            );
            let observation = &observations[0];
            assert_eq!(observation.kind(), DebugObservationKind::InterfaceVersion);
            assert_eq!(observation.route(), Some("claude-agent.sdk"));
            assert_eq!(observation.stage(), Some("first-turn-model-qualification"));
            assert_eq!(
                observation.correlated_code(),
                Some("supported_model_rejected")
            );
            assert!(!observation.detail_truncated());
            let detail: Value =
                serde_json::from_str(observation.detail()).expect("qualification detail is JSON");
            assert_eq!(
                detail,
                serde_json::json!({
                    "requestedModel": "claude-sonnet-5",
                    "effectiveModel": "claude-sonnet-5-20250929",
                    "catalogueSize": 1,
                    "catalogueDigest": if scenario == SdkScenario::AliasOnly {
                        "sha256:4b07d9a517e35f5852d8385b4a192070"
                    } else {
                        "sha256:f9ef6eedd766691f041bc3387784d8dd"
                    },
                    "requestedMembership": requested_membership,
                    "effectiveMembership": effective_membership,
                    "querySource": "sdk.query",
                    "phase": "first-turn-model-qualification",
                    "declaredSdkVersion": "0.3.259",
                    "loadedSdkVersion": "0.3.259",
                    "nativeVersion": "2.1.259"
                })
            );
            let detail_text = observation.detail();
            assert!(!detail_text.contains("read it"));
            assert!(!detail_text.contains("fixture-secret"));
            assert!(!detail_text.contains("/fixture/"));
        } else {
            assert!(
                observations.is_empty(),
                "accepted model qualification must not emit rejection evidence"
            );
        }

        let _ = block_on(Box::new(session).close(cleanup_request(), cleanup_services));
    }
}

#[test]
fn an_empty_supported_model_list_does_not_reject_first_turn_evidence() {
    let host = host_id("claude-agent-sdk.fixture.empty-supported-models");
    let fixture = SdkFixtureHost::new(SdkScenario::EmptySupportedModels);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let mut session = block_on(prepared.open_route_session(services.clone()))
        .expect("empty supported-model list is unavailable, not rejecting");
    let mut turn = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
        .expect("first-turn model evidence is accepted without an available list");
    let terminal = block_on(
        turn.take_terminal_outcome()
            .expect("terminal outcome exists"),
    );
    assert_eq!(
        terminal.status(),
        &swallowtail_runtime::TerminalStatus::Completed
    );
    let _ = block_on(turn.close());
    assert_eq!(session.effective_model(), "claude-sonnet-5");
    let _ = block_on(Box::new(session).close(cleanup_request(), cleanup_services));
}

#[test]
fn first_turn_init_missing_and_initialization_failure_stay_distinct() {
    assert_eq!(first_turn_failure(SdkScenario::InitMissing), "init_missing");
    assert_eq!(
        first_turn_failure(SdkScenario::InitializationFailed),
        "initialization_failed"
    );
}

#[test]
fn a_newer_node_that_passes_the_sidecar_floor_is_unverified_newer() {
    let host = host_id("claude-agent-sdk.fixture.newer-node");
    let fixture = SdkFixtureHost::new(SdkScenario::NewerNode);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let cleanup_services = services.clone();
    let session = block_on(prepared.open_route_session(services)).expect("newer Node opens");
    assert_eq!(session.node_version(), "26.7.0");
    assert_eq!(session.node_version_posture(), "UnverifiedNewer");
    let _ = block_on(Box::new(session).close(cleanup_request(), cleanup_services));
}

#[test]
fn open_rejection_surfaces_the_fixed_sidecar_code_without_raw_details() {
    let host = host_id("claude-agent-sdk.fixture.open-rejection");
    let fixture = SdkFixtureHost::new(SdkScenario::OpenRejected);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let Err(error) = block_on(prepared.open_session(services)) else {
        panic!("open must fail closed");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude-agent.sdk.open_rejected"
    );
    assert!(
        error
            .diagnostic()
            .message()
            .ends_with(": construction_failed")
    );
    for forbidden in ["/fixture/", "@example", "token", "organization"] {
        assert!(!error.diagnostic().message().contains(forbidden));
    }
}

#[test]
fn the_live_capture_path_retains_message_fields_and_close_evidence_provider_free() {
    let host = host_id("claude-agent-sdk.fixture.capture-open-rejection");
    let fixture = SdkFixtureHost::new(SdkScenario::OpenRejected);
    let (services, capture) = captured_services(&fixture, host.clone());
    let prepared = prepared_session(host);
    let Err(error) = block_on(prepared.open_session(services)) else {
        panic!("construction rejection must fail open");
    };
    let rejected = record_open_failure(&error, &capture);
    assert_eq!(
        rejected.route_code.as_deref(),
        Some("swallowtail.claude-agent.sdk.open_rejected")
    );
    assert!(
        rejected
            .diagnostic_message
            .as_deref()
            .is_some_and(|message| message.ends_with(": construction_failed")),
        "route diagnostic must retain the fixed sidecar subcode"
    );
    assert_eq!(
        rejected.wire.open_sidecar_code.as_deref(),
        Some("construction_failed")
    );

    let host = host_id("claude-agent-sdk.fixture.capture-complete");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let (services, capture) = captured_services(&fixture, host.clone());
    let prepared = prepared_session(host);
    let cleanup_services = services.clone();
    let mut session =
        block_on(prepared.open_route_session(services.clone())).expect("complete fixture opens");
    let mut turn = block_on(session.start_turn(turn_request("turn-1", "read it"), services))
        .expect("complete fixture starts one turn");
    let mut events = turn.take_events().expect("turn event stream");
    let terminal_future = turn.take_terminal_outcome().expect("terminal outcome");
    block_on(async { while events.next().await.is_some() {} });
    let terminal = block_on(terminal_future);
    let _ = block_on(turn.close());
    let cleanup = block_on(Box::new(session).close(cleanup_request(), cleanup_services));
    let complete = record_success(&terminal, &cleanup, &capture);

    assert!(matches!(
        complete.terminal_status,
        Some(swallowtail_runtime::TerminalStatus::Completed)
    ));
    assert_eq!(complete.terminal_diagnostic_code, None);
    assert!(
        complete.cleanup_outcome.as_ref().is_some_and(|outcome| {
            matches!(
                outcome,
                swallowtail_runtime::CleanupOutcome::Clean
                    | swallowtail_runtime::CleanupOutcome::Degraded(_)
            )
        }),
        "capture must retain a truthful cleanup posture"
    );
    assert_eq!(
        complete.cleanup_diagnostic_code.as_deref(),
        Some("swallowtail.claude-agent.sdk.close_root_only_degraded")
    );
    assert_eq!(
        complete.wire.stderr_tail.as_deref(),
        Some("<redacted>"),
        "sanitized stderr tail must be retained without raw text"
    );
    assert_eq!(
        complete.wire.result_fields.len(),
        SDK_RESULT_FIELD_NAMES.len()
    );
    for field in SDK_RESULT_FIELD_NAMES {
        assert!(
            complete.wire.result_fields.contains_key(*field),
            "result field presence map omitted {field}"
        );
    }
    for field in ["type", "subtype", "duration_ms", "is_error", "num_turns"] {
        assert_eq!(
            complete.wire.result_fields.get(field),
            Some(&true),
            "{field}"
        );
    }
    for field in ["duration_api_ms", "result", "errors", "uuid", "session_id"] {
        assert_eq!(
            complete.wire.result_fields.get(field),
            Some(&false),
            "{field}"
        );
    }
    assert_eq!(complete.wire.result_subtype.as_deref(), Some("success"));
    assert_eq!(complete.wire.result_is_error, Some(false));
    assert_eq!(complete.wire.result_num_turns, Some(1));
    assert_eq!(complete.wire.result_duration_ms, Some(7));
    assert_eq!(complete.wire.result_error_text_present, Some(false));
    assert_eq!(
        complete.wire.result_error_text_type.as_deref(),
        Some("absent")
    );
    assert_eq!(
        complete.wire.close_timeline,
        vec![
            "close_requested",
            "session_input_closed",
            "sdk_transport_close_ran",
            "native_join_exited"
        ]
    );
    assert_eq!(complete.wire.native_exit_event.as_deref(), Some("exit"));
    assert_eq!(complete.wire.native_exit_code, Some(0));
    assert_eq!(complete.wire.native_exit_signal, None);
    assert_eq!(complete.wire.native_join.as_deref(), Some("exited"));
    assert_eq!(complete.wire.native_exit_observed, Some(true));
    assert_eq!(
        complete.wire.root_exit.map(ProcessExit::code),
        Some(Some(0))
    );
}

#[test]
fn the_selected_model_crosses_the_wire_on_open() {
    let host = host_id("claude-agent-sdk.fixture.model");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    let session = block_on(prepared.open_session(services)).expect("SDK sidecar session opens");
    let open = fixture
        .inputs()
        .into_iter()
        .find(|value| value["command"] == "open")
        .expect("open command is sent");
    assert_eq!(open["params"]["model"], "claude-sonnet-5");
    // Open carries exactly the cwd, model, admitted tool set, and permission
    // mode. The default profile is the unchanged read-only one.
    assert_eq!(open["params"].as_object().expect("params").len(), 4);
    assert_eq!(
        open["params"]["tools"],
        serde_json::json!(["Read", "Glob", "Grep"])
    );
    assert_eq!(open["params"]["permissionMode"], "default");
    let _ = block_on(session.close(cleanup_request(), services_for_cleanup.clone()));
}

#[test]
fn an_open_that_never_reaches_readiness_expires_on_the_host_deadline() {
    // The sidecar holds its open response; only the host clock ends the wait.
    let host = host_id("claude-agent-sdk.fixture.open-deadline");
    let fixture = SdkFixtureHost::new(SdkScenario::OpenHold).with_immediate_time();
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let Err(error) = block_on(prepared.open_session(services)) else {
        panic!("an unbounded open must fail on the host deadline");
    };
    let code = error.diagnostic().code().to_owned();
    fixture.wait_for_cleanup(CleanupEvent::ProcessWait);
    fixture.release_process_hold();
    fixture.wait_for_cleanup(CleanupEvent::CredentialRelease);
    fixture.reaper().shutdown();
    assert_eq!(
        code,
        "swallowtail.claude-agent.sdk.open_cleanup_unconfirmed"
    );
}

#[cfg(windows)]
#[test]
fn windows_is_an_unsupported_platform_for_this_route() {
    use swallowtail_adapter_claude_agent::sdk::claude_agent_sdk_addable_route_descriptor;
    let host = host_id("claude-agent-sdk.fixture.windows");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let prepared = prepared_session(host.clone());
    let services = fixture.services(host);
    let services_for_cleanup = services.clone();
    assert_eq!(
        claude_agent_sdk_addable_route_descriptor(&services).availability(),
        swallowtail_core::AddableRouteAvailability::Unsupported
    );
    let Err(error) = block_on(prepared.open_session(services)) else {
        panic!("an unprovable descendant-tree platform must refuse to open");
    };
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.claude-agent.sdk.unsupported_input"
    );
}

#[cfg(not(windows))]
#[test]
fn a_platform_with_retained_tree_ownership_admits_the_route() {
    use swallowtail_adapter_claude_agent::sdk::claude_agent_sdk_addable_route_descriptor;
    let host = host_id("claude-agent-sdk.fixture.platform");
    let fixture = SdkFixtureHost::new(SdkScenario::Complete);
    let services = fixture.services(host);
    assert_eq!(
        claude_agent_sdk_addable_route_descriptor(&services).availability(),
        swallowtail_core::AddableRouteAvailability::Available
    );
}
