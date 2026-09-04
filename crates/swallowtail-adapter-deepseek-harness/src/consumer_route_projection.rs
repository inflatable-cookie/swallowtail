//! Contract 061 contributions from exact prepared DeepSeek Harness operations.

#[path = "consumer_route_projection/builder.rs"]
mod builder;

use builder::Projection;

use crate::{
    DeepSeekHarnessPreparedRun, DeepSeekHarnessWebPreparedArchive, DeepSeekHarnessWebPreparedFork,
    DeepSeekHarnessWebPreparedRun, DeepSeekHarnessWebPreparedSessionCatalogue,
    DeepSeekHarnessWebPreparedSessionHistory,
};
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
    ConsumerRouteProviderOperationObservation, ConsumerRouteProviderOperationOutcome,
    ConsumerRouteRowIdentity, ConsumerRouteSourceClass, ConsumerRouteStateSupport,
    ConsumerRouteSupportPosture, ConsumerRouteValueDomain, ConsumerRouteValueKind,
    ProviderSessionCatalogueOutcome, ProviderSessionHistoryPage,
};

const JSONRPC_ROUTE: &str = "deepseek-harness.jsonrpc";
const WEB_ROUTE: &str = "deepseek-harness.local-server";

impl DeepSeekHarnessPreparedRun {
    /// Emits the exact JSON-RPC structured-run contribution.
    pub fn consumer_route_projection_contribution(
        &self,
        source: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), source, JSONRPC_ROUTE)
            .with_prepared_capabilities()
            .with_model_selection()
            .build()
    }
}

impl DeepSeekHarnessWebPreparedRun {
    /// Emits the exact local-server structured-run contribution.
    pub fn consumer_route_projection_contribution(
        &self,
        source: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan(), source, WEB_ROUTE)
            .with_prepared_capabilities()
            .with_model_selection()
            .build()
    }
}

impl DeepSeekHarnessWebPreparedSessionCatalogue {
    /// Emits prepared catalogue support without claiming a completed query.
    pub fn consumer_route_projection_contribution(
        &self,
        source: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan().preflight(), source, WEB_ROUTE)
            .with_prepared_facade()
            .with_feature(ConsumerRouteFeatureId::ProviderSessionCatalogue)
            .with_feature(ConsumerRouteFeatureId::WorkingResource)
            .build()
    }

    /// Admits catalogue observation only from this plan's successful outcome.
    pub fn consumer_route_provider_operation_observation(
        &self,
        outcome: &ProviderSessionCatalogueOutcome,
        source: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProviderOperationObservation, ConsumerRouteProjectionFailure> {
        provider_operation_observation(
            self.evidence().operation(),
            ConsumerRouteProviderOperationOutcome::ProviderSessionCatalogue(outcome),
            source,
            "control.provider-session-catalogue",
        )
    }
}

impl DeepSeekHarnessWebPreparedSessionHistory {
    /// Emits prepared history support without claiming a completed query.
    pub fn consumer_route_projection_contribution(
        &self,
        source: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan().preflight(), source, WEB_ROUTE)
            .with_prepared_facade()
            .build()
    }

    /// Admits history observation only from this plan's successful outcome.
    pub fn consumer_route_provider_operation_observation(
        &self,
        outcome: &ProviderSessionHistoryPage,
        source: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProviderOperationObservation, ConsumerRouteProjectionFailure> {
        provider_operation_observation(
            self.evidence().operation(),
            ConsumerRouteProviderOperationOutcome::ProviderSessionHistory(outcome),
            source,
            "control.provider-session-history",
        )
    }
}

impl DeepSeekHarnessWebPreparedFork {
    /// Emits the exact prepared native-fork control.
    pub fn consumer_route_projection_contribution(
        &self,
        source: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan().preflight(), source, WEB_ROUTE)
            .with_prepared_facade()
            .with_namespaced_control(
                "control.provider-session-fork",
                ConsumerRouteValueKind::LifecycleAction,
                "exact provider session; optional u64 event sequence",
                ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
            )
            .build()
    }
}

impl DeepSeekHarnessWebPreparedArchive {
    /// Emits the exact prepared native-archive control.
    pub fn consumer_route_projection_contribution(
        &self,
        source: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::new(self.plan().preflight(), source, WEB_ROUTE)
            .with_prepared_facade()
            .with_feature(ConsumerRouteFeatureId::ProviderSessionArchive)
            .with_namespaced_control(
                "control.provider-session-archive",
                ConsumerRouteValueKind::LifecycleAction,
                "exact inactive provider-session management binding",
                ConsumerRouteOmissionSemantics::Required,
            )
            .build()
    }
}

fn provider_operation_observation(
    evidence: &swallowtail_runtime::PreparedOperationEvidence,
    outcome: ConsumerRouteProviderOperationOutcome<'_>,
    source_id: ConsumerRouteProjectionSourceId,
    semantic: &str,
) -> Result<ConsumerRouteProviderOperationObservation, ConsumerRouteProjectionFailure> {
    let source = ConsumerRouteProjectionSourceIdentity::new(
        source_id,
        ConsumerRouteProjectionSourceKind::ProviderOperationObservation,
    );
    let applicability = ConsumerRouteApplicability::from_prepared_operation(evidence);
    let identity = ConsumerRouteControlId::Namespaced(ConsumerRouteNamespacedExtension::new(
        WEB_ROUTE,
        evidence.plan().protocol_facade_id().as_str(),
        semantic,
    )?);
    let row = ConsumerRouteProjectionRow::new(
        ConsumerRouteRowIdentity::Control(identity),
        applicability,
        source.clone(),
        ConsumerRouteSourceClass::ProviderOperationOutcome,
        ConsumerRouteEvidenceStrength::CompletedProviderOperation,
        ConsumerRouteLifecycle::PostOperationObservationOnly,
    )
    .with_support(ConsumerRouteSupportPosture::Supported)
    .with_availability(ConsumerRouteAvailability::Available)
    .with_actor_posture(ConsumerRouteActorPosture::ObservationOnly)
    .with_state_support(ConsumerRouteStateSupport::descriptor_only().with_observed())
    .with_mutation_authority(ConsumerRouteMutationAuthority::Absent)
    .with_control_value(ConsumerRouteControlValue::new(
        ConsumerRouteValueKind::BoundedQuery,
        ConsumerRouteValueDomain::Descriptor,
        ConsumerRouteOmissionSemantics::NotSelectable,
    ));
    ConsumerRouteProviderOperationObservation::new(evidence, outcome, source, [row])
}

#[cfg(test)]
#[path = "consumer_route_projection/tests.rs"]
mod tests;
