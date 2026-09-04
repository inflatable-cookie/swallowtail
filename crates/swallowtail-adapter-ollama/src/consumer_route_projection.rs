//! Contract 061 contributions from the prepared Ollama attached facades.
//!
//! Every row is proved by one exact prepared plan, retained request value, or
//! adapter-local prepared evidence. Twin census rows are keyed by
//! `(operation shape, semantic id)` and emitted only from that shape's facade.
//! Interactive-session reasoning is withheld at construction.

#[path = "consumer_route_projection/builder.rs"]
mod builder;

use crate::{OllamaPreparedInferenceAttempt, OllamaPreparedInventory, OllamaPreparedSession};
use builder::Projection;
use swallowtail_core::{
    AccessStatus, Capability, CredentialState, EndpointAuthorization, EntitlementState,
    RuntimeReadiness,
};
use swallowtail_runtime::{
    ConsumerRouteAvailability, ConsumerRouteControlId, ConsumerRouteFeatureId,
    ConsumerRouteNamespacedExtension, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailure, ConsumerRouteProjectionSourceId,
};

/// Exact census route the native attached Ollama facades belong to.
pub(crate) const OLLAMA_ATTACHED_ROUTE: &str = "ollama.attached";

impl OllamaPreparedInventory {
    /// Emits only the attached catalogue truth this prepared inventory proves.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), source_id)
            .with_prepared_capabilities()
            .build()
    }
}

impl OllamaPreparedInferenceAttempt {
    /// Emits only the structured-run truth this prepared attempt proves.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        let mut projection = Projection::new(self.plan(), source_id)
            .with_prepared_capabilities()
            .with_model_selection()
            .with_maximum_output_tokens(self.request().maximum_output_tokens())
            .with_context_window(self.evidence().context_window());
        if let Some(mode) = self.request().policy().reasoning_mode() {
            projection = projection.with_reasoning_selection(mode.as_str());
        }
        if self.request().structured_output().is_some() {
            projection = projection.with_structured_output();
        }
        projection.build()
    }
}

impl OllamaPreparedSession {
    /// Emits only the interactive-session truth this prepared session proves.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), source_id)
            .with_prepared_capabilities()
            .with_model_selection()
            .with_context_window(self.evidence().context_window())
            .build()
    }
}

const fn feature_for(capability: Capability) -> Option<ConsumerRouteFeatureId> {
    Some(match capability {
        Capability::ModelCatalog => ConsumerRouteFeatureId::ModelCatalogue,
        Capability::StructuredRun => ConsumerRouteFeatureId::StructuredRun,
        Capability::InteractiveSession => ConsumerRouteFeatureId::InteractiveSession,
        Capability::StreamingEvents => ConsumerRouteFeatureId::StreamingEvents,
        Capability::UsageReporting => ConsumerRouteFeatureId::UsageEvidence,
        Capability::OutputTokenLimit => ConsumerRouteFeatureId::OutputTokenLimit,
        Capability::ReasoningSelection => ConsumerRouteFeatureId::ReasoningSelection,
        Capability::StructuredOutput => ConsumerRouteFeatureId::StructuredOutput,
        Capability::Interruption => ConsumerRouteFeatureId::CancellationOrInterruption,
        Capability::ObservableActivity => ConsumerRouteFeatureId::ActivityObservation,
        _ => return None,
    })
}

fn namespaced_control(
    segment: &str,
    semantic_id: &str,
) -> Result<ConsumerRouteControlId, ConsumerRouteProjectionFailure> {
    ConsumerRouteNamespacedExtension::new(OLLAMA_ATTACHED_ROUTE, segment, semantic_id)
        .map(ConsumerRouteControlId::Namespaced)
}

fn exact(
    value: &str,
) -> Result<swallowtail_runtime::ConsumerRouteValueDomain, ConsumerRouteProjectionFailure> {
    Ok(swallowtail_runtime::ConsumerRouteValueDomain::Enumerated(
        swallowtail_runtime::ConsumerRouteEnumeratedValues::new([
            swallowtail_runtime::ConsumerRouteEnumerableValue::new(value)?,
        ])?,
    ))
}

fn bounded(
    value: &str,
) -> Result<swallowtail_runtime::ConsumerRouteValueDomain, ConsumerRouteProjectionFailure> {
    Ok(swallowtail_runtime::ConsumerRouteValueDomain::Unenumerated(
        swallowtail_runtime::ConsumerRouteEnumerableValue::new(value)?,
    ))
}

const fn availability(status: &AccessStatus) -> ConsumerRouteAvailability {
    if matches!(
        status.credential(),
        CredentialState::Ready | CredentialState::NotRequired
    ) && matches!(status.entitlement(), EntitlementState::Available)
        && matches!(
            status.endpoint_authorization(),
            EndpointAuthorization::Allowed
        )
        && matches!(status.runtime_readiness(), RuntimeReadiness::Ready)
    {
        ConsumerRouteAvailability::Available
    } else {
        ConsumerRouteAvailability::Conditional
    }
}
