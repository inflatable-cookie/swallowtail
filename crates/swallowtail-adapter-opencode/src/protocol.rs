use crate::failure::failure;
use serde::Deserialize;
use serde_json::{Map, Value, json};
use std::collections::BTreeMap;
use swallowtail_core::{
    IntegrationFamilyId, InterfaceVersionBinding, ModelCatalogEntry, ModelCatalogObservations,
    ModelId, ModelMetadata, ModelTokenLimits, ProviderId, ReasoningMetadata, ReasoningMode,
};
use swallowtail_runtime::{
    CallbackResult, HarnessQuestionId, HarnessQuestionOptionId, HarnessUserInputChoiceMode,
    HarnessUserInputOption, HarnessUserInputQuestion, HarnessUserInputQuestionKind,
    HarnessUserInputRequest, OperationContent, StructuredOutputDescriptor,
};
use swallowtail_runtime::{RuntimeFailure, TokenUsage};

mod health;
pub(crate) use health::observe_health;
pub(crate) use health::require_health_matches;

include!("protocol/catalogue.rs");
include!("protocol/sessions.rs");
include!("protocol/prompt_and_callbacks.rs");

include!("protocol/events.rs");
include!("protocol/tests.rs");
