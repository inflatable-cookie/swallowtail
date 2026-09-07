//! Contract 063 registered-capability rows for the Contract 061 projection.
//!
//! Every row here is descriptive. It authorizes no dispatch, opens no lease,
//! and adds no route support. A registered capability without a qualified
//! adapter is published as unavailable; it is never supported by inference.
//! Rows carry safe identities, kinds, transports, and limits only — never tool
//! arguments or results, prompt or skill bodies, credentials, endpoints, or
//! host path material.
//!
//! Rows use bounded namespaced identity rather than new variants in the closed
//! portable enums. Contract 063 declines to widen `HostServiceKind` in this
//! slice for the same reason: widening a closed public enum breaks every
//! consumer that matches it exhaustively, which is a compatibility decision
//! this card does not own. Each extension names the exact route facade and the
//! qualified registered-tool protocol version segment it was admitted under.

use super::applicability::ConsumerRouteApplicability;
use super::contribution::ConsumerRouteProjectionContribution;
use super::failure::{ConsumerRouteProjectionFailure, ConsumerRouteProjectionFailureKind, failure};
use super::identity::{ConsumerRouteProjectionSourceIdentity, ConsumerRouteProjectionSourceKind};
use super::row::ConsumerRouteProjectionRow;
use super::semantics::{
    ConsumerRouteActorPosture, ConsumerRouteAvailability, ConsumerRouteAvailabilityDimension,
    ConsumerRouteControlId, ConsumerRouteEvidenceStrength, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteMutationAuthority, ConsumerRouteNamespacedExtension,
    ConsumerRouteRowIdentity, ConsumerRouteSafeReason, ConsumerRouteSourceClass,
    ConsumerRouteStateSupport, ConsumerRouteSupportPosture,
};
use super::value::{
    ConsumerRouteControlValue, ConsumerRouteEnumerableValue, ConsumerRouteEnumeratedValues,
    ConsumerRouteOmissionSemantics, ConsumerRouteValueDomain, ConsumerRouteValueKind,
};
use crate::registered_tool::{
    REGISTERED_TOOL_QUALIFIED_TRANSPORTS, RegisteredToolProtocolVersion, RegisteredToolReadiness,
    RegisteredToolSelection, RegisteredToolTransport, ResolvedSkillBundle,
};
use std::collections::BTreeSet;
use swallowtail_core::SafeDiagnostic;

/// Safe reason code published when no adapter has qualified the route.
pub const ADAPTER_UNQUALIFIED_CODE: &str = "swallowtail.registered_tool.adapter_unqualified";
/// Safe reason code published when a qualified route lacks one dimension.
pub const ROUTE_DIMENSION_UNSUPPORTED_CODE: &str =
    "swallowtail.registered_tool.route_dimension_unsupported";
/// Safe reason code published for the permanently withheld scheduling row.
pub const SCHEDULING_WITHHELD_CODE: &str = "swallowtail.registered_tool.scheduling_withheld";

/// Bounded semantic id of the registered-capability registration row.
pub const REGISTERED_TOOL_CAPABILITY_SEMANTIC_ID: &str = "registered-tool.capability";
/// Bounded semantic id of the selected execution-kind row.
pub const REGISTERED_TOOL_EXECUTION_KIND_SEMANTIC_ID: &str = "registered-tool.execution-kind";
/// Bounded semantic id of the transport-profile row.
pub const REGISTERED_TOOL_TRANSPORT_SEMANTIC_ID: &str = "registered-tool.transport-profile";
/// Bounded semantic id of the one-shot permission row.
pub const REGISTERED_TOOL_PERMISSION_SEMANTIC_ID: &str = "registered-tool.one-shot-permission";
/// Bounded semantic id of the tool-progress delivery row.
pub const REGISTERED_TOOL_PROGRESS_SEMANTIC_ID: &str = "registered-tool.progress-delivery";
/// Bounded semantic id of the scheduling row, which stays withheld.
pub const REGISTERED_TOOL_SCHEDULING_SEMANTIC_ID: &str = "registered-tool.scheduling";
/// Bounded semantic id of the selected skill/reference control row.
pub const SELECTED_SKILL_BUNDLE_SEMANTIC_ID: &str = "registered-tool.selected-skill-bundle";

