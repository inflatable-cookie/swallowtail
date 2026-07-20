use crate::{
    ConformanceAssertion, ConformanceReport, ContractKernelFixture, PreflightFixtureCase,
    ProfilePreflightFixture, RecordedHostCall, RecordingHostServices, SyntheticProfile,
    assert_changed_revision_invalidates_plan, assert_extension_policies,
    assert_preflight_rejection_without_side_effects, assert_successful_preflight_binding,
    poll_immediate,
};
use futures_core::Stream;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use swallowtail_core::{
    CancellationScope, Diagnostic, InstanceOwnership, PreflightDimension, SafeDiagnostic,
};
use swallowtail_runtime::{
    AttachmentDescriptor, AttachmentRef, AttachmentRole, CancellationControl, CleanupOutcome,
    CredentialRef, ImmediateCancellation, OperationContent, ResourceAccess, ResourceRepresentation,
    RuntimeEvent, RuntimeEventKind, RuntimeEventStream, SchemaDocument, StructuredOutputDescriptor,
    TerminalOutcome, TerminalStatus, WorkingResourceRef, runtime_event_channel,
    terminal_outcome_channel,
};

pub(crate) fn assert_common_contract(profile: SyntheticProfile, report: &mut ConformanceReport) {
    assert_rejections_precede_effects();
    report.record(ConformanceAssertion::PreflightBeforeSideEffects);

    assert_successful_preflight_binding();
    let fixture = ProfilePreflightFixture::new(profile);
    let plan = fixture.preflight().expect("profile preflight must succeed");
    assert_eq!(plan.instance_id(), fixture.instance().id());
    assert_eq!(plan.ownership(), fixture.instance().ownership());
    assert_eq!(
        plan.execution_host_id(),
        fixture.instance().execution_host_id()
    );
    report.record(ConformanceAssertion::BoundSelection);

    assert_changed_revision_invalidates_plan();
    report.record(ConformanceAssertion::StalePlanRejected);

    assert_event_order();
    report.record(ConformanceAssertion::OrderedEvents);

    assert_single_terminal_outcome();
    report.record(ConformanceAssertion::SingleTerminalOutcome);

    assert_semantic_overflow();
    report.record(ConformanceAssertion::SemanticOverflowFails);

    assert_cancellation_timeout_and_join();
    report.record(ConformanceAssertion::CancellationAndTimeoutDistinct);

    assert_cleanup_visibility();
    report.record(ConformanceAssertion::CleanupRemainsVisible);

    assert_external_ownership();
    report.record(ConformanceAssertion::ExternalOwnershipPreserved);

    assert_redaction();
    report.record(ConformanceAssertion::Redaction);

    assert_scoped_inputs();
    report.record(ConformanceAssertion::ScopedInputs);

    assert_schema_is_transport_only();
    report.record(ConformanceAssertion::SchemaTransportOnly);

    assert_extension_policies(ContractKernelFixture::canonical().event_with_extension());
    report.record(ConformanceAssertion::ExtensionPolicyExplicit);

    assert_no_implicit_fallback();
    report.record(ConformanceAssertion::NoImplicitFallback);
}

fn assert_rejections_precede_effects() {
    for (case, dimension) in [
        (PreflightFixtureCase::MissingRole, PreflightDimension::Role),
        (
            PreflightFixtureCase::MissingHostService,
            PreflightDimension::HostService,
        ),
        (
            PreflightFixtureCase::MissingCapability,
            PreflightDimension::Capability,
        ),
        (
            PreflightFixtureCase::MissingConstraint,
            PreflightDimension::Constraint,
        ),
        (
            PreflightFixtureCase::MissingReasoningMode,
            PreflightDimension::Constraint,
        ),
        (
            PreflightFixtureCase::MissingExternalSearch,
            PreflightDimension::Capability,
        ),
        (
            PreflightFixtureCase::MissingSchemaService,
            PreflightDimension::HostService,
        ),
    ] {
        assert_preflight_rejection_without_side_effects(case, dimension);
    }
}

fn assert_event_order() {
    let (sender, mut stream) = runtime_event_channel(3).expect("capacity is valid");
    sender
        .send(RuntimeEvent::new(1, RuntimeEventKind::Started))
        .expect("start is accepted");
    sender
        .send(RuntimeEvent::new(2, RuntimeEventKind::Progress))
        .expect("progress is accepted");
    sender
        .send(RuntimeEvent::new(3, RuntimeEventKind::OutputAvailable))
        .expect("output is accepted");
    sender.mark_terminal();

    let sequences: Vec<_> = (0..3)
        .map(|_| {
            poll_stream_item(&mut stream)
                .expect("stream contains an event")
                .expect("event is successful")
                .sequence()
        })
        .collect();
    assert_eq!(sequences, [1, 2, 3]);
    assert!(poll_stream_item(&mut stream).is_none());
}

fn assert_single_terminal_outcome() {
    let (sender, future) = terminal_outcome_channel();
    let expected = TerminalOutcome::new(TerminalStatus::Completed, CleanupOutcome::Clean);
    sender
        .complete(expected.clone())
        .expect("first terminal outcome wins");
    assert!(
        sender
            .complete(TerminalOutcome::new(
                TerminalStatus::Cancelled,
                CleanupOutcome::Clean,
            ))
            .is_err()
    );
    assert_eq!(poll_immediate(future), expected);
}

fn assert_semantic_overflow() {
    let (sender, mut stream) = runtime_event_channel(1).expect("capacity is valid");
    sender
        .send(RuntimeEvent::new(1, RuntimeEventKind::Started))
        .expect("start is accepted");
    sender
        .send(RuntimeEvent::new(2, RuntimeEventKind::OutputAvailable))
        .expect_err("semantic overflow must fail");
    assert!(
        poll_stream_item(&mut stream)
            .expect("start remains")
            .is_ok()
    );
    assert!(
        poll_stream_item(&mut stream)
            .expect("overflow is surfaced")
            .is_err()
    );
}

