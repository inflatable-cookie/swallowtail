use swallowtail_core::{Diagnostic, EndpointAudience, ReasoningMode, SafeDiagnostic};
use swallowtail_runtime::{
    AttachmentDescriptor, AttachmentRef, AttachmentRole, CancellationControl, CredentialRef,
    Deadline, EndpointRef, ExecutableRef, ExternalNetworkPolicy, ExternalSearchPolicy,
    ImmediateCancellation, LeaseCleanupAuthority, MonotonicInstant, OperationPolicy,
    ProcessRequest, RequestId, ResourceAccess, ResourceRepresentation, SchemaDocument, SchemaRef,
    ScopeId, StructuredOutputDescriptor, StructuredRunRequest, WorkingResourceRef,
};
use swallowtail_testkit::{
    RecordedHostCall, RecordingHostServices, RecordingOutcome,
    assert_cleanup_states_remain_distinct, assert_dynamic_role_registration_and_calls,
    assert_missing_roles_are_explicit, poll_immediate,
};

#[test]
fn dynamic_roles_are_object_safe_registered_and_callable() {
    assert_dynamic_role_registration_and_calls();
    assert_missing_roles_are_explicit();
}

#[test]
fn every_recording_host_service_exposes_attempted_effects() {
    let recording = RecordingHostServices::default();
    let services = recording.services();
    let scope = ScopeId::new("fixture-scope").expect("scope is valid");

    let task = services
        .task()
        .expect("task service is registered")
        .spawn(scope.clone(), Box::pin(async {}))
        .expect("task spawn is scripted to succeed");
    poll_immediate(task.join()).expect("task join is scripted to succeed");
    poll_immediate(
        services
            .blocking_work()
            .expect("blocking service is registered")
            .run(scope.clone(), Box::new(|| Ok(()))),
    )
    .expect("blocking work is scripted to succeed");
    let time = services.time().expect("time service is registered");
    assert_eq!(time.now(), MonotonicInstant::from_ticks(17));
    let deadline = Deadline::at(MonotonicInstant::from_ticks(20));
    let observation = poll_immediate(time.wait_until(deadline));
    assert_eq!(observation.deadline(), deadline);
    assert_eq!(observation.observed_at(), deadline.instant());

    let executable = ExecutableRef::new("fixture-executable").expect("reference is valid");
    let process = poll_immediate(
        services
            .process()
            .expect("process service is registered")
            .start(scope.clone(), ProcessRequest::new(executable)),
    )
    .expect("process start is scripted to succeed");
    poll_immediate(process.request_stop()).expect("graceful stop succeeds");
    poll_immediate(process.force_stop()).expect("force stop succeeds");
    poll_immediate(process.wait()).expect("wait succeeds");

    let endpoint = EndpointRef::new("fixture-endpoint").expect("reference is valid");
    let audience = EndpointAudience::new("fixture-audience").expect("audience is valid");
    poll_immediate(
        services
            .network()
            .expect("network service is registered")
            .authorize(scope.clone(), endpoint, audience.clone()),
    )
    .expect("network authorization succeeds");
    let credential = CredentialRef::new("fixture-credential").expect("reference is valid");
    let credential_lease = poll_immediate(
        services
            .credential()
            .expect("credential service is registered")
            .acquire(scope.clone(), credential, audience),
    )
    .expect("credential acquisition succeeds");
    assert_eq!(
        poll_immediate(
            services
                .credential()
                .expect("credential service is registered")
                .release(credential_lease),
        ),
        swallowtail_runtime::CleanupOutcome::Clean
    );
    let resource = WorkingResourceRef::new("fixture-resource").expect("reference is valid");
    let borrowed = poll_immediate(
        services
            .working_resource()
            .expect("resource service is registered")
            .resolve(
                scope.clone(),
                resource,
                ResourceAccess::Read,
                ResourceRepresentation::Stream,
            ),
    )
    .expect("resource resolution succeeds");
    assert_eq!(
        borrowed.cleanup_authority(),
        LeaseCleanupAuthority::Consumer
    );
    assert_eq!(
        poll_immediate(
            services
                .working_resource()
                .expect("resource service is registered")
                .release(borrowed),
        ),
        swallowtail_runtime::CleanupOutcome::NotApplicable
    );
    let temporary = poll_immediate(
        services
            .working_resource()
            .expect("resource service is registered")
            .create_temporary(
                scope.clone(),
                ResourceAccess::ReadWrite,
                ResourceRepresentation::TemporaryFile,
            ),
    )
    .expect("temporary resource creation succeeds");
    assert_eq!(
        temporary.cleanup_authority(),
        LeaseCleanupAuthority::OperationScope
    );
    assert_eq!(
        poll_immediate(
            services
                .working_resource()
                .expect("resource service is registered")
                .release(temporary),
        ),
        swallowtail_runtime::CleanupOutcome::Clean
    );
    let attachment = AttachmentRef::new("fixture-attachment").expect("reference is valid");
    let attachment_descriptor =
        AttachmentDescriptor::new(attachment, "image/png", AttachmentRole::Input)
            .expect("attachment descriptor is valid");
    let attachment_file = poll_immediate(
        services
            .attachment()
            .expect("attachment service is registered")
            .materialize_file(scope.clone(), attachment_descriptor),
    )
    .expect("attachment file materialization succeeds");
    assert_eq!(
        attachment_file.cleanup_authority(),
        LeaseCleanupAuthority::OperationScope
    );
    assert_eq!(
        attachment_file.file().as_driver_value(),
        "/private/recording/attachment.png"
    );
    assert!(!format!("{attachment_file:?}").contains("/private/recording"));
    assert_eq!(
        poll_immediate(
            services
                .attachment()
                .expect("attachment service is registered")
                .release_file(attachment_file),
        ),
        swallowtail_runtime::CleanupOutcome::Clean
    );
    let schema_file = poll_immediate(
        services
            .schema()
            .expect("schema service is registered")
            .materialize_file(
                scope,
                SchemaDocument::inline(br#"{"type":"object"}"#.to_vec(), 1024)
                    .expect("schema is bounded"),
            ),
    )
    .expect("schema file materialization succeeds");
    assert_eq!(
        schema_file.cleanup_authority(),
        LeaseCleanupAuthority::OperationScope
    );
    assert_eq!(
        schema_file.file().as_driver_value(),
        "/private/recording/schema.json"
    );
    assert!(!format!("{schema_file:?}").contains("/private/recording"));
    assert_eq!(
        poll_immediate(
            services
                .schema()
                .expect("schema service is registered")
                .release_file(schema_file),
        ),
        swallowtail_runtime::CleanupOutcome::Clean
    );
    services
        .diagnostic_observer()
        .expect("diagnostic observer is registered")
        .observe(&Diagnostic::new(SafeDiagnostic::new(
            "fixture.diagnostic",
            "Fixture diagnostic",
        )));

    for expected in [
        RecordedHostCall::TaskSpawn,
        RecordedHostCall::TaskJoin,
        RecordedHostCall::BlockingWork,
        RecordedHostCall::TimeNow,
        RecordedHostCall::TimeWaitUntil,
        RecordedHostCall::ProcessStart,
        RecordedHostCall::ProcessGracefulStop,
        RecordedHostCall::ProcessForceStop,
        RecordedHostCall::ProcessWait,
        RecordedHostCall::NetworkAuthorize,
        RecordedHostCall::CredentialAcquire,
        RecordedHostCall::CredentialRelease,
        RecordedHostCall::WorkingResourceResolve,
        RecordedHostCall::WorkingResourceCreateTemporary,
        RecordedHostCall::AttachmentMaterializeFile,
        RecordedHostCall::AttachmentFileRelease,
        RecordedHostCall::SchemaMaterializeFile,
        RecordedHostCall::SchemaFileRelease,
        RecordedHostCall::DiagnosticObserve,
    ] {
        assert_eq!(recording.count(expected), 1, "missing {expected:?}");
    }
    assert_eq!(recording.count(RecordedHostCall::WorkingResourceRelease), 2);
}

#[test]
fn operation_policy_requires_explicit_search_authority_and_reasoning() {
    let rejected =
        OperationPolicy::new(ExternalNetworkPolicy::Denied, ExternalSearchPolicy::Enabled)
            .expect_err("search cannot imply network authority");
    assert_eq!(
        rejected.diagnostic().code(),
        "swallowtail.operation_policy_rejected"
    );

    let reasoning = ReasoningMode::new("low").expect("reasoning mode is valid");
    let policy = OperationPolicy::new(
        ExternalNetworkPolicy::HostApproved,
        ExternalSearchPolicy::Enabled,
    )
    .expect("explicit network and search policy is valid")
    .with_reasoning_mode(reasoning.clone());
    assert_eq!(policy.reasoning_mode(), Some(&reasoning));

    let request = StructuredRunRequest::new(
        RequestId::new("policy-request").expect("request id is valid"),
        swallowtail_runtime::OperationContent::new("private prompt").expect("content is valid"),
        policy,
    )
    .with_working_resource(WorkingResourceRef::new("policy-resource").expect("resource is valid"));
    assert_eq!(
        request.policy().external_network(),
        ExternalNetworkPolicy::HostApproved
    );
    assert_eq!(
        request.policy().external_search(),
        ExternalSearchPolicy::Enabled
    );
}

#[test]
fn direct_structured_run_needs_no_placeholder_working_resource() {
    let request = StructuredRunRequest::new(
        RequestId::new("direct-request").expect("request id is valid"),
        swallowtail_runtime::OperationContent::new("private prompt").expect("content is valid"),
        OperationPolicy::offline(),
    );

    assert!(request.working_resource().is_none());
}

#[test]
fn cancellation_is_idempotent_and_cleanup_states_stay_distinct() {
    let cancellation =
        ImmediateCancellation::new(swallowtail_core::CancellationScope::StructuredRun);
    assert_eq!(
        poll_immediate(cancellation.request()).expect("first request succeeds"),
        swallowtail_runtime::CancellationAcknowledgement::Requested
    );
    assert_eq!(
        poll_immediate(cancellation.request()).expect("second request succeeds"),
        swallowtail_runtime::CancellationAcknowledgement::AlreadyRequested
    );
    assert_cleanup_states_remain_distinct();
}

#[test]
fn scripted_host_failure_is_recorded_before_rejection() {
    let recording = RecordingHostServices::new(RecordingOutcome::Fail(SafeDiagnostic::new(
        "fixture.host_rejected",
        "Host rejected operation",
    )));
    let endpoint = EndpointRef::new("fixture-endpoint").expect("reference is valid");
    let scope = ScopeId::new("failure-scope").expect("scope is valid");
    let audience = EndpointAudience::new("failure-audience").expect("audience is valid");
    let result = poll_immediate(
        recording
            .services()
            .network()
            .expect("network service is registered")
            .authorize(scope, endpoint, audience),
    );

    assert!(result.is_err());
    assert_eq!(recording.count(RecordedHostCall::NetworkAuthorize), 1);
}

#[test]
fn portable_schema_is_transport_only_and_redacted() {
    let schema = StructuredOutputDescriptor::new(
        SchemaDocument::inline(br#"{"type":"object","secret":"value"}"#.to_vec(), 4_096)
            .expect("schema is within the bound"),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .expect("inline schema descriptor is valid");
    assert_eq!(schema.dialect(), "json-schema-2020-12");
    assert!(!format!("{schema:?}").contains("secret"));

    let reference = SchemaRef::new("host-owned-schema").expect("reference is valid");
    assert!(!format!("{reference:?}").contains(reference.as_host_value()));

    let too_large =
        SchemaDocument::inline(vec![0; 5], 4).expect_err("oversized inline schema must fail");
    assert_eq!(too_large.maximum(), 4);
    assert_eq!(too_large.actual(), 5);
}
