use super::{
    CatalogObservation, ModelCustomizationType, ModelInferenceType, ModelLifecycleObservation,
    ModelModality,
};
use crate::diagnostic::{ValueRequired, required_text};
use crate::runtime_identity::IntegrationFamilyId;
use std::collections::BTreeSet;

/// Mutable observations from one exact catalogue source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelCatalogObservations {
    source: IntegrationFamilyId,
    provider_display_name: Option<String>,
    input_modalities: Option<BTreeSet<CatalogObservation<ModelModality>>>,
    output_modalities: Option<BTreeSet<CatalogObservation<ModelModality>>>,
    response_streaming_supported: Option<bool>,
    inference_types: Option<BTreeSet<CatalogObservation<ModelInferenceType>>>,
    customization_types: Option<BTreeSet<CatalogObservation<ModelCustomizationType>>>,
    lifecycle: Option<ModelLifecycleObservation>,
}

impl ModelCatalogObservations {
    #[must_use]
    pub fn new(source: IntegrationFamilyId) -> Self {
        Self {
            source,
            provider_display_name: None,
            input_modalities: None,
            output_modalities: None,
            response_streaming_supported: None,
            inference_types: None,
            customization_types: None,
            lifecycle: None,
        }
    }

    pub fn with_provider_display_name(
        mut self,
        provider_display_name: impl Into<String>,
    ) -> Result<Self, ValueRequired> {
        self.provider_display_name = Some(required_text(
            "provider display name",
            provider_display_name,
        )?);
        Ok(self)
    }

    #[must_use]
    pub fn with_input_modalities(
        mut self,
        modalities: impl IntoIterator<Item = CatalogObservation<ModelModality>>,
    ) -> Self {
        self.input_modalities = Some(modalities.into_iter().collect());
        self
    }

    #[must_use]
    pub fn with_output_modalities(
        mut self,
        modalities: impl IntoIterator<Item = CatalogObservation<ModelModality>>,
    ) -> Self {
        self.output_modalities = Some(modalities.into_iter().collect());
        self
    }

    #[must_use]
    pub const fn with_response_streaming_supported(mut self, supported: bool) -> Self {
        self.response_streaming_supported = Some(supported);
        self
    }

    #[must_use]
    pub fn with_inference_types(
        mut self,
        inference_types: impl IntoIterator<Item = CatalogObservation<ModelInferenceType>>,
    ) -> Self {
        self.inference_types = Some(inference_types.into_iter().collect());
        self
    }

    #[must_use]
    pub fn with_customization_types(
        mut self,
        customization_types: impl IntoIterator<Item = CatalogObservation<ModelCustomizationType>>,
    ) -> Self {
        self.customization_types = Some(customization_types.into_iter().collect());
        self
    }

    #[must_use]
    pub fn with_lifecycle(mut self, lifecycle: ModelLifecycleObservation) -> Self {
        self.lifecycle = Some(lifecycle);
        self
    }

    #[must_use]
    pub const fn source(&self) -> &IntegrationFamilyId {
        &self.source
    }

    #[must_use]
    pub fn provider_display_name(&self) -> Option<&str> {
        self.provider_display_name.as_deref()
    }

    #[must_use]
    pub const fn input_modalities(&self) -> Option<&BTreeSet<CatalogObservation<ModelModality>>> {
        self.input_modalities.as_ref()
    }

    #[must_use]
    pub const fn output_modalities(&self) -> Option<&BTreeSet<CatalogObservation<ModelModality>>> {
        self.output_modalities.as_ref()
    }

    #[must_use]
    pub const fn response_streaming_supported(&self) -> Option<bool> {
        self.response_streaming_supported
    }

    #[must_use]
    pub const fn inference_types(
        &self,
    ) -> Option<&BTreeSet<CatalogObservation<ModelInferenceType>>> {
        self.inference_types.as_ref()
    }

    #[must_use]
    pub const fn customization_types(
        &self,
    ) -> Option<&BTreeSet<CatalogObservation<ModelCustomizationType>>> {
        self.customization_types.as_ref()
    }

    #[must_use]
    pub const fn lifecycle(&self) -> Option<&ModelLifecycleObservation> {
        self.lifecycle.as_ref()
    }
}
