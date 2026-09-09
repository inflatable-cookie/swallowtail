//! Contract 061 projection of this route's registered-tool mediation.
//!
//! The rows are descriptive. They authorize no dispatch, open no lease, and
//! claim no support. The route-local mediation kind is published as its own
//! bounded namespaced row so a consumer reads `route-local-stdio-mcp-mediation`
//! rather than inferring common dispatch from the portable carrier enum: the
//! pinned SDK exposes no host-dispatch callback, so no such claim exists to
//! make.
//!
//! The registered capability is qualified only for the exact accepted Card 318
//! live tuple (Research 301): SDK `0.3.259`, native `2.1.259`, Node `22.23.2`,
//! and the `0.4.4` sidecar source tag are pinned exactly by this route's
//! one-point claims, so the only axis a compiled route can vary is its
//! platform — the accepted capsules ran on Darwin arm64, and only that target
//! projects the qualified truth. The qualified dimensions are exactly what the
//! capsule proved: one exact Allow and one route-supported Deny
//! (`ExactOneShot`), no consumer tool progress (`NoProgress`), and no selected
//! skill bundle carried by the accepted capsule (`NotCarried`). Off that
//! target the projection publishes the unqualified truth and never infers the
//! live evidence.

use super::carrier::{
    CLAUDE_AGENT_SDK_REGISTERED_TOOL_MEDIATION, ClaudeAgentSdkRegisteredToolCarrier,
};
use super::version::CLAUDE_AGENT_SDK_MCP_PROTOCOL_VERSION;
use swallowtail_core::SafeDiagnostic;
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteApplicability, ConsumerRouteAvailability,
    ConsumerRouteAvailabilityDimension, ConsumerRouteEnumerableValue,
    ConsumerRouteEnumeratedValues, ConsumerRouteEvidenceStrength, ConsumerRouteLifecycle,
    ConsumerRouteMutationAuthority, ConsumerRouteOmissionSemantics,
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure,
    ConsumerRouteProjectionRow, ConsumerRouteProjectionSourceId,
    ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind,
    ConsumerRouteRowIdentity, ConsumerRouteSafeReason, ConsumerRouteSourceClass,
    ConsumerRouteStateSupport, ConsumerRouteSupportPosture, ConsumerRouteValueDomain,
    ConsumerRouteValueKind, REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION,
    RegisteredCapabilityProjectionInput, RegisteredToolPermissionStrength,
    RegisteredToolProgressMode, RegisteredToolProtocolVersion, RegisteredToolQualifiedRoute,
    RegisteredToolReadiness, RegisteredToolRouteQualification, RegisteredToolSkillDelivery,
    ResolvedSkillBundle, SELECTED_SKILL_BUNDLE_SEMANTIC_ID, registered_capability_control_id,
    registered_capability_feature_id,
};

/// Bounded source id of this route's registered-tool contribution.
pub const CLAUDE_AGENT_SDK_REGISTERED_TOOL_SOURCE: &str =
    "claude-agent.sdk.registered-tool-mediation";

/// Bounded semantic id of the route-local mediation-kind row.
pub const CLAUDE_AGENT_SDK_MEDIATION_KIND_SEMANTIC_ID: &str = "registered-tool.mediation-kind";

/// Retired safe reason from the era when the disposable real-route gate had
/// not run.
///
/// No projected row publishes this code anymore: the accepted Card 318 live
/// tuple replaced the pending disposition, and off-platform rows publish
/// [`CLAUDE_AGENT_SDK_REGISTERED_TOOL_PLATFORM_NOT_ADMITTED_CODE`]. The
/// constant stays because the immutable v0.4.3 public surface carries it; it
/// is dead vocabulary, not a live reason.
pub const CLAUDE_AGENT_SDK_REAL_ROUTE_GATE_PENDING_CODE: &str =
    "swallowtail.claude-agent.sdk.registered_tool.real_route_gate_pending";

/// Safe reason code published off the accepted live-gate platform.
pub const CLAUDE_AGENT_SDK_REGISTERED_TOOL_PLATFORM_NOT_ADMITTED_CODE: &str =
    "swallowtail.claude-agent.sdk.registered_tool.platform_not_admitted";

/// The exact registered-capability dimensions the accepted live gate proved.
///
/// Research 301 freezes the evidence: the accepted Card 318 capsules ran one
/// exact Allow that dispatched `desktop/reconcile` once with unchanged `{}` and
/// correlated its `{"ok":true}` result, one Deny that completed without any
/// dispatch, and cancellation and stale/foreign controls that dispatched zero
/// times — so the route carries one exact one-shot Allow and one
/// route-supported Deny. No consumer tool progress was delivered, and no
/// capsule carried a selected skill bundle.
pub const CLAUDE_AGENT_SDK_REGISTERED_TOOL_ROUTE: RegisteredToolQualifiedRoute =
    RegisteredToolQualifiedRoute::new(
        RegisteredToolPermissionStrength::ExactOneShot,
        RegisteredToolProgressMode::NoProgress,
        RegisteredToolSkillDelivery::NotCarried,
    );

