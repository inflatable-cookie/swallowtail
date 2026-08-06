use super::ResponseRef;
use crate::failure::AlibabaProtocolFailure;
use crate::selection::EXACT_MODEL_ID;
use serde_json::Value;
use swallowtail_runtime::{OperationContent, TokenUsage};

include!("sse/decoder.rs");

#[derive(Clone, Debug, Eq, PartialEq)]
/// Validated semantic event from the qualified Responses SSE subset.
pub enum ProviderEvent {
    /// The provider created one exact response.
    Created(ResponseRef),
    /// An assistant output item started.
    AssistantStarted(String),
    /// A known non-content lifecycle event advanced the response.
    Progress(String),
    /// An ordered assistant text delta.
    TextDelta {
        /// Provider output-item identity.
        item: String,
        /// Newly emitted text content.
        content: OperationContent,
    },
    /// The provider closed the assistant text content.
    TextDone {
        /// Provider output-item identity.
        item: String,
        /// Complete assistant text.
        content: OperationContent,
    },
    /// The response reached terminal success with exact identity agreement.
    Completed {
        /// Provider response identity.
        response: ResponseRef,
        /// Provider output-item identity.
        item: String,
        /// Complete assistant output.
        output: OperationContent,
        /// Final provider token usage.
        usage: TokenUsage,
    },
    /// A forward-compatible event outside the semantic projection.
    Unknown(String),
}

#[derive(Default)]
/// Stateful validator for one ordered Model Studio response stream.
pub struct ResponseStream {
    response: Option<ResponseRef>,
    last_sequence: Option<u64>,
    output: String,
    item: Option<String>,
    output_done: bool,
    terminal: bool,
}

impl ResponseStream {
    /// Applies one SSE frame and returns its bounded semantic projection.
    pub fn apply(&mut self, frame: &SseFrame) -> Result<ProviderEvent, AlibabaProtocolFailure> {
        if self.terminal {
            return Err(AlibabaProtocolFailure::invalid(
                "event after terminal response",
            ));
        }
        let value: Value = serde_json::from_slice(frame.data())
            .map_err(|_| AlibabaProtocolFailure::invalid("Responses SSE event"))?;
        if value.get("type").and_then(Value::as_str) != Some(frame.name()) {
            return Err(AlibabaProtocolFailure::invalid("Responses SSE event type"));
        }
        let sequence = value
            .get("sequence_number")
            .and_then(Value::as_u64)
            .ok_or_else(|| AlibabaProtocolFailure::invalid("provider sequence"))?;
        let expected = self.last_sequence.map_or(0, |last| last.saturating_add(1));
        if sequence != expected {
            return Err(AlibabaProtocolFailure::invalid("provider sequence"));
        }

        let event = match frame.name() {
            "response.created" => self.created(&value)?,
            "response.in_progress"
            | "response.content_part.added"
            | "response.content_part.done"
            | "response.output_item.done" => ProviderEvent::Progress(frame.name().to_owned()),
            "response.output_item.added" => self.assistant_started(&value)?,
            "response.output_text.delta" => {
                let delta = text(&value, "/delta")?;
                let item = self.require_item(text(&value, "/item_id")?)?.to_owned();
                self.output.push_str(delta);
                ProviderEvent::TextDelta {
                    item,
                    content: content(delta)?,
                }
            }
            "response.output_text.done" => {
                let done = text(&value, "/text")?;
                let item = self.require_item(text(&value, "/item_id")?)?.to_owned();
                if done != self.output {
                    return Err(AlibabaProtocolFailure::invalid(
                        "completed output agreement",
                    ));
                }
                self.output_done = true;
                ProviderEvent::TextDone {
                    item,
                    content: content(done)?,
                }
            }
            "response.completed" => self.completed(&value)?,
            name if name.starts_with("response.reasoning_") => {
                return Err(AlibabaProtocolFailure::invalid("disabled reasoning event"));
            }
            name if name.contains("search")
                || name.contains("code_interpreter")
                || name.contains("mcp") =>
            {
                return Err(AlibabaProtocolFailure::invalid("disabled tool event"));
            }
            "error" | "response.failed" => return Err(AlibabaProtocolFailure::provider()),
            name => ProviderEvent::Unknown(name.to_owned()),
        };
        self.last_sequence = Some(sequence);
        Ok(event)
    }

