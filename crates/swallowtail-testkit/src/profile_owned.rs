use crate::{
    ConformanceAssertion, ConformanceReport, ProfilePreflightFixture, RecordedHostCall,
    RecordingHostServices, RecordingOutcome, SyntheticProfile, assert_common_contract,
    poll_immediate,
};

mod topology;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use swallowtail_core::{
    EndpointAudience, ExecutionHostId, InstanceOwnership, ModelArtifactBinding,
    ModelArtifactDescriptor, ModelArtifactDigest, ModelArtifactFormat, ModelArtifactId,
    ModelArtifactRef, ModelArtifactRevision, PreflightDimension, SafeDiagnostic,
};
use swallowtail_runtime::{
    BoxFuture, CleanupOutcome, Deadline, ExecutableRef, MonotonicInstant, ObservedServingEndpoint,
    OwnedServingHandle, ProcessRequest, ScopeId, ServingEndpointBinding, ServingInstanceId,
    StartServingRequest, validate_owned_serving_start,
};

struct SyntheticOwnedHandle {
    id: ServingInstanceId,
    ownership: InstanceOwnership,
    execution_host_id: ExecutionHostId,
    endpoint: ServingEndpointBinding,
    stops: Arc<AtomicUsize>,
}

impl OwnedServingHandle for SyntheticOwnedHandle {
    fn serving_instance_id(&self) -> &ServingInstanceId {
        &self.id
    }

    fn ownership(&self) -> InstanceOwnership {
        self.ownership
    }

    fn execution_host_id(&self) -> &ExecutionHostId {
        &self.execution_host_id
    }

    fn endpoint_binding(&self) -> &ServingEndpointBinding {
        &self.endpoint
    }

    fn stop(self: Box<Self>) -> BoxFuture<'static, CleanupOutcome> {
        self.stops.fetch_add(1, Ordering::SeqCst);
        Box::pin(async { CleanupOutcome::Clean })
    }
}

