#![allow(dead_code)]

pub(crate) mod app_server;
mod preflight;
mod resume;
pub(crate) mod topology;

#[allow(unused_imports)]
pub use preflight::{
    app_server_plan, app_server_plan_for, app_server_plan_for_version, app_server_plan_with,
    bind_current_exec_policy, bounded_workspace_plan, bounded_workspace_plan_for,
    bounded_workspace_plan_for_version, current_exec_policy, exec_policy_for_version, plan,
    plan_with, plan_with_version, unqualified_app_server_plan, unqualified_exec_plan,
};
#[allow(unused_imports)]
pub use resume::{session_resume_binding, session_resume_binding_for};

use futures_executor::block_on;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use swallowtail_core::{ExecutionHostId, HostServiceKind};
use swallowtail_runtime::{
    BoxFuture, HostServices, JoinedTask, ProcessExit, ProcessHandle, ProcessInputChunk,
    ProcessOutputChunk, ProcessOutputStream, ProcessRequest, ProcessService, RuntimeFailure,
    ScopeId, ScopedTaskService, WorkingResourceRef,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservedProcessRequest {
    pub executable: String,
    pub arguments: Vec<String>,
    pub environments: Vec<String>,
    pub working_resource: Option<String>,
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

pub struct FakeProcessService {
    state: Arc<ProcessState>,
    output: Mutex<Option<VecDeque<ProcessOutputChunk>>>,
    hold_open: bool,
}

impl FakeProcessService {
    pub fn completed(jsonl: &str) -> (Arc<Self>, Arc<ProcessState>) {
        Self::new(
            [ProcessOutputChunk::new(
                ProcessOutputStream::Stdout,
                jsonl.as_bytes().to_vec(),
            )],
            false,
        )
    }

    pub fn held_open() -> (Arc<Self>, Arc<ProcessState>) {
        Self::new([], true)
    }

    fn new(
        output: impl IntoIterator<Item = ProcessOutputChunk>,
        hold_open: bool,
    ) -> (Arc<Self>, Arc<ProcessState>) {
        let state = Arc::new(ProcessState::default());
        (
            Arc::new(Self {
                state: Arc::clone(&state),
                output: Mutex::new(Some(output.into_iter().collect())),
                hold_open,
            }),
            state,
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
            output: Mutex::new(output),
            hold_open: self.hold_open,
        };
        Box::pin(async move { Ok(Box::new(handle) as Box<dyn ProcessHandle>) })
    }
}

struct FakeProcessHandle {
    state: Arc<ProcessState>,
    output: Mutex<VecDeque<ProcessOutputChunk>>,
    hold_open: bool,
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
            loop {
                if let Some(chunk) = self
                    .output
                    .lock()
                    .expect("output lock is available")
                    .pop_front()
                {
                    return Ok(Some(chunk));
                }
                if !self.hold_open || self.state.force_stopped.load(Ordering::SeqCst) {
                    return Ok(None);
                }
                thread::yield_now();
            }
        })
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state.stdin_closed.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state.force_stopped.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        self.state.waited.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
    }
}

pub struct ThreadTaskService;

struct ThreadTask(Mutex<Option<JoinHandle<()>>>);

impl JoinedTask for ThreadTask {
    fn join(self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async move {
            self.0
                .lock()
                .expect("task lock is available")
                .take()
                .expect("task joins once")
                .join()
                .expect("fake task does not panic");
            Ok(())
        })
    }
}

impl ScopedTaskService for ThreadTaskService {
    fn spawn(
        &self,
        _scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        Ok(Box::new(ThreadTask(Mutex::new(Some(thread::spawn(
            move || block_on(task),
        ))))))
    }
}

pub fn host_services(process: Arc<dyn ProcessService>) -> HostServices {
    host_services_for(
        ExecutionHostId::new("host.local").expect("host id is valid"),
        process,
    )
}

pub fn host_services_for(
    execution_host_id: ExecutionHostId,
    process: Arc<dyn ProcessService>,
) -> HostServices {
    HostServices::new(execution_host_id)
        .with_task(Arc::new(ThreadTaskService))
        .with_process(process)
}

pub fn host_services_with(
    process: Arc<dyn ProcessService>,
    recording: &swallowtail_testkit::RecordingHostServices,
    optional: impl IntoIterator<Item = HostServiceKind>,
) -> HostServices {
    host_services_with_for(
        ExecutionHostId::new("host.local").expect("host id is valid"),
        process,
        recording,
        optional,
    )
}

pub fn host_services_with_for(
    execution_host_id: ExecutionHostId,
    process: Arc<dyn ProcessService>,
    recording: &swallowtail_testkit::RecordingHostServices,
    optional: impl IntoIterator<Item = HostServiceKind>,
) -> HostServices {
    let mut services = host_services_for(execution_host_id, process);
    for kind in optional {
        services = match kind {
            HostServiceKind::Time => services.with_time(
                recording
                    .services()
                    .time()
                    .expect("recording time service is present")
                    .clone(),
            ),
            HostServiceKind::Network => services.with_network(
                recording
                    .services()
                    .network()
                    .expect("recording network service is present")
                    .clone(),
            ),
            HostServiceKind::Attachment => services.with_attachment(
                recording
                    .services()
                    .attachment()
                    .expect("recording attachment service is present")
                    .clone(),
            ),
            HostServiceKind::Schema => services.with_schema(
                recording
                    .services()
                    .schema()
                    .expect("recording schema service is present")
                    .clone(),
            ),
            HostServiceKind::WorkingResource => services.with_working_resource(
                recording
                    .services()
                    .working_resource()
                    .expect("recording working-resource service is present")
                    .clone(),
            ),
            _ => panic!("unsupported optional fixture service {kind:?}"),
        };
    }
    services
}

pub fn working_resource() -> WorkingResourceRef {
    WorkingResourceRef::new("workspace.main").expect("resource is valid")
}
