use super::*;

#[test]
fn finished_reserved_task_returns_to_ordinary_join() {
    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .build_services(host("fixture.host.finished-reserved"));
    let service = local.task_service().clone();
    let scope = scope("finished-reserved");
    let reservation = service
        .reserve_reap(scope.clone())
        .expect("reservation is admitted");
    let (finished_sender, finished_receiver) = mpsc::channel();
    let mut task = Some(
        service
            .spawn_reapable(
                reservation,
                Box::pin(async move {
                    finished_sender.send(()).expect("test receives completion");
                }),
            )
            .expect("reserved task starts"),
    );
    finished_receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("reserved task completes");
    let deadline = Instant::now() + Duration::from_secs(1);
    while !task.as_ref().is_some_and(|task| task.is_finished()) {
        assert!(Instant::now() < deadline, "finished task was not observed");
        thread::yield_now();
    }

    let failure = service
        .relinquish(&scope, &mut task)
        .expect_err("finished task is not relabeled as transferred");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_task.already_finished"
    );
    block_on(task.take().expect("caller retains finished task").join())
        .expect("finished task joins normally");
    local
        .shutdown_task_reapers()
        .expect("ordinary join releases its reservation");
}

#[test]
fn ordinary_task_cannot_mutate_into_late_handoff_and_drop_still_joins() {
    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .build_services(host("fixture.host.blocking-drop"));
    let service = local.task_service().clone();
    local
        .shutdown_task_reapers()
        .expect("reap reservation admission closes");
    let cancellation = DiscoveryCancellation::new();
    let task_cancellation = cancellation.clone();
    let (started_sender, started_receiver) = mpsc::channel();
    let mut task = Some(
        service
            .spawn(
                scope("ordinary"),
                Box::pin(async move {
                    started_sender.send(()).expect("test receives task start");
                    task_cancellation.wait_requested().await;
                }),
            )
            .expect("ordinary spawn remains available"),
    );
    started_receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("ordinary task reaches its stall");
    let failure = service
        .relinquish(&scope("ordinary"), &mut task)
        .expect_err("unreserved late handoff is rejected");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.local_task.reap_reservation_required"
    );
    assert!(task.is_some(), "failed handoff retains caller ownership");

    let (dropped_sender, dropped_receiver) = mpsc::channel();
    let dropper = thread::spawn(move || {
        drop(task);
        dropped_sender
            .send(())
            .expect("test observes ordinary handle drop");
    });
    assert_eq!(
        dropped_receiver.recv_timeout(Duration::from_millis(100)),
        Err(RecvTimeoutError::Timeout),
        "ordinary join-on-drop stopped enforcing task ownership"
    );
    block_on(cancellation.request()).expect("ordinary task is released");
    dropped_receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("ordinary drop returns only after task completion");
    dropper.join().expect("dropper thread joins");
}
