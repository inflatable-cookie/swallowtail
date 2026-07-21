use crate::{
    ConformanceAssertion, ConformanceReport, ExecutionTopologyFixture, ProfilePreflightFixture,
    RecordedHostCall, RecordingHostServices, RecordingOutcome, SyntheticProfile,
    assert_common_contract, poll_immediate,
};
use std::num::NonZeroU64;
use swallowtail_core::{Capability, DriverRole, ExecutionLayer, HostServiceKind, OperationShape};
use swallowtail_runtime::{
    BilledCostObservation, CleanupOutcome, CredentialRef, Currency, EndpointRef,
    OpenSessionRequest, ProviderObservation, RequestId, RuntimeTurnId, ScopeId,
};

pub(crate) fn run() -> ConformanceReport {
    let profile = SyntheticProfile::ConnectionScopedDirectSession;
    let mut report = ConformanceReport::new(profile);
    assert_common_contract(profile, &mut report);

    let fixture = ProfilePreflightFixture::new(profile);
    let plan = fixture
        .preflight()
        .expect("direct-session preflight succeeds");
    assert_eq!(
        plan.requirements().execution_layer(),
        ExecutionLayer::DirectModelInference
    );
    assert_eq!(
        plan.requirements().operation_shape(),
        OperationShape::InteractiveSession
    );
    let required: Vec<_> = fixture
        .driver()
        .required_host_services(DriverRole::InteractiveSession)
        .collect();
    for service in [
        HostServiceKind::Task,
        HostServiceKind::BlockingWork,
        HostServiceKind::Time,
        HostServiceKind::Network,
        HostServiceKind::Credential,
    ] {
        assert!(required.contains(&service));
    }
    assert!(!required.contains(&HostServiceKind::Process));
    assert!(!required.contains(&HostServiceKind::WorkingResource));
    assert!(
        !plan
            .requirements()
            .capabilities()
            .any(|requirement| requirement.capability() == Capability::Resume)
    );

    let open = OpenSessionRequest::resource_free(
        RequestId::new("direct-session-open").expect("request id is valid"),
        None,
    );
    assert!(open.working_resource().is_none());
    assert!(open.access_policy().resource_access().is_none());

    assert_session_leases_and_cleanup(&plan);
    assert_billed_cost_scope(&plan);
    assert_topology_authority();

    report.record(ConformanceAssertion::SessionLifecycle);
    report.record(ConformanceAssertion::HostTopologyPreserved);
    report.record(ConformanceAssertion::HostedEndpointCredentialBinding);
    report.record(ConformanceAssertion::DirectSessionNoResource);
    report.record(ConformanceAssertion::ConnectionScopedLeaseLifecycle);
    report.record(ConformanceAssertion::BilledCostTurnScoped);
    report.record(ConformanceAssertion::NoImplicitSessionRecovery);
    report
}

fn assert_session_leases_and_cleanup(plan: &swallowtail_core::PreflightPlan) {
    let recording = RecordingHostServices::for_host(
        plan.execution_host_id().clone(),
        RecordingOutcome::Succeed,
    );
    let services = recording.services();
    let scope = ScopeId::new("direct-session-scope").expect("scope is valid");
    let audience = plan.endpoint_audience().clone();
    let grant = poll_immediate(
        services
            .network()
            .expect("network service exists")
            .authorize(
                scope.clone(),
                EndpointRef::new("direct-session-endpoint").expect("endpoint is valid"),
                audience.clone(),
            ),
    )
    .expect("endpoint is authorized");
    let credential = poll_immediate(
        services
            .credential()
            .expect("credential service exists")
            .acquire(
                scope.clone(),
                CredentialRef::new("direct-session-credential").expect("credential is valid"),
                audience.clone(),
            ),
    )
    .expect("credential is acquired");
    assert_eq!(grant.scope(), &scope);
    assert_eq!(grant.audience(), &audience);
    assert_eq!(credential.scope(), &scope);
    assert_eq!(credential.audience(), &audience);
    assert!(!format!("{grant:?}").contains("recording.invalid"));

    let task = services
        .task()
        .expect("task service exists")
        .spawn(scope.clone(), Box::pin(async {}))
        .expect("session task starts");
    poll_immediate(task.join()).expect("session task joins");
    poll_immediate(
        services
            .blocking_work()
            .expect("blocking service exists")
            .run(scope, Box::new(|| Ok(()))),
    )
    .expect("connection close completes");
    assert_eq!(
        poll_immediate(
            services
                .credential()
                .expect("credential service exists")
                .release(credential),
        ),
        CleanupOutcome::Clean
    );
    let calls = recording.calls();
    assert!(
        position(&calls, RecordedHostCall::TaskJoin)
            < position(&calls, RecordedHostCall::BlockingWork)
    );
    assert!(
        position(&calls, RecordedHostCall::BlockingWork)
            < position(&calls, RecordedHostCall::CredentialRelease)
    );
    assert_eq!(recording.count(RecordedHostCall::NetworkAuthorize), 1);
    assert_eq!(recording.count(RecordedHostCall::CredentialAcquire), 1);
    assert_eq!(recording.count(RecordedHostCall::CredentialRelease), 1);
    assert_eq!(recording.count(RecordedHostCall::ProcessStart), 0);
    assert_eq!(recording.count(RecordedHostCall::WorkingResourceResolve), 0);
}

fn assert_billed_cost_scope(plan: &swallowtail_core::PreflightPlan) {
    let cost = BilledCostObservation::provider_reported(
        125_000,
        Currency::Usd,
        NonZeroU64::new(10_000_000_000).expect("scale is nonzero"),
        RuntimeTurnId::new("direct-session-turn").expect("turn is valid"),
        plan.model_route_id().expect("route is bound").clone(),
        plan.access_profile_id().clone(),
        NonZeroU64::new(1).expect("attempt is nonzero"),
    );
    assert_eq!(cost.turn_id().as_str(), "direct-session-turn");
    assert_eq!(
        cost.model_route_id(),
        plan.model_route_id().expect("route is bound")
    );
    assert_eq!(cost.access_profile_id(), plan.access_profile_id());
    assert_eq!(cost.provider_attempt().get(), 1);
    assert!(matches!(
        ProviderObservation::BilledCost(cost),
        ProviderObservation::BilledCost(_)
    ));
}

fn assert_topology_authority() {
    for topology in [
        ExecutionTopologyFixture::local(),
        ExecutionTopologyFixture::remote_authoritative(),
    ] {
        let recording = RecordingHostServices::for_host(
            topology.execution_host_id().clone(),
            RecordingOutcome::Succeed,
        );
        recording
            .services()
            .require_execution_host(topology.execution_host_id())
            .expect("topology authority is retained");
    }
}

fn position(calls: &[RecordedHostCall], expected: RecordedHostCall) -> usize {
    calls
        .iter()
        .position(|call| *call == expected)
        .expect("expected host call exists")
}
