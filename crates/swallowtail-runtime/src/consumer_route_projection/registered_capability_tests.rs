//! Provider-free conformance for Contract 063 registered-capability rows.
//!
//! The fixture composes a real projection: rows must survive
//! `compose_consumer_route_projection`, not just row construction.

use super::compose::{ConsumerRouteProjectionInput, compose_consumer_route_projection};
use super::registered_capability::*;
use super::*;
use crate::host_reference::SelectedContentRef;
use crate::registered_tool::*;
use crate::{
    ConfiguredProviderInstanceAdmission, ConfiguredProviderInstanceRecord, HostServices,
    MonotonicInstant, PreparedAccessEvidence, PreparedOperationEvidence,
};
use std::sync::Arc;
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, AdapterId, AdapterIdentity,
    AdapterVersion, Capability, CapabilityProfile, CapabilityRequirement, ConfiguredInstance,
    ConfiguredInstanceId, CredentialMechanism, CredentialState, DriverDescriptor, DriverRole,
    EndpointAudience, EndpointAuthorization, EntitlementMetering, EntitlementState,
    ExecutionHostId, ExecutionLayer, HostServiceKind, InstanceOwnership, InstancePolicyId,
    InstanceRevision, InstanceTargetRef, IntegrationFamilyId, OperationRequirements,
    OperationShape, PreflightContext, PreflightPlan, ProtocolFacadeId, RuntimeReadiness,
    SupportAuthority, TransportFamilyId, preflight,
};

const SKILL_BODY: &[u8] = b"secret-skill-body";
const ADAPTER_SOURCE: &str = "fixture.source.registered-capability";

fn host() -> ExecutionHostId {
    ExecutionHostId::new("fixture.host.local").expect("host id")
}

fn adapter_source() -> ConsumerRouteProjectionSourceIdentity {
    ConsumerRouteProjectionSourceIdentity::new(
        ConsumerRouteProjectionSourceId::new(ADAPTER_SOURCE).expect("source id"),
        ConsumerRouteProjectionSourceKind::AdapterContribution,
    )
}

fn schema(digest: &str) -> RegisteredToolSchema {
    RegisteredToolSchema::new(
        RegisteredToolSchemaNamespace::new("fixture.schema").expect("namespace"),
        RegisteredToolSchemaMediaType::new("application/json").expect("media type"),
        RegisteredToolSchemaDialect::new("json-schema-2020-12").expect("dialect"),
        RegisteredServerRevision::new("1").expect("revision"),
        RegisteredToolSchemaDigest::new(digest).expect("digest"),
        RegisteredToolSchemaDocument::new("{\"secret\":\"schema-body\"}").expect("document"),
    )
}

fn selection() -> RegisteredToolSelection {
    let declaration = RegisteredToolDeclaration::new(
        RegisteredToolId::new(
            RegisteredToolNamespace::new("desktop.tools").expect("namespace"),
            RegisteredToolLocalName::new("apply-edit").expect("local name"),
        ),
        RegisteredToolExecutionKind::NativeClient,
        schema("sha256:input"),
        schema("sha256:output"),
        RegisteredToolEffectPosture::Mutating,
        RegisteredToolRetryPosture::NeverRetry,
        RegisteredToolBounds::ceiling(),
    )
    .expect("declaration");
    let snapshot = RegisteredToolSnapshot::new(RegisteredToolSnapshotInput {
        server_id: RegisteredServerId::new("desktop.server").expect("server id"),
        revision: RegisteredServerRevision::new("11").expect("revision"),
        execution_host_id: host(),
        declarations: vec![declaration.clone()],
        transports: vec![
            RegisteredToolTransportSupport::new(
                RegisteredToolTransport::HostMediatedCallback,
                [protocol_version()],
            )
            .expect("transport support"),
        ],
        required_services: [HostServiceKind::Time].into_iter().collect(),
        credential_references: Vec::new(),
        executable_recipes: Vec::new(),
        environment_recipes: Vec::new(),
        bounds: RegisteredToolBounds::ceiling(),
        source: RegisteredToolSource::new(
            RegisteredToolSourceId::new("desktop.registration").expect("source id"),
            MonotonicInstant::from_ticks(1),
        ),
    })
    .expect("snapshot");
    RegisteredToolSelection::new(
        Arc::new(snapshot),
        [declaration.id().clone()],
        RegisteredToolTransport::HostMediatedCallback,
        protocol_version(),
    )
    .expect("selection")
}

