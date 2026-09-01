use super::*;

#[test]
fn prepared_app_server_profiles_follow_activity_milestones_without_widening_newer_versions() {
    for (version, collaboration, consumer_tool, hook, phased_messages, complete_plan) in [
        ("0.80.0", false, false, false, false, false),
        ("0.85.0", true, false, false, false, false),
        ("0.92.0", true, false, false, false, false),
        ("0.93.0", true, false, false, false, true),
        ("0.105.0", true, false, false, true, true),
        ("0.106.0", true, true, false, true, true),
        ("0.114.0", true, true, true, true, true),
        ("0.145.0", true, true, true, true, true),
        ("0.146.0", true, true, true, true, true),
    ] {
        let prepared = prepared(
            CodexPreparedDriver::AppServer,
            version,
            &RecordingHostServices::default(),
            false,
        );
        let session = prepared
            .prepare_read_only_session(CodexSessionProfileInput::new(
                RequestId::new(format!("activity-{version}")).unwrap(),
                model(),
                working_resource(),
                None,
                SessionOptions::default(),
            ))
            .expect("activity-aware session prepares");
        let profile = session.evidence().operation().observable_activity();
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
        if collaboration {
            let subagents = profile
                .kind(ActivityKindClass::SubagentOrCollaboration)
                .unwrap();
            assert_eq!(
                subagents.subagent_observation(),
                Some(SubagentObservationFidelity::AttributedActivity)
            );
            assert_eq!(subagents.subagent_control_actions().count(), 5);
        }
        assert_eq!(
            profile.kind(ActivityKindClass::ConsumerOwnedTool).is_some(),
            consumer_tool
        );
        assert_eq!(profile.kind(ActivityKindClass::Hook).is_some(), hook);
        let assistant = profile
            .kind(ActivityKindClass::AssistantMessage)
            .expect("assistant activity is present");
        assert_eq!(
            assistant.disclosure(),
            if phased_messages {
                ActivityDisclosure::ProviderDisplayContent
            } else {
                ActivityDisclosure::IdentityAndLifecycleOnly
            }
        );
        assert_eq!(
            assistant
                .content_streams()
                .any(|stream| stream == ActivityContentStream::FinalAnswerText),
            phased_messages
        );
        assert_eq!(
            profile.lifecycle(ActivityKindClass::CommandExecution),
            ActivityLifecycleFidelity::CompleteLifecycle
        );
        assert_eq!(
            profile.lifecycle(ActivityKindClass::Plan),
            if complete_plan {
                ActivityLifecycleFidelity::CompleteLifecycle
            } else {
                ActivityLifecycleFidelity::UpdateAndCompletion
            }
        );
        assert_eq!(
            profile.lifecycle(ActivityKindClass::ContextCompaction),
            if complete_plan {
                ActivityLifecycleFidelity::CompleteLifecycle
            } else {
                ActivityLifecycleFidelity::CompletionOnly
            }
        );
        for capability in [Capability::StreamingEvents, Capability::ObservableActivity] {
            assert!(
                session
                    .plan()
                    .requirements()
                    .capabilities()
                    .any(|requirement| requirement.capability() == capability)
            );
        }
    }

    let qualified = prepared(
        CodexPreparedDriver::AppServer,
        "0.146.0",
        &RecordingHostServices::default(),
        false,
    )
    .prepare_read_only_session(CodexSessionProfileInput::new(
        RequestId::new("qualified-activity").unwrap(),
        model(),
        working_resource(),
        None,
        SessionOptions::default(),
    ))
    .unwrap();
    let newer = prepared(
        CodexPreparedDriver::AppServer,
        "0.152.1",
        &RecordingHostServices::default(),
        false,
    )
    .prepare_read_only_session(CodexSessionProfileInput::new(
        RequestId::new("newer-activity").unwrap(),
        model(),
        working_resource(),
        None,
        SessionOptions::default(),
    ))
    .unwrap();
    assert_eq!(
        qualified.evidence().operation().observable_activity(),
        newer.evidence().operation().observable_activity()
    );
}
