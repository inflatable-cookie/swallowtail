use super::*;

#[test]
fn reservation_binds_before_poll_and_eventually_reaps_after_caller_return() {
    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .build_services(host("fixture.host.relinquish"));
    let service = local.task_service().clone();
    let scope = scope("stalled");
    let cancellation = DiscoveryCancellation::new();
    let task_cancellation = cancellation.clone();
    let (started_sender, started_receiver) = mpsc::channel();
    let (finished_sender, finished_receiver) = mpsc::channel();
    let reservation = service
        .reserve_reap(scope.clone())
        .expect("exact host reserves reap before task work");
    let mut task = Some(
        service
            .spawn_reapable(
                reservation,
                Box::pin(async move {
                    started_sender.send(()).expect("test receives task start");
                    task_cancellation.wait_requested().await;
                    finished_sender
                        .send(())
                        .expect("test receives task completion");
                }),
            )
            .expect("reservation binds before task polling"),
    );
    started_receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("task reaches its stall");
    assert_eq!(
        service
            .relinquish(&scope, &mut task)
            .expect("reserved handoff cannot lose lifecycle capacity"),
        TaskRelinquishOutcome::AcceptedForReap
    );
    assert!(task.is_none(), "accepted handoff clears caller ownership");

    block_on(cancellation.request()).expect("stalled task is released");
    finished_receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("accepted task completes after caller return");
    local
        .shutdown_task_reapers()
        .expect("outer host joins the retained reaper");
}

#[test]
fn shutdown_closes_admission_then_waits_for_an_unused_reservation() {
    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .with_task_reap_capacity(1)
        .build_services(host("fixture.host.unused-shutdown"));
    let service = local.task_service().clone();
    let reservation = service
        .reserve_reap(scope("unused"))
        .expect("reservation is admitted before shutdown");
    let (finished_sender, finished_receiver) = mpsc::channel();
    let shutdown = thread::spawn(move || {
        finished_sender
            .send(local.shutdown_task_reapers())
            .expect("test observes shutdown completion");
    });

    wait_for_closed_admission(&service);
    assert_eq!(
        finished_receiver.recv_timeout(Duration::from_millis(100)),
        Err(RecvTimeoutError::Timeout),
        "shutdown returned while an issued reservation remained live"
    );
    drop(reservation);
    finished_receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("shutdown completes after unused release")
        .expect("reaper shutdown succeeds");
    shutdown.join().expect("shutdown thread joins");
}

#[test]
fn issued_reservation_survives_shutdown_and_handoff_cannot_block_on_task_drop() {
    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .with_task_reap_capacity(2)
        .build_services(host("fixture.host.shutdown-race"));
    let service = local.task_service().clone();
    let scope = scope("shutdown-race");
    let reservation = service
        .reserve_reap(scope.clone())
        .expect("reservation is admitted before effects");
    let cancellation = DiscoveryCancellation::new();
    let task_cancellation = cancellation.clone();
    let (started_sender, started_receiver) = mpsc::channel();
    let task = service
        .spawn_reapable(
            reservation,
            Box::pin(async move {
                started_sender.send(()).expect("test receives task start");
                task_cancellation.wait_requested().await;
            }),
        )
        .expect("reservation-backed task starts");
    started_receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("task reaches its stall");

    let (shutdown_sender, shutdown_receiver) = mpsc::channel();
    let shutdown = thread::spawn(move || {
        shutdown_sender
            .send(local.shutdown_task_reapers())
            .expect("test observes shutdown completion");
    });
    wait_for_closed_admission(&service);

    let relinquish_service = service.clone();
    let relinquish_scope = scope.clone();
    let (result_sender, result_receiver) = mpsc::channel();
    let relinquisher = thread::spawn(move || {
        let mut task = Some(task);
        let result = relinquish_service.relinquish(&relinquish_scope, &mut task);
        result_sender
            .send((result, task))
            .expect("test receives relinquishment result");
    });
    let (result, task) = result_receiver
        .recv_timeout(Duration::from_millis(100))
        .expect("held reservation prevents shutdown-race blocking");
    assert_eq!(
        result.expect("valid reserved handoff remains accepted"),
        TaskRelinquishOutcome::AcceptedForReap
    );
    assert!(task.is_none(), "accepted handoff leaves no blocking drop");
    assert_eq!(
        shutdown_receiver.recv_timeout(Duration::from_millis(100)),
        Err(RecvTimeoutError::Timeout),
        "shutdown returned before accepted work completed"
    );

    block_on(cancellation.request()).expect("accepted task is released");
    shutdown_receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("outer shutdown joins accepted work and its reaper")
        .expect("reaper shutdown succeeds");
    relinquisher.join().expect("relinquishment thread joins");
    shutdown.join().expect("shutdown thread joins");
}

#[test]
fn captured_service_clone_cannot_own_or_deadlock_outer_shutdown() {
    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .build_services(host("fixture.host.captured-service"));
    let service = local.task_service().clone();
    let captured_service = service.clone();
    let scope = scope("captured-service");
    let cancellation = DiscoveryCancellation::new();
    let task_cancellation = cancellation.clone();
    let (started_sender, started_receiver) = mpsc::channel();
    let (task_finished_sender, task_finished_receiver) = mpsc::channel();
    let reservation = service
        .reserve_reap(scope.clone())
        .expect("reservation is admitted");
    let mut task = Some(
        service
            .spawn_reapable(
                reservation,
                Box::pin(async move {
                    started_sender.send(()).expect("test receives task start");
                    task_cancellation.wait_requested().await;
                    drop(captured_service);
                    task_finished_sender
                        .send(())
                        .expect("test observes task completion");
                }),
            )
            .expect("reserved task starts"),
    );
    started_receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("task reaches its stall");
    assert_eq!(
        service
            .relinquish(&scope, &mut task)
            .expect("owning host accepts reserved task"),
        TaskRelinquishOutcome::AcceptedForReap
    );
    drop(service);

    let (shutdown_finished_sender, shutdown_finished_receiver) = mpsc::channel();
    let shutdown = thread::spawn(move || {
        shutdown_finished_sender
            .send(local.shutdown_task_reapers())
            .expect("test observes outer shutdown completion");
    });
    assert_eq!(
        shutdown_finished_receiver.recv_timeout(Duration::from_millis(100)),
        Err(RecvTimeoutError::Timeout),
        "outer shutdown returned before accepted work completed"
    );

    block_on(cancellation.request()).expect("accepted task is released");
    task_finished_receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("task drops captured service clone without deadlock");
    shutdown_finished_receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("outer owner joins retained reaper")
        .expect("reaper shutdown succeeds");
    shutdown.join().expect("shutdown thread joins");
}