/// Returns this route's registered-tool qualification.
///
/// [`RegisteredToolRouteQualification::Qualified`] rests only on the accepted
/// Card 318 live capsules (Research 301) for the exact tuple: SDK `0.3.259`,
/// native `2.1.259`, Node `22.23.2`, the `0.4.4` sidecar source tag, the
/// existing carrier revision, private-loopback mediated-stdio, and MCP
/// `2025-11-25`. This route pins every one of those axes exactly, so the only
/// axis a compiled route can vary is its platform; the accepted capsules ran
/// on Darwin arm64, and every other target projects
/// [`RegisteredToolRouteQualification::Unqualified`]. A callable seam,
/// provider-free fixtures, and a frozen transcript never qualified a route on
/// their own.
#[must_use]
pub const fn claude_agent_sdk_registered_tool_qualification() -> RegisteredToolRouteQualification {
    if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        RegisteredToolRouteQualification::Qualified(CLAUDE_AGENT_SDK_REGISTERED_TOOL_ROUTE)
    } else {
        RegisteredToolRouteQualification::Unqualified
    }
}

/// Projects the route-local registered-tool mediation as a Contract 061
/// contribution.
///
/// Readiness must have been evaluated for exactly the carrier's own selection;
/// mixed evidence rejects the whole contribution rather than composing into an
/// available row.
pub fn project_claude_agent_sdk_registered_tool(
    applicability: &ConsumerRouteApplicability,
    carrier: &ClaudeAgentSdkRegisteredToolCarrier,
    readiness: &RegisteredToolReadiness,
) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
    project_claude_agent_sdk_registered_tool_with_selected_skill(
        applicability,
        carrier,
        readiness,
        None,
    )
}

/// Projects the route-local registered-tool mediation and its selected-skill
/// session-start input.
///
/// On the accepted live-gate platform the registered-tool capability rows
/// carry the exact-tuple qualified route claim; off it they publish the
/// unqualified truth. The selected-skill row is a narrower provider-free
/// transport claim: this exact prepared route carries the resolved bundle,
/// while no live provider-followed claim is made.
pub fn project_claude_agent_sdk_registered_tool_with_selected_skill(
    applicability: &ConsumerRouteApplicability,
    carrier: &ClaudeAgentSdkRegisteredToolCarrier,
    readiness: &RegisteredToolReadiness,
    selected_skill: Option<&ResolvedSkillBundle>,
) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
    project_claude_agent_sdk_registered_tool_with_selected_skill_from_source(
        applicability,
        ConsumerRouteProjectionSourceId::new(CLAUDE_AGENT_SDK_REGISTERED_TOOL_SOURCE)?,
        carrier,
        readiness,
        selected_skill,
    )
}

/// Projects the same selected-skill contribution with a caller-owned source
/// identity for a prepared-session projection.
pub fn project_claude_agent_sdk_registered_tool_with_selected_skill_from_source(
    applicability: &ConsumerRouteApplicability,
    source_id: ConsumerRouteProjectionSourceId,
    carrier: &ClaudeAgentSdkRegisteredToolCarrier,
    readiness: &RegisteredToolReadiness,
    selected_skill: Option<&ResolvedSkillBundle>,
) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
    let source = ConsumerRouteProjectionSourceIdentity::new(
        source_id,
        ConsumerRouteProjectionSourceKind::AdapterContribution,
    );
    let qualification = claude_agent_sdk_registered_tool_qualification();
    let admitted = matches!(
        qualification,
        RegisteredToolRouteQualification::Qualified(_)
    );
    let mut projection_input = RegisteredCapabilityProjectionInput::new(
        applicability.clone(),
        source.clone(),
        carrier.selection(),
        readiness,
    )
    .with_route_qualification(qualification);
    if let Some(bundle) = selected_skill {
        projection_input = projection_input.with_resolved_skill_bundle(bundle);
    }
    let capability = swallowtail_runtime::project_registered_capability(projection_input)?;
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

/// Projects a selected bundle for a prepared session that did not opt into
/// registered-tool mediation. The shared registered-capability builder needs
/// a real selection to publish its carrier rows; this path publishes only the
/// direct prepared input and therefore cannot imply Card 125 qualification.
pub(crate) fn project_claude_agent_sdk_selected_skill_from_source(
    applicability: &ConsumerRouteApplicability,
    source_id: ConsumerRouteProjectionSourceId,
    bundle: &ResolvedSkillBundle,
) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
    let source = ConsumerRouteProjectionSourceIdentity::new(
        source_id,
        ConsumerRouteProjectionSourceKind::AdapterContribution,
    );
    let protocol_version =
        RegisteredToolProtocolVersion::new(REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION)
            .expect("static registered-tool protocol version is valid");
    let row = selected_skill_row(applicability, &protocol_version, &source, bundle)?;
    ConsumerRouteProjectionContribution::new(applicability.clone(), [source], [], [row], [])
}

