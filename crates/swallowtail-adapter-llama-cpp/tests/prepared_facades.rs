#![allow(dead_code)]

mod prepared_support;

use futures_executor::block_on;
use futures_util::StreamExt;
use prepared_support::{
    Fixture, FixtureServer, OwnedCall, OwnedFixture, ProcessStop, PropertiesFixture,
    ScriptedOwnedServices, StreamFixture,
};
use std::num::NonZeroU64;
use swallowtail_adapter_llama_cpp::{
    LlamaCppAttachedPreparationInput, LlamaCppCatalogueProfileInput, LlamaCppContextSize,
    LlamaCppInferenceProfileInput, LlamaCppModelSelection, LlamaCppOwnedPreparationInput,
    LlamaCppOwnedServingSelection, LlamaCppReasoningSelection, llama_cpp_attached_access_profile,
    llama_cpp_owned_access_profile, prepare_llama_cpp_attached, prepare_llama_cpp_owned,
};
use swallowtail_core::{
    AccessProfile, AccessStatus, ConfiguredInstanceId, CredentialState, EndpointAuthorization,
    EntitlementState, ExecutionHostId, InstanceRevision, InstanceTargetRef, ModelId, ModelRouteId,
    ModelRouteRevision, RuntimeReadiness, SupportAuthority,
};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, MonotonicInstant, OperationContent, PreparedAccessEvidence,
    RequestId, ScopeId, ServingInstanceId, TerminalStatus,
};
use swallowtail_testkit::{
    assert_observable_activity_not_applicable, assert_observable_activity_trace,
};

const STARTUP_SUCCESS: &str =
    include_str!("fixtures/llama-cpp-b10069-owned/startup-success.stderr");

#[test]
fn attached_facade_binds_catalogue_and_inference_without_stop_authority() {
    for host in ["host.local", "host.remote-authoritative"] {
        let fixture = Fixture::with_host(host);
        let services = fixture.services();
        let access = llama_cpp_attached_access_profile();
        let prepared = prepare_llama_cpp_attached(
            LlamaCppAttachedPreparationInput::new(
                ConfiguredInstanceId::new(format!("llama-cpp.attached.{host}")).unwrap(),
                InstanceRevision::new("1").unwrap(),
                ExecutionHostId::new(host).unwrap(),
                InstanceTargetRef::new("llama-cpp-fixture-endpoint").unwrap(),
                access.clone(),
                evidence(&access),
            ),
            &services,
        )
        .expect("attached integration prepares");
        assert_eq!(prepared.expected_build(), "9910");
        assert_eq!(prepared.expected_commit(), "f5525f7e7");

        let catalogue = prepared
            .prepare_catalogue(LlamaCppCatalogueProfileInput::new(
                RequestId::new(format!("catalogue-{host}")).unwrap(),
            ))
            .expect("catalogue prepares");
        assert_observable_activity_not_applicable(catalogue.evidence().operation());
        let models = block_on(catalogue.list_models(services.clone())).expect("catalogue succeeds");
        assert_eq!(models[0].id().as_str(), "swallowtail-fixture-stories260k");

        let attempt = prepared
            .prepare_inference_attempt(LlamaCppInferenceProfileInput::new(
                RequestId::new(format!("run-{host}")).unwrap(),
                model_selection("llama-cpp-b9910"),
                OperationContent::new("Fixture prompt").unwrap(),
                NonZeroU64::new(8).unwrap(),
            ))
            .expect("inference prepares");
        let mut run = block_on(attempt.start_run(services)).expect("run starts");
        let mut events = run.take_events().expect("events are available");
        let terminal = run.take_terminal_outcome().expect("terminal is available");
        let collected = block_on(async {
            let mut collected = Vec::new();
            while let Some(event) = events.next().await {
                collected.push(event.expect("event succeeds"));
            }
            collected
        });
        assert_observable_activity_trace(attempt.evidence().observable_activity(), &collected);
        assert_eq!(block_on(terminal).status(), &TerminalStatus::Completed);
        assert_eq!(block_on(run.close()), CleanupOutcome::Clean);
        assert!(fixture.server.is_reachable());
    }
}

