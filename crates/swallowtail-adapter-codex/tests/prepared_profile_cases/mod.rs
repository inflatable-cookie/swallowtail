use super::*;

mod exec_activity;
mod session;
mod session_management;
mod topology;
mod workspace;

#[test]
fn structured_exec_derives_expanded_capabilities_policy_and_request() {
    let recording = RecordingHostServices::default();
    let prepared = prepared(
        CodexPreparedDriver::StructuredExec,
        "0.145.0",
        &recording,
        true,
    );
    let attachment = AttachmentDescriptor::new(
        AttachmentRef::new("image").unwrap(),
        "image/png",
        AttachmentRole::Input,
    )
    .unwrap()
    .with_known_length(512);
    let output = StructuredOutputDescriptor::new(
        SchemaDocument::inline(br#"{"type":"object"}"#.to_vec(), 1024).unwrap(),
        "application/schema+json",
        "json-schema-2020-12",
    )
    .unwrap();
    let profile = prepared
        .prepare_structured_exec(
            CodexExecProfileInput::new(
                RequestId::new("exec").unwrap(),
                OperationContent::new("private prompt").unwrap(),
                model(),
                working_resource(),
                ExternalNetworkPolicy::HostApproved,
                ExternalSearchPolicy::Enabled,
            )
            .with_reasoning_mode(ReasoningMode::new("low").unwrap())
            .with_deadline(Deadline::at(MonotonicInstant::from_ticks(200)))
            .with_attachments([attachment])
            .with_structured_output(output),
        )
        .expect("structured exec prepares");

    for capability in [
        Capability::StructuredRun,
        Capability::ReasoningSelection,
        Capability::Attachments,
        Capability::StructuredOutput,
        Capability::ExternalSearch,
        Capability::StreamingEvents,
        Capability::ObservableActivity,
    ] {
        assert!(
            profile
                .plan()
                .requirements()
                .capabilities()
                .any(|requirement| requirement.capability() == capability)
        );
    }
    for service in [
        HostServiceKind::Task,
        HostServiceKind::Process,
        HostServiceKind::Time,
        HostServiceKind::Attachment,
        HostServiceKind::Schema,
        HostServiceKind::Network,
    ] {
        assert!(
            profile
                .plan()
                .requirements()
                .host_services()
                .any(|candidate| candidate == service)
        );
    }
    assert_eq!(
        profile.request().policy().provider_retention(),
        ProviderRetentionPolicy::Prohibited
    );
    assert_eq!(
        profile.request().policy().harness_configuration_posture(),
        Some(swallowtail_core::HarnessConfigurationPosture::ProviderSuppressed)
    );
    assert_eq!(profile.request().attachments().len(), 1);
    assert!(profile.request().structured_output().is_some());
    assert!(profile.request().deadline().is_some());
}

#[test]
fn prepared_exec_runs_and_legacy_policy_is_derived_from_version() {
    for (version, retention, posture) in [
        (
            "0.90.0",
            ProviderRetentionPolicy::DurableAllowed,
            swallowtail_core::HarnessConfigurationPosture::Ambient,
        ),
        (
            "0.145.0",
            ProviderRetentionPolicy::Prohibited,
            swallowtail_core::HarnessConfigurationPosture::ProviderSuppressed,
        ),
        (
            "0.146.0",
            ProviderRetentionPolicy::Prohibited,
            swallowtail_core::HarnessConfigurationPosture::ProviderSuppressed,
        ),
    ] {
        let prepared = prepared(
            CodexPreparedDriver::StructuredExec,
            version,
            &RecordingHostServices::default(),
            false,
        );
        let profile = prepared
            .prepare_structured_exec(CodexExecProfileInput::new(
                RequestId::new(format!("exec-{version}")).unwrap(),
                OperationContent::new("private prompt").unwrap(),
                model(),
                working_resource(),
                ExternalNetworkPolicy::Denied,
                ExternalSearchPolicy::Disabled,
            ))
            .expect("exec profile prepares");
        assert_eq!(profile.request().policy().provider_retention(), retention);
        assert_eq!(
            profile.request().policy().harness_configuration_posture(),
            Some(posture)
        );

        let (process, state) = FakeProcessService::completed(COMPLETED_JSONL);
        let handle = block_on(profile.start_run(support::host_services(process)))
            .expect("prepared exec starts");
        assert_eq!(block_on(handle.close()), CleanupOutcome::Clean);
        assert!(state.waited());
    }
}

#[test]
fn unsupported_tools_driver_substitution_and_missing_services_fail_before_effects() {
    let recording = RecordingHostServices::default();
    let app = prepared(CodexPreparedDriver::AppServer, "0.145.0", &recording, false);
    let failure = app
        .prepare_structured_exec(CodexExecProfileInput::new(
            RequestId::new("wrong-driver").unwrap(),
            OperationContent::new("prompt").unwrap(),
            model(),
            working_resource(),
            ExternalNetworkPolicy::Denied,
            ExternalSearchPolicy::Disabled,
        ))
        .expect_err("driver substitution fails");
    assert_eq!(failure.stage(), PreparationStage::Preflight);

    let exec = prepared(
        CodexPreparedDriver::StructuredExec,
        "0.145.0",
        &recording,
        false,
    );
    let failure = exec
        .prepare_structured_exec(
            CodexExecProfileInput::new(
                RequestId::new("tools").unwrap(),
                OperationContent::new("prompt").unwrap(),
                model(),
                working_resource(),
                ExternalNetworkPolicy::Denied,
                ExternalSearchPolicy::Disabled,
            )
            .with_tools([tool("unsupported")]),
        )
        .expect_err("exec tools fail during preparation");
    assert_eq!(failure.stage(), PreparationStage::Preflight);

    let failure = exec
        .prepare_structured_exec(CodexExecProfileInput::new(
            RequestId::new("missing-network").unwrap(),
            OperationContent::new("prompt").unwrap(),
            model(),
            working_resource(),
            ExternalNetworkPolicy::HostApproved,
            ExternalSearchPolicy::Enabled,
        ))
        .expect_err("missing host service fails preflight");
    assert_eq!(failure.stage(), PreparationStage::Preflight);
}

#[test]
fn prepared_profile_debug_and_failure_projection_hide_operation_payloads() {
    let prepared = prepared(
        CodexPreparedDriver::StructuredExec,
        "0.145.0",
        &RecordingHostServices::default(),
        false,
    );
    let secret = "private consumer operation payload";
    let profile = prepared
        .prepare_structured_exec(CodexExecProfileInput::new(
            RequestId::new("redacted-exec").unwrap(),
            OperationContent::new(secret).unwrap(),
            model(),
            working_resource(),
            ExternalNetworkPolicy::Denied,
            ExternalSearchPolicy::Disabled,
        ))
        .expect("profile prepares");
    assert!(!format!("{profile:?}").contains(secret));

    let failure = prepared
        .prepare_structured_exec(
            CodexExecProfileInput::new(
                RequestId::new("redacted-failure").unwrap(),
                OperationContent::new(secret).unwrap(),
                model(),
                working_resource(),
                ExternalNetworkPolicy::Denied,
                ExternalSearchPolicy::Disabled,
            )
            .with_tools([tool("unsupported")]),
        )
        .expect_err("unsupported input fails");
    assert!(!failure.to_string().contains(secret));
    assert!(!format!("{failure:?}").contains(secret));
}
