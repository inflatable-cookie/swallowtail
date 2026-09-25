//! Contract 061 contribution from the prepared Goose ACP session.

use crate::GoosePreparedSession;
use crate::mcp::{
    GOOSE_ACP_HTTP_MCP_PLACEMENT, GOOSE_ACP_MCP_SERVER_NAME, GooseAcpRemoteMcpTransport,
};
use swallowtail_core::{
    AccessStatus, Capability, CredentialState, EndpointAuthorization, EntitlementState,
    PreflightPlan, RuntimeReadiness,
};
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteApplicability, ConsumerRouteAvailability,
    ConsumerRouteControlValue, ConsumerRouteEnumerableValue, ConsumerRouteEnumeratedValues,
    ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId, ConsumerRouteLifecycle,
    ConsumerRouteMutationAuthority, ConsumerRouteNamespacedExtension,
    ConsumerRouteOmissionSemantics, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailure, ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceId,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind,
    ConsumerRouteRowIdentity, ConsumerRouteSourceClass, ConsumerRouteStateSupport,
    ConsumerRouteSupportPosture, ConsumerRouteValueDomain, ConsumerRouteValueKind,
};

const MCP_PLACEMENT_SEMANTIC_ID: &str = "mcp.placement";
const MCP_PLACEMENT_VERSION: &str = "acp-v1";
const ROUTE_ID: &str = "goose.acp";

impl GoosePreparedSession {
    /// Emits only the interactive-session truth this prepared session proves.
    pub fn consumer_route_projection_contribution(
        &self,
        source_id: ConsumerRouteProjectionSourceId,
    ) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        SessionProjection::new(self.plan(), source_id)
            .with_prepared_capabilities()
            .with_mcp_placement(self)?
            .build()
    }
}

struct SessionProjection<'a> {
    plan: &'a PreflightPlan,
    applicability: ConsumerRouteApplicability,
    source: ConsumerRouteProjectionSourceIdentity,
    availability: ConsumerRouteAvailability,
    selection: Vec<ConsumerRouteProjectionRow>,
    active_session: Vec<ConsumerRouteProjectionRow>,
}

impl<'a> SessionProjection<'a> {
    fn new(plan: &'a PreflightPlan, source_id: ConsumerRouteProjectionSourceId) -> Self {
        Self {
            plan,
            applicability: ConsumerRouteApplicability::from_plan(plan),
            source: ConsumerRouteProjectionSourceIdentity::new(
                source_id,
                ConsumerRouteProjectionSourceKind::AdapterContribution,
            ),
            availability: availability(plan.access_status()),
            selection: Vec::new(),
            active_session: Vec::new(),
        }
    }

    fn row(
        &self,
        identity: ConsumerRouteRowIdentity,
        source_class: ConsumerRouteSourceClass,
        lifecycle: ConsumerRouteLifecycle,
    ) -> ConsumerRouteProjectionRow {
        ConsumerRouteProjectionRow::new(
            identity,
            self.applicability.clone(),
            self.source.clone(),
            source_class,
            ConsumerRouteEvidenceStrength::PreparedOperation,
            lifecycle,
        )
        .with_support(ConsumerRouteSupportPosture::Supported)
        .with_availability(self.availability)
    }

    fn with_prepared_capabilities(mut self) -> Self {
        self.selection.push(
            self.row(
                ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::PreparedFacade),
                ConsumerRouteSourceClass::PreparedOperationRecord,
                ConsumerRouteLifecycle::SelectionSummary,
            )
            .with_actor_posture(ConsumerRouteActorPosture::Informational),
        );
        for requirement in self.plan.requirements().capabilities() {
            let Some(feature) = feature_for(requirement.capability()) else {
                continue;
            };
            if feature == ConsumerRouteFeatureId::ActivityObservation {
                self.active_session.push(
                    self.row(
                        ConsumerRouteRowIdentity::Feature(feature),
                        ConsumerRouteSourceClass::PreparedOperationRecord,
                        ConsumerRouteLifecycle::PostOpenObservationOnly,
                    )
                    .with_actor_posture(ConsumerRouteActorPosture::ObservationOnly)
                    .with_state_support(ConsumerRouteStateSupport::descriptor_only()),
                );
            } else {
                self.selection.push(
                    self.row(
                        ConsumerRouteRowIdentity::Feature(feature),
                        ConsumerRouteSourceClass::CapabilityProfile,
                        ConsumerRouteLifecycle::SelectionSummary,
                    )
                    .with_actor_posture(ConsumerRouteActorPosture::Informational),
                );
            }
        }
        self
    }

    fn with_mcp_placement(
        mut self,
        session: &GoosePreparedSession,
    ) -> Result<Self, ConsumerRouteProjectionFailure> {
        let Some(remote) = session.http_mcp() else {
            return Ok(self);
        };
        if remote.transport() != GooseAcpRemoteMcpTransport::Http {
            return Ok(self);
        }
        let identity = ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::Namespaced(
            ConsumerRouteNamespacedExtension::new(
                ROUTE_ID,
                MCP_PLACEMENT_VERSION,
                MCP_PLACEMENT_SEMANTIC_ID,
            )?,
        ));
        let values = ConsumerRouteEnumeratedValues::new([
            ConsumerRouteEnumerableValue::new(GOOSE_ACP_HTTP_MCP_PLACEMENT)?,
            ConsumerRouteEnumerableValue::new(GOOSE_ACP_MCP_SERVER_NAME)?,
        ])?;
        self.selection.push(
            self.row(
                identity,
                ConsumerRouteSourceClass::AdapterPreparedInput,
                ConsumerRouteLifecycle::SelectionSummary,
            )
            .with_actor_posture(ConsumerRouteActorPosture::Informational)
            .with_mutation_authority(ConsumerRouteMutationAuthority::Absent)
            .with_control_value(ConsumerRouteControlValue::new(
                ConsumerRouteValueKind::BoundedEnumeration,
                ConsumerRouteValueDomain::Enumerated(values),
                ConsumerRouteOmissionSemantics::NotSelectable,
            )),
        );
        Ok(self)
    }

    fn build(self) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
        ConsumerRouteProjectionContribution::new(
            self.applicability,
            [self.source],
            self.selection,
            [],
            self.active_session,
        )
    }
}

const fn feature_for(capability: Capability) -> Option<ConsumerRouteFeatureId> {
    Some(match capability {
        Capability::InteractiveSession => ConsumerRouteFeatureId::InteractiveSession,
        Capability::StreamingEvents => ConsumerRouteFeatureId::StreamingEvents,
        Capability::Interruption => ConsumerRouteFeatureId::CancellationOrInterruption,
        Capability::WorkingResource => ConsumerRouteFeatureId::WorkingResource,
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
