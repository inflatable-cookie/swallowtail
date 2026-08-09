#![allow(dead_code)]

use futures_executor::block_on;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use swallowtail_core::{ExecutionHostId, ResourceAccess, ResourceRepresentation};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, Deadline, DeadlineObservation, HostServices, JoinedTask,
    MonotonicInstant, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk,
    ProcessRequest, ProcessService, ResourceLease, RuntimeFailure, ScopeId, ScopedTaskService,
    TimeService, WorkingResourceRef, WorkingResourceService,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservedProcess {
    pub executable: String,
    pub arguments: Vec<String>,
    pub environments: Vec<String>,
    pub working_resource: Option<String>,
}

pub struct FixtureHost {
    process: Arc<FixtureProcessService>,
    process_state: Arc<ProcessState>,
    joined: Arc<AtomicBool>,
}

impl FixtureHost {
    pub fn scripted(outputs: impl IntoIterator<Item = &'static str>) -> Self {
        let outputs = outputs
            .into_iter()
            .map(|output| {
                VecDeque::from([stdout_chunk(output.as_bytes().to_vec())])
            })
            .collect::<VecDeque<_>>();
        Self::from_scripted(outputs, ProcessExit::new(true, Some(0)), false)
    }

    pub fn completed(outputs: impl IntoIterator<Item = ProcessOutputChunk>) -> Self {
        Self::with_exit(outputs, ProcessExit::new(true, Some(0)))
    }

    pub fn with_exit(
        outputs: impl IntoIterator<Item = ProcessOutputChunk>,
        exit: ProcessExit,
    ) -> Self {
        Self::from_scripted(
            VecDeque::from([outputs.into_iter().collect()]),
            exit,
            false,
        )
    }

    pub fn held_open() -> Self {
        Self::from_scripted(
            VecDeque::from([VecDeque::new()]),
            ProcessExit::new(false, Some(130)),
            true,
        )
    }

