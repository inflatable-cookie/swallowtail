//! Concurrent-admission race oracles for the serialized progress gate.
//!
//! Every case here holds one live verdict pending while a second admission
//! sequence tries to start, so the serialization is falsified by construction
//! rather than by sequential publishing.

use super::support::{ComposeRegisteredToolHost, RegisteredToolHarness, conformance_deadline};
use crate::registered_tool_fixture::{
    FIXTURE_CLEANUP_BUDGET, FIXTURE_NATIVE_TOOL, ScriptedAdmissionPort,
    ScriptedRegisteredToolDispatcher, drive_fixture, fixture_payload, fixture_tool_id,
    interleave_fixtures, poll_fixture_once,
};
use std::num::NonZeroU64;
use std::sync::{Arc, Mutex};
use std::task::Poll;
use std::time::Duration;
use swallowtail_runtime::{
    RegisteredToolBounds, RegisteredToolCallId, RegisteredToolCallRequest, RegisteredToolFailure,
    RegisteredToolFailureKind, RegisteredToolLimits, RegisteredToolOutcome, RegisteredToolResult,
    RegisteredToolSchemaDigest, RuntimeFailure,
};

type Recorded = Arc<Mutex<Vec<Option<String>>>>;
type PortSlot = Arc<Mutex<Option<Arc<ScriptedAdmissionPort>>>>;

fn request(label: &str) -> RegisteredToolCallRequest {
    RegisteredToolCallRequest::new(
        RegisteredToolCallId::new(label).expect("call id is valid"),
        fixture_tool_id(FIXTURE_NATIVE_TOOL),
        fixture_payload(8, 1024),
        conformance_deadline(),
    )
}

fn settle(call: &swallowtail_runtime::RegisteredToolCall) -> RegisteredToolOutcome {
    RegisteredToolOutcome::completed(
        call,
        RegisteredToolResult::new(
            fixture_payload(4, 1024),
            RegisteredToolSchemaDigest::new("sha256:output").expect("digest"),
        ),
    )
}

fn code(result: Result<(), RuntimeFailure>) -> Option<String> {
    result
        .err()
        .map(|error| error.diagnostic().code().to_owned())
}

/// One queued progress item, so two concurrent publishes contend for one slot.
fn one_slot_limits() -> RegisteredToolLimits {
    RegisteredToolLimits::new(
        RegisteredToolBounds::new(1, 1_024, 1_024, 1_024, 1, Duration::from_secs(30))
            .expect("narrowed bounds"),
    )
}

/// Two concurrent publishes cannot both pass a one-slot queue bound.
///
/// The first publish holds the gate across a pending live verdict. The second
/// cannot pass its pre-verdict checks until the first has committed, so it sees
/// the full queue and is refused without ever requesting a live verdict.
pub(super) fn concurrent_publishes_cannot_both_take_one_slot(compose: &ComposeRegisteredToolHost) {
    let observed: Recorded = Arc::new(Mutex::new(Vec::new()));
    let port_slot: PortSlot = Arc::new(Mutex::new(None));
    let verdicts_at_second_poll = Arc::new(Mutex::new(0_usize));

    let sink_observed = Arc::clone(&observed);
    let sink_port = Arc::clone(&port_slot);
    let sink_verdicts = Arc::clone(&verdicts_at_second_poll);
    let dispatcher = Arc::new(ScriptedRegisteredToolDispatcher::new(Arc::new(
        move |call, context| {
            let port = sink_port
                .lock()
                .expect("port slot")
                .clone()
                .expect("the admission port is bound");
            let progress = context.progress();
            port.hold();

            let mut first = progress.publish(NonZeroU64::MIN, fixture_payload(4, 1024));
            let mut second = progress.publish(
                NonZeroU64::new(2).expect("positive sequence"),
                fixture_payload(4, 1024),
            );
            // Both admission sequences are in flight; the first holds the gate
            // across its pending verdict and the second cannot start.
            assert!(matches!(poll_fixture_once(&mut first), Poll::Pending));
            assert!(matches!(poll_fixture_once(&mut second), Poll::Pending));
            *sink_verdicts.lock().expect("verdict count") = port.validation_count();

            port.release();
            let (first, second) = interleave_fixtures(&mut first, &mut second);
            let mut record = sink_observed.lock().expect("observed lock");
            record.push(code(first));
            record.push(code(second));
            Ok(settle(call))
        },
    )));

    let harness = RegisteredToolHarness::mount_with_limits(
        compose,
        dispatcher,
        FIXTURE_CLEANUP_BUDGET,
        one_slot_limits(),
    );
    *port_slot.lock().expect("port slot") = Some(Arc::clone(&harness.admission));
    let lease = harness.open("turn-concurrent-slot");

    drive_fixture(lease.call(request("call-concurrent-slot")))
        .expect("the kernel settles the call");

    let record = observed.lock().expect("observed lock");
    let admitted = record.iter().filter(|entry| entry.is_none()).count();
    let refused: Vec<_> = record.iter().flatten().cloned().collect();

    assert_eq!(admitted, 1, "exactly one publish may take the single slot");
    assert_eq!(
        refused,
        vec![RegisteredToolFailureKind::LimitExceeded.code().to_owned()],
        "the losing publish is refused for the queue bound"
    );
    assert_eq!(
        *verdicts_at_second_poll.lock().expect("verdict count"),
        2,
        "only the dispatch verdict and the first publish verdict were requested; \
         the second publish never asked for a live verdict before its bound check"
    );
}

