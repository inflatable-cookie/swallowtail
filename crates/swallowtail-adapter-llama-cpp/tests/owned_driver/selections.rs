//! Adapter-local owned-serving selection argv: `--ctx-size N` and `--reasoning off`.

use super::*;

#[test]
fn selected_context_size_is_appended_and_omission_stays_eleven_arguments() {
    let server =
        FixtureServer::start_with(PropertiesFixture::VersionMismatch, StreamFixture::Success);
    let startup = STARTUP_SUCCESS.replace("{{ENDPOINT}}", server.endpoint());
    let fixture = OwnedFixture::new(
        server,
        ScriptedOwnedServices::new(startup, ProcessStop::Graceful),
    );
    let selected = LlamaCppContextSize::from_u64(2048).expect("admitted value");
    let handle = start_with(&fixture, Some(selected), None).expect("owned serving becomes ready");
    assert_eq!(
        fixture.owned.arguments(),
        [
            "--model".to_owned(),
            "/private/models/fixture.gguf".to_owned(),
            "--alias".to_owned(),
            "swallowtail-fixture-stories260k".to_owned(),
            "--host".to_owned(),
            "127.0.0.1".to_owned(),
            "--port".to_owned(),
            "0".to_owned(),
            "--offline".to_owned(),
            "--no-ui".to_owned(),
            "--no-agent".to_owned(),
            "--ctx-size".to_owned(),
            "2048".to_owned(),
        ]
    );
    assert_eq!(block_on(handle.stop()), CleanupOutcome::Clean);
}

#[test]
fn selected_context_size_keeps_joined_cleanup_on_build_mismatch() {
    let server = FixtureServer::start();
    let startup = STARTUP_SUCCESS.replace("{{ENDPOINT}}", server.endpoint());
    let mismatch = OwnedFixture::new(
        server,
        ScriptedOwnedServices::new(startup, ProcessStop::Graceful),
    );
    let selected = LlamaCppContextSize::from_u64(512).expect("admitted value");
    let error = start_with(&mismatch, Some(selected), None)
        .err()
        .expect("wrong build fails");
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

#[test]
fn selected_reasoning_dispatches_the_canonical_off_literal() {
    let server =
        FixtureServer::start_with(PropertiesFixture::VersionMismatch, StreamFixture::Success);
    let startup = STARTUP_SUCCESS.replace("{{ENDPOINT}}", server.endpoint());
    let fixture = OwnedFixture::new(
        server,
        ScriptedOwnedServices::new(startup, ProcessStop::Graceful),
    );
    let handle = start_with(&fixture, None, Some(LlamaCppReasoningSelection::Disabled))
        .expect("owned serving becomes ready");
    assert_eq!(
        fixture.owned.arguments(),
        [
            "--model".to_owned(),
            "/private/models/fixture.gguf".to_owned(),
            "--alias".to_owned(),
            "swallowtail-fixture-stories260k".to_owned(),
            "--host".to_owned(),
            "127.0.0.1".to_owned(),
            "--port".to_owned(),
            "0".to_owned(),
            "--offline".to_owned(),
            "--no-ui".to_owned(),
            "--no-agent".to_owned(),
            "--reasoning".to_owned(),
            "off".to_owned(),
        ]
    );
    assert_eq!(block_on(handle.stop()), CleanupOutcome::Clean);
}

#[test]
fn reasoning_composes_after_context_size_in_canonical_argv() {
    let server =
        FixtureServer::start_with(PropertiesFixture::VersionMismatch, StreamFixture::Success);
    let startup = STARTUP_SUCCESS.replace("{{ENDPOINT}}", server.endpoint());
    let fixture = OwnedFixture::new(
        server,
        ScriptedOwnedServices::new(startup, ProcessStop::Graceful),
    );
    let selected = LlamaCppContextSize::from_u64(4096).expect("admitted value");
    let handle = start_with(
        &fixture,
        Some(selected),
        Some(LlamaCppReasoningSelection::Disabled),
    )
    .expect("owned serving becomes ready");
    assert_eq!(
        fixture.owned.arguments(),
        [
            "--model".to_owned(),
            "/private/models/fixture.gguf".to_owned(),
            "--alias".to_owned(),
            "swallowtail-fixture-stories260k".to_owned(),
            "--host".to_owned(),
            "127.0.0.1".to_owned(),
            "--port".to_owned(),
            "0".to_owned(),
            "--offline".to_owned(),
            "--no-ui".to_owned(),
            "--no-agent".to_owned(),
            "--ctx-size".to_owned(),
            "4096".to_owned(),
            "--reasoning".to_owned(),
            "off".to_owned(),
        ]
    );
    assert_eq!(block_on(handle.stop()), CleanupOutcome::Clean);
}

#[test]
fn selected_reasoning_keeps_joined_cleanup_on_build_mismatch() {
    let server = FixtureServer::start();
    let startup = STARTUP_SUCCESS.replace("{{ENDPOINT}}", server.endpoint());
    let mismatch = OwnedFixture::new(
        server,
        ScriptedOwnedServices::new(startup, ProcessStop::Graceful),
    );
    let error = start_with(&mismatch, None, Some(LlamaCppReasoningSelection::Disabled))
        .err()
        .expect("wrong build fails");
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
