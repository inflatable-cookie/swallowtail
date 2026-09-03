use super::*;
use std::future::Future;
use std::task::{Context, Poll, Waker};

#[test]
fn dropping_unpolled_reserved_join_future_keeps_host_ownership() {
    assert_reserved_join_cancellation_is_host_owned(false);
}

#[test]
fn dropping_polled_reserved_join_future_keeps_host_ownership() {
    assert_reserved_join_cancellation_is_host_owned(true);
}

#[test]
fn reserved_join_future_reports_the_host_owned_join() {
    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .build_services(host("fixture.host.reserved-join"));
    let service = local.task_service().clone();
    let cancellation = DiscoveryCancellation::new();
    let task_cancellation = cancellation.clone();
    let (started_sender, started_receiver) = mpsc::channel();
    let reservation = service
        .reserve_reap(scope("reserved-join"))
        .expect("reservation is admitted");
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

    let (joined_sender, joined_receiver) = mpsc::channel();
    let joiner = thread::spawn(move || {
        joined_sender
            .send(block_on(task.join()))
            .expect("test observes reserved join");
    });
    assert_eq!(
        joined_receiver.recv_timeout(Duration::from_millis(100)),
        Err(RecvTimeoutError::Timeout),
        "reserved join reported before task completion"
    );
    block_on(cancellation.request()).expect("reserved task is released");
    joined_receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("reserved join completes after the task")
        .expect("host-owned worker joins successfully");
    joiner.join().expect("joiner thread joins");
    local
        .shutdown_task_reapers()
        .expect("outer host joins the reserved lane");
}

fn assert_reserved_join_cancellation_is_host_owned(poll_once: bool) {
    let local = LocalProcessHost::builder(LocalProcessLimits::default())
        .build_services(host("fixture.host.join-cancellation"));
    let service = local.task_service().clone();
    let scope = scope("join-cancellation");
    let cancellation = DiscoveryCancellation::new();
    let task_cancellation = cancellation.clone();
    let (started_sender, started_receiver) = mpsc::channel();
    let (finished_sender, finished_receiver) = mpsc::channel();
    let reservation = service
        .reserve_reap(scope.clone())
        .expect("reservation is admitted before task work");
    let task = service
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
        .expect("reservation-backed task starts");
    started_receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("task reaches its stall");

    let mut join = task.join();
    let (dropped_sender, dropped_receiver) = mpsc::channel();
    let dropper = thread::spawn(move || {
        let pending = if poll_once {
            let mut context = Context::from_waker(Waker::noop());
            matches!(Future::poll(join.as_mut(), &mut context), Poll::Pending)
        } else {
            true
        };
        drop(join);
        dropped_sender
            .send(pending)
            .expect("test observes join-future drop");
    });
    let pending = match dropped_receiver.recv_timeout(Duration::from_millis(100)) {
        Ok(pending) => pending,
        Err(RecvTimeoutError::Timeout) => {
            block_on(cancellation.request()).expect("blocked poll is released");
            dropper.join().expect("dropper thread joins after release");
            local
                .shutdown_task_reapers()
                .expect("cleanup joins the released task");
            panic!("polling the reserved join future blocked on the worker");
        }
        Err(error) => panic!("join-future drop channel failed: {error:?}"),
    };
    if !pending {
        block_on(cancellation.request()).expect("unexpected ready poll is released");
        dropper.join().expect("dropper thread joins");
        local
            .shutdown_task_reapers()
            .expect("cleanup joins the released task");
        panic!("stalled reserved join future unexpectedly became ready");
    }
    dropper
        .join()
        .expect("dropping the join future does not block on the worker");

    let (shutdown_sender, shutdown_receiver) = mpsc::channel();
    let shutdown = thread::spawn(move || {
        shutdown_sender
            .send(local.shutdown_task_reapers())
            .expect("test observes shutdown completion");
    });
    wait_for_closed_admission(&service);
    match shutdown_receiver.recv_timeout(Duration::from_millis(100)) {
        Err(RecvTimeoutError::Timeout) => {}
        Ok(result) => {
            block_on(cancellation.request()).expect("detached task is released");
            shutdown.join().expect("shutdown thread joins");
            result.expect("early shutdown result is otherwise clean");
            panic!("shutdown falsely settled while reserved task work was live");
        }
        Err(RecvTimeoutError::Disconnected) => {
            block_on(cancellation.request()).expect("failed shutdown is released");
            let _ = shutdown.join();
            panic!("shutdown completion channel disconnected");
        }
    }

    block_on(cancellation.request()).expect("host-owned task is released");
    finished_receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("reserved task completes after future cancellation");
    shutdown_receiver
        .recv_timeout(Duration::from_secs(1))
        .expect("shutdown completes after host join and reap")
        .expect("reaper shutdown succeeds");
    shutdown.join().expect("shutdown thread joins");
}
