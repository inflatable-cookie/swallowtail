use futures_executor::block_on;
use swallowtail_adapter_antigravity::{
    ANTIGRAVITY_RELEASE_AXIS, AntigravityCatalogueProfileInput,
    AntigravityContinuationProfileInput, AntigravityHeadlessModelSelection,
    AntigravityHeadlessRunProfileInput, AntigravityPreparationInput, AntigravityPreparationProbe,
    AntigravityPreparedCatalogue, AntigravityPreparedContinuation, AntigravityPreparedDriver,
    AntigravityPreparedHeadlessRun, AntigravityPreparedIntegration,
    antigravity_personal_google_access_profile, prepare_antigravity,
};
use swallowtail_core::{
    AccessProfileId, AccessStatus, CredentialState, EndpointAuthorization, EntitlementState,
    ExecutionHostId, HarnessIsolation, InterfaceVersionAxis, ModelId, ModelRouteId,
    ModelRouteRevision, ProviderId, ReasoningMode, ResourceAccess, RuntimeReadiness,
    SupportAuthority,
};
use swallowtail_runtime::{
    Deadline, DiscoveryCancellation, EnvironmentRef, ExecutableRef, InstalledExecutableTarget,
    MonotonicInstant, OperationContent, PreparedAccessEvidence, ProcessOutputChunk,
    ProcessOutputStream, RequestId, SchemaDocument, ScopeId, StructuredOutputDescriptor,
    WorkingResourceRef,
};

#[path = "../support.rs"]
mod support;
use support::FixtureHost;

const VERSION: &str = "1.1.17\n";
const SCHEMA: &str = r#"{"type":"object","properties":{"summary":{"type":"string"}}}"#;

fn stdout(output: &str) -> ProcessOutputChunk {
    ProcessOutputChunk::new(ProcessOutputStream::Stdout, output.as_bytes().to_vec())
}

fn host_id() -> ExecutionHostId {
    ExecutionHostId::new("antigravity.fixture.host").expect("host id")
}

fn working_resource() -> WorkingResourceRef {
    WorkingResourceRef::new("workspace.main").expect("resource")
}

fn deadline() -> Deadline {
    Deadline::at(MonotonicInstant::from_ticks(1_000))
}

fn probe() -> AntigravityPreparationProbe {
    AntigravityPreparationProbe::new(
        RequestId::new("probe").expect("request id"),
        ScopeId::new("antigravity-preparation-probe").expect("scope"),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}

fn evidence(access_id: AccessProfileId) -> PreparedAccessEvidence {
    PreparedAccessEvidence::caller_asserted(AccessStatus::new(
        access_id,
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    ))
}

fn prepare(driver: AntigravityPreparedDriver) -> AntigravityPreparedIntegration {
    let host = host_id();
    let access_id = AccessProfileId::new("antigravity.fixture.access").expect("access id");
    let input = AntigravityPreparationInput::new(
        driver,
        swallowtail_core::ConfiguredInstanceId::new(format!("antigravity.fixture.{driver:?}"))
            .expect("instance"),
        swallowtail_core::InstanceRevision::new("1").expect("revision"),
        host.clone(),
        InstalledExecutableTarget::new(
            ExecutableRef::new("antigravity.fixture.executable").expect("executable"),
            InterfaceVersionAxis::new(ANTIGRAVITY_RELEASE_AXIS).expect("axis"),
        ),
        EnvironmentRef::new("antigravity.fixture.environment").expect("environment"),
        antigravity_personal_google_access_profile(access_id.clone()),
        evidence(access_id),
    );
    let fixture_host = FixtureHost::completed([stdout(VERSION)]);
    block_on(prepare_antigravity(
        input,
        probe(),
        fixture_host.services(host),
    ))
    .expect("Antigravity prepares")
}

fn model() -> AntigravityHeadlessModelSelection {
    AntigravityHeadlessModelSelection::new(
        ModelRouteId::new("antigravity.fixture.route").expect("route"),
        ModelRouteRevision::new("1").expect("revision"),
        ProviderId::new("google").expect("provider"),
        ModelId::new("gemini-3.6-flash-high").expect("model"),
    )
}

pub fn catalogue() -> AntigravityPreparedCatalogue {
    let integration = prepare(AntigravityPreparedDriver::Catalogue);
    let AntigravityPreparedIntegration::Catalogue(catalogue) = integration else {
        panic!("expected catalogue integration");
    };
    let input = AntigravityCatalogueProfileInput::new(RequestId::new("cat").expect("request id"));
    catalogue
        .prepare_catalogue(input)
        .expect("catalogue prepares")
}

pub fn headless_maximal() -> AntigravityPreparedHeadlessRun {
    let integration = prepare(AntigravityPreparedDriver::Headless);
    let AntigravityPreparedIntegration::Headless(headless) = integration else {
        panic!("expected headless integration");
    };
    let structured = StructuredOutputDescriptor::new(
        SchemaDocument::Inline(SCHEMA.as_bytes().to_vec()),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .expect("schema descriptor");
    let input = AntigravityHeadlessRunProfileInput::new(
        RequestId::new("headless-max").expect("request id"),
        model(),
        OperationContent::new("prompt").expect("prompt"),
        working_resource(),
        ResourceAccess::ReadWrite,
        HarnessIsolation::ProviderEnforced,
        deadline(),
    )
    .with_effort(ReasoningMode::new("high").expect("effort"))
    .with_structured_output(structured);
    headless.prepare_run(input).expect("headless run prepares")
}

pub fn headless_minimal() -> AntigravityPreparedHeadlessRun {
    let integration = prepare(AntigravityPreparedDriver::Headless);
    let AntigravityPreparedIntegration::Headless(headless) = integration else {
        panic!("expected headless integration");
    };
    let input = AntigravityHeadlessRunProfileInput::new(
        RequestId::new("headless-min").expect("request id"),
        model(),
        OperationContent::new("prompt").expect("prompt"),
        working_resource(),
        ResourceAccess::Read,
        HarnessIsolation::AmbientHost,
        deadline(),
    );
    headless.prepare_run(input).expect("headless run prepares")
}

pub fn continuation() -> AntigravityPreparedContinuation {
    let integration = prepare(AntigravityPreparedDriver::Continuation);
    let AntigravityPreparedIntegration::Continuation(continuation) = integration else {
        panic!("expected continuation integration");
    };
    let input = AntigravityContinuationProfileInput::new(
        RequestId::new("continuation").expect("request id"),
        model(),
        working_resource(),
    );
    continuation
        .prepare_session(input)
        .expect("continuation prepares")
}
