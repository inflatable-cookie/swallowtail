fn prepare(
    route: CursorPreparedDriver,
    host_id: ExecutionHostId,
) -> Result<CursorPreparedIntegration, PreparationFailure> {
    prepare_release(route, host_id, VERSION)
}

fn prepare_release(
    route: CursorPreparedDriver,
    host_id: ExecutionHostId,
    release: &str,
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
    let host = support::FixtureHost::completed([stdout(release)]);
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
    parameterized_input(
        parameterized_selection("fixture-model").expect("selection"),
        access,
    )
}

fn parameterized_selection(
    model: &str,
) -> Result<CursorHeadlessModelSelection, PreparationFailure> {
    Ok(CursorHeadlessModelSelection::new(
        ModelRouteId::new("cursor.fixture.route").expect("route"),
        ModelRouteRevision::new("1").expect("revision"),
        ProviderId::new("cursor").expect("provider"),
        ModelId::new(model).expect("model"),
    ))
}

fn parameterized_input(
    model: CursorHeadlessModelSelection,
    access: ResourceAccess,
) -> CursorHeadlessRunProfileInput {
    CursorHeadlessRunProfileInput::new(
        request_id(match access {
            ResourceAccess::Read => "headless-read",
            ResourceAccess::ReadWrite => "headless-write",
        }),
        model,
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