fn protocol_version() -> RegisteredToolProtocolVersion {
    RegisteredToolProtocolVersion::new(REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION)
        .expect("protocol version")
}

/// Readiness with no mounted port, so the profile is not currently available.
fn unready(selection: &RegisteredToolSelection) -> RegisteredToolReadiness {
    RegisteredToolReadiness::evaluate(&HostServices::new(host()), selection)
}

/// Readiness against a topology whose port and required services are mounted.
fn ready(selection: &RegisteredToolSelection) -> RegisteredToolReadiness {
    RegisteredToolReadiness::for_topology(&MountedTopologyFixture::topology(), selection)
}

/// Builds the exact mounted topology the readiness gate measures against.
struct MountedTopologyFixture;

impl MountedTopologyFixture {
    fn topology() -> RegisteredToolMountedTopology {
        RegisteredToolMountedTopology::from_hosts(
            &HostServices::new(host())
                .with_time(Arc::new(FixedClock))
                .with_registered_tool_bridge(Arc::new(UnusedPort)),
        )
    }
}

struct FixedClock;

impl crate::TimeService for FixedClock {
    fn now(&self) -> MonotonicInstant {
        MonotonicInstant::from_ticks(1)
    }

    fn wait_until(
        &self,
        deadline: crate::Deadline,
    ) -> crate::BoxFuture<'static, crate::DeadlineObservation> {
        Box::pin(std::future::ready(crate::DeadlineObservation::new(
            deadline,
            self.now(),
        )))
    }
}

/// A mounted port the projection never calls: projection opens nothing.
struct UnusedPort;

impl RegisteredToolBridgeHostService for UnusedPort {
    fn open(
        &self,
        _request: RegisteredToolOpenRequest,
    ) -> crate::BoxFuture<'_, Result<RegisteredToolBridgeLease, crate::RuntimeFailure>> {
        unreachable!("projection never opens a lease")
    }

    fn completion_gate(
        &self,
        _lease: &RegisteredToolBridgeLease,
    ) -> crate::BoxFuture<'_, Result<RegisteredToolCompletionState, crate::RuntimeFailure>> {
        unreachable!("projection never observes a completion gate")
    }

    fn close(
        &self,
        _lease: RegisteredToolBridgeLease,
        _cause: RegisteredToolCleanupCause,
    ) -> crate::BoxFuture<'_, Result<crate::CleanupOutcome, crate::RuntimeFailure>> {
        unreachable!("projection never closes a lease")
    }
}

fn resolved_bundle() -> ResolvedSkillBundle {
    let body = SelectedContentDescriptor::new(
        SelectedContentRef::new("host://skill").expect("reference"),
        RegisteredToolSchemaMediaType::new("text/markdown").expect("media type"),
        SKILL_BODY.len(),
    )
    .expect("descriptor");
    let bundle = SelectedSkillBundle::new(
        SelectedSkillIdentity::new(
            SelectedSkillId::new("desktop.skill.review").expect("skill id"),
            SelectedSkillProvenance::ProjectBound,
            body,
        ),
        SelectedSkillRevision::new("7").expect("revision"),
        SelectedContentDigest::of_bytes(SKILL_BODY),
        [],
        SelectedSkillBundleBounds::ceiling(),
    )
    .expect("bundle");
    let resources =
        SelectedSkillHostResources::new().with_skill_body(SelectedContentResolution::Resolved(
            RegisteredToolPayload::new(
                RegisteredToolSchemaMediaType::new("text/markdown").expect("media type"),
                SKILL_BODY.to_vec(),
                MAX_SELECTED_SKILL_CONTENT_BYTES,
            )
            .expect("payload"),
        ));
    bundle.resolve(&resources).expect("bundle resolves")
}

