use crate::discovery_support::DiscoveryHost;
use crate::headless_support::FixtureHost;

use futures_executor::block_on;
use std::collections::{BTreeMap, BTreeSet};
use swallowtail_adapter_qoder::{
    QODER_EXECUTABLE_NAME, QODER_PACKAGE_AXIS, QODER_PACKAGE_VERSION,
    QoderHeadlessPreparationInput, QoderHeadlessPreparationProbe, QoderHeadlessPreparedIntegration,
    QoderHeadlessPreparedRun, QoderHeadlessRunProfileInput, prepare_qoder_headless,
    qoder_local_config_access_profile,
};
use swallowtail_core::{
    AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialState, EndpointAuthorization,
    EntitlementState, ExecutionHostId, InstanceRevision, InterfaceVersionAxis, OperationShape,
    RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    ConsumerRouteProjectionContribution, Deadline, DiscoveryCancellation, EnvironmentRef,
    ExecutableRef, InstalledExecutableTarget, MonotonicInstant, OperationContent,
    PreparedAccessEvidence, RequestId, ScopeId, WorkingResourceRef,
};

use super::ledger::{QODER_PROFILES, RUN};
use super::naming::{RowIdentity, identities, source};

const ACCESS_PROFILE_ID: &str = "qoder.projection.local-config";
const INSTANCE_ID: &str = "qoder.projection.instance";

/// Prepares the one bounded print run this route admits.
pub(super) fn run() -> QoderHeadlessPreparedRun {
    prepared("1")
        .prepare_run(run_input("projection"))
        .expect("run prepares")
}

/// Prepares one run bound to a different exact configured revision.
pub(super) fn alternate_revision() -> QoderHeadlessPreparedRun {
    prepared("2")
        .prepare_run(run_input("projection-alternate"))
        .expect("run prepares")
}

/// Prepares one qualified integration with the exact host services required.
pub(super) fn prepared(revision: &str) -> QoderHeadlessPreparedIntegration {
    prepared_with(revision, ready_status()).expect("Qoder headless prepares")
}

pub(super) fn prepared_with(
    revision: &str,
    status: AccessStatus,
) -> Result<QoderHeadlessPreparedIntegration, swallowtail_runtime::PreparationFailure> {
    let host_id = ExecutionHostId::new("fixture.projection.local").expect("host id is valid");
    let discovery = DiscoveryHost::new(QODER_PACKAGE_VERSION);
    let operation = FixtureHost::scripted([]);
    let services = discovery.services(host_id.clone()).with_working_resource(
        operation
            .services(host_id.clone())
            .working_resource()
            .expect("resource service")
            .clone(),
    );
    block_on(prepare_qoder_headless(
        preparation_input(host_id, revision, status),
        probe(),
        services,
    ))
}

pub(super) fn contribution(
    run: &QoderHeadlessPreparedRun,
    source_id: &str,
) -> ConsumerRouteProjectionContribution {
    run.consumer_route_projection_contribution(source(source_id))
        .expect("prepared Qoder headless run contributes")
}

/// Collects the exact census identities each prepared profile emits.
pub(super) fn observed_dispositions() -> BTreeMap<&'static str, BTreeSet<RowIdentity>> {
    BTreeMap::from([(RUN, identities(&contribution(&run(), "qoder.headless.run")))])
}

/// Returns the exact prepared operation shape each profile binds its rows to.
pub(super) fn prepared_operation_shapes() -> BTreeMap<&'static str, OperationShape> {
    assert_eq!(QODER_PROFILES.len(), 1);
    BTreeMap::from([(
        RUN,
        operation_shape_of(&contribution(&run(), "qoder.headless.shape")),
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

pub(super) fn run_input(id: &str) -> QoderHeadlessRunProfileInput {
    QoderHeadlessRunProfileInput::new(
        RequestId::new(format!("qoder.projection.{id}")).expect("request id is valid"),
        OperationContent::new("private projection prompt").expect("prompt is valid"),
        WorkingResourceRef::new("qoder.projection.workspace").expect("resource is valid"),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
    )
}

fn preparation_input(
    host_id: ExecutionHostId,
    revision: &str,
    status: AccessStatus,
) -> QoderHeadlessPreparationInput {
    QoderHeadlessPreparationInput::new(
        ConfiguredInstanceId::new(INSTANCE_ID).expect("instance id is valid"),
        InstanceRevision::new(revision).expect("revision is valid"),
        host_id,
        target(),
        EnvironmentRef::new("qoder.projection.isolated").expect("environment is valid"),
        qoder_local_config_access_profile(
            AccessProfileId::new(ACCESS_PROFILE_ID).expect("profile id is valid"),
        ),
        PreparedAccessEvidence::caller_asserted(status),
    )
}

fn probe() -> QoderHeadlessPreparationProbe {
    QoderHeadlessPreparationProbe::new(
        RequestId::new("qoder.projection.probe").expect("request id is valid"),
        ScopeId::new("qoder.projection.probe").expect("scope id is valid"),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
        DiscoveryCancellation::new(),
    )
}

fn target() -> InstalledExecutableTarget {
    InstalledExecutableTarget::new(
        ExecutableRef::new(format!("/fixture/bin/{QODER_EXECUTABLE_NAME}"))
            .expect("executable is valid"),
        InterfaceVersionAxis::new(QODER_PACKAGE_AXIS).expect("axis is valid"),
    )
}
