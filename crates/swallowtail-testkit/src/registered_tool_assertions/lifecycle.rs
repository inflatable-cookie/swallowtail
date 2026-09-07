//! Open, dispatch, progress, completion-gate, and teardown conformance.

use super::support::{ComposeRegisteredToolHost, RegisteredToolHarness};
use crate::registered_tool_fixture::{
    FIXTURE_CLEANUP_BUDGET, FIXTURE_MCP_TOOL, FIXTURE_NATIVE_TOOL, FIXTURE_PROVIDER_TOOL,
    ScriptedRegisteredToolDispatcher, drive_fixture, fixture_media_type, fixture_payload,
    fixture_tool_id,
};
use std::num::NonZeroU64;
use std::sync::{Arc, Mutex};
use swallowtail_runtime::{
    AdmissionPhase, CleanupOutcome, MAX_REGISTERED_TOOL_QUEUED_PROGRESS_ITEMS,
    RegisteredToolAdmissionState, RegisteredToolBridgeLease, RegisteredToolCallId,
    RegisteredToolCallRequest, RegisteredToolExecutionDisposition, RegisteredToolFailureKind,
    RegisteredToolLifecycleState, RegisteredToolOutcome, RegisteredToolPayload,
    RegisteredToolResult, RegisteredToolSchemaDigest,
};

use super::support::conformance_deadline;

fn call_id(label: &str) -> RegisteredToolCallId {
    RegisteredToolCallId::new(label).expect("call id is valid")
}

fn request(label: &str, tool: &str, bytes: usize) -> RegisteredToolCallRequest {
    RegisteredToolCallRequest::new(
        call_id(label),
        fixture_tool_id(tool),
        fixture_payload(bytes, 1024),
        conformance_deadline(),
    )
}

fn digest() -> RegisteredToolSchemaDigest {
    RegisteredToolSchemaDigest::new("sha256:output").expect("digest is valid")
}

pub(super) fn open_binds_a_lease_without_a_listener(harness: &RegisteredToolHarness) {
    let lease = harness.open("turn-ready");

    let state = drive_fixture(
        harness
            .hosts
            .registered_tool_bridge()
            .expect("port is registered")
            .completion_gate(&lease),
    )
    .expect("completion gate observes a live lease");

    assert!(lease.is_live());
    assert!(
        lease.endpoint().is_none() && lease.bearer().is_none(),
        "the host-mediated callback profile binds no listener or bearer"
    );
    assert_eq!(state.admission(), RegisteredToolAdmissionState::Frozen);
    assert_eq!(state.lifecycle(), RegisteredToolLifecycleState::Frozen);
    assert_eq!(state.outstanding_calls(), 0);
    assert!(state.allows_successful_completion());
}

pub(super) fn one_call_settles_exactly_once(compose: &ComposeRegisteredToolHost) {
    let dispatcher = Arc::new(ScriptedRegisteredToolDispatcher::echoing());
    let harness = RegisteredToolHarness::new(compose(dispatcher.clone(), FIXTURE_CLEANUP_BUDGET));
    let lease = harness.open("turn-call");

    let outcome = drive_fixture(lease.call(request("call-1", FIXTURE_NATIVE_TOOL, 8)))
        .expect("a selected native tool dispatches");
    let duplicate = drive_fixture(lease.call(request("call-1", FIXTURE_NATIVE_TOOL, 8)))
        .expect_err("a duplicate call id fails");

    assert_eq!(outcome.call_id(), &call_id("call-1"));
    assert!(outcome.result().is_some());
    assert_eq!(
        outcome.disposition(),
        RegisteredToolExecutionDisposition::Executed
    );
    assert_eq!(dispatcher.dispatches(), 1);
    assert_eq!(
        duplicate.diagnostic().code(),
        RegisteredToolFailureKind::DuplicateCorrelation.code()
    );
    assert_eq!(
        harness.admission.observed_phases(),
        vec![
            AdmissionPhase::BeforeDispatch,
            AdmissionPhase::BeforeDelivery
        ]
    );
}

pub(super) fn unsupported_tools_fail_before_dispatch(compose: &ComposeRegisteredToolHost) {
    let dispatcher = Arc::new(ScriptedRegisteredToolDispatcher::echoing());
    let harness = RegisteredToolHarness::new(compose(dispatcher.clone(), FIXTURE_CLEANUP_BUDGET));
    let lease = harness.open("turn-unsupported");

    let provider_owned =
        drive_fixture(lease.call(request("call-provider", FIXTURE_PROVIDER_TOOL, 8)))
            .expect_err("a provider-owned tool is observation only");
    let absent = drive_fixture(lease.call(request("call-absent", "absent", 8)))
        .expect_err("an unregistered tool cannot dispatch");

    assert_eq!(
        provider_owned.diagnostic().code(),
        RegisteredToolFailureKind::UnsupportedTool.code()
    );
    assert_eq!(
        absent.diagnostic().code(),
        RegisteredToolFailureKind::UnsupportedTool.code()
    );
    assert_eq!(dispatcher.dispatches(), 0);
}

