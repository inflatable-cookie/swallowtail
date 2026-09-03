use super::*;
use std::any::Any;
use swallowtail_runtime::TaskReapReservation;

#[test]
fn reservation_refuses_unsupported_and_exhausted_hosts_before_operation_effects() {
    let effects = Arc::new(std::sync::Mutex::new(Vec::new()));
    let unsupported = LocalScopedTaskService::new(host("fixture.host.unsupported"));
    let unsupported_effects = Arc::clone(&effects);
    let result = reserve_before_effects(&unsupported, unsupported_effects);
    assert_eq!(
        result
            .expect_err("standalone task service has no reap lifecycle")
            .diagnostic()
            .code(),
        "swallowtail.local_task.reap_reservation_unsupported"
    );
    assert!(effects.lock().expect("effects lock").is_empty());

    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .with_task_reap_capacity(1)
        .build_services(host("fixture.host.capacity"));
    let first = local
        .task_service()
        .reserve_reap(scope("first"))
        .expect("first reservation is admitted");
    let result = reserve_before_effects(local.task_service(), Arc::clone(&effects));
    assert_eq!(
        result
            .expect_err("live reservation consumes exact capacity")
            .diagnostic()
            .code(),
        "swallowtail.local_task.reap_reservation_capacity"
    );
    assert!(effects.lock().expect("effects lock").is_empty());

    drop(first);
    wait_for_capacity_release(local.task_service());
    local
        .shutdown_task_reapers()
        .expect("unused reservations settle without work");

    let closing = LocalProcessHost::builder(LocalProcessLimits::default())
        .build_services(host("fixture.host.closing"));
    let closing_service = closing.task_service().clone();
    closing
        .shutdown_task_reapers()
        .expect("test closes reservation admission");
    let result = reserve_before_effects(&closing_service, Arc::clone(&effects));
    assert_eq!(
        result
            .expect_err("closed host refuses before operation effects")
            .diagnostic()
            .code(),
        "swallowtail.local_task.reap_reservation_shutdown"
    );
    assert!(effects.lock().expect("effects lock").is_empty());
}

#[test]
fn exact_host_scope_and_reservation_authority_fail_closed() {
    let first = LocalProcessHost::builder(LocalProcessLimits::default())
        .build_services(host("fixture.host.owner"));
    let second_same_id = LocalProcessHost::builder(LocalProcessLimits::default())
        .build_services(host("fixture.host.owner"));
    let other = LocalProcessHost::builder(LocalProcessLimits::default())
        .build_services(host("fixture.host.other"));
    let service = first.task_service().clone();
    let owning_scope = scope("owned");

    let foreign_polled = Arc::new(AtomicBool::new(false));
    let polled_by_foreign = Arc::clone(&foreign_polled);
    let failure = service
        .spawn_reapable(
            Box::new(ForeignReservation),
            Box::pin(async move {
                polled_by_foreign.store(true, Ordering::SeqCst);
            }),
        )
        .err()
        .expect("forged reservation is rejected");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_task.reap_reservation_foreign"
    );
    assert!(!foreign_polled.load(Ordering::SeqCst));

    let reservation = service
        .reserve_reap(owning_scope.clone())
        .expect("first lifecycle issues reservation");
    let wrong_lifecycle_polled = Arc::new(AtomicBool::new(false));
    let polled_by_wrong_lifecycle = Arc::clone(&wrong_lifecycle_polled);
    let failure = second_same_id
        .task_service()
        .spawn_reapable(
            reservation,
            Box::pin(async move {
                polled_by_wrong_lifecycle.store(true, Ordering::SeqCst);
            }),
        )
        .err()
        .expect("same host id cannot substitute another lifecycle");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_task.reap_reservation_host_mismatch"
    );
    assert!(!wrong_lifecycle_polled.load(Ordering::SeqCst));

    let cancellation = DiscoveryCancellation::new();
    let task_cancellation = cancellation.clone();
    let reservation = service
        .reserve_reap(owning_scope.clone())
        .expect("owning lifecycle issues reservation");
    let mut task = Some(
        service
            .spawn_reapable(
                reservation,
                Box::pin(async move {
                    task_cancellation.wait_requested().await;
                }),
            )
            .expect("reserved task starts"),
    );
    let failure = service
        .relinquish(&scope("wrong"), &mut task)
        .expect_err("wrong scope is rejected");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_task.scope_mismatch"
    );
    assert!(task.is_some());
    let failure = other
        .task_service()
        .relinquish(&owning_scope, &mut task)
        .expect_err("wrong host is rejected");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_task.execution_host_mismatch"
    );
    assert!(task.is_some());
    assert_eq!(
        service
            .relinquish(&owning_scope, &mut task)
            .expect("exact authority accepts task"),
        TaskRelinquishOutcome::AcceptedForReap
    );
    let failure = service
        .relinquish(&owning_scope, &mut task)
        .expect_err("repeat transfer is rejected");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_task.already_relinquished"
    );

    block_on(cancellation.request()).expect("accepted task is released");
    first
        .shutdown_task_reapers()
        .expect("owning lifecycle joins accepted work");
    second_same_id
        .shutdown_task_reapers()
        .expect("second lifecycle joins its refused reservation lane");
    other
        .shutdown_task_reapers()
        .expect("other lifecycle has no accepted work");
}

fn reserve_before_effects(
    service: &LocalScopedTaskService,
    effects: Arc<std::sync::Mutex<Vec<&'static str>>>,
) -> Result<(), swallowtail_runtime::RuntimeFailure> {
    let reservation = service.reserve_reap(scope("pre-effect"))?;
    effects.lock().expect("effects lock").extend([
        "credential",
        "resource",
        "process",
        "task",
        "provider",
    ]);
    drop(reservation);
    Ok(())
}

fn wait_for_capacity_release(service: &LocalScopedTaskService) {
    let deadline = Instant::now() + Duration::from_secs(1);
    loop {
        match service.reserve_reap(scope("released")) {
            Ok(reservation) => {
                drop(reservation);
                return;
            }
            Err(error)
                if error.diagnostic().code()
                    == "swallowtail.local_task.reap_reservation_capacity" => {}
            Err(error) => panic!("unexpected reservation failure: {error:?}"),
        }
        assert!(
            Instant::now() < deadline,
            "unused reservation did not release"
        );
        thread::yield_now();
    }
}

#[derive(Debug)]
struct ForeignReservation;

impl TaskReapReservation for ForeignReservation {
    fn into_any(self: Box<Self>) -> Box<dyn Any + Send> {
        self
    }
}
