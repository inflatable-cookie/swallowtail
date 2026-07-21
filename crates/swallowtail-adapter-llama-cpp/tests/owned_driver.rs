#[path = "owned_driver/failures.rs"]
mod failures;
mod owned_support;

use futures_executor::block_on;
use owned_support::{
    FixtureServer, OwnedCall, OwnedFixture, ProcessStop, PropertiesFixture, ScriptedOwnedServices,
    StreamFixture, assert_order,
};
use swallowtail_adapter_llama_cpp::{LlamaCppOwnedDriver, llama_cpp_owned_descriptor};
use swallowtail_core::{DriverRole, HostServiceKind, InstanceOwnership};
use swallowtail_runtime::{
    CleanupOutcome, Deadline, MonotonicInstant, ScopeId, ServingInstanceDriver, ServingInstanceId,
    StartServingRequest,
};

const STARTUP_SUCCESS: &str =
    include_str!("fixtures/llama-cpp-b10069-owned/startup-success.stderr");
const STARTUP_DUPLICATE: &str =
    include_str!("fixtures/llama-cpp-b10069-owned/startup-duplicate.stderr");
const STARTUP_MALFORMED: &str =
    include_str!("fixtures/llama-cpp-b10069-owned/startup-malformed.stderr");
const STARTUP_NON_LOOPBACK: &str =
    include_str!("fixtures/llama-cpp-b10069-owned/startup-non-loopback.stderr");

#[test]
fn descriptor_keeps_owned_lifecycle_and_b10069_facade_explicit() {
    let descriptor = llama_cpp_owned_descriptor();
    assert_eq!(
        descriptor.identity().id().as_str(),
        "swallowtail.llama-cpp.owned-b10069-openai-chat"
    );
    for role in [
        DriverRole::ServingInstanceLifecycle,
        DriverRole::ModelCatalog,
        DriverRole::StructuredRun,
    ] {
        assert!(descriptor.supports_role(role));
    }
    let services: Vec<_> = descriptor
        .required_host_services(DriverRole::ServingInstanceLifecycle)
        .collect();
    for required in [
        HostServiceKind::Process,
        HostServiceKind::ModelArtifact,
        HostServiceKind::ServingEndpoint,
        HostServiceKind::Network,
    ] {
        assert!(services.contains(&required));
    }
}

#[test]
fn start_waits_for_exact_readiness_and_stop_releases_in_joined_order() {
    let server =
        FixtureServer::start_with(PropertiesFixture::VersionMismatch, StreamFixture::Success);
    let startup = STARTUP_SUCCESS.replace("{{ENDPOINT}}", server.endpoint());
    let fixture = OwnedFixture::new(
        server,
        ScriptedOwnedServices::new(startup, ProcessStop::Graceful),
    );
    let handle = start(&fixture).expect("owned serving becomes ready");
    assert_eq!(handle.ownership(), InstanceOwnership::HostOwnedEphemeral);
    assert_eq!(
        handle.serving_instance_id().as_str(),
        "llama-cpp-owned-fixture"
    );
    assert_eq!(
        fixture.server.targets(),
        ["/health", "/props", "/v1/models"]
    );
    assert_eq!(
        fixture.owned.arguments(),
        [
            "--model",
            "/private/models/fixture.gguf",
            "--alias",
            "swallowtail-fixture-stories260k",
            "--host",
            "127.0.0.1",
            "--port",
            "0",
            "--offline",
            "--no-ui",
            "--no-agent",
        ]
    );
    assert_eq!(block_on(handle.stop()), CleanupOutcome::Clean);
    assert_order(
        &fixture.owned.calls(),
        &[
            OwnedCall::ArtifactAcquire,
            OwnedCall::ProcessStart,
            OwnedCall::EndpointPublish,
            OwnedCall::NetworkAuthorize,
            OwnedCall::GracefulStop,
            OwnedCall::ProcessWait,
            OwnedCall::EndpointRelease,
            OwnedCall::ArtifactRelease,
        ],
    );
}

#[test]
fn stop_escalates_only_for_the_owned_child_then_joins_before_release() {
    let server =
        FixtureServer::start_with(PropertiesFixture::VersionMismatch, StreamFixture::Success);
    let startup = STARTUP_SUCCESS.replace("{{ENDPOINT}}", server.endpoint());
    let fixture = OwnedFixture::new(
        server,
        ScriptedOwnedServices::new(startup, ProcessStop::NeedsForce),
    );
    let handle = start(&fixture).expect("owned serving becomes ready");
    assert_eq!(block_on(handle.stop()), CleanupOutcome::Clean);
    assert_order(
        &fixture.owned.calls(),
        &[
            OwnedCall::GracefulStop,
            OwnedCall::ProcessWait,
            OwnedCall::ForceStop,
            OwnedCall::EndpointRelease,
            OwnedCall::ArtifactRelease,
        ],
    );
}

