use super::CleanupEvent;
use serde_json::Value;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::task::Waker;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    AttachmentDescriptor, AttachmentFileLease, AttachmentService, BlockingJob, BlockingWorkService,
    BoxFuture, CleanupOutcome, HostServices, MaterializedFileRef, ProcessOutputChunk,
    ProcessRequest, RuntimeFailure, ScopeId,
};
use task_time::ThreadTaskService;

mod authority;
mod process;
mod script;
mod task_time;

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum SidecarScenario {
    Complete,
    Hold,
    Disconnect,
    Malformed,
    UnknownEvent,
    TerminalRecord,
    ProviderFailure,
    ResponseMismatch,
    BootstrapCwdMismatch,
    BootstrapVersionMismatch,
    StateMismatch,
    SessionNotFound,
    SessionSubstituted,
    SwitchCwdMismatch,
    ReplayFailure,
    ReplaySequenceGap,
    ReplayCountMismatch,
    ReplayOverflow,
    ReplayAfterResponse,
    ReplayDuringResume,
    HoldReplay,
    ThinkingBootstrapMismatch,
    ThinkingStateMismatch,
    ThinkingStateMissing,
}

/// Provider-session reference the fixture "persists" across sidecar processes.
pub const FIXTURE_SESSION_REF: &str = "00000000-0000-0000-0000-000000000000";

#[derive(Clone)]
pub struct SidecarFixtureHost {
    shared: Arc<Shared>,
    scenario: SidecarScenario,
    process_wait_failure: bool,
    process_exit_failure: bool,
    deadline_task_spawn_failure: bool,
}

struct Shared {
    process_request: Mutex<Option<ProcessRequest>>,
    process: Mutex<ProcessState>,
    changed: Condvar,
    credential_acquisitions: AtomicUsize,
    cleanup: Mutex<Vec<CleanupEvent>>,
    time: Mutex<TimeState>,
    task_spawns: AtomicUsize,
}

#[derive(Default)]
struct TimeState {
    now: u64,
    fire_through: Option<u64>,
    waiters: Vec<Waker>,
}

#[derive(Default)]
struct ProcessState {
    input: Vec<Value>,
    output: VecDeque<ProcessOutputChunk>,
    stopped: bool,
    bootstrap: Option<(String, String, String)>,
    session_ref: Option<String>,
    thinking_level: Option<String>,
}

impl SidecarFixtureHost {
    pub fn new(scenario: SidecarScenario) -> Self {
        Self {
            shared: Arc::new(Shared {
                process_request: Mutex::new(None),
                process: Mutex::new(ProcessState::default()),
                changed: Condvar::new(),
                credential_acquisitions: AtomicUsize::new(0),
                cleanup: Mutex::new(Vec::new()),
                time: Mutex::new(TimeState::default()),
                task_spawns: AtomicUsize::new(0),
            }),
            scenario,
            process_wait_failure: false,
            process_exit_failure: false,
            deadline_task_spawn_failure: false,
        }
    }

    pub fn with_immediate_time(self) -> Self {
        let mut time = self
            .shared
            .time
            .lock()
            .expect("sidecar fixture time lock poisoned");
        time.now = 1_000;
        time.fire_through = Some(u64::MAX);
        drop(time);
        self
    }

    pub fn with_process_wait_failure(mut self) -> Self {
        self.process_wait_failure = true;
        self
    }

    pub fn with_process_exit_failure(mut self) -> Self {
        self.process_exit_failure = true;
        self
    }

    pub fn with_deadline_task_spawn_failure(mut self) -> Self {
        self.deadline_task_spawn_failure = true;
        self
    }

    pub fn services(&self, host: ExecutionHostId) -> HostServices {
        HostServices::new(host)
            .with_task(Arc::new(ThreadTaskService::new(
                Arc::clone(&self.shared),
                self.deadline_task_spawn_failure,
            )))
            .with_process(Arc::new(self.clone()))
            .with_credential(Arc::new(self.clone()))
            .with_working_resource(Arc::new(self.clone()))
            .with_time(Arc::new(self.clone()))
            .with_blocking_work(Arc::new(self.clone()))
            .with_attachment(Arc::new(self.clone()))
    }

    pub fn process_started(&self) -> bool {
        self.shared
            .process_request
            .lock()
            .expect("sidecar fixture process lock poisoned")
            .is_some()
    }

    pub fn process_arguments(&self) -> Vec<String> {
        self.shared
            .process_request
            .lock()
            .expect("sidecar fixture process lock poisoned")
            .as_ref()
            .expect("sidecar fixture process started")
            .arguments()
            .map(str::to_owned)
            .collect()
    }

    pub fn process_environment(&self) -> Vec<String> {
        self.shared
            .process_request
            .lock()
            .expect("sidecar fixture process lock poisoned")
            .as_ref()
            .expect("sidecar fixture process started")
            .environment()
            .map(|value| value.as_host_value().to_owned())
            .collect()
    }

    pub fn wait_for_command(&self, command: &str) {
        let mut state = self
            .shared
            .process
            .lock()
            .expect("sidecar fixture state lock poisoned");
        while !state
            .input
            .iter()
            .any(|value| value.get("command").and_then(Value::as_str) == Some(command))
        {
            state = self
                .shared
                .changed
                .wait(state)
                .expect("sidecar fixture wait lock poisoned");
        }
    }

    pub fn inputs(&self) -> Vec<Value> {
        self.shared
            .process
            .lock()
            .expect("sidecar fixture state lock poisoned")
            .input
            .clone()
    }

    pub fn credential_acquisitions(&self) -> usize {
        self.shared.credential_acquisitions.load(Ordering::SeqCst)
    }

    pub fn cleanup_events(&self) -> Vec<CleanupEvent> {
        self.shared
            .cleanup
            .lock()
            .expect("sidecar fixture cleanup lock poisoned")
            .clone()
    }
}

impl BlockingWorkService for SidecarFixtureHost {
    fn run(
        &self,
        _scope: ScopeId,
        job: BlockingJob,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async move { job() })
    }
}

impl AttachmentService for SidecarFixtureHost {
    fn materialize_file(
        &self,
        scope: ScopeId,
        descriptor: AttachmentDescriptor,
    ) -> BoxFuture<'static, Result<AttachmentFileLease, RuntimeFailure>> {
        let path = std::env::temp_dir().join(format!(
            "swallowtail-pi-sidecar-{}-{}.png",
            std::process::id(),
            descriptor.reference().as_host_value()
        ));
        let result = std::fs::write(&path, b"\x89PNG\r\n\x1a\n").map_err(|_| fixture_failure());
        Box::pin(async move {
            result?;
            Ok(AttachmentFileLease::operation_scoped(
                scope,
                descriptor.reference().clone(),
                MaterializedFileRef::new(path.to_string_lossy()).map_err(|_| fixture_failure())?,
            ))
        })
    }

    fn release_file(&self, lease: AttachmentFileLease) -> BoxFuture<'static, CleanupOutcome> {
        let _ = std::fs::remove_file(lease.file().as_driver_value());
        self.shared
            .cleanup
            .lock()
            .expect("sidecar fixture cleanup lock poisoned")
            .push(CleanupEvent::AttachmentRelease);
        Box::pin(async { CleanupOutcome::Clean })
    }
}

fn fixture_failure() -> RuntimeFailure {
    RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
        "fixture.pi_sdk_sidecar.failed",
        "Pi SDK sidecar fixture failed",
    ))
}
