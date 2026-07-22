use super::{
    MediaChunk, MediaInputCommit, RealtimeMediaEvent, RealtimeMediaEventKind, RealtimeMediaFailure,
    RealtimeMediaFailureKind,
};
use crate::{MediaStreamId, RuntimeSessionId, RuntimeTurnId};
use swallowtail_core::{MediaDirection, RealtimeMediaConfig};

mod output;

#[derive(Debug)]
struct InputState {
    stream_id: MediaStreamId,
    next_sequence: u64,
}

#[derive(Debug)]
struct ResponseState {
    turn_id: RuntimeTurnId,
    started: bool,
    output_stream_id: Option<MediaStreamId>,
    next_output_sequence: u64,
    transcript_completed: bool,
}

#[derive(Debug)]
pub struct RealtimeMediaSessionState {
    session_id: RuntimeSessionId,
    config: RealtimeMediaConfig,
    input: Option<InputState>,
    response: Option<ResponseState>,
    next_event_sequence: u64,
    completed_turns: u32,
    reusable: bool,
}

impl RealtimeMediaSessionState {
    #[must_use]
    pub const fn new(session_id: RuntimeSessionId, config: RealtimeMediaConfig) -> Self {
        Self {
            session_id,
            config,
            input: None,
            response: None,
            next_event_sequence: 1,
            completed_turns: 0,
            reusable: true,
        }
    }

    pub fn append_input(&mut self, chunk: &MediaChunk) -> Result<(), RealtimeMediaFailure> {
        self.require_reusable()?;
        if self.response.is_some() {
            return Err(ordering("Input is not accepted while a response is active"));
        }
        if chunk.session_id() != &self.session_id {
            return Err(RealtimeMediaFailure::new(
                RealtimeMediaFailureKind::SessionMismatch,
                "Realtime media chunk belongs to a different session",
            ));
        }
        if chunk.direction() != MediaDirection::Input
            || chunk.format() != self.config.input_format()
        {
            return Err(RealtimeMediaFailure::new(
                RealtimeMediaFailureKind::FormatMismatch,
                "Realtime media input has the wrong direction or format",
            ));
        }

        let sequence = chunk.sequence().get();
        match &mut self.input {
            Some(input) => {
                if chunk.stream_id() != &input.stream_id {
                    return Err(RealtimeMediaFailure::new(
                        RealtimeMediaFailureKind::StreamMismatch,
                        "Realtime media input crossed stream identities",
                    ));
                }
                if sequence != input.next_sequence {
                    return Err(sequence_invalid());
                }
                input.next_sequence = input.next_sequence.saturating_add(1);
            }
            None => {
                if sequence != 1 {
                    return Err(sequence_invalid());
                }
                self.input = Some(InputState {
                    stream_id: chunk.stream_id().clone(),
                    next_sequence: 2,
                });
            }
        }
        Ok(())
    }

    pub fn commit_input(
        &mut self,
        turn_id: RuntimeTurnId,
        stream_id: MediaStreamId,
    ) -> Result<MediaInputCommit, RealtimeMediaFailure> {
        self.require_reusable()?;
        if self.response.is_some() {
            return Err(ordering("Only one realtime media response may be active"));
        }
        let input = self
            .input
            .take()
            .ok_or_else(|| ordering("Realtime media input must be appended before commit"))?;
        if input.stream_id != stream_id {
            self.input = Some(input);
            return Err(RealtimeMediaFailure::new(
                RealtimeMediaFailureKind::StreamMismatch,
                "Realtime media commit crossed stream identities",
            ));
        }
        self.response = Some(ResponseState {
            turn_id: turn_id.clone(),
            started: false,
            output_stream_id: None,
            next_output_sequence: 1,
            transcript_completed: false,
        });
        Ok(MediaInputCommit::new(turn_id, stream_id))
    }

    pub fn record_response_event(
        &mut self,
        event: &RealtimeMediaEvent,
    ) -> Result<(), RealtimeMediaFailure> {
        self.require_reusable()?;
        if event.sequence().get() != self.next_event_sequence {
            return Err(sequence_invalid());
        }
        let response = self
            .response
            .as_mut()
            .ok_or_else(|| ordering("Realtime media response event arrived without a commit"))?;
        if event.turn_id() != &response.turn_id {
            return Err(ordering("Realtime media response crossed turn identities"));
        }

        match event.kind() {
            RealtimeMediaEventKind::ResponseStarted if !response.started => {
                response.started = true;
            }
            RealtimeMediaEventKind::ResponseStarted => {
                return Err(ordering("Realtime media response started more than once"));
            }
            _ if !response.started => {
                return Err(ordering(
                    "Realtime media output arrived before response start",
                ));
            }
            RealtimeMediaEventKind::OutputAudio(chunk) => {
                output::validate_output(&self.session_id, &self.config, response, chunk)?;
            }
            RealtimeMediaEventKind::TranscriptDelta(_) if response.transcript_completed => {
                return Err(ordering(
                    "Realtime media transcript delta arrived after completion",
                ));
            }
            RealtimeMediaEventKind::TranscriptCompleted(_) if response.transcript_completed => {
                return Err(ordering(
                    "Realtime media transcript completed more than once",
                ));
            }
            RealtimeMediaEventKind::TranscriptCompleted(_) => {
                response.transcript_completed = true;
            }
            RealtimeMediaEventKind::TranscriptDelta(_)
            | RealtimeMediaEventKind::ProviderObservation(_) => {}
            RealtimeMediaEventKind::ResponseTerminal(status) => {
                let completed = !status.ends_session();
                self.reusable = completed;
                if completed {
                    self.completed_turns = self.completed_turns.saturating_add(1);
                }
            }
        }

        self.next_event_sequence = self.next_event_sequence.saturating_add(1);
        if matches!(event.kind(), RealtimeMediaEventKind::ResponseTerminal(_)) {
            self.response = None;
        }
        Ok(())
    }

    #[must_use]
    pub const fn response_active(&self) -> bool {
        self.response.is_some()
    }

    #[must_use]
    pub const fn is_reusable(&self) -> bool {
        self.reusable && self.completed_turns < self.config.maximum_turns().get()
    }

    pub fn close(&mut self) {
        self.reusable = false;
        self.input = None;
        self.response = None;
    }

    fn require_reusable(&self) -> Result<(), RealtimeMediaFailure> {
        if !self.reusable {
            Err(RealtimeMediaFailure::new(
                RealtimeMediaFailureKind::SessionClosed,
                "Realtime media session is no longer reusable",
            ))
        } else if self.completed_turns >= self.config.maximum_turns().get() {
            Err(RealtimeMediaFailure::new(
                RealtimeMediaFailureKind::TurnLimitReached,
                "Realtime media session reached its preflight-bound turn limit",
            ))
        } else {
            Ok(())
        }
    }
}

pub(super) fn sequence_invalid() -> RealtimeMediaFailure {
    RealtimeMediaFailure::new(
        RealtimeMediaFailureKind::SequenceInvalid,
        "Realtime media sequence contains a gap, duplicate, or regression",
    )
}

fn ordering(message: &'static str) -> RealtimeMediaFailure {
    RealtimeMediaFailure::new(RealtimeMediaFailureKind::OrderingInvalid, message)
}
