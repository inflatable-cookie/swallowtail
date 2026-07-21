use crate::{
    ExecutionTopologyFixture, ProfilePreflightFixture, RecordedHostCall, RecordingHostServices,
    RecordingOutcome, SyntheticProfile, poll_immediate,
};
use swallowtail_core::EndpointAudience;
use swallowtail_runtime::{
    CleanupOutcome, Deadline, MonotonicInstant, ObservedServingEndpoint, ScopeId,
    ServingInstanceId, StartServingRequest, validate_owned_serving_start,
};

pub(super) fn assert_topology_authority() {
    let profile = SyntheticProfile::OwnedSelfHosted;
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let fixture =
            ProfilePreflightFixture::for_host(profile, topology.execution_host_id().clone());
        let plan = fixture.preflight().expect("topology preflight succeeds");
        let services = RecordingHostServices::for_host(
            topology.execution_host_id().clone(),
            RecordingOutcome::Succeed,
        );
        let scope = ScopeId::new("topology-serving-scope").expect("scope is valid");
        let request = StartServingRequest::new(
            scope.clone(),
            ServingInstanceId::new("topology-serving").expect("serving id is valid"),
            fixture
                .artifact()
                .expect("owned profile has an artifact")
                .clone(),
            Deadline::at(MonotonicInstant::from_ticks(100)),
        );
        validate_owned_serving_start(&plan, &request, services.services())
            .expect("topology-bound start is valid");
        let artifact = poll_immediate(
            services
                .services()
                .model_artifact()
                .expect("artifact service is available")
                .acquire(
                    scope.clone(),
                    topology.execution_host_id().clone(),
                    request.artifact().clone(),
                ),
        )
        .expect("topology-bound artifact resolves");
        let endpoint = poll_immediate(
            services
                .services()
                .serving_endpoint()
                .expect("endpoint service is available")
                .publish(
                    scope,
                    topology.execution_host_id().clone(),
                    EndpointAudience::new("fixture-local-runtime").expect("audience is valid"),
                    ObservedServingEndpoint::new("http://127.0.0.1:49152")
                        .expect("observation is valid"),
                ),
        )
        .expect("topology-bound endpoint publishes");
        assert_eq!(artifact.execution_host_id(), topology.execution_host_id());
        assert_eq!(
            endpoint.binding().execution_host_id(),
            topology.execution_host_id()
        );
        assert_eq!(
            poll_immediate(
                services
                    .services()
                    .serving_endpoint()
                    .expect("endpoint service is available")
                    .release(endpoint)
            ),
            CleanupOutcome::Clean
        );
        assert_eq!(
            poll_immediate(
                services
                    .services()
                    .model_artifact()
                    .expect("artifact service is available")
                    .release(artifact)
            ),
            CleanupOutcome::NotApplicable
        );
    }
}

pub(super) fn assert_order(calls: &[RecordedHostCall], expected: &[RecordedHostCall]) {
    let positions = expected
        .iter()
        .map(|expected| {
            calls
                .iter()
                .position(|call| call == expected)
                .expect("expected host call exists")
        })
        .collect::<Vec<_>>();
    assert!(positions.windows(2).all(|pair| pair[0] < pair[1]));
}