fn assert_cancellation_timeout_and_join() {
    let cancellation = ImmediateCancellation::new(CancellationScope::StructuredRun);
    poll_immediate(cancellation.request()).expect("cancellation request succeeds");
    assert_ne!(TerminalStatus::Cancelled, TerminalStatus::TimedOut);

    let recording = RecordingHostServices::default();
    let task = recording
        .services()
        .task()
        .expect("task service is available")
        .spawn(
            swallowtail_runtime::ScopeId::new("profile-scope").expect("scope is valid"),
            Box::pin(async {}),
        )
        .expect("task spawn succeeds");
    poll_immediate(task.join()).expect("task join succeeds");
    assert_eq!(recording.count(RecordedHostCall::TaskJoin), 1);
}

fn assert_cleanup_visibility() {
    let diagnostic = SafeDiagnostic::new("fixture.cleanup_failed", "Cleanup failed");
    let outcome = TerminalOutcome::new(
        TerminalStatus::Completed,
        CleanupOutcome::Failed(diagnostic.clone()),
    );
    assert_eq!(outcome.status(), &TerminalStatus::Completed);
    assert_eq!(outcome.cleanup(), &CleanupOutcome::Failed(diagnostic));
}

fn assert_external_ownership() {
    let fixture = ProfilePreflightFixture::new(SyntheticProfile::AttachedSelfHosted);
    let plan = fixture.preflight().expect("attached preflight succeeds");
    assert_eq!(plan.ownership(), InstanceOwnership::ExternalAttached);

    let recording = RecordingHostServices::default();
    assert_eq!(recording.count(RecordedHostCall::ProcessForceStop), 0);
}

fn assert_redaction() {
    let credential = CredentialRef::new("/private/credential/path").expect("reference is valid");
    assert!(!format!("{credential:?}").contains(credential.as_host_value()));
    let diagnostic = Diagnostic::new(SafeDiagnostic::new(
        "fixture.provider_failed",
        "Provider failed",
    ))
    .with_internal_detail("secret provider detail");
    assert!(!format!("{diagnostic:?}").contains("secret provider detail"));

    let content = OperationContent::new("private prompt body").expect("content is valid");
    let event = RuntimeEvent::with_content(1, RuntimeEventKind::OutputAvailable, content.clone());
    let outcome =
        TerminalOutcome::new(TerminalStatus::Completed, CleanupOutcome::Clean).with_output(content);
    assert!(!format!("{event:?}").contains("private prompt body"));
    assert!(!format!("{outcome:?}").contains("private prompt body"));
}

fn assert_scoped_inputs() {
    let recording = RecordingHostServices::default();
    let resource = WorkingResourceRef::new("profile-resource").expect("reference is valid");
    let lease = poll_immediate(
        recording
            .services()
            .working_resource()
            .expect("resource service is available")
            .resolve(
                swallowtail_runtime::ScopeId::new("profile-resource-scope")
                    .expect("scope is valid"),
                resource,
                ResourceAccess::Read,
                ResourceRepresentation::Stream,
            ),
    )
    .expect("resource resolution succeeds");
    assert_eq!(
        poll_immediate(
            recording
                .services()
                .working_resource()
                .expect("resource service is available")
                .release(lease),
        ),
        CleanupOutcome::NotApplicable
    );
    let attachment = AttachmentRef::new("profile-attachment").expect("reference is valid");
    let descriptor = AttachmentDescriptor::new(attachment, "image/png", AttachmentRole::Input)
        .expect("attachment descriptor is valid");
    let lease = poll_immediate(
        recording
            .services()
            .attachment()
            .expect("attachment service is available")
            .materialize_file(
                swallowtail_runtime::ScopeId::new("profile-attachment-scope")
                    .expect("scope is valid"),
                descriptor,
            ),
    )
    .expect("attachment materialization succeeds");
    assert_eq!(
        poll_immediate(
            recording
                .services()
                .attachment()
                .expect("attachment service is available")
                .release_file(lease),
        ),
        CleanupOutcome::Clean
    );

    assert_eq!(recording.count(RecordedHostCall::WorkingResourceResolve), 1);
    assert_eq!(
        recording.count(RecordedHostCall::AttachmentMaterializeFile),
        1
    );
}

fn assert_schema_is_transport_only() {
    let invalid_schema = SchemaDocument::inline(b"not a schema".to_vec(), 128)
        .expect("transport accepts bounded bytes");
    let descriptor = StructuredOutputDescriptor::new(
        invalid_schema,
        "application/schema+json",
        "fixture-dialect",
    )
    .expect("transport descriptor does not perform domain validation");
    assert_eq!(descriptor.dialect(), "fixture-dialect");
}

fn assert_no_implicit_fallback() {
    assert_preflight_rejection_without_side_effects(
        PreflightFixtureCase::WrongExecutionHost,
        PreflightDimension::Topology,
    );
    assert_preflight_rejection_without_side_effects(
        PreflightFixtureCase::RejectedSupportAuthority,
        PreflightDimension::SupportAuthority,
    );
}

fn poll_stream_item(
    stream: &mut RuntimeEventStream,
) -> Option<Result<RuntimeEvent, swallowtail_runtime::RuntimeFailure>> {
    let mut context = Context::from_waker(Waker::noop());
    match Pin::new(stream).poll_next(&mut context) {
        Poll::Ready(item) => item,
        Poll::Pending => panic!("synthetic event stream unexpectedly pending"),
    }
}
