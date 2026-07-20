mod support;

use futures_executor::block_on;
use support::{
    FakeProcessService, host_services, host_services_for, host_services_with, plan_with,
    working_resource,
};
use swallowtail_adapter_codex::CodexExecDriver;
use swallowtail_core::{
    Capability, CapabilityConstraint, CapabilityRequirement, ExecutionHostId, HostServiceKind,
    ReasoningMode,
};
use swallowtail_runtime::{
    AttachmentDescriptor, AttachmentRef, AttachmentRole, EnvironmentRef, ExternalNetworkPolicy,
    ExternalSearchPolicy, OperationContent, OperationPolicy, RequestId, StructuredRunDriver,
    StructuredRunRequest,
};
use swallowtail_testkit::{RecordedHostCall, RecordingHostServices};

#[test]
fn reasoning_selection_must_match_the_exact_preflight_constraint() {
    let (process, state) = FakeProcessService::completed("");
    let low = ReasoningMode::new("low").expect("reasoning mode is valid");
    let high = ReasoningMode::new("high").expect("reasoning mode is valid");
    let plan = plan_with(
        [CapabilityRequirement::new(
            Capability::ReasoningSelection,
            [CapabilityConstraint::reasoning_mode(low)],
        )],
        [],
    );
    let request = request(
        "reasoning-mismatch",
        OperationPolicy::offline().with_reasoning_mode(high),
    );

    let failure = block_on(driver().start_run(plan, request, host_services(process)))
        .err()
        .expect("mismatched reasoning must fail");

    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.codex.exec.request_plan_mismatch"
    );
    assert!(!state.started());
}

#[test]
fn preflight_bound_optional_service_must_exist_at_execution() {
    let (process, state) = FakeProcessService::completed("");
    let plan = plan_with(
        [CapabilityRequirement::new(Capability::ExternalSearch, [])],
        [HostServiceKind::Network],
    );
    let policy = OperationPolicy::new(
        ExternalNetworkPolicy::HostApproved,
        ExternalSearchPolicy::Enabled,
    )
    .expect("search policy is explicit");

    let failure = block_on(driver().start_run(
        plan,
        request("network-service-missing", policy),
        host_services(process),
    ))
    .err()
    .expect("missing runtime service must fail");

    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.codex.exec.host_service_missing"
    );
    assert!(!state.started());
}

#[test]
fn non_image_attachment_rejects_before_materialization_or_process_start() {
    let (process, state) = FakeProcessService::completed("");
    let recording = RecordingHostServices::default();
    let plan = plan_with(
        [CapabilityRequirement::new(
            Capability::Attachments,
            [
                CapabilityConstraint::attachment_media_type("application/pdf")
                    .expect("media type is valid"),
                CapabilityConstraint::AttachmentMaximumBytes(1024),
                CapabilityConstraint::AttachmentMaximumCount(1),
            ],
        )],
        [HostServiceKind::Attachment],
    );
    let attachment = AttachmentDescriptor::new(
        AttachmentRef::new("document.main").expect("attachment reference is valid"),
        "application/pdf",
        AttachmentRole::Input,
    )
    .expect("attachment descriptor is valid")
    .with_known_length(512);
    let request = request("non-image", OperationPolicy::offline()).with_attachments([attachment]);

    let failure = block_on(driver().start_run(
        plan,
        request,
        host_services_with(process, &recording, [HostServiceKind::Attachment]),
    ))
    .err()
    .expect("non-image attachment must fail");

    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.codex.exec.unsupported_input"
    );
    assert!(!state.started());
    assert_eq!(
        recording.count(RecordedHostCall::AttachmentMaterializeFile),
        0
    );
}

#[test]
fn runtime_services_must_belong_to_the_preflight_bound_host() {
    let (process, state) = FakeProcessService::completed("");
    let plan = plan_with([], []);
    let failure = block_on(driver().start_run(
        plan,
        request("wrong-host", OperationPolicy::offline()),
        host_services_for(
            ExecutionHostId::new("host.remote").expect("host id is valid"),
            process,
        ),
    ))
    .err()
    .expect("wrong-host services must fail");

    assert_eq!(
        failure.diagnostic().code(),
        "swallowtail.execution_host_mismatch"
    );
    assert!(!state.started());
}

fn request(id: &str, policy: OperationPolicy) -> StructuredRunRequest {
    StructuredRunRequest::new(
        RequestId::new(id).expect("request id is valid"),
        OperationContent::new("bounded prompt").expect("content is valid"),
        working_resource(),
        policy,
    )
}

fn driver() -> CodexExecDriver {
    CodexExecDriver::new(EnvironmentRef::new("codex-saved-login").expect("environment is valid"))
}