fn selected_skill_row(
    applicability: &ConsumerRouteApplicability,
    protocol_version: &RegisteredToolProtocolVersion,
    source: &ConsumerRouteProjectionSourceIdentity,
    bundle: &ResolvedSkillBundle,
) -> Result<ConsumerRouteProjectionRow, ConsumerRouteProjectionFailure> {
    let identity = ConsumerRouteRowIdentity::Control(registered_capability_control_id(
        applicability.protocol_facade_id().as_str(),
        protocol_version,
        SELECTED_SKILL_BUNDLE_SEMANTIC_ID,
    )?);
    let values = ConsumerRouteEnumeratedValues::new([
        ConsumerRouteEnumerableValue::new(bundle.identity().id().as_str())?,
        ConsumerRouteEnumerableValue::new(bundle.identity().provenance().as_str())?,
        ConsumerRouteEnumerableValue::new(bundle.revision().as_str())?,
        ConsumerRouteEnumerableValue::new(bundle.digest().as_str())?,
        ConsumerRouteEnumerableValue::new(format!("references={}", bundle.references().len()))?,
        ConsumerRouteEnumerableValue::new(format!("bytes={}", bundle.resolved_content_bytes()))?,
    ])?;
    let row = ConsumerRouteProjectionRow::new(
        identity,
        applicability.clone(),
        source.clone(),
        ConsumerRouteSourceClass::AdapterPreparedInput,
        ConsumerRouteEvidenceStrength::RouteValidation,
        ConsumerRouteLifecycle::SessionStartOnly,
    )
    .with_support(ConsumerRouteSupportPosture::Supported)
    // This row proves only that the prepared Claude route carries the
    // bounded session-start input. It stays independent of the registered-tool
    // route qualification, so it never implies a live provider followed the
    // selected content.
    .with_availability(ConsumerRouteAvailability::Available)
    .with_actor_posture(ConsumerRouteActorPosture::ConsumerSelectable)
    .with_state_support(ConsumerRouteStateSupport::descriptor_only().with_prepared())
    .with_mutation_authority(ConsumerRouteMutationAuthority::PreparedSessionStart(
        source.id().clone(),
    ))
    .with_control_value(swallowtail_runtime::ConsumerRouteControlValue::new(
        ConsumerRouteValueKind::StructuredContent,
        ConsumerRouteValueDomain::Enumerated(values),
        ConsumerRouteOmissionSemantics::PreservesRouteBehavior,
    ));
    Ok(row)
}

/// Publishes the exact route-local mediation kind and its carrier identities.
///
/// The mediation is the mechanism the qualified route itself uses: on the
/// accepted live-gate platform the Card 318 capsules ran one registered call
/// through this exact mediated-stdio carrier shape, so the row publishes
/// route-validation support. Off that platform the row falls back to the
/// unknown posture with the exact platform reason, so a consumer never reads
/// qualified support the capsules did not cover.
fn mediation_kind_row(
    applicability: &ConsumerRouteApplicability,
    carrier: &ClaudeAgentSdkRegisteredToolCarrier,
    source: &ConsumerRouteProjectionSourceIdentity,
    admitted: bool,
) -> Result<ConsumerRouteProjectionRow, ConsumerRouteProjectionFailure> {
    let identity = ConsumerRouteRowIdentity::Feature(registered_capability_feature_id(
        applicability.protocol_facade_id().as_str(),
        carrier.selection().protocol_version(),
        CLAUDE_AGENT_SDK_MEDIATION_KIND_SEMANTIC_ID,
    )?);
    let values = ConsumerRouteEnumeratedValues::new([
        ConsumerRouteEnumerableValue::new(CLAUDE_AGENT_SDK_REGISTERED_TOOL_MEDIATION)?,
        ConsumerRouteEnumerableValue::new(carrier.server_name())?,
        ConsumerRouteEnumerableValue::new(format!(
            "mcp-protocol={CLAUDE_AGENT_SDK_MCP_PROTOCOL_VERSION}"
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
    .with_control_value(swallowtail_runtime::ConsumerRouteControlValue::new(
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
                CLAUDE_AGENT_SDK_REGISTERED_TOOL_PLATFORM_NOT_ADMITTED_CODE,
                "the accepted live gate ran only on exact Darwin arm64",
            ),
        )?))
    }
}