/// Exact one-shot permission strength an adapter proved for this route.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RegisteredToolPermissionStrength {
    /// The route carries one exact Allow and one exact route-supported Deny.
    ExactOneShot,
    /// The route cannot represent the consumer's Deny to the provider.
    NotRepresented,
}

/// Exact tool-progress delivery an adapter proved for this route.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RegisteredToolProgressMode {
    /// Bounded, correlated, per-call monotonic progress admission.
    BoundedOrdered,
    /// The route delivers no consumer tool progress.
    NoProgress,
}

/// Exact selected skill/reference delivery an adapter proved for this route.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum RegisteredToolSkillDelivery {
    /// The route carries the bounded selected bundle and its references.
    BoundedSelectedBundle,
    /// The route carries no selected skill bundle.
    NotCarried,
}

/// The dimensions one exact adapter proved for a registered capability.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RegisteredToolQualifiedRoute {
    permission: RegisteredToolPermissionStrength,
    progress: RegisteredToolProgressMode,
    skill_delivery: RegisteredToolSkillDelivery,
}

impl RegisteredToolQualifiedRoute {
    /// Records what one exact adapter route validation proved.
    #[must_use]
    pub const fn new(
        permission: RegisteredToolPermissionStrength,
        progress: RegisteredToolProgressMode,
        skill_delivery: RegisteredToolSkillDelivery,
    ) -> Self {
        Self {
            permission,
            progress,
            skill_delivery,
        }
    }

    /// Returns the proved one-shot permission strength.
    #[must_use]
    pub const fn permission(self) -> RegisteredToolPermissionStrength {
        self.permission
    }

    /// Returns the proved tool-progress mode.
    #[must_use]
    pub const fn progress(self) -> RegisteredToolProgressMode {
        self.progress
    }

    /// Returns the proved selected skill/reference delivery.
    #[must_use]
    pub const fn skill_delivery(self) -> RegisteredToolSkillDelivery {
        self.skill_delivery
    }
}

/// Whether an exact adapter has qualified this registered capability.
///
/// Only an adapter holding route evidence may publish `Qualified`. Without it
/// every projected row stays unavailable and claims no support.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum RegisteredToolRouteQualification {
    /// No adapter has proved this capability for the exact route.
    #[default]
    Unqualified,
    /// One exact adapter proved these registered-capability dimensions.
    Qualified(RegisteredToolQualifiedRoute),
}

/// What the supplied evidence proves about one row's support dimension.
///
/// The three states stay distinct because they carry different bounded reasons.
/// Collapsing "no adapter has qualified this route" into "this qualified route
/// does not support this dimension" would publish an availability claim the
/// source never made.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RowSupportEvidence {
    /// No adapter has qualified this capability; support stays unknown.
    Unqualified,
    /// A qualified adapter proved this exact dimension is not supported.
    QualifiedUnsupported,
    /// A qualified adapter proved this exact dimension is supported.
    QualifiedSupported,
}

impl RowSupportEvidence {
    /// Maps proved support onto the exact evidence state.
    const fn of(qualified: bool, supported: bool) -> Self {
        match (qualified, supported) {
            (false, _) => Self::Unqualified,
            (true, false) => Self::QualifiedUnsupported,
            (true, true) => Self::QualifiedSupported,
        }
    }

    /// Returns descriptive support, which never implies availability.
    const fn support(self) -> ConsumerRouteSupportPosture {
        match self {
            Self::Unqualified => ConsumerRouteSupportPosture::Unknown,
            Self::QualifiedUnsupported => ConsumerRouteSupportPosture::Unsupported,
            Self::QualifiedSupported => ConsumerRouteSupportPosture::Supported,
        }
    }

