impl ProcessHandle for ScriptedAppServerHandle {
    fn write_stdin(&self, chunk: ProcessInputChunk) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.accept_input(chunk.bytes());
        Box::pin(async { Ok(()) })
    }

    fn close_stdin(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state.closed.store(true, Ordering::SeqCst);
        let fail = matches!(
            self.mode,
            AppServerMode::LifecycleCleanupFailure
                | AppServerMode::ThreadCatalogue(ThreadCatalogueMode::CleanupFailure)
        );
        Box::pin(async move {
            if fail {
                Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                    "swallowtail.codex.fixture_cleanup_failed",
                    "fixture cleanup failed",
                )))
            } else {
                Ok(())
            }
        })
    }

    fn read_output(&self) -> BoxFuture<'_, Result<Option<ProcessOutputChunk>, RuntimeFailure>> {
        Box::pin(async move {
            loop {
                if let Some(chunk) = self
                    .state
                    .output
                    .lock()
                    .expect("output lock is available")
                    .pop_front()
                {
                    return Ok(Some(chunk));
                }
                if self.state.closed.load(Ordering::SeqCst)
                    || self.state.forced.load(Ordering::SeqCst)
                {
                    return Ok(None);
                }
                thread::yield_now();
            }
        })
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state.closed.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn force_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state.forced.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(()) })
    }

    fn wait(&self) -> BoxFuture<'_, Result<ProcessExit, RuntimeFailure>> {
        self.state.waited.store(true, Ordering::SeqCst);
        Box::pin(async { Ok(ProcessExit::new(true, Some(0))) })
    }
}
