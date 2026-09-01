fn session(
    scenario: Scenario,
    plan: bool,
    id: &str,
) -> (ClinePreparedSession, FixtureHost, swallowtail_runtime::HostServices) {
    session_at_revision(scenario, plan, id, "1")
}

fn session_at_revision(
    scenario: Scenario,
    plan: bool,
    id: &str,
    revision: &str,
) -> (ClinePreparedSession, FixtureHost, swallowtail_runtime::HostServices) {
    let host_id = ExecutionHostId::new(format!("fixture.projection.{id}")).expect("host");
    let discovery = DiscoveryHost::new(CLINE_PACKAGE_VERSION);
    let operation = FixtureHost::new(scenario);
    let services = discovery.services(host_id.clone()).with_working_resource(
        operation.services(host_id.clone()).working_resource().expect("resource service").clone(),
    );
    let integration = block_on(prepare_cline_acp(
        ClinePreparationInput::new(
            ConfiguredInstanceId::new(format!("cline.projection.{id}")).expect("instance"),
            InstanceRevision::new(revision).expect("revision"),
            host_id.clone(),
            target(),
            EnvironmentRef::new("cline.projection.isolated").expect("environment"),
            cline_local_account_access_profile(
                AccessProfileId::new("cline.projection.local-account").expect("profile"),
            ),
            evidence(),
        ),
        probe(id),
        services,
    )).expect("Cline prepares");
    let mut input = ClineSessionProfileInput::new(
        RequestId::new(format!("cline.projection.session.{id}")).expect("request"),
        WorkingResourceRef::new("cline.projection.workspace").expect("resource"),
    );
    if plan { input = input.with_harness_mode(HarnessMode::Plan); }
    (
        integration.prepare_session(input).expect("session prepares"),
        operation.clone(),
        operation.services(host_id),
    )
}

fn session_preparation_with_status(
    status: AccessStatus,
    id: &str,
) -> Result<ClinePreparedSession, PreparationFailure> {
    let host_id = ExecutionHostId::new(format!("fixture.projection.{id}")).expect("host");
    let discovery = DiscoveryHost::new(CLINE_PACKAGE_VERSION);
    let operation = FixtureHost::new(Scenario::Success);
    let services = discovery.services(host_id.clone()).with_working_resource(
        operation
            .services(host_id.clone())
            .working_resource()
            .expect("resource service")
            .clone(),
    );
    let integration = block_on(prepare_cline_acp(
        ClinePreparationInput::new(
            ConfiguredInstanceId::new(format!("cline.projection.{id}")).expect("instance"),
            InstanceRevision::new("1").expect("revision"),
            host_id,
            target(),
            EnvironmentRef::new("cline.projection.isolated").expect("environment"),
            cline_local_account_access_profile(
                AccessProfileId::new("cline.projection.local-account").expect("profile"),
            ),
            PreparedAccessEvidence::caller_asserted(status),
        ),
        probe(id),
        services,
    ))?;
    integration.prepare_session(ClineSessionProfileInput::new(
        RequestId::new(format!("cline.projection.session.{id}")).expect("request"),
        WorkingResourceRef::new("cline.projection.workspace").expect("resource"),
    ))
}

fn headless_run(plan: bool) -> swallowtail_adapter_cline::ClineHeadlessPreparedRun {
    let host_id = ExecutionHostId::new("fixture.projection.headless").expect("host");
    let discovery = DiscoveryHost::new(CLINE_PACKAGE_VERSION);
    let operation = headless_support::FixtureHost::scripted([HEADLESS_SUCCESS]);
    let services = discovery.services(host_id.clone()).with_working_resource(
        operation.services(host_id.clone()).working_resource().expect("resource service").clone(),
    );
    let integration = block_on(prepare_cline_headless(
        ClineHeadlessPreparationInput::new(
            ConfiguredInstanceId::new("cline.projection.headless").expect("instance"),
            InstanceRevision::new("1").expect("revision"),
            host_id,
            target(),
            EnvironmentRef::new("cline.projection.headless").expect("environment"),
            cline_local_account_access_profile(
                AccessProfileId::new("cline.projection.local-account").expect("profile"),
            ),
            evidence(),
        ),
        ClineHeadlessPreparationProbe::new(
            RequestId::new("cline.projection.headless.probe").expect("request"),
            ScopeId::new("cline.projection.headless.probe").expect("scope"),
            Deadline::at(MonotonicInstant::from_ticks(1_000)),
            DiscoveryCancellation::new(),
        ),
        services,
    )).expect("headless prepares");
    let mut input = ClineHeadlessRunProfileInput::new(
        RequestId::new("cline.projection.headless.run").expect("request"),
        OperationContent::new("fixture prompt").expect("content"),
        WorkingResourceRef::new("cline.projection.workspace").expect("resource"),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
    );
    if plan { input = input.with_harness_mode(HarnessMode::Plan); }
    integration.prepare_run(input).expect("run prepares")
}

