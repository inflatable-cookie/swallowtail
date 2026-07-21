mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use std::num::NonZeroU64;
use std::time::Duration;
use support::{Fixture, FixtureServer, PropertiesFixture, StreamFixture};
use swallowtail_adapter_llama_cpp::{LlamaCppAttachedDriver, llama_cpp_attached_descriptor};
use swallowtail_core::DriverRole;
use swallowtail_runtime::{
    ModelCatalogDriver, ModelCatalogRequest, OperationContent, OperationPolicy,
    ProviderObservation, RequestId, StructuredRunDriver, StructuredRunRequest, TerminalStatus,
};

#[test]
fn descriptor_has_no_process_or_serving_lifecycle_authority() {
    let descriptor = llama_cpp_attached_descriptor();
    assert_eq!(
        descriptor.identity().id().as_str(),
        "swallowtail.llama-cpp.attached-openai-chat"
    );
    assert_eq!(descriptor.integration_family().as_str(), "llama.cpp");
    assert!(descriptor.supports_role(DriverRole::ModelCatalog));
    assert!(descriptor.supports_role(DriverRole::StructuredRun));
    assert!(!descriptor.supports_role(DriverRole::ServingInstanceLifecycle));
    assert!(
        descriptor
            .required_host_services(DriverRole::StructuredRun)
            .all(|service| service != swallowtail_core::HostServiceKind::Process)
    );
}

