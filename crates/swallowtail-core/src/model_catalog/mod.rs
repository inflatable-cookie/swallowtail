#![deny(missing_docs)]

mod lifecycle;
mod observations;
mod value;

pub use lifecycle::{
    CatalogTimestamp, ModelLifecycleObservation, ModelLifecycleStatus, ModelLifecycleTransition,
};
pub use observations::ModelCatalogObservations;
pub use value::{CatalogObservation, InvalidCatalogObservation, ProviderCatalogValue};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Common input or output modality reported by a model catalogue.
pub enum ModelModality {
    /// Natural-language or structured text.
    Text,
    /// Raster or vector image content.
    Image,
    /// Numeric embedding vectors.
    Embedding,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Common provider allocation mode for model inference.
pub enum ModelInferenceType {
    /// Capacity allocated when a request is made.
    OnDemand,
    /// Pre-allocated provider capacity.
    Provisioned,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
/// Common model-customization technique advertised by a catalogue.
pub enum ModelCustomizationType {
    /// Fine-tuning from labelled examples.
    FineTuning,
    /// Additional pre-training on a selected corpus.
    ContinuedPreTraining,
    /// Training a smaller model from a teacher model.
    Distillation,
}

#[cfg(test)]
mod tests;
