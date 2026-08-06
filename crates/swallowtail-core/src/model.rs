#![deny(missing_docs)]

use crate::AttachedModelObservation;
use crate::diagnostic::{ValueRequired, required_text};
use crate::model_catalog::ModelCatalogObservations;
use std::collections::BTreeSet;

/// Stable adapter-owned model identity.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ModelId(String);

impl ModelId {
    /// Creates a model identity after rejecting blank text.
    pub fn new(value: impl Into<String>) -> Result<Self, ValueRequired> {
        required_text("model id", value).map(Self)
    }

    #[must_use]
    /// Returns the adapter-owned model identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Stable adapter-owned provider identity when a harness exposes one separately.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProviderId(String);

impl ProviderId {
    /// Creates a provider identity after rejecting blank text.
    pub fn new(value: impl Into<String>) -> Result<Self, ValueRequired> {
        required_text("provider id", value).map(Self)
    }

    #[must_use]
    /// Returns the harness-reported provider identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Driver-owned name for one reasoning mode accepted by a model route.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ReasoningMode(String);

impl ReasoningMode {
    /// Creates a reasoning-mode name after rejecting blank text.
    pub fn new(value: impl Into<String>) -> Result<Self, ValueRequired> {
        required_text("reasoning mode", value).map(Self)
    }

    #[must_use]
    /// Returns the driver-owned reasoning-mode name.
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
/// Optional input and output token ceilings reported for a model.
pub struct ModelTokenLimits {
    maximum_input_tokens: Option<u64>,
    maximum_output_tokens: Option<u64>,
}

impl ModelTokenLimits {
    /// Creates token-limit metadata without manufacturing absent bounds.
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
    /// Returns the reported maximum input-token count.
    pub const fn maximum_input_tokens(&self) -> Option<u64> {
        self.maximum_input_tokens
    }

    #[must_use]
    /// Returns the reported maximum output-token count.
    pub const fn maximum_output_tokens(&self) -> Option<u64> {
        self.maximum_output_tokens
    }
}

impl ReasoningMetadata {
    /// Creates reasoning metadata from supported modes and an optional default.
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

    /// Iterates supported reasoning modes in stable order.
    pub fn supported_modes(&self) -> impl ExactSizeIterator<Item = &ReasoningMode> {
        self.supported_modes.iter()
    }

    #[must_use]
    /// Returns the provider-reported default mode, when known.
    pub const fn default_mode(&self) -> Option<&ReasoningMode> {
        self.default_mode.as_ref()
    }

    #[must_use]
    /// Reports whether the catalogue advertises `mode`.
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
    /// Starts model metadata with a validated display name.
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

    /// Adds a validated human-readable model description.
    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Result<Self, ValueRequired> {
        self.description = Some(required_text("model description", description)?);
        Ok(self)
    }

    #[must_use]
    /// Marks whether this model is the catalogue default.
    pub const fn with_default(mut self, is_default: bool) -> Self {
        self.is_default = is_default;
        self
    }

    #[must_use]
    /// Adds model-specific reasoning metadata.
    pub fn with_reasoning(mut self, reasoning: ReasoningMetadata) -> Self {
        self.reasoning = Some(reasoning);
        self
    }

    #[must_use]
    /// Adds provider-reported token limits.
    pub const fn with_token_limits(mut self, token_limits: ModelTokenLimits) -> Self {
        self.token_limits = Some(token_limits);
        self
    }

    #[must_use]
    /// Adds observations from the exact catalogue source.
    pub fn with_catalog_observations(mut self, observations: ModelCatalogObservations) -> Self {
        self.catalog_observations = Some(observations);
        self
    }

    #[must_use]
    /// Replaces observations made against an attached model runtime.
    pub fn with_attached_model_observations(
        mut self,
        observations: impl IntoIterator<Item = AttachedModelObservation>,
    ) -> Self {
        self.attached_model_observations = observations.into_iter().collect();
        self
    }

    #[must_use]
    /// Returns the optional human-readable display name.
    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }

    #[must_use]
    /// Returns the optional human-readable description.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    #[must_use]
    /// Reports whether this model is the catalogue default.
    pub const fn is_default(&self) -> bool {
        self.is_default
    }

    #[must_use]
    /// Returns model-specific reasoning metadata, when observed.
    pub const fn reasoning(&self) -> Option<&ReasoningMetadata> {
        self.reasoning.as_ref()
    }

    #[must_use]
    /// Returns provider-reported token limits, when observed.
    pub const fn token_limits(&self) -> Option<ModelTokenLimits> {
        self.token_limits
    }

    #[must_use]
    /// Returns observations from the exact catalogue source, when present.
    pub const fn catalog_observations(&self) -> Option<&ModelCatalogObservations> {
        self.catalog_observations.as_ref()
    }

    /// Iterates observations made against attached model runtimes.
    pub fn attached_model_observations(
        &self,
    ) -> impl ExactSizeIterator<Item = &AttachedModelObservation> {
        self.attached_model_observations.iter()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
/// One stable model identity and its mutable catalogue presentation.
pub struct ModelCatalogEntry {
    id: ModelId,
    provider_id: Option<ProviderId>,
    metadata: ModelMetadata,
}

impl ModelCatalogEntry {
    /// Creates an entry without a separate provider identity.
    #[must_use]
    pub const fn new(id: ModelId, metadata: ModelMetadata) -> Self {
        Self {
            id,
            provider_id: None,
            metadata,
        }
    }

    #[must_use]
    /// Adds the provider identity reported separately by a harness.
    pub fn with_provider_id(mut self, provider_id: ProviderId) -> Self {
        self.provider_id = Some(provider_id);
        self
    }

    #[must_use]
    /// Returns the stable adapter-owned model identity.
    pub const fn id(&self) -> &ModelId {
        &self.id
    }

    #[must_use]
    /// Returns the separate provider identity, when reported.
    pub const fn provider_id(&self) -> Option<&ProviderId> {
        self.provider_id.as_ref()
    }

    #[must_use]
    /// Returns the mutable catalogue metadata.
    pub const fn metadata(&self) -> &ModelMetadata {
        &self.metadata
    }
}

#[cfg(test)]
mod tests;
