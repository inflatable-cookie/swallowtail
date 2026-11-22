use super::{driver, make_host_id};
use crate::support::{
    CleanupEvent, PendingProcessStart, SidecarFixtureHost, SidecarScenario,
    sidecar_catalogue_selection,
};
use futures_executor::block_on;
use futures_util::FutureExt;
use std::sync::Arc;
use swallowtail_runtime::{
    Deadline, ModelCatalogDriver, ModelCatalogRequest, MonotonicInstant, RequestId,
};

#[test]
fn catalogue_lists_bounded_models_from_the_explicit_runtime() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.catalogue");
    let fixture = SidecarFixtureHost::new(SidecarScenario::Complete);
    let selected = sidecar_catalogue_selection(host_id.clone());
    let models = block_on(driver(selected.credential.clone()).list_models(
        selected.plan,
        ModelCatalogRequest::new(RequestId::new("sidecar-catalogue").expect("valid request")),
        fixture.services(host_id),
    ))
    .expect("sidecar catalogue loads");
    assert_eq!(models.len(), 2);
    assert_eq!(models[0].id().as_str(), "fixture-model");
    assert_eq!(
        models[0].provider_id().map(|provider| provider.as_str()),
        Some("fixture-provider")
    );
    assert_eq!(fixture.process_arguments(), Vec::<String>::new());
    let inputs = fixture.inputs();
    assert_eq!(inputs[0]["command"], "bootstrap");
    assert_eq!(inputs[0]["params"]["catalogueOnly"], true);
    assert!(
        inputs.iter().any(|value| value["command"] == "close"),
        "catalogue sidecar is closed and joined"
    );
    assert_eq!(
        fixture.cleanup_events(),
        [CleanupEvent::ProcessWait, CleanupEvent::CredentialRelease,]
    );
    assert!(!format!("{models:?}").contains("fixture-private"));
}

#[test]
fn catalogue_deadline_bounds_pending_process_start() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.catalogue-start-timeout");
    let fixture = SidecarFixtureHost::new(SidecarScenario::Complete).with_immediate_time();
    let selected = sidecar_catalogue_selection(host_id.clone());
    let services = fixture
        .services(host_id)
        .with_process(Arc::new(PendingProcessStart));
    let error = driver(selected.credential)
        .list_models(selected.plan, bounded_request(), services)
        .now_or_never()
        .expect("deadline bounds pending process start")
        .expect_err("no process starts before the deadline");

    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.sdk-sidecar.catalogue_timed_out"
    );
    assert_eq!(fixture.cleanup_events(), [CleanupEvent::CredentialRelease]);
}

#[test]
fn timed_out_catalogue_skips_protocol_close_and_reaps_the_child() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.catalogue-timeout");
    let fixture = SidecarFixtureHost::new(SidecarScenario::Complete)
        .with_held_command("bootstrap")
        .with_unresponsive_shutdown()
        .with_immediate_time();
    let selected = sidecar_catalogue_selection(host_id.clone());
    let error = block_on(driver(selected.credential).list_models(
        selected.plan,
        bounded_request(),
        fixture.services(host_id),
    ))
    .expect_err("catalogue times out");

    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.sdk-sidecar.catalogue_timed_out"
    );
    let inputs = fixture.inputs();
    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0]["command"], "bootstrap");
    assert_force_stopped_and_released(&fixture);
}

#[test]
fn stalled_catalogue_close_escalates_within_the_shutdown_grace() {
    let host_id = make_host_id("pi.fixture.sdk-sidecar.catalogue-close-timeout");
    let fixture = SidecarFixtureHost::new(SidecarScenario::Complete)
        .with_held_command("close")
        .with_unresponsive_shutdown();
    let selected = sidecar_catalogue_selection(host_id.clone());
    let services = fixture.services(host_id);
    let worker = std::thread::spawn(move || {
        block_on(driver(selected.credential).list_models(
            selected.plan,
            ModelCatalogRequest::new(
                RequestId::new("sidecar-catalogue-close").expect("valid request"),
            ),
            services,
        ))
        .expect_err("forced cleanup surfaces as an error after a successful catalogue")
    });
    fixture.wait_for_command("close");
    fixture.advance_time(500);

    let error = worker.join().expect("catalogue thread joins");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.pi.sdk-sidecar.catalogue_cleanup_failed"
    );
    assert_force_stopped_and_released(&fixture);
}

fn bounded_request() -> ModelCatalogRequest {
    ModelCatalogRequest::new(RequestId::new("sidecar-catalogue-deadline").expect("valid request"))
        .with_deadline(Deadline::at(MonotonicInstant::from_ticks(1_001)))
}

fn assert_force_stopped_and_released(fixture: &SidecarFixtureHost) {
    assert_eq!(fixture.force_stops(), 1);
    assert!(fixture.owned_tree_members().is_empty());
    assert_eq!(
        fixture.cleanup_events(),
        [CleanupEvent::ProcessWait, CleanupEvent::CredentialRelease]
    );
}
