use super::malformed;
use serde_json::{Value, json};
use std::num::NonZeroU64;
use swallowtail_core::ReasoningMode;
use swallowtail_runtime::RuntimeFailure;

pub(crate) struct Request;

impl Request {
    pub(crate) fn turn(
        model: &str,
        input: &str,
        continuation: Option<&str>,
        reasoning: Option<&ReasoningMode>,
        maximum_output_tokens: Option<NonZeroU64>,
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
        if let Some(reasoning) = reasoning {
            value["reasoning"] = json!({"effort": reasoning.as_str()});
        }
        if let Some(maximum_output_tokens) = maximum_output_tokens {
            value["max_output_tokens"] = json!(maximum_output_tokens.get());
        }
        serde_json::to_string(&value).map_err(|_| malformed())
    }
}
