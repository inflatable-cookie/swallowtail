use super::lifecycle::{ActiveResponse, ResponseCancellation, reap_finished};
use super::pump::{PumpContext, pump_response};
use super::session::OpenAiRealtimeSession;
use crate::failure::failure;
use crate::realtime_protocol::ClientEvent;
use futures_channel::mpsc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use swallowtail_runtime::{
    BoxFuture, CancellationControl, CleanupOutcome, HostServices, MediaChunk, MediaInputCommit,
    RealtimeMediaResponseHandle, RealtimeMediaSessionHandle, RequestId, RuntimeFailure,
    RuntimeSessionId, RuntimeTurnId, terminal_outcome_channel,
};

mod response;

use response::OpenAiRealtimeResponse;

const EVENT_CAPACITY: usize = 32;

impl RealtimeMediaSessionHandle for OpenAiRealtimeSession {
    fn request_id(&self) -> &RequestId {
        &self.request_id
    }

    fn session_id(&self) -> &RuntimeSessionId {
        &self.session_id
    }

    fn append_input<'a>(
        &'a mut self,
        chunk: MediaChunk,
        services: HostServices,
    ) -> BoxFuture<'a, Result<(), RuntimeFailure>> {
        Box::pin(async move {
            services.require_execution_host(self.services.execution_host_id())?;
            reap_finished(&self.active).await?;
            self.require_reusable()?;
            self.state
                .lock()
                .expect("media state lock poisoned")
                .append_input(&chunk)
                .map_err(|error| RuntimeFailure::new(error.diagnostic().clone()))?;
            let event_id = format!("input-append-{}", self.next_append_event);
            self.next_append_event = self.next_append_event.saturating_add(1);
            if let Err(error) = self
                .worker
                .send(
                    ClientEvent::InputAudioAppend {
                        event_id: &event_id,
                        chunk: &chunk,
                    }
                    .to_json(),
                )
                .await
            {
                self.invalidate();
                return Err(error);
            }
            Ok(())
        })
    }

    fn commit_input<'a>(
        &'a mut self,
        commit: MediaInputCommit,
        services: HostServices,
    ) -> BoxFuture<'a, Result<Box<dyn RealtimeMediaResponseHandle>, RuntimeFailure>> {
        Box::pin(async move {
            services.require_execution_host(self.services.execution_host_id())?;
            reap_finished(&self.active).await?;
            self.require_reusable()?;
            if self
                .active
                .lock()
                .expect("active response lock poisoned")
                .is_some()
            {
                return Err(failure(
                    "swallowtail.openai.realtime_response_active",
                    "OpenAI Realtime session already has an active response",
                ));
            }
            self.state
                .lock()
                .expect("media state lock poisoned")
                .commit_input(commit.turn_id().clone(), commit.stream_id().clone())
                .map_err(|error| RuntimeFailure::new(error.diagnostic().clone()))?;
            self.turn_index = self.turn_index.saturating_add(1);
            let commit_event = format!("input-commit-{}", self.turn_index);
            let create_event = format!("response-create-{}", self.turn_index);
            if let Err(error) = self
                .worker
                .send(
                    ClientEvent::InputAudioCommit {
                        event_id: &commit_event,
                    }
                    .to_json(),
                )
                .await
            {
                self.invalidate();
                return Err(error);
            }
            if let Err(error) = self
                .worker
                .send(
                    ClientEvent::ResponseCreate {
                        event_id: &create_event,
                    }
                    .to_json(),
                )
                .await
            {
                self.invalidate();
                return Err(error);
            }
            self.spawn_response(commit.turn_id().clone())
        })
    }

    fn cancellation(&self) -> &dyn CancellationControl {
        self.cancellation.as_ref()
    }

    fn close(mut self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async move { self.close_inner().await })
    }
}

impl OpenAiRealtimeSession {
    fn require_reusable(&self) -> Result<(), RuntimeFailure> {
        if self.cancellation.is_requested() || !self.reusable.load(Ordering::SeqCst) {
            Err(failure(
                "swallowtail.openai.realtime_session_closed",
                "OpenAI Realtime session can no longer accept input",
            ))
        } else {
            Ok(())
        }
    }

    fn invalidate(&self) {
        self.reusable.store(false, Ordering::SeqCst);
        self.state
            .lock()
            .expect("media state lock poisoned")
            .close();
        self.worker.abort();
    }

    fn spawn_response(
        &mut self,
        turn_id: RuntimeTurnId,
    ) -> Result<Box<dyn RealtimeMediaResponseHandle>, RuntimeFailure> {
        let updates = self
            .updates
            .lock()
            .expect("updates lock poisoned")
            .take()
            .ok_or_else(|| {
                failure(
                    "swallowtail.openai.realtime_updates_unavailable",
                    "OpenAI Realtime response reader was unavailable",
                )
            })?;
        let (events, stream) = mpsc::channel(EVENT_CAPACITY);
        let (terminal_sender, terminal_future) = terminal_outcome_channel();
        let terminal_flag = Arc::new(AtomicBool::new(false));
        let cancellation = Arc::new(ResponseCancellation::new(
            self.worker.clone(),
            format!("response-cancel-{}", self.turn_index),
            Arc::clone(&self.reusable),
        ));
        let context = PumpContext {
            turn_id: turn_id.clone(),
            session_id: self.session_id.clone(),
            config: self.config.clone(),
            state: Arc::clone(&self.state),
            reusable: Arc::clone(&self.reusable),
            next_event_sequence: Arc::clone(&self.next_event_sequence),
            cancellation: Arc::clone(&cancellation),
            worker: self.worker.clone(),
            request_ref: self.worker.request_ref().clone(),
        };
        let deadline = self.deadline.map(|deadline| {
            self.services
                .time()
                .expect("validated time service")
                .wait_until(deadline)
        });
        let returned_updates = Arc::clone(&self.updates);
        let task_terminal = Arc::clone(&terminal_flag);
        let scope = swallowtail_runtime::ScopeId::new(format!(
            "openai-realtime:response:{}",
            turn_id.as_str()
        ))
        .map_err(|_| {
            failure(
                "swallowtail.openai.realtime_identity_invalid",
                "OpenAI Realtime response identity was invalid",
            )
        })?;
        let task = self
            .services
            .task()
            .expect("validated task service")
            .spawn(
                scope,
                Box::pin(async move {
                    let (outcome, updates) =
                        pump_response(updates, events, deadline, context).await;
                    *returned_updates.lock().expect("updates lock poisoned") = Some(updates);
                    task_terminal.store(true, Ordering::SeqCst);
                    let _ = terminal_sender.complete(outcome);
                }),
            )?;
        *self.active.lock().expect("active response lock poisoned") = Some(ActiveResponse {
            turn_id: turn_id.clone(),
            task: Some(task),
            cancellation: Arc::clone(&cancellation),
            terminal: Arc::clone(&terminal_flag),
        });
        Ok(Box::new(OpenAiRealtimeResponse::new(
            turn_id,
            Box::pin(stream),
            Box::pin(terminal_future),
            cancellation,
            terminal_flag,
            Arc::clone(&self.active),
            self.worker.clone(),
        )))
    }
}