pub(super) fn oversized_argument_fails_before_dispatch(compose: &ComposeRegisteredToolHost) {
    let dispatcher = Arc::new(ScriptedRegisteredToolDispatcher::echoing());
    let harness = RegisteredToolHarness::new(compose(dispatcher.clone(), FIXTURE_CLEANUP_BUDGET));
    let lease = harness.open("turn-oversized-argument");
    let bounds = lease.selection().effective_bounds();
    let oversized = RegisteredToolPayload::new(
        fixture_media_type(),
        vec![b'x'; bounds.max_argument_bytes() + 1],
        bounds.max_argument_bytes() + 1,
    )
    .expect("payload built at its own explicit bound");

    let error = drive_fixture(lease.call(RegisteredToolCallRequest::new(
        call_id("call-oversized"),
        fixture_tool_id(FIXTURE_NATIVE_TOOL),
        oversized,
        conformance_deadline(),
    )))
    .expect_err("oversized arguments must fail");

    assert_eq!(
        error.diagnostic().code(),
        RegisteredToolFailureKind::LimitExceeded.code()
    );
    assert_eq!(dispatcher.dispatches(), 0);
}

pub(super) fn oversized_result_is_rejected(compose: &ComposeRegisteredToolHost) {
    let dispatcher = Arc::new(ScriptedRegisteredToolDispatcher::new(Arc::new(
        |call, _context| {
            let bytes = call.binding().effective_bounds().max_result_bytes() + 1;
            Ok(RegisteredToolOutcome::completed(
                call,
                RegisteredToolResult::new(
                    RegisteredToolPayload::new(fixture_media_type(), vec![b'x'; bytes], bytes)
                        .expect("payload built at its own explicit bound"),
                    digest(),
                ),
            ))
        },
    )));
    let harness = RegisteredToolHarness::new(compose(dispatcher, FIXTURE_CLEANUP_BUDGET));
    let lease = harness.open("turn-oversized-result");

    let outcome = drive_fixture(lease.call(request("call-result", FIXTURE_MCP_TOOL, 8)))
        .expect("the kernel settles the call");

    assert_eq!(
        outcome
            .failure()
            .map(swallowtail_runtime::RegisteredToolFailure::kind),
        Some(RegisteredToolFailureKind::LimitExceeded)
    );
    assert_eq!(
        outcome.disposition(),
        RegisteredToolExecutionDisposition::Executed
    );
    assert!(!outcome.permits_explicit_retry());
}

/// A dispatcher cannot fabricate an outcome for another call: minting one
/// requires the kernel-issued `RegisteredToolCall`. Server failure therefore
/// stays typed and correlated to the exact call the kernel committed.
pub(super) fn server_failure_stays_typed_and_correlated(compose: &ComposeRegisteredToolHost) {
    let dispatcher = Arc::new(ScriptedRegisteredToolDispatcher::new(Arc::new(
        |call, _context| {
            Ok(RegisteredToolOutcome::failed(
                call,
                swallowtail_runtime::RegisteredToolFailure::new(
                    RegisteredToolFailureKind::ServerExecutionFailed,
                ),
                RegisteredToolExecutionDisposition::NotExecuted,
            ))
        },
    )));
    let harness = RegisteredToolHarness::new(compose(dispatcher, FIXTURE_CLEANUP_BUDGET));
    let lease = harness.open("turn-server-failure");

    let outcome = drive_fixture(lease.call(request("call-fail", FIXTURE_NATIVE_TOOL, 8)))
        .expect("the kernel settles the call");

    assert_eq!(
        outcome
            .failure()
            .map(swallowtail_runtime::RegisteredToolFailure::kind),
        Some(RegisteredToolFailureKind::ServerExecutionFailed)
    );
    assert!(outcome.permits_explicit_retry());
}

pub(super) fn progress_admits_only_forward_sequences(compose: &ComposeRegisteredToolHost) {
    let observed: Arc<Mutex<Vec<Option<&'static str>>>> = Arc::new(Mutex::new(Vec::new()));
    let sink_observed = Arc::clone(&observed);
    let dispatcher = Arc::new(ScriptedRegisteredToolDispatcher::new(Arc::new(
        move |call, context| {
            let progress = context.progress();
            assert_eq!(progress.call_id(), call.call_id());
            let mut record = sink_observed.lock().expect("observed lock");
            for sequence in [1_u64, 1, 3, 2] {
                let outcome = progress.publish(
                    NonZeroU64::new(sequence).expect("positive sequence"),
                    fixture_payload(4, 1024),
                );
                record.push(outcome.err().map(|error| match error.diagnostic().code() {
                    code if code == RegisteredToolFailureKind::DuplicateCorrelation.code() => {
                        "duplicate"
                    }
                    _ => "other",
                }));
            }
            Ok(RegisteredToolOutcome::completed(
                call,
                RegisteredToolResult::new(fixture_payload(4, 1024), digest()),
            ))
        },
    )));
    let harness = RegisteredToolHarness::new(compose(dispatcher, FIXTURE_CLEANUP_BUDGET));
    let lease = harness.open("turn-progress");

    drive_fixture(lease.call(request("call-progress", FIXTURE_NATIVE_TOOL, 8)))
        .expect("the kernel settles the call");

    assert_eq!(
        *observed.lock().expect("observed lock"),
        vec![None, Some("duplicate"), None, Some("duplicate")]
    );
}

