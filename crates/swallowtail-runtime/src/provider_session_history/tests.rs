use super::{
    ProviderSessionHistoryAgreement, ProviderSessionHistoryBounds, ProviderSessionHistoryCursor,
    ProviderSessionHistoryPage, ProviderSessionHistoryPlan, ProviderSessionHistoryRequest,
    ProviderSessionHistoryTotal, page_provider_session_history_window,
};
use crate::{
    CleanupOutcome, Deadline, MonotonicInstant, OperationContent, ProviderSessionHistoryId,
    RequestId, SessionReplayItem, SessionReplayKind, SessionResumeBinding, WorkingResourceRef,
};
use std::num::{NonZeroU32, NonZeroU64};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessRequirement, AccessStatus, AdapterId, AdapterIdentity,
    AdapterVersion, Capability, CapabilityConstraint, CapabilityProfile, CapabilityRequirement,
    ConfiguredInstance, ConfiguredInstanceId, CredentialMechanism, CredentialState,
    DriverDescriptor, DriverRole, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, ExecutionLayer, HarnessIsolation, HostServiceKind,
    InstanceOwnership, InstancePolicyId, InstanceRevision, InstanceTargetRef, IntegrationFamilyId,
    InterfaceBehaviorRevision, InterfaceCompatibilityClaim, InterfaceCompatibilityClaimId,
    InterfaceNewerVersionPosture, InterfaceSupportStatus, InterfaceVersion, InterfaceVersionAxis,
    InterfaceVersionBinding, InterfaceVersionScheme, InterfaceVersionSegment, ModelId, ModelRoute,
    ModelRouteId, ModelRouteRevision, OperationRequirements, OperationShape, PreflightContext,
    ProtocolFacadeId, ResourceAccess, ResourceRepresentation, RuntimeReadiness,
    SessionAccessPolicy, SessionProviderStatePolicy, SessionRef, SupportAuthority,
    TransportFamilyId, preflight,
};

struct Fixture {
    driver: DriverDescriptor,
    instance: ConfiguredInstance,
    route: ModelRoute,
    access_profile: AccessProfile,
    access_status: AccessStatus,
}

