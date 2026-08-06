use super::support;
use futures_executor::block_on;
use swallowtail_adapter_muse::{
    MUSE_CODE_PAYLOAD_BASENAME, MUSE_CODE_RELEASE_AXIS, MUSE_META_PROVIDER_ID, MUSE_SPARK_MODEL_ID,
    MuseHeadlessModelSelection, MusePreparationInput, MusePreparationProbe, MuseRunProfileInput,
    muse_local_meta_account_access_profile, prepare_muse_headless,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialState,
    EndpointAuthorization, EntitlementState, ExecutionHostId, InstanceRevision,
    InterfaceVersionAxis, ModelId, ModelRouteId, ModelRouteRevision, ProviderId, ReasoningMode,
    RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, EnvironmentRef, ExecutableRef, InstalledExecutableTarget,
    MonotonicInstant, OperationContent, PreparedAccessEvidence, RequestId, ScopeId,
    WorkingResourceRef,
};

pub(super) const ARTIFACT: &str = include_str!("../fixtures/muse-code-0.1.0-R708.1/artifact.json");
pub(super) const PROTOCOL: &str = include_str!("../fixtures/muse-code-0.1.0-R708.1/protocol.json");
pub(super) const VERSION: &str = include_str!("../fixtures/muse-code-0.1.0-R708.1/version.txt");
pub(super) const SUCCESS: &str =
    include_str!("../fixtures/muse-code-0.1.0-R708.1/meta-success.jsonl");

pub(super) fn prepare(
    host_id: ExecutionHostId,
) -> swallowtail_adapter_muse::MusePreparedIntegration {
    let access_id = AccessProfileId::new("muse.fixture.access").expect("access id");
    let host = support::FixtureHost::scripted([VERSION]);
    let prepared = block_on(prepare_muse_headless(
        preparation_input(
            host_id.clone(),
            muse_local_meta_account_access_profile(access_id.clone()),
            evidence(access_id),
        ),
        probe(),
        host.services(host_id),
    ))
    .expect("Muse Code prepares");
    assert_eq!(host.observations().len(), 1);
    assert_eq!(host.observations()[0].arguments, ["--version"]);
    prepared
}

pub(super) fn preparation_input(
    host_id: ExecutionHostId,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
) -> MusePreparationInput {
    MusePreparationInput::new(
        ConfiguredInstanceId::new("muse.fixture.instance").expect("instance"),
        InstanceRevision::new("1").expect("instance revision"),
        host_id,
        InstalledExecutableTarget::new(
            ExecutableRef::new(format!("/fixture/bin/{MUSE_CODE_PAYLOAD_BASENAME}"))
                .expect("executable"),
            InterfaceVersionAxis::new(MUSE_CODE_RELEASE_AXIS).expect("axis"),
        ),
        EnvironmentRef::new("muse.fixture.environment").expect("environment"),
        access_profile,
        access_evidence,
    )
}

pub(super) fn run_input(model: MuseHeadlessModelSelection, effort: &str) -> MuseRunProfileInput {
    MuseRunProfileInput::new(
        RequestId::new(format!("muse.fixture.run.{effort}")).expect("request"),
        model,
        OperationContent::new("private prompt").expect("prompt"),
        ReasoningMode::new(effort).expect("effort"),
        WorkingResourceRef::new("muse.fixture.workspace").expect("resource"),
        deadline(),
    )
}

pub(super) fn model() -> MuseHeadlessModelSelection {
    MuseHeadlessModelSelection::new(
        ModelRouteId::new("muse.fixture.route").expect("route"),
        ModelRouteRevision::new("1").expect("route revision"),
        ProviderId::new(MUSE_META_PROVIDER_ID).expect("provider"),
        ModelId::new(MUSE_SPARK_MODEL_ID).expect("model"),
    )
}

pub(super) fn evidence(access_id: AccessProfileId) -> PreparedAccessEvidence {
    PreparedAccessEvidence::caller_asserted(AccessStatus::new(
        access_id,
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    ))
}

pub(super) fn probe() -> MusePreparationProbe {
    MusePreparationProbe::new(
        RequestId::new("muse.fixture.probe").expect("request"),
        ScopeId::new("muse.fixture.probe").expect("scope"),
        deadline(),
        DiscoveryCancellation::new(),
    )
}

pub(super) fn host_id() -> ExecutionHostId {
    ExecutionHostId::new("muse.fixture.host").expect("host")
}

pub(super) fn deadline() -> Deadline {
    Deadline::at(MonotonicInstant::from_ticks(1_000))
}

pub(super) fn exact_model(effort: &str) -> MuseHeadlessModelSelection {
    MuseHeadlessModelSelection::new(
        ModelRouteId::new(format!("muse.fixture.{effort}")).expect("route"),
        ModelRouteRevision::new("1").expect("route revision"),
        ProviderId::new(MUSE_META_PROVIDER_ID).expect("provider"),
        ModelId::new(MUSE_SPARK_MODEL_ID).expect("model"),
    )
}
