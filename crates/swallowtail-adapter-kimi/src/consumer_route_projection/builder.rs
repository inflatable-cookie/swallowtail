#[path = "builder/active.rs"]
mod active;
#[path = "builder/capability.rs"]
mod capability;
#[path = "builder/control.rs"]
mod control;

use capability::availability;

use swallowtail_core::{InstanceOwnership, PreflightPlan};
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteApplicability, ConsumerRouteAvailability,
    ConsumerRouteControlId, ConsumerRouteControlValue, ConsumerRouteEvidenceStrength,
    ConsumerRouteFeatureId, ConsumerRouteLifecycle, ConsumerRouteMutationAuthority,
    ConsumerRouteNamespacedExtension, ConsumerRouteOmissionSemantics,
    ConsumerRouteProjectionFailure, ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceId,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind,
    ConsumerRouteRowIdentity, ConsumerRouteSourceClass, ConsumerRouteStateSupport,
    ConsumerRouteSupportPosture, ConsumerRouteValueDomain, ConsumerRouteValueKind,
};

#[derive(Clone, Copy)]
pub(super) enum Route {
    Acp,
    Headless,
    Local,
}

impl Route {
    pub(super) const fn id(self) -> &'static str {
        match self {
            Self::Acp => "kimi-code.acp",
            Self::Headless => "kimi-code.headless",
            Self::Local => "kimi-code.local-server",
        }
    }
}

pub(super) struct Projection<'a> {
    plan: &'a PreflightPlan,
    route: Route,
    applicability: ConsumerRouteApplicability,
    prepared_source: ConsumerRouteProjectionSourceIdentity,
    active_source: Option<ConsumerRouteProjectionSourceIdentity>,
    availability: ConsumerRouteAvailability,
    selection: Vec<ConsumerRouteProjectionRow>,
    session_start: Vec<ConsumerRouteProjectionRow>,
    active: Vec<ConsumerRouteProjectionRow>,
    rejected: Option<ConsumerRouteProjectionFailure>,
}

impl<'a> Projection<'a> {
    pub(super) fn prepared(
        plan: &'a PreflightPlan,
        route: Route,
        source: ConsumerRouteProjectionSourceId,
    ) -> Self {
        Self::new(plan, route, source, None)
    }

    pub(super) fn observed(
        plan: &'a PreflightPlan,
        prepared: ConsumerRouteProjectionSourceId,
        active: ConsumerRouteProjectionSourceId,
    ) -> Self {
        Self::new(plan, Route::Acp, prepared, Some(active))
    }

    fn new(
        plan: &'a PreflightPlan,
        route: Route,
        prepared: ConsumerRouteProjectionSourceId,
        active: Option<ConsumerRouteProjectionSourceId>,
    ) -> Self {
        Self {
            plan,
            route,
            applicability: ConsumerRouteApplicability::from_plan(plan),
            prepared_source: ConsumerRouteProjectionSourceIdentity::new(
                prepared,
                ConsumerRouteProjectionSourceKind::AdapterContribution,
            ),
            active_source: active.map(|source| {
                ConsumerRouteProjectionSourceIdentity::new(
                    source,
                    ConsumerRouteProjectionSourceKind::ActiveSessionObservation,
                )
            }),
            availability: availability(plan.access_status()),
            selection: Vec::new(),
            session_start: Vec::new(),
            active: Vec::new(),
            rejected: None,
        }
    }