fn fixture() -> Fixture {
    let axis = InterfaceVersionAxis::new("fixture.session").expect("axis is valid");
    let version = InterfaceVersion::new("1.0.0").expect("version is valid");
    let capabilities = CapabilityProfile::new([
        CapabilityRequirement::new(
            Capability::ProviderSessionHistory,
            [
                CapabilityConstraint::ReplayMaximumItems(2),
                CapabilityConstraint::ReplayMaximumBytes(64),
            ],
        ),
        CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
        CapabilityRequirement::new(
            Capability::WorkingResource,
            [
                CapabilityConstraint::ResourceAccess(ResourceAccess::Read),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        ),
    ]);
    let driver = DriverDescriptor::new(
        AdapterIdentity::new(
            AdapterId::new("fixture.driver").expect("driver id is valid"),
            AdapterVersion::new("1.0.0").expect("driver version is valid"),
        ),
        IntegrationFamilyId::new("fixture").expect("family is valid"),
        TransportFamilyId::new("fixture-rpc").expect("transport is valid"),
    )
    .with_roles([DriverRole::ProviderSessionHistory])
    .with_execution_layers([ExecutionLayer::HarnessInteraction])
    .with_operation_shapes([OperationShape::ProviderSessionHistory])
    .with_interface_compatibility(
        InterfaceCompatibilityClaim::new(
            InterfaceCompatibilityClaimId::new("fixture.session.support")
                .expect("claim id is valid"),
            axis.clone(),
            InterfaceVersionScheme::Semantic,
            InterfaceNewerVersionPosture::QualifiedOnly,
            [InterfaceVersionSegment::exact(
                version.clone(),
                InterfaceBehaviorRevision::new("fixture-v1").expect("revision is valid"),
                InterfaceSupportStatus::Maintained,
            )],
            [],
        )
        .expect("claim is valid"),
    );
    let access_id = AccessProfileId::new("fixture.access").expect("access id is valid");
    let instance_id = ConfiguredInstanceId::new("fixture.instance").expect("instance id is valid");
    let instance = ConfiguredInstance::new(
        instance_id.clone(),
        InstanceRevision::new("revision-1").expect("revision is valid"),
        AdapterId::new("fixture.driver").expect("driver id is valid"),
        ExecutionHostId::new("fixture.host").expect("host id is valid"),
        InstanceTargetRef::new("private/service/target").expect("target is valid"),
        InstanceOwnership::ExternalAttached,
        access_id.clone(),
        SupportAuthority::IntegrationMaintainerSupported,
        ProtocolFacadeId::new("fixture.facade").expect("facade is valid"),
        InstancePolicyId::new("fixture.policy").expect("policy is valid"),
        capabilities.clone(),
    )
    .with_interface_versions([InterfaceVersionBinding::new(axis, version)]);
    let route = ModelRoute::new(
        ModelRouteId::new("fixture.route").expect("route id is valid"),
        ModelRouteRevision::new("route-1").expect("route revision is valid"),
        instance_id,
        ModelId::new("fixture-model").expect("model id is valid"),
        capabilities,
    );
    let access_profile = AccessProfile::new(
        access_id.clone(),
        CredentialMechanism::Unauthenticated,
        EntitlementMetering::Unknown,
        EndpointAudience::new("fixture").expect("audience is valid"),
        SupportAuthority::IntegrationMaintainerSupported,
    );
    let access_status = AccessStatus::new(
        access_id,
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::IntegrationMaintainerSupported,
    );
    Fixture {
        driver,
        instance,
        route,
        access_profile,
        access_status,
    }
}

fn bounds(page_items: u32, page_bytes: u64, snapshot_items: u32) -> ProviderSessionHistoryBounds {
    ProviderSessionHistoryBounds::new(
        NonZeroU32::new(page_items).expect("nonzero"),
        NonZeroU64::new(page_bytes).expect("nonzero"),
        NonZeroU32::new(64).expect("nonzero"),
        NonZeroU32::new(snapshot_items).expect("nonzero"),
    )
}

fn history_plan(
    fixture: &Fixture,
    id: &str,
    page_items: u32,
    page_bytes: u64,
    snapshot_items: u32,
) -> ProviderSessionHistoryPlan {
    let policy = SessionAccessPolicy::ambient_harness(ResourceAccess::Read);
    let requirements = OperationRequirements::new(
        ExecutionLayer::HarnessInteraction,
        OperationShape::ProviderSessionHistory,
        DriverRole::ProviderSessionHistory,
        fixture.instance.execution_host_id().clone(),
        AccessRequirement::new(fixture.access_profile.id().clone())
            .with_credential_states([CredentialState::Ready])
            .with_entitlement_states([EntitlementState::Available])
            .with_endpoint_authorizations([EndpointAuthorization::Allowed])
            .with_runtime_readiness([RuntimeReadiness::Ready])
            .with_support_authorities([SupportAuthority::IntegrationMaintainerSupported]),
    )
    .with_ownership_modes([InstanceOwnership::ExternalAttached])
    .with_host_services([
        HostServiceKind::Time,
        HostServiceKind::WorkingResource,
    ])
    .with_capabilities([
        CapabilityRequirement::new(
            Capability::ProviderSessionHistory,
            [
                CapabilityConstraint::ReplayMaximumItems(page_items),
                CapabilityConstraint::ReplayMaximumBytes(page_bytes),
            ],
        ),
        CapabilityRequirement::new(Capability::ProviderDurableRetention, []),
        CapabilityRequirement::new(
            Capability::WorkingResource,
            [
                CapabilityConstraint::ResourceAccess(ResourceAccess::Read),
                CapabilityConstraint::ResourceRepresentation(ResourceRepresentation::Filesystem),
            ],
        ),
    ])
    .with_interface_versions(fixture.instance.interface_versions().cloned())
    .require_model_route()
    .with_harness_isolation(HarnessIsolation::AmbientHost)
    .with_session_access_policy(policy.clone())
    .with_session_provider_state_policy(SessionProviderStatePolicy::DurableProviderSessionPreserved);
    let preflight = preflight(
        &PreflightContext::new(
            &fixture.driver,
            &fixture.instance,
            &fixture.access_profile,
            &fixture.access_status,
            [HostServiceKind::Time, HostServiceKind::WorkingResource],
        )
        .with_model_route(&fixture.route),
        &requirements,
    )
    .expect("history preflight is valid");
    let binding = SessionResumeBinding::new(
        SessionRef::new("provider/private/session").expect("session ref is valid"),
        fixture.instance.id().clone(),
        fixture.instance.execution_host_id().clone(),
        fixture.route.id().clone(),
        fixture.route.model_id().clone(),
        WorkingResourceRef::new("resource-a").expect("resource is valid"),
        policy,
    );
    ProviderSessionHistoryPlan::new(
        preflight,
        ProviderSessionHistoryAgreement::new(
            ProviderSessionHistoryId::new(id).expect("history id is valid"),
            binding,
            bounds(page_items, page_bytes, snapshot_items),
            Some(Deadline::at(MonotonicInstant::from_ticks(100))),
        ),
    )
    .expect("history plan is valid")
}

fn replay_item(sequence: u64, content: &str) -> SessionReplayItem {
    SessionReplayItem::with_content(
        SessionRef::new("provider/private/session").expect("session ref is valid"),
        sequence,
        SessionReplayKind::AgentMessage,
        OperationContent::new(content).expect("content is valid"),
    )
}

fn sequences(page: &ProviderSessionHistoryPage) -> Vec<u64> {
    page.items().map(SessionReplayItem::sequence).collect()
}

#[test]
fn empty_history_returns_exact_zero_without_older_cursor() {
    let fixture = fixture();
    let plan = history_plan(&fixture, "history-a", 2, 64, 8);
    let request = ProviderSessionHistoryRequest::from_plan(
        RequestId::new("history-empty").expect("request id is valid"),
        &plan,
        None,
    )
    .expect("request is valid");
    let window = page_provider_session_history_window(
        &plan,
        &request,
        Vec::new(),
        ProviderSessionHistoryTotal::Exact(0),
    )
    .expect("empty history is valid");
    let page = ProviderSessionHistoryPage::new(&plan, &request, window, CleanupOutcome::Clean)
        .expect("page is valid");

    assert_eq!(page.fetched_count(), 0);
    assert!(!page.has_older());
    assert!(page.older_cursor().is_none());
    assert_eq!(page.total(), ProviderSessionHistoryTotal::Exact(0));
}

#[test]
fn first_page_returns_newest_window_and_older_continuation() {
    let fixture = fixture();
    let plan = history_plan(&fixture, "history-a", 2, 64, 8);
    let request = ProviderSessionHistoryRequest::from_plan(
        RequestId::new("history-first").expect("request id is valid"),
        &plan,
        None,
    )
    .expect("request is valid");
    let snapshot = vec![
        replay_item(0, "a"),
        replay_item(1, "b"),
        replay_item(2, "c"),
        replay_item(3, "d"),
    ];
    let window = page_provider_session_history_window(
        &plan,
        &request,
        snapshot,
        ProviderSessionHistoryTotal::Exact(4),
    )
    .expect("first page is valid");
    let page = ProviderSessionHistoryPage::new(&plan, &request, window, CleanupOutcome::Clean)
        .expect("page is valid");

    assert_eq!(sequences(&page), [2, 3]);
    assert!(page.has_older());
    let older = page
        .older_cursor()
        .expect("older cursor")
        .clone();
    assert_eq!(older.older_end(), 2);

    let older_request = ProviderSessionHistoryRequest::from_plan(
        RequestId::new("history-older").expect("request id is valid"),
        &plan,
        Some(older),
    )
    .expect("older request is valid");
    let older_window = page_provider_session_history_window(
        &plan,
        &older_request,
        vec![
            replay_item(0, "a"),
            replay_item(1, "b"),
            replay_item(2, "c"),
            replay_item(3, "d"),
        ],
        ProviderSessionHistoryTotal::Exact(4),
    )
    .expect("older page is valid");
    let older_page =
        ProviderSessionHistoryPage::new(&plan, &older_request, older_window, CleanupOutcome::Clean)
            .expect("older page validates");

    assert_eq!(sequences(&older_page), [0, 1]);
    assert!(!older_page.has_older());
    assert!(older_page.older_cursor().is_none());
}

#[test]
fn cursor_from_another_plan_fails_closed() {
    let fixture = fixture();
    let first = history_plan(&fixture, "history-a", 2, 64, 8);
    let second = history_plan(&fixture, "history-b", 2, 64, 8);
    let first_request = ProviderSessionHistoryRequest::from_plan(
        RequestId::new("history-first").expect("request id is valid"),
        &first,
        None,
    )
    .expect("request is valid");
    let window = page_provider_session_history_window(
        &first,
        &first_request,
        vec![replay_item(0, "a"), replay_item(1, "b"), replay_item(2, "c")],
        ProviderSessionHistoryTotal::Exact(3),
    )
    .expect("window is valid");
    let cursor = window.older_cursor().expect("cursor").clone();

    let error = ProviderSessionHistoryRequest::from_plan(
        RequestId::new("history-mismatch").expect("request id is valid"),
        &second,
        Some(cursor),
    )
    .expect_err("foreign cursor must fail");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.provider_session_history.cursor_plan_mismatch"
    );
}