#[test]
fn attached_build_drift_fails_before_catalogue_inventory() {
    let fixture = Fixture::with_server(FixtureServer::start_with(
        PropertiesFixture::VersionMismatch,
        StreamFixture::Success,
    ));
    let services = fixture.services();
    let access = llama_cpp_attached_access_profile();
    let prepared = prepare_llama_cpp_attached(
        LlamaCppAttachedPreparationInput::new(
            ConfiguredInstanceId::new("llama-cpp.attached.drift").unwrap(),
            InstanceRevision::new("1").unwrap(),
            ExecutionHostId::new("host.llama-cpp").unwrap(),
            InstanceTargetRef::new("llama-cpp-fixture-endpoint").unwrap(),
            access.clone(),
            evidence(&access),
        ),
        &services,
    )
    .unwrap();
    let catalogue = prepared
        .prepare_catalogue(LlamaCppCatalogueProfileInput::new(
            RequestId::new("catalogue-drift").unwrap(),
        ))
        .unwrap();
    let error = block_on(catalogue.list_models(services)).expect_err("build drift fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.llama_cpp.version_mismatch"
    );
    assert_eq!(fixture.server.targets(), ["/health", "/props"]);
}

#[test]
fn owned_facade_returns_only_after_readiness_and_preserves_cleanup_order() {
    for host in ["host.local", "host.remote-authoritative"] {
        let server =
            FixtureServer::start_with(PropertiesFixture::VersionMismatch, StreamFixture::Success);
        let startup = STARTUP_SUCCESS.replace("{{ENDPOINT}}", server.endpoint());
        let fixture = OwnedFixture::for_host(
            server,
            ScriptedOwnedServices::new(startup, ProcessStop::Graceful),
            ExecutionHostId::new(host).unwrap(),
        );
        let services = fixture.services();
        let access = llama_cpp_owned_access_profile();
        let prepared = prepare_llama_cpp_owned(
            LlamaCppOwnedPreparationInput::new(
                ConfiguredInstanceId::new(format!("llama-cpp.owned.{host}")).unwrap(),
                InstanceRevision::new("1").unwrap(),
                fixture.host_id(),
                InstanceTargetRef::new("llama-server.b10069").unwrap(),
                access.clone(),
                evidence(&access),
                LlamaCppOwnedServingSelection::new(
                    fixture.artifact(),
                    model_selection("llama-cpp-b10069"),
                ),
            ),
            &services,
        )
        .expect("owned integration prepares");
        let start = prepared
            .prepare_serving_start(
                ScopeId::new(format!("owned-scope-{host}")).unwrap(),
                ServingInstanceId::new(format!("owned-instance-{host}")).unwrap(),
                Deadline::at(MonotonicInstant::from_ticks(10_000)),
            )
            .expect("serving start prepares");
        assert_observable_activity_not_applicable(start.evidence().operation());
        assert_eq!(start.evidence().artifact(), &fixture.artifact());
        assert_eq!(start.request().artifact(), prepared.artifact());
        let handle = block_on(start.start(services)).expect("ready handle is returned");
        assert_eq!(
            fixture.server.targets(),
            ["/health", "/props", "/v1/models"]
        );
        assert_eq!(block_on(handle.stop()), CleanupOutcome::Clean);
        let calls = fixture.owned.calls();
        let endpoint_release = position(&calls, OwnedCall::EndpointRelease);
        let artifact_release = position(&calls, OwnedCall::ArtifactRelease);
        assert!(endpoint_release < artifact_release);
    }
}

#[test]
fn owned_preparation_rejects_host_drift_before_effects() {
    let fixture = OwnedFixture::new(FixtureServer::start(), ScriptedOwnedServices::exited());
    let services = fixture.services();
    let access = llama_cpp_owned_access_profile();
    let error = prepare_llama_cpp_owned(
        LlamaCppOwnedPreparationInput::new(
            ConfiguredInstanceId::new("llama-cpp.owned.wrong-host").unwrap(),
            InstanceRevision::new("1").unwrap(),
            ExecutionHostId::new("host.other").unwrap(),
            InstanceTargetRef::new("llama-server.b10069").unwrap(),
            access.clone(),
            evidence(&access),
            LlamaCppOwnedServingSelection::new(
                fixture.artifact(),
                model_selection("llama-cpp-b10069"),
            ),
        ),
        &services,
    )
    .err()
    .expect("host drift fails");
    assert_eq!(
        error.stage(),
        swallowtail_runtime::PreparationStage::TargetSelection
    );
    assert!(fixture.owned.calls().is_empty());
}

include!("prepared_facades/selections.rs");
include!("prepared_facades/support.rs");
