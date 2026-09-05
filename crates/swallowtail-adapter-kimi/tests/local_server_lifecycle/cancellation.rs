use super::local_server_lifecycle_support::{FixtureHost, FixtureServer};
use super::{binding, prepare_attached, value};
use futures_executor::block_on;
use std::sync::Arc;
use swallowtail_adapter_kimi::KimiLocalServerSessionManagementInput;
use swallowtail_core::{ExecutionHostId, ProviderSessionEffectTruth};
use swallowtail_runtime::{CancellationControl, Deadline, MonotonicInstant, RequestId};

#[test]
fn cancellation_and_deadline_keep_before_and_after_dispatch_truth_distinct() {
    let server = FixtureServer::start();
    let host = FixtureHost::new(&server);
    let execution_host = value(ExecutionHostId::new, "host.local");
    let services = host.services(execution_host.clone(), false);
    let prepared = prepare_attached(execution_host, services.clone());

    let cancelled_before = prepared
        .prepare_archive_session(KimiLocalServerSessionManagementInput::new(
            value(RequestId::new, "cancel-before"),
            binding(&prepared),
        ))
        .expect("cancel-before archive prepares");
    block_on(cancelled_before.request().cancellation().request())
        .expect("cancellation is requested");
    let outcome =
        block_on(cancelled_before.execute(services.clone())).expect("cancel-before returns truth");
    assert_eq!(
        outcome.effect().truth(),
        ProviderSessionEffectTruth::FailedBeforeEffect
    );

    let deadline_before = prepared
        .prepare_archive_session(
            KimiLocalServerSessionManagementInput::new(
                value(RequestId::new, "deadline-before"),
                binding(&prepared),
            )
            .with_deadline(Deadline::at(MonotonicInstant::from_ticks(10))),
        )
        .expect("deadline-before archive prepares");
    host.set_now(10);
    let outcome =
        block_on(deadline_before.execute(services.clone())).expect("deadline-before returns truth");
    assert_eq!(
        outcome.effect().truth(),
        ProviderSessionEffectTruth::FailedBeforeEffect
    );

    host.set_now(0);
    server.hold_lifecycle_responses();
    let cancelled_after = prepared
        .prepare_archive_session(KimiLocalServerSessionManagementInput::new(
            value(RequestId::new, "cancel-after"),
            binding(&prepared),
        ))
        .expect("cancel-after archive prepares");
    let cancellation = Arc::clone(cancelled_after.request().cancellation());
    let outcome = std::thread::scope(|scope| {
        let operation = scope.spawn(|| block_on(cancelled_after.execute(services.clone())));
        server.wait_until_seen("/api/v1/sessions/session-1:archive");
        block_on(cancellation.request()).expect("post-dispatch cancellation is requested");
        operation.join().expect("operation thread joins")
    });
    server.release_lifecycle_responses();
    let outcome = outcome.expect("cancel-after returns truth");
    assert_eq!(
        outcome.effect().truth(),
        ProviderSessionEffectTruth::UnconfirmedAfterEffect
    );

    let deadline_after = prepared
        .prepare_restore_session(
            KimiLocalServerSessionManagementInput::new(
                value(RequestId::new, "deadline-after"),
                binding(&prepared),
            )
            .with_deadline(Deadline::at(MonotonicInstant::from_ticks(20))),
        )
        .expect("deadline-after restore prepares");
    server.hold_lifecycle_responses();
    let outcome = std::thread::scope(|scope| {
        let operation = scope.spawn(|| block_on(deadline_after.execute(services)));
        server.wait_until_seen("/api/v1/sessions/session-1:restore");
        host.set_now(20);
        operation.join().expect("operation thread joins")
    });
    server.release_lifecycle_responses();
    let outcome = outcome.expect("deadline-after returns truth");
    assert_eq!(
        outcome.effect().truth(),
        ProviderSessionEffectTruth::UnconfirmedAfterEffect
    );
}
