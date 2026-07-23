use crate::AttachedModelObservation;
use crate::diagnostic::{ValueRequired, required_text};
use crate::model_catalog::ModelCatalogObservations;
use std::collections::BTreeSet;

/// Stable adapter-owned model identity.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModelId(String);

impl ModelId {
    pub fn new(value: impl Into<String>) -> Result<Self, ValueRequired> {
        required_text("model id", value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Stable adapter-owned provider identity when a harness exposes one separately.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProviderId(String);

impl ProviderId {
    pub fn new(value: impl Into<String>) -> Result<Self, ValueRequired> {
        required_text("provider id", value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Driver-owned name for one reasoning mode accepted by a model route.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ReasoningMode(String);

impl ReasoningMode {
    pub fn new(value: impl Into<String>) -> Result<Self, ValueRequired> {
        required_text("reasoning mode", value).map(Self)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Mutable catalog evidence. It does not select a mode for an operation.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ReasoningMetadata {
    supported_modes: BTreeSet<ReasoningMode>,
    default_mode: Option<ReasoningMode>,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ModelTokenLimits {
    maximum_input_tokens: Option<u64>,
    maximum_output_tokens: Option<u64>,
}

impl ModelTokenLimits {
    #[must_use]
    pub const fn new(
        maximum_input_tokens: Option<u64>,
        maximum_output_tokens: Option<u64>,
    ) -> Self {
        Self {
            maximum_input_tokens,
            maximum_output_tokens,
        }
    }

    #[must_use]
    pub const fn maximum_input_tokens(&self) -> Option<u64> {
        self.maximum_input_tokens
    }

    #[must_use]
    pub const fn maximum_output_tokens(&self) -> Option<u64> {
        self.maximum_output_tokens
    }
}

impl ReasoningMetadata {
    #[must_use]
    pub fn new(
        supported_modes: impl IntoIterator<Item = ReasoningMode>,
        default_mode: Option<ReasoningMode>,
    ) -> Self {
        Self {
            supported_modes: supported_modes.into_iter().collect(),
            default_mode,
        }
    }

    pub fn supported_modes(&self) -> impl ExactSizeIterator<Item = &ReasoningMode> {
        self.supported_modes.iter()
    }

    #[must_use]
    pub const fn default_mode(&self) -> Option<&ReasoningMode> {
        self.default_mode.as_ref()
    }

    #[must_use]
    pub fn supports(&self, mode: &ReasoningMode) -> bool {
        self.supported_modes.contains(mode)
    }
}

/// Mutable catalog presentation kept separate from stable model identity.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ModelMetadata {
    display_name: Option<String>,
    description: Option<String>,
    is_default: bool,
    reasoning: Option<ReasoningMetadata>,
    token_limits: Option<ModelTokenLimits>,
    catalog_observations: Option<ModelCatalogObservations>,
    attached_model_observations: Vec<AttachedModelObservation>,
}

impl ModelMetadata {
    pub fn with_display_name(display_name: impl Into<String>) -> Result<Self, ValueRequired> {
        Ok(Self {
            display_name: Some(required_text("model display name", display_name)?),
            description: None,
            is_default: false,
            reasoning: None,
            token_limits: None,
            catalog_observations: None,
            attached_model_observations: Vec::new(),
        })
    }

    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Result<Self, ValueRequired> {
        self.description = Some(required_text("model description", description)?);
        Ok(self)
    }

    #[must_use]
    pub const fn with_default(mut self, is_default: bool) -> Self {
        self.is_default = is_default;
        self
    }

    #[must_use]
    pub fn with_reasoning(mut self, reasoning: ReasoningMetadata) -> Self {
        self.reasoning = Some(reasoning);
        self
    }

    #[must_use]
    pub const fn with_token_limits(mut self, token_limits: ModelTokenLimits) -> Self {
        self.token_limits = Some(token_limits);
        self
    }

    #[must_use]
    pub fn with_catalog_observations(mut self, observations: ModelCatalogObservations) -> Self {
        self.catalog_observations = Some(observations);
        self
    }

    #[must_use]
    pub fn with_attached_model_observations(
        mut self,
        observations: impl IntoIterator<Item = AttachedModelObservation>,
    ) -> Self {
        self.attached_model_observations = observations.into_iter().collect();
        self
    }

    #[must_use]
    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }

    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    #[must_use]
    pub const fn is_default(&self) -> bool {
        self.is_default
    }

    #[must_use]
    pub const fn reasoning(&self) -> Option<&ReasoningMetadata> {
        self.reasoning.as_ref()
    }

    #[must_use]
    pub const fn token_limits(&self) -> Option<ModelTokenLimits> {
        self.token_limits
    }

    #[must_use]
    pub const fn catalog_observations(&self) -> Option<&ModelCatalogObservations> {
        self.catalog_observations.as_ref()
    }

    pub fn attached_model_observations(
        &self,
    ) -> impl ExactSizeIterator<Item = &AttachedModelObservation> {
        self.attached_model_observations.iter()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelCatalogEntry {
    id: ModelId,
    provider_id: Option<ProviderId>,
    metadata: ModelMetadata,
}

impl ModelCatalogEntry {
    #[must_use]
    pub const fn new(id: ModelId, metadata: ModelMetadata) -> Self {
        Self {
            id,
            provider_id: None,
            metadata,
        }
    }

    #[must_use]
    pub fn with_provider_id(mut self, provider_id: ProviderId) -> Self {
        self.provider_id = Some(provider_id);
        self
    }

    #[must_use]
    pub const fn id(&self) -> &ModelId {
        &self.id
    }

    #[must_use]
    pub const fn provider_id(&self) -> Option<&ProviderId> {
        self.provider_id.as_ref()
    }

    #[must_use]
    pub const fn metadata(&self) -> &ModelMetadata {
        &self.metadata
    }
}

#[cfg(test)]
mod tests;
