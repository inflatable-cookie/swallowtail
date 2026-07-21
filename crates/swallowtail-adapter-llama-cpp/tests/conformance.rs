mod owned_support;

use futures_executor::block_on;
use owned_support::{
    FixtureServer, OwnedCall, OwnedFixture, ProcessStop, PropertiesFixture, ScriptedOwnedServices,
    StreamFixture, assert_order,
};
use swallowtail_adapter_llama_cpp::LlamaCppOwnedDriver;
use swallowtail_core::InstanceOwnership;
use swallowtail_runtime::{
    CleanupOutcome, Deadline, MonotonicInstant, ScopeId, ServingInstanceDriver, ServingInstanceId,
    StartServingRequest,
};
use swallowtail_testkit::{
    ConformanceAssertion, ExecutionTopologyFixture, SyntheticProfile,
    run_attached_self_hosted_profile, run_owned_self_hosted_profile,
};

const STARTUP_SUCCESS: &str =
    include_str!("fixtures/llama-cpp-b10069-owned/startup-success.stderr");

#[test]
fn provider_neutral_attached_profile_covers_llama_cpp_boundaries() {
    let report = run_attached_self_hosted_profile();
    assert_eq!(report.profile(), SyntheticProfile::AttachedSelfHosted);
    for assertion in [
        ConformanceAssertion::PreflightBeforeSideEffects,
        ConformanceAssertion::BoundSelection,
        ConformanceAssertion::StalePlanRejected,
        ConformanceAssertion::OrderedEvents,
        ConformanceAssertion::SingleTerminalOutcome,
        ConformanceAssertion::CancellationAndTimeoutDistinct,
        ConformanceAssertion::CleanupRemainsVisible,
        ConformanceAssertion::ExternalOwnershipPreserved,
        ConformanceAssertion::Redaction,
        ConformanceAssertion::NoImplicitFallback,
        ConformanceAssertion::AttachedServiceNeverStopped,
    ] {
        assert!(report.covers(assertion), "missing {assertion:?}");
    }
}

#[test]
fn provider_neutral_owned_profile_covers_llama_cpp_boundaries() {
    let report = run_owned_self_hosted_profile();
    assert_eq!(report.profile(), SyntheticProfile::OwnedSelfHosted);
    for assertion in [
        ConformanceAssertion::PreflightBeforeSideEffects,
        ConformanceAssertion::BoundSelection,
        ConformanceAssertion::StalePlanRejected,
        ConformanceAssertion::OrderedEvents,
        ConformanceAssertion::SingleTerminalOutcome,
        ConformanceAssertion::CancellationAndTimeoutDistinct,
        ConformanceAssertion::CleanupRemainsVisible,
        ConformanceAssertion::Redaction,
        ConformanceAssertion::NoImplicitFallback,
        ConformanceAssertion::HostTopologyPreserved,
        ConformanceAssertion::OwnedServiceStops,
        ConformanceAssertion::OwnedArtifactLease,
        ConformanceAssertion::OwnedEndpointBinding,
        ConformanceAssertion::OwnedCleanupOrdered,
    ] {
        assert!(report.covers(assertion), "missing {assertion:?}");
    }
}

#[test]
fn owned_driver_preserves_local_and_remote_host_scope_through_cleanup() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let server =
            FixtureServer::start_with(PropertiesFixture::VersionMismatch, StreamFixture::Success);
        let startup = STARTUP_SUCCESS.replace("{{ENDPOINT}}", server.endpoint());
        let fixture = OwnedFixture::for_host(
            server,
            ScriptedOwnedServices::new(startup, ProcessStop::Graceful),
            topology.execution_host_id().clone(),
        );
        let scope = ScopeId::new("llama-cpp-owned-conformance").expect("scope is valid");
        let handle = block_on(LlamaCppOwnedDriver::new().start(
            fixture.plan(),
            StartServingRequest::new(
                scope.clone(),
                ServingInstanceId::new("llama-cpp-owned-conformance").expect("serving id is valid"),
                fixture.artifact(),
                Deadline::at(MonotonicInstant::from_ticks(10_000)),
            ),
            fixture.services(),
        ))
        .expect("owned serving becomes ready");

        assert_eq!(handle.ownership(), InstanceOwnership::HostOwnedEphemeral);
        assert_eq!(handle.execution_host_id(), topology.execution_host_id());
        assert_eq!(handle.endpoint_binding().scope(), &scope);
        assert_eq!(
            handle.endpoint_binding().execution_host_id(),
            topology.execution_host_id()
        );
        assert_eq!(block_on(handle.stop()), CleanupOutcome::Clean);
        assert_order(
            &fixture.owned.calls(),
            &[
                OwnedCall::ProcessStart,
                OwnedCall::EndpointPublish,
                OwnedCall::ProcessWait,
                OwnedCall::EndpointRelease,
                OwnedCall::ArtifactRelease,
            ],
        );
    }
}
