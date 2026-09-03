use super::{host, scope};
use crate::task::LocalScopedTaskService;
use crate::{LocalProcessHost, LocalProcessLimits};
use futures_executor::block_on;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::thread;
use std::time::{Duration, Instant};
use swallowtail_runtime::{
    CancellationControl, DiscoveryCancellation, ScopedTaskService, TaskRelinquishOutcome,
};

mod admission;
mod lifecycle;
mod ordinary;

fn wait_for_closed_admission(service: &LocalScopedTaskService) {
    let deadline = Instant::now() + Duration::from_secs(1);
    loop {
        match service.reserve_reap(scope("shutdown-probe")) {
            Ok(reservation) => drop(reservation),
            Err(error)
                if error.diagnostic().code()
                    == "swallowtail.local_task.reap_reservation_shutdown" =>
            {
                return;
            }
            Err(error)
                if error.diagnostic().code()
                    == "swallowtail.local_task.reap_reservation_capacity" => {}
            Err(error) => panic!("unexpected reservation failure: {error:?}"),
        }
        assert!(
            Instant::now() < deadline,
            "shutdown did not close admission"
        );
        thread::yield_now();
    }
}