    /// Returns how strongly the named source proves the row.
    const fn evidence_strength(self) -> ConsumerRouteEvidenceStrength {
        match self {
            Self::Unqualified => ConsumerRouteEvidenceStrength::RuntimeType,
            Self::QualifiedUnsupported | Self::QualifiedSupported => {
                ConsumerRouteEvidenceStrength::RouteValidation
            }
        }
    }
}

/// Exact borrowed evidence one registered-capability contribution is built from.
#[derive(Clone, Debug)]
pub struct RegisteredCapabilityProjectionInput<'a> {
    applicability: ConsumerRouteApplicability,
    source: ConsumerRouteProjectionSourceIdentity,
    selection: &'a RegisteredToolSelection,
    readiness: &'a RegisteredToolReadiness,
    qualification: RegisteredToolRouteQualification,
    bundle: Option<&'a ResolvedSkillBundle>,
}

impl<'a> RegisteredCapabilityProjectionInput<'a> {
    /// Binds the exact applicability, source, selection, and readiness.
    #[must_use]
    pub const fn new(
        applicability: ConsumerRouteApplicability,
        source: ConsumerRouteProjectionSourceIdentity,
        selection: &'a RegisteredToolSelection,
        readiness: &'a RegisteredToolReadiness,
    ) -> Self {
        Self {
            applicability,
            source,
            selection,
            readiness,
            qualification: RegisteredToolRouteQualification::Unqualified,
            bundle: None,
        }
    }

    /// Records what one exact adapter route validation proved.
    #[must_use]
    pub const fn with_route_qualification(
        mut self,
        qualification: RegisteredToolRouteQualification,
    ) -> Self {
        self.qualification = qualification;
        self
    }

    /// Records the resolved bundle the consumer selected for this operation.
    #[must_use]
    pub const fn with_resolved_skill_bundle(mut self, bundle: &'a ResolvedSkillBundle) -> Self {
        self.bundle = Some(bundle);
        self
    }
}

/// Projects one registered capability as a Contract 061 contribution.
///
/// The builder is pure: it opens nothing, dispatches nothing, and never widens
/// support beyond what the supplied qualification and readiness prove.
///
/// Readiness must have been evaluated for exactly this selection and topology.
/// Mixed evidence — a ready record from another snapshot revision, tool subset,
/// carrier, protocol version, or host — rejects the whole contribution rather
/// than composing into an available row.
pub fn project_registered_capability(
    input: RegisteredCapabilityProjectionInput<'_>,
) -> Result<ConsumerRouteProjectionContribution, ConsumerRouteProjectionFailure> {
    if input.source.kind() != ConsumerRouteProjectionSourceKind::AdapterContribution {
        return Err(failure(
            ConsumerRouteProjectionFailureKind::IdentityInvalid,
            "swallowtail.consumer_route_projection.registered_capability_source_kind_rejected",
            "A registered-capability contribution must name an adapter contribution source",
        ));
    }
    if !input.readiness.was_evaluated_for(input.selection) {
        return Err(failure(
            ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement,
            "swallowtail.consumer_route_projection.registered_capability_readiness_mismatch",
            "Registered-capability readiness evidence belongs to another selection",
        ));
    }
    let qualified = match input.qualification {
        RegisteredToolRouteQualification::Qualified(route) => Some(route),
        RegisteredToolRouteQualification::Unqualified => None,
    };
    let selection_rows = vec![
        capability_row(&input, qualified)?,
        execution_kind_row(&input, qualified)?,
        transport_row(&input, qualified)?,
        permission_row(&input, qualified)?,
        progress_row(&input, qualified)?,
        scheduling_row(&input)?,
    ];
    let session_start_rows = vec![skill_bundle_row(&input, qualified)?];
    ConsumerRouteProjectionContribution::new(
        input.applicability.clone(),
        [input.source.clone()],
        selection_rows,
        session_start_rows,
        [],
    )
}

