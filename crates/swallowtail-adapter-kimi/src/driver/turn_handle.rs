struct KimiTurnHandle {
    runtime_id: swallowtail_runtime::RuntimeTurnId,
    events: Option<BoxEventStream>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: TurnCancellation,
    active: ActiveSlot,
}

impl TurnHandle for KimiTurnHandle {
    fn turn_id(&self) -> &swallowtail_runtime::RuntimeTurnId {
        &self.runtime_id
    }
    fn provider_turn_ref(&self) -> Option<&swallowtail_core::TurnRef> {
        None
    }
    fn take_events(&mut self) -> Option<BoxEventStream> {
        self.events.take()
    }
    fn cancellation(&self) -> &dyn CancellationControl {
        &self.cancellation
    }
    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>> {
        self.terminal.take()
    }
    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            if !self.cancellation.turn.is_finished() {
                let _ = self.cancellation.request().await;
            }
            let active = {
                let mut slot = self.active.lock().expect("ACP active-task lock poisoned");
                if slot
                    .as_ref()
                    .is_some_and(|active| Arc::ptr_eq(&active.turn, &self.cancellation.turn))
                {
                    slot.take()
                } else {
                    None
                }
            };
            if let Some(task) = active.and_then(|mut active| active.task.take()) {
                if task.join().await.is_err() {
                    cleanup_failure(
                        "turn_join_failed",
                        "Kimi ACP prompt task did not join cleanly",
                    )
                } else {
                    CleanupOutcome::NotApplicable
                }
            } else {
                CleanupOutcome::NotApplicable
            }
        })
    }
}
