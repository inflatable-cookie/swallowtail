use serde_json::{Value, json};

mod value;

use value::{completed_output, optional_usage, string, usage};

pub const MAX_FRAME_BYTES: usize = 64 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureError {
    FrameTooLarge,
    InvalidJson,
    MissingField,
    InvalidField,
    UnknownEvent,
    CorrelationFailed,
    OrderFailed,
    OutputMismatch,
    IncompleteTurn,
    TurnActive,
    ChainInvalid,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProviderFailure {
    PreviousResponseNotFound,
    ConnectionLimitReached,
    Other,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    Created {
        response_id: String,
    },
    InProgress {
        response_id: String,
        usage: Option<TurnEvidence>,
    },
    TextDelta {
        response_id: String,
        delta: String,
    },
    TextDone {
        response_id: String,
        text: String,
    },
    Completed {
        response_id: String,
        output: String,
        usage: TurnEvidence,
    },
    ProviderFailed(ProviderFailure),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TurnEvidence {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub total_tokens: u64,
    pub cost_in_usd_ticks: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Continuation(String);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TurnState {
    Idle,
    Created,
    Streaming,
}

#[derive(Debug)]
pub struct Conversation {
    model: String,
    continuation: Option<Continuation>,
    active_response: Option<String>,
    state: TurnState,
    output: String,
    text_done: Option<String>,
    latest_evidence: Option<TurnEvidence>,
    chain_valid: bool,
}

impl Conversation {
    pub fn new(model: &str) -> Self {
        Self {
            model: model.to_owned(),
            continuation: None,
            active_response: None,
            state: TurnState::Idle,
            output: String::new(),
            text_done: None,
            latest_evidence: None,
            chain_valid: true,
        }
    }

    pub fn begin_turn(&mut self, input: &str) -> Result<String, FixtureError> {
        if self.state != TurnState::Idle {
            return Err(FixtureError::TurnActive);
        }
        if !self.chain_valid {
            return Err(FixtureError::ChainInvalid);
        }

        let mut request = json!({
            "type": "response.create",
            "model": self.model,
            "store": false,
            "input": [{
                "type": "message",
                "role": "user",
                "content": [{"type": "input_text", "text": input}]
            }],
            "tools": []
        });
        if let Some(continuation) = &self.continuation {
            request["previous_response_id"] = Value::String(continuation.0.clone());
        }

        self.state = TurnState::Created;
        self.active_response = None;
        self.output.clear();
        self.text_done = None;
        self.latest_evidence = None;
        serde_json::to_string(&request).map_err(|_| FixtureError::InvalidJson)
    }

    pub fn apply(&mut self, event: Event) -> Result<Option<TurnEvidence>, FixtureError> {
        match event {
            Event::Created { response_id } => {
                if self.state != TurnState::Created || self.active_response.is_some() {
                    return Err(FixtureError::OrderFailed);
                }
                self.active_response = Some(response_id);
                self.state = TurnState::Streaming;
                Ok(None)
            }
            Event::InProgress { response_id, usage } => {
                self.require_active(&response_id)?;
                if let Some(usage) = usage {
                    self.latest_evidence = Some(usage);
                }
                Ok(None)
            }
            Event::TextDelta { response_id, delta } => {
                self.require_active(&response_id)?;
                if self.text_done.is_some() {
                    return Err(FixtureError::OrderFailed);
                }
                self.output.push_str(&delta);
                Ok(None)
            }
            Event::TextDone { response_id, text } => {
                self.require_active(&response_id)?;
                if self.text_done.replace(text).is_some() {
                    return Err(FixtureError::OrderFailed);
                }
                Ok(None)
            }
            Event::Completed {
                response_id,
                output,
                usage,
            } => {
                self.require_active(&response_id)?;
                if self.text_done.as_deref() != Some(output.as_str()) || self.output != output {
                    return Err(FixtureError::OutputMismatch);
                }
                self.latest_evidence = Some(usage.clone());
                self.continuation = Some(Continuation(response_id));
                self.active_response = None;
                self.state = TurnState::Idle;
                Ok(Some(usage))
            }
            Event::ProviderFailed(failure) => {
                if failure == ProviderFailure::PreviousResponseNotFound {
                    self.chain_valid = false;
                    self.continuation = None;
                }
                self.active_response = None;
                self.state = TurnState::Idle;
                Ok(None)
            }
        }
    }

    pub fn disconnect(&mut self) -> Result<(), FixtureError> {
        self.chain_valid = false;
        self.continuation = None;
        self.active_response = None;
        let was_complete = self.state == TurnState::Idle;
        self.state = TurnState::Idle;
        if was_complete {
            Ok(())
        } else {
            Err(FixtureError::IncompleteTurn)
        }
    }

    pub const fn latest_evidence(&self) -> Option<&TurnEvidence> {
        self.latest_evidence.as_ref()
    }

    fn require_active(&self, response_id: &str) -> Result<(), FixtureError> {
        if self.state != TurnState::Streaming {
            return Err(FixtureError::OrderFailed);
        }
        if self.active_response.as_deref() != Some(response_id) {
            return Err(FixtureError::CorrelationFailed);
        }
        Ok(())
    }
}

pub fn parse_event(frame: &str) -> Result<Event, FixtureError> {
    if frame.len() > MAX_FRAME_BYTES {
        return Err(FixtureError::FrameTooLarge);
    }
    let value: Value = serde_json::from_str(frame).map_err(|_| FixtureError::InvalidJson)?;
    let event_type = string(&value, "/type")?;
    match event_type {
        "response.created" => Ok(Event::Created {
            response_id: string(&value, "/response/id")?.to_owned(),
        }),
        "response.in_progress" => Ok(Event::InProgress {
            response_id: string(&value, "/response/id")?.to_owned(),
            usage: optional_usage(&value, "/response/usage")?,
        }),
        "response.output_text.delta" => Ok(Event::TextDelta {
            response_id: string(&value, "/response_id")?.to_owned(),
            delta: string(&value, "/delta")?.to_owned(),
        }),
        "response.output_text.done" => Ok(Event::TextDone {
            response_id: string(&value, "/response_id")?.to_owned(),
            text: string(&value, "/text")?.to_owned(),
        }),
        "response.completed" => Ok(Event::Completed {
            response_id: string(&value, "/response/id")?.to_owned(),
            output: completed_output(&value)?,
            usage: usage(&value, "/response/usage")?,
        }),
        "error" => Ok(Event::ProviderFailed(
            match string(&value, "/error/code")? {
                "previous_response_not_found" => ProviderFailure::PreviousResponseNotFound,
                "websocket_connection_limit_reached" => ProviderFailure::ConnectionLimitReached,
                _ => ProviderFailure::Other,
            },
        )),
        _ => Err(FixtureError::UnknownEvent),
    }
}
