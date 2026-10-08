use super::{
    MAX_CATALOG_MODELS, Response, bounded_json, protocol_failure, require_success,
    unsupported_semantics,
};
use crate::selection::{ollama_runtime_binding, ollama_runtime_claim};
use serde::Deserialize;
use std::collections::BTreeSet;
use swallowtail_core::{
    AttachedModelObservation, AttachedModelObservationScope, AttachedModelTag, CatalogTimestamp,
    ConfiguredInstanceId, ExecutionHostId, InterfaceVersionBinding, ModelManifestDigest,
};
use swallowtail_runtime::RuntimeFailure;

#[derive(Clone, Debug, Eq, PartialEq)]
/// Immutable identity attached to each model observation in one probe.
pub struct ObservationBinding {
    /// Configured runtime instance being observed.
    pub instance_id: ConfiguredInstanceId,
    /// Execution host that owns the attached runtime binding.
    pub execution_host_id: ExecutionHostId,
    /// Exact observed runtime-version binding.
    pub runtime_version: InterfaceVersionBinding,
    /// Host-supplied catalogue timestamp.
    pub observed_at: CatalogTimestamp,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Model capabilities admitted from the selected Ollama model detail.
pub enum OllamaModelCapability {
    /// Ordinary completion/chat generation is supported.
    Completion,
    /// Native thinking control is supported.
    Thinking,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Native runner variant observed on one mapped catalogue row.
pub(crate) enum OllamaNativeRunner {
    /// Legacy GGUF runner that can start provider-owned compatibility migration.
    Ggml,
    /// Converted llama.cpp GGUF child in a manifest list.
    LlamaCpp,
}

impl OllamaNativeRunner {
    /// Returns the native `runner` request value for this variant.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ggml => "ggml",
            Self::LlamaCpp => "llamacpp",
        }
    }