fn qualified() -> RegisteredToolRouteQualification {
    RegisteredToolRouteQualification::Qualified(RegisteredToolQualifiedRoute::new(
        RegisteredToolPermissionStrength::ExactOneShot,
        RegisteredToolProgressMode::BoundedOrdered,
        RegisteredToolSkillDelivery::BoundedSelectedBundle,
    ))
}

/// Minimal configured record and prepared evidence that agree exactly.
struct RouteFixture {
    driver: DriverDescriptor,
    instance: ConfiguredInstance,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
}

impl RouteFixture {
    fn new() -> Self {
        let adapter_id = AdapterId::new("fixture.registered-capability").expect("adapter id");
        let access_id = AccessProfileId::new("fixture.access").expect("access id");
        let capabilities =
            CapabilityProfile::new([CapabilityRequirement::new(Capability::StructuredRun, [])]);
        let driver = DriverDescriptor::new(
            AdapterIdentity::new(
                adapter_id.clone(),
                AdapterVersion::new("1").expect("adapter version"),
            ),
            IntegrationFamilyId::new("fixture-family").expect("family id"),
            TransportFamilyId::new("fixture-transport").expect("transport id"),
        )
        .with_roles([DriverRole::StructuredRun])
        .with_execution_layers([ExecutionLayer::HarnessInteraction])
        .with_operation_shapes([OperationShape::StructuredRun]);
        let instance = ConfiguredInstance::new(
            ConfiguredInstanceId::new("fixture.instance").expect("instance id"),
            InstanceRevision::new("revision-1").expect("instance revision"),
            adapter_id,
            host(),
            InstanceTargetRef::new("private-instance-target").expect("target"),
            InstanceOwnership::HostOwnedPersistent,
            access_id.clone(),
            SupportAuthority::ProviderSupported,
            ProtocolFacadeId::new("fixture-facade").expect("facade id"),
            InstancePolicyId::new("fixture-policy").expect("policy id"),
            capabilities,
        );
        let access_profile = AccessProfile::new(
            access_id.clone(),
            CredentialMechanism::LocalUnauthenticated,
            EntitlementMetering::PayAsYouGo,
            EndpointAudience::new("fixture-audience").expect("audience"),
            SupportAuthority::ProviderSupported,
        );
        let access_evidence = PreparedAccessEvidence::caller_asserted(AccessStatus::new(
            access_id,
            CredentialState::NotRequired,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        ));
        Self {
            driver,
            instance,
            access_profile,
            access_evidence,
        }
    }

    fn plan(&self) -> PreflightPlan {
        let status = self.access_evidence.status();
        let requirements = OperationRequirements::new(
            ExecutionLayer::HarnessInteraction,
            OperationShape::StructuredRun,
            DriverRole::StructuredRun,
            self.instance.execution_host_id().clone(),
            AccessRequirement::new(self.access_profile.id().clone())
                .with_credential_states([status.credential()])
                .with_entitlement_states([status.entitlement()])
                .with_endpoint_authorizations([status.endpoint_authorization()])
                .with_runtime_readiness([status.runtime_readiness()])
                .with_support_authorities([status.support_authority()]),
        )
        .with_ownership_modes([self.instance.ownership()])
        .with_capabilities([CapabilityRequirement::new(Capability::StructuredRun, [])]);
        preflight(
            &PreflightContext::new(
                &self.driver,
                &self.instance,
                &self.access_profile,
                status,
                [],
            ),
            &requirements,
        )
        .expect("fixture preflight succeeds")
    }

    fn prepared(&self) -> PreparedOperationEvidence {
        PreparedOperationEvidence::from_plan(self.plan(), self.access_evidence.clone())
            .expect("prepared evidence")
    }

    fn record(&self) -> ConfiguredProviderInstanceRecord {
        ConfiguredProviderInstanceRecord::admit(
            ConfiguredProviderInstanceAdmission::new(
                self.driver.clone(),
                self.instance.clone(),
                self.access_profile.clone(),
                self.access_evidence.clone(),
            )
            .with_prepared_routes([self.prepared()]),
        )
        .expect("configured record")
    }

