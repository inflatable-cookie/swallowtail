use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::Waker;

#[path = "process_handle.rs"]
mod handle;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    BoxFuture, ProcessExit, ProcessHandle, ProcessOutputChunk, ProcessOutputStream, ProcessRequest,
    ProcessService, RuntimeFailure, ScopeId,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservedProcessRequest {
    pub executable: String,
    pub arguments: Vec<String>,
    pub environments: Vec<String>,
    pub working_resource: Option<String>,
}

struct OutputState {
    chunks: VecDeque<ProcessOutputChunk>,
    closed: bool,
    waker: Option<Waker>,
}

impl OutputState {
    fn wake(&mut self) {
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }
}

#[derive(Default)]
pub struct ProcessState {
    request: Mutex<Option<ObservedProcessRequest>>,
    stdin: Mutex<Vec<u8>>,
    stdin_closed: AtomicBool,
    force_stopped: AtomicBool,
    waited: AtomicBool,
}

impl ProcessState {
    pub fn started(&self) -> bool {
        self.request
            .lock()
            .expect("request lock is available")
            .is_some()
    }

    pub fn request(&self) -> ObservedProcessRequest {
        self.request
            .lock()
            .expect("request lock is available")
            .clone()
            .expect("process request was captured")
    }

    pub fn stdin(&self) -> Vec<u8> {
        self.stdin.lock().expect("stdin lock is available").clone()
    }

    pub fn stdin_closed(&self) -> bool {
        self.stdin_closed.load(Ordering::SeqCst)
    }

    pub fn force_stopped(&self) -> bool {
        self.force_stopped.load(Ordering::SeqCst)
    }

    pub fn waited(&self) -> bool {
        self.waited.load(Ordering::SeqCst)
    }
}

pub struct ProcessCompleter {
    output: Arc<Mutex<OutputState>>,
    exit: Arc<Mutex<ProcessExit>>,
    hold_open: Arc<AtomicBool>,
}

impl ProcessCompleter {
    pub fn push_stdout(&self, stdout: &str) {
        if stdout.is_empty() {
            return;
        }
        let mut output = self.output.lock().expect("output lock is available");
        output.chunks.push_back(ProcessOutputChunk::new(
            ProcessOutputStream::Stdout,
            stdout.as_bytes().to_vec(),
        ));
        output.wake();
    }

    pub fn complete(&self, stdout: &str, exit: ProcessExit) {
        *self.exit.lock().expect("exit lock is available") = exit;
        {
            let mut output = self.output.lock().expect("output lock is available");
            if !stdout.is_empty() {
                output.chunks.push_back(ProcessOutputChunk::new(
                    ProcessOutputStream::Stdout,
                    stdout.as_bytes().to_vec(),
                ));
            }
            output.closed = true;
            output.wake();
        }
        self.hold_open.store(false, Ordering::SeqCst);
    }
}

pub struct FakeProcessService {
    state: Arc<ProcessState>,
    output: Mutex<Option<Arc<Mutex<OutputState>>>>,
    exit: Arc<Mutex<ProcessExit>>,
    hold_open: Arc<AtomicBool>,
    fail_start: bool,
    fail_stdin: Arc<AtomicBool>,
}

impl FakeProcessService {
    pub fn completed(stdout: &str) -> (Arc<Self>, Arc<ProcessState>) {
        let (service, state, _) = Self::new(
            handle::stdout_chunks(stdout),
            ProcessExit::new(true, Some(0)),
            false,
            false,
            false,
        );
        (service, state)
    }

    pub fn with_exit(stdout: &str, exit: ProcessExit) -> (Arc<Self>, Arc<ProcessState>) {
        let (service, state, _) =
            Self::new(handle::stdout_chunks(stdout), exit, false, false, false);
        (service, state)
    }

    pub fn held_open() -> (Arc<Self>, Arc<ProcessState>) {
        let (service, state, _) = Self::new(
            VecDeque::new(),
            ProcessExit::new(false, Some(130)),
            true,
            false,
            false,
        );
        (service, state)
    }

    pub fn controllable() -> (Arc<Self>, Arc<ProcessState>, ProcessCompleter) {
        Self::new(
            VecDeque::new(),
            ProcessExit::new(false, Some(130)),
            true,
            false,
            false,
        )
    }

    pub fn fail_start() -> Arc<Self> {
        let (service, _, _) = Self::new(
            VecDeque::new(),
            ProcessExit::new(false, Some(1)),
            false,
            true,
            false,
        );
        service
    }

    pub fn fail_stdin() -> (Arc<Self>, Arc<ProcessState>) {
        let (service, state, _) = Self::new(
            VecDeque::new(),
            ProcessExit::new(false, Some(1)),
            true,
            false,
            true,
        );
        (service, state)
    }

    fn new(
        chunks: VecDeque<ProcessOutputChunk>,
        exit: ProcessExit,
        hold_open: bool,
        fail_start: bool,
        fail_stdin: bool,
    ) -> (Arc<Self>, Arc<ProcessState>, ProcessCompleter) {
        let state = Arc::new(ProcessState::default());
        let output = Arc::new(Mutex::new(OutputState {
            chunks,
            closed: !hold_open,
            waker: None,
        }));
        let exit = Arc::new(Mutex::new(exit));
        let hold_open = Arc::new(AtomicBool::new(hold_open));
        let fail_stdin = Arc::new(AtomicBool::new(fail_stdin));
        let completer = ProcessCompleter {
            output: Arc::clone(&output),
            exit: Arc::clone(&exit),
            hold_open: Arc::clone(&hold_open),
        };
        (
            Arc::new(Self {
                state: Arc::clone(&state),
                output: Mutex::new(Some(output)),
                exit,
                hold_open,
                fail_start,
                fail_stdin,
            }),
            state,
            completer,
        )
    }
}

impl ProcessService for FakeProcessService {
    fn start(
        &self,
        _scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        if self.fail_start {
            return Box::pin(async {
                Err(RuntimeFailure::new(SafeDiagnostic::new(
                    "fixture.process.start_failed",
                    "Fixture process start failed",
                )))
            });
        }
        *self
            .state
            .request
            .lock()
            .expect("request lock is available") = Some(ObservedProcessRequest {
            executable: request.executable().as_host_value().to_owned(),
            arguments: request.arguments().map(str::to_owned).collect(),
            environments: request
                .environment()
                .map(|value| value.as_host_value().to_owned())
                .collect(),
            working_resource: request
                .working_resource()
                .map(|value| value.as_host_value().to_owned()),
        });
        let output = self
            .output
            .lock()
            .expect("output lock is available")
            .take()
            .expect("fake process starts once");
        let handle = handle::FakeProcessHandle {
            state: Arc::clone(&self.state),
            output,
            exit: Arc::clone(&self.exit),
            hold_open: Arc::clone(&self.hold_open),
            fail_stdin: Arc::clone(&self.fail_stdin),
        };
        Box::pin(async move { Ok(Box::new(handle) as Box<dyn ProcessHandle>) })
    }
}
