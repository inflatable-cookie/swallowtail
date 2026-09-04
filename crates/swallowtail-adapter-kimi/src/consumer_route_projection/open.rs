#[path = "open/admission.rs"]
mod admission;
#[path = "open/types.rs"]
mod types;

use super::builder::Projection;
use crate::KimiPreparedSession;
use crate::driver::{KimiAcknowledgement, KimiOpenObservation, KimiOpenRejection};
use admission::{AdmissionFailure, compound_acknowledgement};
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteApplicability, ConsumerRouteControlValue,
    ConsumerRouteEnumerableValue, ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteMutationAuthority, ConsumerRouteNamespacedExtension,
    ConsumerRouteOmissionSemantics, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceId, ConsumerRouteRowIdentity,
    ConsumerRouteSourceClass, ConsumerRouteStateSupport, ConsumerRouteValueDomain,
    ConsumerRouteValueKind, Deadline, HostServices, MonotonicInstant, RuntimeFailure,
    SessionCleanupRequest,
};

pub use admission::KimiProviderValue;
pub use types::{KimiProjectionOpenFailure, KimiProjectionOpenFuture, KimiProjectionOpenOutcome};

impl KimiPreparedSession {
    /// Opens Kimi ACP and publishes only exact admitted provider observations.
    pub fn open_session_with_projection(
        &self,
        prepared_source_id: ConsumerRouteProjectionSourceId,
        active_session_source_id: ConsumerRouteProjectionSourceId,
        services: HostServices,
    ) -> KimiProjectionOpenFuture {
        if prepared_source_id == active_session_source_id {
            return Box::pin(async {
                Err(KimiProjectionOpenFailure::Runtime(crate::failure::failure(
                    "swallowtail.kimi.projection_source_identity_invalid",
                    "Kimi prepared and active-session projection sources must differ",
                )))
            });
        }
        let prepared = self.clone();
        let driver = self.low_level_driver();
        Box::pin(async move {
            let lifecycle = driver.open_session_lifecycle(
                prepared.plan().clone(),
                prepared.request().clone(),
                services.clone(),
            );
            match lifecycle.await {
                Ok((session, observation)) => {
                    let contribution = match observed_contribution(
                        &prepared,
                        prepared_source_id,
                        active_session_source_id,
                        &observation,
                        session.negotiated_model_options().is_some(),
                    ) {
                        Ok(contribution) => contribution,
                        Err(ObservedFailure::Admission(admission)) => {
                            let _ = session.close(cleanup_request(), services).await;
                            return Err(KimiProjectionOpenFailure::Runtime(admission.runtime()));
                        }
                        Err(ObservedFailure::Projection(failure)) => {
                            let _ = session.close(cleanup_request(), services).await;
                            return Err(KimiProjectionOpenFailure::Runtime(RuntimeFailure::new(
                                failure.diagnostic().clone(),
                            )));
                        }
                    };
                    Ok(KimiProjectionOpenOutcome {
                        session,
                        contribution,
                    })
                }
                Err(rejection) => rejected_failure(
                    &prepared,
                    prepared_source_id,
                    active_session_source_id,
                    rejection,
                ),
            }
        })
    }
}

#[allow(clippy::result_large_err)]
fn rejected_failure(
    prepared: &KimiPreparedSession,
    prepared_source: ConsumerRouteProjectionSourceId,
    active_source: ConsumerRouteProjectionSourceId,
    rejection: KimiOpenRejection,
) -> Result<KimiProjectionOpenOutcome, KimiProjectionOpenFailure> {
    let accepted_code = matches!(
        rejection.failure().diagnostic().code(),
        "swallowtail.negotiated_reasoning.effective_mismatch"
            | "swallowtail.kimi.acp.harness_mode_mismatch"
    );
    let Some(behavior) = rejection.behavior else {
        return Err(KimiProjectionOpenFailure::Runtime(rejection.into_failure()));
    };
    if !accepted_code {
        return Err(KimiProjectionOpenFailure::Runtime(rejection.into_failure()));
    }
    let observation = KimiOpenObservation {
        behavior,
        reasoning: rejection.reasoning.clone(),
        plan: rejection.plan.clone(),
    };
    let contribution = match observed_contribution(
        prepared,
        prepared_source,
        active_source,
        &observation,
        false,
    ) {
        Ok(contribution) => contribution,
        Err(_) => return Err(KimiProjectionOpenFailure::Runtime(rejection.into_failure())),
    };
    Err(KimiProjectionOpenFailure::Rejected {
        failure: rejection.into_failure(),
        contribution,
    })
}

