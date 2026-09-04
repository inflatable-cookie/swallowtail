use super::discovery_support::{FakeProcessService, services as discovery_services};
use super::*;
use std::collections::BTreeSet;
use swallowtail_adapter_grok::{
    GROK_BUILD_ACP_AXIS, GrokModelSelection, GrokPreparationInput, GrokPreparationProbe,
    GrokRunProfileInput, GrokSessionProfileInput, grok_build_subscription_access_profile,
    prepare_grok_build,
};
use swallowtail_core::InterfaceVersionAxis;
use swallowtail_runtime::{ConsumerRouteProjectionSourceId, ExecutableRef};

#[test]
fn candidate_e_grok_routes_reconcile_executable_projection_truth() {
    let prepared = prepared_integration("grok.projection.reconcile");
    let session = prepared
        .prepare_session(GrokSessionProfileInput::new(
            RequestId::new("grok.projection.reconcile.session").expect("request"),
            GrokModelSelection::new(
                ModelRouteId::new("grok.fixture.route").expect("route"),
                ModelRouteRevision::new("grok.fixture.route-r1").expect("revision"),
                ModelId::new("grok-4.5").expect("model"),
            ),
            WorkingResourceRef::new("grok.fixture.workspace").expect("resource"),
            swallowtail_runtime::SessionOptions::default(),
        ))
        .expect("session prepares");
    let run = prepared
        .prepare_run(GrokRunProfileInput::new(
            RequestId::new("grok.projection.reconcile.run").expect("request"),
            GrokModelSelection::new(
                ModelRouteId::new("grok.fixture.run-route").expect("route"),
                ModelRouteRevision::new("grok.fixture.run-route-r1").expect("revision"),
                ModelId::new("grok-4.5").expect("model"),
            ),
            OperationContent::new("projection proof").expect("content"),
            WorkingResourceRef::new("grok.fixture.workspace").expect("resource"),
            None,
        ))
        .expect("run prepares");
    let source = |value| ConsumerRouteProjectionSourceId::new(value).expect("source");
    let session = session
        .consumer_route_projection_contribution(source("grok.projection.reconcile.session"))
        .expect("session projection");
    let run = run
        .consumer_route_projection_contribution(source("grok.projection.reconcile.run"))
        .expect("run projection");
    let emitted = rows(&session).chain(rows(&run)).collect::<BTreeSet<_>>();
    assert_eq!(emitted.len(), 10);
    assert_eq!(emitted, expected_emitted());
    for withheld in [
        "ModelCatalogue",
        "persistent-session-posture",
        "negotiated-model-options-observation",
    ] {
        assert!(
            !emitted
                .iter()
                .any(|(_, identity)| identity.contains(withheld)),
            "Grok must withhold {withheld}"
        );
    }
}

fn rows(
    contribution: &swallowtail_runtime::ConsumerRouteProjectionContribution,
) -> impl Iterator<Item = (String, String)> + '_ {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
        .map(move |row| {
            let identity = row.identity().namespaced_extension().map_or_else(
                || format!("{:?}", row.identity()),
                |extension| extension.semantic_id().to_owned(),
            );
            let route_specific = if identity.starts_with("Control") {
                format!("{:?}", contribution.applicability().operation_shape())
            } else {
                String::new()
            };
            (route_specific, identity)
        })
}
fn expected_emitted() -> BTreeSet<(String, String)> {
    [
        ("", "Feature(PreparedFacade)"),
        ("", "Feature(StructuredRun)"),
        ("", "Feature(InteractiveSession)"),
        ("", "Feature(StreamingEvents)"),
        ("", "Feature(UsageEvidence)"),
        ("", "Feature(WorkingResource)"),
        ("", "Feature(ActivityObservation)"),
        ("InteractiveSession", "Control(ModelSelection)"),
        ("InteractiveSession", "Control(SessionOptions)"),
        ("StructuredRun", "Control(ModelSelection)"),
    ]
    .into_iter()
    .map(|(shape, identity)| (shape.to_owned(), identity.to_owned()))
    .collect()
}

