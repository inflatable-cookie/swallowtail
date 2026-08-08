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
    if scope == AttachedModelObservationScope::SelectedModelDetail {
        return Err(protocol_failure("inventory scope"));
    }
    require_success(response, "model inventory")?;
    let inventory: InventoryEnvelope = bounded_json(&response.body, "model inventory")?;
    if inventory.models.len() > MAX_CATALOG_MODELS {
        return Err(super::limit_failure());
    }
    inventory
        .models
        .into_iter()
        .map(|model| inventory_observation(model, scope, binding))
        .collect()
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

fn inventory_observation(
    model: InventoryModel,
    scope: AttachedModelObservationScope,
    binding: &ObservationBinding,
) -> Result<AttachedModelObservation, RuntimeFailure> {
    if model.name != model.model
        || model.remote_model.is_some()
        || model.remote_host.is_some()
        || model.details.format != "gguf"
    {
        return Err(unsupported_semantics());
    }
    let tag = AttachedModelTag::new(model.model).map_err(|_| protocol_failure("model tag"))?;
    let digest = normalized_digest(&model.digest)?;
    Ok(observation(scope, binding, tag, digest))
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
}