enum ObservedFailure {
    Admission(AdmissionFailure),
    Projection(swallowtail_runtime::ConsumerRouteProjectionFailure),
}

fn observed_contribution(
    prepared: &KimiPreparedSession,
    prepared_source: ConsumerRouteProjectionSourceId,
    active_source: ConsumerRouteProjectionSourceId,
    observation: &KimiOpenObservation,
    has_model: bool,
) -> Result<ConsumerRouteProjectionContribution, ObservedFailure> {
    let mut projection = Projection::observed(prepared.plan(), prepared_source, active_source)
        .capabilities()
        .model_selection()
        .reasoning_control(
            prepared
                .request()
                .options()
                .reasoning_mode()
                .map(|mode| mode.as_str()),
            true,
        )
        .session_options("portable reasoning and Plan session options");
    if prepared.request().options().reasoning_mode().is_none()
        && prepared.request().options().harness_mode().is_none()
    {
        projection = projection
            .portable_control(
                swallowtail_runtime::ConsumerRouteControlId::LoadSession,
                "exact resume binding",
            )
            .portable_control(
                swallowtail_runtime::ConsumerRouteControlId::ResumeSession,
                "exact resume binding",
            );
    }
    if !matches!(observation.reasoning, KimiAcknowledgement::Absent)
        || !matches!(observation.plan, KimiAcknowledgement::Absent)
    {
        let (acknowledgement, state) =
            compound_acknowledgement(observation).map_err(ObservedFailure::Admission)?;
        let feature = ConsumerRouteFeatureId::ActiveSessionReasoningAcknowledgement;
        let source = projection
            .active_source()
            .expect("observed projection has active source")
            .clone();
        let row = ConsumerRouteProjectionRow::new(
            ConsumerRouteRowIdentity::Feature(feature),
            ConsumerRouteApplicability::from_plan(prepared.plan()),
            source.clone(),
            ConsumerRouteSourceClass::RouteAcknowledgementEvidence,
            ConsumerRouteEvidenceStrength::WireAcknowledgement,
            ConsumerRouteLifecycle::PostOpenObservationOnly,
        )
        .with_support(swallowtail_runtime::ConsumerRouteSupportPosture::Supported)
        .with_availability(swallowtail_runtime::ConsumerRouteAvailability::Available)
        .with_actor_posture(ConsumerRouteActorPosture::ObservationOnly)
        .with_mutation_authority(ConsumerRouteMutationAuthority::Acknowledged(
            source.id().clone(),
        ))
        .with_state_support(state)
        .with_compound_acknowledgement(acknowledgement);
        projection = projection.active_row(row);
    }
    if has_model {
        let feature = ConsumerRouteNamespacedExtension::new(
            "kimi-code.acp",
            prepared.plan().protocol_facade_id().as_str(),
            "feature.negotiated-model-options-observation",
        )
        .map(ConsumerRouteFeatureId::Namespaced)
        .map_err(ObservedFailure::Projection)?;
        let domain = ConsumerRouteEnumerableValue::new(
            "exact bounded negotiated model options on the open session",
        )
        .map(ConsumerRouteValueDomain::Unenumerated)
        .map_err(ObservedFailure::Projection)?;
        let value = ConsumerRouteControlValue::new(
            ConsumerRouteValueKind::Observation,
            domain,
            ConsumerRouteOmissionSemantics::NotSelectable,
        );
        let row = projection
            .active_feature_row(
                feature,
                ConsumerRouteStateSupport::descriptor_only().with_observed(),
                value,
            )
            .expect("observed projection has active source");
        projection = projection.active_row(row);
    }
    projection.build().map_err(ObservedFailure::Projection)
}

fn cleanup_request() -> SessionCleanupRequest {
    SessionCleanupRequest::new(Deadline::at(MonotonicInstant::from_ticks(u64::MAX)))
}
