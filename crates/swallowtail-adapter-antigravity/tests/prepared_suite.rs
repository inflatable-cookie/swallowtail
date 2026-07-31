mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use support::FixtureHost;
use swallowtail_adapter_antigravity::{
    ANTIGRAVITY_RELEASE_AXIS, AntigravityCatalogueProfileInput,
    AntigravityContinuationProfileInput, AntigravityHeadlessModelSelection,
    AntigravityHeadlessRunProfileInput, AntigravityPreparationInput, AntigravityPreparationProbe,
    AntigravityPreparedDriver, AntigravityPreparedIntegration,
    antigravity_personal_google_access_profile, prepare_antigravity,
};
use swallowtail_core::{
    AccessProfileId, AccessStatus, CredentialState, EndpointAuthorization, EntitlementState,
    ExecutionHostId, HarnessIsolation, InterfaceVersionAxis, ModelId, ModelRouteId,
    ModelRouteRevision, ObservableActivityAvailability, ProviderId, ReasoningMode, ResourceAccess,
    RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, DiscoveryCancellation, EnvironmentRef, ExecutableRef,
    InstalledExecutableTarget, MonotonicInstant, OperationContent, PreparedAccessEvidence,
    ProcessOutputChunk, ProcessOutputStream, RequestId, RuntimeTurnId, SchemaDocument, ScopeId,
    StructuredOutputDescriptor, TerminalStatus, TurnRequest, WorkingResourceRef,
};
use swallowtail_testkit::assert_prepared_operation_evidence_matches_plan;

const VERSION: &str = include_str!("fixtures/antigravity-cli-1.1.9/version.txt");
const CATALOGUE: &str = include_str!("fixtures/antigravity-cli-1.1.9/models.txt");
const STRUCTURED: &str = include_str!("fixtures/antigravity-cli-1.1.9/headless-structured.jsonl");
const FIRST: &str = include_str!("fixtures/antigravity-cli-1.1.9/continuation-first.jsonl");
const SECOND: &str = include_str!("fixtures/antigravity-cli-1.1.9/continuation-second.jsonl");
const SCHEMA: &str =
    r#"{"type":"object","properties":{"answer":{"type":"string"}},"required":["answer"]}"#;

#[test]
fn facade_keeps_catalogue_run_and_continuation_explicit() {
    let host_id = host_id();
    let catalogue = prepare(AntigravityPreparedDriver::Catalogue, host_id.clone());
    let AntigravityPreparedIntegration::Catalogue(catalogue) = catalogue else {
        panic!("catalogue branch remains explicit");
    };
    let catalogue = catalogue
        .prepare_catalogue(AntigravityCatalogueProfileInput::new(request_id(
            "catalogue",
        )))
        .expect("catalogue prepares");
    assert_prepared_operation_evidence_matches_plan(catalogue.evidence(), catalogue.plan());
    let catalogue_host = FixtureHost::completed([stdout(CATALOGUE)]);
    let models = block_on(catalogue.list_models(catalogue_host.services(host_id.clone())))
        .expect("catalogue executes");
    assert_eq!(models.len(), 11);

    let headless = prepare(AntigravityPreparedDriver::Headless, host_id.clone());
    let AntigravityPreparedIntegration::Headless(headless) = headless else {
        panic!("headless branch remains explicit");
    };
    let run = headless
        .prepare_run(
            AntigravityHeadlessRunProfileInput::new(
                request_id("headless"),
                model(),
                OperationContent::new("private structured prompt").expect("prompt"),
                working_resource(),
                ResourceAccess::ReadWrite,
                HarnessIsolation::ProviderEnforced,
                deadline(),
            )
            .with_effort(ReasoningMode::new("high").expect("effort"))
            .with_structured_output(
                StructuredOutputDescriptor::new(
                    SchemaDocument::Inline(SCHEMA.as_bytes().to_vec()),
                    "application/schema+json",
                    "json-schema-2020-12",
                )
                .expect("schema descriptor"),
            ),
        )
        .expect("headless run prepares");
    assert_prepared_operation_evidence_matches_plan(run.evidence(), run.plan());
    assert_eq!(
        run.evidence().observable_activity().availability(),
        ObservableActivityAvailability::Available
    );
    let run_host = FixtureHost::completed([stdout(STRUCTURED)]);
    let mut handle =
        block_on(run.start_run(run_host.services(host_id.clone()))).expect("prepared run starts");
    let events = block_on(handle.take_events().expect("events").collect::<Vec<_>>());
    let terminal = block_on(handle.take_terminal_outcome().expect("terminal"));
    assert!(!events.is_empty());
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
    let arguments = run_host.observed().arguments;
    for exact in ["--sandbox", "--effort", "high", "--json-schema"] {
        assert!(arguments.iter().any(|argument| argument == exact));
    }
    assert!(
        !arguments
            .iter()
            .any(|argument| argument == "--dangerously-skip-permissions")
    );

    let continuation = prepare(AntigravityPreparedDriver::Continuation, host_id.clone());
    let AntigravityPreparedIntegration::Continuation(continuation) = continuation else {
        panic!("continuation branch remains explicit");
    };
    let continuation = continuation
        .prepare_session(AntigravityContinuationProfileInput::new(
            request_id("continuation"),
            model(),
            working_resource(),
        ))
        .expect("continuation prepares");
    assert_prepared_operation_evidence_matches_plan(continuation.evidence(), continuation.plan());
    assert_eq!(
        continuation.evidence().observable_activity().availability(),
        ObservableActivityAvailability::Available
    );
    let continuation_host = FixtureHost::scripted(&[FIRST, SECOND]);
    let services = continuation_host.services(host_id);
    let mut session =
        block_on(continuation.open_session(services.clone())).expect("continuation session opens");
    for (index, prompt) in ["first", "second"].into_iter().enumerate() {
        let mut turn = block_on(
            session.start_turn(
                TurnRequest::new(
                    RuntimeTurnId::new(format!("prepared-turn-{index}")).expect("turn"),
                    OperationContent::new(prompt).expect("prompt"),
                )
                .with_deadline(deadline()),
                services.clone(),
            ),
        )
        .expect("turn starts");
        let terminal = block_on(turn.take_terminal_outcome().expect("terminal"));
        assert_eq!(terminal.status(), &TerminalStatus::Completed);
        assert_eq!(block_on(turn.close()), CleanupOutcome::Clean);
    }
    let observations = continuation_host.observations();
    assert!(
        !observations[0]
            .arguments
            .iter()
            .any(|argument| argument == "--conversation")
    );
    assert!(
        observations[1]
            .arguments
            .windows(2)
            .any(|pair| pair == ["--conversation", "fixture-conversation"])
    );
    assert_eq!(block_on(session.close()), CleanupOutcome::Clean);
}

