use crate::failure::failure;
use serde_json::{Value, json};
use swallowtail_core::ReasoningMode;
use swallowtail_runtime::{
    OperationContent, RuntimeFailure, SchemaDocument, StructuredOutputDescriptor,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Method {
    Delete,
    Get,
    Post,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Request {
    pub method: Method,
    pub path: String,
    pub query: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
}

impl Request {
    pub(crate) fn models() -> Self {
        Self {
            method: Method::Get,
            path: "/v1/models".to_owned(),
            query: Vec::new(),
            body: None,
        }
    }

    pub(crate) fn create(
        model: &str,
        content: &OperationContent,
        maximum_output_tokens: u64,
        reasoning: Option<&ReasoningMode>,
        structured_output: Option<&StructuredOutputDescriptor>,
    ) -> Result<Self, RuntimeFailure> {
        let maximum = u32::try_from(maximum_output_tokens).map_err(|_| {
            failure(
                "swallowtail.openai.output_limit_invalid",
                "OpenAI maximum output tokens exceeded the supported request range",
            )
        })?;
        let mut request = json!({
            "model": model,
            "input": [{
                "role": "user",
                "content": [{"type": "input_text", "text": content.as_str()}]
            }],
            "background": true,
            "stream": true,
            "store": false,
            "max_output_tokens": maximum
        });
        if let Some(reasoning) = reasoning {
            request["reasoning"] = json!({"effort": reasoning.as_str()});
        }
        if let Some(output) = structured_output {
            let schema = match output.document() {
                SchemaDocument::Inline(bytes) => {
                    serde_json::from_slice::<Value>(bytes).map_err(|_| invalid_schema())?
                }
                SchemaDocument::Reference(_) => return Err(invalid_schema()),
            };
            request["text"] = json!({
                "format": {
                    "type": "json_schema",
                    "name": "swallowtail_response",
                    "strict": true,
                    "schema": schema
                }
            });
        }
        let body = serde_json::to_vec(&request).expect("create request JSON serializes");
        Ok(Self {
            method: Method::Post,
            path: "/v1/responses".to_owned(),
            query: Vec::new(),
            body: Some(body),
        })
    }

    pub(crate) fn retrieve(response_id: &str) -> Result<Self, RuntimeFailure> {
        Ok(Self {
            method: Method::Get,
            path: response_path(response_id)?,
            query: Vec::new(),
            body: None,
        })
    }

    pub(crate) fn reattach(response_id: &str, starting_after: u64) -> Result<Self, RuntimeFailure> {
        Ok(Self {
            method: Method::Get,
            path: response_path(response_id)?,
            query: vec![
                ("stream".to_owned(), "true".to_owned()),
                ("starting_after".to_owned(), starting_after.to_string()),
            ],
            body: None,
        })
    }

    pub(crate) fn cancel(response_id: &str) -> Result<Self, RuntimeFailure> {
        Ok(Self {
            method: Method::Post,
            path: format!("{}/cancel", response_path(response_id)?),
            query: Vec::new(),
            body: None,
        })
    }

    pub(crate) fn delete(response_id: &str) -> Result<Self, RuntimeFailure> {
        Ok(Self {
            method: Method::Delete,
            path: response_path(response_id)?,
            query: Vec::new(),
            body: None,
        })
    }

    pub(crate) fn expects_stream(&self) -> bool {
        self.path == "/v1/responses"
            || self
                .query
                .iter()
                .any(|(key, value)| key == "stream" && value == "true")
    }
}

fn invalid_schema() -> RuntimeFailure {
    failure(
        "swallowtail.openai.schema_invalid",
        "OpenAI structured-output schema could not be encoded",
    )
}

fn response_path(response_id: &str) -> Result<String, RuntimeFailure> {
    if response_id.starts_with("resp_")
        && response_id.len() <= 256
        && response_id
            .bytes()
            .all(|value| value.is_ascii_alphanumeric() || value == b'_' || value == b'-')
    {
        Ok(format!("/v1/responses/{response_id}"))
    } else {
        Err(failure(
            "swallowtail.openai.response_id_invalid",
            "OpenAI response identity was invalid",
        ))
    }
}
