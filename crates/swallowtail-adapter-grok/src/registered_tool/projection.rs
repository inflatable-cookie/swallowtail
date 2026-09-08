//! Contract 061 projection of this route's registered-tool mediation.
//!
//! The rows are descriptive. They authorize no dispatch, open no lease, and
//! add no runtime work. The route-local mediation kind is published as its own
//! bounded namespaced row so a consumer reads
//! `route-local-acp-client-mcp-courier` rather than inferring common dispatch
//! from the portable carrier enum: Grok exposes no host-dispatch callback, so
//! no such claim exists to make.
//!
//! The registered capability itself is published
//! [`RegisteredToolRouteQualification::Qualified`] on the strength of the
//! accepted Card 128 live gate: exact Grok Build `1.0.4` and `1.0.5` each
//! admitted the Swallowtail-owned mediated-stdio courier, listed its tools,
//! and completed one registered call through it (Research 295). The qualified
//! dimensions are exactly what those capsules proved — the provider cannot
//! represent a consumer Deny, the route delivers no consumer tool progress,
//! and the capsules carried no selected skill bundle. Versions outside the
//! maintained `1.0.4..=1.0.5` executable segments never reach this projection:
//! the carrier claim is admissible only alongside the already qualified
//! `grok-build.executable` claim.

use super::carrier::{GROK_ACP_REGISTERED_TOOL_MEDIATION, GrokRegisteredToolCarrier};
use super::version::GROK_ACP_REGISTERED_TOOL_MCP_PROTOCOL_VERSION;
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteApplicability, ConsumerRouteAvailability,
    ConsumerRouteControlValue, ConsumerRouteEnumerableValue, ConsumerRouteEnumeratedValues,
    ConsumerRouteEvidenceStrength, ConsumerRouteLifecycle, ConsumerRouteMutationAuthority,
    ConsumerRouteOmissionSemantics, ConsumerRouteProjectionContribution,
    ConsumerRouteProjectionFailure, ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceId,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind,
    ConsumerRouteRowIdentity, ConsumerRouteSourceClass, ConsumerRouteStateSupport,
    ConsumerRouteSupportPosture, ConsumerRouteValueDomain, ConsumerRouteValueKind,
    RegisteredCapabilityProjectionInput, RegisteredToolPermissionStrength,
    RegisteredToolProgressMode, RegisteredToolQualifiedRoute, RegisteredToolReadiness,
    RegisteredToolRouteQualification, RegisteredToolSkillDelivery,
    registered_capability_feature_id,
};

/// Bounded source id of this route's registered-tool contribution.
pub const GROK_ACP_REGISTERED_TOOL_SOURCE: &str = "grok-build.acp.registered-tool-mediation";

/// Bounded semantic id of the route-local mediation-kind row.
pub const GROK_ACP_MEDIATION_KIND_SEMANTIC_ID: &str = "registered-tool.mediation-kind";

/// The exact registered-capability dimensions the accepted live gate proved.
///
/// Research 295 freezes the evidence: on exact Grok Build `1.0.4` and `1.0.5`
/// the courier was admitted, its tools were listed, one registered call
/// completed, and the turn reached `end_turn`. The provider-owned one-shot
/// permission exchange is a separate channel, so a consumer Deny is
/// `NotRepresented`; no consumer tool progress was delivered (`NoProgress`);
/// and no capsule carried a selected skill, whose ACP v1 surface does not
/// exist (`NotCarried`).
pub const GROK_ACP_REGISTERED_TOOL_ROUTE: RegisteredToolQualifiedRoute =
    RegisteredToolQualifiedRoute::new(
        RegisteredToolPermissionStrength::NotRepresented,
        RegisteredToolProgressMode::NoProgress,
        RegisteredToolSkillDelivery::NotCarried,
    );

