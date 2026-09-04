#![allow(dead_code, unused_imports)]

#[path = "support/mod.rs"]
mod acp_support;
#[path = "support/discovery.rs"]
mod discovery_support;
#[path = "headless_support/mod.rs"]
mod headless_support;
#[path = "live_support/mod.rs"]
mod live_support;

use futures_executor::block_on;
use std::collections::BTreeSet;
use std::num::NonZeroU64;
use swallowtail_adapter_gemini::{
    GEMINI_CLI_ACP_AXIS, GeminiCliPreparedIntegration, GeminiHeadlessModelSelection,
    GeminiHeadlessRunProfileInput, GeminiLiveContextWindowCompression,
    GeminiLiveSessionProfileInput, GeminiPreparationInput, GeminiPreparationProbe,
    GeminiSessionProfileInput, prepare_gemini_acp, prepare_gemini_cli, prepare_gemini_live,
};
use swallowtail_core::{
    AccessProfile, AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialMechanism,
    CredentialRef, CredentialState, EndpointAudience, EndpointAuthorization, EntitlementMetering,
    EntitlementState, ExecutionHostId, HarnessMode, InstanceRevision, InterfaceVersionAxis,
    ModelId, ModelRouteId, ModelRouteRevision, ProviderId, ReasoningMode, RuntimeReadiness,
    SupportAuthority,
};
use swallowtail_runtime::{
    ConsumerRouteProjectionContribution, ConsumerRouteProjectionSourceId, Deadline,
    DiscoveryCancellation, EnvironmentRef, ExecutableRef, InstalledExecutableTarget,
    MonotonicInstant, OperationContent, PreparedAccessEvidence, RequestId, ScopeId, SessionOptions,
    WorkingResourceRef,
};

#[test]
fn candidate_e_gemini_routes_reconcile_executable_projection_truth() {
    let acp = prepared_acp()
        .consumer_route_projection_contribution(source("card075.gemini.acp"))
        .expect("ACP projection");
    let headless = prepared_headless()
        .consumer_route_projection_contribution(source("card075.gemini.headless"))
        .expect("headless projection");
    let live = prepared_live()
        .consumer_route_projection_contribution(source("card075.gemini.live"))
        .expect("Live projection");

    let emitted = [
        ("gemini-cli.acp", &acp),
        ("gemini-cli.headless", &headless),
        ("gemini.live", &live),
    ]
    .into_iter()
    .flat_map(|(route, contribution)| rows(route, contribution))
    .collect::<BTreeSet<_>>();
    assert_eq!(rows("gemini-cli.acp", &acp).count(), 7);
    assert_eq!(rows("gemini-cli.headless", &headless).count(), 8);
    assert_eq!(rows("gemini.live", &live).count(), 14);
    assert_eq!(emitted.len(), 29);
    assert_eq!(emitted, expected_emitted());

    // The 14 Gemini omissions are checked against the executable projections,
    // not counted from an audit ledger. Grok's companion proof supplies 10/3.
    let withheld = [
        ("gemini-cli.acp", "ModelCatalogue"),
        ("gemini-cli.acp", "StructuredRun"),
        ("gemini-cli.acp", "UsageEvidence"),
        ("gemini-cli.acp", "bounded-workspace-text-write"),
        ("gemini-cli.acp", "owned-remote-resource-cleanup"),
        ("gemini-cli.acp", "persistent-session-posture"),
        ("gemini-cli.acp", "negotiated-model-options-observation"),
        ("gemini-cli.headless", "ModelCatalogue"),
        ("gemini-cli.headless", "InteractiveSession"),
        ("gemini-cli.headless", "bounded-workspace-text-write"),
        ("gemini-cli.headless", "owned-remote-resource-cleanup"),
        ("gemini-cli.headless", "persistent-session-posture"),
        ("gemini.live", "ModelCatalogue"),
        ("gemini.live", "persistent-session-posture"),
    ];
    assert_eq!(withheld.len(), 14);
    assert_eq!(emitted.len() + withheld.len(), 43);
    for (route, identity) in withheld {
        assert!(
            !emitted
                .iter()
                .any(|(actual_route, actual)| actual_route == route && actual.contains(identity)),
            "{route} must withhold {identity}"
        );
    }
    assert!(
        emitted
            .iter()
            .any(|(route, identity)| route == "gemini.live"
                && identity.contains("context-window-compression"))
    );
}

