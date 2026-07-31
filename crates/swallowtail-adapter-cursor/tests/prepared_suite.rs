mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use std::sync::Arc;
use swallowtail_adapter_cursor::{
    CURSOR_AGENT_RELEASE_AXIS, CursorAcpSessionProfileInput, CursorCatalogueProfileInput,
    CursorHeadlessModelSelection, CursorHeadlessRunProfileInput, CursorPreparationInput,
    CursorPreparationProbe, CursorPreparedDriver, CursorPreparedIntegration,
    cursor_subscription_access_profile, prepare_cursor,
};
use swallowtail_core::{
    AccessProfileId, AccessStatus, CredentialState, EndpointAuthorization, EntitlementState,
    ExecutionHostId, InterfaceVersionAxis, ModelId, ModelRouteId, ModelRouteRevision, ProviderId,
    ResourceAccess, ResourceRepresentation, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, Deadline, DiscoveryCancellation, EnvironmentRef, ExecutableRef,
    HostServices, InstalledExecutableTarget, MaterializedResourceRef, MonotonicInstant,
    OperationContent, PreparationFailure, PreparedAccessEvidence, ProcessOutputChunk,
    ProcessOutputStream, RequestId, ResourceLease, RuntimeFailure, ScopeId, TerminalStatus,
    WorkingResourceIoService, WorkingResourceReadRequest, WorkingResourceRef,
    WorkingResourceService, WorkingResourceText, WorkingResourceWriteRequest,
};

const VERSION: &str = "2026.07.01-41b2de7\n";
const CATALOGUE: &str =
    "Available models\n\nauto - Auto (current, default)\nfixture-model - Fixture Model\n";
const HEADLESS: &str =
    include_str!("fixtures/cursor-agent-2026.07.01-41b2de7/headless-success.jsonl");