/// Returns this route's registered-tool qualification.
///
/// [`RegisteredToolRouteQualification::Qualified`] rests only on the accepted
/// live capsules for exact Grok Build `1.0.4` and `1.0.5`; a callable seam,
/// provider-free fixtures, and an admitted client-supplied MCP declaration
/// never qualified a route on their own.
#[must_use]
pub const fn grok_build_acp_registered_tool_qualification() -> RegisteredToolRouteQualification {
    RegisteredToolRouteQualification::Qualified(GROK_ACP_REGISTERED_TOOL_ROUTE)
}

/// Projects the route-local registered-tool mediation as a Contract 061
/// contribution.
///
/// Readiness must have been evaluated for exactly this carrier's own
/// selection; mixed evidence rejects the whole contribution rather than
/// composing into an available row.
pub fn project_grok_build_acp_registered_tool(
    applicability: &ConsumerRouteApplicability,
    carrier: &GrokRegisteredToolCarrier,
    readiness: &RegisteredToolReadiness,
) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
    let source = ConsumerRouteProjectionSourceIdentity::new(
        ConsumerRouteProjectionSourceId::new(GROK_ACP_REGISTERED_TOOL_SOURCE)?,
        ConsumerRouteProjectionSourceKind::AdapterContribution,
    );
    let capability = swallowtail_runtime::project_registered_capability(
        RegisteredCapabilityProjectionInput::new(
            applicability.clone(),
            source.clone(),
            carrier.selection(),
            readiness,
        )
        .with_route_qualification(grok_build_acp_registered_tool_qualification()),
    )?;
    let mediation = mediation_kind_row(applicability, carrier, &source)?;
    ConsumerRouteProjectionContribution::new(
        applicability.clone(),
        capability.sources().cloned(),
        capability
            .selection_rows()
            .cloned()
            .chain(std::iter::once(mediation)),
        capability.session_start_rows().cloned(),
        capability.active_session_rows().cloned(),
    )
}

/// Publishes the exact route-local mediation kind and its carrier identities.
///
/// The mediation is the mechanism the qualified route itself uses: the
/// accepted live capsules ran one registered call through this exact courier
/// shape, so the row publishes route-validation support instead of the
/// unknown posture it carried while the gate was pending.
fn mediation_kind_row(
    applicability: &ConsumerRouteApplicability,
    carrier: &GrokRegisteredToolCarrier,
    source: &ConsumerRouteProjectionSourceIdentity,
) -> Result<ConsumerRouteProjectionRow, ConsumerRouteProjectionFailure> {
    let identity = ConsumerRouteRowIdentity::Feature(registered_capability_feature_id(
        applicability.protocol_facade_id().as_str(),
        carrier.selection().protocol_version(),
        GROK_ACP_MEDIATION_KIND_SEMANTIC_ID,
    )?);
    let values = ConsumerRouteEnumeratedValues::new([
        ConsumerRouteEnumerableValue::new(GROK_ACP_REGISTERED_TOOL_MEDIATION)?,
        ConsumerRouteEnumerableValue::new(carrier.server_name())?,
        ConsumerRouteEnumerableValue::new(format!(
            "mcp-protocol={GROK_ACP_REGISTERED_TOOL_MCP_PROTOCOL_VERSION}"
        ))?,
    ])?;
    Ok(ConsumerRouteProjectionRow::new(
        identity,
        applicability.clone(),
        source.clone(),
        ConsumerRouteSourceClass::AdapterPreparedInput,
        ConsumerRouteEvidenceStrength::RouteValidation,
        ConsumerRouteLifecycle::SelectionSummary,
    )
    .with_support(ConsumerRouteSupportPosture::Supported)
    .with_availability(ConsumerRouteAvailability::Available)
    .with_actor_posture(ConsumerRouteActorPosture::Informational)
    .with_state_support(ConsumerRouteStateSupport::descriptor_only())
    .with_mutation_authority(ConsumerRouteMutationAuthority::Absent)
    .with_control_value(ConsumerRouteControlValue::new(
        ConsumerRouteValueKind::BoundedEnumeration,
        ConsumerRouteValueDomain::Enumerated(values),
        ConsumerRouteOmissionSemantics::NotSelectable,
    )))
}