    fn from_scripted(
        outputs: VecDeque<VecDeque<ProcessOutputChunk>>,
        exit: ProcessExit,
        hold_open: bool,
    ) -> Self {
        let process_state = Arc::new(ProcessState::default());
        Self {
            process: Arc::new(FixtureProcessService {
                outputs: Mutex::new(outputs),
                exit,
                hold_open,
                state: Arc::clone(&process_state),
            }),
            process_state,
            joined: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn services(&self, host_id: ExecutionHostId) -> HostServices {
        self.services_with_time(host_id, Arc::new(PendingTime))
    }

    pub fn services_with_time(
        &self,
        host_id: ExecutionHostId,
        time: Arc<dyn TimeService>,
    ) -> HostServices {
        HostServices::new(host_id)
            .with_task(Arc::new(ThreadTaskService {
                joined: Arc::clone(&self.joined),
            }))
            .with_time(time)
            .with_process(self.process.clone())
            .with_working_resource(Arc::new(FixtureWorkingResource))
    }

    pub fn observations(&self) -> Vec<ObservedProcess> {
        self.process_state
            .observed
            .lock()
            .expect("fixture process lock is available")
            .clone()
    }

    pub fn observed(&self) -> ObservedProcess {
        self.observations()
            .last()
            .cloned()
            .expect("process was observed")
    }

    pub fn started(&self) -> bool {
        !self.observations().is_empty()
    }

    pub fn stdin_closed(&self) -> bool {
        self.process_state.stdin_closed.load(Ordering::SeqCst)
    }

    pub fn stdin(&self) -> Vec<u8> {
        self.process_state
            .stdin
            .lock()
            .expect("fixture stdin lock is available")
            .clone()
    }

    pub fn waited(&self) -> bool {
        self.process_state.waited.load(Ordering::SeqCst)
    }

    pub fn joined(&self) -> bool {
        self.joined.load(Ordering::SeqCst)
    }

    pub fn force_stopped(&self) -> bool {
        self.process_state.force_stopped.load(Ordering::SeqCst)
    }
}

pub fn stdout_chunk(bytes: Vec<u8>) -> ProcessOutputChunk {
    ProcessOutputChunk::new(swallowtail_runtime::ProcessOutputStream::Stdout, bytes)
}

pub fn stderr_chunk(bytes: Vec<u8>) -> ProcessOutputChunk {
    ProcessOutputChunk::new(swallowtail_runtime::ProcessOutputStream::Stderr, bytes)
}

struct FixtureProcessService {
    outputs: Mutex<VecDeque<VecDeque<ProcessOutputChunk>>>,
    exit: ProcessExit,
    hold_open: bool,
    state: Arc<ProcessState>,
}

#[derive(Default)]
struct ProcessState {
    observed: Mutex<Vec<ObservedProcess>>,
    stdin_closed: AtomicBool,
    stdin: Mutex<Vec<u8>>,
    force_stopped: AtomicBool,
    waited: AtomicBool,
}

impl ProcessService for FixtureProcessService {
    fn start(
        &self,
        _scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        self.state
            .observed
            .lock()
            .expect("fixture process lock is available")
            .push(ObservedProcess {
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
        let outputs = self
            .outputs
            .lock()
            .expect("fixture output lock is available")
            .pop_front()
            .expect("fixture process has a scripted output");
        let handle = FixtureProcessHandle {
            outputs: Mutex::new(outputs),
            exit: self.exit,
            hold_open: self.hold_open,
            state: Arc::clone(&self.state),
        };
        Box::pin(async move { Ok(Box::new(handle) as Box<dyn ProcessHandle>) })
    }
}

struct FixtureProcessHandle {
    outputs: Mutex<VecDeque<ProcessOutputChunk>>,
    exit: ProcessExit,
    hold_open: bool,
    state: Arc<ProcessState>,
}

impl ProcessHandle for FixtureProcessHandle {
    fn write_stdin(&self, chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state
            .stdin
            .lock()
            .expect("fixture stdin lock is available")
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
                if let Some(output) = self
                    .outputs
                    .lock()
                    .expect("fixture output lock is available")
                    .pop_front()
                {
                    return Ok(Some(output));
                }
                if !self.hold_open || self.state.force_stopped.load(Ordering::SeqCst) {
                    return Ok(None);
                }
                std::thread::yield_now();
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
        let exit = self.exit;
        Box::pin(async move { Ok(exit) })
    }
}

struct ThreadTaskService {
    joined: Arc<AtomicBool>,
}

impl ScopedTaskService for ThreadTaskService {
    fn spawn(
        &self,
        _scope: ScopeId,
        task: BoxFuture<'static, ()>,
    ) -> Result<Box<dyn JoinedTask>, RuntimeFailure> {
        Ok(Box::new(ThreadTask {
            handle: Some(std::thread::spawn(move || block_on(task))),
            joined: Arc::clone(&self.joined),
        }))
    }
}

struct ThreadTask {
    handle: Option<std::thread::JoinHandle<()>>,
    joined: Arc<AtomicBool>,
}

impl JoinedTask for ThreadTask {
    fn join(mut self: Box<Self>) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        let handle = self.handle.take().expect("fixture task joins once");
        Box::pin(async move {
            handle.join().expect("fixture task does not panic");
            self.joined.store(true, Ordering::SeqCst);
            Ok(())
        })
    }
}

struct FixtureWorkingResource;

impl WorkingResourceService for FixtureWorkingResource {
    fn resolve(
        &self,
        scope: ScopeId,
        reference: WorkingResourceRef,
        access: ResourceAccess,
        representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>> {
        Box::pin(async move {
            Ok(ResourceLease::consumer_owned(
                scope,
                reference,
                access,
                representation,
            ))
        })
    }

    fn create_temporary(
        &self,
        _scope: ScopeId,
        _access: ResourceAccess,
        _representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>> {
        Box::pin(async {
            Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                "fixture.temporary_resource_unsupported",
                "Fixture does not create temporary resources",
            )))
        })
    }

    fn release(&self, _lease: ResourceLease) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async { CleanupOutcome::NotApplicable })
    }
}

struct PendingTime;

impl TimeService for PendingTime {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(0)
    }

    fn wait_until(&self, _deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(std::future::pending())
    }
}

pub struct ImmediateTime;

impl TimeService for ImmediateTime {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(1_000)
    }

    fn wait_until(&self, deadline: Deadline) -> BoxFuture<'static, DeadlineObservation> {
        Box::pin(async move { DeadlineObservation::new(deadline, deadline.instant()) })
    }
}
