//! Contract 061 projection of this route's registered-tool mediation.
//!
//! The rows are descriptive. They authorize no dispatch, open no lease, and
//! claim no support. The route-local mediation kind is published as its own
//! bounded namespaced row so a consumer reads
//! `route-local-acp-client-mcp-courier` rather than inferring common dispatch
//! from the portable carrier enum: Grok exposes no host-dispatch callback, so
//! no such claim exists to make.
//!
//! The registered capability itself is published `Unqualified`. The typed
//! mapping and the provider-free callable proof are present, and the exact
//! route was observed admitting a client-supplied ACP MCP server, but the
//! disposable real-route gate — one real Grok turn calling a real registered
//! tool through a real courier — is separately authorized and has not run.
//! Missing evidence stays a blocked capability rather than an inferred one.

use super::carrier::{GROK_ACP_REGISTERED_TOOL_MEDIATION, GrokRegisteredToolCarrier};
use super::version::GROK_ACP_REGISTERED_TOOL_MCP_PROTOCOL_VERSION;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteApplicability, ConsumerRouteAvailability,
    ConsumerRouteAvailabilityDimension, ConsumerRouteControlValue, ConsumerRouteEnumerableValue,
    ConsumerRouteEnumeratedValues, ConsumerRouteEvidenceStrength, ConsumerRouteLifecycle,
    ConsumerRouteMutationAuthority, ConsumerRouteOmissionSemantics,
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure,
    ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceId,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind,
    ConsumerRouteRowIdentity, ConsumerRouteSafeReason, ConsumerRouteSourceClass,
    ConsumerRouteStateSupport, ConsumerRouteSupportPosture, ConsumerRouteValueDomain,
    ConsumerRouteValueKind, RegisteredCapabilityProjectionInput, RegisteredToolReadiness,
    RegisteredToolRouteQualification, registered_capability_feature_id,
};

/// Bounded source id of this route's registered-tool contribution.
pub const GROK_ACP_REGISTERED_TOOL_SOURCE: &str = "grok-build.acp.registered-tool-mediation";

/// Bounded semantic id of the route-local mediation-kind row.
pub const GROK_ACP_MEDIATION_KIND_SEMANTIC_ID: &str = "registered-tool.mediation-kind";

/// Safe reason published while the disposable real-route gate has not run.
pub const GROK_ACP_REAL_ROUTE_GATE_PENDING_CODE: &str =
    "swallowtail.grok.acp.registered_tool.real_route_gate_pending";

/// Returns this route's registered-tool qualification.
///
/// It stays [`RegisteredToolRouteQualification::Unqualified`] until the
/// separately authorized disposable real-route gate passes. A callable seam,
/// provider-free fixtures, and an admitted client-supplied MCP declaration do
/// not qualify a route.
#[must_use]
pub const fn grok_build_acp_registered_tool_qualification() -> RegisteredToolRouteQualification {
    RegisteredToolRouteQualification::Unqualified
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
        ConsumerRouteEvidenceStrength::RuntimeType,
        ConsumerRouteLifecycle::SelectionSummary,
    )
    .with_support(ConsumerRouteSupportPosture::Unknown)
    .with_availability(ConsumerRouteAvailability::Unavailable)
    .with_actor_posture(ConsumerRouteActorPosture::Informational)
    .with_state_support(ConsumerRouteStateSupport::descriptor_only())
    .with_mutation_authority(ConsumerRouteMutationAuthority::Absent)
    .with_control_value(ConsumerRouteControlValue::new(
        ConsumerRouteValueKind::BoundedEnumeration,
        ConsumerRouteValueDomain::Enumerated(values),
        ConsumerRouteOmissionSemantics::NotSelectable,
    ))
    .with_safe_reason(ConsumerRouteSafeReason::new(
        ConsumerRouteAvailabilityDimension::SupportAuthority,
        source.id().clone(),
        SafeDiagnostic::new(
            GROK_ACP_REAL_ROUTE_GATE_PENDING_CODE,
            "callable seam present; live gate pending",
        ),
    )?))
}
