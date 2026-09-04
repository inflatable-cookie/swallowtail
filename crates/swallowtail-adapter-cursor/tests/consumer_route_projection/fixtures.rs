use futures_executor::block_on;
use std::sync::Arc;
use swallowtail_adapter_cursor::{
    CURSOR_AGENT_RELEASE_AXIS, CursorAcpSessionProfileInput, CursorCatalogueProfileInput,
    CursorHeadlessContext, CursorHeadlessFast, CursorHeadlessModelSelection,
    CursorHeadlessReadMode, CursorHeadlessRunProfileInput, CursorPreparationInput,
    CursorPreparationProbe, CursorPreparedAcpSession, CursorPreparedCatalogue,
    CursorPreparedDriver, CursorPreparedHeadlessRun, CursorPreparedIntegration,
    cursor_subscription_access_profile, prepare_cursor,
};
use swallowtail_core::{
    AccessProfileId, AccessStatus, CredentialState, EndpointAuthorization, EntitlementState,
    ExecutionHostId, InterfaceVersionAxis, ModelId, ModelRouteId, ModelRouteRevision, ProviderId,
    ReasoningMode, ResourceAccess, ResourceRepresentation, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, Deadline, DiscoveryCancellation, EnvironmentRef, ExecutableRef,
    HostServices, InstalledExecutableTarget, MaterializedResourceRef, MonotonicInstant,
    OperationContent, PreparedAccessEvidence, ProcessOutputChunk, ProcessOutputStream, RequestId,
    ResourceLease, RuntimeFailure, ScopeId, WorkingResourceIoService, WorkingResourceReadRequest,
    WorkingResourceRef, WorkingResourceService, WorkingResourceText, WorkingResourceWriteRequest,
};

#[path = "../support.rs"]
mod support;
use support::FixtureHost;

const VERSION: &str = "2026.07.01-41b2de7\n";

fn stdout(output: &str) -> ProcessOutputChunk {
    ProcessOutputChunk::new(ProcessOutputStream::Stdout, output.as_bytes().to_vec())
}

fn host_id() -> ExecutionHostId {
    ExecutionHostId::new("cursor.fixture.host").expect("host id")
}

fn working_resource() -> WorkingResourceRef {
    WorkingResourceRef::new("workspace.main").expect("resource")
}

fn deadline() -> Deadline {
    Deadline::at(MonotonicInstant::from_ticks(1_000))
}

fn probe() -> CursorPreparationProbe {
    CursorPreparationProbe::new(
        RequestId::new("probe").expect("request id"),
        ScopeId::new("cursor-preparation-probe").expect("scope"),
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

fn services(host: &FixtureHost, host_id: ExecutionHostId) -> HostServices {
    host.services(host_id)
        .with_working_resource(Arc::new(DummyResourceService))
        .with_working_resource_io(Arc::new(DummyResourceIoService))
}

struct DummyResourceService;

impl WorkingResourceService for DummyResourceService {
    fn resolve(
        &self,
        scope: ScopeId,
        reference: WorkingResourceRef,
        access: ResourceAccess,
        representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>> {
        Box::pin(async move {
            Ok(
                ResourceLease::consumer_owned(scope, reference, access, representation)
                    .with_filesystem(
                        MaterializedResourceRef::new("/fixture/cursor").expect("materialized"),
                    ),
            )
        })
    }

    fn create_temporary(
        &self,
        _scope: ScopeId,
        _access: ResourceAccess,
        _representation: ResourceRepresentation,
    ) -> BoxFuture<'static, Result<ResourceLease, RuntimeFailure>> {
        Box::pin(async {
            Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                "fixture.unused",
                "unused",
            )))
        })
    }

    fn release(&self, _lease: ResourceLease) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async { CleanupOutcome::Clean })
    }
}

struct DummyResourceIoService;

impl WorkingResourceIoService for DummyResourceIoService {
    fn read_text(
        &self,
        _lease: &ResourceLease,
        _request: WorkingResourceReadRequest,
    ) -> BoxFuture<'static, Result<WorkingResourceText, RuntimeFailure>> {
        Box::pin(async {
            Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                "fixture.unused",
                "unused",
            )))
        })
    }

    fn write_text(
        &self,
        _lease: &ResourceLease,
        _request: WorkingResourceWriteRequest,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async {
            Err(RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
                "fixture.unused",
                "unused",
            )))
        })
    }
}