pub(super) fn progress_queue_is_positively_bounded(compose: &ComposeRegisteredToolHost) {
    let overflow: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    let sink_overflow = Arc::clone(&overflow);
    let dispatcher = Arc::new(ScriptedRegisteredToolDispatcher::new(Arc::new(
        move |call, context| {
            let progress = context.progress();
            for sequence in 1..=MAX_REGISTERED_TOOL_QUEUED_PROGRESS_ITEMS {
                progress
                    .publish(
                        NonZeroU64::new(sequence as u64).expect("positive sequence"),
                        fixture_payload(1, 1024),
                    )
                    .expect("bounded progress is admitted");
            }
            let error = progress
                .publish(
                    NonZeroU64::new(MAX_REGISTERED_TOOL_QUEUED_PROGRESS_ITEMS as u64 + 1)
                        .expect("positive sequence"),
                    fixture_payload(1, 1024),
                )
                .expect_err("a full progress queue fails explicitly");
            *sink_overflow.lock().expect("overflow lock") =
                Some(error.diagnostic().code().to_owned());
            Ok(RegisteredToolOutcome::completed(
                call,
                RegisteredToolResult::new(fixture_payload(4, 1024), digest()),
            ))
        },
    )));
    let harness = RegisteredToolHarness::new(compose(dispatcher, FIXTURE_CLEANUP_BUDGET));
    let lease = harness.open("turn-progress-overflow");

    drive_fixture(lease.call(request("call-overflow", FIXTURE_NATIVE_TOOL, 8)))
        .expect("the kernel settles the call");

    assert_eq!(
        overflow.lock().expect("overflow lock").as_deref(),
        Some(RegisteredToolFailureKind::LimitExceeded.code())
    );
}

pub(super) fn terminal_barrier_and_close_are_joined(compose: &ComposeRegisteredToolHost) {
    let dispatcher = Arc::new(ScriptedRegisteredToolDispatcher::echoing());
    let harness = RegisteredToolHarness::new(compose(dispatcher, FIXTURE_CLEANUP_BUDGET));
    let port = harness
        .hosts
        .registered_tool_bridge()
        .expect("port is registered")
        .clone();
    let lease = harness.open("turn-close");

    drive_fixture(lease.call(request("call-before", FIXTURE_NATIVE_TOOL, 8)))
        .expect("a call before the barrier settles");
    let frozen = drive_fixture(port.completion_gate(&lease)).expect("gate observes and freezes");
    let after_barrier = drive_fixture(lease.call(request("call-after", FIXTURE_NATIVE_TOOL, 8)))
        .expect_err("no work is admitted after the terminal barrier");
    let cleanup = drive_fixture(port.close(
        lease,
        swallowtail_runtime::RegisteredToolCleanupCause::Completion,
    ))
    .expect("close joins");

    assert!(frozen.allows_successful_completion());
    assert_eq!(
        after_barrier.diagnostic().code(),
        RegisteredToolFailureKind::PostTerminalCorrelation.code()
    );
    assert_eq!(cleanup, CleanupOutcome::Clean);
}

pub(super) fn a_closed_lease_is_not_reusable(compose: &ComposeRegisteredToolHost) {
    let dispatcher = Arc::new(ScriptedRegisteredToolDispatcher::echoing());
    let harness = RegisteredToolHarness::new(compose(dispatcher, FIXTURE_CLEANUP_BUDGET));
    let port = harness
        .hosts
        .registered_tool_bridge()
        .expect("port is registered")
        .clone();
    let first = harness.open("turn-reuse");
    let cleanup = drive_fixture(port.close(
        first,
        swallowtail_runtime::RegisteredToolCleanupCause::ExplicitClose,
    ))
    .expect("close joins");

    let second = harness.open("turn-reuse");

    assert_eq!(cleanup, CleanupOutcome::Clean);
    assert!(
        second.generation().get() > 1,
        "a later lease never reuses a retired generation"
    );
    assert_lease_redacts_private_material(&second);
}

fn assert_lease_redacts_private_material(lease: &RegisteredToolBridgeLease) {
    let rendered = format!("{lease:?}");

    assert!(!rendered.contains("secret"));
    assert!(rendered.contains("<redacted>") || !rendered.contains("token: Some"));
}
