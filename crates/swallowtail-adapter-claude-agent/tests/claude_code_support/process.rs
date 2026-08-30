use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use swallowtail_runtime::{
    BoxFuture, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk,
    ProcessOutputStream, ProcessRequest, ProcessService, RuntimeFailure, ScopeId,
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
    wake: Arc<Condvar>,
    exit: Arc<Mutex<ProcessExit>>,
    hold_open: Arc<AtomicBool>,
}

impl ProcessCompleter {
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
        }
        self.hold_open.store(false, Ordering::SeqCst);
        self.wake.notify_all();
    }
}

pub struct FakeProcessService {
    state: Arc<ProcessState>,
    output: Mutex<Option<Arc<Mutex<OutputState>>>>,
    wake: Arc<Condvar>,
    exit: Arc<Mutex<ProcessExit>>,
    hold_open: Arc<AtomicBool>,
}

impl FakeProcessService {
    pub fn completed(stdout: &str) -> (Arc<Self>, Arc<ProcessState>) {
        let (service, state, _) = Self::new(
            stdout_chunks(stdout),
            ProcessExit::new(true, Some(0)),
            false,
        );
        (service, state)
    }

    pub fn with_exit(stdout: &str, exit: ProcessExit) -> (Arc<Self>, Arc<ProcessState>) {
        let (service, state, _) = Self::new(stdout_chunks(stdout), exit, false);
        (service, state)
    }

    pub fn held_open() -> (Arc<Self>, Arc<ProcessState>) {
        let (service, state, _) =
            Self::new(VecDeque::new(), ProcessExit::new(false, Some(130)), true);
        (service, state)
    }

    pub fn controllable() -> (Arc<Self>, Arc<ProcessState>, ProcessCompleter) {
        Self::new(VecDeque::new(), ProcessExit::new(false, Some(130)), true)
    }

    fn new(
        chunks: VecDeque<ProcessOutputChunk>,
        exit: ProcessExit,
        hold_open: bool,
    ) -> (Arc<Self>, Arc<ProcessState>, ProcessCompleter) {
        let state = Arc::new(ProcessState::default());
        let output = Arc::new(Mutex::new(OutputState {
            chunks,
            closed: !hold_open,
        }));
        let wake = Arc::new(Condvar::new());
        let exit = Arc::new(Mutex::new(exit));
        let hold_open = Arc::new(AtomicBool::new(hold_open));
        let completer = ProcessCompleter {
            output: Arc::clone(&output),
            wake: Arc::clone(&wake),
            exit: Arc::clone(&exit),
            hold_open: Arc::clone(&hold_open),
        };
        (
            Arc::new(Self {
                state: Arc::clone(&state),
                output: Mutex::new(Some(output)),
                wake,
                exit,
                hold_open,
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
        let handle = FakeProcessHandle {
            state: Arc::clone(&self.state),
            output,
            wake: Arc::clone(&self.wake),
            exit: Arc::clone(&self.exit),
            hold_open: Arc::clone(&self.hold_open),
        };
        Box::pin(async move { Ok(Box::new(handle) as Box<dyn ProcessHandle>) })
    }
}

struct FakeProcessHandle {
    state: Arc<ProcessState>,
    output: Arc<Mutex<OutputState>>,
    wake: Arc<Condvar>,
    exit: Arc<Mutex<ProcessExit>>,
    hold_open: Arc<AtomicBool>,
}

impl ProcessHandle for FakeProcessHandle {
    fn write_stdin(&self, chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state
            .stdin
            .lock()
            .expect("stdin lock is available")
            .extend_from_slice(chunk.bytes());
        Box::pin(async { Ok(()) })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state.stdin_closed.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(async move {
            let mut output = self.output.lock().expect("output lock is available");
            loop {
                if let Some(chunk) = output.chunks.pop_front() {
                    return Ok(Some(chunk));
                }
                if output.closed
                    || !self.hold_open.load(Ordering::SeqCst)
                    || self.state.force_stopped.load(Ordering::SeqCst)
                {
                    return Ok(None);
                }
                output = self.wake.wait(output).expect("output condvar is available");
            }
        })
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state.stdin_closed.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state.force_stopped.store(true, Ordering::SeqCst);
        self.hold_open.store(false, Ordering::SeqCst);
        self.wake.notify_all();
        Box::pin(async { Ok(()) })
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        self.state.waited.store(true, Ordering::SeqCst);
        let exit = *self.exit.lock().expect("exit lock is available");
        Box::pin(async move { Ok(exit) })
    }
}

fn stdout_chunks(stdout: &str) -> VecDeque<ProcessOutputChunk> {
    if stdout.is_empty() {
        VecDeque::new()
    } else {
        VecDeque::from([ProcessOutputChunk::new(
            ProcessOutputStream::Stdout,
            stdout.as_bytes().to_vec(),
        )])
    }
}
