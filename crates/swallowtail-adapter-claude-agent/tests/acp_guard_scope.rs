//! Test-owned process lifetime; EOF is a descriptor observation, never tree attestation.
use super::*;
use nix::fcntl::{FcntlArg, FdFlag, fcntl};
use std::os::unix::net::UnixStream;
use std::sync::{Condvar, Mutex};
use swallowtail_runtime::{
    BoxFuture, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk, RuntimeFailure,
};

struct State {
    handles: Vec<Arc<dyn ProcessHandle>>,
    deadline: Instant,
    stopping: bool,
    done: bool,
    timed_out: bool,
    waits: Vec<Value>,
    pending: usize,
}
struct Owner {
    state: Mutex<State>,
    changed: Condvar,
    workers: Mutex<Vec<std::thread::JoinHandle<()>>>,
}
impl Owner {
    fn stop(&self) {
        let handles = {
            let mut state = self.state.lock().expect("owned fixture lock");
            if state.stopping {
                while !state.done {
                    state = self.changed.wait(state).expect("owned cleanup completion");
                }
                return;
            }
            state.stopping = true;
            self.changed.notify_all();
            state.handles.clone()
        };
        // LocalProcessHost::wait completes only after its private group-owner
        // wait and output-reader joins. It still reports RootOnly.
        let mut waits = vec![];
        for handle in &handles {
            let result = block_on(handle.request_stop());
            waits.push(json!({"stop":result.is_ok()}));
        }
        for (handle, record) in handles.iter().zip(&mut waits) {
            let result = block_on(handle.wait());
            *record = match result {
                Ok(exit) => {
                    json!({"stop":record["stop"],"wait":true,"success":exit.success(),"code":exit.code(),"tree":format!("{:?}",exit.tree_completion())})
                }
                Err(error) => {
                    json!({"stop":record["stop"],"wait":false,"diagnostic":error.diagnostic().code()})
                }
            };
        }
        {
            let mut state = self.state.lock().expect("owned fixture lock");
            state.waits.extend(waits);
            while state.pending != 0 {
                state = self
                    .changed
                    .wait(state)
                    .expect("pending starts complete cleanup");
            }
        }
        for worker in self.workers.lock().expect("owned start workers").drain(..) {
            worker.join().expect("owned start worker joined");
        }
        let mut state = self.state.lock().expect("owned fixture lock");
        state.done = true;
        self.changed.notify_all();
    }
    fn register(
        &self,
        handle: Arc<dyn ProcessHandle>,
    ) -> Result<Box<dyn ProcessHandle>, RuntimeFailure> {
        {
            let mut state = self.state.lock().expect("owned fixture lock");
            if !state.stopping {
                state.handles.push(handle.clone());
                return Ok(Box::new(TrackedHandle(handle)));
            }
        }
        // Admission finishing after the fixture deadline is not an unowned
        // late success. Keep its capability until stop and wait finish.
        let stop_result = block_on(handle.force_stop());
        let joined = block_on(handle.wait());
        stop_result?;
        let exit = joined?;
        self.state.lock().expect("owned fixture lock").waits.push(json!({"stop":true,"wait":true,"late":true,"success":exit.success(),"code":exit.code(),"tree":format!("{:?}",exit.tree_completion())}));
        Err(stopped())
    }
}
fn stopped() -> RuntimeFailure {
    RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
        "swallowtail.test.fixture_stopped",
        "Owned test fixture has stopped",
    ))
}
struct TrackedHandle(Arc<dyn ProcessHandle>);
impl ProcessHandle for TrackedHandle {
    fn write_stdin(&self, chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.0.write_stdin(chunk)
    }
    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.0.close_stdin()
    }
    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        self.0.read_output()
    }
    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.0.request_stop()
    }
    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.0.force_stop()
    }
    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        self.0.wait()
    }
}
struct TrackedService {
    local: LocalProcessHost,
    owner: Arc<Owner>,
}
impl ProcessService for TrackedService {
    fn start(
        &self,
        _scope: ScopeId,
        _request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        // This fixture admits only the production checked path.
        Box::pin(async { Err(stopped()) })
    }
    fn validate_checked(
        &self,
        scope: ScopeId,
        binding: CheckedProcessRef,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        self.local.validate_checked(scope, binding, request)
    }
    fn start_checked(
        &self,
        scope: ScopeId,
        binding: CheckedProcessRef,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        // Drive bootstrap to completion even if the prepared caller drops its
        // response future. The worker owns start, so a failed worker creation
        // has not created an OS child or helper.
        let mut workers = self.owner.workers.lock().expect("owned start workers");
        {
            let mut state = self.owner.state.lock().expect("owned fixture lock");
            if state.stopping {
                return Box::pin(async { Err(stopped()) });
            }
            state.pending += 1;
        }
        let local = self.local.clone();
        let owner = self.owner.clone();
        let (send, response) = futures_channel::oneshot::channel();
        let worker = std::thread::Builder::new()
            .name("acp-fixture-start".into())
            .spawn(move || {
                let result = block_on(local.start_checked(scope, binding, request))
                    .and_then(|process| owner.register(Arc::from(process)));
                let _ = send.send(result);
                let mut state = owner.state.lock().expect("owned fixture lock");
                state.pending -= 1;
                owner.changed.notify_all();
            });
        match worker {
            Ok(worker) => workers.push(worker),
            Err(_) => {
                let mut state = self.owner.state.lock().expect("owned fixture lock");
                state.pending -= 1;
                self.owner.changed.notify_all();
                return Box::pin(async { Err(stopped()) });
            }
        }
        Box::pin(async move { response.await.map_err(|_| stopped())? })
    }
}

