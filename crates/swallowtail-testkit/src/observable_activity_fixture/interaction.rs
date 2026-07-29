use super::ObservableActivityTraceFixture;
use super::support::{
    activity_id, available, content, event, kind_profile, observation, trace, trace_with_posture,
    with_content,
};
use swallowtail_core::{
    ActivityContentStream, ActivityCorrelationKind, ActivityDisclosure, ActivityKindClass,
    ActivityLifecycleFidelity, ActivityUnknownEventPosture,
};
use swallowtail_runtime::{
    ActivityAssistantPhase, ActivityCorrelation, ActivityKind, ActivityLifecyclePhase,
    ActivityNamespace, ActivityStatus, CallbackId, DirectToolCallId, RuntimeEvent,
    RuntimeEventKind,
};

pub(super) fn callback() -> ObservableActivityTraceFixture {
    let callback_id = CallbackId::new("fixture.callback.activity").expect("callback id is valid");
    correlated_tool_trace(
        RuntimeEventKind::CallbackRequested(callback_id.clone()),
        ActivityCorrelation::Callback(callback_id),
        ActivityCorrelationKind::Callback,
        "fixture.activity.callback",
    )
}

pub(super) fn direct_tool() -> ObservableActivityTraceFixture {
    let tool_id = DirectToolCallId::new("fixture.direct-tool.activity").expect("tool id is valid");
    correlated_tool_trace(
        RuntimeEventKind::DirectToolCallAvailable(tool_id.clone()),
        ActivityCorrelation::DirectToolCall(tool_id),
        ActivityCorrelationKind::DirectToolCall,
        "fixture.activity.direct-tool",
    )
}

fn correlated_tool_trace(
    exchange: RuntimeEventKind,
    correlation: ActivityCorrelation,
    correlation_kind: ActivityCorrelationKind,
    id: &str,
) -> ObservableActivityTraceFixture {
    let activity = observation(
        activity_id(id),
        ActivityKind::ConsumerOwnedTool,
        ActivityLifecyclePhase::Completed,
        ActivityStatus::Completed,
        None,
        ActivityDisclosure::IdentityAndLifecycleOnly,
    )
    .with_correlation(correlation);
    ObservableActivityTraceFixture {
        profile: available(kind_profile(
            ActivityKindClass::ConsumerOwnedTool,
            ActivityLifecycleFidelity::CompletionOnly,
            [],
            ActivityDisclosure::IdentityAndLifecycleOnly,
            [correlation_kind],
        )),
        events: vec![
            RuntimeEvent::new(1, RuntimeEventKind::Started),
            RuntimeEvent::new(2, exchange),
            event(3, activity),
        ],
    }
}

pub(super) fn intermediate_assistant() -> ObservableActivityTraceFixture {
    assistant_trace(
        "fixture.activity.assistant.intermediate",
        ActivityAssistantPhase::Intermediate,
        ActivityContentStream::IntermediateAssistantText,
        "I am checking the workspace.",
        false,
    )
}

pub(super) fn final_assistant() -> ObservableActivityTraceFixture {
    assistant_trace(
        "fixture.activity.assistant.final",
        ActivityAssistantPhase::Final,
        ActivityContentStream::FinalAnswerText,
        "The requested work is complete.",
        true,
    )
}

fn assistant_trace(
    id: &str,
    assistant_phase: ActivityAssistantPhase,
    stream: ActivityContentStream,
    text: &str,
    include_output: bool,
) -> ObservableActivityTraceFixture {
    let activity = with_content(
        observation(
            activity_id(id),
            ActivityKind::AssistantMessage,
            ActivityLifecyclePhase::Completed,
            ActivityStatus::Completed,
            Some(assistant_phase),
            ActivityDisclosure::ProviderDisplayContent,
        ),
        stream,
        text,
    );
    let mut events = vec![
        RuntimeEvent::new(1, RuntimeEventKind::Started),
        event(2, activity),
    ];
    if include_output {
        events.push(RuntimeEvent::with_content(
            3,
            RuntimeEventKind::OutputAvailable,
            content(text),
        ));
    }
    ObservableActivityTraceFixture {
        profile: available(kind_profile(
            ActivityKindClass::AssistantMessage,
            ActivityLifecycleFidelity::CompletionOnly,
            [stream],
            ActivityDisclosure::ProviderDisplayContent,
            [],
        )),
        events,
    }
}

pub(super) fn reasoning_summary() -> ObservableActivityTraceFixture {
    trace(
        kind_profile(
            ActivityKindClass::ReasoningSummary,
            ActivityLifecycleFidelity::CompletionOnly,
            [ActivityContentStream::ReasoningSummaryText],
            ActivityDisclosure::ProviderDisplayContent,
            [],
        ),
        [event(
            2,
            with_content(
                observation(
                    activity_id("fixture.activity.reasoning-summary"),
                    ActivityKind::ReasoningSummary,
                    ActivityLifecyclePhase::Completed,
                    ActivityStatus::Completed,
                    None,
                    ActivityDisclosure::ProviderDisplayContent,
                ),
                ActivityContentStream::ReasoningSummaryText,
                "The provider marked this summary for client display.",
            ),
        )],
    )
}

pub(super) fn unknown_semantic() -> ObservableActivityTraceFixture {
    trace_with_posture(
        kind_profile(
            ActivityKindClass::Unknown,
            ActivityLifecycleFidelity::CompletionOnly,
            [],
            ActivityDisclosure::IdentityAndLifecycleOnly,
            [],
        ),
        [event(
            2,
            observation(
                activity_id("fixture.activity.unknown"),
                ActivityKind::Unknown(
                    ActivityNamespace::new("fixture.provider/semantic-item")
                        .expect("namespace is valid"),
                ),
                ActivityLifecyclePhase::Completed,
                ActivityStatus::Completed,
                None,
                ActivityDisclosure::IdentityAndLifecycleOnly,
            ),
        )],
        ActivityUnknownEventPosture::PreserveNamespaced,
    )
}
