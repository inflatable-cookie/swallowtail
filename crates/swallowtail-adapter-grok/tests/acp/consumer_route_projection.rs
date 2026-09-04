use super::discovery_support::{FakeProcessService, services as discovery_services};
use super::*;
use swallowtail_adapter_grok::{
    GROK_BUILD_ACP_AXIS, GrokModelSelection, GrokPreparationInput, GrokPreparationProbe,
    GrokSessionProfileInput, grok_build_subscription_access_profile, prepare_grok_build,
};
use swallowtail_core::InterfaceVersionAxis;
use swallowtail_runtime::{ConsumerRouteProjectionSourceId, ExecutableRef};

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
