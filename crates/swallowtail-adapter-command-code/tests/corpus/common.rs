use super::support;
use futures_executor::block_on;
use swallowtail_adapter_command_code::{
    COMMAND_CODE_EXECUTABLE_NAME, COMMAND_CODE_RELEASE_AXIS, CommandCodeHeadlessModelSelection,
    CommandCodePreparationInput, CommandCodePreparationProbe, CommandCodeRunProfileInput,
    command_code_local_account_access_profile, prepare_command_code_headless,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialState,
    EndpointAuthorization, EntitlementState, ExecutionHostId, InstanceRevision,
    InterfaceVersionAxis, ModelId, ModelRouteId, ModelRouteRevision, ProviderId, RuntimeReadiness,
    SupportAuthority,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, EnvironmentRef, ExecutableRef, InstalledExecutableTarget,
    MonotonicInstant, OperationContent, PreparedAccessEvidence, RequestId, ScopeId,
    WorkingResourceRef,
};

pub(super) const ARTIFACT: &str = include_str!("../fixtures/command-code-1.15.1/artifact.json");
pub(super) const PROTOCOL: &str = include_str!("../fixtures/command-code-1.15.1/protocol.json");
pub(super) const VERSION: &str = include_str!("../fixtures/command-code-1.15.1/version.txt");
pub(super) const NO_TOOL_SUCCESS: &str =
    include_str!("../fixtures/command-code-1.15.1/no-tool-success.jsonl");
pub(super) const TOOL_SUCCESS: &str =
    include_str!("../fixtures/command-code-1.15.1/tool-success.jsonl");
pub(super) const CREDIT_FAILURE: &str =
    include_str!("../fixtures/command-code-1.15.1/credit-failure.jsonl");
pub(super) const UNKNOWN_EVENT: &str =
    include_str!("../fixtures/command-code-1.15.1/unknown-event.jsonl");

pub(super) const FIXTURE_PROVIDER_ID: &str = "fixture-provider";
pub(super) const FIXTURE_MODEL_ID: &str = "fixture-model";

pub(super) fn prepare(
    host_id: ExecutionHostId,
) -> swallowtail_adapter_command_code::CommandCodePreparedIntegration {
    let access_id = AccessProfileId::new("command-code.fixture.access").expect("access id");
    let host = support::FixtureHost::scripted([VERSION]);
    let prepared = block_on(prepare_command_code_headless(
        preparation_input(
            host_id.clone(),
            command_code_local_account_access_profile(access_id.clone()),
            evidence(access_id),
        ),
        probe(),
        host.services(host_id),
    ))
    .expect("Command Code prepares");
    assert_eq!(host.observations().len(), 1);
    assert_eq!(host.observations()[0].arguments, ["--version"]);
    prepared
}

pub(super) fn preparation_input(
    host_id: ExecutionHostId,
    access_profile: AccessProfile,
    access_evidence: PreparedAccessEvidence,
) -> CommandCodePreparationInput {
    CommandCodePreparationInput::new(
        ConfiguredInstanceId::new("command-code.fixture.instance").expect("instance"),
        InstanceRevision::new("1").expect("instance revision"),
        host_id,
        InstalledExecutableTarget::new(
            ExecutableRef::new(format!("/fixture/bin/{COMMAND_CODE_EXECUTABLE_NAME}"))
                .expect("executable"),
            InterfaceVersionAxis::new(COMMAND_CODE_RELEASE_AXIS).expect("axis"),
        ),
        EnvironmentRef::new("command-code.fixture.environment").expect("environment"),
        access_profile,
        access_evidence,
    )
}

pub(super) fn run_input(
    model: CommandCodeHeadlessModelSelection,
    id: &str,
) -> CommandCodeRunProfileInput {
    CommandCodeRunProfileInput::new(
        RequestId::new(format!("command-code.fixture.run.{id}")).expect("request"),
        model,
        OperationContent::new("private prompt").expect("prompt"),
        WorkingResourceRef::new("command-code.fixture.workspace").expect("resource"),
        deadline(),
    )
}

pub(super) fn model() -> CommandCodeHeadlessModelSelection {
    CommandCodeHeadlessModelSelection::new(
        ModelRouteId::new("command-code.fixture.route").expect("route"),
        ModelRouteRevision::new("1").expect("route revision"),
        ProviderId::new(FIXTURE_PROVIDER_ID).expect("provider"),
        ModelId::new(FIXTURE_MODEL_ID).expect("model"),
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

pub(super) fn probe() -> CommandCodePreparationProbe {
    CommandCodePreparationProbe::new(
        RequestId::new("command-code.fixture.probe").expect("request"),
        ScopeId::new("command-code.fixture.probe").expect("scope"),
        deadline(),
        DiscoveryCancellation::new(),
    )
}

pub(super) fn host_id() -> ExecutionHostId {
    ExecutionHostId::new("command-code.fixture.host").expect("host")
}

pub(super) fn deadline() -> Deadline {
    Deadline::at(MonotonicInstant::from_ticks(1_000))
}
