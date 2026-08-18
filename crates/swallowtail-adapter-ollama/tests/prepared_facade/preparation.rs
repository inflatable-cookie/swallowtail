use super::fixtures::{attempt_input, inventory_input, preparation_input, prepared, probe};
use crate::support::{Fixture, FixtureServer, StreamFixture, VersionFixture};
use futures_executor::block_on;
use swallowtail_adapter_ollama::prepare_ollama_attached;
use swallowtail_core::{InstanceTargetRef, InterfaceCompatibilityAssessment, ReasoningMode};
use swallowtail_runtime::{
    CancellationControl, CleanupOutcome, DiscoveryCancellation, PreparationStage, TerminalStatus,
};

#[test]
fn exact_stable_newer_is_visible_while_known_exclusion_stays_closed() {
    let newer = Fixture::with_server(FixtureServer::start_with(
        VersionFixture::Newer,
        StreamFixture::Success,
    ));
    let prepared = prepared(&newer);
    let InterfaceCompatibilityAssessment::UnverifiedNewer(assessment) =
        prepared.runtime().compatibility()
    else {
        panic!("newer stable Ollama must remain visibly unverified");
    };
    assert_eq!(assessment.version().as_str(), "0.32.15");
    assert_eq!(
        prepared
            .instance()
            .interface_versions()
            .next()
            .unwrap()
            .version()
            .as_str(),
        "0.32.15"
    );
    let inventory = prepared
        .prepare_inventory(inventory_input("newer-inventory"))
        .expect("unverified newer plan prepares");
    block_on(inventory.observe_inventory(newer.services()))
        .expect("unverified newer executes through qualified behavior");
    let attempt = prepared
        .prepare_inference_attempt(
            attempt_input("newer-reasoning")
                .with_reasoning_mode(ReasoningMode::new("high").expect("mode is valid")),
        )
        .expect("unverified newer control uses the latest qualified mapping");
    let mut run = block_on(attempt.start_run(newer.services())).expect("run starts");
    let outcome = block_on(
        run.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(block_on(run.close()), CleanupOutcome::Clean);

    let excluded = Fixture::with_server(FixtureServer::start_with(
        VersionFixture::Excluded,
        StreamFixture::Success,
    ));
    let failure = block_on(prepare_ollama_attached(
        preparation_input(&excluded),
        probe(&excluded, DiscoveryCancellation::new()),
        excluded.services(),
    ))
    .expect_err("known excluded version remains incompatible");
    assert_eq!(
        failure.stage(),
        PreparationStage::CompatibilityClassification
    );
    assert_eq!(excluded.server.targets(), ["/api/version"]);
}

#[test]
fn cancellation_and_binding_drift_fail_without_inference_or_server_ownership() {
    let cancelled = Fixture::new();
    let cancellation = DiscoveryCancellation::new();
    block_on(cancellation.request()).expect("cancellation is requested");
    let failure = block_on(prepare_ollama_attached(
        preparation_input(&cancelled),
        probe(&cancelled, cancellation),
        cancelled.services(),
    ))
    .expect_err("cancelled preparation fails");
    assert_eq!(failure.stage(), PreparationStage::BoundedOutput);
    assert!(cancelled.server.targets().is_empty());
    assert_eq!(cancelled.server.inference_attempts(), 0);
    assert!(cancelled.server.is_reachable());

    let fixture = Fixture::new();
    let prepared = prepared(&fixture);
    let failure = prepared
        .validate_execution_binding(
            fixture.host_id(),
            &InstanceTargetRef::new("other.ollama.endpoint").unwrap(),
        )
        .expect_err("endpoint drift is rejected");
    assert_eq!(failure.stage(), PreparationStage::TargetSelection);
}

#[test]
fn runtime_version_drift_after_preparation_fails_before_inventory_or_inference() {
    let fixture = Fixture::with_server(FixtureServer::start_with(
        VersionFixture::DriftAfterPreparation,
        StreamFixture::Success,
    ));
    let prepared = prepared(&fixture);
    assert_eq!(
        prepared.runtime().runtime_version().version().as_str(),
        "0.30.0"
    );
    let inventory = prepared
        .prepare_inventory(inventory_input("drift-inventory"))
        .expect("inventory plan prepares");
    let failure = block_on(inventory.observe_inventory(fixture.services()))
        .expect_err("runtime drift rejects operation");
    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.ollama.version_drift"
    );
    assert_eq!(fixture.server.version_requests(), 2);
    assert_eq!(fixture.server.inference_attempts(), 0);
    assert!(fixture.server.is_reachable());
}
