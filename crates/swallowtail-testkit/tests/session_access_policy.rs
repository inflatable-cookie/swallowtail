use swallowtail_core::{ExecutionHostId, PreflightDimension, ResourceAccess, SessionAccessPolicy};
use swallowtail_runtime::{
    OpenSessionRequest, RequestId, ResourceRepresentation, ScopeId, WorkingResourceRef,
    validate_session_access_plan, validate_session_resource_lease,
};
use swallowtail_testkit::{
    RecordedHostCall, RecordingHostServices, RecordingOutcome, SessionAccessFixtureCase,
    SessionAccessPreflightFixture, poll_immediate,
};

fn host_id(value: &str) -> ExecutionHostId {
    ExecutionHostId::new(value).expect("fixture host id is valid")
}

#[test]
fn interactive_requests_and_preflight_keep_read_only_as_the_default() {
    let fixture = SessionAccessPreflightFixture::for_case(
        SessionAccessFixtureCase::ReadOnly,
        host_id("fixture.host.local"),
    );
    let plan = fixture.preflight().expect("read-only preflight succeeds");
    let request = OpenSessionRequest::new(
        RequestId::new("read-only-request").expect("request id is valid"),
        fixture.working_resource().clone(),
        None,
    );

    assert_eq!(request.access_policy(), &SessionAccessPolicy::read_only());
    assert_eq!(
        plan.requirements().session_access_policy(),
        Some(request.access_policy())
    );
    validate_session_access_plan(&plan, request.access_policy())
        .expect("default request matches default preflight");
    assert_eq!(fixture.provider_side_effect_count(), 0);
}

#[test]
fn bounded_workspace_requires_exact_capability_service_and_extension_binding() {
    for (case, expected) in [
        (
            SessionAccessFixtureCase::MissingWriteCapability,
            PreflightDimension::SessionAccess,
        ),
        (
            SessionAccessFixtureCase::MissingWorkingResourceService,
            PreflightDimension::SessionAccess,
        ),
        (
            SessionAccessFixtureCase::UnboundProviderRequest,
            PreflightDimension::SessionAccess,
        ),
    ] {
        let fixture = SessionAccessPreflightFixture::for_case(case, host_id("fixture.host.local"));
        let failure = fixture
            .preflight()
            .expect_err("invalid bounded-session preflight must fail");

        assert_eq!(failure.dimension(), expected);
        assert_eq!(fixture.provider_side_effect_count(), 0);
    }
}

#[test]
fn bounded_workspace_lease_retains_exact_access_and_execution_host() {
    for host in ["fixture.host.local", "fixture.host.remote-authoritative"] {
        let host = host_id(host);
        let fixture = SessionAccessPreflightFixture::for_case(
            SessionAccessFixtureCase::BoundedWorkspace,
            host.clone(),
        );
        let plan = fixture.preflight().expect("bounded preflight succeeds");
        let recording = RecordingHostServices::for_host(host, RecordingOutcome::Succeed);
        recording
            .services()
            .require_execution_host(plan.execution_host_id())
            .expect("service host matches the immutable plan");
        let lease = poll_immediate(
            recording
                .services()
                .working_resource()
                .expect("working-resource service is present")
                .resolve(
                    ScopeId::new("bounded-session-scope").expect("scope is valid"),
                    fixture.working_resource().clone(),
                    fixture.policy().resource_access(),
                    ResourceRepresentation::Filesystem,
                ),
        )
        .expect("resource resolution succeeds");

        assert_eq!(lease.access(), ResourceAccess::ReadWrite);
        validate_session_resource_lease(fixture.policy(), fixture.working_resource(), &lease)
            .expect("host lease matches the request policy");
        assert_eq!(recording.count(RecordedHostCall::WorkingResourceResolve), 1);
    }
}

#[test]
fn remote_plan_rejects_local_services_before_resource_resolution() {
    let fixture = SessionAccessPreflightFixture::for_case(
        SessionAccessFixtureCase::BoundedWorkspace,
        host_id("fixture.host.remote-authoritative"),
    );
    let plan = fixture.preflight().expect("remote preflight succeeds");
    let local =
        RecordingHostServices::for_host(host_id("fixture.host.local"), RecordingOutcome::Succeed);

    local
        .services()
        .require_execution_host(plan.execution_host_id())
        .expect_err("local services cannot impersonate the remote execution host");
    assert_eq!(local.count(RecordedHostCall::WorkingResourceResolve), 0);
}

#[test]
fn request_policy_mismatch_is_rejected_before_provider_work() {
    let fixture = SessionAccessPreflightFixture::for_case(
        SessionAccessFixtureCase::ReadOnly,
        host_id("fixture.host.local"),
    );
    let plan = fixture.preflight().expect("read-only preflight succeeds");
    let writable = SessionAccessPolicy::bounded_workspace([]);

    validate_session_access_plan(&plan, &writable)
        .expect_err("writable request cannot use a read-only plan");
    assert_eq!(fixture.provider_side_effect_count(), 0);
}

#[test]
fn resource_lease_mismatch_is_rejected_explicitly() {
    let fixture = SessionAccessPreflightFixture::for_case(
        SessionAccessFixtureCase::BoundedWorkspace,
        host_id("fixture.host.local"),
    );
    let recording = RecordingHostServices::default();
    let other = WorkingResourceRef::new("fixture.other.resource").expect("resource is valid");
    let lease = poll_immediate(
        recording
            .services()
            .working_resource()
            .expect("working-resource service is present")
            .resolve(
                ScopeId::new("bounded-session-scope").expect("scope is valid"),
                other,
                ResourceAccess::ReadWrite,
                ResourceRepresentation::Filesystem,
            ),
    )
    .expect("recording resolution succeeds");

    validate_session_resource_lease(fixture.policy(), fixture.working_resource(), &lease)
        .expect_err("a different resource lease must fail");
}