/// Publishes the registration revision and every selected schema digest.
fn capability_row(
    input: &RegisteredCapabilityProjectionInput<'_>,
    qualified: Option<RegisteredToolQualifiedRoute>,
) -> Result<ConsumerRouteProjectionRow, ConsumerRouteProjectionFailure> {
    let snapshot = input.selection.snapshot();
    let mut values = vec![
        value(snapshot.server_id().as_str())?,
        value(snapshot.revision().as_str())?,
    ];
    let mut digests = BTreeSet::new();
    for id in input.selection.selected() {
        let Some(declaration) = snapshot.declaration(id) else {
            return Err(failure(
                ConsumerRouteProjectionFailureKind::ApplicabilityDisagreement,
                "swallowtail.consumer_route_projection.registered_capability_tool_unknown",
                "A projected registered capability names a tool absent from its snapshot",
            ));
        };
        digests.insert(declaration.input_schema().digest().as_str().to_owned());
        digests.insert(declaration.output_schema().digest().as_str().to_owned());
    }
    for digest in digests {
        values.push(value(digest)?);
    }
    descriptive_row(
        input,
        feature_identity(input, REGISTERED_TOOL_CAPABILITY_SEMANTIC_ID)?,
        RowSupportEvidence::of(qualified.is_some(), true),
        ConsumerRouteValueKind::StructuredDeclarations,
        ConsumerRouteValueDomain::Enumerated(ConsumerRouteEnumeratedValues::new(values)?),
    )
}

/// Publishes the one execution kind each selected tool identity binds.
fn execution_kind_row(
    input: &RegisteredCapabilityProjectionInput<'_>,
    qualified: Option<RegisteredToolQualifiedRoute>,
) -> Result<ConsumerRouteProjectionRow, ConsumerRouteProjectionFailure> {
    let snapshot = input.selection.snapshot();
    let kinds: BTreeSet<_> = input
        .selection
        .selected()
        .iter()
        .filter_map(|id| snapshot.declaration(id))
        .map(|declaration| declaration.kind().as_str())
        .collect();
    let values = kinds
        .into_iter()
        .map(value)
        .collect::<Result<Vec<_>, _>>()?;
    descriptive_row(
        input,
        feature_identity(input, REGISTERED_TOOL_EXECUTION_KIND_SEMANTIC_ID)?,
        RowSupportEvidence::of(qualified.is_some(), true),
        ConsumerRouteValueKind::BoundedEnumeration,
        ConsumerRouteValueDomain::Enumerated(ConsumerRouteEnumeratedValues::new(values)?),
    )
}

/// Publishes host-mediated native dispatch versus a private MCP attachment.
fn transport_row(
    input: &RegisteredCapabilityProjectionInput<'_>,
    qualified: Option<RegisteredToolQualifiedRoute>,
) -> Result<ConsumerRouteProjectionRow, ConsumerRouteProjectionFailure> {
    let transport = input.selection.transport();
    let mediation = if transport.binds_listener() {
        "private-mcp-attachment"
    } else {
        "host-mediated-native"
    };
    let values = vec![value(transport.as_str())?, value(mediation)?];
    descriptive_row(
        input,
        feature_identity(input, REGISTERED_TOOL_TRANSPORT_SEMANTIC_ID)?,
        RowSupportEvidence::of(qualified.is_some(), qualified_transport(transport)),
        ConsumerRouteValueKind::BoundedEnumeration,
        ConsumerRouteValueDomain::Enumerated(ConsumerRouteEnumeratedValues::new(values)?),
    )
}

