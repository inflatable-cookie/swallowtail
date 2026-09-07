//! Kernel-integrity oracles for the five exact-head review counterexamples.

use super::support::{
    ComposeRegisteredToolHost, RegisteredToolHarness, conformance_deadline, conformance_instance,
    conformance_scope, conformance_turn,
};
use crate::registered_tool_fixture::{
    FIXTURE_CLEANUP_BUDGET, FIXTURE_MCP_TOOL, FIXTURE_NATIVE_TOOL, ScriptedAdmissionPort,
    ScriptedRegisteredToolDispatcher, drive_fixture, fixture_admission, fixture_payload,
    fixture_protocol_version, fixture_selection, fixture_snapshot_input, fixture_tool_id,
};
use std::num::NonZeroU64;
use std::sync::{Arc, Mutex};
use swallowtail_core::HostServiceKind;
use swallowtail_runtime::{
    RegisteredToolCallId, RegisteredToolCallRequest, RegisteredToolFailure,
    RegisteredToolFailureKind, RegisteredToolLimits, RegisteredToolOpenRequest,
    RegisteredToolOutcome, RegisteredToolPreparation, RegisteredToolReadiness,
    RegisteredToolResult, RegisteredToolSchemaDigest, RegisteredToolSelection,
    RegisteredToolSnapshot, RegisteredToolTransport,
};

fn request(label: &str) -> RegisteredToolCallRequest {
    RegisteredToolCallRequest::new(
        RegisteredToolCallId::new(label).expect("call id is valid"),
        fixture_tool_id(FIXTURE_NATIVE_TOOL),
        fixture_payload(8, 1024),
        conformance_deadline(),
    )
}

/// Blocker 1: no binding, call, or lease can be minted outside the kernel.
///
/// The compile-time proof is the absent surface: there is no public
/// `ValidatedRegisteredToolBinding` constructor, no public lease constructor or
/// rebind, and no host token type to forge. The runtime proof is that a lease
/// only ever authenticates against the exact kernel that created it.
pub(super) fn bindings_are_minted_only_by_the_kernel(compose: &ComposeRegisteredToolHost) {
    let first = RegisteredToolHarness::mount(
        compose,
        Arc::new(ScriptedRegisteredToolDispatcher::echoing()),
        FIXTURE_CLEANUP_BUDGET,
    );
    let second = RegisteredToolHarness::mount(
        compose,
        Arc::new(ScriptedRegisteredToolDispatcher::echoing()),
        FIXTURE_CLEANUP_BUDGET,
    );
    let foreign_port = second
        .hosts
        .registered_tool_bridge()
        .expect("port is registered")
        .clone();

    let lease = first.open("turn-minting");
    let observed = drive_fixture(foreign_port.completion_gate(&lease))
        .expect_err("a lease is not usable through another host's kernel");
    let captured =
        drive_fixture(lease.call(request("call-minted"))).expect("its own kernel settles the call");

    assert!(
        matches!(
            observed.diagnostic().code(),
            "swallowtail.registered_tool.closed"
                | "swallowtail.registered_tool.foreign_correlation"
        ),
        "a lease minted by one kernel is not resolvable by another host's registry"
    );
    assert!(captured.result().is_some());
}

