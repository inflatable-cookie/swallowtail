use crate::failure::failure;
use serde::Deserialize;
use std::collections::BTreeMap;
use swallowtail_core::{
    CatalogObservation, IntegrationFamilyId, ModelCatalogEntry, ModelCatalogObservations, ModelId,
    ModelMetadata, ModelModality, ModelTokenLimits, ProviderCatalogValue, ProviderId,
    ReasoningMetadata, ReasoningMode,
};
use swallowtail_protocol_openai_chat::{
    ChatMessage, ChatRequest, CodecLimits, Payload, ProtocolError, ProtocolErrorKind, SseRecord,
    decode_payload, encode_request,
};
use swallowtail_runtime::{OperationContent, RuntimeFailure, TokenUsage};

pub(crate) const PROVIDER_ID: &str = "moonshot";
pub(crate) const MODEL_ID: &str = "kimi-k3";
pub(crate) const AUDIENCE: &str = "api.moonshot.ai";
pub(crate) const MAXIMUM_OUTPUT_TOKENS: u64 = 1_048_576;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Request {
    pub method: Method,
    pub path: String,
    pub body: Option<Vec<u8>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Method {
    Get,
    Post,
}

impl Request {
    pub(crate) fn models() -> Self {
        Self {
            method: Method::Get,
            path: "/v1/models".to_owned(),
            body: None,
        }
    }

    pub(crate) fn chat(
        model: &str,
        content: &OperationContent,
        reasoning: &ReasoningMode,
        maximum_output_tokens: u64,
    ) -> Result<Self, RuntimeFailure> {
        if maximum_output_tokens == 0 || maximum_output_tokens > MAXIMUM_OUTPUT_TOKENS {
            return Err(failure(
                "swallowtail.kimi_platform.output_limit_invalid",
                "Kimi Platform maximum output tokens exceeded the K3 route bound",
            ));
        }
        let mut chat = ChatRequest::new(
            model,
            vec![ChatMessage::new("user", content.as_str())],
            true,
            true,
        );
        chat.insert_extension("reasoning_effort", serde_json::json!(reasoning.as_str()))
            .expect("Kimi reasoning extension is structurally valid");
        chat.insert_extension(
            "max_completion_tokens",
            serde_json::json!(maximum_output_tokens),
        )
        .expect("Kimi output extension is structurally valid");
        let body = encode_request(&chat, CodecLimits::default()).map_err(|_| {
            failure(
                "swallowtail.kimi_platform.request_invalid",
                "Kimi Platform request exceeded the compatible codec bounds",
            )
        })?;
        Ok(Self {
            method: Method::Post,
            path: "/v1/chat/completions".to_owned(),
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
#[serde(deny_unknown_fields)]
struct ModelList {
    object: String,
    data: Vec<Model>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Model {
    id: String,
    object: String,
    created: u64,
    owned_by: String,
    context_length: u64,
    supports_image_in: bool,
    supports_video_in: bool,
    supports_reasoning: bool,
}

pub(crate) fn parse_models(response: &Response) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
    require_success(response, "model catalogue")?;
    let list: ModelList = parse_json(&response.body, "model catalogue")?;
    if list.object != "list" || list.data.len() > 128 {
        return Err(protocol_failure("bounded model catalogue"));
    }
    list.data.into_iter().map(model_entry).collect()
}

fn model_entry(model: Model) -> Result<ModelCatalogEntry, RuntimeFailure> {
    if model.object != "model" || model.created == 0 {
        return Err(protocol_failure("model object"));
    }
    let id = ModelId::new(model.id).map_err(|_| protocol_failure("model identity"))?;
    let provider =
        ProviderId::new(model.owned_by).map_err(|_| protocol_failure("provider identity"))?;
    let source = IntegrationFamilyId::new("kimi-platform").expect("static family is valid");
    let mut modalities = Vec::new();
    if model.supports_image_in {
        modalities.push(CatalogObservation::Known(ModelModality::Image));
    }
    if model.supports_video_in {
        modalities.push(CatalogObservation::ProviderDefined(
            ProviderCatalogValue::new(source.clone(), "video")
                .expect("static provider value is valid"),
        ));
    }
    let observations = ModelCatalogObservations::new(source)
        .with_input_modalities(modalities)
        .with_response_streaming_supported(true);
    let mut metadata = ModelMetadata::default()
        .with_token_limits(ModelTokenLimits::new(Some(model.context_length), None))
        .with_catalog_observations(observations);
    if model.supports_reasoning {
        let low = mode("low");
        let high = mode("high");
        let maximum = mode("max");
        metadata = metadata.with_reasoning(ReasoningMetadata::new(
            [low, high, maximum.clone()],
            Some(maximum),
        ));
    }
    Ok(ModelCatalogEntry::new(id, metadata).with_provider_id(provider))
}

fn mode(value: &str) -> ReasoningMode {
    ReasoningMode::new(value).expect("static reasoning mode is valid")
}

pub(crate) fn require_success(response: &Response, operation: &str) -> Result<(), RuntimeFailure> {
    if (200..300).contains(&response.status) {
        return Ok(());
    }
    let kind = decode_payload(&response.body, CodecLimits::default())
        .ok()
        .and_then(|payload| match payload {
            Payload::Error(error) => Some(classify_error(&error.error.kind)),
            Payload::Chunk(_) => None,
        });
    Err(provider_failure(
        kind.unwrap_or(ProviderErrorKind::Other),
        operation,
    ))
}

fn parse_json<T: for<'de> Deserialize<'de>>(
    bytes: &[u8],
    subject: &str,
) -> Result<T, RuntimeFailure> {
    serde_json::from_slice(bytes).map_err(|_| protocol_failure(subject))
}

pub(crate) fn protocol_failure(subject: &str) -> RuntimeFailure {
    failure(
        "swallowtail.kimi_platform.protocol_invalid",
        format!("Kimi Platform {subject} was invalid"),
    )
}

include!("protocol/events.rs");

#[cfg(test)]
mod tests;