fn rows<'a>(
    route: &'a str,
    contribution: &'a ConsumerRouteProjectionContribution,
) -> impl Iterator<Item = (String, String)> + 'a {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
        .map(move |row| (route.to_owned(), semantic(row)))
}
fn semantic(row: &swallowtail_runtime::ConsumerRouteProjectionRow) -> String {
    row.identity().namespaced_extension().map_or_else(
        || format!("{:?}", row.identity()),
        |extension| extension.semantic_id().to_owned(),
    )
}
fn expected_emitted() -> BTreeSet<(String, String)> {
    [
        ("gemini-cli.acp", "Feature(PreparedFacade)"),
        ("gemini-cli.acp", "Feature(InteractiveSession)"),
        ("gemini-cli.acp", "Feature(StreamingEvents)"),
        ("gemini-cli.acp", "Feature(CancellationOrInterruption)"),
        ("gemini-cli.acp", "Feature(WorkingResource)"),
        ("gemini-cli.acp", "Feature(ActivityObservation)"),
        ("gemini-cli.acp", "control.harness-mode"),
        ("gemini-cli.headless", "Feature(PreparedFacade)"),
        ("gemini-cli.headless", "Feature(StructuredRun)"),
        ("gemini-cli.headless", "Feature(StreamingEvents)"),
        ("gemini-cli.headless", "Feature(UsageEvidence)"),
        ("gemini-cli.headless", "Feature(CancellationOrInterruption)"),
        ("gemini-cli.headless", "Feature(WorkingResource)"),
        ("gemini-cli.headless", "Feature(ActivityObservation)"),
        ("gemini-cli.headless", "Control(ModelSelection)"),
        ("gemini.live", "Feature(PreparedFacade)"),
        ("gemini.live", "Feature(RealtimeMediaSession)"),
        ("gemini.live", "Feature(StreamingEvents)"),
        ("gemini.live", "Feature(UsageEvidence)"),
        ("gemini.live", "Feature(OutputTokenLimit)"),
        ("gemini.live", "Feature(ReasoningSelection)"),
        ("gemini.live", "Feature(CancellationOrInterruption)"),
        ("gemini.live", "feature.planned-connection-rollover"),
        ("gemini.live", "Feature(ActivityObservation)"),
        ("gemini.live", "Control(ReasoningSelection)"),
        ("gemini.live", "Control(MaximumOutputTokens)"),
        ("gemini.live", "Control(RealtimeMediaConfig)"),
        ("gemini.live", "control.context-window-compression"),
        ("gemini.live", "Control(PlannedConnectionRollover)"),
    ]
    .into_iter()
    .map(|(route, identity)| (route.to_owned(), identity.to_owned()))
    .collect()
}
fn source(value: &str) -> ConsumerRouteProjectionSourceId {
    ConsumerRouteProjectionSourceId::new(value).expect("source")
}