/// Publishes the exact one-shot Allow/Deny strength the adapter proved.
fn permission_row(
    input: &RegisteredCapabilityProjectionInput<'_>,
    qualified: Option<RegisteredToolQualifiedRoute>,
) -> Result<ConsumerRouteProjectionRow, ConsumerRouteProjectionFailure> {
    let strength = qualified.map(RegisteredToolQualifiedRoute::permission);
    let label = match strength {
        Some(RegisteredToolPermissionStrength::ExactOneShot) => "exact-one-shot",
        Some(RegisteredToolPermissionStrength::NotRepresented) => "not-represented",
        None => "unqualified",
    };
    descriptive_row(
        input,
        feature_identity(input, REGISTERED_TOOL_PERMISSION_SEMANTIC_ID)?,
        RowSupportEvidence::of(
            qualified.is_some(),
            matches!(
                strength,
                Some(RegisteredToolPermissionStrength::ExactOneShot)
            ),
        ),
        ConsumerRouteValueKind::CapabilityState,
        ConsumerRouteValueDomain::Enumerated(ConsumerRouteEnumeratedValues::new([value(label)?])?),
    )
}

/// Publishes the exact bounded tool-progress mode the adapter proved.
fn progress_row(
    input: &RegisteredCapabilityProjectionInput<'_>,
    qualified: Option<RegisteredToolQualifiedRoute>,
) -> Result<ConsumerRouteProjectionRow, ConsumerRouteProjectionFailure> {
    let mode = qualified.map(RegisteredToolQualifiedRoute::progress);
    let label = match mode {
        Some(RegisteredToolProgressMode::BoundedOrdered) => "bounded-ordered",
        Some(RegisteredToolProgressMode::NoProgress) => "no-progress",
        None => "unqualified",
    };
    descriptive_row(
        input,
        feature_identity(input, REGISTERED_TOOL_PROGRESS_SEMANTIC_ID)?,
        RowSupportEvidence::of(
            qualified.is_some(),
            matches!(mode, Some(RegisteredToolProgressMode::BoundedOrdered)),
        ),
        ConsumerRouteValueKind::CapabilityState,
        ConsumerRouteValueDomain::Enumerated(ConsumerRouteEnumeratedValues::new([value(label)?])?),
    )
}

/// Publishes scheduling as unsupported: no route passes a Contract 028 gate.
fn scheduling_row(
    input: &RegisteredCapabilityProjectionInput<'_>,
) -> Result<ConsumerRouteProjectionRow, ConsumerRouteProjectionFailure> {
    let row = ConsumerRouteProjectionRow::new(
        feature_identity(input, REGISTERED_TOOL_SCHEDULING_SEMANTIC_ID)?,
        input.applicability.clone(),
        input.source.clone(),
        ConsumerRouteSourceClass::AdapterPreparedInput,
        ConsumerRouteEvidenceStrength::RuntimeType,
        ConsumerRouteLifecycle::SelectionSummary,
    )
    .with_support(ConsumerRouteSupportPosture::Unsupported)
    .with_availability(ConsumerRouteAvailability::Unavailable)
    .with_actor_posture(ConsumerRouteActorPosture::Informational)
    .with_state_support(ConsumerRouteStateSupport::descriptor_only())
    .with_mutation_authority(ConsumerRouteMutationAuthority::Absent)
    .with_control_value(ConsumerRouteControlValue::new(
        ConsumerRouteValueKind::CapabilityState,
        ConsumerRouteValueDomain::Enumerated(ConsumerRouteEnumeratedValues::new([value(
            "withheld",
        )?])?),
        ConsumerRouteOmissionSemantics::NotSelectable,
    ))
    .with_safe_reason(ConsumerRouteSafeReason::new(
        ConsumerRouteAvailabilityDimension::SupportAuthority,
        input.source.id().clone(),
        SafeDiagnostic::new(
            SCHEDULING_WITHHELD_CODE,
            "Mid-turn steering and provider queueing remain withheld for every route",
        ),
    )?);
    Ok(row)
}