    pub(super) fn capabilities(mut self) -> Self {
        self.selection
            .push(self.prepared_feature(ConsumerRouteFeatureId::PreparedFacade));
        for requirement in self.plan.requirements().capabilities() {
            let Some(feature) = self.feature_for(requirement.capability()) else {
                continue;
            };
            let row = self.row(
                ConsumerRouteRowIdentity::Feature(feature.clone()),
                &self.prepared_source,
                ConsumerRouteSourceClass::CapabilityProfile,
                ConsumerRouteEvidenceStrength::PreparedOperation,
                if feature == ConsumerRouteFeatureId::ActivityObservation {
                    ConsumerRouteLifecycle::PostOpenObservationOnly
                } else {
                    ConsumerRouteLifecycle::SelectionSummary
                },
            );
            if feature == ConsumerRouteFeatureId::ActivityObservation {
                self.active
                    .push(row.with_actor_posture(ConsumerRouteActorPosture::ObservationOnly));
            } else {
                self.selection.push(row);
            }
        }
        if matches!(self.route, Route::Local)
            && self.plan.ownership() == InstanceOwnership::HostOwnedEphemeral
            && let Some(feature) = self.local_feature("feature.owned-runtime-lifecycle")
        {
            let row = self.prepared_feature(feature);
            self.selection.push(row);
        }
        self
    }

    pub(super) fn active_row(mut self, row: ConsumerRouteProjectionRow) -> Self {
        self.active.push(row);
        self
    }

    pub(super) fn active_source(&self) -> Option<&ConsumerRouteProjectionSourceIdentity> {
        self.active_source.as_ref()
    }

    fn prepared_feature(&self, feature: ConsumerRouteFeatureId) -> ConsumerRouteProjectionRow {
        self.row(
            ConsumerRouteRowIdentity::Feature(feature),
            &self.prepared_source,
            ConsumerRouteSourceClass::PreparedOperationRecord,
            ConsumerRouteEvidenceStrength::PreparedOperation,
            ConsumerRouteLifecycle::SelectionSummary,
        )
    }

    fn control_row(
        &self,
        control: ConsumerRouteControlId,
        kind: ConsumerRouteValueKind,
        domain: ConsumerRouteValueDomain,
        omission: ConsumerRouteOmissionSemantics,
    ) -> ConsumerRouteProjectionRow {
        self.row(
            ConsumerRouteRowIdentity::Control(control),
            &self.prepared_source,
            ConsumerRouteSourceClass::AdapterPreparedInput,
            ConsumerRouteEvidenceStrength::RouteValidation,
            ConsumerRouteLifecycle::SessionStartOnly,
        )
        .with_actor_posture(ConsumerRouteActorPosture::ConsumerSelectable)
        .with_mutation_authority(ConsumerRouteMutationAuthority::PreparedSessionStart(
            self.prepared_source.id().clone(),
        ))
        .with_state_support(
            ConsumerRouteStateSupport::descriptor_only()
                .with_requested()
                .with_prepared(),
        )
        .with_control_value(ConsumerRouteControlValue::new(kind, domain, omission))
    }

    fn push_control(
        &mut self,
        control: ConsumerRouteControlId,
        kind: ConsumerRouteValueKind,
        domain: ConsumerRouteValueDomain,
        omission: ConsumerRouteOmissionSemantics,
    ) {
        let row = self.control_row(control, kind, domain, omission);
        self.session_start.push(row);
    }

    fn local_control(&mut self, semantic: &str) -> Option<ConsumerRouteControlId> {
        match ConsumerRouteNamespacedExtension::new(
            self.route.id(),
            self.plan.protocol_facade_id().as_str(),
            semantic,
        ) {
            Ok(extension) => Some(ConsumerRouteControlId::Namespaced(extension)),
            Err(error) => {
                self.rejected = Some(error);
                None
            }
        }
    }

    fn row(
        &self,
        identity: ConsumerRouteRowIdentity,
        source: &ConsumerRouteProjectionSourceIdentity,
        class: ConsumerRouteSourceClass,
        evidence: ConsumerRouteEvidenceStrength,
        lifecycle: ConsumerRouteLifecycle,
    ) -> ConsumerRouteProjectionRow {
        ConsumerRouteProjectionRow::new(
            identity,
            self.applicability.clone(),
            source.clone(),
            class,
            evidence,
            lifecycle,
        )
        .with_support(ConsumerRouteSupportPosture::Supported)
        .with_availability(self.availability)
    }
}