#[test]
fn preparation_rejects_axis_drift_before_discovery() {
    let host_id = host_id();
    let access_id = AccessProfileId::new("antigravity.fixture.access").expect("access id");
    let host = FixtureHost::completed([stdout(VERSION)]);
    let input = AntigravityPreparationInput::new(
        AntigravityPreparedDriver::Headless,
        swallowtail_core::ConfiguredInstanceId::new("antigravity.fixture").expect("instance"),
        swallowtail_core::InstanceRevision::new("1").expect("revision"),
        host_id.clone(),
        InstalledExecutableTarget::new(
            ExecutableRef::new("antigravity.fixture.executable").expect("executable"),
            InterfaceVersionAxis::new("wrong.axis").expect("axis"),
        ),
        EnvironmentRef::new("antigravity.fixture.environment").expect("environment"),
        antigravity_personal_google_access_profile(access_id.clone()),
        evidence(access_id),
    );
    let error = block_on(prepare_antigravity(input, probe(), host.services(host_id)))
        .expect_err("axis drift fails");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.antigravity.preparation.target_axis_mismatch"
    );
    assert!(!host.started());
}

fn prepare(
    driver: AntigravityPreparedDriver,
    host_id: ExecutionHostId,
) -> AntigravityPreparedIntegration {
    let access_id = AccessProfileId::new("antigravity.fixture.access").expect("access id");
    let input = AntigravityPreparationInput::new(
        driver,
        swallowtail_core::ConfiguredInstanceId::new(format!("antigravity.fixture.{driver:?}"))
            .expect("instance"),
        swallowtail_core::InstanceRevision::new("1").expect("revision"),
        host_id.clone(),
        InstalledExecutableTarget::new(
            ExecutableRef::new("antigravity.fixture.executable").expect("executable"),
            InterfaceVersionAxis::new(ANTIGRAVITY_RELEASE_AXIS).expect("axis"),
        ),
        EnvironmentRef::new("antigravity.fixture.environment").expect("environment"),
        antigravity_personal_google_access_profile(access_id.clone()),
        evidence(access_id),
    );
    let host = FixtureHost::completed([stdout(VERSION)]);
    block_on(prepare_antigravity(input, probe(), host.services(host_id)))
        .expect("Antigravity prepares")
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

fn model() -> AntigravityHeadlessModelSelection {
    AntigravityHeadlessModelSelection::new(
        ModelRouteId::new("antigravity.fixture.route").expect("route"),
        ModelRouteRevision::new("1").expect("revision"),
        ProviderId::new("google").expect("provider"),
        ModelId::new("gemini-3.6-flash-high").expect("model"),
    )
}

fn probe() -> AntigravityPreparationProbe {
    AntigravityPreparationProbe::new(
        request_id("probe"),
        ScopeId::new("antigravity-preparation-probe").expect("scope"),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}

fn host_id() -> ExecutionHostId {
    ExecutionHostId::new("antigravity.fixture.host").expect("host id")
}

fn request_id(value: &str) -> RequestId {
    RequestId::new(value).expect("request id")
}

fn working_resource() -> WorkingResourceRef {
    WorkingResourceRef::new("workspace.main").expect("resource")
}

fn deadline() -> Deadline {
    Deadline::at(MonotonicInstant::from_ticks(1_000))
}

fn stdout(value: &str) -> ProcessOutputChunk {
    ProcessOutputChunk::new(ProcessOutputStream::Stdout, value.as_bytes().to_vec())
}
