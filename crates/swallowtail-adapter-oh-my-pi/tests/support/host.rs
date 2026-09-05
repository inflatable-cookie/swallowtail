use serde_json::Value;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::task::Waker;
use swallowtail_core::ExecutionHostId;
use swallowtail_runtime::{
    AttachmentDescriptor, AttachmentFileLease, AttachmentService, BlockingJob, BlockingWorkService,
    BoxFuture, CleanupOutcome, HostServices, MaterializedFileRef, ProcessExit, ProcessHandle,
    ProcessInputChunk, ProcessOutputChunk, ProcessOutputStream, ProcessRequest, ProcessService,
    RuntimeFailure, ScopeId,
};

use self::script::respond;
use self::task_time::ThreadTaskService;

mod authority;
mod inspection;
mod script;
mod task_time;

static NEXT_ATTACHMENT: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum Scenario {
    Complete,
    Disconnect,
    Hold,
    Malformed,
    ProviderFailure,
    PromptUi,
    ResponseMismatch,
    RetryDrift,
    SummarizationRetryDrift,
    StateMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CleanupEvent {
    ProcessWait,
    ResourceRelease,
    AttachmentRelease,
}

#[derive(Clone)]
pub struct FixtureHost {
    shared: Arc<Shared>,
    scenario: Scenario,
    process_wait_failure: bool,
    version: Option<String>,
}

struct Shared {
    process_request: Mutex<Option<ProcessRequest>>,
    process: Mutex<ProcessState>,
    changed: Condvar,
    cleanup: Mutex<Vec<CleanupEvent>>,
    time: Mutex<TimeState>,
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
}

impl FixtureHost {
    pub fn new(scenario: Scenario) -> Self {
        Self {
            shared: Arc::new(Shared {
                process_request: Mutex::new(None),
                process: Mutex::new(ProcessState::default()),
                changed: Condvar::new(),
                cleanup: Mutex::new(Vec::new()),
                time: Mutex::new(TimeState::default()),
            }),
            scenario,
            process_wait_failure: false,
            version: None,
        }
    }

    #[allow(dead_code)]
    pub fn version_probe(version: &str) -> Self {
        let mut host = Self::new(Scenario::Complete);
        host.version = Some(format!("omp/{version}"));
        host
    }

    pub fn with_immediate_time(self) -> Self {
        let mut time = self
            .shared
            .time
            .lock()
            .expect("OhMyPi fixture time lock poisoned");
        time.now = 1_000;
        time.fire_through = Some(u64::MAX);
        drop(time);
        self
    }

    pub fn with_process_wait_failure(mut self) -> Self {
        self.process_wait_failure = true;
        self
    }

    pub fn services(&self, host: ExecutionHostId) -> HostServices {
        HostServices::new(host)
            .with_task(Arc::new(ThreadTaskService))
            .with_process(Arc::new(self.clone()))
            .with_working_resource(Arc::new(self.clone()))
            .with_time(Arc::new(self.clone()))
            .with_blocking_work(Arc::new(self.clone()))
            .with_attachment(Arc::new(self.clone()))
    }
}

impl BlockingWorkService for FixtureHost {
    fn run(
        &self,
        _scope: ScopeId,
        job: BlockingJob,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async move { job() })
    }
}

impl AttachmentService for FixtureHost {
    fn materialize_file(
        &self,
        scope: ScopeId,
        descriptor: AttachmentDescriptor,
    ) -> BoxFuture<'static, Result<AttachmentFileLease, RuntimeFailure>> {
        let sequence = NEXT_ATTACHMENT.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "swallowtail-pi-{}-{sequence}-{}.png",
            std::process::id(),
            descriptor.reference().as_host_value()
        ));
        let result = std::fs::File::create_new(&path)
            .and_then(|mut file| std::io::Write::write_all(&mut file, b"\x89PNG\r\n\x1a\n"))
            .map_err(|_| fixture_failure());
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
            .expect("OhMyPi fixture cleanup lock poisoned")
            .push(CleanupEvent::AttachmentRelease);
        Box::pin(async { CleanupOutcome::Clean })
    }
}

