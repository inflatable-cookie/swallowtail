//! Contract 061 projection of this route's registered-tool mediation.
//!
//! The rows are descriptive. They authorize no dispatch, open no lease, and
//! add no runtime work. The route-local mediation kind is published as its own
//! bounded namespaced row so a consumer reads
//! `route-local-acp-client-mcp-courier` rather than inferring common dispatch
//! from the portable carrier enum: Grok exposes no host-dispatch callback, so
//! no such claim exists to make.
//!
//! The registered capability is qualified only for the exact executable
//! versions the accepted Card 128 live gate ran: Grok Build `1.0.4` and
//! `1.0.5` each admitted the Swallowtail-owned mediated-stdio courier, listed
//! its tools, and completed one registered call through it (Research 295).
//! The qualified dimensions are exactly what those capsules proved — the
//! provider cannot represent a consumer Deny, the route delivers no consumer
//! tool progress, and the capsules carried no selected skill bundle. A plan
//! bound to any other `grok-build.executable` version — deprecated `0.2.x`,
//! the unprobed gap, or an unverified-newer point — projects the unqualified
//! truth and refuses a registered open, so every published row stays scoped
//! to the executable version it was asked about.

use super::carrier::{GROK_ACP_REGISTERED_TOOL_MEDIATION, GrokRegisteredToolCarrier};
use super::version::GROK_ACP_REGISTERED_TOOL_MCP_PROTOCOL_VERSION;
use crate::selection::grok_build_acp_claim;
use swallowtail_core::{
    InterfaceCompatibilityAssessment, InterfaceSupportStatus, InterfaceVersion, SafeDiagnostic,
};
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
    ConsumerRouteValueKind, RegisteredCapabilityProjectionInput, RegisteredToolPermissionStrength,
    RegisteredToolProgressMode, RegisteredToolQualifiedRoute, RegisteredToolReadiness,
    RegisteredToolRouteQualification, RegisteredToolSkillDelivery,
    registered_capability_feature_id,
};

/// Bounded source id of this route's registered-tool contribution.
pub const GROK_ACP_REGISTERED_TOOL_SOURCE: &str = "grok-build.acp.registered-tool-mediation";

/// Bounded semantic id of the route-local mediation-kind row.
pub const GROK_ACP_MEDIATION_KIND_SEMANTIC_ID: &str = "registered-tool.mediation-kind";

/// Safe reason code published when an executable version carries no accepted
/// registered-tool evidence.
pub const GROK_ACP_REGISTERED_TOOL_VERSION_NOT_ADMITTED_CODE: &str =
    "swallowtail.grok.acp.registered_tool.version_not_admitted";

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

/// Returns this route's registered-tool qualification for one executable
/// version.
///
/// [`RegisteredToolRouteQualification::Qualified`] rests only on the accepted
/// live capsules for exact Grok Build `1.0.4` and `1.0.5`: the maintained
/// segment of the `grok-build.executable` claim. Deprecated `0.2.x` segments,
/// the unprobed gap, and unverified-newer points return
/// [`RegisteredToolRouteQualification::Unqualified`]; a callable seam,
/// provider-free fixtures, and an admitted client-supplied MCP declaration
/// never qualified a route on their own.
#[must_use]
pub fn grok_build_acp_registered_tool_qualification(
    version: &InterfaceVersion,
) -> RegisteredToolRouteQualification {
    match grok_build_acp_claim().assess(version) {
        InterfaceCompatibilityAssessment::Qualified(matched)
            if matched.support_status() == InterfaceSupportStatus::Maintained =>
        {
            RegisteredToolRouteQualification::Qualified(GROK_ACP_REGISTERED_TOOL_ROUTE)
        }
        _ => RegisteredToolRouteQualification::Unqualified,
    }
}

/// Projects the route-local registered-tool mediation as a Contract 061
/// contribution for the plan's exact executable version.
///
/// Readiness must have been evaluated for exactly this carrier's own
/// selection; mixed evidence rejects the whole contribution rather than
/// composing into an available row. A version outside the accepted live
/// segments projects the unqualified truth instead of the qualified rows.
pub fn project_grok_build_acp_registered_tool(
    applicability: &ConsumerRouteApplicability,
    carrier: &GrokRegisteredToolCarrier,
    readiness: &RegisteredToolReadiness,
    version: &InterfaceVersion,
) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
    let qualification = grok_build_acp_registered_tool_qualification(version);
    let admitted = matches!(
        qualification,
        RegisteredToolRouteQualification::Qualified(_)
    );
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
        .with_route_qualification(qualification),
    )?;
    let mediation = mediation_kind_row(applicability, carrier, &source, admitted)?;
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
/// The mediation is the mechanism the qualified route itself uses: on an
/// admitted version the accepted live capsules ran one registered call through
/// this exact courier shape, so the row publishes route-validation support.
/// On any other executable version the row falls back to the unknown posture
/// with the exact version reason, so a consumer never reads qualified support
/// the capsules did not cover.
fn mediation_kind_row(
    applicability: &ConsumerRouteApplicability,
    carrier: &GrokRegisteredToolCarrier,
    source: &ConsumerRouteProjectionSourceIdentity,
    admitted: bool,
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
    let row = ConsumerRouteProjectionRow::new(
        identity,
        applicability.clone(),
        source.clone(),
        ConsumerRouteSourceClass::AdapterPreparedInput,
        if admitted {
            ConsumerRouteEvidenceStrength::RouteValidation
        } else {
            ConsumerRouteEvidenceStrength::RuntimeType
        },
        ConsumerRouteLifecycle::SelectionSummary,
    )
    .with_support(if admitted {
        ConsumerRouteSupportPosture::Supported
    } else {
        ConsumerRouteSupportPosture::Unknown
    })
    .with_availability(if admitted {
        ConsumerRouteAvailability::Available
    } else {
        ConsumerRouteAvailability::Unavailable
    })
    .with_actor_posture(ConsumerRouteActorPosture::Informational)
    .with_state_support(ConsumerRouteStateSupport::descriptor_only())
    .with_mutation_authority(ConsumerRouteMutationAuthority::Absent)
    .with_control_value(ConsumerRouteControlValue::new(
        ConsumerRouteValueKind::BoundedEnumeration,
        ConsumerRouteValueDomain::Enumerated(values),
        ConsumerRouteOmissionSemantics::NotSelectable,
    ));
    if admitted {
        Ok(row)
    } else {
        Ok(row.with_safe_reason(ConsumerRouteSafeReason::new(
            ConsumerRouteAvailabilityDimension::SupportAuthority,
            source.id().clone(),
            SafeDiagnostic::new(
                GROK_ACP_REGISTERED_TOOL_VERSION_NOT_ADMITTED_CODE,
                "the accepted live gate ran only on exact maintained Grok Build 1.0.4..=1.0.5",
            ),
        )?))
    }
}
