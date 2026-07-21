use crate::failure::failure;
use serde::Deserialize;
use serde_json::{Map, Value, json};
use std::collections::BTreeMap;
use swallowtail_core::{ModelCatalogEntry, ModelId, ModelMetadata, ModelTokenLimits, ProviderId};
use swallowtail_runtime::RuntimeFailure;

pub(crate) const OBSERVED_VERSION: &str = "1.14.48";

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
    pub(crate) fn get(path: impl Into<String>) -> Self {
        Self {
            method: Method::Get,
            path: path.into(),
            query: Vec::new(),
            body: None,
        }
    }

    pub(crate) fn post(path: impl Into<String>, body: Option<Value>) -> Self {
        Self {
            method: Method::Post,
            path: path.into(),
            query: Vec::new(),
            body: body.map(|value| serde_json::to_vec(&value).expect("JSON value serializes")),
        }
    }

    pub(crate) fn with_directory(mut self, directory: &str) -> Self {
        self.query
            .push(("directory".to_owned(), directory.to_owned()));
        self
    }
}

#[derive(Debug)]
pub(crate) struct Response {
    pub status: u32,
    pub body: Vec<u8>,
}

#[derive(Deserialize)]
struct Health {
    healthy: bool,
    version: String,
}

pub(crate) fn parse_health(response: &Response) -> Result<(), RuntimeFailure> {
    require_success(response, "health request")?;
    let health: Health = parse_json(&response.body, "health response")?;
    if !health.healthy {
        return Err(failure(
            "swallowtail.opencode.unhealthy",
            "OpenCode reported an unhealthy server",
        ));
    }
    if health.version != OBSERVED_VERSION {
        return Err(failure(
            "swallowtail.opencode.version_mismatch",
            "OpenCode server version is outside the observed protocol fixture",
        ));
    }
    Ok(())
}

#[derive(Deserialize)]
struct ProviderList {
    all: Vec<Provider>,
    #[serde(rename = "default")]
    defaults: BTreeMap<String, String>,
}

#[derive(Deserialize)]
struct Provider {
    id: String,
    models: BTreeMap<String, ProviderModel>,
}

#[derive(Deserialize)]
struct ProviderModel {
    id: String,
    name: Option<String>,
    limit: Option<ModelLimit>,
}

#[derive(Clone, Copy, Deserialize)]
struct ModelLimit {
    input: Option<u64>,
    output: Option<u64>,
}

pub(crate) fn parse_catalog(response: &Response) -> Result<Vec<ModelCatalogEntry>, RuntimeFailure> {
    require_success(response, "provider catalogue request")?;
    let providers: ProviderList = parse_json(&response.body, "provider catalogue response")?;
    let mut entries = Vec::new();
    for provider in providers.all {
        let provider_id = ProviderId::new(provider.id.clone()).map_err(|_| {
            failure(
                "swallowtail.opencode.catalog_invalid",
                "OpenCode returned an invalid provider identity",
            )
        })?;
        for (key, model) in provider.models {
            if key != model.id {
                return Err(failure(
                    "swallowtail.opencode.catalog_invalid",
                    "OpenCode returned inconsistent model identities",
                ));
            }
            let model_id = ModelId::new(model.id.clone()).map_err(|_| {
                failure(
                    "swallowtail.opencode.catalog_invalid",
                    "OpenCode returned an invalid model identity",
                )
            })?;
            let mut metadata = match model.name {
                Some(name) => ModelMetadata::with_display_name(name).map_err(|_| {
                    failure(
                        "swallowtail.opencode.catalog_invalid",
                        "OpenCode returned invalid model metadata",
                    )
                })?,
                None => ModelMetadata::default(),
            };
            metadata =
                metadata.with_default(providers.defaults.get(&provider.id) == Some(&model.id));
            if let Some(limit) = model.limit {
                metadata =
                    metadata.with_token_limits(ModelTokenLimits::new(limit.input, limit.output));
            }
            entries.push(
                ModelCatalogEntry::new(model_id, metadata).with_provider_id(provider_id.clone()),
            );
        }
    }
    Ok(entries)
}

pub(crate) fn session_create(provider_id: &str, model_id: &str, directory: &str) -> Request {
    Request::post(
        "/session",
        Some(json!({
            "title": "Swallowtail session",
            "model": {"id": model_id, "providerID": provider_id},
            "permission": [
                {"permission": "*", "pattern": "*", "action": "deny"},
                {"permission": "read", "pattern": "*", "action": "allow"},
                {"permission": "glob", "pattern": "*", "action": "allow"},
                {"permission": "grep", "pattern": "*", "action": "allow"}
            ]
        })),
    )
    .with_directory(directory)
}

pub(crate) fn parse_session(response: &Response) -> Result<String, RuntimeFailure> {
    require_success(response, "session create request")?;
    #[derive(Deserialize)]
    struct Session {
        id: String,
        version: String,
    }
    let session: Session = parse_json(&response.body, "session create response")?;
    if session.version != OBSERVED_VERSION || session.id.trim().is_empty() {
        return Err(failure(
            "swallowtail.opencode.session_invalid",
            "OpenCode returned an invalid session binding",
        ));
    }
    Ok(session.id)
}

pub(crate) fn prompt(
    session_id: &str,
    provider_id: &str,
    model_id: &str,
    directory: &str,
    content: &str,
) -> Request {
    Request::post(
        format!("/session/{session_id}/prompt_async"),
        Some(json!({
            "model": {"providerID": provider_id, "modelID": model_id},
            "parts": [{"type": "text", "text": content}]
        })),
    )
    .with_directory(directory)
}

pub(crate) fn abort(session_id: &str, directory: &str) -> Request {
    Request::post(format!("/session/{session_id}/abort"), None).with_directory(directory)
}

pub(crate) fn require_no_content(response: &Response) -> Result<(), RuntimeFailure> {
    if response.status == 204 {
        Ok(())
    } else {
        Err(http_failure("prompt request"))
    }
}

pub(crate) fn require_abort_success(response: &Response) -> Result<(), RuntimeFailure> {
    require_success(response, "abort request")?;
    match serde_json::from_slice::<bool>(&response.body) {
        Ok(true) => Ok(()),
        _ => Err(failure(
            "swallowtail.opencode.abort_failed",
            "OpenCode did not acknowledge session abort",
        )),
    }
}

fn require_success(response: &Response, operation: &'static str) -> Result<(), RuntimeFailure> {
    if (200..300).contains(&response.status) {
        Ok(())
    } else {
        Err(http_failure(operation))
    }
}

fn http_failure(operation: &'static str) -> RuntimeFailure {
    RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
        "swallowtail.opencode.http_failed",
        format!("OpenCode {operation} failed"),
    ))
}

fn parse_json<T: for<'de> Deserialize<'de>>(
    bytes: &[u8],
    operation: &'static str,
) -> Result<T, RuntimeFailure> {
    serde_json::from_slice(bytes).map_err(|_| {
        RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
            "swallowtail.opencode.protocol_invalid",
            format!("OpenCode {operation} was invalid"),
        ))
    })
}

include!("protocol/events.rs");
include!("protocol/tests.rs");
