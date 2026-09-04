//! Contract 061 contributions from exact prepared DeepSeek operations.

#[path = "consumer_route_projection/builder.rs"]
mod builder;

use builder::Projection;

use crate::{DeepSeekPreparedCatalogue, DeepSeekPreparedRun, DeepSeekPreparedSession};
use swallowtail_core::{
    AccessStatus, Capability, CredentialState, EndpointAuthorization, EntitlementState,
    PreflightPlan, RuntimeReadiness,
};
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteApplicability, ConsumerRouteAvailability,
    ConsumerRouteControlId, ConsumerRouteControlValue, ConsumerRouteEnumerableValue,
    ConsumerRouteEnumeratedValues, ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteMutationAuthority, ConsumerRouteNamespacedExtension,
    ConsumerRouteOmissionSemantics, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailure, ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceId,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind,
    ConsumerRouteRowIdentity, ConsumerRouteSourceClass, ConsumerRouteStateSupport,
    ConsumerRouteSupportPosture, ConsumerRouteValueDomain, ConsumerRouteValueKind,
};

const ROUTE: &str = "deepseek.continuation";

impl DeepSeekPreparedCatalogue {
    /// Emits the exact catalogue contribution proved by this prepared operation.
    pub fn consumer_route_projection_contribution(
        &self,
        source: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), source)
            .with_prepared_facade()
            .with_feature(ConsumerRouteFeatureId::ModelCatalogue)
            .build()
    }
}

impl DeepSeekPreparedRun {
    /// Emits the exact structured-run contribution proved by this prepared operation.
    pub fn consumer_route_projection_contribution(
        &self,
        source: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        let mut projection = Projection::new(self.plan(), source)
            .with_prepared_capabilities()
            .with_model_selection();
        if let Some(reasoning) = self.evidence().reasoning_mode() {
            projection = projection.with_control(
                ConsumerRouteControlId::ReasoningSelection,
                ConsumerRouteValueKind::BoundedEnumeration,
                reasoning.as_str(),
                ConsumerRouteOmissionSemantics::Required,
            );
        } else if let Some(thinking) = self.evidence().thinking_mode() {
            projection = projection.with_control(
                ConsumerRouteControlId::ReasoningSelection,
                ConsumerRouteValueKind::BoundedEnumeration,
                thinking.as_str(),
                ConsumerRouteOmissionSemantics::Required,
            );
        }
        let maximum = self
            .request()
            .maximum_output_tokens()
            .expect("DeepSeek run preparation requires a maximum");
        projection
            .with_control(
                ConsumerRouteControlId::MaximumOutputTokens,
                ConsumerRouteValueKind::BoundedInteger,
                &maximum.get().to_string(),
                ConsumerRouteOmissionSemantics::Required,
            )
            .with_namespaced_control(
                "control.inference-cache-policy",
                ConsumerRouteValueKind::BoundedPolicy,
                cache_policy_value(),
                ConsumerRouteOmissionSemantics::Required,
            )
            .build()
    }
}

impl DeepSeekPreparedSession {
    /// Emits the exact interactive-session contribution proved by this preparation.
    pub fn consumer_route_projection_contribution(
        &self,
        source: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        let options = self.request().options();
        let reasoning = options
            .reasoning_mode()
            .expect("DeepSeek session preparation requires reasoning");
        Projection::new(self.plan(), source)
            .with_prepared_capabilities()
            .with_model_selection()
            .with_control(
                ConsumerRouteControlId::ReasoningSelection,
                ConsumerRouteValueKind::BoundedEnumeration,
                reasoning.as_str(),
                ConsumerRouteOmissionSemantics::Required,
            )
            .with_namespaced_control(
                "control.inference-cache-policy",
                ConsumerRouteValueKind::BoundedPolicy,
                cache_policy_value(),
                ConsumerRouteOmissionSemantics::Required,
            )
            .with_control(
                ConsumerRouteControlId::ToolDeclarations,
                ConsumerRouteValueKind::StructuredDeclarations,
                &format!(
                    "{} declared tools (admitted range 1..=8)",
                    options.tools().len()
                ),
                ConsumerRouteOmissionSemantics::Required,
            )
            .build()
    }
}

fn cache_policy_value() -> &'static str {
    "accepted-without-management-authority"
}

fn feature_for(capability: Capability) -> Option<ConsumerRouteFeatureId> {
    Some(match capability {
        Capability::StructuredRun => ConsumerRouteFeatureId::StructuredRun,
        Capability::InteractiveSession => ConsumerRouteFeatureId::InteractiveSession,
        Capability::StreamingEvents => ConsumerRouteFeatureId::StreamingEvents,
        Capability::UsageReporting => ConsumerRouteFeatureId::UsageEvidence,
        Capability::OutputTokenLimit => ConsumerRouteFeatureId::OutputTokenLimit,
        Capability::ReasoningSelection => ConsumerRouteFeatureId::ReasoningSelection,
        Capability::ToolCalls => ConsumerRouteFeatureId::ConsumerToolExchange,
        Capability::Interruption => ConsumerRouteFeatureId::CancellationOrInterruption,
        Capability::ObservableActivity => ConsumerRouteFeatureId::ActivityObservation,
        _ => return None,
    })
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
