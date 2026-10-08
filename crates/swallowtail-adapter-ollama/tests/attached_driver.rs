mod support;

use futures_executor::block_on;
use futures_util::StreamExt;
use std::num::NonZeroU64;
use std::time::Duration;
use support::{CatalogueFixture, Fixture, FixtureServer, StreamFixture, VersionFixture};
use swallowtail_adapter_ollama::{OllamaNativeAttachedDriver, ollama_native_descriptor};
use swallowtail_core::{
    AttachedModelObservationScope, AttachedRuntimeResidency, DriverRole, HostServiceKind,
};
use swallowtail_runtime::{
    HostServices, ModelCatalogDriver, ModelCatalogRequest, OperationContent, OperationPolicy,
    ProviderObservation, RequestId, StructuredRunDriver, StructuredRunRequest, TerminalStatus,
};

#[test]
fn descriptor_has_no_process_credential_or_serving_authority() {
    let descriptor = ollama_native_descriptor();
    assert_eq!(
        descriptor.identity().id().as_str(),
        "swallowtail.ollama.native-attached"
    );
    assert!(descriptor.supports_role(DriverRole::ModelCatalog));
    assert!(descriptor.supports_role(DriverRole::StructuredRun));
    assert!(!descriptor.supports_role(DriverRole::ServingInstanceLifecycle));
    assert!(
        descriptor
            .required_host_services(DriverRole::StructuredRun)
            .all(|service| !matches!(
                service,
                HostServiceKind::Process
                    | HostServiceKind::Credential
                    | HostServiceKind::ModelArtifact
                    | HostServiceKind::ServingEndpoint
            ))
    );
}

#[test]
fn catalogue_retains_installed_running_and_selected_detail_observations() {
    let fixture = Fixture::new();
    let models = block_on(OllamaNativeAttachedDriver::new().list_models(
        fixture.plan(DriverRole::ModelCatalog),
        ModelCatalogRequest::new(RequestId::new("catalog").expect("request id is valid")),
        fixture.services(),
    ))
    .expect("catalogue succeeds");

    assert_eq!(models.len(), 1);
    assert_eq!(models[0].id().as_str(), "fixture-model:8b");
    assert_eq!(models[0].provider_id(), None);
    let observations: Vec<_> = models[0].metadata().attached_model_observations().collect();
    assert_eq!(observations.len(), 3);
    assert!(observations.iter().any(|observation| {
        observation.scope() == AttachedModelObservationScope::InstalledInventory
    }));
    assert!(observations.iter().any(|observation| {
        observation.scope() == AttachedModelObservationScope::RunningInventory
    }));
    assert!(observations.iter().any(|observation| {
        observation.scope() == AttachedModelObservationScope::SelectedModelDetail
    }));
    assert!(observations.iter().all(|observation| {
        observation.observed_at().epoch_seconds() == 1_700_000_000
            && observation.observed_at().subsecond_nanos() == 42
    }));
    assert_eq!(
        fixture.server.targets(),
        ["/api/version", "/api/tags", "/api/ps", "/api/show"]
    );
    assert_eq!(fixture.server.inference_attempts(), 0);
}

#[test]
fn one_native_stream_attempt_preserves_output_usage_and_external_residency() {
    let fixture = Fixture::new();
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
    assert_eq!(
        fixture.server.targets(),
        [
            "/api/version",
            "/api/tags",
            "/api/ps",
            "/api/show",
            "/api/chat"
        ]
    );
    assert!(matches!(
        block_on(run.close()),
        swallowtail_runtime::CleanupOutcome::Clean
    ));
    assert!(
        fixture.server.is_reachable(),
        "attached close must not stop Ollama"
    );
}

include!("attached_driver/failures.rs");

#[test]
fn manifest_list_pins_selected_digest_and_keeps_sibling_observations() {
    let fixture = Fixture::with_server(FixtureServer::start_with_catalogue(
        VersionFixture::Expected,
        StreamFixture::Success,
        CatalogueFixture::ManifestList,
    ));
    let models = block_on(OllamaNativeAttachedDriver::new().list_models(
        fixture.plan(DriverRole::ModelCatalog),
        ModelCatalogRequest::new(RequestId::new("manifest-list").expect("request id is valid")),
        fixture.services(),
    ))
    .expect("catalogue binds the selected digest among sibling rows");

    assert_eq!(models.len(), 1);
    let observations: Vec<_> = models[0].metadata().attached_model_observations().collect();
    assert_eq!(observations.len(), 4);
    assert_eq!(
        observations
            .iter()
            .filter(|observation| {
                observation.scope() == AttachedModelObservationScope::InstalledInventory
            })
            .count(),
        2
    );
    assert!(observations.iter().any(|observation| {
        observation.scope() == AttachedModelObservationScope::InstalledInventory
            && observation.manifest_digest().map(|digest| digest.as_str())
                == Some("sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb")
    }));
    let show: serde_json::Value =
        serde_json::from_slice(&fixture.server.show_bodies()[0]).expect("show JSON parses");
    assert_eq!(show["model"], "fixture-model:8b");
    assert_eq!(show["runner"], "ggml");
    assert_eq!(fixture.server.inference_attempts(), 0);
}