fn semantic_ids(contribution: &ConsumerRouteProjectionContribution) -> BTreeSet<String> {
    contribution.selection_rows().chain(contribution.session_start_rows()).chain(contribution.active_session_rows()).map(|row| match row.identity() {
        identity if identity.namespaced_extension().is_some() => identity.namespaced_extension().expect("checked").semantic_id().to_owned(),
        ConsumerRouteRowIdentity::Feature(feature) => match feature {
            swallowtail_runtime::ConsumerRouteFeatureId::StructuredRun => "feature.structured-run",
            swallowtail_runtime::ConsumerRouteFeatureId::InteractiveSession => "feature.interactive-session",
            swallowtail_runtime::ConsumerRouteFeatureId::StreamingEvents => "feature.streaming-events",
            swallowtail_runtime::ConsumerRouteFeatureId::CancellationOrInterruption => "feature.cancellation-or-interruption",
            swallowtail_runtime::ConsumerRouteFeatureId::WorkingResource => "feature.working-resource",
            swallowtail_runtime::ConsumerRouteFeatureId::ActivityObservation => "feature.activity-observation",
            swallowtail_runtime::ConsumerRouteFeatureId::PreparedFacade => "feature.prepared-facade",
            other => panic!("unexpected feature {other:?}"),
        }.to_owned(),
        other => panic!("unexpected identity {other:?}"),
    }).collect()
}

fn cline_shape(route: &str, semantic: &str) -> &'static str {
    match (route, semantic) {
        ("cline.acp", "feature.interactive-session" | "feature.active-session-plan-ack" | "feature.negotiated-model-options-observation" | "control.harness-mode") => "interactive-session",
        ("cline.headless", "feature.structured-run" | "control.harness-mode") => "structured-run",
        (_, "feature.streaming-events" | "feature.activity-observation") => "route-observation",
        (_, "feature.cancellation-or-interruption" | "feature.working-resource" | "feature.prepared-facade") => "route-capability",
        _ => panic!("unexpected observed Cline tuple {route}/{semantic}"),
    }
}

fn census_tuples(routes: &[&str]) -> BTreeSet<(String, String, String)> {
    include_str!("../fixtures/consumer-route-projection-census.csv")
        .lines().skip(1).filter_map(|line| {
            let mut fields = line.split(',');
            let route = fields.next()?;
            let shape = fields.next()?;
            let semantic = fields.next()?;
            routes.contains(&route).then(|| (route.to_owned(), shape.to_owned(), semantic.to_owned()))
        }).collect()
}

fn source(value: &str) -> ConsumerRouteProjectionSourceId {
    ConsumerRouteProjectionSourceId::new(value).expect("source")
}

fn target() -> InstalledExecutableTarget {
    InstalledExecutableTarget::new(
        ExecutableRef::new(format!("/fixture/bin/{CLINE_EXECUTABLE_NAME}")).expect("executable"),
        InterfaceVersionAxis::new(CLINE_PACKAGE_AXIS).expect("axis"),
    )
}

fn probe(id: &str) -> ClinePreparationProbe {
    ClinePreparationProbe::new(
        RequestId::new(format!("cline.projection.probe.{id}")).expect("request"),
        ScopeId::new(format!("cline.projection.probe.{id}")).expect("scope"),
        Deadline::at(MonotonicInstant::from_ticks(1_000)),
        DiscoveryCancellation::new(),
    )
}

fn evidence() -> PreparedAccessEvidence {
    PreparedAccessEvidence::caller_asserted(AccessStatus::new(
        AccessProfileId::new("cline.projection.local-account").expect("profile"),
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    ))
}