    fn applicability(&self) -> ConsumerRouteApplicability {
        ConsumerRouteApplicability::from_prepared_operation(&self.prepared())
    }
}

/// Composes the contribution into a real projection, as a consumer would.
fn compose(contribution: &ConsumerRouteProjectionContribution) -> ConsumerRouteProjection {
    let fixture = RouteFixture::new();
    let record = fixture.record();
    let prepared = fixture.prepared();
    compose_consumer_route_projection(
        ConsumerRouteProjectionInput::new(
            &record,
            ConsumerRouteProjectionSourceIdentity::new(
                ConsumerRouteProjectionSourceId::new("fixture.source.record").expect("source id"),
                ConsumerRouteProjectionSourceKind::ConfiguredInstance,
            ),
            &prepared,
            ConsumerRouteProjectionSourceIdentity::new(
                ConsumerRouteProjectionSourceId::new("fixture.source.prepared").expect("source id"),
                ConsumerRouteProjectionSourceKind::PreparedOperation,
            ),
        )
        .with_contributions([contribution]),
    )
    .expect("projection composes")
}

fn feature(semantic_id: &str) -> ConsumerRouteRowIdentity {
    ConsumerRouteRowIdentity::Feature(
        registered_capability_feature_id("fixture-facade", &protocol_version(), semantic_id)
            .expect("bounded feature identity"),
    )
}

fn selection_row<'a>(
    projection: &'a ConsumerRouteProjection,
    semantic_id: &str,
) -> &'a ConsumerRouteProjectionRow {
    let identity = feature(semantic_id);
    projection
        .selection_summary()
        .rows()
        .find(|row| row.identity() == &identity)
        .expect("selection row is published")
}

fn skill_row(projection: &ConsumerRouteProjection) -> &ConsumerRouteProjectionRow {
    let identity = ConsumerRouteRowIdentity::Control(
        registered_capability_control_id(
            "fixture-facade",
            &protocol_version(),
            SELECTED_SKILL_BUNDLE_SEMANTIC_ID,
        )
        .expect("bounded control identity"),
    );
    projection
        .session_start_controls()
        .rows()
        .find(|row| row.identity() == &identity)
        .expect("skill bundle row is published")
}

fn domain_values(row: &ConsumerRouteProjectionRow) -> Vec<String> {
    match row.control_value().expect("row publishes a value").domain() {
        ConsumerRouteValueDomain::Enumerated(values) => values
            .values()
            .map(|value| value.as_str().to_owned())
            .collect(),
        _ => Vec::new(),
    }
}

fn project(
    qualification: RegisteredToolRouteQualification,
    readiness: &RegisteredToolReadiness,
    selection: &RegisteredToolSelection,
    bundle: Option<&ResolvedSkillBundle>,
) -> ConsumerRouteProjectionContribution {
    let fixture = RouteFixture::new();
    let mut input = RegisteredCapabilityProjectionInput::new(
        fixture.applicability(),
        adapter_source(),
        selection,
        readiness,
    )
    .with_route_qualification(qualification);
    if let Some(bundle) = bundle {
        input = input.with_resolved_skill_bundle(bundle);
    }
    project_registered_capability(input).expect("contribution is admitted")
}

#[test]
fn an_unqualified_registered_capability_is_never_supported_by_inference() {
    let selection = selection();
    let readiness = ready(&selection);
    let contribution = project(
        RegisteredToolRouteQualification::Unqualified,
        &readiness,
        &selection,
        None,
    );

    let projection = compose(&contribution);
    for row in projection.selection_summary().rows() {
        assert_ne!(row.support(), ConsumerRouteSupportPosture::Supported);
        assert_eq!(row.availability(), ConsumerRouteAvailability::Unavailable);
        assert_eq!(
            row.safe_reason().expect("reason is published").dimension(),
            ConsumerRouteAvailabilityDimension::SupportAuthority
        );
    }
    let capability = selection_row(&projection, REGISTERED_TOOL_CAPABILITY_SEMANTIC_ID);
    assert_eq!(capability.support(), ConsumerRouteSupportPosture::Unknown);
    assert_eq!(
        capability.evidence_strength(),
        ConsumerRouteEvidenceStrength::RuntimeType
    );
    assert_eq!(
        skill_row(&projection).support(),
        ConsumerRouteSupportPosture::Unknown
    );
}

