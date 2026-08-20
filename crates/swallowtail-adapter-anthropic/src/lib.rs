//! Anthropic integration drivers for Swallowtail.
//!
#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod activity;
mod addable;
mod driver;
mod failure;
mod managed;
mod managed_activity;
mod managed_driver;
mod managed_recovery;
mod managed_selection;
mod managed_transport;
mod prepared;
mod prepared_managed;
mod prepared_managed_profile;
mod prepared_managed_recovery;
mod prepared_profile;
mod protocol;
mod selection;
mod transport;

pub use addable::{
    ANTHROPIC_MESSAGES_ADDABLE_ROUTE_ID, ANTHROPIC_MESSAGES_API_KEY_ENVIRONMENT_NAME,
    ANTHROPIC_MESSAGES_API_KEY_FIELD_ID, ANTHROPIC_MESSAGES_ENDPOINT_FIELD_ID,
    anthropic_messages_addable_route_descriptor,
};
pub use driver::{AnthropicDirectDriver, anthropic_direct_descriptor};
pub use managed_driver::{AnthropicManagedAgentDriver, anthropic_managed_agent_descriptor};
pub use managed_selection::{
    ANTHROPIC_MANAGED_ACCESS_PROFILE_ID, ANTHROPIC_MANAGED_API_ENDPOINT,
    ANTHROPIC_MANAGED_ENDPOINT_AUDIENCE, ANTHROPIC_MANAGED_FACADE_REVISION,
    anthropic_managed_access_profile, anthropic_managed_facade_binding,
    anthropic_managed_facade_claim, anthropic_managed_instance, anthropic_managed_model_route,
    anthropic_managed_requirements,
};
pub use prepared::{
    AnthropicPreparationInput, AnthropicPreparedIntegration, prepare_anthropic_direct,
};
pub use prepared_managed::{
    AnthropicManagedPreparationInput, AnthropicManagedPreparedIntegration,
    prepare_anthropic_managed_agent,
};
pub use prepared_managed_profile::{
    AnthropicManagedAgentRunInput, AnthropicManagedModelSelection,
    AnthropicManagedPreparedEvidence, AnthropicPreparedManagedAgentRun,
};
pub use prepared_managed_recovery::{
    AnthropicManagedRecoveredCleanupInput, AnthropicManagedRunReconciliationInput,
    AnthropicPreparedManagedRecoveredCleanup, AnthropicPreparedManagedRunReconciliation,
};
pub use prepared_profile::{
    AnthropicCatalogueProfileInput, AnthropicInferenceAttemptInput, AnthropicModelSelection,
    AnthropicPreparedCatalogue, AnthropicPreparedEvidence, AnthropicPreparedInferenceAttempt,
    AnthropicPreparedSession, AnthropicSessionProfileInput, AnthropicWebSearchInput,
    anthropic_messages_continuation_config,
};
pub use selection::{
    ANTHROPIC_MESSAGES_FACADE_REVISION, anthropic_messages_facade_binding,
    anthropic_messages_facade_claim,
};
