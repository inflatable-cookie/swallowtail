#![allow(dead_code)]

#[path = "support/discovery.rs"]
mod discovery_support;
mod support;

use discovery_support::DiscoveryHost;
use futures_executor::block_on;
use std::collections::BTreeSet;
use support::{FixtureHost, Scenario};
use swallowtail_adapter_copilot_cli::{
    COPILOT_CLI_EXECUTABLE_NAME, COPILOT_CLI_PACKAGE_AXIS, COPILOT_CLI_PACKAGE_VERSION,
    CopilotCliPreparationInput, CopilotCliPreparationProbe, CopilotCliSessionProfileInput,
    copilot_cli_host_account_access_profile, prepare_copilot_cli_acp,
};
use swallowtail_core::{
    AccessProfileId, AccessStatus, ConfiguredInstanceId, CredentialState, EndpointAuthorization,
    EntitlementState, ExecutionHostId, InstanceRevision, InterfaceVersionAxis, OperationShape,
    RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    ConsumerRouteActorPosture, ConsumerRouteAvailability, ConsumerRouteFeatureId,
    ConsumerRouteLifecycle, ConsumerRouteProjectionContribution, ConsumerRouteProjectionSourceId,
    ConsumerRouteRowIdentity, ConsumerRouteStateSupport, ConsumerRouteSupportPosture, Deadline,
    DiscoveryCancellation, EnvironmentRef, ExecutableRef, InstalledExecutableTarget,
    MonotonicInstant, PreparedAccessEvidence, RequestId, ScopeId, WorkingResourceRef,
};

#[test]
fn exact_six_rows_and_three_negative_ledger_entries_are_proved() {
    let session = prepared_session();
    let contribution = session
        .consumer_route_projection_contribution(source("copilot-cli.projection.session"))
        .expect("session contributes");
    assert_eq!(
        contribution.applicability().operation_shape(),
        OperationShape::InteractiveSession
    );
    let ids = identities(&contribution);
    assert_eq!(ids.len(), 6);
    assert_eq!(
        ids,
        BTreeSet::from([
            "feature.activity-observation",
            "feature.cancellation-or-interruption",
            "feature.interactive-session",
            "feature.prepared-facade",
            "feature.streaming-events",
            "feature.working-resource",
        ])
    );
    for withheld in [
        "feature.model-catalogue",
        "feature.persistent-session-posture",
        "audit.no-public-route-specific-selectable-control",
    ] {
        assert!(!ids.contains(withheld));
    }
    for row in rows(&contribution) {
        assert_eq!(row.applicability(), contribution.applicability());
        assert_eq!(row.support(), ConsumerRouteSupportPosture::Supported);
        assert_eq!(row.availability(), ConsumerRouteAvailability::Available);
        assert!(matches!(
            row.identity(),
            ConsumerRouteRowIdentity::Feature(_)
        ));
        if row.identity()
            == &ConsumerRouteRowIdentity::Feature(ConsumerRouteFeatureId::ActivityObservation)
        {
            assert_eq!(
                row.lifecycle(),
                ConsumerRouteLifecycle::PostOpenObservationOnly
            );
            assert_eq!(
                row.actor_posture(),
                ConsumerRouteActorPosture::ObservationOnly
            );
            assert_eq!(
                row.state_support(),
                ConsumerRouteStateSupport::descriptor_only()
            );
        }
    }
    const LEDGER: [(&str, &str, bool); 9] = [
        ("model-catalogue", "feature.model-catalogue", false),
        ("interactive-session", "feature.interactive-session", true),
        ("route-observation", "feature.streaming-events", true),
        (
            "route-capability",
            "feature.cancellation-or-interruption",
            true,
        ),
        ("route-capability", "feature.working-resource", true),
        (
            "session-lifecycle",
            "feature.persistent-session-posture",
            false,
        ),
        ("route-capability", "feature.prepared-facade", true),
        ("route-observation", "feature.activity-observation", true),
        (
            "route-selection",
            "audit.no-public-route-specific-selectable-control",
            false,
        ),
    ];
    assert_eq!(LEDGER.iter().filter(|row| row.2).count(), 6);
    assert_eq!(LEDGER.iter().filter(|row| !row.2).count(), 3);
    assert_eq!(
        LEDGER
            .iter()
            .map(|(shape, semantic, _)| {
                (
                    "copilot-cli.acp".to_owned(),
                    (*shape).to_owned(),
                    (*semantic).to_owned(),
                )
            })
            .collect::<BTreeSet<_>>(),
        census_tuples()
    );
}