#[test]
fn snapshot_overflow_fails_closed() {
    let fixture = fixture();
    let plan = history_plan(&fixture, "history-a", 2, 64, 2);
    let request = ProviderSessionHistoryRequest::from_plan(
        RequestId::new("history-overflow").expect("request id is valid"),
        &plan,
        None,
    )
    .expect("request is valid");
    let error = page_provider_session_history_window(
        &plan,
        &request,
        vec![
            replay_item(0, "a"),
            replay_item(1, "b"),
            replay_item(2, "c"),
        ],
        ProviderSessionHistoryTotal::Exact(3),
    )
    .expect_err("snapshot overflow must fail");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.provider_session_history.snapshot_limit_exceeded"
    );
}

#[test]
fn totals_stay_honest_for_exact_at_least_and_unknown() {
    let fixture = fixture();
    let plan = history_plan(&fixture, "history-a", 2, 64, 8);
    let request = ProviderSessionHistoryRequest::from_plan(
        RequestId::new("history-total").expect("request id is valid"),
        &plan,
        None,
    )
    .expect("request is valid");
    let snapshot = vec![replay_item(0, "a"), replay_item(1, "b")];

    let exact = page_provider_session_history_window(
        &plan,
        &request,
        snapshot.clone(),
        ProviderSessionHistoryTotal::Exact(2),
    )
    .expect("exact total");
    assert_eq!(exact.total(), ProviderSessionHistoryTotal::Exact(2));

    let at_least = page_provider_session_history_window(
        &plan,
        &request,
        snapshot.clone(),
        ProviderSessionHistoryTotal::AtLeast(2),
    )
    .expect("at-least total");
    assert_eq!(at_least.total(), ProviderSessionHistoryTotal::AtLeast(2));

    let unknown = page_provider_session_history_window(
        &plan,
        &request,
        snapshot,
        ProviderSessionHistoryTotal::Unknown,
    )
    .expect("unknown total");
    assert_eq!(unknown.total(), ProviderSessionHistoryTotal::Unknown);

    let dishonest = page_provider_session_history_window(
        &plan,
        &request,
        vec![replay_item(0, "a")],
        ProviderSessionHistoryTotal::Exact(2),
    )
    .expect_err("dishonest exact total");
    assert_eq!(
        dishonest.diagnostic().code(),
        "swallowtail.provider_session_history.total_invalid"
    );
}

#[test]
fn oversized_next_item_fails_instead_of_empty_page() {
    let fixture = fixture();
    let plan = history_plan(&fixture, "history-a", 2, 64, 8);
    let request = ProviderSessionHistoryRequest::from_plan(
        RequestId::new("history-bytes").expect("request id is valid"),
        &plan,
        None,
    )
    .expect("request is valid");
    let oversized = "x".repeat(65);
    let error = page_provider_session_history_window(
        &plan,
        &request,
        vec![replay_item(0, &oversized)],
        ProviderSessionHistoryTotal::Exact(1),
    )
    .expect_err("oversized item must fail");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.provider_session_history.page_limit_exceeded"
    );
}

#[test]
fn cursor_value_is_opaque_in_debug() {
    let fixture = fixture();
    let plan = history_plan(&fixture, "history-a", 2, 64, 8);
    let cursor = ProviderSessionHistoryCursor::from_older_end(&plan, 2).expect("cursor");
    let debug = format!("{cursor:?}");
    assert!(debug.contains("<opaque:"));
    assert!(!debug.contains("provider/private/session"));
}