pub(crate) fn run() -> ConformanceReport {
    let profile = SyntheticProfile::OwnedSelfHosted;
    let mut report = ConformanceReport::new(profile);
    assert_common_contract(profile, &mut report);

    let fixture = ProfilePreflightFixture::new(profile);
    let missing_artifact = fixture
        .preflight_without_artifact()
        .expect_err("owned preflight requires an artifact");
    assert_eq!(
        missing_artifact.dimension(),
        PreflightDimension::ModelArtifact
    );
    let plan = fixture.preflight().expect("owned preflight succeeds");
    assert_eq!(plan.ownership(), InstanceOwnership::HostOwnedEphemeral);
    assert!(plan.model_route_id().is_some());
    assert!(plan.model_artifact_binding().is_some());

    let services = RecordingHostServices::for_host(
        plan.execution_host_id().clone(),
        RecordingOutcome::Succeed,
    );
    let scope = ScopeId::new("owned-serving-scope").expect("scope is valid");
    let matching_request = StartServingRequest::new(
        scope.clone(),
        ServingInstanceId::new("owned-ephemeral").expect("serving id is valid"),
        fixture
            .artifact()
            .expect("owned profile has an artifact")
            .clone(),
        Deadline::at(MonotonicInstant::from_ticks(100)),
    );
    validate_owned_serving_start(&plan, &matching_request, services.services())
        .expect("matching owned start is valid");

    let mismatched_request = StartServingRequest::new(
        scope.clone(),
        ServingInstanceId::new("owned-mismatch").expect("serving id is valid"),
        artifact("different-artifact"),
        Deadline::at(MonotonicInstant::from_ticks(100)),
    );
    let mismatch = validate_owned_serving_start(&plan, &mismatched_request, services.services())
        .expect_err("artifact substitution is rejected");
    assert_eq!(
        mismatch.diagnostic().code(),
        "swallowtail.model_artifact_mismatch"
    );
    for call in [
        RecordedHostCall::ModelArtifactAcquire,
        RecordedHostCall::ServingEndpointPublish,
        RecordedHostCall::ProcessStart,
        RecordedHostCall::NetworkAuthorize,
    ] {
        assert_eq!(services.count(call), 0);
    }

    let artifact_lease = poll_immediate(
        services
            .services()
            .model_artifact()
            .expect("artifact service is available")
            .acquire(
                scope.clone(),
                plan.execution_host_id().clone(),
                matching_request.artifact().clone(),
            ),
    )
    .expect("recording artifact acquisition succeeds");
    assert_eq!(artifact_lease.scope(), &scope);
    assert_eq!(artifact_lease.execution_host_id(), plan.execution_host_id());

    let process = poll_immediate(
        services
            .services()
            .process()
            .expect("process service is available")
            .start(
                scope.clone(),
                ProcessRequest::new(
                    ExecutableRef::new("fixture.owned-server")
                        .expect("executable reference is valid"),
                ),
            ),
    )
    .expect("recording serving process starts");

    let endpoint_lease = poll_immediate(
        services
            .services()
            .serving_endpoint()
            .expect("serving endpoint service is available")
            .publish(
                scope.clone(),
                plan.execution_host_id().clone(),
                EndpointAudience::new("fixture-local-runtime").expect("audience is valid"),
                ObservedServingEndpoint::new("http://127.0.0.1:49152")
                    .expect("observation is valid"),
            ),
    )
    .expect("recording endpoint publication succeeds");
    let endpoint = endpoint_lease.binding().clone();

    let stops = Arc::new(AtomicUsize::new(0));
    for (suffix, ownership) in [
        ("ephemeral", InstanceOwnership::HostOwnedEphemeral),
        ("persistent", InstanceOwnership::HostOwnedPersistent),
    ] {
        let handle: Box<dyn OwnedServingHandle> = Box::new(SyntheticOwnedHandle {
            id: ServingInstanceId::new(format!("owned-{suffix}")).expect("serving id is valid"),
            ownership,
            execution_host_id: plan.execution_host_id().clone(),
            endpoint: endpoint.clone(),
            stops: Arc::clone(&stops),
        });
        assert_eq!(handle.ownership(), ownership);
        assert_eq!(handle.execution_host_id(), plan.execution_host_id());
        assert_eq!(handle.endpoint_binding(), &endpoint);
        assert_eq!(poll_immediate(handle.stop()), CleanupOutcome::Clean);
    }
    assert_eq!(stops.load(Ordering::SeqCst), 2);

    poll_immediate(process.request_stop()).expect("owned serving process stops");
    poll_immediate(process.wait()).expect("owned serving process is joined");

    assert_eq!(
        poll_immediate(
            services
                .services()
                .serving_endpoint()
                .expect("serving endpoint service is available")
                .release(endpoint_lease)
        ),
        CleanupOutcome::Clean
    );
    assert_eq!(
        poll_immediate(
            services
                .services()
                .model_artifact()
                .expect("artifact service is available")
                .release(artifact_lease)
        ),
        CleanupOutcome::NotApplicable
    );
    assert_eq!(
        services.count(RecordedHostCall::AttachmentMaterializeFile),
        0
    );
    let calls = services.calls();
    topology::assert_order(
        &calls,
        &[
            RecordedHostCall::ModelArtifactAcquire,
            RecordedHostCall::ProcessStart,
            RecordedHostCall::ServingEndpointPublish,
            RecordedHostCall::ProcessGracefulStop,
            RecordedHostCall::ProcessWait,
            RecordedHostCall::ServingEndpointRelease,
            RecordedHostCall::ModelArtifactRelease,
        ],
    );

    topology::assert_topology_authority();

    let failing = RecordingHostServices::new(RecordingOutcome::Fail(SafeDiagnostic::new(
        "fixture.resource_unavailable",
        "Resource unavailable",
    )));
    let result = poll_immediate(
        failing
            .services()
            .model_artifact()
            .expect("artifact service is available")
            .acquire(
                swallowtail_runtime::ScopeId::new("owned-artifact-scope").expect("scope is valid"),
                plan.execution_host_id().clone(),
                artifact("owned-artifact"),
            ),
    );
    assert!(result.is_err());
    assert_eq!(failing.count(RecordedHostCall::ModelArtifactAcquire), 1);
    assert_eq!(failing.count(RecordedHostCall::ProcessStart), 0);
    assert_eq!(failing.count(RecordedHostCall::ServingEndpointPublish), 0);

    report.record(ConformanceAssertion::OwnedServiceStops);
    report.record(ConformanceAssertion::OwnedArtifactLease);
    report.record(ConformanceAssertion::OwnedEndpointBinding);
    report.record(ConformanceAssertion::OwnedCleanupOrdered);
    report.record(ConformanceAssertion::HostTopologyPreserved);
    report
}

fn artifact(reference: &str) -> ModelArtifactBinding {
    ModelArtifactBinding::new(
        ModelArtifactRef::new(reference).expect("reference is valid"),
        ModelArtifactDescriptor::new(
            ModelArtifactId::new(reference).expect("id is valid"),
            ModelArtifactFormat::new("gguf").expect("format is valid"),
            ModelArtifactRevision::new("revision-1").expect("revision is valid"),
            ModelArtifactDigest::new("sha256:fixture").expect("digest is valid"),
        ),
    )
}
