use super::RealtimeServerEvent;
use crate::failure::failure;
use swallowtail_runtime::RuntimeFailure;

pub(crate) struct RealtimeServerStream {
    active_response: Option<String>,
    terminal: bool,
}

impl RealtimeServerStream {
    pub(crate) const fn new() -> Self {
        Self {
            active_response: None,
            terminal: false,
        }
    }

    pub(crate) fn apply(&mut self, event: &RealtimeServerEvent) -> Result<(), RuntimeFailure> {
        match event {
            RealtimeServerEvent::ResponseStarted(response) if self.active_response.is_none() => {
                self.active_response = Some(response.clone());
            }
            RealtimeServerEvent::ResponseStarted(_) => return Err(ordering()),
            RealtimeServerEvent::AudioDelta { response_id, .. }
            | RealtimeServerEvent::AudioCompleted { response_id }
            | RealtimeServerEvent::TranscriptDelta { response_id, .. }
            | RealtimeServerEvent::TranscriptCompleted { response_id, .. }
            | RealtimeServerEvent::Usage { response_id, .. }
                if self.active_response.as_ref() != Some(response_id) =>
            {
                return Err(ordering());
            }
            RealtimeServerEvent::Usage { .. } | RealtimeServerEvent::ProviderFailed => {
                self.active_response = None;
                self.terminal = true;
            }
            _ => {}
        }
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn disconnected(&self) -> Result<(), RuntimeFailure> {
        if self.terminal {
            Ok(())
        } else {
            Err(failure(
                "swallowtail.openai.realtime_disconnected",
                "OpenAI Realtime connection ended before a terminal event",
            ))
        }
    }
}

fn ordering() -> RuntimeFailure {
    failure(
        "swallowtail.openai.realtime_event_order_invalid",
        "OpenAI Realtime event ordering or correlation is invalid",
    )
}
