use super::FixtureHost;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use swallowtail_runtime::{
    BoxFuture, ProcessExit, ProcessHandle, ProcessInputChunk, ProcessOutputChunk,
    ProcessOutputStream, ProcessRequest, ProcessService, RuntimeFailure, ScopeId,
};

impl ProcessService for FixtureHost {
    fn start(
        &self,
        _scope: ScopeId,
        request: ProcessRequest,
    ) -> BoxFuture<'static, Result<Box<dyn ProcessHandle>, RuntimeFailure>> {
        *self
            .process_request
            .lock()
            .expect("fixture process request lock is not poisoned") = Some(request);
        let process = FixtureProcess {
            endpoint: self
                .ready_endpoint
                .lock()
                .expect("fixture ready-endpoint lock is not poisoned")
                .clone(),
            emitted: AtomicBool::new(false),
            stopped: Arc::clone(&self.process_stopped),
            waited: Arc::clone(&self.process_waited),
        };
        Box::pin(async move { Ok(Box::new(process) as Box<dyn ProcessHandle>) })
    }
}

struct FixtureProcess {
    endpoint: String,
    emitted: AtomicBool,
    stopped: Arc<AtomicBool>,
    waited: Arc<AtomicBool>,
}

impl ProcessHandle for FixtureProcess {
    fn write_stdin(&self, _chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        Box::pin(async { Ok(()) })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        let output = if self.emitted.swap(true, Ordering::SeqCst) {
            None
        } else {
            Some(ProcessOutputChunk::new(
                ProcessOutputStream::Stdout,
                format!("Kimi server: {}/#token=fixture-private", self.endpoint).into_bytes(),
            ))
        };
        Box::pin(async move { Ok(output) })
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.stopped.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.stopped.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        self.waited.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
    }
}