fn prepared_acp() -> swallowtail_adapter_gemini::GeminiPreparedSession {
    let host = ExecutionHostId::new("card075.gemini.acp.host").expect("host");
    let operation_host = acp_support::FixtureHost::new(acp_support::Scenario::Success);
    let operation_services = operation_host.services(host.clone());
    let discovery = discovery_support::DiscoveryHost::new("0.51.0");
    let services = discovery
        .services(host.clone())
        .with_working_resource(
            operation_services
                .working_resource()
                .expect("resource")
                .clone(),
        )
        .with_working_resource_io(
            operation_services
                .working_resource_io()
                .expect("resource I/O")
                .clone(),
        );
    block_on(prepare_gemini_acp(
        preparation_input(host),
        probe(),
        services,
    ))
    .expect("ACP prepares")
    .prepare_session(GeminiSessionProfileInput::new(
        RequestId::new("card075.gemini.acp").expect("request"),
        WorkingResourceRef::new("card075.gemini.acp.workspace").expect("resource"),
        SessionOptions::default().with_harness_mode(HarnessMode::Plan),
    ))
    .expect("ACP profile prepares")
}
fn prepared_headless() -> swallowtail_adapter_gemini::GeminiHeadlessPreparedRun {
    let host = ExecutionHostId::new("card075.gemini.headless.host").expect("host");
    let (process, _) = headless_support::FakeProcessService::completed("0.52.0\n");
    let (services, _) = headless_support::host_services_for(
        host.clone(),
        process,
        std::sync::Arc::new(headless_support::PendingTimeService),
    );
    let GeminiCliPreparedIntegration::Headless(prepared) = block_on(prepare_gemini_cli(
        headless_support::cli_preparation_input(host),
        headless_support::cli_probe(),
        services,
    ))
    .expect("headless prepares") else {
        panic!("headless route stays typed");
    };
    prepared
        .prepare_run(GeminiHeadlessRunProfileInput::new(
            RequestId::new("card075.gemini.headless").expect("request"),
            GeminiHeadlessModelSelection::new(
                ModelRouteId::new("card075.gemini.headless.route").expect("route"),
                ModelRouteRevision::new("1").expect("revision"),
                ProviderId::new("gemini").expect("provider"),
                ModelId::new("gemini-2.5-flash").expect("model"),
            ),
            OperationContent::new("projection proof").expect("content"),
            WorkingResourceRef::new("card075.gemini.headless.workspace").expect("resource"),
            Deadline::at(MonotonicInstant::from_ticks(1_000)),
        ))
        .expect("headless profile prepares")
}
fn prepared_live() -> swallowtail_adapter_gemini::GeminiPreparedLiveSession {
    let fixture = live_support::LiveFixture::new(
        live_support::LiveScenario::TwoTurnsRollover,
        live_support::TimeMode::Pending,
    );
    let prepared = prepare_gemini_live(fixture.preparation_input(), &fixture.services())
        .expect("Live prepares");
    prepared
        .prepare_live_session(
            GeminiLiveSessionProfileInput::manual_pcm_with_one_rollover(
                RequestId::new("card075.gemini.live").expect("request"),
                None,
            )
            .with_reasoning_mode(ReasoningMode::new("high").expect("mode"))
            .with_maximum_output_tokens(NonZeroU64::new(1_024).expect("maximum"))
            .with_context_window_compression(GeminiLiveContextWindowCompression::sliding_window()),
        )
        .expect("Live profile prepares")
}
fn preparation_input(host: ExecutionHostId) -> GeminiPreparationInput {
    GeminiPreparationInput::new(
        ConfiguredInstanceId::new("card075.gemini.acp.instance").expect("instance"),
        InstanceRevision::new("1").expect("revision"),
        host,
        InstalledExecutableTarget::new(
            ExecutableRef::new("card075.gemini.acp.executable").expect("executable"),
            InterfaceVersionAxis::new(GEMINI_CLI_ACP_AXIS).expect("axis"),
        ),
        EnvironmentRef::new("card075.gemini.acp.environment").expect("environment"),
        AccessProfile::new(
            AccessProfileId::new("card075.gemini.acp.access").expect("access"),
            CredentialMechanism::ApiKey,
            EntitlementMetering::PayAsYouGo,
            EndpointAudience::new("gemini-developer-api").expect("audience"),
            SupportAuthority::ProviderSupported,
        )
        .with_credential_reference(
            CredentialRef::new("card075.gemini.acp.credential").expect("credential"),
        ),
        PreparedAccessEvidence::caller_asserted(AccessStatus::new(
            AccessProfileId::new("card075.gemini.acp.access").expect("access"),
            CredentialState::Ready,
            EntitlementState::Available,
            EndpointAuthorization::Allowed,
            RuntimeReadiness::Ready,
            SupportAuthority::ProviderSupported,
        )),
    )
}
fn probe() -> GeminiPreparationProbe {
    GeminiPreparationProbe::new(
        RequestId::new("card075.gemini.acp.probe").expect("request"),
        ScopeId::new("card075.gemini.acp.probe").expect("scope"),
        Deadline::at(MonotonicInstant::from_ticks(100)),
        DiscoveryCancellation::new(),
    )
}
