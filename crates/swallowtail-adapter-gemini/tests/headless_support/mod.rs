#![allow(dead_code)]

mod preflight;
mod process;
mod run;
mod task;
mod time;

pub use preflight::{cli_preparation_input, cli_probe, plan_for, request_for};
pub use process::{FakeProcessService, ScriptedProcessService};
pub use run::{assert_redacted, assert_status_code, cancelled, completed, driver, timed_out};
pub use task::TaskState;
pub use time::{ImmediateTimeService, PendingTimeService};

use std::sync::Arc;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{HostServices, ProcessService, TimeService};

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

pub fn session_id(request_id: &str) -> String {
    use std::fmt::Write;

    let mut value = String::from("swallowtail-");
    for byte in request_id.bytes() {
        write!(&mut value, "{byte:02x}").expect("writing to a string cannot fail");
    }
    value
}

pub fn fixture(name: &str, request_id: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/gemini-headless-0.51.0-0.52.0")
        .join(name);
    std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("failed to read {name}: {error}"))
        .replace("<SESSION_ID>", &session_id(request_id))
}