#[test]
fn a_qualified_ready_capability_publishes_its_safe_registration_evidence() {
    let selection = selection();
    let readiness = ready(&selection);
    let bundle = resolved_bundle();
    let contribution = project(qualified(), &readiness, &selection, Some(&bundle));

    let projection = compose(&contribution);
    let capability = selection_row(&projection, REGISTERED_TOOL_CAPABILITY_SEMANTIC_ID);
    assert_eq!(capability.support(), ConsumerRouteSupportPosture::Supported);
    assert_eq!(
        capability.availability(),
        ConsumerRouteAvailability::Available
    );
    assert!(capability.safe_reason().is_none());
    assert_eq!(
        domain_values(capability),
        vec![
            "desktop.server".to_owned(),
            "11".to_owned(),
            "sha256:input".to_owned(),
            "sha256:output".to_owned(),
        ]
    );
    assert_eq!(
        domain_values(selection_row(
            &projection,
            REGISTERED_TOOL_EXECUTION_KIND_SEMANTIC_ID
        )),
        vec!["native-client".to_owned()]
    );
    assert_eq!(
        domain_values(selection_row(
            &projection,
            REGISTERED_TOOL_TRANSPORT_SEMANTIC_ID
        )),
        vec![
            "host-mediated-callback".to_owned(),
            "host-mediated-native".to_owned(),
        ]
    );
}

#[test]
fn readiness_is_kept_separate_from_descriptive_support() {
    let selection = selection();
    let readiness = unready(&selection);
    let contribution = project(qualified(), &readiness, &selection, None);

    let projection = compose(&contribution);
    let capability = selection_row(&projection, REGISTERED_TOOL_CAPABILITY_SEMANTIC_ID);
    assert_eq!(capability.support(), ConsumerRouteSupportPosture::Supported);
    assert_eq!(
        capability.availability(),
        ConsumerRouteAvailability::Unavailable
    );
    assert_eq!(
        capability.safe_reason().expect("reason").dimension(),
        ConsumerRouteAvailabilityDimension::RuntimeReadiness
    );
    assert_eq!(
        capability
            .safe_reason()
            .expect("reason")
            .diagnostic()
            .code(),
        RegisteredToolFailureKind::MissingHostService.code()
    );
}

#[test]
fn a_route_that_cannot_represent_deny_publishes_unsupported_permission() {
    let selection = selection();
    let readiness = ready(&selection);
    let qualification =
        RegisteredToolRouteQualification::Qualified(RegisteredToolQualifiedRoute::new(
            RegisteredToolPermissionStrength::NotRepresented,
            RegisteredToolProgressMode::NoProgress,
            RegisteredToolSkillDelivery::NotCarried,
        ));
    let contribution = project(qualification, &readiness, &selection, None);

    let projection = compose(&contribution);
    for feature in [
        REGISTERED_TOOL_PERMISSION_SEMANTIC_ID,
        REGISTERED_TOOL_PROGRESS_SEMANTIC_ID,
    ] {
        let row = selection_row(&projection, feature);
        assert_eq!(row.support(), ConsumerRouteSupportPosture::Unsupported);
        assert_eq!(row.availability(), ConsumerRouteAvailability::Unavailable);
    }
    let skill = skill_row(&projection);
    assert_eq!(skill.support(), ConsumerRouteSupportPosture::Unsupported);
    assert_eq!(
        skill.actor_posture(),
        ConsumerRouteActorPosture::Informational
    );
    assert!(skill.mutation_authority().source().is_none());
}

#[test]
fn scheduling_stays_withheld_for_every_qualified_route() {
    let selection = selection();
    let readiness = ready(&selection);
    let contribution = project(qualified(), &readiness, &selection, None);

    let projection = compose(&contribution);
    let scheduling = selection_row(&projection, REGISTERED_TOOL_SCHEDULING_SEMANTIC_ID);
    assert_eq!(
        scheduling.support(),
        ConsumerRouteSupportPosture::Unsupported
    );
    assert_eq!(
        scheduling.availability(),
        ConsumerRouteAvailability::Unavailable
    );
    assert_eq!(
        scheduling
            .safe_reason()
            .expect("reason")
            .diagnostic()
            .code(),
        "swallowtail.registered_tool.scheduling_withheld"
    );
}

