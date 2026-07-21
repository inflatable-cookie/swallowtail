mod lifecycle;
mod observations;
mod value;

pub use lifecycle::{
    CatalogTimestamp, ModelLifecycleObservation, ModelLifecycleStatus, ModelLifecycleTransition,
};
pub use observations::ModelCatalogObservations;
pub use value::{CatalogObservation, InvalidCatalogObservation, ProviderCatalogValue};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ModelModality {
    Text,
    Image,
    Embedding,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ModelInferenceType {
    OnDemand,
    Provisioned,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ModelCustomizationType {
    FineTuning,
    ContinuedPreTraining,
    Distillation,
}

#[cfg(test)]
mod tests;