fn prepared_integration(prefix: &str) -> swallowtail_adapter_grok::GrokPreparedIntegration {
    let host = ExecutionHostId::new(format!("{prefix}.host")).expect("host");
    let credential = CredentialRef::new(format!("{prefix}.credential")).expect("credential");
    let access = grok_build_subscription_access_profile(credential);
    let status = AccessStatus::new(
        access.id().clone(),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    );
    let target = swallowtail_runtime::InstalledExecutableTarget::new(
        ExecutableRef::new(format!("{prefix}.executable")).expect("target"),
        InterfaceVersionAxis::new(GROK_BUILD_ACP_AXIS).expect("axis"),
    );
    let (process, _) = FakeProcessService::completed("grok 0.2.114 (0c785038798) [stable]\n");
    block_on(prepare_grok_build(
        GrokPreparationInput::new(
            ConfiguredInstanceId::new(format!("{prefix}.instance")).expect("instance"),
            InstanceRevision::new("1").expect("revision"),
            host.clone(),
            target,
            EnvironmentRef::new(format!("{prefix}.environment")).expect("environment"),
            access,
            swallowtail_runtime::PreparedAccessEvidence::caller_asserted(status),
        ),
        GrokPreparationProbe::new(
            RequestId::new(format!("{prefix}.probe")).expect("request"),
            ScopeId::new(format!("{prefix}.scope")).expect("scope"),
            Deadline::at(MonotonicInstant::from_ticks(100)),
            swallowtail_runtime::DiscoveryCancellation::new(),
        ),
        discovery_services(host, process),
    ))
    .expect("prepares")
}

#[test]
fn projected_open_publishes_grok_model_options_only_after_successful_open() {
    let host = ExecutionHostId::new("grok.projection.host").expect("host");
    let credential = CredentialRef::new("grok.projection.credential").expect("credential");
    let access = grok_build_subscription_access_profile(credential);
    let status = AccessStatus::new(
        access.id().clone(),
        CredentialState::Ready,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ProviderSupported,
    );
    let target = swallowtail_runtime::InstalledExecutableTarget::new(
        ExecutableRef::new("grok.projection.executable").expect("target"),
        InterfaceVersionAxis::new(GROK_BUILD_ACP_AXIS).expect("axis"),
    );
    let (process, _) = FakeProcessService::completed("grok 0.2.114 (0c785038798) [stable]\n");
    let prepared = block_on(prepare_grok_build(
        GrokPreparationInput::new(
            ConfiguredInstanceId::new("grok.projection.instance").expect("instance"),
            InstanceRevision::new("1").expect("revision"),
            host.clone(),
            target,
            EnvironmentRef::new("grok.projection.environment").expect("environment"),
            access,
            swallowtail_runtime::PreparedAccessEvidence::caller_asserted(status),
        ),
        GrokPreparationProbe::new(
            RequestId::new("grok.projection.probe").expect("request"),
            ScopeId::new("grok.projection.scope").expect("scope"),
            Deadline::at(MonotonicInstant::from_ticks(100)),
            swallowtail_runtime::DiscoveryCancellation::new(),
        ),
        discovery_services(host.clone(), process),
    ))
    .expect("prepares");
    let session = prepared
        .prepare_session(GrokSessionProfileInput::new(
            RequestId::new("grok.projection.session").expect("request"),
            GrokModelSelection::new(
                ModelRouteId::new("grok.fixture.route").expect("route"),
                ModelRouteRevision::new("grok.fixture.route-r1").expect("revision"),
                ModelId::new("grok-4.5").expect("model"),
            ),
            WorkingResourceRef::new("grok.fixture.workspace").expect("resource"),
            swallowtail_runtime::SessionOptions::default(),
        ))
        .expect("session prepares");
    let source = |value| ConsumerRouteProjectionSourceId::new(value).expect("source");
    assert!(
        !session
            .consumer_route_projection_contribution(source("grok.projection.pre-open"))
            .expect("contribution")
            .active_session_rows()
            .any(|row| row
                .identity()
                .namespaced_extension()
                .is_some_and(|extension| extension.semantic_id()
                    == "feature.negotiated-model-options-observation"))
    );
    let fixture = FixtureHost::new(Scenario::Success);
    let services = fixture.services(host);
    let outcome = block_on(session.open_session_with_projection(
        source("grok.projection.prepared"),
        source("grok.projection.active"),
        SessionCleanupRequest::new(run_deadline()),
        services.clone(),
    ))
    .unwrap_or_else(|failure| panic!("projected open: {}", failure.failure()));
    assert!(outcome.negotiated_model_options().is_some());
    assert!(outcome.contribution().active_session_rows().any(|row| {
        row.source().id().as_str() == "grok.projection.active"
            && row
                .identity()
                .namespaced_extension()
                .is_some_and(|extension| {
                    extension.semantic_id() == "feature.negotiated-model-options-observation"
                })
    }));
    assert_eq!(
        block_on(close_session(outcome.into_parts().0, services)),
        CleanupOutcome::Clean
    );
}
