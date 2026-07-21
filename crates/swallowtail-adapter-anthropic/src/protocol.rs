use crate::failure::failure;
use serde::Deserialize;
use serde_json::json;
use std::collections::BTreeMap;
use swallowtail_core::{ModelCatalogEntry, ModelId, ModelMetadata, ModelTokenLimits, ProviderId};
use swallowtail_runtime::{OperationContent, RuntimeFailure};

pub(crate) const API_VERSION: &str = "2023-06-01";
pub(crate) const PROVIDER_ID: &str = "anthropic";

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Request {
    pub method: Method,
    pub path: String,
    pub query: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Method {
    Get,
    Post,
}

impl Request {
    pub(crate) fn models(after: Option<&str>) -> Self {
        let mut query = vec![("limit".to_owned(), "2".to_owned())];
        if let Some(after) = after {
            query.push(("after_id".to_owned(), after.to_owned()));
        }
        Self {
            method: Method::Get,
            path: "/v1/models".to_owned(),
            query,
            body: None,
        }
    }

    pub(crate) fn message(
        model: &str,
        content: &OperationContent,
        maximum_output_tokens: u64,
    ) -> Result<Self, RuntimeFailure> {
        let maximum = u32::try_from(maximum_output_tokens).map_err(|_| {
            failure(
                "swallowtail.anthropic.output_limit_invalid",
                "Anthropic maximum output tokens exceeded the supported request range",
            )
        })?;
        let body = serde_json::to_vec(&json!({
            "model": model,
            "max_tokens": maximum,
            "messages": [{"role": "user", "content": content.as_str()}],
            "stream": true
        }))
        .expect("message request JSON serializes");
        Ok(Self {
            method: Method::Post,
            path: "/v1/messages".to_owned(),
            query: Vec::new(),
            body: Some(body),
        })
    }
}

#[derive(Debug)]
pub(crate) struct Response {
    pub status: u32,
    pub _headers: BTreeMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Deserialize)]
struct ModelPage {
    data: Vec<Model>,
    has_more: bool,
    last_id: String,
}

#[derive(Deserialize)]
struct Model {
    id: String,
    display_name: Option<String>,
    max_input_tokens: Option<u64>,
    max_tokens: Option<u64>,
}

pub(crate) fn parse_model_page(
    response: &Response,
) -> Result<(Vec<ModelCatalogEntry>, Option<String>), RuntimeFailure> {
    require_success(response, "model catalogue")?;
    let page: ModelPage = parse_json(&response.body, "model catalogue")?;
    let provider = ProviderId::new(PROVIDER_ID).expect("static provider id is valid");
    let mut entries = Vec::with_capacity(page.data.len());
    for model in page.data {
        let id = ModelId::new(model.id).map_err(|_| protocol_failure("model identity"))?;
        let mut metadata = match model.display_name {
            Some(name) => ModelMetadata::with_display_name(name)
                .map_err(|_| protocol_failure("model display metadata"))?,
            None => ModelMetadata::default(),
        };
        if model.max_input_tokens.is_some() || model.max_tokens.is_some() {
            metadata = metadata.with_token_limits(ModelTokenLimits::new(
                model.max_input_tokens,
                model.max_tokens,
            ));
        }
        entries.push(ModelCatalogEntry::new(id, metadata).with_provider_id(provider.clone()));
    }
    if page.has_more && page.last_id.trim().is_empty() {
        return Err(protocol_failure("catalogue cursor"));
    }
    Ok((entries, page.has_more.then_some(page.last_id)))
}

pub(crate) fn require_success(response: &Response, operation: &str) -> Result<(), RuntimeFailure> {
    if (200..300).contains(&response.status) {
        return Ok(());
    }
    let value: Option<serde_json::Value> = serde_json::from_slice(&response.body).ok();
    let kind = value
        .as_ref()
        .and_then(|value| value["error"]["type"].as_str());
    Err(provider_failure(classify_error(kind), operation))
}

fn parse_json<T: for<'de> Deserialize<'de>>(
    bytes: &[u8],
    subject: &str,
) -> Result<T, RuntimeFailure> {
    serde_json::from_slice(bytes).map_err(|_| protocol_failure(subject))
}

pub(crate) fn protocol_failure(subject: &str) -> RuntimeFailure {
    failure(
        "swallowtail.anthropic.protocol_invalid",
        format!("Anthropic {subject} was invalid"),
    )
}

include!("protocol/events.rs");

#[cfg(test)]
include!("protocol/tests.rs");