fn prepared_session() -> swallowtail_adapter_copilot_cli::CopilotCliPreparedSession {
    let host_id = ExecutionHostId::new("fixture.projection.local").expect("host");
    let discovery = DiscoveryHost::new(COPILOT_CLI_PACKAGE_VERSION);
    let operation = FixtureHost::new(Scenario::Success);
    let services = discovery.services(host_id.clone()).with_working_resource(
        operation
            .services(host_id.clone())
            .working_resource()
            .expect("resource service")
            .clone(),
    );
    let integration = block_on(prepare_copilot_cli_acp(
        CopilotCliPreparationInput::new(
            ConfiguredInstanceId::new("copilot-cli.projection.instance").expect("instance"),
            InstanceRevision::new("1").expect("revision"),
            host_id,
            InstalledExecutableTarget::new(
                ExecutableRef::new(format!("/fixture/bin/{COPILOT_CLI_EXECUTABLE_NAME}"))
                    .expect("executable"),
                InterfaceVersionAxis::new(COPILOT_CLI_PACKAGE_AXIS).expect("axis"),
            ),
            EnvironmentRef::new("copilot-cli.projection.isolated").expect("environment"),
            copilot_cli_host_account_access_profile(
                AccessProfileId::new("copilot-cli.projection.host-account").expect("profile"),
            ),
            evidence(),
        ),
        CopilotCliPreparationProbe::new(
            RequestId::new("copilot-cli.projection.probe").expect("request"),
            ScopeId::new("copilot-cli.projection.probe").expect("scope"),
            Deadline::at(MonotonicInstant::from_ticks(1_000)),
            DiscoveryCancellation::new(),
        ),
        services,
    ))
    .expect("Copilot CLI prepares");
    integration
        .prepare_session(CopilotCliSessionProfileInput::new(
            RequestId::new("copilot-cli.projection.session").expect("request"),
            WorkingResourceRef::new("copilot-cli.projection.workspace").expect("resource"),
        ))
        .expect("session prepares")
}

fn evidence() -> PreparedAccessEvidence {
    PreparedAccessEvidence::caller_asserted(AccessStatus::new(
        AccessProfileId::new("copilot-cli.projection.host-account").expect("profile"),
        CredentialState::NotRequired,
        EntitlementState::Available,
        EndpointAuthorization::Allowed,
        RuntimeReadiness::Ready,
        SupportAuthority::ExperimentalObserved,
    ))
}

fn identities(contribution: &ConsumerRouteProjectionContribution) -> BTreeSet<&'static str> {
    rows(contribution)
        .map(|row| match row.identity() {
            ConsumerRouteRowIdentity::Feature(feature) => match feature {
                ConsumerRouteFeatureId::InteractiveSession => "feature.interactive-session",
                ConsumerRouteFeatureId::StreamingEvents => "feature.streaming-events",
                ConsumerRouteFeatureId::CancellationOrInterruption => {
                    "feature.cancellation-or-interruption"
                }
                ConsumerRouteFeatureId::WorkingResource => "feature.working-resource",
                ConsumerRouteFeatureId::ActivityObservation => "feature.activity-observation",
                ConsumerRouteFeatureId::PreparedFacade => "feature.prepared-facade",
                other => panic!("unexpected feature {other:?}"),
            },
            other => panic!("no selectable control is admitted: {other:?}"),
        })
        .collect()
}

fn rows(
    contribution: &ConsumerRouteProjectionContribution,
) -> impl Iterator<Item = &swallowtail_runtime::ConsumerRouteProjectionRow> {
    contribution
        .selection_rows()
        .chain(contribution.session_start_rows())
        .chain(contribution.active_session_rows())
}

fn source(value: &str) -> ConsumerRouteProjectionSourceId {
    ConsumerRouteProjectionSourceId::new(value).expect("source")
}

fn census_tuples() -> BTreeSet<(String, String, String)> {
    include_str!("fixtures/consumer-route-projection-census.csv")
        .lines()
        .skip(1)
        .filter_map(|line| {
            let mut fields = line.split(',');
            let route = fields.next()?;
            let shape = fields.next()?;
            let semantic = fields.next()?;
            (route == "copilot-cli.acp")
                .then(|| (route.to_owned(), shape.to_owned(), semantic.to_owned()))
        })
        .collect()
}