pub(super) struct FixtureScope {
    owner: Arc<Owner>,
    watchdog: Option<std::thread::JoinHandle<()>>,
    writer: Option<UnixStream>,
    reader: UnixStream,
    finished: Option<Value>,
    receipt: Arc<Mutex<Option<Value>>>,
    live_inheritance: bool,
}
impl FixtureScope {
    pub(super) fn new(local: LocalProcessHost) -> (Self, Arc<dyn ProcessService>) {
        // Our diagnostic descriptor, never Effigy's cleanup pin. Keep the
        // read end CLOEXEC and deliberately inherit only the write end.
        let (reader, writer) = UnixStream::pair().expect("owned diagnostic socket");
        let mut flags = FdFlag::from_bits_truncate(
            fcntl(&writer, FcntlArg::F_GETFD).expect("own descriptor flags"),
        );
        flags.remove(FdFlag::FD_CLOEXEC);
        fcntl(&writer, FcntlArg::F_SETFD(flags)).expect("intentional own diagnostic inheritance");
        reader
            .set_nonblocking(true)
            .expect("bounded diagnostic read");
        let owner = Arc::new(Owner {
            state: Mutex::new(State {
                handles: vec![],
                deadline: Instant::now() + Duration::from_secs(30),
                stopping: false,
                done: false,
                timed_out: false,
                waits: vec![],
                pending: 0,
            }),
            changed: Condvar::new(),
            workers: Mutex::new(vec![]),
        });
        let watched = owner.clone();
        let watchdog = std::thread::spawn(move || {
            let mut state = watched.state.lock().expect("owned fixture lock");
            while !state.stopping {
                let now = Instant::now();
                if now >= state.deadline {
                    state.timed_out = true;
                    drop(state);
                    watched.stop();
                    return;
                }
                let remaining = state.deadline - now;
                state = watched
                    .changed
                    .wait_timeout(state, remaining)
                    .expect("fixture deadline")
                    .0;
            }
        });
        let service = Arc::new(TrackedService {
            local,
            owner: owner.clone(),
        });
        (
            Self {
                owner,
                watchdog: Some(watchdog),
                writer: Some(writer),
                reader,
                finished: None,
                receipt: Arc::new(Mutex::new(None)),
                live_inheritance: false,
            },
            service,
        )
    }
    pub(super) fn receipt(&self) -> Arc<Mutex<Option<Value>>> {
        self.receipt.clone()
    }
    pub(super) fn expire_soon(&self) {
        self.owner
            .state
            .lock()
            .expect("owned fixture lock")
            .deadline = Instant::now() + Duration::from_millis(150);
        self.owner.changed.notify_all();
    }
    pub(super) fn prove_live_inheritance(&mut self) {
        drop(self.writer.take());
        let result = self.reader.read(&mut [0]);
        self.live_inheritance = true;
        assert!(
            result.is_err_and(|e| e.kind() == std::io::ErrorKind::WouldBlock),
            "the live owned child/helper must still hold our diagnostic descriptor"
        );
    }
    pub(super) fn finish(&mut self) -> Value {
        if let Some(value) = &self.finished {
            return value.clone();
        }
        self.owner.stop();
        if let Some(watchdog) = self.watchdog.take() {
            watchdog.join().expect("fixture watchdog joined");
        }
        // Close only our parent copy AFTER stop/waits. A leaked child keeps
        // this read non-EOF and fails the counterexample instead of hiding it.
        drop(self.writer.take());
        let deadline = Instant::now() + Duration::from_secs(2);
        let eof = loop {
            match self.reader.read(&mut [0]) {
                Ok(0) => break true,
                Err(e)
                    if e.kind() == std::io::ErrorKind::WouldBlock && Instant::now() < deadline =>
                {
                    std::thread::sleep(Duration::from_millis(5))
                }
                _ => break false,
            }
        };
        let state = self.owner.state.lock().expect("owned fixture lock");
        let result = json!({"waits":state.waits,"timedOut":state.timed_out,"ownDescriptorEof":eof,"kernelTreeAttestation":false,"liveInheritanceObserved":self.live_inheritance});
        *self.receipt.lock().expect("cleanup receipt") = Some(result.clone());
        self.finished = Some(result.clone());
        result
    }
}
impl Drop for FixtureScope {
    fn drop(&mut self) {
        let result = self.finish();
        eprintln!("ACP_FIXTURE_CLEANUP {result}");
        if !std::thread::panicking() {
            assert!(result["ownDescriptorEof"] == true);
            assert!(
                result["waits"]
                    .as_array()
                    .expect("wait receipts")
                    .iter()
                    .all(|r| r["stop"] == true && r["wait"] == true)
            );
        }
    }
}
