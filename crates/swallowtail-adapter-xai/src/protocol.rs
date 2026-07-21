use crate::failure::failure;
use serde_json::{Value, json};
use swallowtail_runtime::{RuntimeFailure, TokenUsage};

pub(crate) const MAX_FRAME_BYTES: usize = 64 * 1024;

pub(crate) struct Request;

impl Request {
    pub(crate) fn turn(
        model: &str,
        input: &str,
        continuation: Option<&str>,
    ) -> Result<String, RuntimeFailure> {
        let mut value = json!({
            "type": "response.create",
            "model": model,
            "store": false,
            "input": [{
                "type": "message",
                "role": "user",
                "content": [{"type": "input_text", "text": input}]
            }],
            "tools": []
        });
        if let Some(continuation) = continuation {
            value["previous_response_id"] = Value::String(continuation.to_owned());
        }
        serde_json::to_string(&value).map_err(|_| malformed())
    }
}

pub(crate) enum ProviderFailure {
    PreviousResponseNotFound,
    ConnectionLimitReached,
    Other,
}

pub(crate) enum TurnUpdate {
    None,
    Delta(String),
    Complete {
        continuation: String,
        output: String,
        usage: TokenUsage,
        cost_in_usd_ticks: u64,
    },
    ProviderFailed(ProviderFailure),
}

#[derive(Default)]
pub(crate) struct TurnState {
    response_id: Option<String>,
    output: String,
    text_done: Option<String>,
    phase: Phase,
}

#[derive(Clone, Copy, Default, Eq, PartialEq)]
enum Phase {
    #[default]
    Start,
    Created,
    Streaming,
}

impl TurnState {
    pub(crate) fn apply(&mut self, frame: &str) -> Result<TurnUpdate, RuntimeFailure> {
        if frame.len() > MAX_FRAME_BYTES {
            return Err(failure(
                "swallowtail.xai.frame_too_large",
                "xAI WebSocket frame exceeded the adapter limit",
            ));
        }
        let value: Value = serde_json::from_str(frame).map_err(|_| malformed())?;
        match string(&value, "/type")? {
            "response.created" => self.created(&value),
            "response.in_progress" => self.in_progress(&value),
            "response.output_text.delta" => self.delta(&value),
            "response.output_text.done" => self.text_done(&value),
            "response.completed" => self.completed(&value),
            "error" => provider_failure(&value),
            _ => Err(failure(
                "swallowtail.xai.event_unknown",
                "xAI WebSocket returned an unsupported semantic event",
            )),
        }
    }

    fn created(&mut self, value: &Value) -> Result<TurnUpdate, RuntimeFailure> {
        if self.phase != Phase::Start {
            return Err(order_failure());
        }
        self.response_id = Some(string(value, "/response/id")?.to_owned());
        self.phase = Phase::Created;
        Ok(TurnUpdate::None)
    }

    fn in_progress(&mut self, value: &Value) -> Result<TurnUpdate, RuntimeFailure> {
        self.correlate(string(value, "/response/id")?)?;
        if self.phase != Phase::Created {
            return Err(order_failure());
        }
        self.phase = Phase::Streaming;
        Ok(TurnUpdate::None)
    }

    fn delta(&mut self, value: &Value) -> Result<TurnUpdate, RuntimeFailure> {
        self.correlate(string(value, "/response_id")?)?;
        if self.phase != Phase::Streaming || self.text_done.is_some() {
            return Err(order_failure());
        }
        let delta = string(value, "/delta")?.to_owned();
        self.output.push_str(&delta);
        Ok(TurnUpdate::Delta(delta))
    }

    fn text_done(&mut self, value: &Value) -> Result<TurnUpdate, RuntimeFailure> {
        self.correlate(string(value, "/response_id")?)?;
        if self.phase != Phase::Streaming || self.text_done.is_some() {
            return Err(order_failure());
        }
        self.text_done = Some(string(value, "/text")?.to_owned());
        Ok(TurnUpdate::None)
    }

    fn completed(&mut self, value: &Value) -> Result<TurnUpdate, RuntimeFailure> {
        let response_id = string(value, "/response/id")?;
        self.correlate(response_id)?;
        if self.phase != Phase::Streaming {
            return Err(order_failure());
        }
        let output = completed_output(value)?;
        if self.text_done.as_deref() != Some(output.as_str()) || self.output != output {
            return Err(failure(
                "swallowtail.xai.output_mismatch",
                "xAI WebSocket completed output did not match ordered text events",
            ));
        }
        let usage = value.pointer("/response/usage").ok_or_else(malformed)?;
        let input_tokens = integer(usage, "/input_tokens")?;
        let output_tokens = integer(usage, "/output_tokens")?;
        let total_tokens = integer(usage, "/total_tokens")?;
        if input_tokens.checked_add(output_tokens) != Some(total_tokens) {
            return Err(malformed());
        }
        Ok(TurnUpdate::Complete {
            continuation: response_id.to_owned(),
            output,
            usage: TokenUsage::new(Some(input_tokens), Some(output_tokens)),
            cost_in_usd_ticks: integer(usage, "/cost_in_usd_ticks")?,
        })
    }

    fn correlate(&self, response_id: &str) -> Result<(), RuntimeFailure> {
        if self.response_id.as_deref() == Some(response_id) {
            Ok(())
        } else {
            Err(failure(
                "swallowtail.xai.response_correlation_failed",
                "xAI WebSocket event did not match the active response",
            ))
        }
    }
}

fn provider_failure(value: &Value) -> Result<TurnUpdate, RuntimeFailure> {
    let failure = match string(value, "/error/code")? {
        "previous_response_not_found" => ProviderFailure::PreviousResponseNotFound,
        "websocket_connection_limit_reached" => ProviderFailure::ConnectionLimitReached,
        _ => ProviderFailure::Other,
    };
    Ok(TurnUpdate::ProviderFailed(failure))
}

fn completed_output(value: &Value) -> Result<String, RuntimeFailure> {
    let output = value
        .pointer("/response/output")
        .and_then(Value::as_array)
        .ok_or_else(malformed)?;
    if output.len() != 1
        || string(&output[0], "/type")? != "message"
        || string(&output[0], "/role")? != "assistant"
    {
        return Err(malformed());
    }
    let content = output[0]
        .get("content")
        .and_then(Value::as_array)
        .ok_or_else(malformed)?;
    if content.len() != 1 || string(&content[0], "/type")? != "output_text" {
        return Err(malformed());
    }
    Ok(string(&content[0], "/text")?.to_owned())
}

fn string<'a>(value: &'a Value, pointer: &str) -> Result<&'a str, RuntimeFailure> {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .ok_or_else(malformed)
}

fn integer(value: &Value, pointer: &str) -> Result<u64, RuntimeFailure> {
    value
        .pointer(pointer)
        .and_then(Value::as_u64)
        .ok_or_else(malformed)
}

fn malformed() -> RuntimeFailure {
    failure(
        "swallowtail.xai.protocol_malformed",
        "xAI WebSocket returned a malformed protocol frame",
    )
}

fn order_failure() -> RuntimeFailure {
    failure(
        "swallowtail.xai.event_order_invalid",
        "xAI WebSocket event order was invalid",
    )
}
