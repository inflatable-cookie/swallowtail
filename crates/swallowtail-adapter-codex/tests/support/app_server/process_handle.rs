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
            let ignore_eof = matches!(
                self.mode, AppServerMode::ThreadCatalogue(ThreadCatalogueMode::IgnoreEof)
            );
            let mut eof_since = None;
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
                if self.state.forced.load(Ordering::SeqCst) {
                    return Ok(None);
                }
                if self.state.closed.load(Ordering::SeqCst) {
                    if !ignore_eof
                        || eof_since.get_or_insert_with(std::time::Instant::now).elapsed()
                            >= std::time::Duration::from_secs(3)
                    {
                        return Ok(None);
                    }
                    thread::sleep(std::time::Duration::from_millis(1));
                } else {
                    thread::yield_now();
                }
            }
        })
    }

    fn request_stop(&self) -> BoxFuture<'_, Result<(), RuntimeFailure>> {
        self.state.stop_requested.store(true, Ordering::SeqCst);
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
