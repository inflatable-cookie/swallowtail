#![allow(dead_code)]

mod preflight;
mod process;
mod task;
mod time;

#[allow(unused_imports)]
pub use preflight::{plan, plan_for, request, request_for, working_resource};
#[allow(unused_imports)]
pub use process::{FakeProcessService, ProcessState};
pub use task::TaskState;
pub use time::{ImmediateTimeService, PendingTimeService};

use std::sync::Arc;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{HostServices, ProcessService, TimeService};

pub fn host_services(process: Arc<dyn ProcessService>, time: Arc<dyn TimeService>) -> HostServices {
    host_services_for(
        ExecutionHostId::new("host.local").expect("host id is valid"),
        process,
        time,
    )
    .0
}

pub fn host_services_for(
    host: ExecutionHostId,
    process: Arc<dyn ProcessService>,
    time: Arc<dyn TimeService>,
) -> (HostServices, Arc<TaskState>) {
    let task = Arc::new(TaskState::default());
    let services = HostServices::new(host)
        .with_task(Arc::new(task::ThreadTaskService::new(Arc::clone(&task))))
        .with_process(process)
        .with_time(time);
    (services, task)
}
