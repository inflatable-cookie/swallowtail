use super::response::{BackgroundStatus, ResponseSnapshot, parse_snapshot_value};
use crate::failure::{failure, malformed};
use serde_json::Value;
use swallowtail_runtime::RuntimeFailure;

include!("sse/decoder.rs");

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ProviderEvent {
    Created(ResponseSnapshot),
    Status(BackgroundStatus),
    OutputDelta(String),
    OutputDone(String),
    Terminal(ResponseSnapshot),
    Error,
}

pub(crate) struct BackgroundStream {
    response_id: Option<String>,
    last_sequence: Option<u64>,
    phase: StreamPhase,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StreamPhase {
    Start,
    Created,
    Queued,
    InProgress,
    Output,
    OutputDone,
    Terminal,
}

impl BackgroundStream {
    pub(crate) const fn initial() -> Self {
        Self {
            response_id: None,
            last_sequence: None,
            phase: StreamPhase::Start,
        }
    }

    #[cfg(test)]
    pub(crate) fn reattached(response_id: String, last_sequence: u64) -> Self {
        Self {
            response_id: Some(response_id),
            last_sequence: Some(last_sequence),
            phase: StreamPhase::Output,
        }
    }

    pub(crate) fn apply(&mut self, frame: &SseFrame) -> Result<ProviderEvent, RuntimeFailure> {
        let value: Value = serde_json::from_slice(&frame.data).map_err(|_| malformed())?;
        if value.get("type").and_then(Value::as_str) != Some(frame.name.as_str()) {
            return Err(protocol_order_failure());
        }
        let sequence = value
            .get("sequence_number")
            .and_then(Value::as_u64)
            .ok_or_else(malformed)?;
        if self
            .last_sequence
            .is_some_and(|last| last.checked_add(1) != Some(sequence))
        {
            return Err(protocol_order_failure());
        }

        let event = match frame.name.as_str() {
            "response.created" => {
                if self.response_id.is_some() || self.phase != StreamPhase::Start {
                    return Err(protocol_order_failure());
                }
                let snapshot = parse_snapshot_value(value.get("response").ok_or_else(malformed)?)?;
                self.phase = match snapshot.status {
                    BackgroundStatus::Queued => StreamPhase::Created,
                    BackgroundStatus::InProgress => StreamPhase::InProgress,
                    _ => return Err(protocol_order_failure()),
                };
                self.response_id = Some(snapshot.response_id.clone());
                ProviderEvent::Created(snapshot)
            }
            "response.queued" => {
                if self.phase != StreamPhase::Created {
                    return Err(protocol_order_failure());
                }
                self.phase = StreamPhase::Queued;
                self.status(&value, BackgroundStatus::Queued)?
            }
            "response.in_progress" => {
                if !matches!(self.phase, StreamPhase::Created | StreamPhase::Queued) {
                    return Err(protocol_order_failure());
                }
                self.phase = StreamPhase::InProgress;
                self.status(&value, BackgroundStatus::InProgress)?
            }
            "response.output_text.delta" => {
                if !matches!(self.phase, StreamPhase::InProgress | StreamPhase::Output) {
                    return Err(protocol_order_failure());
                }
                self.correlate(&value, "/response_id")?;
                self.phase = StreamPhase::Output;
                ProviderEvent::OutputDelta(super::response::string(&value, "/delta")?.to_owned())
            }
            "response.output_text.done" => {
                if self.phase != StreamPhase::Output {
                    return Err(protocol_order_failure());
                }
                self.correlate(&value, "/response_id")?;
                self.phase = StreamPhase::OutputDone;
                ProviderEvent::OutputDone(super::response::string(&value, "/text")?.to_owned())
            }
            "response.completed"
            | "response.incomplete"
            | "response.failed"
            | "response.cancelled" => {
                let snapshot = parse_snapshot_value(value.get("response").ok_or_else(malformed)?)?;
                self.require_response(&snapshot.response_id)?;
                if !snapshot.status.is_terminal() {
                    return Err(protocol_order_failure());
                }
                if snapshot.status == BackgroundStatus::Completed {
                    if self.phase != StreamPhase::OutputDone {
                        return Err(protocol_order_failure());
                    }
                } else if matches!(self.phase, StreamPhase::Start | StreamPhase::Terminal) {
                    return Err(protocol_order_failure());
                }
                self.phase = StreamPhase::Terminal;
                ProviderEvent::Terminal(snapshot)
            }
            "error" => {
                if self.phase == StreamPhase::Terminal {
                    return Err(protocol_order_failure());
                }
                self.phase = StreamPhase::Terminal;
                ProviderEvent::Error
            }
            _ => {
                return Err(failure(
                    "swallowtail.openai.event_unknown",
                    "OpenAI Responses returned an unsupported semantic event",
                ));
            }
        };
        self.last_sequence = Some(sequence);
        Ok(event)
    }

    #[cfg(test)]
    pub(crate) fn response_id(&self) -> Option<&str> {
        self.response_id.as_deref()
    }

    pub(crate) const fn last_sequence(&self) -> Option<u64> {
        self.last_sequence
    }

    fn status(
        &self,
        value: &Value,
        expected: BackgroundStatus,
    ) -> Result<ProviderEvent, RuntimeFailure> {
        let response = value.get("response").ok_or_else(malformed)?;
        self.require_response(super::response::string(response, "/id")?)?;
        if BackgroundStatus::parse(super::response::string(response, "/status")?)? != expected {
            return Err(protocol_order_failure());
        }
        Ok(ProviderEvent::Status(expected))
    }

    fn correlate(&self, value: &Value, pointer: &str) -> Result<(), RuntimeFailure> {
        self.require_response(super::response::string(value, pointer)?)
    }

    fn require_response(&self, response_id: &str) -> Result<(), RuntimeFailure> {
        if self.response_id.as_deref() == Some(response_id) {
            Ok(())
        } else {
            Err(failure(
                "swallowtail.openai.response_correlation_failed",
                "OpenAI Responses event did not match the active response",
            ))
        }
    }
}

fn protocol_order_failure() -> RuntimeFailure {
    failure(
        "swallowtail.openai.event_order_invalid",
        "OpenAI Responses event sequence was invalid",
    )
}
