//! Revocation, concurrency, stale correlation, and failed-cleanup oracles.

use super::support::{ComposeRegisteredToolHost, RegisteredToolHarness, conformance_deadline};
use crate::registered_tool_fixture::{
    FIXTURE_CLEANUP_BUDGET, FIXTURE_NATIVE_TOOL, ScriptedRegisteredToolDispatcher,
    UncooperativeRegisteredToolDispatcher, drive_fixture, fixture_payload, fixture_tool_id,
    poll_fixture_once,
};
use std::num::NonZeroU64;
use std::sync::{Arc, Mutex};
use std::task::Poll;
use swallowtail_runtime::{
    AdmissionPhase, CleanupOutcome, RegisteredToolAdmissionState, RegisteredToolCallId,
    RegisteredToolCallRequest, RegisteredToolCleanupCause, RegisteredToolExecutionDisposition,
    RegisteredToolFailure, RegisteredToolFailureKind, RegisteredToolOutcome,
    RegisteredToolProgressSink, RegisteredToolResult, RegisteredToolSchemaDigest,
};

fn request(label: &str) -> RegisteredToolCallRequest {
    RegisteredToolCallRequest::new(
        RegisteredToolCallId::new(label).expect("call id is valid"),
        fixture_tool_id(FIXTURE_NATIVE_TOOL),
        fixture_payload(8, 1024),
        conformance_deadline(),
    )
}

pub(super) fn revocation_before_dispatch_never_reaches_the_dispatcher(
    compose: &ComposeRegisteredToolHost,
) {
    let dispatcher = Arc::new(ScriptedRegisteredToolDispatcher::echoing());
    let harness = RegisteredToolHarness::mount(compose, dispatcher.clone(), FIXTURE_CLEANUP_BUDGET);
    let lease = harness.open("turn-revoked-dispatch");
    harness
        .admission
        .revoke_from(AdmissionPhase::BeforeDispatch);

    let error =
        drive_fixture(lease.call(request("call-revoked"))).expect_err("revocation fails closed");
    let later = drive_fixture(lease.call(request("call-after-revocation")))
        .expect_err("revocation freezes later calls");

    assert_eq!(
        error.diagnostic().code(),
        RegisteredToolFailureKind::Revoked.code()
    );
    assert_eq!(
        later.diagnostic().code(),
        RegisteredToolFailureKind::Revoked.code()
    );
    assert_eq!(dispatcher.dispatches(), 0);
}

pub(super) fn revocation_before_delivery_reports_honest_execution(
    compose: &ComposeRegisteredToolHost,
) {
    let dispatcher = Arc::new(ScriptedRegisteredToolDispatcher::echoing());
    let harness = RegisteredToolHarness::mount(compose, dispatcher.clone(), FIXTURE_CLEANUP_BUDGET);
    let lease = harness.open("turn-revoked-delivery");
    harness
        .admission
        .revoke_from(AdmissionPhase::BeforeDelivery);

    let outcome =
        drive_fixture(lease.call(request("call-executed"))).expect("the kernel settles the call");

    assert_eq!(dispatcher.dispatches(), 1);
    assert_eq!(
        outcome.failure().map(RegisteredToolFailure::kind),
        Some(RegisteredToolFailureKind::Revoked)
    );
    assert_eq!(
        outcome.disposition(),
        RegisteredToolExecutionDisposition::Executed,
        "revocation after admission never claims the effect was undone"
    );
    assert!(!outcome.permits_explicit_retry());
}

pub(super) fn a_retained_sink_fails_after_the_call_settles(compose: &ComposeRegisteredToolHost) {
    let retained: Arc<Mutex<Option<RegisteredToolProgressSink>>> = Arc::new(Mutex::new(None));
    let sink_slot = Arc::clone(&retained);
    let dispatcher = Arc::new(ScriptedRegisteredToolDispatcher::new(Arc::new(
        move |call, context| {
            *sink_slot.lock().expect("sink lock") = Some(context.progress().clone());
            Ok(RegisteredToolOutcome::completed(
                call,
                RegisteredToolResult::new(
                    fixture_payload(4, 1024),
                    RegisteredToolSchemaDigest::new("sha256:output").expect("digest"),
                ),
            ))
        },
    )));
    let harness = RegisteredToolHarness::mount(compose, dispatcher, FIXTURE_CLEANUP_BUDGET);
    let lease = harness.open("turn-stale-sink");

    drive_fixture(lease.call(request("call-settled"))).expect("the kernel settles the call");
    let sink = retained
        .lock()
        .expect("sink lock")
        .take()
        .expect("the dispatcher retained its sink");
    let error = drive_fixture(sink.publish(NonZeroU64::MIN, fixture_payload(4, 1024)))
        .expect_err("a stale sink cannot publish after the call settles");

    assert_eq!(
        error.diagnostic().code(),
        RegisteredToolFailureKind::ForeignCorrelation.code()
    );
}

pub(super) fn one_outstanding_call_per_lease(compose: &ComposeRegisteredToolHost) {
    let dispatcher = Arc::new(UncooperativeRegisteredToolDispatcher::default());
    let harness = RegisteredToolHarness::mount(compose, dispatcher.clone(), FIXTURE_CLEANUP_BUDGET);
    let port = harness
        .hosts
        .registered_tool_bridge()
        .expect("port is registered")
        .clone();
    let lease = harness.open("turn-concurrency");

    let mut pending = lease.call(request("call-pending"));
    assert!(matches!(poll_fixture_once(&mut pending), Poll::Pending));
    let second =
        drive_fixture(lease.call(request("call-second"))).expect_err("concurrency is bounded");
    let observed = drive_fixture(port.completion_gate(&lease)).expect("gate observes");

    assert_eq!(dispatcher.dispatches(), 1);
    assert_eq!(
        second.diagnostic().code(),
        RegisteredToolFailureKind::LimitExceeded.code()
    );
    assert_eq!(observed.outstanding_calls(), 1);
    assert_eq!(observed.admission(), RegisteredToolAdmissionState::Open);
    assert!(!observed.allows_successful_completion());
}

pub(super) fn failed_cleanup_retains_ownership(compose: &ComposeRegisteredToolHost) {
    let dispatcher = Arc::new(UncooperativeRegisteredToolDispatcher::default());
    let harness = RegisteredToolHarness::mount(compose, dispatcher.clone(), FIXTURE_CLEANUP_BUDGET);
    let port = harness
        .hosts
        .registered_tool_bridge()
        .expect("port is registered")
        .clone();
    let lease = harness.open("turn-uncooperative");
    let mut pending = lease.call(request("call-uncooperative"));
    assert!(matches!(poll_fixture_once(&mut pending), Poll::Pending));

    let observed_before =
        drive_fixture(port.completion_gate(&lease)).expect("gate observes a live lease");
    drop(pending);
    let cleanup = drive_fixture(port.close(lease, RegisteredToolCleanupCause::Deadline))
        .expect("close reports its exact truth");
    let reopen = harness
        .try_open("turn-uncooperative")
        .err()
        .map(|error| error.diagnostic().code().to_owned());

    assert_eq!(observed_before.outstanding_calls(), 1);
    assert!(
        matches!(cleanup, CleanupOutcome::Failed(_)),
        "an unjoined dispatcher never reports a clean close"
    );
    assert_eq!(
        cleanup
            .diagnostic()
            .map(swallowtail_core::SafeDiagnostic::code),
        Some(RegisteredToolFailureKind::TeardownFailed.code())
    );
    assert_eq!(
        reopen.as_deref(),
        Some("swallowtail.registered_tool.already_open"),
        "the failed lease retains ownership of its turn attempt"
    );
}