#[test]
fn selected_identity_drift_fails_before_inference() {
    let fixture = Fixture::with_server(FixtureServer::start_with_catalogue(
        VersionFixture::Expected,
        StreamFixture::Success,
        CatalogueFixture::SelectedIdentityDrift,
    ));
    let error = block_on(OllamaNativeAttachedDriver::new().start_run(
        fixture.plan(DriverRole::StructuredRun),
        run_request("identity-drift"),
        fixture.services(),
    ))
    .err()
    .expect("converted sibling is not the selected identity");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.ollama.selected_identity_drift"
    );
    assert_eq!(fixture.server.inference_attempts(), 0);
    assert!(fixture.server.is_reachable());
}

#[test]
fn unmapped_sibling_is_skipped_and_selected_ggml_row_is_pinned() {
    let fixture = Fixture::with_server(FixtureServer::start_with_catalogue(
        VersionFixture::Expected,
        StreamFixture::Success,
        CatalogueFixture::UnmappedSibling,
    ));
    let models = block_on(OllamaNativeAttachedDriver::new().list_models(
        fixture.plan(DriverRole::ModelCatalog),
        ModelCatalogRequest::new(RequestId::new("unmapped-sibling").expect("request id is valid")),
        fixture.services(),
    ))
    .expect("unmapped mlx sibling is skipped");
    let observations: Vec<_> = models[0].metadata().attached_model_observations().collect();
    assert_eq!(observations.len(), 3);
    assert!(observations.iter().all(|observation| {
        observation.manifest_digest().map(|digest| digest.as_str())
            != Some("sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc")
    }));
    let show: serde_json::Value =
        serde_json::from_slice(&fixture.server.show_bodies()[0]).expect("show JSON parses");
    assert_eq!(show["runner"], "ggml");
}

#[test]
fn inference_discloses_provider_owned_sibling_without_changing_selected_identity() {
    let fixture = Fixture::with_server(FixtureServer::start_with_catalogue(
        VersionFixture::Expected,
        StreamFixture::Success,
        CatalogueFixture::SiblingAfterChat,
    ));
    let (_, _, outcome) = complete_run(&fixture);
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    let chat: serde_json::Value =
        serde_json::from_slice(&fixture.server.inference_bodies()[0]).expect("chat JSON parses");
    assert!(chat.get("runner").is_none());
    assert_eq!(fixture.server.inference_attempts(), 1);

    let models = block_on(OllamaNativeAttachedDriver::new().list_models(
        fixture.plan(DriverRole::ModelCatalog),
        ModelCatalogRequest::new(RequestId::new("after-chat").expect("request id is valid")),
        fixture.services(),
    ))
    .expect("post-chat catalogue still binds the original digest");
    let observations: Vec<_> = models[0].metadata().attached_model_observations().collect();
    assert!(observations.iter().any(|observation| {
        observation.scope() == AttachedModelObservationScope::InstalledInventory
            && observation.manifest_digest().map(|digest| digest.as_str())
                == Some("sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")
    }));
    assert!(observations.iter().any(|observation| {
        observation.scope() == AttachedModelObservationScope::InstalledInventory
            && observation.manifest_digest().map(|digest| digest.as_str())
                == Some("sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb")
    }));
    let show: serde_json::Value = serde_json::from_slice(
        fixture
            .server
            .show_bodies()
            .last()
            .expect("post-chat show was sent"),
    )
    .expect("show JSON parses");
    assert_eq!(show["runner"], "ggml");
}

#[test]
fn local_and_remote_authoritative_hosts_share_the_same_driver_seam() {
    for host in ["host.local", "host.remote-authoritative"] {
        let fixture = Fixture::with_host(host);
        let models = block_on(OllamaNativeAttachedDriver::new().list_models(
            fixture.plan(DriverRole::ModelCatalog),
            ModelCatalogRequest::new(
                RequestId::new(format!("catalog-{host}")).expect("request id is valid"),
            ),
            fixture.services(),
        ))
        .expect("catalogue succeeds");
        assert_eq!(models[0].id().as_str(), "fixture-model:8b");
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
    complete_run_with_services(fixture, fixture.services(), "fixture-run")
}

fn complete_run_with_services(
    fixture: &Fixture,
    services: HostServices,
    request_id: &str,
) -> (
    Box<dyn swallowtail_runtime::RunHandle>,
    Vec<swallowtail_runtime::RuntimeEvent>,
    swallowtail_runtime::TerminalOutcome,
) {
    let run = block_on(OllamaNativeAttachedDriver::new().start_run(
        fixture.plan(DriverRole::StructuredRun),
        run_request(request_id),
        services,
    ))
    .expect("run starts");
    drain_run(run)
}

fn drain_run(
    mut run: Box<dyn swallowtail_runtime::RunHandle>,
) -> (
    Box<dyn swallowtail_runtime::RunHandle>,
    Vec<swallowtail_runtime::RuntimeEvent>,
    swallowtail_runtime::TerminalOutcome,
) {
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
        policy(),
    )
    .with_maximum_output_tokens(NonZeroU64::new(8).expect("limit is nonzero"))
}

fn policy() -> OperationPolicy {
    OperationPolicy::offline()
        .with_attached_runtime_residency(AttachedRuntimeResidency::RuntimeManaged)
}