/// Blocker 3: the mounted low-level open applies the typed readiness gate.
///
/// A declared-but-unqualified carrier, an unqualified protocol version, and a
/// missing required service each fail at the mounted port, not only at prepare.
pub(super) fn mounted_open_enforces_typed_readiness(compose: &ComposeRegisteredToolHost) {
    let harness = RegisteredToolHarness::mount(
        compose,
        Arc::new(ScriptedRegisteredToolDispatcher::echoing()),
        FIXTURE_CLEANUP_BUDGET,
    );
    let port = harness
        .hosts
        .registered_tool_bridge()
        .expect("port is registered")
        .clone();

    // A snapshot requiring a service this host never registers.
    let mut input =
        fixture_snapshot_input(&harness.selection.snapshot().execution_host_id().clone());
    input.required_services = [HostServiceKind::DeviceCodeDisplay].into_iter().collect();
    let unmet = Arc::new(RegisteredToolSnapshot::new(input).expect("snapshot"));
    let unmet_selection = fixture_selection(Arc::clone(&unmet));

    let direct = drive_fixture(port.open(RegisteredToolOpenRequest::new(
        harness.hosts.execution_host_id().clone(),
        conformance_instance(),
        conformance_scope(),
        conformance_turn("turn-direct-open"),
        unmet_selection.clone(),
        fixture_admission(Arc::new(ScriptedAdmissionPort::current())),
        conformance_deadline(),
    )))
    .expect_err("a direct mounted open cannot bypass the topology gate");

    let prepared = RegisteredToolPreparation::new(
        Arc::clone(&unmet),
        unmet_selection,
        fixture_admission(Arc::new(ScriptedAdmissionPort::current())),
        RegisteredToolLimits::ceiling(),
    )
    .prepare(
        &harness.hosts,
        conformance_instance(),
        conformance_scope(),
        conformance_turn("turn-direct-prepare"),
        conformance_deadline(),
    )
    .expect_err("prepare rejects the same unmet topology");

    assert_eq!(
        direct.diagnostic().code(),
        RegisteredToolFailureKind::MissingHostService.code(),
        "the mounted port applies the same typed gate as prepare"
    );
    assert_eq!(
        prepared.diagnostic().code(),
        RegisteredToolFailureKind::MissingHostService.code()
    );
}

/// Blocker 3: an unqualified carrier never reaches a mounted open either.
pub(super) fn unqualified_carrier_never_opens(compose: &ComposeRegisteredToolHost) {
    let harness = RegisteredToolHarness::mount(
        compose,
        Arc::new(ScriptedRegisteredToolDispatcher::echoing()),
        FIXTURE_CLEANUP_BUDGET,
    );
    let host = harness.selection.snapshot().execution_host_id().clone();
    let mut input = fixture_snapshot_input(&host);
    input.transports.push(
        swallowtail_runtime::RegisteredToolTransportSupport::new(
            RegisteredToolTransport::PrivateLoopbackHttp,
            [fixture_protocol_version()],
        )
        .expect("declared carrier"),
    );
    let snapshot = Arc::new(RegisteredToolSnapshot::new(input).expect("snapshot"));
    let declared_but_unqualified = RegisteredToolSelection::new(
        Arc::clone(&snapshot),
        [fixture_tool_id(FIXTURE_MCP_TOOL)],
        RegisteredToolTransport::PrivateLoopbackHttp,
        fixture_protocol_version(),
    )
    .expect("the snapshot declares the carrier");
    let port = harness
        .hosts
        .registered_tool_bridge()
        .expect("port is registered")
        .clone();

    let readiness = RegisteredToolReadiness::evaluate(&harness.hosts, &declared_but_unqualified);
    let direct = drive_fixture(port.open(RegisteredToolOpenRequest::new(
        harness.hosts.execution_host_id().clone(),
        conformance_instance(),
        conformance_scope(),
        conformance_turn("turn-unqualified-carrier"),
        declared_but_unqualified,
        fixture_admission(Arc::new(ScriptedAdmissionPort::current())),
        conformance_deadline(),
    )))
    .expect_err("an unqualified carrier never opens a live kernel");

    assert!(!readiness.transport_qualified());
    assert_eq!(
        direct.diagnostic().code(),
        RegisteredToolFailureKind::UnsupportedTransport.code()
    );
}

