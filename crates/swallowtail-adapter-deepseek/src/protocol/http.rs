use super::response::{ProviderFailureKind, classify_failure};
use super::{ProtocolFailure, ProtocolFailureKind};
use crate::selection::DEEPSEEK_PROVIDER_ID;
use serde::Deserialize;
use std::collections::BTreeMap;
use swallowtail_core::{ModelCatalogEntry, ModelId, ModelMetadata, ProviderId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct HttpRequest {
    pub(crate) method: Method,
    pub(crate) path: &'static str,
    pub(crate) body: Option<Vec<u8>>,
    pub(crate) stream: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Method {
    Get,
    Post,
}

impl HttpRequest {
    pub(crate) const fn models() -> Self {
        Self {
            method: Method::Get,
            path: "/models",
            body: None,
            stream: false,
        }
    }

    pub(crate) fn completion(body: Vec<u8>, stream: bool) -> Self {
        Self {
            method: Method::Post,
            path: "/chat/completions",
            body: Some(body),
            stream,
        }
    }
}

#[derive(Debug)]
pub(crate) struct HttpResponse {
    pub(crate) status: u32,
    pub(crate) headers: BTreeMap<String, String>,
    pub(crate) body: Vec<u8>,
}

pub(crate) fn require_success(response: &HttpResponse) -> Result<(), ProviderFailureKind> {
    if (200..300).contains(&response.status) {
        Ok(())
    } else {
        Err(classify_failure(response.status as u16).unwrap_or(ProviderFailureKind::Provider))
    }
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
    owned_by: String,
}

pub(crate) fn parse_models(bytes: &[u8]) -> Result<Vec<ModelCatalogEntry>, ProtocolFailure> {
    let list: ModelList = serde_json::from_slice(bytes)
        .map_err(|_| ProtocolFailure::new(ProtocolFailureKind::InvalidStructure))?;
    if list.object != "list" || list.data.len() > 128 {
        return Err(ProtocolFailure::new(ProtocolFailureKind::BoundExceeded));
    }
    list.data
        .into_iter()
        .map(|model| {
            if model.object != "model" || model.owned_by != DEEPSEEK_PROVIDER_ID {
                return Err(ProtocolFailure::new(ProtocolFailureKind::InvalidStructure));
            }
            let id = ModelId::new(model.id)
                .map_err(|_| ProtocolFailure::new(ProtocolFailureKind::InvalidStructure))?;
            let provider = ProviderId::new(model.owned_by)
                .map_err(|_| ProtocolFailure::new(ProtocolFailureKind::InvalidStructure))?;
            Ok(ModelCatalogEntry::new(id, ModelMetadata::default()).with_provider_id(provider))
        })
        .collect()
}