#[test]
fn catalogue_and_stream_require_observed_properties_and_leave_server_running() {
    let fixture = Fixture::new();
    let driver = LlamaCppAttachedDriver::new();
    let models = block_on(driver.list_models(
        fixture.plan(DriverRole::ModelCatalog),
        ModelCatalogRequest::new(RequestId::new("catalog").expect("request id is valid")),
        fixture.services(),
    ))
    .expect("catalogue succeeds");
    assert_eq!(models.len(), 1);
    assert_eq!(models[0].id().as_str(), "swallowtail-fixture-stories260k");
    assert_eq!(models[0].provider_id(), None);

    let (run, events, outcome) = complete_run(&fixture);
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(
        outcome.output().expect("output exists").as_str(),
        "Fixture output"
    );
    assert!(events.iter().any(|event| matches!(
        event.kind(),
        swallowtail_runtime::RuntimeEventKind::ProviderObservation(ProviderObservation::Usage(_))
    )));
    assert_eq!(fixture.server.inference_attempts(), 1);
    assert!(matches!(
        block_on(run.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    ));
    assert!(
        fixture.server.is_reachable(),
        "attached close must not stop the server"
    );
    assert_eq!(
        &fixture.server.targets()[..6],
        [
            "/health",
            "/props",
            "/v1/models",
            "/health",
            "/props",
            "/v1/chat/completions"
        ]
    );
}

#[test]
fn unobserved_build_fails_before_catalogue_or_inference() {
    let fixture = Fixture::with_server(FixtureServer::start_with(
        PropertiesFixture::VersionMismatch,
        StreamFixture::Success,
    ));
    let error = block_on(LlamaCppAttachedDriver::new().list_models(
        fixture.plan(DriverRole::ModelCatalog),
        ModelCatalogRequest::new(RequestId::new("wrong-build").expect("request id is valid")),
        fixture.services(),
    ))
    .expect_err("unobserved build fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.llama_cpp.version_mismatch"
    );
    assert_eq!(fixture.server.targets(), ["/health", "/props"]);
    assert_eq!(fixture.server.inference_attempts(), 0);
}

#[test]
fn missing_output_bound_fails_before_endpoint_work() {
    let fixture = Fixture::new();
    let request = StructuredRunRequest::new(
        RequestId::new("missing-limit").expect("request id is valid"),
        OperationContent::new("Fixture prompt").expect("content is valid"),
        OperationPolicy::offline(),
    );
    let error = block_on(LlamaCppAttachedDriver::new().start_run(
        fixture.plan(DriverRole::StructuredRun),
        request,
        fixture.services(),
    ))
    .err()
    .expect("missing bound fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.llama_cpp.output_limit_missing"
    );
    assert!(fixture.server.targets().is_empty());
}

#[test]
fn midstream_provider_error_fails_once_and_keeps_payload_redacted() {
    let fixture = Fixture::with_server(FixtureServer::start_with(
        PropertiesFixture::Expected,
        StreamFixture::MidstreamError,
    ));
    let (run, events, outcome) = complete_run(&fixture);
    assert!(matches!(
        outcome.status(),
        TerminalStatus::ProviderFailed(_)
    ));
    assert_eq!(fixture.server.inference_attempts(), 1);
    assert!(events.iter().any(|event| matches!(
        event.kind(),
        swallowtail_runtime::RuntimeEventKind::OutputDelta
    )));
    assert!(!format!("{:?}", outcome.status()).contains("raw-provider-payload"));
    assert!(matches!(
        block_on(run.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    ));
    assert!(fixture.server.is_reachable());
}

#[test]
fn cancellation_closes_owned_stream_only_and_server_remains_reachable() {
    let fixture = Fixture::with_server(FixtureServer::start_with(
        PropertiesFixture::Expected,
        StreamFixture::WaitForCancel,
    ));
    let request = run_request("cancel-run");
    let mut run = block_on(LlamaCppAttachedDriver::new().start_run(
        fixture.plan(DriverRole::StructuredRun),
        request,
        fixture.services(),
    ))
    .expect("run starts");
    let _events = run.take_events().expect("events are available");
    let terminal = run.take_terminal_outcome().expect("terminal is available");
    for _ in 0..100 {
        if fixture.server.inference_attempts() == 1 {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    block_on(run.cancellation().request()).expect("cancellation is accepted");
    let outcome = block_on(terminal);
    assert_eq!(outcome.status(), &TerminalStatus::Cancelled);
    assert!(matches!(
        block_on(run.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    ));
    assert!(fixture.server.is_reachable());
}

#[test]
fn deadline_stays_distinct_from_cancellation_and_does_not_stop_server() {
    let fixture = Fixture::with_server(FixtureServer::start_with(
        PropertiesFixture::Expected,
        StreamFixture::WaitForCancel,
    ));
    let request = run_request("deadline-run")
        .with_deadline(fixture.thread.deadline_after(Duration::from_millis(100)));
    let mut run = block_on(LlamaCppAttachedDriver::new().start_run(
        fixture.plan(DriverRole::StructuredRun),
        request,
        fixture.services(),
    ))
    .expect("run starts");
    let _events = run.take_events().expect("events are available");
    let outcome = block_on(
        run.take_terminal_outcome()
            .expect("terminal outcome is available"),
    );
    assert_eq!(outcome.status(), &TerminalStatus::TimedOut);
    assert!(matches!(
        block_on(run.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    ));
    assert!(fixture.server.is_reachable());
}

#[test]
fn local_and_remote_authoritative_hosts_use_the_same_attached_catalogue_seam() {
    for host in ["host.local", "host.remote-authoritative"] {
        let fixture = Fixture::with_host(host);
        let models = block_on(LlamaCppAttachedDriver::new().list_models(
            fixture.plan(DriverRole::ModelCatalog),
            ModelCatalogRequest::new(
                RequestId::new(format!("catalog-{host}")).expect("request id is valid"),
            ),
            fixture.services(),
        ))
        .expect("catalogue succeeds");
        assert_eq!(models[0].id().as_str(), "swallowtail-fixture-stories260k");
        assert!(fixture.server.is_reachable());
    }
}

fn complete_run(
    fixture: &Fixture,
) -> (
    Box<dyn swallowtail_runtime::RunHandle>,
    Vec<swallowtail_runtime::RuntimeEvent>,
    swallowtail_runtime::TerminalOutcome,
) {
    let mut run = block_on(LlamaCppAttachedDriver::new().start_run(
        fixture.plan(DriverRole::StructuredRun),
        run_request("fixture-run"),
        fixture.services(),
    ))
    .expect("run starts");
    let mut stream = run.take_events().expect("events are available");
    let terminal = run.take_terminal_outcome().expect("terminal is available");
    let (events, outcome) = block_on(async {
        let mut events = Vec::new();
        while let Some(event) = stream.next().await {
            events.push(event.expect("event succeeds"));
        }
        (events, terminal.await)
    });
    (run, events, outcome)
}

fn run_request(id: &str) -> StructuredRunRequest {
    StructuredRunRequest::new(
        RequestId::new(id).expect("request id is valid"),
        OperationContent::new("Fixture prompt").expect("content is valid"),
        OperationPolicy::offline(),
    )
    .with_maximum_output_tokens(NonZeroU64::new(8).expect("limit is nonzero"))
}