    fn parse(value: &str) -> Option<Self> {
        match value {
            "ggml" => Some(Self::Ggml),
            "llamacpp" | "llama.cpp" | "llama-cpp" | "llama_cpp" => Some(Self::LlamaCpp),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// One mapped installed or running catalogue row, including optional runner pin.
pub(crate) struct OllamaInventoryRow {
    pub observation: AttachedModelObservation,
    pub runner: Option<OllamaNativeRunner>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct OllamaInventoryParse {
    pub mapped: Vec<OllamaInventoryRow>,
    unmapped: Vec<(AttachedModelTag, ModelManifestDigest)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// Selected-model observation paired with its admitted native capabilities.
pub struct SelectedModelDetail {
    observation: AttachedModelObservation,
    capabilities: BTreeSet<OllamaModelCapability>,
}

impl SelectedModelDetail {
    /// Returns the digest-bound selected-model observation.
    #[must_use]
    pub const fn observation(&self) -> &AttachedModelObservation {
        &self.observation
    }

    /// Iterates admitted capabilities for the selected model.
    pub fn capabilities(&self) -> impl ExactSizeIterator<Item = OllamaModelCapability> + '_ {
        self.capabilities.iter().copied()
    }

    /// Reports whether the selected model advertises one admitted capability.
    #[must_use]
    pub fn supports(&self, capability: OllamaModelCapability) -> bool {
        self.capabilities.contains(&capability)
    }

    pub(crate) fn into_parts(self) -> (AttachedModelObservation, BTreeSet<OllamaModelCapability>) {
        (self.observation, self.capabilities)
    }
}

/// Parses and qualifies an Ollama `/api/version` response.
pub fn parse_version(response: &Response) -> Result<InterfaceVersionBinding, RuntimeFailure> {
    require_success(response, "version")?;
    let envelope: VersionEnvelope = bounded_json(&response.body, "version")?;
    let binding = ollama_runtime_binding(&envelope.version).ok_or_else(|| {
        crate::failure::failure(
            "swallowtail.ollama.version_parse_failed",
            "Ollama runtime version could not be parsed",
        )
    })?;
    if ollama_runtime_claim().permits(binding.version()) {
        Ok(binding)
    } else {
        Err(crate::failure::failure(
            "swallowtail.ollama.version_unsupported",
            "Ollama runtime version is outside the qualified compatibility window",
        ))
    }
}

/// Parses installed or running local model inventory under one binding.
pub fn parse_inventory(
    response: &Response,
    scope: AttachedModelObservationScope,
    binding: &ObservationBinding,
) -> Result<Vec<AttachedModelObservation>, RuntimeFailure> {
    Ok(parse_inventory_rows(response, scope, binding)?
        .mapped
        .into_iter()
        .map(|row| row.observation)
        .collect())
}

/// Parses mapped catalogue rows plus unmapped sibling identities.
pub(crate) fn parse_inventory_rows(
    response: &Response,
    scope: AttachedModelObservationScope,
    binding: &ObservationBinding,
) -> Result<OllamaInventoryParse, RuntimeFailure> {
    if scope == AttachedModelObservationScope::SelectedModelDetail {
        return Err(protocol_failure("inventory scope"));
    }
    require_success(response, "model inventory")?;
    let inventory: InventoryEnvelope = bounded_json(&response.body, "model inventory")?;
    if inventory.models.len() > MAX_CATALOG_MODELS {
        return Err(super::limit_failure());
    }
    let mut mapped = Vec::new();
    let mut unmapped = Vec::new();
    for model in inventory.models {
        match inventory_row(model, scope, binding)? {
            InventoryRowKind::Mapped(row) => mapped.push(row),
            InventoryRowKind::Unmapped { tag, digest } => unmapped.push((tag, digest)),
        }
    }
    Ok(OllamaInventoryParse { mapped, unmapped })
}

/// Binds the preflight tag and digest without treating sibling runner rows as
/// the selected identity.
pub(crate) fn bind_selected_inventory<'a>(
    parse: &'a OllamaInventoryParse,
    model_tag: &AttachedModelTag,
    manifest_digest: &ModelManifestDigest,
) -> Result<&'a OllamaInventoryRow, RuntimeFailure> {
    if parse
        .unmapped
        .iter()
        .any(|(tag, digest)| tag == model_tag && digest == manifest_digest)
    {
        return Err(unsupported_semantics());
    }
    if let Some(row) = parse.mapped.iter().find(|row| {
        row.observation.model_tag() == model_tag
            && row.observation.manifest_digest() == Some(manifest_digest)
    }) {
        return Ok(row);
    }
    if parse
        .mapped
        .iter()
        .any(|row| row.observation.model_tag() == model_tag)
        || parse.unmapped.iter().any(|(tag, _)| tag == model_tag)
    {
        return Err(crate::failure::failure(
            "swallowtail.ollama.selected_identity_drift",
            "The preflight-bound Ollama model tag is present only under a different manifest digest",
        ));
    }
    Err(crate::failure::failure(
        "swallowtail.ollama.model_not_installed",
        "The preflight-bound Ollama model is not installed",
    ))
}

/// Parses detail for the exact selected local model tag and manifest digest.
pub fn parse_model_detail(
    response: &Response,
    binding: &ObservationBinding,
    model_tag: AttachedModelTag,
    manifest_digest: ModelManifestDigest,
) -> Result<SelectedModelDetail, RuntimeFailure> {
    require_success(response, "model detail")?;
    let detail: ShowResponse = bounded_json(&response.body, "model detail")?;
    let capabilities = detail
        .capabilities
        .into_iter()
        .map(|capability| match capability.as_str() {
            "completion" => Ok(OllamaModelCapability::Completion),
            "thinking" => Ok(OllamaModelCapability::Thinking),
            _ => Err(unsupported_semantics()),
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    if !capabilities.contains(&OllamaModelCapability::Completion)
        || detail.details.format != "gguf"
        || detail.details.family.trim().is_empty()
        || detail.remote_model.is_some()
        || detail.remote_host.is_some()
    {
        return Err(unsupported_semantics());
    }
    Ok(SelectedModelDetail {
        observation: observation(
            AttachedModelObservationScope::SelectedModelDetail,
            binding,
            model_tag,
            manifest_digest,
        ),
        capabilities,
    })
}

enum InventoryRowKind {
    Mapped(OllamaInventoryRow),
    Unmapped {
        tag: AttachedModelTag,
        digest: ModelManifestDigest,
    },
}

fn inventory_row(
    model: InventoryModel,
    scope: AttachedModelObservationScope,
    binding: &ObservationBinding,
) -> Result<InventoryRowKind, RuntimeFailure> {
    if model.name != model.model {
        return Err(protocol_failure("model tag"));
    }
    if model.remote_model.is_some() || model.remote_host.is_some() {
        return Err(unsupported_semantics());
    }
    let tag = AttachedModelTag::new(model.model).map_err(|_| protocol_failure("model tag"))?;
    let digest = normalized_digest(&model.digest)?;
    let runner = model
        .details
        .runner
        .as_deref()
        .or(model.runner.as_deref())
        .filter(|value| !value.is_empty());
    if model.details.format != "gguf" {
        return Ok(InventoryRowKind::Unmapped { tag, digest });
    }
    if model.details.family.trim().is_empty() {
        return Err(unsupported_semantics());
    }
    let runner = match runner {
        Some(value) => match OllamaNativeRunner::parse(value) {
            Some(runner) => Some(runner),
            None => return Ok(InventoryRowKind::Unmapped { tag, digest }),
        },
        None => None,
    };
    Ok(InventoryRowKind::Mapped(OllamaInventoryRow {
        observation: observation(scope, binding, tag, digest),
        runner,
    }))
}

fn observation(
    scope: AttachedModelObservationScope,
    binding: &ObservationBinding,
    model_tag: AttachedModelTag,
    digest: ModelManifestDigest,
) -> AttachedModelObservation {
    AttachedModelObservation::new(
        scope,
        binding.instance_id.clone(),
        binding.execution_host_id.clone(),
        binding.runtime_version.clone(),
        binding.observed_at,
        model_tag,
    )
    .with_manifest_digest(digest)
}

fn normalized_digest(value: &str) -> Result<ModelManifestDigest, RuntimeFailure> {
    let value = if value.starts_with("sha256:") {
        value.to_owned()
    } else {
        format!("sha256:{value}")
    };
    ModelManifestDigest::new(value).map_err(|_| protocol_failure("model digest"))
}

#[derive(Deserialize)]
struct VersionEnvelope {
    version: String,
}

#[derive(Deserialize)]
struct InventoryEnvelope {
    models: Vec<InventoryModel>,
}

#[derive(Deserialize)]
struct InventoryModel {
    name: String,
    model: String,
    digest: String,
    details: ModelDetails,
    #[serde(default)]
    runner: Option<String>,
    #[serde(default)]
    remote_model: Option<String>,
    #[serde(default)]
    remote_host: Option<String>,
}

#[derive(Deserialize)]
struct ShowResponse {
    capabilities: Vec<String>,
    details: ModelDetails,
    #[serde(default)]
    remote_model: Option<String>,
    #[serde(default)]
    remote_host: Option<String>,
}

#[derive(Deserialize)]
struct ModelDetails {
    format: String,
    family: String,
    #[serde(default)]
    runner: Option<String>,
}