/// Publishes the selected skill bundle as a session-start control row.
///
/// The row carries the bundle's safe identity, revision, digest, reference
/// count, and resolved byte total. It never carries a skill or reference body.
fn skill_bundle_row(
    input: &RegisteredCapabilityProjectionInput<'_>,
    qualified: Option<RegisteredToolQualifiedRoute>,
) -> Result<ConsumerRouteProjectionRow, ConsumerRouteProjectionFailure> {
    let delivery = qualified.map(RegisteredToolQualifiedRoute::skill_delivery);
    let carried = matches!(
        delivery,
        Some(RegisteredToolSkillDelivery::BoundedSelectedBundle)
    );
    let domain = match input.bundle {
        Some(bundle) => {
            ConsumerRouteValueDomain::Enumerated(ConsumerRouteEnumeratedValues::new([
                value(bundle.identity().id().as_str())?,
                value(bundle.identity().provenance().as_str())?,
                value(bundle.revision().as_str())?,
                value(bundle.digest().as_str())?,
                value(format!("references={}", bundle.references().len()))?,
                value(format!("bytes={}", bundle.resolved_content_bytes()))?,
            ])?)
        }
        None => ConsumerRouteValueDomain::Descriptor,
    };
    let omission = if input.bundle.is_some() {
        ConsumerRouteOmissionSemantics::PreservesRouteBehavior
    } else {
        ConsumerRouteOmissionSemantics::NotSelectable
    };
    let evidence = RowSupportEvidence::of(qualified.is_some(), carried);
    let mut row = ConsumerRouteProjectionRow::new(
        control_identity(input, SELECTED_SKILL_BUNDLE_SEMANTIC_ID)?,
        input.applicability.clone(),
        input.source.clone(),
        ConsumerRouteSourceClass::AdapterPreparedInput,
        evidence.evidence_strength(),
        ConsumerRouteLifecycle::SessionStartOnly,
    )
    .with_support(evidence.support())
    .with_availability(availability(input, evidence))
    .with_control_value(ConsumerRouteControlValue::new(
        ConsumerRouteValueKind::StructuredContent,
        domain,
        omission,
    ));
    if carried && input.bundle.is_some() {
        row = row
            .with_actor_posture(ConsumerRouteActorPosture::ConsumerSelectable)
            .with_state_support(ConsumerRouteStateSupport::descriptor_only().with_prepared())
            .with_mutation_authority(ConsumerRouteMutationAuthority::PreparedSessionStart(
                input.source.id().clone(),
            ));
    }
    if let Some(reason) = unavailable_reason(input, evidence)? {
        row = row.with_safe_reason(reason);
    }
    Ok(row)
}

/// Builds one informational selection-summary row with its safe reason.
fn descriptive_row(
    input: &RegisteredCapabilityProjectionInput<'_>,
    identity: ConsumerRouteRowIdentity,
    evidence: RowSupportEvidence,
    kind: ConsumerRouteValueKind,
    domain: ConsumerRouteValueDomain,
) -> Result<ConsumerRouteProjectionRow, ConsumerRouteProjectionFailure> {
    let mut row = ConsumerRouteProjectionRow::new(
        identity,
        input.applicability.clone(),
        input.source.clone(),
        ConsumerRouteSourceClass::AdapterPreparedInput,
        evidence.evidence_strength(),
        ConsumerRouteLifecycle::SelectionSummary,
    )
    .with_support(evidence.support())
    .with_availability(availability(input, evidence))
    .with_actor_posture(ConsumerRouteActorPosture::Informational)
    .with_control_value(ConsumerRouteControlValue::new(
        kind,
        domain,
        ConsumerRouteOmissionSemantics::NotSelectable,
    ));
    if let Some(reason) = unavailable_reason(input, evidence)? {
        row = row.with_safe_reason(reason);
    }
    Ok(row)
}

/// Availability is separate from support and never inferred from it alone.
///
/// Only a dimension a qualified adapter proved supported can become available,
/// and only when readiness for this exact selection also passes.
fn availability(
    input: &RegisteredCapabilityProjectionInput<'_>,
    evidence: RowSupportEvidence,
) -> ConsumerRouteAvailability {
    if matches!(evidence, RowSupportEvidence::QualifiedSupported) && input.readiness.is_ready() {
        ConsumerRouteAvailability::Available
    } else {
        ConsumerRouteAvailability::Unavailable
    }
}