    fn assistant_started(
        &mut self,
        value: &Value,
    ) -> Result<ProviderEvent, AlibabaProtocolFailure> {
        if self.item.is_some()
            || text(value, "/item/type")? != "message"
            || text(value, "/item/role")? != "assistant"
            || text(value, "/item/status")? != "in_progress"
        {
            return Err(AlibabaProtocolFailure::invalid("assistant output item"));
        }
        let item = text(value, "/item/id")?.to_owned();
        self.item = Some(item.clone());
        Ok(ProviderEvent::AssistantStarted(item))
    }

    fn require_item<'a>(&'a self, item: &str) -> Result<&'a str, AlibabaProtocolFailure> {
        match self.item.as_deref() {
            Some(expected) if expected == item => Ok(expected),
            _ => Err(AlibabaProtocolFailure::invalid(
                "assistant output item correlation",
            )),
        }
    }

    fn created(&mut self, value: &Value) -> Result<ProviderEvent, AlibabaProtocolFailure> {
        if self.response.is_some()
            || text(value, "/response/status")? != "queued"
            || text(value, "/response/model")? != EXACT_MODEL_ID
        {
            return Err(AlibabaProtocolFailure::invalid("response creation"));
        }
        let response = ResponseRef::new(text(value, "/response/id")?)?;
        self.response = Some(response.clone());
        Ok(ProviderEvent::Created(response))
    }

    fn completed(&mut self, value: &Value) -> Result<ProviderEvent, AlibabaProtocolFailure> {
        if !self.output_done || text(value, "/response/status")? != "completed" {
            return Err(AlibabaProtocolFailure::invalid("response completion"));
        }
        let response_id = text(value, "/response/id")?;
        let item = text(value, "/response/output/0/id")?;
        if self.response.as_ref().map(ResponseRef::as_str) != Some(response_id)
            || self.require_item(item).is_err()
            || text(value, "/response/model")? != EXACT_MODEL_ID
            || text(value, "/response/output/0/type")? != "message"
            || text(value, "/response/output/0/content/0/type")? != "output_text"
            || text(value, "/response/output/0/content/0/text")? != self.output
            || value
                .pointer("/response/tools")
                .and_then(Value::as_array)
                .is_some_and(|v| !v.is_empty())
            || value
                .pointer("/response/usage/input_tokens_details/cached_tokens")
                .and_then(Value::as_u64)
                .unwrap_or(0)
                != 0
            || value
                .pointer("/response/usage/output_tokens_details/reasoning_tokens")
                .and_then(Value::as_u64)
                .unwrap_or(0)
                != 0
        {
            return Err(AlibabaProtocolFailure::invalid(
                "response completion agreement",
            ));
        }
        let response = ResponseRef::new(response_id)?;
        let usage = TokenUsage::new(
            value
                .pointer("/response/usage/input_tokens")
                .and_then(Value::as_u64),
            value
                .pointer("/response/usage/output_tokens")
                .and_then(Value::as_u64),
        );
        if usage.input_tokens().is_none() || usage.output_tokens().is_none() {
            return Err(AlibabaProtocolFailure::invalid("response usage"));
        }
        self.terminal = true;
        Ok(ProviderEvent::Completed {
            response,
            item: item.to_owned(),
            output: content(&self.output)?,
            usage,
        })
    }
}

fn content(value: &str) -> Result<OperationContent, AlibabaProtocolFailure> {
    OperationContent::new(value.to_owned())
        .map_err(|_| AlibabaProtocolFailure::invalid("response text"))
}

fn text<'a>(value: &'a Value, pointer: &str) -> Result<&'a str, AlibabaProtocolFailure> {
    value
        .pointer(pointer)
        .and_then(Value::as_str)
        .ok_or_else(|| AlibabaProtocolFailure::invalid("Responses event field"))
}