#[test]
fn a_projected_bundle_carries_safe_identity_and_limits_only() {
    let selection = selection();
    let readiness = ready(&selection);
    let bundle = resolved_bundle();
    let contribution = project(qualified(), &readiness, &selection, Some(&bundle));

    let projection = compose(&contribution);
    let skill = skill_row(&projection);
    assert_eq!(skill.support(), ConsumerRouteSupportPosture::Supported);
    assert_eq!(skill.availability(), ConsumerRouteAvailability::Available);
    assert_eq!(skill.lifecycle(), ConsumerRouteLifecycle::SessionStartOnly);
    assert!(skill.state_support().prepared());
    assert!(skill.mutation_authority().is_prepared_session_start());
    assert_eq!(
        domain_values(skill),
        vec![
            "desktop.skill.review".to_owned(),
            "project-bound".to_owned(),
            "7".to_owned(),
            SelectedContentDigest::of_bytes(SKILL_BODY)
                .as_str()
                .to_owned(),
            "references=0".to_owned(),
            format!("bytes={}", SKILL_BODY.len()),
        ]
    );
    let rendered = format!("{projection:?}");
    assert!(!rendered.contains("secret-skill-body"));
    assert!(!rendered.contains("schema-body"));
    assert!(!rendered.contains("host://skill"));
    assert!(!rendered.contains("private-instance-target"));
}

#[test]
fn a_non_adapter_source_cannot_publish_registered_capability_rows() {
    let fixture = RouteFixture::new();
    let selection = selection();
    let readiness = ready(&selection);

    let failure = project_registered_capability(RegisteredCapabilityProjectionInput::new(
        fixture.applicability(),
        ConsumerRouteProjectionSourceIdentity::new(
            ConsumerRouteProjectionSourceId::new(ADAPTER_SOURCE).expect("source id"),
            ConsumerRouteProjectionSourceKind::PreparedOperation,
        ),
        &selection,
        &readiness,
    ))
    .expect_err("a non-adapter source is rejected");

    assert_eq!(
        failure.kind(),
        ConsumerRouteProjectionFailureKind::IdentityInvalid
    );
}

#[test]
fn every_row_names_its_exact_route_and_qualified_version_segment() {
    let selection = selection();
    let readiness = ready(&selection);
    let bundle = resolved_bundle();
    let contribution = project(qualified(), &readiness, &selection, Some(&bundle));

    let projection = compose(&contribution);
    let mut semantic_ids = Vec::new();
    for row in projection
        .selection_summary()
        .rows()
        .chain(projection.session_start_controls().rows())
    {
        let extension = row
            .identity()
            .namespaced_extension()
            .expect("registered-capability rows use bounded namespaced identity");
        assert_eq!(extension.route(), "fixture-facade");
        assert_eq!(
            extension.version_segment(),
            REGISTERED_TOOL_CONFORMANCE_PROTOCOL_VERSION
        );
        semantic_ids.push(extension.semantic_id().to_owned());
    }

    let mut expected = vec![
        REGISTERED_TOOL_CAPABILITY_SEMANTIC_ID.to_owned(),
        REGISTERED_TOOL_EXECUTION_KIND_SEMANTIC_ID.to_owned(),
        REGISTERED_TOOL_PERMISSION_SEMANTIC_ID.to_owned(),
        REGISTERED_TOOL_PROGRESS_SEMANTIC_ID.to_owned(),
        REGISTERED_TOOL_SCHEDULING_SEMANTIC_ID.to_owned(),
        REGISTERED_TOOL_TRANSPORT_SEMANTIC_ID.to_owned(),
        SELECTED_SKILL_BUNDLE_SEMANTIC_ID.to_owned(),
    ];
    semantic_ids.sort();
    expected.sort();
    assert_eq!(semantic_ids, expected);
}