/// Names why an unavailable row is unavailable, without inventing a reason.
///
/// Each state carries the dimension its own source supplied: absent support
/// authority when no adapter has qualified the route, a route capability
/// constraint when a qualified adapter proved that exact dimension
/// unsupported, and the typed readiness diagnostic when a supported dimension
/// is not currently ready. A ready supported row carries no reason at all.
fn unavailable_reason(
    input: &RegisteredCapabilityProjectionInput<'_>,
    evidence: RowSupportEvidence,
) -> Result<Option<ConsumerRouteSafeReason>, ConsumerRouteProjectionFailure> {
    let (dimension, diagnostic) = match evidence {
        RowSupportEvidence::Unqualified => (
            ConsumerRouteAvailabilityDimension::SupportAuthority,
            SafeDiagnostic::new(
                ADAPTER_UNQUALIFIED_CODE,
                "No qualified adapter proves this registered capability for the exact route",
            ),
        ),
        RowSupportEvidence::QualifiedUnsupported => (
            ConsumerRouteAvailabilityDimension::CapabilityConstraint,
            SafeDiagnostic::new(
                ROUTE_DIMENSION_UNSUPPORTED_CODE,
                "The qualified route does not support this registered-capability dimension",
            ),
        ),
        RowSupportEvidence::QualifiedSupported => match input.readiness.require_ready() {
            Ok(_) => return Ok(None),
            Err(readiness_failure) => (
                ConsumerRouteAvailabilityDimension::RuntimeReadiness,
                readiness_failure.diagnostic(),
            ),
        },
    };
    ConsumerRouteSafeReason::new(dimension, input.source.id().clone(), diagnostic).map(Some)
}

/// Reuses the one qualified-carrier list the readiness gate enforces.
fn qualified_transport(transport: RegisteredToolTransport) -> bool {
    REGISTERED_TOOL_QUALIFIED_TRANSPORTS.contains(&transport)
}

fn value(
    text: impl Into<String>,
) -> Result<ConsumerRouteEnumerableValue, ConsumerRouteProjectionFailure> {
    ConsumerRouteEnumerableValue::new(text)
}

/// Builds one bounded feature identity for this exact route and version.
pub fn registered_capability_feature_id(
    route: &str,
    protocol_version: &RegisteredToolProtocolVersion,
    semantic_id: &str,
) -> Result<ConsumerRouteFeatureId, ConsumerRouteProjectionFailure> {
    ConsumerRouteNamespacedExtension::new(route, protocol_version.as_str(), semantic_id)
        .map(ConsumerRouteFeatureId::Namespaced)
}

/// Builds one bounded control identity for this exact route and version.
pub fn registered_capability_control_id(
    route: &str,
    protocol_version: &RegisteredToolProtocolVersion,
    semantic_id: &str,
) -> Result<ConsumerRouteControlId, ConsumerRouteProjectionFailure> {
    ConsumerRouteNamespacedExtension::new(route, protocol_version.as_str(), semantic_id)
        .map(ConsumerRouteControlId::Namespaced)
}

fn feature_identity(
    input: &RegisteredCapabilityProjectionInput<'_>,
    semantic_id: &str,
) -> Result<ConsumerRouteRowIdentity, ConsumerRouteProjectionFailure> {
    registered_capability_feature_id(
        input.applicability.protocol_facade_id().as_str(),
        input.selection.protocol_version(),
        semantic_id,
    )
    .map(ConsumerRouteRowIdentity::Feature)
}

fn control_identity(
    input: &RegisteredCapabilityProjectionInput<'_>,
    semantic_id: &str,
) -> Result<ConsumerRouteRowIdentity, ConsumerRouteProjectionFailure> {
    registered_capability_control_id(
        input.applicability.protocol_facade_id().as_str(),
        input.selection.protocol_version(),
        semantic_id,
    )
    .map(ConsumerRouteRowIdentity::Control)
}