impl ProcessService for FixtureHost {
    fn start(
        &self,
        _scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        *self
            .shared
            .process_request
            .lock()
            .expect("OhMyPi fixture process lock poisoned") = Some(request);
        if let Some(version) = &self.version {
            let mut state = self
                .shared
                .process
                .lock()
                .expect("OhMyPi fixture state lock poisoned");
            state.output.push_back(ProcessOutputChunk::new(
                ProcessOutputStream::Stdout,
                format!("{version}\n").into_bytes(),
            ));
            state.stopped = true;
        } else {
            let mut state = self
                .shared
                .process
                .lock()
                .expect("OhMyPi fixture state lock poisoned");
            let mut ready = serde_json::to_vec(&serde_json::json!({
                "type": "ready",
                "protocolVersion": 1,
                "supportedProtocolVersions": [1, 2],
                "maxFrameBytes": 1024 * 1024,
                "maxReassembledFrameBytes": 64 * 1024 * 1024
            }))
            .expect("ready frame serializes");
            ready.push(b'\n');
            state
                .output
                .push_back(ProcessOutputChunk::new(ProcessOutputStream::Stdout, ready));
            for value in [
                serde_json::json!({
                    "type": "extension_ui_request",
                    "id": "fixture-startup-widget",
                    "method": "setWidget",
                    "widgetLines": ["fixture startup state"]
                }),
                serde_json::json!({
                    "type": "available_commands_update",
                    "commands": []
                }),
            ] {
                let mut bytes =
                    serde_json::to_vec(&value).expect("startup lifecycle frame serializes");
                bytes.push(b'\n');
                state
                    .output
                    .push_back(ProcessOutputChunk::new(ProcessOutputStream::Stdout, bytes));
            }
        }
        let handle = FixtureProcess {
            shared: Arc::clone(&self.shared),
            scenario: self.scenario,
            wait_failure: self.process_wait_failure,
        };
        Box::pin(async move { Ok(Box::new(handle) as Box<dyn ProcessHandle>) })
    }
}

struct FixtureProcess {
    shared: Arc<Shared>,
    scenario: Scenario,
    wait_failure: bool,
}

impl ProcessHandle for FixtureProcess {
    fn write_stdin(&self, chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        let result = (|| {
            let line = chunk
                .bytes()
                .strip_suffix(b"\n")
                .ok_or_else(fixture_failure)?;
            let value: Value = serde_json::from_slice(line).map_err(|_| fixture_failure())?;
            let mut state = self
                .shared
                .process
                .lock()
                .expect("OhMyPi fixture state lock poisoned");
            state.input.push(value.clone());
            respond(self.scenario, &value, &mut state)?;
            self.shared.changed.notify_all();
            Ok(())
        })();
        Box::pin(async move { result })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.stop()
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(async move {
            let mut state = self
                .shared
                .process
                .lock()
                .expect("OhMyPi fixture state lock poisoned");
            while state.output.is_empty() && !state.stopped {
                state = self
                    .shared
                    .changed
                    .wait(state)
                    .expect("OhMyPi fixture wait lock poisoned");
            }
            Ok(state.output.pop_front())
        })
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.stop()
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.stop()
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        self.shared
            .cleanup
            .lock()
            .expect("OhMyPi fixture cleanup lock poisoned")
            .push(CleanupEvent::ProcessWait);
        let wait_failure = self.wait_failure;
        Box::pin(async move {
            if wait_failure {
                Err(fixture_failure())
            } else {
                Ok(ProcessExit::new(true, Some(0)))
            }
        })
    }
}

impl FixtureProcess {
    fn stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.shared
            .process
            .lock()
            .expect("OhMyPi fixture state lock poisoned")
            .stopped = true;
        self.shared.changed.notify_all();
        Box::pin(async { Ok(()) })
    }
}

fn fixture_failure() -> RuntimeFailure {
    RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
        "fixture.oh_my_pi_rpc.failed",
        "OhMyPi RPC fixture failed",
    ))
}
