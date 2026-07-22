use crate::realtime::lifecycle::{ActiveSlot, ResponseCancellation, join_response};
use crate::realtime::worker::WorkerHandle;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use swallowtail_runtime::{
    BoxFuture, BoxRealtimeMediaEventStream, CancellationControl, CleanupOutcome,
    RealtimeMediaResponseHandle, RuntimeTurnId, TerminalOutcome,
};

pub(super) struct OpenAiRealtimeResponse {
    turn_id: RuntimeTurnId,
    events: Option<BoxRealtimeMediaEventStream>,
    terminal: Option<BoxFuture<'static, TerminalOutcome>>,
    cancellation: Arc<ResponseCancellation>,
    terminal_flag: Arc<AtomicBool>,
    active: ActiveSlot,
    worker: WorkerHandle,
}

impl OpenAiRealtimeResponse {
    pub(super) fn new(
        turn_id: RuntimeTurnId,
        events: BoxRealtimeMediaEventStream,
        terminal: BoxFuture<'static, TerminalOutcome>,
        cancellation: Arc<ResponseCancellation>,
        terminal_flag: Arc<AtomicBool>,
        active: ActiveSlot,
        worker: WorkerHandle,
    ) -> Self {
        Self {
            turn_id,
            events: Some(events),
            terminal: Some(terminal),
            cancellation,
            terminal_flag,
            active,
            worker,
        }
    }
}

impl RealtimeMediaResponseHandle for OpenAiRealtimeResponse {
    fn turn_id(&self) -> &RuntimeTurnId {
        &self.turn_id
    }

    fn take_events(&mut self) -> Option<BoxRealtimeMediaEventStream> {
        self.events.take()
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        self.cancellation.as_ref()
    }

    fn take_terminal_outcome(&mut self) -> Option<BoxFuture<'static, TerminalOutcome>> {
        self.terminal.take()
    }

    fn close(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move {
            if !self.terminal_flag.load(Ordering::SeqCst) {
                let _ = self.cancellation.request().await;
                let _ = self.worker.close().await;
            }
            join_response(&self.active, &self.turn_id).await
        })
    }
}
