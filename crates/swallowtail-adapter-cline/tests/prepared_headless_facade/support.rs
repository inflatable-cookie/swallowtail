fn prepare(
    host_id: ExecutionHostId,
) -> swallowtail_adapter_cline::ClineHeadlessPreparedIntegration {
    let discovery = DiscoveryHost::new(CLINE_PACKAGE_VERSION);
    let operation = FixtureHost::scripted([SUCCESS]);
    let mut services = discovery.services(host_id.clone());
    services = services.with_working_resource(
        operation
            .services(host_id.clone())
            .working_resource()
            .expect("resource service")
            .clone(),
    );
    block_on(prepare_cline_headless(
        preparation_input(host_id),
        probe(),
        services,
    ))
    .expect("Cline headless prepares")
}

fn preparation_input(host_id: ExecutionHostId) -> ClineHeadlessPreparationInput {
    ClineHeadlessPreparationInput::new(
        ConfiguredInstanceId::new("cline.fixture.instance").expect("instance"),
        InstanceRevision::new("1").expect("revision"),
        host_id,
        target(),
        EnvironmentRef::new("cline.fixture.isolated").expect("environment"),
        cline_local_account_access_profile(
            AccessProfileId::new("cline.fixture.local-account").expect("access"),
        ),
        evidence(),
    )
}

fn run_input(id: &str) -> ClineHeadlessRunProfileInput {
    ClineHeadlessRunProfileInput::new(
        RequestId::new(format!("cline.fixture.run.{id}")).expect("request"),
        OperationContent::new("private fixture prompt").expect("prompt"),
        WorkingResourceRef::new("cline.fixture.workspace").expect("resource"),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
    )
}

fn probe() -> ClineHeadlessPreparationProbe {
    ClineHeadlessPreparationProbe::new(
        RequestId::new("cline.fixture.headless.probe").expect("request"),
        ScopeId::new("cline.fixture.headless.probe").expect("scope"),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
        DiscoveryCancellation::new(),
    )
}

fn target() -> InstalledExecutableTarget {
    InstalledExecutableTarget::new(
        ExecutableRef::new(format!("/fixture/bin/{CLINE_EXECUTABLE_NAME}")).expect("executable"),
        InterfaceVersionAxis::new(CLINE_PACKAGE_AXIS).expect("axis"),
    )
}

fn evidence() -> PreparedAccessEvidence {
    PreparedAccessEvidence::caller_asserted(AccessStatus::new(
        AccessProfileId::new("cline.fixture.local-account").expect("access"),
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    ))
}
