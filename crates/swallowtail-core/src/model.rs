use crate::diagnostic::{ValueRequired, required_text};
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
}

impl ModelMetadata {
    pub fn with_display_name(display_name: impl Into<String>) -> Result<Self, ValueRequired> {
        Ok(Self {
            display_name: Some(required_text("model display name", display_name)?),
            description: None,
            is_default: false,
            reasoning: None,
            token_limits: None,
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
mod tests {
    use super::{
        ModelCatalogEntry, ModelId, ModelMetadata, ModelTokenLimits, ProviderId, ReasoningMetadata,
        ReasoningMode,
    };

    #[test]
    fn stable_identity_is_separate_from_display_metadata() {
        let id = ModelId::new("provider/model-1").expect("model id is valid");
        let entry = ModelCatalogEntry::new(
            id.clone(),
            ModelMetadata::with_display_name("Model One")
                .expect("display name is valid")
                .with_description("A model")
                .expect("description is valid")
                .with_default(true),
        );

        assert_eq!(entry.id(), &id);
        assert_eq!(entry.metadata().display_name(), Some("Model One"));
        assert_eq!(entry.metadata().description(), Some("A model"));
        assert!(entry.metadata().is_default());
    }

    #[test]
    fn harness_provider_and_model_ids_remain_separate() {
        let provider = ProviderId::new("anthropic").expect("provider id is valid");
        let model = ModelId::new("claude-sonnet").expect("model id is valid");
        let entry = ModelCatalogEntry::new(model.clone(), ModelMetadata::default())
            .with_provider_id(provider.clone());

        assert_eq!(entry.provider_id(), Some(&provider));
        assert_eq!(entry.id(), &model);
    }

    #[test]
    fn reasoning_metadata_is_evidence_not_an_implicit_selection() {
        let low = ReasoningMode::new("low").expect("mode is valid");
        let high = ReasoningMode::new("high").expect("mode is valid");
        let metadata = ModelMetadata::default().with_reasoning(ReasoningMetadata::new(
            [low.clone(), high],
            Some(low.clone()),
        ));

        let reasoning = metadata.reasoning().expect("reasoning evidence is present");
        assert!(reasoning.supports(&low));
        assert_eq!(reasoning.default_mode(), Some(&low));
    }

    #[test]
    fn absent_token_limits_are_unknown_and_observed_limits_are_mutable_metadata() {
        let unknown = ModelMetadata::default();
        assert_eq!(unknown.token_limits(), None);

        let observed = unknown.with_token_limits(ModelTokenLimits::new(Some(200_000), Some(8_192)));
        assert_eq!(
            observed
                .token_limits()
                .and_then(|limits| limits.maximum_input_tokens()),
            Some(200_000)
        );
        assert_eq!(
            observed
                .token_limits()
                .and_then(|limits| limits.maximum_output_tokens()),
            Some(8_192)
        );
    }
}
