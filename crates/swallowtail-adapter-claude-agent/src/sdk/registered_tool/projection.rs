//! Contract 061 projection of this route's registered-tool mediation.
//!
//! The rows are descriptive. They authorize no dispatch, open no lease, and
//! claim no support. The route-local mediation kind is published as its own
//! bounded namespaced row so a consumer reads `route-local-stdio-mcp-mediation`
//! rather than inferring common dispatch from the portable carrier enum: the
//! pinned SDK exposes no host-dispatch callback, so no such claim exists to
//! make.
//!
//! The registered capability itself is published `Unqualified`. The typed
//! mapping, the frozen MCP transcript, and the carrier/SDK/native version
//! qualification are all present, but the disposable real-route gate — one real
//! provider turn against a real carrier process — is separately authorized and
//! has not run. Missing evidence stays a blocked capability rather than an
//! inferred one.

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
    ConsumerRouteValueKind, RegisteredCapabilityProjectionInput, RegisteredToolReadiness,
    RegisteredToolRouteQualification, ResolvedSkillBundle, SELECTED_SKILL_BUNDLE_SEMANTIC_ID,
    registered_capability_control_id, registered_capability_feature_id,
};

/// Bounded source id of this route's registered-tool contribution.
pub const CLAUDE_AGENT_SDK_REGISTERED_TOOL_SOURCE: &str =
    "claude-agent.sdk.registered-tool-mediation";

/// Bounded semantic id of the route-local mediation-kind row.
pub const CLAUDE_AGENT_SDK_MEDIATION_KIND_SEMANTIC_ID: &str = "registered-tool.mediation-kind";

/// Safe reason published while the disposable real-route gate has not run.
pub const CLAUDE_AGENT_SDK_REAL_ROUTE_GATE_PENDING_CODE: &str =
    "swallowtail.claude-agent.sdk.registered_tool.real_route_gate_pending";

/// Returns this route's registered-tool qualification.
///
/// It stays [`RegisteredToolRouteQualification::Unqualified`] until the
/// separately authorized disposable real-route gate passes. Deterministic
/// fixtures, a frozen transcript, and a typed mapping do not qualify a route.
#[must_use]
pub const fn claude_agent_sdk_registered_tool_qualification() -> RegisteredToolRouteQualification {
    RegisteredToolRouteQualification::Unqualified
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
/// The registered-tool capability rows remain `Unqualified` until the
/// separately authorized real-route gate passes. The selected-skill row is a
/// narrower provider-free transport claim: this exact prepared route carries
/// the resolved bundle, while no live provider-followed claim is made.
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
    let capability = swallowtail_runtime::project_registered_capability(
        RegisteredCapabilityProjectionInput::new(
            applicability.clone(),
            source.clone(),
            carrier.selection(),
            readiness,
        )
        .with_route_qualification(claude_agent_sdk_registered_tool_qualification()),
    )?;
    let mediation = mediation_kind_row(applicability, carrier, &source)?;
    let session_start_rows = capability.session_start_rows().filter(|row| {
        !row.identity()
            .namespaced_extension()
            .is_some_and(|extension| extension.semantic_id() == SELECTED_SKILL_BUNDLE_SEMANTIC_ID)
    });
    let selected_skill_row = selected_skill
        .map(|bundle| selected_skill_row(applicability, carrier, readiness, &source, bundle))
        .transpose()?;
    ConsumerRouteProjectionContribution::new(
        applicability.clone(),
        capability.sources().cloned(),
        capability
            .selection_rows()
            .cloned()
            .chain(std::iter::once(mediation)),
        session_start_rows.cloned().chain(selected_skill_row),
        capability.active_session_rows().cloned(),
    )
}

fn selected_skill_row(
    applicability: &ConsumerRouteApplicability,
    carrier: &ClaudeAgentSdkRegisteredToolCarrier,
    readiness: &RegisteredToolReadiness,
    source: &ConsumerRouteProjectionSourceIdentity,
    bundle: &ResolvedSkillBundle,
) -> Result<ConsumerRouteProjectionRow, ConsumerRouteProjectionFailure> {
    let identity = ConsumerRouteRowIdentity::Control(registered_capability_control_id(
        applicability.protocol_facade_id().as_str(),
        carrier.selection().protocol_version(),
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
    // bounded session-start input. The registered-tool capability rows above
    // remain unavailable until the independent real-route gate qualifies the
    // mediated carrier.
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
    let _ = readiness;
    Ok(row)
}

/// Publishes the exact route-local mediation kind and its carrier identities.
fn mediation_kind_row(
    applicability: &ConsumerRouteApplicability,
    carrier: &ClaudeAgentSdkRegisteredToolCarrier,
    source: &ConsumerRouteProjectionSourceIdentity,
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
    .with_control_value(swallowtail_runtime::ConsumerRouteControlValue::new(
        ConsumerRouteValueKind::BoundedEnumeration,
        ConsumerRouteValueDomain::Enumerated(values),
        ConsumerRouteOmissionSemantics::NotSelectable,
    ))
    .with_safe_reason(ConsumerRouteSafeReason::new(
        ConsumerRouteAvailabilityDimension::SupportAuthority,
        source.id().clone(),
        SafeDiagnostic::new(
            CLAUDE_AGENT_SDK_REAL_ROUTE_GATE_PENDING_CODE,
            "callable seam present; live gate pending",
        ),
    )?))
}
