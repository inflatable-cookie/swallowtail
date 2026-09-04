//! Contract 061 contributions from the prepared llama.cpp attached and owned facades.
//!
//! Every row is proved by one exact prepared plan, retained request value, or
//! adapter-local prepared evidence. Attached and owned stay distinct routes.
//! `StreamingEvents` and `Interruption` on the owned plan have no census row
//! and are skipped. Attached interruption is withheld at construction.

#[path = "consumer_route_projection/builder.rs"]
mod builder;

use crate::{
    LlamaCppPreparedCatalogue, LlamaCppPreparedInferenceAttempt, LlamaCppPreparedServingStart,
};
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

/// Exact census route for the externally attached llama.cpp driver.
pub(crate) const LLAMA_CPP_ATTACHED_ROUTE: &str = "llama-cpp.attached";
/// Exact census route for the host-owned llama.cpp driver.
pub(crate) const LLAMA_CPP_OWNED_ROUTE: &str = "llama-cpp.owned";

impl LlamaCppPreparedCatalogue {
    /// Emits only the attached catalogue truth this prepared operation proves.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(
            self.plan(),
            LLAMA_CPP_ATTACHED_ROUTE,
            source_id,
            attached_feature,
        )
        .with_prepared_capabilities()
        .build()
    }
}

impl LlamaCppPreparedInferenceAttempt {
    /// Emits only the attached structured-run truth this prepared attempt proves.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(
            self.plan(),
            LLAMA_CPP_ATTACHED_ROUTE,
            source_id,
            attached_feature,
        )
        .with_prepared_capabilities()
        .with_model_selection()
        .with_maximum_output_tokens(self.request().maximum_output_tokens())
        .build()
    }
}

impl LlamaCppPreparedServingStart {
    /// Emits only the owned serving-start truth this prepared operation proves.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), LLAMA_CPP_OWNED_ROUTE, source_id, owned_feature)
            .with_prepared_capabilities()
            .with_owned_runtime_lifecycle()
            .with_serving_model_artifact()
            .with_serving_context_size(self.evidence().context_size())
            .with_serving_reasoning(self.evidence().reasoning())
            .build()
    }
}

/// Maps one attached prepared capability to a portable feature identity.
///
/// `Capability::Interruption` is absent: no attached prepared plan requires it.
const fn attached_feature(capability: Capability) -> Option<ConsumerRouteFeatureId> {
    Some(match capability {
        Capability::ModelCatalog => ConsumerRouteFeatureId::ModelCatalogue,
        Capability::StructuredRun => ConsumerRouteFeatureId::StructuredRun,
        Capability::StreamingEvents => ConsumerRouteFeatureId::StreamingEvents,
        Capability::UsageReporting => ConsumerRouteFeatureId::UsageEvidence,
        Capability::OutputTokenLimit => ConsumerRouteFeatureId::OutputTokenLimit,
        Capability::ObservableActivity => ConsumerRouteFeatureId::ActivityObservation,
        _ => return None,
    })
}

/// Maps one owned prepared capability to a portable feature identity.
///
/// `StreamingEvents` and `Interruption` have no `llama-cpp.owned` census row.
const fn owned_feature(capability: Capability) -> Option<ConsumerRouteFeatureId> {
    match capability {
        Capability::ObservableActivity => Some(ConsumerRouteFeatureId::ActivityObservation),
        _ => None,
    }
}

fn route_local(
    route: &str,
    segment: &str,
    semantic_id: &str,
) -> Result<ConsumerRouteNamespacedExtension, ConsumerRouteProjectionFailure> {
    ConsumerRouteNamespacedExtension::new(route, segment, semantic_id)
}

fn namespaced_control(
    route: &str,
    segment: &str,
    semantic_id: &str,
) -> Result<ConsumerRouteControlId, ConsumerRouteProjectionFailure> {
    route_local(route, segment, semantic_id).map(ConsumerRouteControlId::Namespaced)
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