#[test]
fn malformed_duplicate_and_non_loopback_startup_fail_before_publication() {
    for (label, startup, expected) in [
        (
            "malformed",
            STARTUP_MALFORMED,
            "swallowtail.llama_cpp.serving_endpoint_invalid",
        ),
        (
            "duplicate",
            STARTUP_DUPLICATE,
            "swallowtail.llama_cpp.serving_endpoint_duplicate",
        ),
        (
            "non-loopback",
            STARTUP_NON_LOOPBACK,
            "swallowtail.llama_cpp.serving_endpoint_invalid",
        ),
    ] {
        let fixture = OwnedFixture::new(
            FixtureServer::start(),
            ScriptedOwnedServices::new(startup, ProcessStop::Graceful),
        );
        let error = start(&fixture).err().expect(label);
        assert_eq!(error.diagnostic().code(), expected);
        assert!(!format!("{error:?}").contains("49152"));
        assert_eq!(
            fixture
                .owned
                .calls()
                .iter()
                .filter(|call| **call == OwnedCall::EndpointPublish)
                .count(),
            0
        );
        assert_order(
            &fixture.owned.calls(),
            &[
                OwnedCall::ProcessStart,
                OwnedCall::GracefulStop,
                OwnedCall::ProcessWait,
                OwnedCall::ArtifactRelease,
            ],
        );
    }
}

#[test]
fn duplicate_reported_during_http_readiness_still_prevents_a_handle() {
    let server =
        FixtureServer::start_with(PropertiesFixture::VersionMismatch, StreamFixture::Success);
    let first = format!("srv listening on {}\n", server.endpoint()).into_bytes();
    let second = format!("srv listening on {}\n", server.endpoint()).into_bytes();
    let fixture = OwnedFixture::new(
        server,
        ScriptedOwnedServices::with_chunks([first, second], ProcessStop::Graceful),
    );
    let error = start(&fixture).err().expect("late duplicate fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.llama_cpp.serving_endpoint_duplicate"
    );
    assert_order(
        &fixture.owned.calls(),
        &[
            OwnedCall::EndpointPublish,
            OwnedCall::GracefulStop,
            OwnedCall::ProcessWait,
            OwnedCall::EndpointRelease,
            OwnedCall::ArtifactRelease,
        ],
    );
}

#[test]
fn early_exit_and_build_mismatch_take_the_same_joined_cleanup_path() {
    let exited = OwnedFixture::new(FixtureServer::start(), ScriptedOwnedServices::exited());
    let error = start(&exited).err().expect("early exit fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.llama_cpp.serving_process_exited"
    );
    assert_order(
        &exited.owned.calls(),
        &[
            OwnedCall::ProcessStart,
            OwnedCall::GracefulStop,
            OwnedCall::ProcessWait,
            OwnedCall::ArtifactRelease,
        ],
    );

    let server = FixtureServer::start_with(PropertiesFixture::Expected, StreamFixture::Success);
    let startup = STARTUP_SUCCESS.replace("{{ENDPOINT}}", server.endpoint());
    let mismatch = OwnedFixture::new(
        server,
        ScriptedOwnedServices::new(startup, ProcessStop::Graceful),
    );
    let error = start(&mismatch).err().expect("wrong build fails");
    assert_eq!(
        error.diagnostic().code(),
        "swallowtail.llama_cpp.serving_build_mismatch"
    );
    assert_order(
        &mismatch.owned.calls(),
        &[
            OwnedCall::EndpointPublish,
            OwnedCall::GracefulStop,
            OwnedCall::ProcessWait,
            OwnedCall::EndpointRelease,
            OwnedCall::ArtifactRelease,
        ],
    );
}

fn start(
    fixture: &OwnedFixture,
) -> Result<Box<dyn swallowtail_runtime::OwnedServingHandle>, swallowtail_runtime::RuntimeFailure> {
    let plan = fixture.plan();
    block_on(LlamaCppOwnedDriver::new().start(
        plan,
        StartServingRequest::new(
            ScopeId::new("llama-cpp-owned-scope").expect("scope is valid"),
            ServingInstanceId::new("llama-cpp-owned-fixture").expect("serving id is valid"),
            fixture.artifact(),
            Deadline::at(MonotonicInstant::from_ticks(10_000)),
        ),
        fixture.services(),
    ))
}