/// Blocker 4: the effective call deadline is enforced, not merely declared.
pub(super) fn expired_calls_never_dispatch(compose: &ComposeRegisteredToolHost) {
    let dispatcher = Arc::new(ScriptedRegisteredToolDispatcher::echoing());
    let harness = RegisteredToolHarness::mount(compose, dispatcher.clone(), FIXTURE_CLEANUP_BUDGET);
    let lease = harness.open("turn-expired");

    // The operation deadline has passed on the virtual clock.
    harness
        .clock
        .advance(conformance_deadline().instant().ticks() + 1);
    let error = drive_fixture(lease.call(request("call-expired")))
        .expect_err("an expired call never reaches the dispatcher");

    assert_eq!(
        error.diagnostic().code(),
        RegisteredToolFailureKind::DeadlineExceeded.code()
    );
    assert_eq!(dispatcher.dispatches(), 0);
}

/// Blocker 4: a call that outruns its deadline mid-dispatch settles honestly.
pub(super) fn a_call_that_outruns_its_deadline_is_unknown(compose: &ComposeRegisteredToolHost) {
    let clock_slot: Arc<Mutex<Option<Arc<crate::registered_tool_fixture::FakeClock>>>> =
        Arc::new(Mutex::new(None));
    let dispatch_clock = Arc::clone(&clock_slot);
    let dispatcher = Arc::new(ScriptedRegisteredToolDispatcher::new(Arc::new(
        move |call, _context| {
            // The linked host runs past the effective deadline before settling.
            if let Some(clock) = dispatch_clock.lock().expect("clock slot").as_ref() {
                clock.advance(conformance_deadline().instant().ticks() + 1);
            }
            Ok(RegisteredToolOutcome::completed(
                call,
                RegisteredToolResult::new(
                    fixture_payload(4, 1024),
                    RegisteredToolSchemaDigest::new("sha256:output").expect("digest"),
                ),
            ))
        },
    )));
    let harness = RegisteredToolHarness::mount(compose, dispatcher.clone(), FIXTURE_CLEANUP_BUDGET);
    *clock_slot.lock().expect("clock slot") = Some(Arc::clone(&harness.clock));
    let lease = harness.open("turn-outran");

    let outcome = drive_fixture(lease.call(request("call-outran")))
        .expect("the kernel settles the call at its terminal boundary");

    assert_eq!(dispatcher.dispatches(), 1);
    assert_eq!(
        outcome.failure().map(RegisteredToolFailure::kind),
        Some(RegisteredToolFailureKind::DeadlineExceeded)
    );
    assert_eq!(
        outcome.disposition(),
        swallowtail_runtime::RegisteredToolExecutionDisposition::Unknown,
        "an expired call never claims its effect did or did not happen"
    );
    assert!(!outcome.permits_explicit_retry());
}