#[test]
fn explicit_routes_prepare_only_their_typed_operations() {
    let host_id = host_id();
    let catalogue =
        prepare(CursorPreparedDriver::Catalogue, host_id.clone()).expect("catalogue prepares");
    let CursorPreparedIntegration::Catalogue(catalogue) = catalogue else {
        panic!("catalogue route remains explicit");
    };
    let prepared = catalogue
        .prepare_catalogue(CursorCatalogueProfileInput::new(request_id("catalogue")))
        .expect("catalogue operation prepares");
    assert_eq!(
        prepared
            .evidence()
            .binding()
            .driver_identity()
            .id()
            .as_str(),
        "swallowtail.cursor-agent.catalogue"
    );
    let operation_host = support::FixtureHost::completed([stdout(CATALOGUE)]);
    let models = block_on(prepared.list_models(operation_host.services(host_id.clone())))
        .expect("prepared catalogue executes");
    assert_eq!(models.len(), 2);

    let acp = prepare(CursorPreparedDriver::Acp, host_id.clone()).expect("ACP prepares");
    let CursorPreparedIntegration::Acp(acp) = acp else {
        panic!("ACP route remains explicit");
    };
    let prepared = acp
        .prepare_session(CursorAcpSessionProfileInput::new(
            request_id("acp"),
            working_resource(),
        ))
        .expect("ACP session prepares");
    assert_eq!(
        prepared
            .evidence()
            .binding()
            .driver_identity()
            .id()
            .as_str(),
        "swallowtail.cursor-agent.acp"
    );
    assert_eq!(
        prepared.request().access_policy(),
        &swallowtail_core::SessionAccessPolicy::ambient_harness(ResourceAccess::ReadWrite)
    );

    let headless =
        prepare(CursorPreparedDriver::Headless, host_id.clone()).expect("headless prepares");
    let CursorPreparedIntegration::Headless(headless) = headless else {
        panic!("headless route remains explicit");
    };
    let prepared = headless
        .prepare_run(headless_input(ResourceAccess::Read))
        .expect("headless run prepares");
    assert_eq!(
        prepared
            .evidence()
            .binding()
            .driver_identity()
            .id()
            .as_str(),
        "swallowtail.cursor-agent.headless"
    );
    assert_eq!(
        prepared.plan().model_id().map(ModelId::as_str),
        Some("fixture-model")
    );
    assert_eq!(
        prepared.evidence().observable_activity().availability(),
        swallowtail_core::ObservableActivityAvailability::Available
    );
    let operation_host = support::FixtureHost::completed([stdout(HEADLESS)]);
    let mut handle = block_on(prepared.start_run(operation_host.services(host_id)))
        .expect("prepared headless run starts");
    let _events = block_on(handle.take_events().expect("events").collect::<Vec<_>>());
    let terminal = block_on(handle.take_terminal_outcome().expect("terminal"));
    assert_eq!(terminal.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
}

#[test]
fn headless_preparation_keeps_read_and_write_authority_distinct() {
    for access in [ResourceAccess::Read, ResourceAccess::ReadWrite] {
        let prepared = prepare(CursorPreparedDriver::Headless, host_id()).expect("prepares");
        let CursorPreparedIntegration::Headless(prepared) = prepared else {
            panic!("headless route");
        };
        let run = prepared
            .prepare_run(headless_input(access))
            .expect("run prepares");
        assert!(run.plan().requirements().capabilities().any(|requirement| {
            requirement.capability() == swallowtail_core::Capability::WorkingResource
                && requirement.constraints().any(|constraint| {
                    constraint == &swallowtail_core::CapabilityConstraint::ResourceAccess(access)
                })
        }));
    }
}

#[test]
fn preparation_rejects_access_and_axis_drift_before_discovery() {
    let host_id = host_id();
    let access_id = AccessProfileId::new("cursor.fixture.access").expect("access id");
    let access = cursor_subscription_access_profile(access_id.clone());
    let evidence = evidence(access_id);
    let host = support::FixtureHost::completed([stdout(VERSION)]);
    let input = CursorPreparationInput::new(
        CursorPreparedDriver::Catalogue,
        swallowtail_core::ConfiguredInstanceId::new("cursor.fixture").expect("instance"),
        swallowtail_core::InstanceRevision::new("1").expect("revision"),
        host_id.clone(),
        InstalledExecutableTarget::new(
            ExecutableRef::new("cursor.fixture.executable").expect("executable"),
            InterfaceVersionAxis::new("wrong.axis").expect("axis"),
        ),
        EnvironmentRef::new("cursor.fixture.environment").expect("environment"),
        access,
        evidence,
    );
    let error = block_on(prepare_cursor(input, probe(), host.services(host_id)))
        .expect_err("axis drift fails");
    assert_eq!(
        error.diagnostic().safe().code(),
        "swallowtail.cursor.preparation.target_axis_mismatch"
    );
    assert!(!host.started());
}

fn prepare(
    route: CursorPreparedDriver,
    host_id: ExecutionHostId,
) -> Result<CursorPreparedIntegration, PreparationFailure> {
    let access_id = AccessProfileId::new("cursor.fixture.access").expect("access id");
    let input = CursorPreparationInput::new(
        route,
        swallowtail_core::ConfiguredInstanceId::new(format!("cursor.fixture.{route:?}"))
            .expect("instance"),
        swallowtail_core::InstanceRevision::new("1").expect("revision"),
        host_id.clone(),
        InstalledExecutableTarget::new(
            ExecutableRef::new("cursor.fixture.executable").expect("executable"),
            InterfaceVersionAxis::new(CURSOR_AGENT_RELEASE_AXIS).expect("axis"),
        ),
        EnvironmentRef::new("cursor.fixture.environment").expect("environment"),
        cursor_subscription_access_profile(access_id.clone()),
        evidence(access_id),
    );
    let host = support::FixtureHost::completed([stdout(VERSION)]);
    block_on(prepare_cursor(
        input,
        probe(),
        services_with_resource(&host, host_id),
    ))
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

fn headless_input(access: ResourceAccess) -> CursorHeadlessRunProfileInput {
    CursorHeadlessRunProfileInput::new(
        request_id(match access {
            ResourceAccess::Read => "headless-read",
            ResourceAccess::ReadWrite => "headless-write",
        }),
        CursorHeadlessModelSelection::new(
            ModelRouteId::new("cursor.fixture.route").expect("route"),
            ModelRouteRevision::new("1").expect("revision"),
            ProviderId::new("cursor").expect("provider"),
            ModelId::new("fixture-model").expect("model"),
        ),
        OperationContent::new("fixture-private-prompt").expect("prompt"),
        working_resource(),
        access,
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
    )
}

fn probe() -> CursorPreparationProbe {
    CursorPreparationProbe::new(
        request_id("probe"),
        ScopeId::new("cursor-preparation-probe").expect("scope"),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}

fn services_with_resource(host: &support::FixtureHost, host_id: ExecutionHostId) -> HostServices {
    host.services(host_id)
        .with_working_resource(Arc::new(UnusedResourceService))
        .with_working_resource_io(Arc::new(UnusedResourceIoService))
}

struct UnusedResourceService;

impl WorkingResourceService for UnusedResourceService {
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
        Box::pin(async { Err(fixture_failure()) })
    }

    fn release(&self, _lease: ResourceLease) -> BoxFuture<'static, CleanupOutcome> {
        Box::pin(async { CleanupOutcome::Clean })
    }
}

struct UnusedResourceIoService;

impl WorkingResourceIoService for UnusedResourceIoService {
    fn read_text(
        &self,
        _lease: &ResourceLease,
        _request: WorkingResourceReadRequest,
    ) -> BoxFuture<'static, Result<WorkingResourceText, RuntimeFailure>> {
        Box::pin(async { Err(fixture_failure()) })
    }

    fn write_text(
        &self,
        _lease: &ResourceLease,
        _request: WorkingResourceWriteRequest,
    ) -> BoxFuture<'static, Result<(), RuntimeFailure>> {
        Box::pin(async { Err(fixture_failure()) })
    }
}

fn fixture_failure() -> RuntimeFailure {
    RuntimeFailure::new(swallowtail_core::SafeDiagnostic::new(
        "fixture.cursor.prepared.unused",
        "Unused Cursor prepared fixture service was called",
    ))
}

fn host_id() -> ExecutionHostId {
    ExecutionHostId::new("cursor.fixture.host").expect("host id")
}

fn request_id(value: &str) -> RequestId {
    RequestId::new(value).expect("request id")
}

fn working_resource() -> WorkingResourceRef {
    WorkingResourceRef::new("workspace.main").expect("resource")
}

fn stdout(value: &str) -> ProcessOutputChunk {
    ProcessOutputChunk::new(ProcessOutputStream::Stdout, value.as_bytes().to_vec())
}
