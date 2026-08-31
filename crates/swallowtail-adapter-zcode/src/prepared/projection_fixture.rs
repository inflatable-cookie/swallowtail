use super::{ZcodePreparationInput, ZcodePreparedIntegration, configured_instance, validate_input};
use crate::{
    ZCODE_EXECUTABLE_BASENAME, ZCODE_RELEASE_AXIS, ZCODE_RELEASE_VERSION, ZcodeAppServerMode,
    ZcodeModelSelection, ZcodePreparedRun, ZcodeRunProfileInput, zcode_access_profile,
    zcode_app_server_claim, zcode_release_binding,
};
use swallowtail_core::{
    AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialState, EndpointAuthorization,
    EntitlementState, ExecutionHostId, HostServiceKind, InstalledExecutableObservation,
    InstanceRevision, InterfaceVersionAxis, ModelId, ModelRouteId, ModelRouteRevision, ProviderId,
    RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    Deadline, EnvironmentRef, ExecutableRef, InstalledExecutableTarget, MonotonicInstant,
    OperationContent, PreparationFailure, PreparedAccessEvidence, RequestId, WorkingResourceRef,
};

/// Exact access profile every projection fixture prepares against.
pub(crate) const ACCESS_PROFILE_ID: &str = "zcode.projection.access";
/// Exact model route every projection fixture binds.
pub(crate) const MODEL_ROUTE_ID: &str = "zcode.projection.route";
/// Exact model identity every projection fixture binds.
pub(crate) const MODEL_ID: &str = "zcode-projection-model";
/// Exact provider identity every projection fixture binds.
pub(crate) const PROVIDER_ID: &str = "zai";

/// Prepares one app-server run bound to the exact supplied mode.
pub(crate) fn run(mode: ZcodeAppServerMode) -> ZcodePreparedRun {
    prepared("prepared-1")
        .prepare_run(run_input("projection", mode))
        .expect("explicit app-server run prepares")
}

/// Prepares one run bound to a different exact configured revision.
pub(crate) fn alternate_revision() -> ZcodePreparedRun {
    prepared("prepared-2")
        .prepare_run(run_input(
            "projection-alternate",
            ZcodeAppServerMode::plan(),
        ))
        .expect("explicit app-server run prepares")
}

pub(crate) fn prepared(revision: &str) -> ZcodePreparedIntegration {
    prepared_with(revision, ready_status()).expect("ZCode app-server prepares")
}

/// Promotes one qualified integration through the exact access validation.
///
/// Installed discovery checks the pinned payload digest against a real host
/// file, so the fixture builds the promoted record directly instead. Access
/// validation still runs, so drifted access evidence fails exactly as it does
/// through `prepare_zcode_app_server`.
pub(crate) fn prepared_with(
    revision: &str,
    status: AccessStatus,
) -> Result<ZcodePreparedIntegration, PreparationFailure> {
    let input = preparation_input(revision, status);
    validate_input(&input)?;
    let observation = InstalledExecutableObservation::classify(
        input.execution_host_id.clone(),
        zcode_release_binding(ZCODE_RELEASE_VERSION).expect("exact release binds"),
        &zcode_app_server_claim(),
    )
    .expect("the exact qualified release classifies");
    let instance = configured_instance(&input, &observation)?;
    Ok(ZcodePreparedIntegration {
        interpreter: input.interpreter,
        environment: input.environment,
        target: input.target,
        observation,
        access_profile: input.access_profile,
        access_evidence: input.access_evidence,
        instance,
        available_host_services: [
            HostServiceKind::Task,
            HostServiceKind::Process,
            HostServiceKind::Time,
        ]
        .into_iter()
        .collect(),
    })
}

pub(crate) fn run_input(id: &str, mode: ZcodeAppServerMode) -> ZcodeRunProfileInput {
    ZcodeRunProfileInput::new(
        RequestId::new(format!("zcode.projection.{id}")).expect("request id is valid"),
        ZcodeModelSelection::new(
            ModelRouteId::new(MODEL_ROUTE_ID).expect("route id is valid"),
            ModelRouteRevision::new("projection-1").expect("route revision is valid"),
            ProviderId::new(PROVIDER_ID).expect("provider id is valid"),
            ModelId::new(MODEL_ID).expect("model id is valid"),
        ),
        mode,
        OperationContent::new("private projection prompt").expect("prompt is valid"),
        WorkingResourceRef::new("zcode.projection.workspace").expect("resource is valid"),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
    )
}

pub(crate) fn ready_status() -> AccessStatus {
    status(
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    )
}

/// Builds one observed access snapshot with every dimension named explicitly.
pub(crate) fn status(
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
pub(crate) fn drifted_observations() -> [AccessStatus; 5] {
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

fn preparation_input(revision: &str, status: AccessStatus) -> ZcodePreparationInput {
    let access_id = AccessProfileId::new(ACCESS_PROFILE_ID).expect("profile id is valid");
    ZcodePreparationInput::new(
        ConfiguredInstanceId::new("zcode.projection.instance").expect("instance id is valid"),
        InstanceRevision::new(revision).expect("revision is valid"),
        ExecutionHostId::new("zcode.projection.host").expect("host id is valid"),
        ExecutableRef::new("/fixture/bin/node").expect("interpreter is valid"),
        InstalledExecutableTarget::new(
            ExecutableRef::new(format!("/fixture/vendor/{ZCODE_EXECUTABLE_BASENAME}"))
                .expect("target is valid"),
            InterfaceVersionAxis::new(ZCODE_RELEASE_AXIS).expect("axis is valid"),
        ),
        EnvironmentRef::new("/fixture/settings.json").expect("environment is valid"),
        zcode_access_profile(access_id),
        PreparedAccessEvidence::caller_asserted(status),
    )
}