/// A revocation landing while one verdict is pending blocks every later commit.
///
/// The first publish is suspended inside its live validation. The consumer
/// revokes, the verdict resolves as revoked, and neither that publish nor the
/// concurrent one may commit.
pub(super) fn revocation_during_a_pending_verdict_blocks_every_commit(
    compose: &ComposeRegisteredToolHost,
) {
    let observed: Recorded = Arc::new(Mutex::new(Vec::new()));
    let port_slot: PortSlot = Arc::new(Mutex::new(None));

    let sink_observed = Arc::clone(&observed);
    let sink_port = Arc::clone(&port_slot);
    let dispatcher = Arc::new(ScriptedRegisteredToolDispatcher::new(Arc::new(
        move |call, context| {
            let port = sink_port
                .lock()
                .expect("port slot")
                .clone()
                .expect("the admission port is bound");
            let progress = context.progress();
            port.hold();

            let mut first = progress.publish(NonZeroU64::MIN, fixture_payload(4, 1024));
            let mut second = progress.publish(
                NonZeroU64::new(2).expect("positive sequence"),
                fixture_payload(4, 1024),
            );
            assert!(matches!(poll_fixture_once(&mut first), Poll::Pending));
            assert!(matches!(poll_fixture_once(&mut second), Poll::Pending));

            // The consumer revokes while the first verdict is still in flight.
            port.revoke_now();
            port.release();
            let (first, second) = interleave_fixtures(&mut first, &mut second);
            let mut record = sink_observed.lock().expect("observed lock");
            record.push(code(first));
            record.push(code(second));
            Ok(settle(call))
        },
    )));

    let harness = RegisteredToolHarness::mount(compose, dispatcher, FIXTURE_CLEANUP_BUDGET);
    *port_slot.lock().expect("port slot") = Some(Arc::clone(&harness.admission));
    let lease = harness.open("turn-pending-revocation");

    let outcome = drive_fixture(lease.call(request("call-pending-revocation")))
        .expect("the kernel settles the call");

    let revoked = RegisteredToolFailureKind::Revoked.code().to_owned();
    assert_eq!(
        *observed.lock().expect("observed lock"),
        vec![Some(revoked.clone()), Some(revoked)],
        "no publish commits once a revocation resolves under the gate"
    );
    assert_eq!(
        outcome.failure().map(RegisteredToolFailure::kind),
        Some(RegisteredToolFailureKind::Revoked)
    );
}

/// Concurrent publishes keep exact ordering and correlation.
///
/// A later sequence commits first, so the earlier one is regressive and is
/// refused rather than committing out of order.
pub(super) fn concurrent_publishes_keep_exact_ordering(compose: &ComposeRegisteredToolHost) {
    let observed: Recorded = Arc::new(Mutex::new(Vec::new()));
    let port_slot: PortSlot = Arc::new(Mutex::new(None));

    let sink_observed = Arc::clone(&observed);
    let sink_port = Arc::clone(&port_slot);
    let dispatcher = Arc::new(ScriptedRegisteredToolDispatcher::new(Arc::new(
        move |call, context| {
            let port = sink_port
                .lock()
                .expect("port slot")
                .clone()
                .expect("the admission port is bound");
            let progress = context.progress();
            port.hold();

            let mut ahead = progress.publish(
                NonZeroU64::new(4).expect("positive sequence"),
                fixture_payload(4, 1024),
            );
            let mut behind = progress.publish(
                NonZeroU64::new(2).expect("positive sequence"),
                fixture_payload(4, 1024),
            );
            assert!(matches!(poll_fixture_once(&mut ahead), Poll::Pending));
            assert!(matches!(poll_fixture_once(&mut behind), Poll::Pending));

            port.release();
            let (ahead, behind) = interleave_fixtures(&mut ahead, &mut behind);
            let mut record = sink_observed.lock().expect("observed lock");
            record.push(code(ahead));
            record.push(code(behind));
            Ok(settle(call))
        },
    )));

    let harness = RegisteredToolHarness::mount(compose, dispatcher, FIXTURE_CLEANUP_BUDGET);
    *port_slot.lock().expect("port slot") = Some(Arc::clone(&harness.admission));
    let lease = harness.open("turn-concurrent-order");

    drive_fixture(lease.call(request("call-concurrent-order")))
        .expect("the kernel settles the call");

    assert_eq!(
        *observed.lock().expect("observed lock"),
        vec![
            None,
            Some(
                RegisteredToolFailureKind::DuplicateCorrelation
                    .code()
                    .to_owned()
            )
        ],
        "the sequence that commits first wins and a regressive publish is refused"
    );
}
