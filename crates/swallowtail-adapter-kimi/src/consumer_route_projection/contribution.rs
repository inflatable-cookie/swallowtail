#[path = "contribution/local.rs"]
mod local;

use super::builder::{Projection, Route};
use crate::{
    KimiHeadlessPreparedRun, KimiPreparedSession, KimiPreparedSessionCatalogue,
    KimiPreparedSessionImport,
};
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteApplicability, ConsumerRouteAvailability,
    ConsumerRouteControlId, ConsumerRouteControlValue, ConsumerRouteEvidenceStrength,
    ConsumerRouteLifecycle, ConsumerRouteMutationAuthority, ConsumerRouteNamespacedExtension,
    ConsumerRouteOmissionSemantics, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailure, ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceId,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind,
    ConsumerRouteProviderOperationObservation, ConsumerRouteProviderOperationOutcome,
    ConsumerRouteRowIdentity, ConsumerRouteSourceClass, ConsumerRouteStateSupport,
    ConsumerRouteSupportPosture, ConsumerRouteValueDomain, ConsumerRouteValueKind,
    ProviderSessionCatalogueOutcome,
};

impl KimiPreparedSession {
    /// Emits only the prepared ACP rows proved by this exact session.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        let attachment = self.request().options().reasoning_mode().is_none()
            && self.request().options().harness_mode().is_none();
        let mut projection = Projection::prepared(self.plan(), Route::Acp, source_id)
            .capabilities()
            .model_selection()
            .reasoning_control(
                self.request()
                    .options()
                    .reasoning_mode()
                    .map(|mode| mode.as_str()),
                true,
            )
            .session_options("portable reasoning and Plan session options");
        if attachment {
            projection = projection
                .portable_control(ConsumerRouteControlId::LoadSession, "exact resume binding")
                .portable_control(
                    ConsumerRouteControlId::ResumeSession,
                    "exact resume binding",
                );
        }
        projection.build()
    }
}

impl KimiPreparedSessionCatalogue {
    /// Emits only prepared provider-session catalogue capability truth.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::prepared(self.plan().preflight(), Route::Acp, source_id)
            .capabilities()
            .build()
    }

    /// Projects the catalogue control only from one matching completed outcome.
    pub fn consumer_route_provider_operation_observation(
        &self,
        outcome: &ProviderSessionCatalogueOutcome,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProviderOperationObservation, ConsumerRouteProjectionFailure> {
        let source = ConsumerRouteProjectionSourceIdentity::new(
            source_id,
            ConsumerRouteProjectionSourceKind::ProviderOperationObservation,
        );
        let applicability =
            ConsumerRouteApplicability::from_prepared_operation(self.evidence().operation());
        let extension = ConsumerRouteNamespacedExtension::new(
            Route::Acp.id(),
            self.plan().preflight().protocol_facade_id().as_str(),
            "control.provider-session-catalogue",
        )?;
        let row = ConsumerRouteProjectionRow::new(
            ConsumerRouteRowIdentity::Control(ConsumerRouteControlId::Namespaced(extension)),
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
        ConsumerRouteProviderOperationObservation::new(
            self.evidence().operation(),
            ConsumerRouteProviderOperationOutcome::ProviderSessionCatalogue(outcome),
            source,
            [row],
        )
    }
}

impl KimiPreparedSessionImport {
    /// Emits exact prepared import and durable-session truth.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::prepared(self.plan().preflight(), Route::Acp, source_id)
            .capabilities()
            .portable_feature(swallowtail_runtime::ConsumerRouteFeatureId::PersistentSessionPosture)
            .named_control(
                "control.provider-session-import",
                "exact admitted provider-session candidate",
                ConsumerRouteOmissionSemantics::Required,
            )
            .build()
    }
}

impl KimiHeadlessPreparedRun {
    /// Emits exact prepared headless structured-run truth.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        Projection::prepared(self.plan(), Route::Headless, source_id)
            .capabilities()
            .named_feature("feature.provider-managed-recovery")
            .portable_feature(swallowtail_runtime::ConsumerRouteFeatureId::PersistentSessionPosture)
            .model_selection()
            .named_control(
                "control.provider-managed-recovery",
                "explicit provider-managed recovery acceptance",
                ConsumerRouteOmissionSemantics::Required,
            )
            .build()
    }
}
