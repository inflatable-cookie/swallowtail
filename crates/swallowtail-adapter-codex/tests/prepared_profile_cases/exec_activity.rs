use super::*;

#[test]
fn prepared_exec_profiles_follow_exact_lifecycle_and_do_not_widen_newer_versions() {
    for (version, collaboration) in [
        ("0.80.0", false),
        ("0.91.0", false),
        ("0.92.0", true),
        ("0.145.0", true),
        ("0.146.0", true),
    ] {
        let exec = prepared(
            CodexPreparedDriver::StructuredExec,
            version,
            &RecordingHostServices::default(),
            false,
        )
        .prepare_structured_exec(CodexExecProfileInput::new(
            RequestId::new(format!("exec-activity-{version}")).unwrap(),
            OperationContent::new("private prompt").unwrap(),
            model(),
            working_resource(),
            ExternalNetworkPolicy::Denied,
            ExternalSearchPolicy::Disabled,
        ))
        .expect("activity-aware exec prepares");
        let profile = exec.evidence().operation().observable_activity();
        assert_eq!(
            profile.availability(),
            ObservableActivityAvailability::Available
        );
        assert_eq!(
            profile.unknown_event_posture(),
            ActivityUnknownEventPosture::PreserveNamespaced
        );
        assert_eq!(
            profile
                .kind(ActivityKindClass::SubagentOrCollaboration)
                .is_some(),
            collaboration
        );
        for kind in [
            ActivityKindClass::AssistantMessage,
            ActivityKindClass::ReasoningSummary,
            ActivityKindClass::FileChange,
            ActivityKindClass::WarningOrError,
        ] {
            assert_eq!(
                profile.lifecycle(kind),
                ActivityLifecycleFidelity::CompletionOnly
            );
        }
        for kind in [
            ActivityKindClass::CommandExecution,
            ActivityKindClass::ProviderOwnedTool,
            ActivityKindClass::ExternalSearch,
            ActivityKindClass::Task,
        ] {
            assert_eq!(
                profile.lifecycle(kind),
                ActivityLifecycleFidelity::CompleteLifecycle
            );
        }
        assert!(profile.kind(ActivityKindClass::ConsumerOwnedTool).is_none());
        assert!(profile.kind(ActivityKindClass::Plan).is_none());
        assert!(profile.kind(ActivityKindClass::Hook).is_none());
        for capability in [Capability::StreamingEvents, Capability::ObservableActivity] {
            assert!(
                exec.plan()
                    .requirements()
                    .capabilities()
                    .any(|requirement| requirement.capability() == capability)
            );
        }
    }

    let profile = |version: &str| {
        prepared(
            CodexPreparedDriver::StructuredExec,
            version,
            &RecordingHostServices::default(),
            false,
        )
        .prepare_structured_exec(CodexExecProfileInput::new(
            RequestId::new(format!("exec-profile-{version}")).unwrap(),
            OperationContent::new("private prompt").unwrap(),
            model(),
            working_resource(),
            ExternalNetworkPolicy::Denied,
            ExternalSearchPolicy::Disabled,
        ))
        .unwrap()
        .evidence()
        .operation()
        .observable_activity()
        .clone()
    };
    assert_eq!(profile("0.145.0"), profile("0.146.0"));
}
