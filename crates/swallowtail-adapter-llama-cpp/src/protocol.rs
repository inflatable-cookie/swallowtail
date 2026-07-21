use crate::failure::failure;
use serde::Deserialize;
use serde_json::json;
use swallowtail_core::{ModelCatalogEntry, ModelId, ModelMetadata};
use swallowtail_runtime::{OperationContent, RuntimeFailure};

#[derive(Clone, Copy)]
pub(crate) struct ObservedVersion {
    pub build: &'static str,
    pub commit: &'static str,
}

pub(crate) const ATTACHED_VERSION: ObservedVersion = ObservedVersion {
    build: "9910",
    commit: "f5525f7e7",
};

pub(crate) const OWNED_VERSION: ObservedVersion = ObservedVersion {
    build: "10069",
    commit: "178a6c449",
};

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
    pub(crate) fn health() -> Self {
        Self::get("/health")
    }

    pub(crate) fn properties() -> Self {
        Self::get("/props")
    }

    pub(crate) fn models() -> Self {
        Self::get("/v1/models")
    }

    pub(crate) fn chat(
        model: &str,
        content: &OperationContent,
        maximum_output_tokens: u64,
    ) -> Result<Self, RuntimeFailure> {
        let maximum = u32::try_from(maximum_output_tokens).map_err(|_| {
            failure(
                "swallowtail.llama_cpp.output_limit_invalid",
                "llama.cpp maximum output tokens exceeded the supported request range",
            )
        })?;
        let body = serde_json::to_vec(&json!({
            "model": model,
            "max_tokens": maximum,
            "messages": [{"role": "user", "content": content.as_str()}],
            "stream": true,
            "stream_options": {"include_usage": true},
            "temperature": 0
        }))
        .expect("chat request JSON serializes");
        Ok(Self {
            method: Method::Post,
            path: "/v1/chat/completions".to_owned(),
            body: Some(body),
        })
    }

    fn get(path: &str) -> Self {
        Self {
            method: Method::Get,
            path: path.to_owned(),
            body: None,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Response {
    pub status: u32,
    pub body: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Readiness {
    Loading,
    Ready,
}

#[derive(Deserialize)]
struct Health {
    status: String,
}

pub(crate) fn parse_health(response: &Response) -> Result<Readiness, RuntimeFailure> {
    if response.status == 503 {
        let error: ErrorEnvelope = parse_json(&response.body, "health response")?;
        if error.error.kind == "unavailable_error" {
            return Ok(Readiness::Loading);
        }
        return Err(protocol_failure("health response"));
    }
    require_success(response, "health request")?;
    let health: Health = parse_json(&response.body, "health response")?;
    if health.status == "ok" {
        Ok(Readiness::Ready)
    } else {
        Err(protocol_failure("health status"))
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
pub(crate) struct ChatTemplateCapabilities {
    pub supports_string_content: bool,
    pub supports_typed_content: bool,
    pub supports_tools: bool,
    pub supports_tool_calls: bool,
    pub supports_parallel_tool_calls: bool,
    pub supports_system_role: bool,
    pub supports_preserve_reasoning: bool,
    pub supports_object_arguments: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DeploymentEvidence {
    pub model_alias: String,
    pub chat_template: String,
    pub chat_template_capabilities: ChatTemplateCapabilities,
}

#[derive(Deserialize)]
struct Properties {
    model_alias: String,
    build_info: String,
    chat_template: String,
    chat_template_caps: ChatTemplateCapabilities,
    modalities: Modalities,
}

#[derive(Deserialize)]
struct Modalities {
    vision: bool,
    audio: bool,
    video: bool,
}

pub(crate) fn parse_properties(
    response: &Response,
    version: ObservedVersion,
) -> Result<DeploymentEvidence, RuntimeFailure> {
    require_success(response, "properties request")?;
    let properties: Properties = parse_json(&response.body, "properties response")?;
    if !properties.build_info.contains(version.build)
        || !properties.build_info.contains(version.commit)
    {
        return Err(failure(
            "swallowtail.llama_cpp.version_mismatch",
            "llama.cpp server version is outside the observed protocol fixture",
        ));
    }
    if properties.model_alias.trim().is_empty() || properties.chat_template.trim().is_empty() {
        return Err(protocol_failure("deployment identity"));
    }
    if properties.modalities.vision || properties.modalities.audio || properties.modalities.video {
        return Err(failure(
            "swallowtail.llama_cpp.fixture_mismatch",
            "llama.cpp deployment exposes modalities outside the observed text fixture",
        ));
    }
    Ok(DeploymentEvidence {
        model_alias: properties.model_alias,
        chat_template: properties.chat_template,
        chat_template_capabilities: properties.chat_template_caps,
    })
}

#[derive(Deserialize)]
struct ModelList {
    object: String,
    data: Vec<Model>,
}

#[derive(Deserialize)]
struct Model {
    id: String,
    object: String,
}

pub(crate) fn parse_models(response: &Response) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
    require_success(response, "model catalogue request")?;
    let list: ModelList = parse_json(&response.body, "model catalogue response")?;
    if list.object != "list" || list.data.len() != 1 || list.data[0].object != "model" {
        return Err(protocol_failure("single-model catalogue"));
    }
    let model = list.data.into_iter().next().expect("one model exists");
    let id = ModelId::new(model.id).map_err(|_| protocol_failure("model identity"))?;
    Ok(vec![ModelCatalogEntry::new(id, ModelMetadata::default())])
}

pub(crate) fn require_success(response: &Response, operation: &str) -> Result<(), RuntimeFailure> {
    if (200..300).contains(&response.status) {
        Ok(())
    } else {
        let kind = serde_json::from_slice::<ErrorEnvelope>(&response.body)
            .ok()
            .map(|envelope| envelope.error.kind);
        Err(provider_failure(kind.as_deref(), operation))
    }
}

#[derive(Deserialize)]
struct ErrorEnvelope {
    error: ProviderError,
}

#[derive(Deserialize)]
struct ProviderError {
    #[serde(rename = "type")]
    kind: String,
}

pub(crate) fn provider_failure(kind: Option<&str>, operation: &str) -> RuntimeFailure {
    let (code, label) = match kind {
        Some("unavailable_error") => (
            "swallowtail.llama_cpp.deployment_unavailable",
            "deployment was unavailable",
        ),
        Some("invalid_request_error") => (
            "swallowtail.llama_cpp.invalid_request",
            "rejected the request",
        ),
        Some("not_found_error") => (
            "swallowtail.llama_cpp.model_not_found",
            "did not expose the selected model",
        ),
        _ => ("swallowtail.llama_cpp.provider_failed", "request failed"),
    };
    failure(code, format!("llama.cpp {operation} {label}"))
}

fn parse_json<T: for<'de> Deserialize<'de>>(
    bytes: &[u8],
    subject: &str,
) -> Result<T, RuntimeFailure> {
    serde_json::from_slice(bytes).map_err(|_| protocol_failure(subject))
}

pub(crate) fn protocol_failure(subject: &str) -> RuntimeFailure {
    failure(
        "swallowtail.llama_cpp.protocol_invalid",
        format!("llama.cpp {subject} was invalid"),
    )
}

include!("protocol/events.rs");

#[cfg(test)]
include!("protocol/tests.rs");