/// Blocker 4: progress published past the deadline is refused.
pub(super) fn progress_past_the_deadline_is_refused(compose: &ComposeRegisteredToolHost) {
    let observed: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    let clock_slot: Arc<Mutex<Option<Arc<crate::registered_tool_fixture::FakeClock>>>> =
        Arc::new(Mutex::new(None));
    let sink_observed = Arc::clone(&observed);
    let dispatch_clock = Arc::clone(&clock_slot);
    let dispatcher = Arc::new(ScriptedRegisteredToolDispatcher::new(Arc::new(
        move |call, context| {
            let progress = context.progress();
            drive_fixture(progress.publish(NonZeroU64::MIN, fixture_payload(4, 1024)))
                .expect("progress inside the deadline is admitted");
            if let Some(clock) = dispatch_clock.lock().expect("clock slot").as_ref() {
                clock.advance(conformance_deadline().instant().ticks() + 1);
            }
            let error = drive_fixture(progress.publish(
                NonZeroU64::new(2).expect("positive sequence"),
                fixture_payload(4, 1024),
            ))
            .expect_err("progress past the deadline is refused");
            *sink_observed.lock().expect("observed lock") =
                Some(error.diagnostic().code().to_owned());
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
    *clock_slot.lock().expect("clock slot") = Some(Arc::clone(&harness.clock));
    let lease = harness.open("turn-progress-deadline");

    drive_fixture(lease.call(request("call-progress-deadline")))
        .expect("the kernel settles the call");

    assert_eq!(
        observed.lock().expect("observed lock").as_deref(),
        Some(RegisteredToolFailureKind::DeadlineExceeded.code())
    );
}

/// Blocker 5: progress re-checks the live revocation verdict per event.
///
/// The consumer revokes between the dispatch admission and the second progress
/// item. The first item is admitted, the second is refused, and the settled
/// outcome reports revocation with an honest execution disposition.
pub(super) fn progress_rechecks_live_revocation(compose: &ComposeRegisteredToolHost) {
    let observed: Arc<Mutex<Vec<Option<String>>>> = Arc::new(Mutex::new(Vec::new()));
    let admission_slot: Arc<Mutex<Option<Arc<ScriptedAdmissionPort>>>> = Arc::new(Mutex::new(None));
    let sink_observed = Arc::clone(&observed);
    let dispatch_admission = Arc::clone(&admission_slot);
    let dispatcher = Arc::new(ScriptedRegisteredToolDispatcher::new(Arc::new(
        move |call, context| {
            let progress = context.progress();
            let mut record = sink_observed.lock().expect("observed lock");
            let first = drive_fixture(progress.publish(NonZeroU64::MIN, fixture_payload(4, 1024)));
            record.push(
                first
                    .err()
                    .map(|error| error.diagnostic().code().to_owned()),
            );
            // The consumer revokes admission mid-call.
            if let Some(port) = dispatch_admission.lock().expect("admission slot").as_ref() {
                port.revoke_from(swallowtail_runtime::AdmissionPhase::BeforeDelivery);
            }
            let second = drive_fixture(progress.publish(
                NonZeroU64::new(2).expect("positive sequence"),
                fixture_payload(4, 1024),
            ));
            record.push(
                second
                    .err()
                    .map(|error| error.diagnostic().code().to_owned()),
            );
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
    *admission_slot.lock().expect("admission slot") = Some(Arc::clone(&harness.admission));
    let lease = harness.open("turn-progress-revocation");

    let outcome = drive_fixture(lease.call(request("call-progress-revocation")))
        .expect("the kernel settles the call");

    assert_eq!(
        *observed.lock().expect("observed lock"),
        vec![
            None,
            Some(RegisteredToolFailureKind::Revoked.code().to_owned())
        ],
        "each progress event re-checks the live revocation verdict"
    );
    assert_eq!(
        outcome.failure().map(RegisteredToolFailure::kind),
        Some(RegisteredToolFailureKind::Revoked)
    );
}

/// Blocker 5: revocation between dispatch and progress uses one barrier.
pub(super) fn revocation_between_dispatch_and_progress_is_linearized(
    compose: &ComposeRegisteredToolHost,
) {
    let observed: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    let sink_observed = Arc::clone(&observed);
    let dispatcher = Arc::new(ScriptedRegisteredToolDispatcher::new(Arc::new(
        move |call, context| {
            let error = drive_fixture(
                context
                    .progress()
                    .publish(NonZeroU64::MIN, fixture_payload(4, 1024)),
            )
            .expect_err("progress after a committed revocation is refused");
            *sink_observed.lock().expect("observed lock") =
                Some(error.diagnostic().code().to_owned());
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
    let lease = harness.open("turn-linearized");
    // Current for the dispatch admission, revoked for everything after it.
    harness.admission.revoke_after_validations(1);

    let outcome =
        drive_fixture(lease.call(request("call-linearized"))).expect("the kernel settles the call");

    assert_eq!(
        observed.lock().expect("observed lock").as_deref(),
        Some(RegisteredToolFailureKind::Revoked.code())
    );
    assert_eq!(
        outcome.failure().map(RegisteredToolFailure::kind),
        Some(RegisteredToolFailureKind::Revoked)
    );
}