fn prepare(driver: CursorPreparedDriver) -> CursorPreparedIntegration {
    let host = host_id();
    let access_id = AccessProfileId::new("cursor.fixture.access").expect("access id");
    let input = CursorPreparationInput::new(
        driver,
        swallowtail_core::ConfiguredInstanceId::new(format!("cursor.fixture.{driver:?}"))
            .expect("instance"),
        swallowtail_core::InstanceRevision::new("1").expect("revision"),
        host.clone(),
        InstalledExecutableTarget::new(
            ExecutableRef::new("cursor.fixture.executable").expect("executable"),
            InterfaceVersionAxis::new(CURSOR_AGENT_RELEASE_AXIS).expect("axis"),
        ),
        EnvironmentRef::new("cursor.fixture.environment").expect("environment"),
        cursor_subscription_access_profile(access_id.clone()),
        evidence(access_id),
    );
    let fixture_host = FixtureHost::completed([stdout(VERSION)]);
    block_on(prepare_cursor(
        input,
        probe(),
        services(&fixture_host, host),
    ))
    .expect("Cursor prepares")
}

pub fn acp() -> CursorPreparedAcpSession {
    let integration = prepare(CursorPreparedDriver::Acp);
    let CursorPreparedIntegration::Acp(acp) = integration else {
        panic!("expected acp integration");
    };
    let input = CursorAcpSessionProfileInput::new(
        RequestId::new("acp").expect("request id"),
        working_resource(),
    );
    acp.prepare_session(input).expect("acp prepares")
}

pub fn catalogue() -> CursorPreparedCatalogue {
    let integration = prepare(CursorPreparedDriver::Catalogue);
    let CursorPreparedIntegration::Catalogue(catalogue) = integration else {
        panic!("expected catalogue integration");
    };
    let input = CursorCatalogueProfileInput::new(RequestId::new("cat").expect("request id"));
    catalogue
        .prepare_catalogue(input)
        .expect("catalogue prepares")
}

pub fn headless_maximal() -> CursorPreparedHeadlessRun {
    let integration = prepare(CursorPreparedDriver::Headless);
    let CursorPreparedIntegration::Headless(headless) = integration else {
        panic!("expected headless integration");
    };
    let model = CursorHeadlessModelSelection::new(
        ModelRouteId::new("cursor.fixture.route").expect("route"),
        ModelRouteRevision::new("1").expect("revision"),
        ProviderId::new("cursor").expect("provider"),
        ModelId::new("claude-opus-4-8").expect("model"),
    )
    .with_fast(CursorHeadlessFast::Standard)
    .expect("fast")
    .with_context(CursorHeadlessContext::OneMillion)
    .expect("context")
    .with_effort(ReasoningMode::new("high").expect("high"))
    .expect("effort");

    let input = CursorHeadlessRunProfileInput::new(
        RequestId::new("headless-max").expect("request id"),
        model,
        OperationContent::new("prompt").expect("prompt"),
        working_resource(),
        ResourceAccess::Read,
        deadline(),
    )
    .with_read_mode(CursorHeadlessReadMode::Plan)
    .expect("read mode");

    headless
        .prepare_run(input)
        .expect("headless maximal prepares")
}

pub fn headless_minimal() -> CursorPreparedHeadlessRun {
    let integration = prepare(CursorPreparedDriver::Headless);
    let CursorPreparedIntegration::Headless(headless) = integration else {
        panic!("expected headless integration");
    };
    let model = CursorHeadlessModelSelection::new(
        ModelRouteId::new("cursor.fixture.route").expect("route"),
        ModelRouteRevision::new("1").expect("revision"),
        ProviderId::new("cursor").expect("provider"),
        ModelId::new("fixture-model").expect("model"),
    );

    let input = CursorHeadlessRunProfileInput::new(
        RequestId::new("headless-min").expect("request id"),
        model,
        OperationContent::new("prompt").expect("prompt"),
        working_resource(),
        ResourceAccess::ReadWrite,
        deadline(),
    );

    headless
        .prepare_run(input)
        .expect("headless minimal prepares")
}
