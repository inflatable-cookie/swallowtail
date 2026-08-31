use crate::discovery_support::DiscoveryHost;
use crate::support::{FixtureHost, Scenario};

use futures_executor::block_on;
use std::collections::{BTreeMap, BTreeSet};
use swallowtail_adapter_kiro::{
    KIRO_CLI_EXECUTABLE_NAME, KIRO_CLI_RELEASE_AXIS, KIRO_CLI_RELEASE_VERSION,
    KiroPreparationInput, KiroPreparationProbe, KiroPreparedIntegration, KiroPreparedSession,
    KiroSessionProfileInput, kiro_local_account_access_profile, prepare_kiro_acp,
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

use super::ledger::{KIRO_PROFILES, SESSION};
use super::naming::{RowIdentity, identities, source};

const ACCESS_PROFILE_ID: &str = "kiro.projection.local-account";
const INSTANCE_ID: &str = "kiro.projection.instance";

/// Prepares the one read-only ACP session this route admits.
pub(super) fn session() -> KiroPreparedSession {
    prepared("1")
        .prepare_session(session_input("projection"))
        .expect("session prepares")
}

/// Prepares one session bound to a different exact configured revision.
pub(super) fn alternate_revision() -> KiroPreparedSession {
    prepared("2")
        .prepare_session(session_input("projection-alternate"))
        .expect("session prepares")
}

/// Prepares one qualified integration with the exact host services required.
pub(super) fn prepared(revision: &str) -> KiroPreparedIntegration {
    prepared_with(revision, ready_status()).expect("Kiro ACP prepares")
}

pub(super) fn prepared_with(
    revision: &str,
    status: AccessStatus,
) -> Result<KiroPreparedIntegration, swallowtail_runtime::PreparationFailure> {
    let host_id = ExecutionHostId::new("fixture.projection.local").expect("host id is valid");
    let discovery = DiscoveryHost::new(KIRO_CLI_RELEASE_VERSION);
    let operation = FixtureHost::new(Scenario::Success);
    let services = discovery.services(host_id.clone()).with_working_resource(
        operation
            .services(host_id.clone())
            .working_resource()
            .expect("resource service")
            .clone(),
    );
    block_on(prepare_kiro_acp(
        preparation_input(host_id, revision, status),
        probe(),
        services,
    ))
}

pub(super) fn contribution(
    session: &KiroPreparedSession,
    source_id: &str,
) -> ConsumerRouteProjectionContribution {
    session
        .consumer_route_projection_contribution(source(source_id))
        .expect("prepared Kiro session contributes")
}

/// Collects the exact census identities each prepared profile emits.
pub(super) fn observed_dispositions() -> BTreeMap<&'static str, BTreeSet<RowIdentity>> {
    BTreeMap::from([(
        SESSION,
        identities(&contribution(&session(), "kiro.acp.session")),
    )])
}

/// Returns the exact prepared operation shape each profile binds its rows to.
pub(super) fn prepared_operation_shapes() -> BTreeMap<&'static str, OperationShape> {
    assert_eq!(KIRO_PROFILES.len(), 1);
    BTreeMap::from([(
        SESSION,
        operation_shape_of(&contribution(&session(), "kiro.acp.shape")),
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

pub(super) fn session_input(id: &str) -> KiroSessionProfileInput {
    KiroSessionProfileInput::new(
        RequestId::new(format!("kiro.projection.{id}")).expect("request id is valid"),
        WorkingResourceRef::new("kiro.projection.workspace").expect("resource is valid"),
    )
}

fn preparation_input(
    host_id: ExecutionHostId,
    revision: &str,
    status: AccessStatus,
) -> KiroPreparationInput {
    KiroPreparationInput::new(
        ConfiguredInstanceId::new(INSTANCE_ID).expect("instance id is valid"),
        InstanceRevision::new(revision).expect("revision is valid"),
        host_id,
        target(),
        EnvironmentRef::new("kiro.projection.isolated").expect("environment is valid"),
        kiro_local_account_access_profile(
            AccessProfileId::new(ACCESS_PROFILE_ID).expect("profile id is valid"),
        ),
        PreparedAccessEvidence::caller_asserted(status),
    )
}

fn probe() -> KiroPreparationProbe {
    KiroPreparationProbe::new(
        RequestId::new("kiro.projection.probe").expect("request id is valid"),
        ScopeId::new("kiro.projection.probe").expect("scope id is valid"),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
        DiscoveryCancellation::new(),
    )
}

fn target() -> InstalledExecutableTarget {
    InstalledExecutableTarget::new(
        ExecutableRef::new(format!("/fixture/bin/{KIRO_CLI_EXECUTABLE_NAME}"))
            .expect("executable is valid"),
        InterfaceVersionAxis::new(KIRO_CLI_RELEASE_AXIS).expect("axis is valid"),
    )
}
