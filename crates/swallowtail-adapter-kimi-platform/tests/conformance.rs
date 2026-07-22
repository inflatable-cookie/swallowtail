#[allow(dead_code)]
#[path = "direct_driver/fixture.rs"]
mod fixture;
#[allow(dead_code)]
mod support;

use fixture::Fixture;
use futures_executor::block_on;
use futures_util::StreamExt;
use std::num::NonZeroU64;
use swallowtail_adapter_kimi_platform::KimiPlatformDirectDriver;
use swallowtail_core::{CredentialMechanism, DriverRole, ReasoningMode};
use swallowtail_runtime::{
    CleanupOutcome, ModelCatalogDriver, ModelCatalogRequest, OperationContent, OperationPolicy,
    ProviderObservation, RequestId, RuntimeEventKind, StructuredRunDriver, StructuredRunRequest,
    TerminalStatus,
};
use swallowtail_testkit::{
    ConformanceAssertion, ExecutionTopologyFixture, SyntheticProfile, run_all_synthetic_profiles,
    run_hosted_direct_api_profile,
};

#[test]
fn provider_neutral_hosted_profile_covers_kimi_direct_boundaries() {
    let report = run_hosted_direct_api_profile();
    assert_eq!(report.profile(), SyntheticProfile::HostedDirectApi);
    for assertion in [
        ConformanceAssertion::PreflightBeforeSideEffects,
        ConformanceAssertion::BoundSelection,
        ConformanceAssertion::OrderedEvents,
        ConformanceAssertion::SingleTerminalOutcome,
        ConformanceAssertion::CancellationAndTimeoutDistinct,
        ConformanceAssertion::CleanupRemainsVisible,
        ConformanceAssertion::Redaction,
        ConformanceAssertion::NoImplicitFallback,
        ConformanceAssertion::HostedApiNeedsNoProcess,
        ConformanceAssertion::HostedEndpointCredentialBinding,
        ConformanceAssertion::DirectRunNoResource,
        ConformanceAssertion::DirectRunOutputBound,
        ConformanceAssertion::ProviderEvidenceSeparated,
    ] {
        assert!(report.covers(assertion), "missing {assertion:?}");
    }
}

#[test]
fn local_and_remote_authority_preserve_the_same_exact_k3_lifecycle() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        assert_topology(&topology);
    }
}

#[test]
fn all_provider_neutral_profiles_keep_the_common_contract() {
    let reports = run_all_synthetic_profiles();
    assert_eq!(reports.len(), 11);
    assert_eq!(
        reports
            .iter()
            .filter(|report| report.profile() == SyntheticProfile::HostedDirectApi)
            .count(),
        1
    );
    for report in reports {
        assert!(report.covers(ConformanceAssertion::PreflightBeforeSideEffects));
        assert!(report.covers(ConformanceAssertion::NoImplicitFallback));
        assert!(report.covers(ConformanceAssertion::Redaction));
    }
}

fn assert_topology(topology: &ExecutionTopologyFixture) {
    let fixture = Fixture::for_topology(topology);
    let driver = KimiPlatformDirectDriver::new();
    let catalogue_plan = fixture.plan(DriverRole::ModelCatalog);
    assert_eq!(
        catalogue_plan.execution_host_id(),
        topology.execution_host_id()
    );
    assert_eq!(
        catalogue_plan.instance_id(),
        topology.configured_instance_id()
    );
    assert_eq!(
        catalogue_plan.instance_target_ref(),
        topology.instance_target()
    );
    assert_eq!(
        catalogue_plan.endpoint_audience().as_str(),
        "api.moonshot.ai"
    );
    assert_eq!(
        catalogue_plan.credential_mechanism(),
        &CredentialMechanism::ApiKey
    );

    let models = block_on(driver.list_models(
        catalogue_plan,
        ModelCatalogRequest::new(RequestId::new("conformance-catalogue").expect("request id")),
        fixture.services(),
    ))
    .expect("catalogue succeeds");
    assert_eq!(models.len(), 1);
    assert_eq!(models[0].id().as_str(), "kimi-k3");
    assert_eq!(
        models[0]
            .provider_id()
            .expect("catalogue provider")
            .as_str(),
        "moonshot"
    );
    assert_eq!(
        models[0]
            .metadata()
            .catalog_observations()
            .expect("source-scoped observations")
            .source()
            .as_str(),
        "kimi-platform"
    );

    let run_plan = fixture.plan(DriverRole::StructuredRun);
    assert_eq!(run_plan.execution_host_id(), topology.execution_host_id());
    assert_eq!(run_plan.instance_id(), topology.configured_instance_id());
    assert_eq!(
        run_plan.model_route_id().expect("route").as_str(),
        "kimi-platform-k3"
    );
    assert_eq!(run_plan.model_id().expect("model").as_str(), "kimi-k3");
    assert_eq!(
        run_plan.provider_id().expect("provider").as_str(),
        "moonshot"
    );

    let request = StructuredRunRequest::new(
        RequestId::new("conformance-run").expect("request id"),
        OperationContent::new("private topology prompt").expect("content"),
        OperationPolicy::offline()
            .with_reasoning_mode(ReasoningMode::new("high").expect("reasoning mode")),
    )
    .with_maximum_output_tokens(NonZeroU64::new(128).expect("output bound"));
    let mut run =
        block_on(driver.start_run(run_plan, request, fixture.services())).expect("run starts");
    let mut stream = run.take_events().expect("events");
    let terminal = run.take_terminal_outcome().expect("terminal");
    let (events, outcome) = block_on(async {
        let mut events = Vec::new();
        while let Some(event) = stream.next().await {
            events.push(event.expect("event"));
        }
        (events, terminal.await)
    });
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(outcome.output().expect("output").as_str(), "Fixture answer");
    assert!(
        events
            .iter()
            .any(|event| event.kind() == &RuntimeEventKind::ReasoningProgress)
    );
    assert!(events.iter().any(|event| matches!(
        event.kind(),
        RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(_))
    )));
    assert!(!format!("{events:?}").contains("private topology prompt"));
    assert!(!format!("{outcome:?}").contains("Fixture answer"));
    assert_eq!(fixture.server.attempts(), 1);
    assert_eq!(fixture.server.requests().len(), 2);
    assert_eq!(fixture.releases(), 2);
    assert_eq!(fixture.release_after_blocking(), [1, 2]);
    assert!(matches!(block_on(run.close()), CleanupOutcome::Clean));
}
