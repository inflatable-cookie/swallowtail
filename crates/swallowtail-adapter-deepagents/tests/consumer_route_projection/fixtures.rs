use crate::discovery_support::DiscoveryHost;
use crate::support::{FixtureHost, Scenario};

use futures_executor::block_on;
use std::collections::{BTreeMap, BTreeSet};
use swallowtail_adapter_deepagents::{
    DEEPAGENTS_ACP_EXECUTABLE_NAME, DEEPAGENTS_ACP_PACKAGE_AXIS, DEEPAGENTS_ACP_PACKAGE_VERSION,
    DeepAgentsPreparationInput, DeepAgentsPreparationProbe, DeepAgentsPreparedIntegration,
    DeepAgentsPreparedSession, DeepAgentsSessionProfileInput,
    deepagents_provider_api_key_access_profile, prepare_deepagents_acp,
};
use swallowtail_core::{
    AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialState, EndpointAuthorization,
    EntitlementState, ExecutionHostId, InstanceRevision, InterfaceVersionAxis, OperationShape,
    RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    ConsumerRouteProjectionContribution, Deadline, DiscoveryCancellation, EnvironmentRef,
    ExecutableRef, InstalledExecutableTarget, MonotonicInstant, PreparedAccessEvidence, RequestId,
    ScopeId, WorkingResourceRef,
};

use super::ledger::{DEEPAGENTS_PROFILES, SESSION};
use super::naming::{RowIdentity, identities, source};

const ACCESS_PROFILE_ID: &str = "deepagents.projection.provider-api-key";
const INSTANCE_ID: &str = "deepagents.projection.instance";

/// Prepares the one read-only ACP session this route admits.
pub(super) fn session() -> DeepAgentsPreparedSession {
    prepared("1")
        .prepare_session(session_input("projection"))
        .expect("session prepares")
}

/// Prepares one session bound to a different exact configured revision.
pub(super) fn alternate_revision() -> DeepAgentsPreparedSession {
    prepared("2")
        .prepare_session(session_input("projection-alternate"))
        .expect("session prepares")
}

/// Prepares one qualified integration with the exact host services required.
pub(super) fn prepared(revision: &str) -> DeepAgentsPreparedIntegration {
    prepared_with(revision, ready_status()).expect("Deep Agents ACP prepares")
}

pub(super) fn prepared_with(
    revision: &str,
    status: AccessStatus,
) -> Result<DeepAgentsPreparedIntegration, swallowtail_runtime::PreparationFailure> {
    let host_id = ExecutionHostId::new("fixture.projection.local").expect("host id is valid");
    let discovery = DiscoveryHost::new(DEEPAGENTS_ACP_PACKAGE_VERSION);
    let operation = FixtureHost::new(Scenario::Success);
    let services = discovery.services(host_id.clone()).with_working_resource(
        operation
            .services(host_id.clone())
            .working_resource()
            .expect("resource service")
            .clone(),
    );
    block_on(prepare_deepagents_acp(
        preparation_input(host_id, revision, status),
        probe(),
        services,
    ))
}

pub(super) fn contribution(
    session: &DeepAgentsPreparedSession,
    source_id: &str,
) -> ConsumerRouteProjectionContribution {
    session
        .consumer_route_projection_contribution(source(source_id))
        .expect("prepared Deep Agents session contributes")
}

/// Collects the exact census identities each prepared profile emits.
pub(super) fn observed_dispositions() -> BTreeMap<&'static str, BTreeSet<RowIdentity>> {
    BTreeMap::from([(
        SESSION,
        identities(&contribution(&session(), "deepagents.acp.session")),
    )])
}

/// Returns the exact prepared operation shape each profile binds its rows to.
pub(super) fn prepared_operation_shapes() -> BTreeMap<&'static str, OperationShape> {
    assert_eq!(DEEPAGENTS_PROFILES.len(), 1);
    BTreeMap::from([(
        SESSION,
        operation_shape_of(&contribution(&session(), "deepagents.acp.shape")),
    )])
}

/// Returns the operation shape every row of one contribution is bound to.
fn operation_shape_of(contribution: &ConsumerRouteProjectionContribution) -> OperationShape {
    for row in contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
    {
        assert_eq!(
            row.applicability(),
            contribution.applicability(),
            "{:?} is not bound to the contribution's exact applicability",
            row.identity()
        );
    }
    contribution.applicability().operation_shape()
}

pub(super) fn ready_status() -> AccessStatus {
    status(
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    )
}

/// Builds one observed access snapshot with every dimension named explicitly.
pub(super) fn status(
    credential: CredentialState,
    entitlement: EntitlementState,
    endpoint: EndpointAuthorization,
    readiness: RuntimeReadiness,
    authority: SupportAuthority,
) -> AccessStatus {
    AccessStatus::new(
        AccessProfileId::new(ACCESS_PROFILE_ID).expect("profile id is valid"),
        credential,
        entitlement,
        endpoint,
        readiness,
        authority,
    )
}

/// Returns one drifted observation per exact access dimension.
pub(super) fn drifted_observations() -> [AccessStatus; 5] {
    [
        status(
            CredentialState::Expired,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        ),
        status(
            CredentialState::NotRequired,
            EntitlementState::Exhausted,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        ),
        status(
            CredentialState::NotRequired,
            EntitlementState::Available,
            EndpointAuthorization::Denied,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        ),
        status(
            CredentialState::NotRequired,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Degraded,
            SupportAuthority::ProviderSupported,
        ),
        status(
            CredentialState::NotRequired,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ExperimentalObserved,
        ),
    ]
}

pub(super) fn session_input(id: &str) -> DeepAgentsSessionProfileInput {
    DeepAgentsSessionProfileInput::new(
        RequestId::new(format!("deepagents.projection.{id}")).expect("request id is valid"),
        WorkingResourceRef::new("deepagents.projection.workspace").expect("resource is valid"),
    )
}

fn preparation_input(
    host_id: ExecutionHostId,
    revision: &str,
    status: AccessStatus,
) -> DeepAgentsPreparationInput {
    DeepAgentsPreparationInput::new(
        ConfiguredInstanceId::new(INSTANCE_ID).expect("instance id is valid"),
        InstanceRevision::new(revision).expect("revision is valid"),
        host_id,
        target(),
        EnvironmentRef::new("deepagents.projection.isolated").expect("environment is valid"),
        deepagents_provider_api_key_access_profile(
            AccessProfileId::new(ACCESS_PROFILE_ID).expect("profile id is valid"),
        ),
        PreparedAccessEvidence::caller_asserted(status),
    )
}

fn probe() -> DeepAgentsPreparationProbe {
    DeepAgentsPreparationProbe::new(
        RequestId::new("deepagents.projection.probe").expect("request id is valid"),
        ScopeId::new("deepagents.projection.probe").expect("scope id is valid"),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
        DiscoveryCancellation::new(),
    )
}

fn target() -> InstalledExecutableTarget {
    InstalledExecutableTarget::new(
        ExecutableRef::new(format!("/fixture/bin/{DEEPAGENTS_ACP_EXECUTABLE_NAME}"))
            .expect("executable is valid"),
        InterfaceVersionAxis::new(DEEPAGENTS_ACP_PACKAGE_AXIS).expect("axis is valid"),
    )
}
